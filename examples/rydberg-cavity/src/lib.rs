//! Rydberg-Cavity Superradiant Clock Phase Discovery Engine
//!
//! This crate implements a quantum many-body simulation for discovering and
//! characterizing the superradiant clock (SRC) phase in frustrated triangular
//! Rydberg arrays coupled to a quantized optical cavity.
//!
//! The SRC phase features coexisting spatial clock order and macroscopic photon
//! occupation, driven by long-range light-matter coupling that lifts
//! frustration-induced degeneracy.

pub mod basis;
pub mod hamiltonian;
pub mod lattice;
pub mod observables;
pub mod scanner;
pub mod solver;
pub mod symmetry;
pub mod types;

pub use types::*;
