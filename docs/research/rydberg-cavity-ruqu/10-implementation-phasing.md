# Implementation Phasing and Timeline

## From Research Document to Working Code

---

## 1. Phasing Strategy

The implementation follows a strict dependency order: each phase produces a working, testable artifact before the next phase begins. No phase depends on the RVF capsule layer — that comes last as a packaging step, not a prerequisite.

```
Phase 0: Foundation         (weeks 1–2)
Phase 1: Exact Core         (weeks 3–5)
Phase 2: Phase Discovery    (weeks 6–8)
Phase 3: RuVector Memory    (weeks 9–10)
Phase 4: Scaling & Validation (weeks 11–13)
Phase 5: RVF Capsule        (weeks 14–16)
```

---

## 2. Phase 0: Foundation (Weeks 1–2)

### Goal
Lattice geometry, basis enumeration, and sparse matrix infrastructure.

### Deliverables

| Module | File | What It Does | Test |
|--------|------|-------------|------|
| `lattice.rs` | Triangular lattice geometry | Site positions, neighbor lists, boundary conditions, sublattice assignments | Unit: 12-site cluster has 18 NN bonds, 18 NNN bonds |
| `types.rs` | Shared types | `PhaseLabel`, `ParamPoint`, `PhaseResult`, `OrderParams`, `CavityParams`, `LatticeGeometry` | Unit: serialization roundtrip |
| `basis.rs` | Hilbert space enumeration | Enumerate |s₁...s_N; n_ph⟩ basis states, blockade filtering, filling constraints | Unit: dim(N=6, N_max=5) = 384 |
| `symmetry.rs` (skeleton) | Symmetry group definitions | C₃v, C₆v generators for triangular clusters, character tables | Unit: group multiplication closure |

### MVP Test
```
Given: triangular lattice N=6, N_max=3
Enumerate: all basis states respecting blockade at r_b = 1.5a
Assert: count matches expected dimension
```

### Exit Criterion
All 12-site cluster geometry and basis tests pass. No Hamiltonian, no solver yet.

---

## 3. Phase 1: Exact Core (Weeks 3–5)

### Goal
Working Hamiltonian builder and exact diagonalization for small clusters.

### Deliverables

| Module | File | What It Does | Test |
|--------|------|-------------|------|
| `hamiltonian.rs` | Hamiltonian builder | Construct sparse H from lattice + cavity + coupling + drive parameters | Unit: H is Hermitian, correct dimension, known matrix elements |
| `symmetry.rs` (full) | Symmetry reduction | Project H into momentum and point-group sectors | Unit: sum of sector dimensions = full dimension |
| `solver.rs` (exact) | Exact diagonalization | Full diag for dim < 5000, Lanczos for dim < 10⁶ | Unit: ground state energy of known test case |
| `observables.rs` | Observable extraction | ψ_clock, ⟨n_ph⟩, S(Q), Binder cumulant from ground state | Unit: normal phase has |ψ_clock| ≈ 0, ⟨n_ph⟩ ≈ 0 |

### MVP Test (Acceptance Milestone 1)
```
Given: triangular N=12, PBC, N_max=8
       g/Ω = 0 (no cavity coupling)
       Δ/Ω = 3.0 (in the clock-ordered regime)

Compute: ground state via symmetry-reduced exact diag
Assert:  |ψ_clock| > 0.1 (clock order present)
Assert:  ⟨n_ph⟩ ≈ 0 (no photons without coupling)
Assert:  wall-clock < 1 second
```

### MVP Test (Acceptance Milestone 2)
```
Given: triangular N=12, PBC, N_max=8
       g/Ω = 0.5 (strong cavity coupling)
       Δ/Ω = 3.0

Compute: ground state
Assert:  ⟨n_ph⟩ > 0 (photons present with coupling)
Assert:  energy differs from g=0 case
```

### Exit Criterion
Exact diag produces correct ground state for N=12, N_max=8 in under 1 second. Observables extract correctly. Symmetry reduction gives matching results to unsymmetrized Hamiltonian.

---

## 4. Phase 2: Phase Discovery (Weeks 6–8)

### Goal
Parameter sweep engine that maps the phase diagram and classifies phases.

### Deliverables

| Module | File | What It Does | Test |
|--------|------|-------------|------|
| `scanner.rs` | Parameter sweep engine | Grid scan, adaptive refinement near boundaries | Integration: 50×50 grid completes in < 5 min for N=12 |
| `classifier.rs` | Phase classification | Threshold-based labeling from observables | Unit: known parameter points classified correctly |
| `observables.rs` (extended) | Binder cumulant, susceptibility | χ, U₄ for finite-size scaling | Unit: U₄ = 2/3 for Gaussian distribution |

