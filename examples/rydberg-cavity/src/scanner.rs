use sha2::{Digest, Sha256};

use crate::basis::HilbertBasis;
use crate::hamiltonian::build_hamiltonian;
use crate::lattice::TriangularLattice;
use crate::observables::{classify_phase, extract_observables};
use crate::solver;
use crate::types::*;

/// Scan a parameter grid and produce a phase diagram
pub fn phase_scan(
    lattice: &TriangularLattice,
    grid: &ParamGrid,
    cavity_template: &CavityParams,
    interactions: &InteractionProfile,
    method: SolverMethod,
) -> PhaseDiagram {
    let basis = HilbertBasis::enumerate(lattice, cavity_template.photon_cutoff, interactions.blockade_radius);

    let mut results = Vec::new();

    for (detuning, coupling) in grid.iter() {
        let cavity = CavityParams {
            coupling,
            photon_cutoff: cavity_template.photon_cutoff,
            frequency: cavity_template.frequency,
            decay_rate: cavity_template.decay_rate,
        };

        let drive = DriveParams {
            rabi_frequency: 1.0, // Ω = 1 sets the energy scale
            detuning,
        };

        let hamiltonian = build_hamiltonian(lattice, &basis, &cavity, &drive, interactions);

        // Choose solver based on dimension
        let actual_method = if basis.dimension() < 5000 {
            method
        } else {
            SolverMethod::Lanczos
        };

        let sol = solver::solve(&hamiltonian, actual_method);
        let obs = extract_observables(&sol, &basis, lattice);
        let label = classify_phase(&obs);

        // Compute witness hash
        let witness = compute_witness(&detuning, &coupling, &obs);

        let param_point = ParamPoint {
            detuning,
            coupling,
            cavity_decay: cavity.decay_rate,
            filling: 0.0,
            lattice_size: lattice.num_sites,
            photon_cutoff: cavity.photon_cutoff,
        };

        let confidence = compute_confidence(&obs);

        results.push(PhaseResult {
            params: param_point,
            label,
            order_params: obs,
            confidence,
            solver_method: format!("{:?}", actual_method),
            convergence: sol.convergence,
            witness_hash: witness,
        });
    }

    // Detect phase boundaries
    let boundaries = detect_boundaries(&results, grid);

    PhaseDiagram {
        points: results,
        boundaries,
        lattice_size: lattice.num_sites,
        photon_cutoff: cavity_template.photon_cutoff,
    }
}

/// Compute a witness hash for reproducibility
fn compute_witness(detuning: &f64, coupling: &f64, obs: &OrderParams) -> String {
    let mut hasher = Sha256::new();
    hasher.update(detuning.to_le_bytes());
    hasher.update(coupling.to_le_bytes());
    hasher.update(obs.energy.to_le_bytes());
    hasher.update(obs.photon_number.to_le_bytes());
    hasher.update(obs.clock_order.re.to_le_bytes());
    hasher.update(obs.clock_order.im.to_le_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

/// Compute classification confidence based on distance from thresholds
fn compute_confidence(obs: &OrderParams) -> f64 {
    let clock_mag = obs.clock_order.norm();
    let photon_dens = obs.photon_number;

    // Distance from classification thresholds
    let clock_dist = (clock_mag - 0.1).abs();
    let photon_dist = (photon_dens - 0.01).abs();

    // Higher confidence when far from boundaries
    let min_dist = clock_dist.min(photon_dist);
    (min_dist * 10.0).min(1.0)
}

/// Detect phase boundaries from a grid of results
fn detect_boundaries(results: &[PhaseResult], grid: &ParamGrid) -> Vec<PhaseBoundary> {
    let mut boundaries = Vec::new();
    let cols = grid.coupling_steps;
    let rows = grid.detuning_steps;

    // Scan for neighboring points with different phases
    let mut boundary_map: std::collections::HashMap<(PhaseLabel, PhaseLabel), Vec<ParamPoint>> =
        std::collections::HashMap::new();

    for i in 0..rows {
        for j in 0..cols {
            let idx = i * cols + j;
            if idx >= results.len() {
                continue;
            }
            let current = &results[idx];

            // Check right neighbor
            if j + 1 < cols {
                let right_idx = i * cols + (j + 1);
                if right_idx < results.len() {
                    let right = &results[right_idx];
                    if current.label != right.label {
                        let midpoint = ParamPoint {
                            detuning: (current.params.detuning + right.params.detuning) / 2.0,
                            coupling: (current.params.coupling + right.params.coupling) / 2.0,
                            cavity_decay: current.params.cavity_decay,
                            filling: 0.0,
                            lattice_size: current.params.lattice_size,
                            photon_cutoff: current.params.photon_cutoff,
                        };
                        let key = ordered_pair(current.label, right.label);
                        boundary_map.entry(key).or_default().push(midpoint);
                    }
                }
            }

            // Check bottom neighbor
            if i + 1 < rows {
                let below_idx = (i + 1) * cols + j;
                if below_idx < results.len() {
                    let below = &results[below_idx];
                    if current.label != below.label {
                        let midpoint = ParamPoint {
                            detuning: (current.params.detuning + below.params.detuning) / 2.0,
                            coupling: (current.params.coupling + below.params.coupling) / 2.0,
                            cavity_decay: current.params.cavity_decay,
                            filling: 0.0,
                            lattice_size: current.params.lattice_size,
                            photon_cutoff: current.params.photon_cutoff,
                        };
                        let key = ordered_pair(current.label, below.label);
                        boundary_map.entry(key).or_default().push(midpoint);
                    }
                }
            }
        }
    }

    for ((phase_a, phase_b), points) in boundary_map {
        boundaries.push(PhaseBoundary {
            phase_a,
            phase_b,
            transition_order: TransitionOrder::Unknown,
            boundary_points: points,
        });
    }

    boundaries
}

fn ordered_pair(a: PhaseLabel, b: PhaseLabel) -> (PhaseLabel, PhaseLabel) {
    if (a as u8) <= (b as u8) {
        (a, b)
    } else {
        (b, a)
    }
}

/// Print a summary of the phase diagram
pub fn print_diagram_summary(diagram: &PhaseDiagram) {
    let total = diagram.points.len();
    let mut counts = std::collections::HashMap::new();
    for p in &diagram.points {
        *counts.entry(p.label).or_insert(0usize) += 1;
    }

    println!("Phase Diagram Summary (N={}, N_max={}):", diagram.lattice_size, diagram.photon_cutoff);
    println!("  Total points: {}", total);
    for (label, count) in &counts {
        println!("  {}: {} ({:.1}%)", label, count, *count as f64 / total as f64 * 100.0);
    }
    println!("  Boundaries detected: {}", diagram.boundaries.len());
    for b in &diagram.boundaries {
        println!("    {} ↔ {} ({} points)", b.phase_a, b.phase_b, b.boundary_points.len());
    }
}
