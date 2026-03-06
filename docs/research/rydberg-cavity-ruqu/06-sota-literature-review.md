# SOTA Literature Review: Rydberg-Cavity QED and the Superradiant Clock Phase

## Cited References for the ruQu Integration Series

---

## 1. Primary Paper

### [P1] Liang, Dong, Xiong, Zhang — "Frustrated Rydberg Atom Arrays Meet Cavity-QED: Emergence of the Superradiant Clock Phase"

- **arXiv:** [2504.05126](https://arxiv.org/abs/2504.05126) (April 2025)
- **Journal:** Physical Review Letters ([PRL](https://journals.aps.org/prl/abstract/10.1103/m14q-xbzc))
- **Method:** Large-scale quantum Monte Carlo (stochastic series expansion)
- **System:** Rydberg atoms on triangular lattice coupled to single-mode optical cavity
- **Key results:**
  - Around half-filling, long-range cavity coupling lifts frustration-induced degeneracy
  - Novel superradiant clock (SRC) phase with coexisting spatial clock order and macroscopic photon occupation
  - SRC completely destroys the fragile order-by-disorder (OBD) phase seen with classical driving
  - Ginzburg-Landau analysis: competition between Z₃ (threefold) and Z₆ (sixfold) clock terms
  - First-order transition at Z₂ symmetry line attributed to nonzero photon density coupling to threefold clock term
  - Cavity-mediated nonlocal ring exchange interactions identified as critical mechanism
  - Authors propose extensions to kagome lattice and spin ice geometries
- **Significance:** First identification of the SRC phase. Demonstrates that quantized light is not merely a probe but actively reshapes many-body order in frustrated systems.

---

## 2. Quantum Monte Carlo Methods for Cavity-Lattice Systems

### [QMC1] Langheld et al. — "Quantum phase diagrams of Dicke-Ising models by a wormhole algorithm"

- **arXiv:** [2409.15082](https://arxiv.org/abs/2409.15082) (September 2024)
- **Method:** Wormhole QMC algorithm for the Dicke-Ising model
- **Key results:**
  - Sign-problem-free QMC for Dicke-Ising models, including geometrically frustrated lattices
  - Confirms intermediate phase with simultaneous antiferromagnetic and superradiant order on chain and square lattice
  - All second-order superradiant transitions match variational mean-field theory valid in infinite dimensions
  - Phase diagram contains first-order, second-order, and multicritical points
- **Relevance to ruQu:** The wormhole algorithm is the methodological foundation for sign-free simulation of frustrated cavity-lattice systems. ruQu's solver backend should implement or interface with this approach.

### [QMC2] Peng et al. — "Quantum phase transitions of the anisotropic Dicke-Ising model in driven Rydberg arrays"

- **arXiv:** [2511.22230](https://arxiv.org/abs/2511.22230) (November 2025)
- **Method:** Improved SSE QMC with explicit cavity Fock state tracking
- **Key results:**
  - Tunable anisotropy parameter in driven Rydberg arrays yields rich phase landscape
  - Scaling laws of photon number in superradiant phase determined via data collapse
  - Computational complexity proportional to system size × inverse temperature (no sign problem)
- **Relevance to ruQu:** Provides the scaling benchmarks ruQu needs for budget estimation. The SSE approach with explicit photon tracking is directly implementable in the solver backend.

### [QMC3] Mendonça et al. — "Quantitative approach for the Dicke-Ising chain"

- **arXiv:** [2601.10210](https://arxiv.org/abs/2601.10210) (January 2026)
- **Method:** NLCE + DMRG for the Dicke-Ising chain in thermodynamic limit
- **Key results:**
  - In thermodynamic limit, Dicke-Ising chain maps exactly to self-consistent effective spin Hamiltonian
  - Cavity enters only via self-consistent transverse field — no quantum correlations between photons and spins needed
  - NLCE + DMRG yields high-precision results in thermodynamic limit
- **Relevance to ruQu:** The self-consistent mapping eliminates the photon Hilbert space entirely for 1D chains, dramatically reducing computational cost. ruQu should implement this as a "thermodynamic limit" solver mode.

---

## 3. Tensor Network and DMRG for Cavity QED

### [TN1] Vu, Cances, Maday, Reichman — "Polaritonic Chemistry Using the Density Matrix Renormalization Group Method"

- **Journal:** J. Chem. Theory Comput. (2024) ([link](https://pubs.acs.org/doi/10.1021/acs.jctc.4c00986))
- **Method:** DMRG with Pauli-Fierz Hamiltonian, cavity photons as auxiliary MPS site
- **Key results:**
  - DMRG efficiently optimizes excess photonic degrees of freedom
  - Asymptotically constant computational cost as photonic basis increases
  - MPS naturally mixes photon number sectors
  - Applied to pentacene (22 correlated π orbitals)
- **Relevance to ruQu:** Demonstrates that DMRG handles the photon mode efficiently by treating it as an additional site in the MPS. This is the recommended approach for ruQu's tensor network solver on quasi-1D Rydberg-cavity geometries.

### [TN2] Tensor-Network Approach to Quantum Optical State Evolution

- **arXiv:** [2511.15295](https://arxiv.org/abs/2511.15295) (November 2025)
- **Method:** MPS formalism for continuous-variable quantum optical systems
- **Key results:**
  - Direct numerical integration of Schrödinger equation in MPS form
  - Highly compressed representation of photonic states
- **Relevance to ruQu:** Applicable to time-dependent problems, complementing the ground-state DMRG approach.

---

## 4. Experimental Realization of Cavity-Coupled Rydberg Arrays

### [EXP1] De Santis, Dura-Kovács, Öncü, Bouscal, Vasileiadis, Zeiher — "Realization of a cavity-coupled Rydberg array"

- **arXiv:** [2602.12152](https://arxiv.org/abs/2602.12152) (February 2026)
- **Group:** Max Planck Institute for Quantum Optics (Zeiher group)
- **Key results:**
  - First demonstration of scalable optical tweezer array with strong cavity coupling AND Rydberg excitation at the same location
  - Dispersive cavity shift ~3 MHz splitting, linewidth 2π × 0.84 MHz
  - Continuous cooling of tweezered atoms for 322 seconds inside cavity
  - Electric-field shielding platform for Rydberg state integrity
- **Significance:** This is the experimental platform that could realize the SRC phase. The gap between theory [P1] and experiment [EXP1] is now one of parameter optimization, not fundamental capability. Direct relevance: ruQu phase capsules could guide the experimental parameter search.

### [EXP2] Hartung et al. — "A quantum-network register assembled with optical tweezers in an optical cavity"

- **Journal:** Science 385, 179–183 (2024)
- **Key results:** Tweezer-assembled atomic register inside optical cavity for quantum networking
- **Relevance:** Demonstrates atom-cavity integration at the single-atom level needed for controlled Rydberg-cavity experiments.

### [EXP3] Grinkemeyer et al. — "Error-detected quantum operations with neutral atoms mediated by an optical cavity"

- **Journal:** Science 387, 1301–1305 (2025)
- **Key results:** Error-detected entangling operations between neutral atoms via cavity
- **Relevance:** Shows that cavity-mediated operations between individual atoms are experimentally mature.

### [EXP4] Cryogenic Rydberg Tweezer Arrays

- **Journal:** PRX Quantum 6, 020337 (2025) ([link](https://link.aps.org/doi/10.1103/PRXQuantum.6.020337))
- **Key results:** ⁸⁷Rb tweezer array in cryogenic environment, 3000-second trap lifetime
- **Relevance:** Long coherence times needed for frustrated phase preparation.

### [EXP5] Bluvstein et al. — "A fault-tolerant neutral-atom architecture"

- **Journal:** Nature (2025) ([link](https://www.nature.com/articles/s41586-025-09848-5))
- **Key results:** 448-atom reconfigurable arrays with Rydberg entangling operations
- **Relevance:** Demonstrates the scale of Rydberg arrays (hundreds of atoms) now experimentally accessible.

---

## 5. Superradiant Lasers and Clocks

### [SR1] Meiser, Ye, Carlson, Holland — "Prospects for a Millihertz-Linewidth Laser"

- **Journal:** Phys. Rev. Lett. 102, 163601 (2009)
- **Key result:** Theoretical proposal for superradiant laser on alkaline-earth clock transition with mHz linewidth
- **Relevance:** Foundational theory for superradiant clocks. The SRC phase could provide a more robust ordered regime for these devices.

### [SR2] Bohnet, Chen, Weiner, Meiser, Holland, Thompson — "A steady-state superradiant laser with less than one intracavity photon"

- **Journal:** Nature 484, 78–81 (2012) ([link](https://www.nature.com/articles/nature10920))
- **Group:** JILA (Thompson lab)
- **Key result:** First experimental superradiant laser. Linewidth 10,000× smaller than quantum limit for non-superradiant lasers. Less than one intracavity photon.
- **Relevance:** Proof of concept for superradiant emission. The SRC phase adds spatial structure to this collective emission.

### [SR3] Norcia et al. — "Superradiance on the millihertz linewidth strontium clock transition"

- **Journal:** Science Advances 2, e1601231 (2016) ([link](https://www.science.org/doi/10.1126/sciadv.1601231))
- **Key result:** Pulsed superradiance on the ⁸⁷Sr clock transition
- **Relevance:** Demonstrates superradiance on a metrologically relevant transition.

### [SR4] Reilly et al. — "SU(3) Superradiant Laser"

- **arXiv:** [2506.12267](https://arxiv.org/abs/2506.12267) (June 2025)
- **Key result:** Three-level model achieving zero cavity pulling — output frequency completely insensitive to cavity frequency
- **Relevance:** Shows that collective emission physics continues to yield new capabilities.

### [SR5] Dissipation-induced superradiant transition in strontium cavity-QED

- **Journal:** Science Advances (2025) ([link](https://www.science.org/doi/10.1126/sciadv.adu5799))
- **Key result:** Second-order phase transition in cold ⁸⁸Sr atoms coupled to driven high-finesse cavity
- **Relevance:** Experimental observation of superradiant phase transition in cavity QED.

### [SR6] Observation of the magnonic Dicke superradiant phase transition

- **Journal:** Science Advances (April 2025) ([link](https://www.science.org/doi/10.1126/sciadv.adt1691))
- **Key result:** First spectroscopic evidence for equilibrium Dicke superradiant phase transition (in ErFeO₃, magnonic system)
- **Relevance:** Demonstrates that superradiant phase transitions are observable in equilibrium, validating the theoretical framework.

---

## 6. Frustrated Magnetism and Order by Disorder

### [FM1] Villain — "Order as an effect of disorder"

- **Journal:** J. Phys. (Paris) 41, 1263 (1980)
- **Key result:** Original formulation of the order-by-disorder concept
- **Relevance:** The mechanism that the SRC phase replaces.

### [FM2] Henley — "Ordering due to disorder in a frustrated vector antiferromagnet"

- **Journal:** Phys. Rev. Lett. 62, 2056 (1989)
- **Key result:** Demonstrated quantum order by disorder in the frustrated J₁-J₂ model
- **Relevance:** Classic reference for quantum fluctuation-selected order.

### [FM3] Moessner, Ramirez — "Geometrical frustration"

- **Journal:** Physics Today 59, 24 (2006)
- **Key result:** Review of geometric frustration across condensed matter
- **Relevance:** Background on triangular, kagome, and pyrochlore frustration.

### [FM4] Savary, Balents — "Quantum spin liquids: a review"

- **Journal:** Rep. Prog. Phys. 80, 016502 (2017)
- **Key result:** Comprehensive review of spin liquid phases in frustrated magnets
- **Relevance:** The SRC phase borders spin-liquid-like regimes in the phase diagram.

### [FM5] Review: Order and disorder in geometrically frustrated magnets

- **arXiv:** [2408.16054](https://arxiv.org/abs/2408.16054) (August 2024)
- **Key result:** Updated review of GF magnets including recent theoretical and experimental advances
- **Relevance:** Current state of the field that the SRC paper extends.

---

## 7. Neural Quantum States for Frustrated Systems

### [NQS1] Roth et al. — "High-accuracy variational Monte Carlo for frustrated magnets with deep neural networks"

- **Journal:** Phys. Rev. B 108, 054410 (2023) ([arXiv:2211.07749](https://arxiv.org/abs/2211.07749))
- **Method:** Group convolutional neural networks (GCNNs) with 4–16 layers
- **Key results:**
  - State-of-the-art ground-state energies for J₁-J₂ Heisenberg models on square and triangular lattices
  - Outperforms DMRG and other variational methods in 2D frustrated systems
  - Symmetry-respecting ansätze via group convolutions
- **Relevance to ruQu:** NQS could serve as a variational ansatz for the Rydberg-cavity system, though extending to include photon modes is an open problem. No existing work combines NQS with cavity QED — this is a gap ruQu could fill.

### [NQS2] Design principles of deep translationally symmetric NQS

- **Journal:** Phys. Rev. Research (2025) ([DOI](https://doi.org/10.1103/ybgv-35jm))
- **Key result:** ConvNext architecture applied to quantum many-body states
- **Relevance:** Architecture insights applicable to ruQu's variational solver.

---

## 8. Rydberg Atom Physics

### [RYD1] Saffman, Walker, Mølmer — "Quantum information with Rydberg atoms"

- **Journal:** Rev. Mod. Phys. 82, 2313 (2010)
- **Key result:** Comprehensive review of Rydberg blockade, interaction strengths, gate protocols
- **Relevance:** Canonical reference for Rydberg physics used throughout this series.

### [RYD2] Browaeys, Lahaye — "Many-body physics with individually controlled Rydberg atoms"

- **Journal:** Nature Physics 16, 132 (2020)
- **Key result:** Review of experimental many-body physics with Rydberg tweezer arrays
- **Relevance:** Experimental context for frustrated Rydberg arrays.

### [RYD3] C₆ coefficients for interacting Rydberg atoms

- **arXiv:** [1912.01568](https://arxiv.org/abs/1912.01568) (2020)
- **Key results:**
  - C₆ ∝ n*¹¹ scaling (n* = effective principal quantum number)
  - Rb 43S₁/₂: C₆ ≈ 1.7 × 10¹⁹ atomic units
  - Blockade radius r_b ≈ 5–10 μm for typical experimental parameters
  - Fitting formula: C₆ = n¹¹(c₀ + c₁·n + c₂·n²)
- **Relevance to ruQu:** Provides the numerical interaction coefficients for the Hamiltonian builder. ruQu should use ARC (Alkali Rydberg Calculator) library values or these fitted forms.

### [RYD4] Weber et al. — "Unravelling the Structures in the van der Waals Interactions of Alkali Rydberg Atoms"

- **arXiv:** [2412.14861](https://arxiv.org/abs/2412.14861) (December 2024)
- **Key result:** Precalculated C₆ datasets with systematic error analysis
- **Relevance:** Most current reference for Rydberg interaction parameters.

---

## 9. Rydberg Quantum Sensors

### [SENS1] Cavity-Enhanced Rydberg Atom Microwave Receiver

- **Journal:** Chinese Physics Letters (2025) ([link](https://cpl.iphy.ac.cn/en/article/doi/10.1088/0256-307X/42/5/053201))
- **Key result:** 18 dB power sensitivity enhancement via microwave cavity
- **Relevance:** Direct application of cavity-enhanced Rydberg sensing.

### [SENS2] Liu et al. — "Cavity-enhanced three-photon system for low-frequency field measurement"

- **Journal:** Frontiers in Physics (2024) ([link](https://www.frontiersin.org/journals/physics/articles/10.3389/fphy.2024.1405149/full))
- **Key result:** Optical cavity + three-photon Rydberg excitation for low-frequency E-field detection
- **Relevance:** Shows cavity integration improves Rydberg sensor performance.

### [SENS3] Adams, Weatherill et al. — "Rydberg atom electric field sensing for metrology, communication and hybrid quantum systems"

- **Journal:** ScienceDirect (2024) ([link](https://www.sciencedirect.com/science/article/abs/pii/S2095927324001889))
- **Key result:** Comprehensive review of Rydberg sensing: metrology, communications, hybrid systems
- **Relevance:** Application landscape for cavity-enhanced Rydberg systems.

---

## 10. Cavity QED Spin Glasses and Exotic Phases

### [CQED1] Lev Lab (Stanford) — Quantum-optical spin glass

- **Source:** [Stanford Lev Lab](https://levlab.stanford.edu/cavity-qed-quantum-soft-matter)
- **Key result:** Photon-mediated interactions produce self-organized quantum spin glasses in multimode cavities
- **Relevance:** Shows cavity QED stabilizes exotic phases beyond the SRC, including spin glasses usable as quantum neural networks.

### [CQED2] Far-from-equilibrium field theory for frustrated multi-mode cavity QED

- **arXiv:** [2312.11624](https://arxiv.org/abs/2312.11624) (December 2023)
- **Key result:** Non-equilibrium field theory framework for frustrated cavity QED with strong disorder
- **Relevance:** Theoretical tools for extending beyond equilibrium phase diagrams.

### [CQED3] Cavity-renormalized quantum criticality in honeycomb bilayer antiferromagnet

- **Journal:** Communications Physics (2023) ([link](https://www.nature.com/articles/s42005-023-01359-x))
- **Method:** QMC for 2D quantum critical magnet coupled to single cavity mode
- **Key result:** Quantum critical point unchanged but critical fluctuations significantly enhanced by cavity
- **Relevance:** Shows cavity coupling modifies universality even when it doesn't shift the critical point.

### [CQED4] Koziol, Langheld, Schmidt — "Coupling long-range interacting matter to a light mode"

- **Source:** FAU LMQ (2025) ([link](https://www.lightmatter.fau.de/2025/10/fau-lmq-research-spotlight-coupling-long-range-interacting-matter-to-a-light-mode/))
- **Key result:** Devil's staircases melt when matter is coupled to a single photonic mode; combined light-matter phases at intermediate coupling
- **Relevance:** Another example of cavity coupling producing novel phases in structured matter.

---

## 11. Dicke Model and Superradiance Theory

### [DICKE1] Dicke — "Coherence in Spontaneous Radiation Processes"

- **Journal:** Phys. Rev. 93, 99 (1954)
- **Key result:** Original prediction of superradiance: N² emission scaling for coherent atomic ensembles

### [DICKE2] Hepp, Lieb — "On the superradiant phase transition"

- **Journal:** Ann. Phys. 76, 360 (1973)
- **Key result:** Rigorous proof of superradiant phase transition in the Dicke model in thermodynamic limit

### [DICKE3] Emary, Brandes — "Chaos and the quantum phase transition in the Dicke model"

- **Journal:** Phys. Rev. E 67, 066203 (2003)
- **Key result:** Complete characterization of the Dicke QPT including quantum chaos signatures

### [DICKE4] Kirton et al. — "Introduction to the Dicke Model"

- **Journal:** Adv. Quantum Technol. 2, 1800043 (2019)
- **Key result:** Pedagogical review of Dicke model physics
- **Relevance:** Standard reference for the cavity-matter coupling framework.

---

## 12. WebAssembly Deterministic Computing

### [WASM1] WebAssembly 3.0 Specification

- **Source:** [WASM 3.0](https://webassembly.org/news/2025-09-17-wasm-3.0/) (September 2025)
- **Key result:** Deterministic execution profile mandating canonical NaN production for all non-deterministic floating-point operations
- **Relevance to ruQu RVF capsules:** This is the standard that enables reproducible cross-host phase computation.

### [WASM2] Wasmtime Deterministic Execution Documentation

- **Source:** [Wasmtime docs](https://docs.wasmtime.dev/examples-deterministic-wasm-execution.html)
- **Key result:** Practical guide to eliminating all WASM non-determinism sources
- **Relevance:** Implementation reference for RVF phase capsule runtime.

### [WASM3] "No installation required: how WebAssembly is changing scientific computing"

- **Journal:** Nature Technology Feature (March 2024) ([link](https://www.nature.com/articles/d41586-024-00725-1))
- **Key result:** Survey of WASM adoption in scientific computing
- **Relevance:** Context for RVF capsules as scientific computing artifacts.

---

## 13. Clock Models on Triangular Lattices

### [CLOCK1] José, Kadanoff, Kirkpatrick, Nelson — "Renormalization, vortices, and symmetry-breaking perturbations in the two-dimensional planar model"

- **Journal:** Phys. Rev. B 16, 1217 (1977)
- **Key result:** Original formulation of the Z_p clock model and its relation to the XY model
- **Relevance:** Theoretical foundation for the clock order parameter in the SRC phase.

### [CLOCK2] Huse, Fisher — "Commensurate melting, domain walls, and dislocations"

- **Journal:** Phys. Rev. B 29, 239 (1984)
- **Key result:** Phase transitions in p-state clock models including effects of domain walls
- **Relevance:** Classification of clock-phase transitions relevant to SRC boundary analysis.

---

## 14. Proof-Carrying Computation

### [PCC1] Necula — "Proof-Carrying Code"

- **Conference:** POPL 1997
- **Key result:** Original formulation of attaching machine-checkable proofs to code
- **Relevance:** Conceptual ancestor of the receipt chain in RVF phase capsules.

### [PCC2] FIPS 204 — ML-DSA (Module-Lattice-Based Digital Signature Algorithm)

- **Source:** NIST (2024)
- **Key result:** Post-quantum digital signature standard
- **Relevance:** ML-DSA-65 signatures provide quantum-resistant attestation for phase capsules.

---

## Gap Analysis: What Does Not Exist Yet

| Gap | Description | ruQu Opportunity |
|-----|-------------|-----------------|
| NQS + cavity QED | No work combines neural quantum states with cavity photon modes | Variational solver with NQS ansatz including photon sector |
| Frustrated 2D lattice + cavity DMRG | DMRG for Dicke models exists only for 1D chains | Extend to cylinder geometries with photon auxiliary site |
| RVF deterministic scientific computing | No existing format packages reproducible simulations as attestable artifacts | Phase capsule architecture (docs 04-05) |
| Federated phase discovery | No protocol for multi-institution collaborative phase diagram construction | Federated capsule protocol (doc 05) |
| Cavity-enhanced Rydberg sensors + frustrated arrays | Sensor work uses disordered ensembles, not structured frustrated arrays | Phase-aware sensing using SRC order |

---

*This document provides the SOTA literature review with citations for the Rydberg-cavity ruQu research series.*
*Series index: [00-superradiant-clock-phase.md](00-superradiant-clock-phase.md)*