### MVP Test (Acceptance Milestone 3 — the core acceptance test)
```
Given: triangular N=12, PBC, N_max=8
       Grid: 50×50 in (Δ/Ω ∈ [0,5], g/Ω ∈ [0,1])

Compute: phase diagram via grid scan
Assert:  at least 3 distinct phases detected (Normal, Clock, SR or SRC)
Assert:  if SRC region found: |ψ_clock| > 0.1 AND ⟨n_ph⟩/N > 0.01 coexist
Assert:  phase boundaries are contiguous (no isolated single-point phases)
Assert:  total wall-clock < 30 minutes
```

### Exit Criterion
Phase diagram at N=12 shows a plausible phase structure consistent with [P1]. Phase boundaries are detected. If SRC is not visible at N=12 (possible due to finite size), the code should still produce correct Normal/Clock/SR phases.

---

## 5. Phase 3: RuVector Memory (Weeks 9–10)

### Goal
Store phase scan results in RuVector with HNSW indexing and coherence queries.

### Deliverables

| Module | File | What It Does | Test |
|--------|------|-------------|------|
| `ruvector.rs` | Phase memory interface | Store/query phase points, HNSW indexing | Integration: store 2500 points, retrieve nearest in < 1 ms |
| `coherence.rs` | Mincut boundary detection | Build observable-similarity graph, compute mincut | Integration: mincut matches direct classification on N=12 data |
| `witness.rs` | Witness log generation | Hash-chain witnesses for each computed point | Unit: witness chain verification passes |

### MVP Test (Acceptance Milestone 4)
```
Given: N=12 phase diagram from Phase 2 (2500 points)

Store: all points in RuVector namespace "rydberg_cavity_n12"
Query: nearest 5 points to (Δ/Ω=2.5, g/Ω=0.3) → returns correct neighbors
Mincut: compute mincut boundary → precision > 0.8 vs. direct classification
Cache: re-query same point → returns cached result, not recomputed
```

### Exit Criterion
Phase diagram is stored, queryable, and mincut boundary detection works with > 80% precision against direct classification.

---

## 6. Phase 4: Scaling & Validation (Weeks 11–13)

### Goal
Finite-size scaling across N=12, 18, 24. Cross-validation between exact diag and (optional) QMC.

### Deliverables

| Module | File | What It Does | Test |
|--------|------|-------------|------|
| `scaling.rs` | Finite-size scaling analysis | Binder crossing, data collapse, exponent extraction | Integration: Binder curves cross within error bars |
| `solver.rs` (extended) | Mean-field solver | Self-consistent mean-field for coarse scans | Unit: mean-field phase boundaries within 20% of exact |
| `scanner.rs` (extended) | Multi-size sweep | Automated sweep across N=12,18,24 | Integration: full scaling run completes in < 2 days |

### MVP Test (Acceptance Milestone 5 — the full acceptance test from doc 03)
```
Given: triangular Rydberg lattice, N = 12, 18, 24
       Cavity coupling with tunable g

ruQu must:
1. Recover a parameter region where |ψ_clock| > 0.1 and ⟨n_ph⟩/N > 0.01 coexist
   (at N=24 if not visible at N=12)
2. Map the transition boundary as coupling varies
3. Identify first-order character at Z₂ symmetry line
   (Binder dip or bimodal distribution at N=24)
4. Finite-size extrapolation: boundaries shift consistently with 1/L scaling
5. All computed points carry witness hashes
6. RuVector queries correctly retrieve phase points and boundaries
```

### Exit Criterion
Publication-quality phase diagram with finite-size scaling analysis. SRC phase identified or its absence explained. Mincut validated against standard classification for all system sizes.

---

## 7. Phase 5: RVF Capsule (Weeks 14–16)

### Goal
Package the solver and results as portable, verifiable RVF phase capsules.

### Deliverables

| Module | File | What It Does | Test |
|--------|------|-------------|------|
| `wasm.rs` | WASM compilation target | Compile solver to wasm32-unknown-unknown | Build: WASM binary < 300 KB |
| `capsule.rs` | Phase capsule builder | Package solver + results + receipts into RVF | Unit: capsule roundtrip (build → verify) |
| `wit/` | WIT interface definitions | ruqu:rydberg-cavity@0.1.0 world | Compile: WIT validates without errors |
| `verify.rs` | Receipt verification | Ed25519 signature check, hash chain verification | Unit: valid receipt passes, tampered receipt fails |

### MVP Test (Acceptance Milestone 6)
```
Given: completed phase diagram from Phase 4

Build: RVF phase capsule containing:
  - WASM solver
  - N=12 phase diagram (2500 points)
  - HNSW index
  - Receipt chain
  - Ed25519 signature

Verify: load capsule on different host → signature passes
Execute: re-compute 10 random points using embedded solver → identical results
Size: capsule < 10 MB
```

