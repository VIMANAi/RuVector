//! Scan the (detuning, coupling) parameter space and produce a phase diagram.
//! Usage: cargo run --example phase_scan

use rydberg_cavity::lattice::TriangularLattice;
use rydberg_cavity::scanner::{phase_scan, print_diagram_summary};
use rydberg_cavity::types::*;

fn main() {
    println!("=== Rydberg-Cavity Phase Diagram Scan ===\n");

    // N=6 for fast demonstration (N=12 for real physics)
    let lattice = TriangularLattice::cluster(6, BoundaryCondition::Periodic);
    println!("Lattice: {} sites", lattice.num_sites);

    let cavity = CavityParams {
        frequency: 1.0,
        coupling: 0.0, // will be varied
        decay_rate: 0.05,
        photon_cutoff: 4,
    };

    let interactions = InteractionProfile::default();

    // Scan grid: 10x10 for demo (50x50 for production)
    let grid = ParamGrid::new(
        (0.0, 5.0),   // detuning range
        (0.0, 1.0),   // coupling range
        10,            // detuning steps
        10,            // coupling steps
    );

    println!("Scanning {}x{} grid over Δ/Ω ∈ [{:.1}, {:.1}], g/Ω ∈ [{:.1}, {:.1}]...\n",
        grid.detuning_steps, grid.coupling_steps,
        grid.detuning_range.0, grid.detuning_range.1,
        grid.coupling_range.0, grid.coupling_range.1);

    let diagram = phase_scan(&lattice, &grid, &cavity, &interactions, SolverMethod::ExactFull);

    print_diagram_summary(&diagram);

    // Print ASCII phase map
    println!("\nPhase Map (Δ → rows, g → cols):");
    println!("  N=Normal, C=Clock, S=SR, X=SRC, ?=Unknown\n");
    print!("g\\Δ ");
    for i in 0..grid.detuning_steps {
        let d = grid.detuning_range.0 + i as f64 * (grid.detuning_range.1 - grid.detuning_range.0) / (grid.detuning_steps - 1) as f64;
        print!("{:>4.1} ", d);
    }
    println!();

    for j in (0..grid.coupling_steps).rev() {
        let c = grid.coupling_range.0 + j as f64 * (grid.coupling_range.1 - grid.coupling_range.0) / (grid.coupling_steps - 1) as f64;
        print!("{:.1} ", c);
        for i in 0..grid.detuning_steps {
            let idx = i * grid.coupling_steps + j;
            if idx < diagram.points.len() {
                let ch = match diagram.points[idx].label {
                    PhaseLabel::Normal => "N",
                    PhaseLabel::Clock => "C",
                    PhaseLabel::Superradiant => "S",
                    PhaseLabel::SuperradiantClock => "X",
                    PhaseLabel::Unknown => "?",
                };
                print!("   {} ", ch);
            }
        }
        println!();
    }

    // Print witness hashes for first few points
    println!("\nWitness hashes (first 5 points):");
    for (i, p) in diagram.points.iter().take(5).enumerate() {
        println!("  [{}] Δ={:.2}, g={:.2} → {} ({})",
            i, p.params.detuning, p.params.coupling,
            p.label, &p.witness_hash[..16]);
    }
}
