# Quantitative Hilbert Space Scaling Analysis

## Hard Numbers for the Rydberg-Cavity System

---

## 1. Hilbert Space Dimension

The total Hilbert space dimension for N Rydberg atoms (each with 2 internal states: ground |g⟩ and Rydberg |r⟩) coupled to a single cavity mode truncated at N_max photons is:

```
dim(H_total) = 2^N × (N_max + 1)
```

### 1.1 Raw Dimensions

| N (atoms) | N_max (photons) | dim(H_total) | Memory (f64 state vector) | Memory (f32) |
|-----------|-----------------|---------------|---------------------------|--------------|
| 6         | 5               | 384           | 3 KB                      | 1.5 KB       |
| 6         | 10              | 704           | 5.5 KB                    | 2.8 KB       |
| 9         | 5               | 3,072         | 24 KB                     | 12 KB        |
| 9         | 10              | 5,632         | 44 KB                     | 22 KB        |
| 12        | 5               | 24,576        | 192 KB                    | 96 KB        |
| 12        | 8               | 36,864        | 288 KB                    | 144 KB       |
| 12        | 10              | 45,056        | 352 KB                    | 176 KB       |
| 15        | 5               | 196,608       | 1.5 MB                    | 768 KB       |
| 15        | 10              | 360,448       | 2.8 MB                    | 1.4 MB       |
| 18        | 5               | 1,572,864     | 12 MB                     | 6 MB         |
| 18        | 8               | 2,359,296     | 18 MB                     | 9 MB         |
| 18        | 10              | 2,883,584     | 22 MB                     | 11 MB        |
| 21        | 5               | 12,582,912    | 96 MB                     | 48 MB        |
| 21        | 10              | 23,068,672    | 176 MB                    | 88 MB        |
| 24        | 5               | 100,663,296   | 768 MB                    | 384 MB       |
| 24        | 8               | 150,994,944   | 1.13 GB                   | 576 MB       |
| 24        | 10              | 184,549,376   | 1.38 GB                   | 704 MB       |
| 27        | 5               | 805,306,368   | 6.0 GB                    | 3.0 GB       |
| 30        | 5               | 6,442,450,944 | 48 GB                     | 24 GB        |

**State vector memory** = dim × 16 bytes (complex f64) or dim × 8 bytes (complex f32).

### 1.2 Hamiltonian Storage

The Hamiltonian is a sparse matrix. For nearest-neighbor interactions on the triangular lattice plus cavity coupling:

```
nnz(H) ≈ N × 2^N × (N_max + 1) + 2^N × N_max    (interaction + cavity terms)
        ≈ (N + 1) × dim(H_total)                    (approximate)
```

| N | N_max | dim | nnz(H) approx | Sparse matrix (CSR, f64) |
|---|-------|-----|----------------|--------------------------|
| 12 | 8 | 36,864 | ~480K | 7 MB |
| 18 | 8 | 2,359,296 | ~45M | 680 MB |
| 24 | 8 | 150,994,944 | ~3.8B | 57 GB |

For N ≥ 24, the Hamiltonian itself does not fit in memory as a stored sparse matrix. Matrix-free methods (applying H as an operator without storing it) are required.

---

## 2. Symmetry Reduction

### 2.1 Available Symmetries

**Triangular lattice with PBC:**

| Symmetry | Group | Order | Reduction factor |
|----------|-------|-------|------------------|
| Translation | T_N (depends on cluster) | N_unit (unit cells) | N_unit |
| Point group (C₃v or C₆v) | Depends on cluster shape | 6 or 12 | 6–12 |
| Z₃ clock | Z₃ | 3 | 3 |
| Photon parity (if Ω=0) | Z₂ | 2 | 2 |

**Combined reduction factor:** up to 36–72× for favorable cluster shapes.

### 2.2 Reduced Dimensions

For a 12-site triangular cluster with 4 unit cells (√12 geometry), PBC, C₃v symmetry, and Z₃ clock sector:

```
Reduction factor ≈ 4 (translation) × 6 (C₃v) × 3 (Z₃) = 72

dim_reduced ≈ 36,864 / 72 ≈ 512     (with N_max=8)
```

This is trivially diagonalizable.

For 18-site cluster (6 unit cells, C₆v):

```
Reduction factor ≈ 6 × 12 × 3 = 216

dim_reduced ≈ 2,359,296 / 216 ≈ 10,923     (with N_max=8)
```

Still very manageable for exact diagonalization.

For 24-site cluster (8 unit cells):

```
Reduction factor ≈ 8 × 6 × 3 = 144

dim_reduced ≈ 150,994,944 / 144 ≈ 1,048,576     (with N_max=8)
```

dim ≈ 10⁶. This requires Lanczos or Arnoldi iterative methods rather than full diagonalization, but is well within reach. Memory for the Lanczos vectors: ~16 MB per vector, need ~100-300 vectors → 1.6–4.8 GB.

For 30-site cluster:

```
dim_reduced ≈ 6.4 × 10⁹ / (10 × 6 × 3) ≈ 35,000,000     (with N_max=5)
```

