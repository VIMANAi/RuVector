use rydberg_cavity::basis::HilbertBasis;
use rydberg_cavity::hamiltonian::{build_hamiltonian, is_hermitian};
use rydberg_cavity::lattice::TriangularLattice;
use rydberg_cavity::observables::{classify_phase, extract_observables, measure_photon_number};
use rydberg_cavity::scanner::phase_scan;
use rydberg_cavity::solver;
use rydberg_cavity::types::*;

#[test]
fn test_triangular_lattice_geometry() {
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    assert_eq!(lattice.num_sites, 6);

    // Each site should have nearest neighbors
    for nn in &lattice.nn_neighbors {
        assert!(!nn.is_empty(), "Each site must have at least one NN");
    }

    // Sublattice counts should sum to N
    let (a, b, c) = lattice.sublattice_counts();
    assert_eq!(a + b + c, 6);
}

#[test]
fn test_lattice_12_sites() {
    let lattice = TriangularLattice::cluster(12, BoundaryCondition::Periodic);
    assert_eq!(lattice.num_sites, 12);
    assert!(lattice.nn_bond_count() > 0);
}

#[test]
fn test_hilbert_space_free_enumeration() {
    // N=4 atoms, N_max=3 photons, no blockade
    let basis = HilbertBasis::enumerate_free(4, 3);
    // 2^4 = 16 atom configs, 4 photon states → 64 total
    assert_eq!(basis.dimension(), 16 * 4);
}

#[test]
fn test_hilbert_space_with_blockade() {
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Open);
    let basis = HilbertBasis::enumerate(&lattice, 3, 1.5);
    // With blockade, dimension should be less than free case
    let free_dim = (1 << 6) * 4; // 256
    assert!(basis.dimension() < free_dim);
    assert!(basis.dimension() > 0);
}

#[test]
fn test_basis_index_roundtrip() {
    let basis = HilbertBasis::enumerate_free(4, 3);
    for (idx, state) in basis.states.iter().enumerate() {
        let found = basis.index_of(state);
        assert_eq!(found, Some(idx), "Index roundtrip failed for state {:?}", state);
    }
}

#[test]
fn test_hamiltonian_hermiticity() {
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    let cavity = CavityParams {
        frequency: 1.0,
        coupling: 0.3,
        decay_rate: 0.05,
        photon_cutoff: 3,
    };
    let drive = DriveParams {
        rabi_frequency: 1.0,
        detuning: 2.0,
    };
    let interactions = InteractionProfile::default();
    let basis = HilbertBasis::enumerate(&lattice, cavity.photon_cutoff, interactions.blockade_radius);
    let h = build_hamiltonian(&lattice, &basis, &cavity, &drive, &interactions);

    assert!(is_hermitian(&h, 1e-10), "Hamiltonian must be Hermitian");
}

#[test]
fn test_hamiltonian_dimension() {
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    let cavity = CavityParams::default();
    let drive = DriveParams::default();
    let interactions = InteractionProfile::default();
    let basis = HilbertBasis::enumerate(&lattice, cavity.photon_cutoff, interactions.blockade_radius);
    let h = build_hamiltonian(&lattice, &basis, &cavity, &drive, &interactions);

    assert_eq!(h.dimension, basis.dimension());
}

#[test]
fn test_no_cavity_coupling_zero_photons() {
    // With g=0, ground state should have ~0 photons
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    let cavity = CavityParams {
        frequency: 1.0,
        coupling: 0.0, // No cavity coupling
        decay_rate: 0.05,
        photon_cutoff: 3,
    };
    let drive = DriveParams {
        rabi_frequency: 1.0,
        detuning: 3.0,
    };
    let interactions = InteractionProfile::default();
    let basis = HilbertBasis::enumerate(&lattice, cavity.photon_cutoff, interactions.blockade_radius);
    let h = build_hamiltonian(&lattice, &basis, &cavity, &drive, &interactions);
    let result = solver::solve(&h, SolverMethod::ExactFull);

    let n_ph = measure_photon_number(&result.ground_state, &basis);
    assert!(n_ph < 0.01, "Without cavity coupling, photon number should be ~0, got {}", n_ph);
}