### Exit Criterion
Self-verifying phase capsule that produces identical results on any WASM host.

---

## 8. Dependency Graph

```
Phase 0: Foundation
  lattice.rs ─── types.rs ─── basis.rs ─── symmetry.rs (skeleton)
       │              │            │
       ▼              ▼            ▼
Phase 1: Exact Core
  hamiltonian.rs ── symmetry.rs (full) ── solver.rs (exact) ── observables.rs
       │                                       │                     │
       ▼                                       ▼                     ▼
Phase 2: Phase Discovery
  scanner.rs ────────────────── classifier.rs
       │                              │
       ▼                              ▼
Phase 3: RuVector Memory
  ruvector.rs ── coherence.rs ── witness.rs
       │              │
       ▼              ▼
Phase 4: Scaling & Validation
  scaling.rs ── solver.rs (mean-field) ── scanner.rs (multi-size)
       │
       ▼
Phase 5: RVF Capsule
  wasm.rs ── capsule.rs ── wit/ ── verify.rs
```

No phase depends on a later phase. Each phase adds to the previous without requiring rework.

---

## 9. Risk-Adjusted Schedule

| Phase | Optimistic | Expected | Pessimistic | Risk |
|-------|-----------|----------|-------------|------|
| 0: Foundation | 1 week | 2 weeks | 3 weeks | Low: well-defined geometry code |
| 1: Exact Core | 2 weeks | 3 weeks | 5 weeks | Medium: sparse matrix + Lanczos correctness |
| 2: Phase Discovery | 2 weeks | 3 weeks | 4 weeks | Medium: phase classification tuning |
| 3: RuVector Memory | 1 week | 2 weeks | 3 weeks | Low: RuVector API is established |
| 4: Scaling | 2 weeks | 3 weeks | 5 weeks | High: N=24 may reveal surprises |
| 5: RVF Capsule | 2 weeks | 3 weeks | 4 weeks | Medium: WASM float determinism |
| **Total** | **10 weeks** | **16 weeks** | **24 weeks** | |

### Critical Path
Foundation → Exact Core → Phase Discovery → Scaling.

RuVector Memory and RVF Capsule are on a parallel path once Phase 2 produces data.

### Highest-Risk Item
Phase 4 (Scaling). If N=24 exact diag reveals unexpected behavior (SRC absent, different phase structure), the project may need to:
- Implement QMC solver (adds 4–6 weeks)
- Extend to larger system sizes (adds compute time)
- Revise the acceptance test criteria

---

## 10. Resource Requirements

### Compute

| Phase | Hardware | Duration |
|-------|---------|----------|
| Phases 0–2 | Laptop / workstation (8 cores, 16 GB) | Interactive |
| Phase 3 | Workstation (16 cores, 32 GB) | < 1 day |
| Phase 4 (N=12,18) | Workstation (16 cores, 32 GB) | < 1 day each |
| Phase 4 (N=24) | Workstation (16 cores, 64 GB) | 1–3 days |
| Phase 4 (N=27 optional) | Server (32 cores, 128 GB) | 1 week |
| Phase 5 | Laptop | < 1 day |

### Dependencies

| Dependency | Status | Required By |
|-----------|--------|-------------|
| Sparse matrix library (sprs or nalgebra-sparse) | Available | Phase 1 |
| Lanczos/Arnoldi solver (arpack-rs or custom) | Available / custom | Phase 1 |
| RuVector SDK (HNSW, coherence engine) | Existing | Phase 3 |
| wasm-bindgen or C ABI exports | Available | Phase 5 |
| Ed25519 (ed25519-dalek) | Available | Phase 5 |
| RVF format library | Existing | Phase 5 |

### Human

| Role | FTE | Phases |
|------|-----|--------|
| Quantum simulation developer | 1.0 | Phases 0–4 |
| RuVector integration developer | 0.5 | Phases 3–4 |
| WASM/RVF developer | 0.5 | Phase 5 |

---

## 11. MVP Definition

**Minimum viable product (end of Phase 2):**
- Working exact diag solver for N=12 Rydberg-cavity system
- Parameter sweep producing a phase diagram
- Phase classification from observables
- No RuVector, no RVF, no scaling — just the physics working

**This proves the architecture before investing in infrastructure.**

If the MVP phase diagram at N=12 shows physically reasonable results (correct limits, plausible phase structure), proceed to Phase 3. If it shows unexpected behavior, investigate before building memory and capsule layers on potentially incorrect foundations.

---

*This document defines the implementation phasing and timeline for the Rydberg-cavity ruQu module.*
*Series index: [00-superradiant-clock-phase.md](00-superradiant-clock-phase.md)*
