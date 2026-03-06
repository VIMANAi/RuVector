use serde::{Deserialize, Serialize};
use crate::types::BoundaryCondition;

/// 2D position
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// Sublattice assignment for the triangular lattice (A, B, C for 3-sublattice)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sublattice {
    A,
    B,
    C,
}

/// Triangular lattice geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangularLattice {
    pub num_sites: usize,
    pub rows: usize,
    pub cols: usize,
    pub positions: Vec<Position>,
    pub sublattices: Vec<Sublattice>,
    pub nn_neighbors: Vec<Vec<usize>>,  // nearest neighbors per site
    pub nnn_neighbors: Vec<Vec<usize>>, // next-nearest neighbors per site
    pub boundary: BoundaryCondition,
    pub lattice_spacing: f64,
}

impl TriangularLattice {
    /// Create a triangular lattice with given dimensions.
    /// Sites are laid out in rows x cols with triangular geometry.
    /// For N=12: rows=3, cols=4 or rows=4, cols=3.
    pub fn new(rows: usize, cols: usize, boundary: BoundaryCondition) -> Self {
        let num_sites = rows * cols;
        let lattice_spacing = 1.0;

        // Generate positions: triangular lattice
        // Row i is offset by 0.5 * lattice_spacing if i is odd
        let mut positions = Vec::with_capacity(num_sites);
        let mut sublattices = Vec::with_capacity(num_sites);

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f64 * lattice_spacing + if row % 2 == 1 { 0.5 * lattice_spacing } else { 0.0 };
                let y = row as f64 * lattice_spacing * (3.0_f64).sqrt() / 2.0;
                positions.push(Position::new(x, y));

                // 3-sublattice coloring: depends on (row, col) mod 3
                let sub_idx = (row + col) % 3;
                sublattices.push(match sub_idx {
                    0 => Sublattice::A,
                    1 => Sublattice::B,
                    _ => Sublattice::C,
                });
            }
        }

        // Build neighbor lists
        let nn_neighbors = Self::build_nn_neighbors(rows, cols, boundary);
        let nnn_neighbors = Self::build_nnn_neighbors(rows, cols, boundary);

        Self {
            num_sites,
            rows,
            cols,
            positions,
            sublattices,
            nn_neighbors,
            nnn_neighbors,
            boundary,
            lattice_spacing,
        }
    }

    /// Create a standard cluster for a given N.
    /// Picks appropriate rows x cols.
    pub fn cluster(n: usize, boundary: BoundaryCondition) -> Self {
        let (rows, cols) = match n {
            6 => (2, 3),
            9 => (3, 3),
            12 => (3, 4),
            16 => (4, 4),
            18 => (3, 6),
            24 => (4, 6),
            _ => {
                let side = (n as f64).sqrt().ceil() as usize;
                (side, (n + side - 1) / side)
            }
        };
        assert!(rows * cols >= n, "Cannot create cluster of size {n} with {rows}x{cols}");
        Self::new(rows, cols, boundary)
    }

    fn site_index(row: usize, col: usize, cols: usize) -> usize {
        row * cols + col
    }

    fn build_nn_neighbors(rows: usize, cols: usize, boundary: BoundaryCondition) -> Vec<Vec<usize>> {
        let n = rows * cols;
        let mut neighbors = vec![Vec::new(); n];

        for row in 0..rows {
            for col in 0..cols {
                let idx = Self::site_index(row, col, cols);
                let mut nn = Vec::new();

                // Same-row neighbors (left, right)
                match boundary {
                    BoundaryCondition::Periodic => {
                        nn.push(Self::site_index(row, (col + cols - 1) % cols, cols));
                        nn.push(Self::site_index(row, (col + 1) % cols, cols));
                    }
                    BoundaryCondition::Open => {
                        if col > 0 { nn.push(Self::site_index(row, col - 1, cols)); }
                        if col + 1 < cols { nn.push(Self::site_index(row, col + 1, cols)); }
                    }
                }

                // Diagonal neighbors depend on row parity
                let offsets: &[(i32, i32)] = if row % 2 == 0 {
                    &[(-1, -1), (-1, 0), (1, -1), (1, 0)]
                } else {
                    &[(-1, 0), (-1, 1), (1, 0), (1, 1)]
                };

                for &(dr, dc) in offsets {
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;

                    match boundary {
                        BoundaryCondition::Periodic => {
                            let nr = ((nr % rows as i32) + rows as i32) as usize % rows;
                            let nc = ((nc % cols as i32) + cols as i32) as usize % cols;
                            nn.push(Self::site_index(nr, nc, cols));
                        }
                        BoundaryCondition::Open => {
                            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                                nn.push(Self::site_index(nr as usize, nc as usize, cols));
                            }
                        }
                    }
                }

                nn.sort();
                nn.dedup();
                // Remove self
                nn.retain(|&x| x != idx);
                neighbors[idx] = nn;
            }
        }

        neighbors
    }

    fn build_nnn_neighbors(rows: usize, cols: usize, boundary: BoundaryCondition) -> Vec<Vec<usize>> {
        let n = rows * cols;
        let mut neighbors = vec![Vec::new(); n];

        // NNN on triangular lattice: distance = sqrt(3) * a
        // These are the 6 next-nearest neighbors
        for row in 0..rows {
            for col in 0..cols {
                let idx = Self::site_index(row, col, cols);
                let mut nnn = Vec::new();

                // NNN offsets for triangular lattice
                let offsets: &[(i32, i32)] = if row % 2 == 0 {
                    &[(-2, 0), (2, 0), (-1, 1), (1, 1), (-1, -2), (1, -2)]
                } else {
                    &[(-2, 0), (2, 0), (-1, -1), (1, -1), (-1, 2), (1, 2)]
                };

                for &(dr, dc) in offsets {
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;

                    match boundary {
                        BoundaryCondition::Periodic => {
                            let nr = ((nr % rows as i32) + rows as i32) as usize % rows;
                            let nc = ((nc % cols as i32) + cols as i32) as usize % cols;
                            let nidx = Self::site_index(nr, nc, cols);
                            if nidx != idx {
                                nnn.push(nidx);
                            }
                        }
                        BoundaryCondition::Open => {
                            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                                nnn.push(Self::site_index(nr as usize, nc as usize, cols));
                            }
                        }
                    }
                }

                nnn.sort();
                nnn.dedup();
                neighbors[idx] = nnn;
            }
        }

        neighbors
    }

    /// Count total nearest-neighbor bonds (each bond counted once)
    pub fn nn_bond_count(&self) -> usize {
        let total: usize = self.nn_neighbors.iter().map(|n| n.len()).sum();
        total / 2
    }

    /// Count total next-nearest-neighbor bonds
    pub fn nnn_bond_count(&self) -> usize {
        let total: usize = self.nnn_neighbors.iter().map(|n| n.len()).sum();
        total / 2
    }

    /// Get sublattice filling fractions
    pub fn sublattice_counts(&self) -> (usize, usize, usize) {
        let a = self.sublattices.iter().filter(|&&s| s == Sublattice::A).count();
        let b = self.sublattices.iter().filter(|&&s| s == Sublattice::B).count();
        let c = self.sublattices.iter().filter(|&&s| s == Sublattice::C).count();
        (a, b, c)
    }
}