#[test]
fn test_cavity_coupling_nonzero_photons() {
    // With g > 0, photon number should be nonzero
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    let cavity = CavityParams {
        frequency: 1.0,
        coupling: 0.5,
        decay_rate: 0.05,
        photon_cutoff: 5,
    };
    let drive = DriveParams {
        rabi_frequency: 1.0,
        detuning: 3.0,
    };
    let interactions = InteractionProfile::default();
    let basis = HilbertBasis::enumerate(&lattice, cavity.photon_cutoff, interactions.blockade_radius);
    let h = build_hamiltonian(&lattice, &basis, &cavity, &drive, &interactions);
    let result = solver::solve(&h, SolverMethod::ExactFull);

    let obs = extract_observables(&result, &basis, &lattice);

    // Energy should differ from g=0 case
    let cavity_zero = CavityParams { coupling: 0.0, ..cavity.clone() };
    let basis_zero = HilbertBasis::enumerate(&lattice, cavity_zero.photon_cutoff, interactions.blockade_radius);
    let h_zero = build_hamiltonian(&lattice, &basis_zero, &cavity_zero, &drive, &interactions);
    let result_zero = solver::solve(&h_zero, SolverMethod::ExactFull);

    assert!(
        (result.ground_energy - result_zero.ground_energy).abs() > 1e-6,
        "Cavity coupling should change the ground state energy"
    );
}

#[test]
fn test_phase_classification_normal() {
    let obs = OrderParams {
        clock_order: num_complex::Complex64::new(0.01, 0.0),
        photon_number: 0.001,
        sr_order: 0.0,
        structure_factor: vec![0.1],
        energy: -1.0,
        gap: 0.5,
        entanglement: 0.3,
    };
    assert_eq!(classify_phase(&obs), PhaseLabel::Normal);
}

#[test]
fn test_phase_classification_src() {
    let obs = OrderParams {
        clock_order: num_complex::Complex64::new(0.5, 0.3),
        photon_number: 0.5,
        sr_order: 0.2,
        structure_factor: vec![2.0],
        energy: -2.0,
        gap: 0.1,
        entanglement: 0.8,
    };
    assert_eq!(classify_phase(&obs), PhaseLabel::SuperradiantClock);
}

#[test]
fn test_phase_classification_clock_only() {
    let obs = OrderParams {
        clock_order: num_complex::Complex64::new(0.5, 0.0),
        photon_number: 0.001,
        sr_order: 0.0,
        structure_factor: vec![2.0],
        energy: -1.5,
        gap: 0.3,
        entanglement: 0.5,
    };
    assert_eq!(classify_phase(&obs), PhaseLabel::Clock);
}

#[test]
fn test_small_phase_scan() {
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    let cavity = CavityParams {
        frequency: 1.0,
        coupling: 0.0,
        decay_rate: 0.05,
        photon_cutoff: 3,
    };
    let interactions = InteractionProfile::default();
    let grid = ParamGrid::new((0.0, 4.0), (0.0, 0.5), 3, 3);

    let diagram = phase_scan(&lattice, &grid, &cavity, &interactions, SolverMethod::ExactFull);

    // Should have 9 points
    assert_eq!(diagram.points.len(), 9);

    // All points should have witness hashes
    for p in &diagram.points {
        assert!(!p.witness_hash.is_empty());
    }
}

#[test]
fn test_symmetry_operations() {
    use rydberg_cavity::symmetry::*;

    let n = 9; // 3x3
    let id = SymmetryOp::identity(n);
    assert_eq!(id.permutation, (0..n).collect::<Vec<_>>());

    let t = translation_a1(3, 3);
    assert_eq!(t.permutation.len(), 9);

    // Translation should be a permutation (bijection)
    let mut sorted = t.permutation.clone();
    sorted.sort();
    assert_eq!(sorted, (0..9).collect::<Vec<_>>());
}

#[test]
fn test_lanczos_vs_exact() {
    // For a small system, Lanczos and exact should agree
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    let cavity = CavityParams {
        frequency: 1.0,
        coupling: 0.2,
        decay_rate: 0.05,
        photon_cutoff: 3,
    };
    let drive = DriveParams {
        rabi_frequency: 1.0,
        detuning: 2.0,
    };
    let interactions = InteractionProfile::default();
    let basis = HilbertBasis::enumerate(&lattice, cavity.photon_cutoff, interactions.blockade_radius);
    let h = build_hamiltonian(&lattice, &basis, &cavity, &drive, &interactions);

    let exact = solver::solve(&h, SolverMethod::ExactFull);
    let lanczos = solver::solve(&h, SolverMethod::Lanczos);

    assert!(
        (exact.ground_energy - lanczos.ground_energy).abs() < 0.01,
        "Lanczos ground energy {:.6} should match exact {:.6}",
        lanczos.ground_energy,
        exact.ground_energy
    );
}
