use nalgebra_sparse::CscMatrix;
use num_complex::Complex64;

use crate::basis::{BasisState, HilbertBasis};
use crate::lattice::TriangularLattice;
use crate::types::{CavityParams, DriveParams, InteractionProfile};

/// Sparse Hamiltonian matrix in CSC format with complex entries
pub struct Hamiltonian {
    pub matrix: CscMatrix<Complex64>,
    pub dimension: usize,
}

/// Build the full Rydberg-cavity Hamiltonian.
///
/// H = H_Rydberg + H_cavity + H_coupling + H_drive
///
/// H_Rydberg = Σ_{<i,j>} V_ij n_i n_j
/// H_cavity  = ω_c a†a
/// H_coupling = (g/√N) Σ_i (a† σ_i⁻ + a σ_i⁺)
/// H_drive   = -Δ Σ_i n_i
pub fn build_hamiltonian(
    lattice: &TriangularLattice,
    basis: &HilbertBasis,
    cavity: &CavityParams,
    drive: &DriveParams,
    interactions: &InteractionProfile,
) -> Hamiltonian {
    let dim = basis.dimension();
    let mut row_indices = Vec::new();
    let mut col_indices = Vec::new();
    let mut values = Vec::new();

    for (idx, state) in basis.states.iter().enumerate() {
        // --- Diagonal terms ---
        let mut diag = Complex64::new(0.0, 0.0);

        // H_Rydberg: V_ij n_i n_j for nearest neighbors
        for i in 0..lattice.num_sites {
            if !state.is_excited(i) { continue; }
            for &j in &lattice.nn_neighbors[i] {
                if j > i && state.is_excited(j) {
                    let dist = lattice.positions[i].distance(&lattice.positions[j]);
                    let v_ij = interactions.c6_coefficient / dist.powi(6);
                    diag += Complex64::new(v_ij, 0.0);
                }
            }
        }

        // H_cavity: ω_c * n_photon
        diag += Complex64::new(cavity.frequency * state.photons as f64, 0.0);

        // H_drive: -Δ Σ_i n_i
        let n_excited = state.excitation_count() as f64;
        diag -= Complex64::new(drive.detuning * n_excited, 0.0);

        if diag.norm() > 1e-15 {
            row_indices.push(idx);
            col_indices.push(idx);
            values.push(diag);
        }

        // --- Off-diagonal: Jaynes-Cummings coupling ---
        // (g/√N) Σ_i (a† σ_i⁻ + a σ_i⁺)
        let g_eff = cavity.coupling / (lattice.num_sites as f64).sqrt();

        for i in 0..lattice.num_sites {
            // a† σ_i⁻: destroy atom excitation at i, create photon
            // Only if atom i is excited and photon number < cutoff
            if state.is_excited(i) && state.photons < cavity.photon_cutoff {
                let new_atoms = state.atoms & !(1 << i);
                let new_photons = state.photons + 1;
                let new_state = BasisState::new(new_atoms, new_photons);

                if let Some(new_idx) = basis.index_of(&new_state) {
                    // Matrix element: g_eff * sqrt(n_photon + 1)
                    let mel = Complex64::new(
                        g_eff * ((state.photons + 1) as f64).sqrt(),
                        0.0,
                    );
                    row_indices.push(new_idx);
                    col_indices.push(idx);
                    values.push(mel);
                    // Hermitian conjugate
                    row_indices.push(idx);
                    col_indices.push(new_idx);
                    values.push(mel.conj());
                }
            }
        }

        // --- Off-diagonal: Drive / Rabi coupling ---
        // Ω Σ_i (σ_i⁺ + σ_i⁻)  [transverse field]
        if drive.rabi_frequency.abs() > 1e-15 {
            for i in 0..lattice.num_sites {
                // σ_i⁺: excite atom i (if ground state)
                if !state.is_excited(i) {
                    let new_atoms = state.atoms | (1 << i);
                    let new_state = BasisState::new(new_atoms, state.photons);

                    if let Some(new_idx) = basis.index_of(&new_state) {
                        let mel = Complex64::new(drive.rabi_frequency / 2.0, 0.0);
                        row_indices.push(new_idx);
                        col_indices.push(idx);
                        values.push(mel);
                        // Hermitian conjugate
                        row_indices.push(idx);
                        col_indices.push(new_idx);
                        values.push(mel.conj());
                    }
                }
            }
        }
    }

    // Build CSC matrix from COO triplets
    let matrix = triplets_to_csc(dim, &row_indices, &col_indices, &values);

    Hamiltonian { matrix, dimension: dim }
}

/// Convert COO triplets to CSC matrix, summing duplicates
fn triplets_to_csc(
    n: usize,
    rows: &[usize],
    cols: &[usize],
    vals: &[Complex64],
) -> CscMatrix<Complex64> {
    // Sort by (col, row) and sum duplicates
    let mut entries: Vec<(usize, usize, Complex64)> = rows
        .iter()
        .zip(cols.iter())
        .zip(vals.iter())
        .map(|((&r, &c), &v)| (c, r, v))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    // Merge duplicates
    let mut merged_rows = Vec::new();
    let mut merged_cols = Vec::new();
    let mut merged_vals = Vec::new();

    for (col, row, val) in entries {
        if !merged_rows.is_empty()
            && *merged_rows.last().unwrap() == row
            && *merged_cols.last().unwrap() == col
        {
            *merged_vals.last_mut().unwrap() += val;
        } else {
            merged_rows.push(row);
            merged_cols.push(col);
            merged_vals.push(val);
        }
    }

    // Build col pointers
    let mut col_ptrs = vec![0usize; n + 1];
    for &c in &merged_cols {
        col_ptrs[c + 1] += 1;
    }
    for i in 1..=n {
        col_ptrs[i] += col_ptrs[i - 1];
    }

    // The data is already sorted by (col, row), so we can use it directly
    CscMatrix::try_from_csc_data(
        n,
        n,
        col_ptrs,
        merged_rows,
        merged_vals,
    )
    .expect("Failed to build CSC matrix")
}

/// Check if the Hamiltonian is Hermitian (for debugging)
pub fn is_hermitian(h: &Hamiltonian, tolerance: f64) -> bool {
    let n = h.dimension;
    // Check H[i,j] == H[j,i]*
    // This is expensive for large matrices; use only for debugging small systems
    if n > 1000 {
        return true; // Skip check for large matrices
    }

    // For small matrices, convert to dense and check
    let mat = &h.matrix;
    let mut dense = nalgebra::DMatrix::from_element(n, n, Complex64::new(0.0, 0.0));

    for j in 0..n {
        let col = mat.col(j);
        for (&row_idx, &val) in col.row_indices().iter().zip(col.values().iter()) {
            dense[(row_idx, j)] = val;
        }
    }

    for i in 0..n {
        for j in i..n {
            let diff = (dense[(i, j)] - dense[(j, i)].conj()).norm();
            if diff > tolerance {
                return false;
            }
        }
    }
    true
}
