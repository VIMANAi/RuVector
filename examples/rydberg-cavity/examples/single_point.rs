//! Compute a single phase point for the Rydberg-cavity system.
//! Usage: cargo run --example single_point

use rydberg_cavity::basis::HilbertBasis;
use rydberg_cavity::hamiltonian::{build_hamiltonian, is_hermitian};
use rydberg_cavity::lattice::TriangularLattice;
use rydberg_cavity::observables::{classify_phase, extract_observables};
use rydberg_cavity::solver;
use rydberg_cavity::types::*;

fn main() {
    println!("=== Rydberg-Cavity Single Point Computation ===\n");

    // Set up a small triangular lattice
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    println!("Lattice: {} sites ({} x {}), {} NN bonds",
        lattice.num_sites, lattice.rows, lattice.cols, lattice.nn_bond_count());

    let cavity = CavityParams {
        frequency: 1.0,
        coupling: 0.3,
        decay_rate: 0.05,
        photon_cutoff: 5,
    };

    let drive = DriveParams {
        rabi_frequency: 1.0,
        detuning: 3.0,
    };

    let interactions = InteractionProfile::default();

    // Enumerate basis
    let basis = HilbertBasis::enumerate(&lattice, cavity.photon_cutoff, interactions.blockade_radius);
    println!("Hilbert space dimension: {} ({} atom configs x {} photon states)",
        basis.dimension(), basis.num_atom_configs(), cavity.photon_cutoff + 1);

    // Build Hamiltonian
    println!("\nBuilding Hamiltonian...");
    let hamiltonian = build_hamiltonian(&lattice, &basis, &cavity, &drive, &interactions);
    println!("Hamiltonian dimension: {}x{}", hamiltonian.dimension, hamiltonian.dimension);

    // Check Hermiticity
    if is_hermitian(&hamiltonian, 1e-10) {
        println!("Hermiticity check: PASSED");
    } else {
        println!("WARNING: Hermiticity check FAILED");
    }

    // Solve
    println!("\nSolving (exact diagonalization)...");
    let result = solver::solve(&hamiltonian, SolverMethod::ExactFull);
    println!("Ground state energy: {:.6}", result.ground_energy);
    if let Some(e1) = result.first_excited_energy {
        println!("First excited energy: {:.6}", e1);
        println!("Gap: {:.6}", e1 - result.ground_energy);
    }

    // Extract observables
    let obs = extract_observables(&result, &basis, &lattice);
    println!("\n--- Order Parameters ---");
    println!("Clock order |ψ_clock|: {:.6}", obs.clock_order.norm());
    println!("Photon number ⟨n_ph⟩: {:.6}", obs.photon_number);
    println!("SR order Φ_SR: {:.6}", obs.sr_order);
    println!("Structure factor S(Q): {:.6}", obs.structure_factor[0]);
    println!("Entanglement entropy: {:.6}", obs.entanglement);

    // Classify phase
    let phase = classify_phase(&obs);
    println!("\n=> Phase classification: {}", phase);
}