dim ≈ 3.5 × 10⁷. This is at the edge of iterative methods. Memory for Lanczos: ~560 MB per vector, need ~100 → 56 GB. Feasible on a large workstation or cluster node.

### 2.3 Summary: Exact Diagonalization Feasibility

| N (atoms) | N_max | Full dim | Reduced dim (est.) | Method | Memory (est.) | Wall-clock (est.) |
|-----------|-------|----------|--------------------|---------|--------------|--------------------|
| 12 | 8 | 36,864 | ~500 | Full diag | < 1 MB | < 1 s |
| 12 | 15 | 65,536 | ~900 | Full diag | < 1 MB | < 1 s |
| 18 | 8 | 2.4M | ~11K | Full diag | ~20 MB | ~5 s |
| 18 | 15 | 4.2M | ~19K | Full diag | ~60 MB | ~30 s |
| 24 | 5 | 101M | ~700K | Lanczos | ~2 GB | ~10 min |
| 24 | 8 | 151M | ~1.0M | Lanczos | ~5 GB | ~30 min |
| 24 | 10 | 185M | ~1.3M | Lanczos | ~6 GB | ~1 hr |
| 27 | 5 | 805M | ~4.5M | Lanczos | ~15 GB | ~4 hr |
| 30 | 5 | 6.4B | ~35M | Lanczos | ~56 GB | ~24 hr |

**Conclusion:** Exact methods are comfortable up to N=24, feasible up to N=27 on a workstation, and require cluster resources for N=30.

---

## 3. Photon Truncation Convergence

The cavity photon number in the SRC phase scales as:

```
⟨n_ph⟩ ~ O(N)     in the superradiant phase
⟨n_ph⟩ ~ O(1)     in the normal phase
```

For small clusters (N=12), the superradiant photon number is O(10). The truncation N_max must satisfy:

```
N_max >> ⟨n_ph⟩     (rule of thumb: N_max ≥ 2 × ⟨n_ph⟩_max)
```

### 3.1 Convergence Protocol

```
1. Start with N_max = 5
2. Compute ground state energy E₀ and ⟨n_ph⟩
3. Increase N_max by 3
4. Recompute E₀' and ⟨n_ph⟩'
5. If |E₀' - E₀| / |E₀| < ε and |⟨n_ph⟩' - ⟨n_ph⟩| / ⟨n_ph⟩ < ε:
     converged → use current N_max
6. Else: go to step 3

Tolerance: ε = 10⁻⁴ for phase diagram scanning
           ε = 10⁻⁶ for publication-quality results
```

### 3.2 Expected Convergence

| Phase | ⟨n_ph⟩/N | Required N_max (N=12) | Required N_max (N=24) |
|-------|-----------|----------------------|----------------------|
| Normal | ~0 | 3–5 | 3–5 |
| Clock (no SR) | ~0 | 3–5 | 3–5 |
| SRC (weak coupling) | 0.01–0.1 | 5–8 | 5–8 |
| SRC (strong coupling) | 0.1–0.5 | 8–15 | 10–20 |
| Deep superradiant | > 0.5 | 15–25 | 20–40 |

**Critical observation:** In the deep superradiant regime, photon truncation becomes the dominant cost. For N=24 with N_max=20, the unreduced dimension is 2²⁴ × 21 ≈ 352M, and even with symmetry reduction, dim_reduced ≈ 2.4M. This is still tractable with Lanczos but memory becomes tight.

---

## 4. QMC Scaling

For sign-free QMC methods (SSE/wormhole algorithm from [QMC1], [QMC2]):

```
CPU time ∝ N × β × ⟨n_operators⟩
         ∝ N² × β     (typical scaling with system size)

Memory ∝ N × β × ⟨n_operators⟩
       ∝ N² × β       (operator list storage)
```

where β = 1/T is inverse temperature.

### 4.1 QMC Time Estimates

Assuming 10⁶ Monte Carlo sweeps for equilibration + 10⁶ for measurement:

| N | β | Sweeps | Wall-clock (single core) | Wall-clock (16 cores) |
|---|---|--------|--------------------------|----------------------|
| 12 | 10 | 2M | ~2 min | ~8 s |
| 18 | 10 | 2M | ~5 min | ~20 s |
| 24 | 10 | 2M | ~15 min | ~1 min |
| 36 | 10 | 2M | ~45 min | ~3 min |
| 48 | 20 | 2M | ~4 hr | ~15 min |
| 72 | 20 | 2M | ~18 hr | ~1 hr |

**Key advantage:** QMC does not require photon truncation (the algorithm samples photon occupation stochastically). The sign-free property of the wormhole algorithm [QMC1] eliminates the exponential cost of the sign problem for this specific Hamiltonian class.

**Caveat:** QMC provides stochastic estimates with statistical error bars, not exact results. Typically need 10⁶–10⁸ sweeps for ⟨n_ph⟩ accuracy of 1%.

### 4.2 QMC Memory

QMC memory requirements are modest compared to exact methods:

