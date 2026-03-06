use num_complex::Complex64;
use std::f64::consts::PI;

use crate::basis::HilbertBasis;
use crate::lattice::TriangularLattice;
use crate::solver::SolverResult;
use crate::types::{OrderParams, PhaseLabel};

/// Phase classification thresholds
pub struct ClassificationThresholds {
    pub clock_order_min: f64,
    pub photon_density_min: f64,
}

impl Default for ClassificationThresholds {
    fn default() -> Self {
        Self {
            clock_order_min: 0.1,
            photon_density_min: 0.01,
        }
    }
}

/// Extract all order parameters from a solved ground state
pub fn extract_observables(
    result: &SolverResult,
    basis: &HilbertBasis,
    lattice: &TriangularLattice,
) -> OrderParams {
    let psi = &result.ground_state;

    let clock = measure_clock_order(psi, basis, lattice);
    let n_ph = measure_photon_number(psi, basis);
    let sr = measure_sr_order(psi, basis, lattice);

    // Clock wavevector Q for triangular lattice: Q = (4π/3, 0)
    let q_clock = [4.0 * PI / 3.0, 0.0];
    let sq = measure_structure_factor(psi, basis, lattice, &q_clock);

    let gap = result
        .first_excited_energy
        .map(|e1| e1 - result.ground_energy)
        .unwrap_or(0.0);

    let entropy = measure_entanglement(psi, basis, lattice);

    OrderParams {
        clock_order: clock,
        photon_number: n_ph,
        sr_order: sr,
        structure_factor: vec![sq],
        energy: result.ground_energy,
        gap,
        entanglement: entropy,
    }
}

/// Clock order parameter: ψ_clock = (1/N) Σ exp(i·2π·s_i/3)·exp(i·Q·r_i)
/// where s_i is the sublattice index (0, 1, 2)
pub fn measure_clock_order(
    psi: &[Complex64],
    basis: &HilbertBasis,
    lattice: &TriangularLattice,
) -> Complex64 {
    let n = lattice.num_sites;
    let mut order = Complex64::new(0.0, 0.0);

    // Q vector for triangular lattice clock order
    let qx = 4.0 * PI / 3.0;
    let qy = 0.0;

    for (idx, state) in basis.states.iter().enumerate() {
        let prob = psi[idx].norm_sqr();
        if prob < 1e-15 {
            continue;
        }

        let mut local_order = Complex64::new(0.0, 0.0);
        for i in 0..n {
            if state.is_excited(i) {
                // Phase from sublattice
                let sub_phase = match lattice.sublattices[i] {
                    crate::lattice::Sublattice::A => 0.0,
                    crate::lattice::Sublattice::B => 2.0 * PI / 3.0,
                    crate::lattice::Sublattice::C => 4.0 * PI / 3.0,
                };
                // Phase from position
                let pos_phase = qx * lattice.positions[i].x + qy * lattice.positions[i].y;
                let total_phase = sub_phase + pos_phase;
                local_order += Complex64::from_polar(1.0, total_phase);
            }
        }
        local_order /= n as f64;
        order += prob * local_order;
    }

    order
}

/// Photon number expectation value: ⟨n_ph⟩ = Σ |c_i|² · n_ph_i
pub fn measure_photon_number(psi: &[Complex64], basis: &HilbertBasis) -> f64 {
    basis
        .states
        .iter()
        .zip(psi.iter())
        .map(|(state, &c)| c.norm_sqr() * state.photons as f64)
        .sum()
}

/// Superradiant order parameter: Φ_SR = ⟨a⟩/√N
/// Approximated via: ⟨a⟩ = Σ_i c_i* c_j where j has one less photon
pub fn measure_sr_order(
    psi: &[Complex64],
    basis: &HilbertBasis,
    lattice: &TriangularLattice,
) -> f64 {
    let n = lattice.num_sites;
    let mut a_expect = Complex64::new(0.0, 0.0);

    for (idx, state) in basis.states.iter().enumerate() {
        if state.photons == 0 {
            continue;
        }
        // ⟨a⟩: couples |..., n_ph⟩ to |..., n_ph - 1⟩ with factor √n_ph
        let lowered = crate::basis::BasisState::new(state.atoms, state.photons - 1);
        if let Some(low_idx) = basis.index_of(&lowered) {
            a_expect += psi[low_idx].conj() * psi[idx] * (state.photons as f64).sqrt();
        }
    }

    a_expect.norm() / (n as f64).sqrt()
}

