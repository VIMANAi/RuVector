use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Phase labels for the Rydberg-cavity system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhaseLabel {
    Normal,
    Clock,
    Superradiant,
    SuperradiantClock, // The target SRC phase
    Unknown,
}

impl fmt::Display for PhaseLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Clock => write!(f, "Clock"),
            Self::Superradiant => write!(f, "SR"),
            Self::SuperradiantClock => write!(f, "SRC"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Boundary condition type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundaryCondition {
    Open,
    Periodic,
}

/// Cavity parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CavityParams {
    pub frequency: f64,       // ω_c / Ω
    pub coupling: f64,        // g / Ω
    pub decay_rate: f64,      // κ / g
    pub photon_cutoff: usize, // N_max
}

impl Default for CavityParams {
    fn default() -> Self {
        Self {
            frequency: 1.0,
            coupling: 0.3,
            decay_rate: 0.05,
            photon_cutoff: 8,
        }
    }
}

/// Drive parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveParams {
    pub rabi_frequency: f64, // Ω (sets energy scale, typically = 1)
    pub detuning: f64,       // Δ / Ω
}

impl Default for DriveParams {
    fn default() -> Self {
        Self {
            rabi_frequency: 1.0,
            detuning: 3.0,
        }
    }
}

/// Interaction profile for Rydberg atoms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionProfile {
    pub c6_coefficient: f64,   // C₆ van der Waals coefficient
    pub blockade_radius: f64,  // r_b in units of lattice spacing
}

impl Default for InteractionProfile {
    fn default() -> Self {
        Self {
            c6_coefficient: 1.0,
            blockade_radius: 1.5,
        }
    }
}

/// A point in parameter space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamPoint {
    pub detuning: f64,        // Δ/Ω
    pub coupling: f64,        // g/Ω
    pub cavity_decay: f64,    // κ/g
    pub filling: f64,         // sublattice occupation fraction
    pub lattice_size: usize,  // N atoms
    pub photon_cutoff: usize, // N_max
}

/// Measured order parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderParams {
    pub clock_order: Complex64,    // ψ_clock
    pub photon_number: f64,        // ⟨n_ph⟩
    pub sr_order: f64,             // Φ_SR
    pub structure_factor: Vec<f64>,// S(Q) at key wavevectors
    pub energy: f64,               // ground state energy
    pub gap: f64,                  // excitation gap
    pub entanglement: f64,         // von Neumann entropy
}

/// Result of a phase point computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseResult {
    pub params: ParamPoint,
    pub label: PhaseLabel,
    pub order_params: OrderParams,
    pub confidence: f64,
    pub solver_method: String,
    pub convergence: f64,
    pub witness_hash: String,
}

/// Phase boundary between two phases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseBoundary {
    pub phase_a: PhaseLabel,
    pub phase_b: PhaseLabel,
    pub transition_order: TransitionOrder,
    pub boundary_points: Vec<ParamPoint>,
}

/// Transition order classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionOrder {
    First,
    Second,
    Crossover,
    Unknown,
}

/// Full phase diagram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseDiagram {
    pub points: Vec<PhaseResult>,
    pub boundaries: Vec<PhaseBoundary>,
    pub lattice_size: usize,
    pub photon_cutoff: usize,
}

/// Solver method selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolverMethod {
    ExactFull,    // Full diagonalization for dim < 5000
    Lanczos,      // Iterative for larger systems
    MeanField,    // Self-consistent mean field
}

/// Parameter grid for scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamGrid {
    pub detuning_range: (f64, f64),
    pub coupling_range: (f64, f64),
    pub detuning_steps: usize,
    pub coupling_steps: usize,
}

impl ParamGrid {
    pub fn new(det_range: (f64, f64), coup_range: (f64, f64), det_steps: usize, coup_steps: usize) -> Self {
        Self {
            detuning_range: det_range,
            coupling_range: coup_range,
            detuning_steps: det_steps,
            coupling_steps: coup_steps,
        }
    }

    /// Iterator over all grid points
    pub fn iter(&self) -> impl Iterator<Item = (f64, f64)> + '_ {
        let d_step = if self.detuning_steps > 1 {
            (self.detuning_range.1 - self.detuning_range.0) / (self.detuning_steps - 1) as f64
        } else { 0.0 };
        let c_step = if self.coupling_steps > 1 {
            (self.coupling_range.1 - self.coupling_range.0) / (self.coupling_steps - 1) as f64
        } else { 0.0 };

        (0..self.detuning_steps).flat_map(move |i| {
            let d = self.detuning_range.0 + i as f64 * d_step;
            (0..self.coupling_steps).map(move |j| {
                let c = self.coupling_range.0 + j as f64 * c_step;
                (d, c)
            })
        })
    }
}