| N | β | Memory |
|---|---|--------|
| 24 | 10 | ~100 MB |
| 48 | 20 | ~500 MB |
| 72 | 20 | ~2 GB |
| 144 | 20 | ~15 GB |

---

## 5. Mean-Field Scaling

Mean-field (self-consistent Hartree-type) has the cheapest scaling:

```
CPU time ∝ N × N_iter × N_max     (N_iter = self-consistency iterations, typically 20-100)
Memory ∝ N × N_max                  (single-particle states + cavity field)
```

| N | N_max | N_iter | Wall-clock | Memory |
|---|-------|--------|------------|--------|
| 12 | 10 | 50 | < 1 ms | < 1 KB |
| 100 | 10 | 50 | ~10 ms | ~8 KB |
| 1000 | 10 | 100 | ~1 s | ~80 KB |
| 10000 | 10 | 100 | ~100 s | ~800 KB |

**Mean-field is qualitative.** It captures phase boundaries approximately but misses quantum fluctuations that are critical near transitions and in the OBD regime. Use for coarse scanning and initial phase diagram overview only.

---

## 6. Budget Token Calibration

Based on the scaling analysis, concrete budget policies for the WIT interface:

### 6.1 Per-Point Budgets

| Solver | N=12, N_max=8 | N=18, N_max=8 | N=24, N_max=8 |
|--------|---------------|---------------|---------------|
| Exact (symmetry-reduced) | 10 ms, 1 MB | 5 s, 60 MB | 30 min, 5 GB |
| QMC (10⁶ sweeps, 1 core) | 2 min, 50 MB | 5 min, 80 MB | 15 min, 100 MB |
| QMC (10⁶ sweeps, 16 cores) | 8 s, 50 MB | 20 s, 80 MB | 1 min, 100 MB |
| Mean-field | < 1 ms, < 1 KB | < 1 ms, < 1 KB | < 1 ms, < 1 KB |

### 6.2 Full Phase Scan Budgets

For a 100×100 grid (10,000 points):

| Solver | N=12 | N=18 | N=24 |
|--------|------|------|------|
| Exact | 100 s, 1 MB peak | 14 hr, 60 MB peak | 208 days, 5 GB peak |
| QMC (16 cores) | 22 hr, 50 MB | 55 hr, 80 MB | 170 hr, 100 MB |
| Mean-field | 10 s, < 1 MB | 10 s, < 1 MB | 10 s, < 1 MB |

### 6.3 Recommended Strategy

```
Phase 1: Mean-field coarse scan (10 s)
  → 100×100 grid → phase diagram overview
  → identify candidate SRC region and boundaries

Phase 2: QMC refinement at N=24 (1 week on 16 cores)
  → 1000 points in candidate SRC region
  → accurate phase boundaries with error bars

Phase 3: Exact validation at N=12,18 (< 1 day each)
  → full symmetry-reduced diag at selected parameter points
  → cross-validate QMC results

Phase 4: Finite-size scaling (combine Phase 2 + Phase 3)
  → extrapolate phase boundaries to thermodynamic limit
```

**Total compute budget for a complete, publication-quality phase diagram:**
- ~1 week on a 16-core workstation with 32 GB RAM
- ~2 days on a 64-core cluster node with 128 GB RAM

---

## 7. RuVector Storage Budget

### 7.1 Per-Point Storage

Each phase point in RuVector:

```
Parameter coordinates:   5 × f64 = 40 bytes
Order parameters:        8 × f64 = 64 bytes
Phase label + metadata:  ~20 bytes
HNSW embedding (16-dim): 16 × f32 = 64 bytes
Witness hash:            32 bytes
Receipt (if capsule):    ~256 bytes

Total per point: ~476 bytes (without receipt)
                 ~732 bytes (with receipt)
```

### 7.2 Phase Diagram Storage

| Grid resolution | Points | Storage (no receipts) | Storage (with receipts) | HNSW index |
|----------------|--------|-----------------------|-------------------------|------------|
| 50×50 | 2,500 | 1.2 MB | 1.8 MB | ~200 KB |
| 100×100 | 10,000 | 4.7 MB | 7.3 MB | ~800 KB |
| 200×200 | 40,000 | 19 MB | 29 MB | ~3.2 MB |
| 500×500 | 250,000 | 119 MB | 183 MB | ~20 MB |

### 7.3 Phase Capsule Size Estimates

| Capsule contents | Size |
|-----------------|------|
| WASM solver | ~200 KB |
| Phase diagram (100×100) | ~7.3 MB |
| HNSW index | ~800 KB |
| Witness chain | ~320 KB |
| Metadata + signatures | ~50 KB |
| **Total** | **~8.7 MB** |

A publication-quality phase capsule with 10,000 grid points, full receipt chain, and embedded solver fits comfortably in under 10 MB. This is well within RVF segment budgets.

---

*This document provides quantitative Hilbert space scaling analysis for the Rydberg-cavity ruQu module.*
*Series index: [00-superradiant-clock-phase.md](00-superradiant-clock-phase.md)*