/// Structure factor: S(Q) = (1/N) |Σ n_i exp(i·Q·r_i)|²
pub fn measure_structure_factor(
    psi: &[Complex64],
    basis: &HilbertBasis,
    lattice: &TriangularLattice,
    q: &[f64; 2],
) -> f64 {
    let n = lattice.num_sites;
    let mut sq = 0.0;

    for (idx, state) in basis.states.iter().enumerate() {
        let prob = psi[idx].norm_sqr();
        if prob < 1e-15 {
            continue;
        }

        let mut fourier = Complex64::new(0.0, 0.0);
        for i in 0..n {
            if state.is_excited(i) {
                let phase = q[0] * lattice.positions[i].x + q[1] * lattice.positions[i].y;
                fourier += Complex64::from_polar(1.0, phase);
            }
        }
        sq += prob * fourier.norm_sqr() / n as f64;
    }

    sq
}

/// Von Neumann entanglement entropy of the bipartition (first half | second half)
pub fn measure_entanglement(
    psi: &[Complex64],
    basis: &HilbertBasis,
    lattice: &TriangularLattice,
) -> f64 {
    // Simple bipartition: sites 0..N/2 vs N/2..N
    let n = lattice.num_sites;
    let n_a = n / 2;

    // For small systems, compute reduced density matrix
    if basis.dimension() > 10000 {
        return 0.0; // Skip for large systems
    }

    // Group basis states by subsystem A configuration
    use std::collections::HashMap;
    let mut rho_a: HashMap<(u32, usize), HashMap<(u32, usize), Complex64>> = HashMap::new();

    let mask_a = (1u32 << n_a) - 1;

    for (idx, state) in basis.states.iter().enumerate() {
        let a_config = state.atoms & mask_a;
        let b_config = state.atoms >> n_a;

        // Use (a_config, photons) as the A-subsystem key
        // and (b_config, 0) as the B-subsystem key for tracing
        // Simplified: trace over B subsystem
        let a_key = (a_config, state.photons);
        let b_key = (b_config, 0usize);

        for (idx2, state2) in basis.states.iter().enumerate() {
            let a_config2 = state2.atoms & mask_a;
            let b_config2 = state2.atoms >> n_a;
            let b_key2 = (b_config2, 0usize);

            // Only contributes when B parts match
            if b_key != b_key2 || state.photons != state2.photons {
                continue;
            }

            let a_key2 = (a_config2, state2.photons);
            let val = psi[idx] * psi[idx2].conj();

            *rho_a.entry(a_key).or_default().entry(a_key2).or_insert(Complex64::new(0.0, 0.0)) += val;
        }
    }

    // Convert to dense matrix and compute eigenvalues
    let keys: Vec<_> = rho_a.keys().cloned().collect();
    let dim_a = keys.len();
    if dim_a == 0 {
        return 0.0;
    }

    let mut rho_dense = nalgebra::DMatrix::from_element(dim_a, dim_a, 0.0_f64);
    for (i, ki) in keys.iter().enumerate() {
        if let Some(row) = rho_a.get(ki) {
            for (j, kj) in keys.iter().enumerate() {
                if let Some(val) = row.get(kj) {
                    rho_dense[(i, j)] = val.re; // Should be Hermitian, take real part
                }
            }
        }
    }

    let eigen = rho_dense.symmetric_eigen();
    let mut entropy = 0.0;
    for &ev in eigen.eigenvalues.iter() {
        if ev > 1e-15 {
            entropy -= ev * ev.ln();
        }
    }

    entropy
}

/// Classify the phase from order parameters
pub fn classify_phase(params: &OrderParams) -> PhaseLabel {
    classify_phase_with_thresholds(params, &ClassificationThresholds::default())
}

/// Classify phase with custom thresholds
pub fn classify_phase_with_thresholds(
    params: &OrderParams,
    thresholds: &ClassificationThresholds,
) -> PhaseLabel {
    let has_clock = params.clock_order.norm() > thresholds.clock_order_min;
    let has_photons = params.photon_number > thresholds.photon_density_min;

    match (has_clock, has_photons) {
        (true, true) => PhaseLabel::SuperradiantClock,
        (true, false) => PhaseLabel::Clock,
        (false, true) => PhaseLabel::Superradiant,
        (false, false) => PhaseLabel::Normal,
    }
}
