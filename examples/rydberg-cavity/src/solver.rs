use nalgebra::DMatrix;
use num_complex::Complex64;

use crate::hamiltonian::Hamiltonian;
use crate::types::SolverMethod;

/// Result of solving the Hamiltonian
#[derive(Debug, Clone)]
pub struct SolverResult {
    /// Ground state energy
    pub ground_energy: f64,
    /// Ground state wavefunction coefficients
    pub ground_state: Vec<Complex64>,
    /// First excited state energy (if computed)
    pub first_excited_energy: Option<f64>,
    /// Method used
    pub method: SolverMethod,
    /// Convergence metric (0 = no convergence info, small = well converged)
    pub convergence: f64,
}

/// Solve the Hamiltonian for the ground state
pub fn solve(hamiltonian: &Hamiltonian, method: SolverMethod) -> SolverResult {
    match method {
        SolverMethod::ExactFull => solve_exact(hamiltonian),
        SolverMethod::Lanczos => solve_lanczos(hamiltonian, 100),
        SolverMethod::MeanField => solve_mean_field(hamiltonian),
    }
}

/// Full exact diagonalization using nalgebra
/// Suitable for dim < 5000
fn solve_exact(hamiltonian: &Hamiltonian) -> SolverResult {
    let n = hamiltonian.dimension;

    // Convert sparse to dense
    let mut dense = DMatrix::from_element(n, n, Complex64::new(0.0, 0.0));
    let mat = &hamiltonian.matrix;
    for j in 0..n {
        let col = mat.col(j);
        for (&row_idx, &val) in col.row_indices().iter().zip(col.values().iter()) {
            dense[(row_idx, j)] = val;
        }
    }

    // Since H is Hermitian, eigenvalues are real
    // Use the Hermitian eigendecomposition
    // nalgebra doesn't have complex Hermitian eigendecomposition directly,
    // so we use the real-part approach for Hermitian matrices
    // For a Hermitian matrix, we can decompose H = (H + H†)/2 which is real-symmetric
    // if we work in the right basis.

    // Simple approach: convert to real symmetric if H is real
    // Check if imaginary parts are negligible
    let is_real = dense.iter().all(|v| v.im.abs() < 1e-14);

    if is_real {
        let real_dense = DMatrix::from_fn(n, n, |i, j| dense[(i, j)].re);
        let eigen = real_dense.symmetric_eigen();

        // Find ground state (minimum eigenvalue)
        let mut min_idx = 0;
        let mut min_val = eigen.eigenvalues[0];
        let mut second_min = f64::MAX;

        for i in 1..n {
            if eigen.eigenvalues[i] < min_val {
                second_min = min_val;
                min_val = eigen.eigenvalues[i];
                min_idx = i;
            } else if eigen.eigenvalues[i] < second_min {
                second_min = eigen.eigenvalues[i];
            }
        }

        let gs_vec: Vec<Complex64> = eigen
            .eigenvectors
            .column(min_idx)
            .iter()
            .map(|&v| Complex64::new(v, 0.0))
            .collect();

        SolverResult {
            ground_energy: min_val,
            ground_state: gs_vec,
            first_excited_energy: if second_min < f64::MAX {
                Some(second_min)
            } else {
                None
            },
            method: SolverMethod::ExactFull,
            convergence: 0.0,
        }
    } else {
        // Complex Hermitian case: use power iteration / Lanczos as fallback
        solve_lanczos(hamiltonian, 200)
    }
}

