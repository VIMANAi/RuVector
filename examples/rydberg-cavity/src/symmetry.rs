use serde::{Deserialize, Serialize};

/// A symmetry operation represented as a permutation of site indices
#[derive(Debug, Clone)]
pub struct SymmetryOp {
    pub name: String,
    /// permutation[i] = j means site i maps to site j
    pub permutation: Vec<usize>,
}

impl SymmetryOp {
    pub fn identity(n: usize) -> Self {
        Self {
            name: "E".to_string(),
            permutation: (0..n).collect(),
        }
    }

    /// Compose two operations: self followed by other
    pub fn compose(&self, other: &SymmetryOp) -> SymmetryOp {
        let n = self.permutation.len();
        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = other.permutation[self.permutation[i]];
        }
        SymmetryOp {
            name: format!("{}*{}", self.name, other.name),
            permutation: result,
        }
    }

    /// Apply to an atom configuration bitmask
    pub fn apply_to_config(&self, config: u32) -> u32 {
        let n = self.permutation.len();
        let mut result = 0u32;
        for i in 0..n {
            if (config >> i) & 1 == 1 {
                result |= 1 << self.permutation[i];
            }
        }
        result
    }
}

/// Symmetry sector for block diagonalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymmetrySector {
    pub name: String,
    pub momentum: Option<(f64, f64)>,       // crystal momentum (kx, ky)
    pub point_group_irrep: Option<String>,   // irreducible representation label
    pub dimension_reduction: f64,            // expected dimension reduction factor
}

impl Default for SymmetrySector {
    fn default() -> Self {
        Self {
            name: "full".to_string(),
            momentum: None,
            point_group_irrep: None,
            dimension_reduction: 1.0,
        }
    }
}

/// Generate C3 rotation for a rows x cols triangular lattice
/// Rotates 120 degrees around the center
pub fn c3_rotation(rows: usize, cols: usize) -> Option<SymmetryOp> {
    // C3 rotation is only exact for specific cluster geometries
    // For a 3x4 cluster this is approximate; for equilateral clusters it's exact
    if rows != cols {
        return None; // Only for square-ish clusters where C3 is defined
    }

    let n = rows * cols;
    let cx = (cols - 1) as f64 / 2.0;
    let cy = (rows - 1) as f64 * (3.0_f64).sqrt() / 4.0;

    let cos120 = -0.5_f64;
    let sin120 = (3.0_f64).sqrt() / 2.0;

    let mut perm = vec![0usize; n];
    let mut valid = true;

    for row in 0..rows {
        for col in 0..cols {
            let idx = row * cols + col;
            let x = col as f64 + if row % 2 == 1 { 0.5 } else { 0.0 } - cx;
            let y = row as f64 * (3.0_f64).sqrt() / 2.0 - cy;

            // Rotate by 120 degrees
            let rx = x * cos120 - y * sin120 + cx;
            let ry = x * sin120 + y * cos120 + cy;

            // Find nearest site
            let mut best = 0;
            let mut best_dist = f64::MAX;
            for r2 in 0..rows {
                for c2 in 0..cols {
                    let x2 = c2 as f64 + if r2 % 2 == 1 { 0.5 } else { 0.0 };
                    let y2 = r2 as f64 * (3.0_f64).sqrt() / 2.0;
                    let d = (rx - x2).powi(2) + (ry - y2).powi(2);
                    if d < best_dist {
                        best_dist = d;
                        best = r2 * cols + c2;
                    }
                }
            }
            if best_dist > 0.1 {
                valid = false;
                break;
            }
            perm[idx] = best;
        }
        if !valid { break; }
    }

    if valid {
        Some(SymmetryOp { name: "C3".to_string(), permutation: perm })
    } else {
        None
    }
}

/// Generate translation operator along lattice vector a1
pub fn translation_a1(rows: usize, cols: usize) -> SymmetryOp {
    let n = rows * cols;
    let mut perm = vec![0usize; n];
    for row in 0..rows {
        for col in 0..cols {
            let idx = row * cols + col;
            let new_col = (col + 1) % cols;
            perm[idx] = row * cols + new_col;
        }
    }
    SymmetryOp { name: "T_a1".to_string(), permutation: perm }
}

/// Generate translation operator along lattice vector a2
pub fn translation_a2(rows: usize, cols: usize) -> SymmetryOp {
    let n = rows * cols;
    let mut perm = vec![0usize; n];
    for row in 0..rows {
        for col in 0..cols {
            let idx = row * cols + col;
            let new_row = (row + 1) % rows;
            perm[idx] = new_row * cols + col;
        }
    }
    SymmetryOp { name: "T_a2".to_string(), permutation: perm }
}
