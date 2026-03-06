use crate::lattice::TriangularLattice;

/// A basis state: atom configuration (bitfield) + photon number
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasisState {
    /// Atom excitation pattern as a bitmask (bit i = 1 means atom i is excited)
    pub atoms: u32,
    /// Photon occupation number
    pub photons: usize,
}

impl BasisState {
    pub fn new(atoms: u32, photons: usize) -> Self {
        Self { atoms, photons }
    }

    /// Check if atom i is excited
    pub fn is_excited(&self, i: usize) -> bool {
        (self.atoms >> i) & 1 == 1
    }

    /// Count excited atoms
    pub fn excitation_count(&self) -> u32 {
        self.atoms.count_ones()
    }
}

/// Complete Hilbert space basis for the Rydberg-cavity system
#[derive(Debug, Clone)]
pub struct HilbertBasis {
    /// All valid basis states
    pub states: Vec<BasisState>,
    /// Number of atoms
    pub num_atoms: usize,
    /// Photon cutoff
    pub photon_cutoff: usize,
    /// Map from (atom_config, photon_number) to basis index
    atom_config_indices: Vec<u32>, // sorted atom configs
}

impl HilbertBasis {
    /// Enumerate all basis states respecting Rydberg blockade.
    /// Two atoms within blockade_radius cannot both be excited.
    pub fn enumerate(lattice: &TriangularLattice, photon_cutoff: usize, blockade_radius: f64) -> Self {
        let n = lattice.num_sites;
        assert!(n <= 30, "System size {n} exceeds maximum supported (30)");

        // Find blockaded pairs: pairs of sites within blockade_radius
        let mut blockaded_pairs: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let dist = lattice.positions[i].distance(&lattice.positions[j]);
                if dist < blockade_radius * lattice.lattice_spacing {
                    blockaded_pairs.push((i, j));
                }
            }
        }

        // Enumerate valid atom configurations
        let mut valid_configs: Vec<u32> = Vec::new();
        let max_config = 1u32 << n;

        for config in 0..max_config {
            if Self::respects_blockade(config, &blockaded_pairs) {
                valid_configs.push(config);
            }
        }

        // Build full basis: each valid atom config × each photon number
        let mut states = Vec::with_capacity(valid_configs.len() * (photon_cutoff + 1));
        for &config in &valid_configs {
            for n_ph in 0..=photon_cutoff {
                states.push(BasisState::new(config, n_ph));
            }
        }

        Self {
            states,
            num_atoms: n,
            photon_cutoff,
            atom_config_indices: valid_configs,
        }
    }

    /// Enumerate without blockade constraint (free atoms + photons)
    pub fn enumerate_free(num_atoms: usize, photon_cutoff: usize) -> Self {
        assert!(num_atoms <= 24, "Free enumeration limited to 24 atoms");

        let max_config = 1u32 << num_atoms;
        let valid_configs: Vec<u32> = (0..max_config).collect();

        let mut states = Vec::with_capacity(valid_configs.len() * (photon_cutoff + 1));
        for &config in &valid_configs {
            for n_ph in 0..=photon_cutoff {
                states.push(BasisState::new(config, n_ph));
            }
        }

        Self {
            states,
            num_atoms,
            photon_cutoff,
            atom_config_indices: valid_configs,
        }
    }

    fn respects_blockade(config: u32, blockaded_pairs: &[(usize, usize)]) -> bool {
        for &(i, j) in blockaded_pairs {
            if (config >> i) & 1 == 1 && (config >> j) & 1 == 1 {
                return false;
            }
        }
        true
    }

    /// Dimension of the Hilbert space
    pub fn dimension(&self) -> usize {
        self.states.len()
    }

    /// Number of valid atom configurations (without photon degree of freedom)
    pub fn num_atom_configs(&self) -> usize {
        self.atom_config_indices.len()
    }

    /// Find the index of a basis state, or None if not in the basis
    pub fn index_of(&self, state: &BasisState) -> Option<usize> {
        // Binary search for the atom configuration
        let config_pos = self.atom_config_indices.binary_search(&state.atoms).ok()?;
        if state.photons > self.photon_cutoff {
            return None;
        }
        Some(config_pos * (self.photon_cutoff + 1) + state.photons)
    }
}