/// Lanczos algorithm for finding the lowest eigenvalue/eigenvector of a sparse Hermitian matrix
fn solve_lanczos(hamiltonian: &Hamiltonian, max_iter: usize) -> SolverResult {
    let n = hamiltonian.dimension;
    let mat = &hamiltonian.matrix;

    // Start with random initial vector
    let mut rng_state: u64 = 12345; // Simple deterministic seed
    let mut v: Vec<Complex64> = (0..n)
        .map(|_| {
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let r = (rng_state >> 33) as f64 / (1u64 << 31) as f64 - 0.5;
            Complex64::new(r, 0.0)
        })
        .collect();

    // Normalize
    let norm: f64 = v.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
    for c in v.iter_mut() {
        *c /= norm;
    }

    let mut alphas = Vec::new();
    let mut betas = Vec::new();
    let mut lanczos_vecs: Vec<Vec<Complex64>> = Vec::new();
    lanczos_vecs.push(v.clone());

    let mut v_prev = vec![Complex64::new(0.0, 0.0); n];
    let k = max_iter.min(n).min(300); // Lanczos iterations

    for _iter in 0..k {
        // w = H * v
        let w = sparse_matvec(mat, &v, n);

        // alpha = <v, w>
        let alpha: Complex64 = v.iter().zip(w.iter()).map(|(vi, wi)| vi.conj() * wi).sum();
        alphas.push(alpha.re);

        // w = w - alpha * v - beta_prev * v_prev
        let beta_prev = betas.last().copied().unwrap_or(0.0);
        let mut w_orth: Vec<Complex64> = w
            .iter()
            .zip(v.iter())
            .zip(v_prev.iter())
            .map(|((wi, vi), vpi)| {
                wi - Complex64::new(alpha.re, 0.0) * vi - Complex64::new(beta_prev, 0.0) * vpi
            })
            .collect();

        // Full reorthogonalization against all previous Lanczos vectors
        for prev_v in &lanczos_vecs {
            let overlap: Complex64 = prev_v.iter().zip(w_orth.iter()).map(|(a, b)| a.conj() * b).sum();
            for (wo, pv) in w_orth.iter_mut().zip(prev_v.iter()) {
                *wo -= overlap * pv;
            }
        }

        let beta: f64 = w_orth.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        if beta < 1e-12 {
            break; // Invariant subspace found
        }
        betas.push(beta);

        v_prev = v.clone();
        v = w_orth.iter().map(|c| c / beta).collect();
        lanczos_vecs.push(v.clone());
    }

    // Diagonalize the tridiagonal matrix
    let m = alphas.len();
    let mut tri = DMatrix::from_element(m, m, 0.0_f64);
    for i in 0..m {
        tri[(i, i)] = alphas[i];
        if i + 1 < m && i < betas.len() {
            tri[(i, i + 1)] = betas[i];
            tri[(i + 1, i)] = betas[i];
        }
    }

    let eigen = tri.symmetric_eigen();

    let mut min_idx = 0;
    let mut min_val = eigen.eigenvalues[0];
    let mut second_min = f64::MAX;
    for i in 1..m {
        if eigen.eigenvalues[i] < min_val {
            second_min = min_val;
            min_val = eigen.eigenvalues[i];
            min_idx = i;
        } else if eigen.eigenvalues[i] < second_min {
            second_min = eigen.eigenvalues[i];
        }
    }

    // Reconstruct ground state in original basis
    let mut gs = vec![Complex64::new(0.0, 0.0); n];
    for (j, lv) in lanczos_vecs.iter().enumerate().take(m) {
        let coeff = eigen.eigenvectors[(j, min_idx)];
        for (i, val) in lv.iter().enumerate() {
            gs[i] += Complex64::new(coeff, 0.0) * val;
        }
    }

    // Normalize
    let gs_norm: f64 = gs.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
    for c in gs.iter_mut() {
        *c /= gs_norm;
    }

    SolverResult {
        ground_energy: min_val,
        ground_state: gs,
        first_excited_energy: if second_min < f64::MAX {
            Some(second_min)
        } else {
            None
        },
        method: SolverMethod::Lanczos,
        convergence: if betas.is_empty() { 0.0 } else { *betas.last().unwrap() },
    }
}

/// Simple mean-field solver (placeholder for qualitative phase diagram)
fn solve_mean_field(hamiltonian: &Hamiltonian) -> SolverResult {
    // Mean-field: assume product state, self-consistently solve
    // For now, return a trivial result as placeholder
    let n = hamiltonian.dimension;
    let mut gs = vec![Complex64::new(0.0, 0.0); n];
    if n > 0 {
        gs[0] = Complex64::new(1.0, 0.0); // vacuum state
    }

    SolverResult {
        ground_energy: 0.0,
        ground_state: gs,
        first_excited_energy: None,
        method: SolverMethod::MeanField,
        convergence: 1.0, // Not converged — placeholder
    }
}

/// Sparse matrix-vector product for CSC format
fn sparse_matvec(
    mat: &nalgebra_sparse::CscMatrix<Complex64>,
    vec: &[Complex64],
    n: usize,
) -> Vec<Complex64> {
    let mut result = vec![Complex64::new(0.0, 0.0); n];
    for j in 0..n {
        let col = mat.col(j);
        let vj = vec[j];
        for (&row, &val) in col.row_indices().iter().zip(col.values().iter()) {
            result[row] += val * vj;
        }
    }
    result
}
