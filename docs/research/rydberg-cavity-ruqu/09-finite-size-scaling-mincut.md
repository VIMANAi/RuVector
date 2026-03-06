# Finite-Size Scaling Protocol and Mincut Phase Detection Validation

## Making Phase Transitions Real at Finite System Size

---

## 1. The Problem

True phase transitions occur only in the thermodynamic limit (N → ∞). At any finite system size N, order parameters are analytic functions of the control parameters — there are no true singularities. What appears as a sharp phase boundary at N=24 may be a smooth crossover that sharpens or shifts as N increases.

The SRC phase is identified in [P1] using finite-size QMC simulations. To validate ruQu's phase identification, we need a rigorous finite-size scaling protocol that:

1. Distinguishes genuine transitions from finite-size artifacts
2. Extrapolates phase boundary locations to the thermodynamic limit
3. Classifies transition order (first vs. second) from finite-size data
4. Validates that RuVector's mincut detection correctly identifies the physical phase boundary

---

## 2. Finite-Size Scaling Theory

### 2.1 Second-Order Transitions

Near a second-order transition at control parameter g = g_c, the order parameter scales as:

```
⟨O⟩(g, L) = L^{-β/ν} × f_O((g - g_c) × L^{1/ν})

where:
  L = linear system size (N = L² for 2D, or cluster-specific)
  β = order parameter critical exponent
  ν = correlation length exponent
  f_O = universal scaling function
```

**Data collapse procedure:**

1. Compute ⟨O⟩ vs. g for sizes L₁, L₂, L₃ (e.g., L = √12, √18, √24)
2. Plot L^{β/ν} × ⟨O⟩ vs. (g - g_c) × L^{1/ν}
3. Adjust g_c, β/ν, and 1/ν until curves for all sizes collapse onto a single curve
4. If collapse works: transition is second-order with extracted exponents
5. If collapse fails: transition is first-order or crossover

**For the SRC phase:** The clock order parameter ψ_clock should scale this way near the clock-ordering transition. The superradiant order Φ_SR should scale independently near the SR transition. At the SRC phase boundary, both scalings must be checked.

### 2.2 First-Order Transitions

At a first-order transition, the order parameter jumps discontinuously. At finite size, the jump is rounded:

```
⟨O⟩(g, L) shows a rounded step near g = g_t(L)

with:
  g_t(L) = g_t(∞) + a/L^d + ...     (shift)
  width of step ∝ 1/L^d              (sharpening)
```

**Binder cumulant for first-order detection:**

```
U₄ = 1 - ⟨O⁴⟩ / (3⟨O²⟩²)

At second-order transition: U₄ has a fixed point (size-independent crossing)
At first-order transition: U₄ develops a negative dip that deepens with L
```

**Energy histogram method:**

```
P(E) at the transition:
  Second-order: single Gaussian peak
  First-order: double peak (bimodal distribution)
  Peak separation grows with L for first-order
```

**For the SRC phase:** The Z₂ symmetry line transition is predicted to be first-order in [P1]. The protocol must confirm:
- Bimodal energy distribution at the transition
- Negative Binder cumulant dip
- Jump in ⟨n_ph⟩ sharpening with L

### 2.3 Available System Sizes

Triangular lattice clusters with full point-group symmetry:

| N (sites) | L (linear) | Shape | Symmetry group | Notes |
|-----------|-----------|-------|---------------|-------|
| 9 | 3 | 3×3 rhombus | C₃v | Too small for SRC |
| 12 | 2√3 | 12-site cluster | C₆v | Minimum viable |
| 18 | 3√2 | 18-site cluster | C₃v | Good intermediate |
| 21 | √21 | 21-site cluster | C₃ | Lower symmetry |
| 24 | 2√6 | 24-site cluster | C₆v | Target for exact |
| 27 | 3√3 | 27-site cluster | C₃v | Exact + QMC |
| 36 | 6 | 6×6 rhombus | C₆v | QMC only |
| 48 | 4√3 | 48-site cluster | C₆v | QMC only |

**Minimum requirement:** Three system sizes for finite-size scaling. Recommended: N = 12, 18, 24 for exact diagonalization cross-validated with QMC, plus N = 36, 48 QMC-only for scaling confidence.

---

## 3. Finite-Size Scaling Protocol for ruQu

### 3.1 Step-by-Step Procedure

```
PROTOCOL: Finite-Size Scaling for SRC Phase Identification

Prerequisites:
  - Converged ground-state results at N = 12, 18, 24 (exact diag)
  - Converged QMC results at N = 12, 18, 24, 36 (optional: 48)
  - Parameter sweep along a path crossing the predicted SRC boundary

Step 1: Identify approximate transition location
  For each system size N:
    a. Sweep g/Ω from 0 to 1 at fixed Δ/Ω (through predicted SRC region)
    b. Compute: ψ_clock(g), ⟨n_ph⟩(g), U₄_clock(g), U₄_SR(g)
    c. Record location where ψ_clock rises above noise and ⟨n_ph⟩/N rises above threshold
    d. Record location of U₄ crossing or dip

Step 2: Binder cumulant analysis
  a. Plot U₄_clock(g) for all system sizes on same axes
  b. Look for crossing point: all curves should cross at g = g_c (second-order)
     or develop negative dip (first-order)
  c. Repeat for U₄_SR

Step 3: Order parameter scaling (if second-order)
  a. Extract g_c from Binder crossing
  b. Attempt data collapse: plot L^{β/ν} × ψ_clock vs. (g - g_c) × L^{1/ν}
  c. Optimize β/ν and ν by minimizing collapse residual
  d. Compare extracted exponents with known universality classes:
     - 3-state Potts: β/ν ≈ 2/15, ν ≈ 5/6
     - Z₃ clock: same as 3-state Potts in 2D
     - Dicke SR: mean-field (β = 1/2, ν = 1/2) or beyond

Step 4: First-order analysis (at Z₂ line)
  a. Compute energy histogram P(E) at the estimated transition point
  b. Look for double-peak structure
  c. Compute peak separation ΔE vs. system size
  d. If ΔE grows with L^d: confirmed first-order
  e. Compute latent heat: Q = ΔE × density of states at transition

Step 5: Thermodynamic extrapolation
  a. For transition location: g_c(L) = g_c(∞) + a/L^{1/ν} (second-order)
                          or g_t(L) = g_t(∞) + a/L^d (first-order)
  b. For order parameter jump: Δψ(L) → Δψ(∞) as L → ∞
  c. For photon density: ⟨n_ph⟩/N should converge to finite value in SRC

Step 6: Coexistence verification
  At the center of the predicted SRC region:
  a. Confirm |ψ_clock| does NOT vanish as N → ∞ (extrapolate from 3+ sizes)
  b. Confirm ⟨n_ph⟩/N does NOT vanish as N → ∞
  c. If both survive: SRC is genuine
  d. If either vanishes: SRC may be finite-size artifact at this parameter point

Step 7: Phase boundary mapping
  Repeat Steps 1-5 along multiple parameter paths to trace the full SRC boundary
  in the (Δ/Ω, g/Ω) plane
```

### 3.2 Convergence Criteria

| Test | Criterion for genuine SRC phase |
|------|------|
| Clock order | |ψ_clock|(N) extrapolates to nonzero value as N → ∞ |
| Photon density | ⟨n_ph⟩/N remains finite (not vanishing) with increasing N |
| Binder cumulant | Clean crossing or consistent first-order dip across 3+ sizes |
| Data collapse | Reduced χ² < 2 for scaling collapse with physical exponents |
| Consistency | Exact diag and QMC agree within error bars at shared system sizes |

### 3.3 Failure Modes

| Observation | Interpretation | Action |
|---|---|---|
| ψ_clock vanishes for all N > 18 | SRC may not survive at larger sizes | Extend to N=36 QMC, check for slow convergence |
| No Binder crossing | Crossover, not a transition | Report as crossover, not phase boundary |
| Exponents differ from all known classes | Novel universality or insufficient system sizes | Add N=48 QMC data, compare with field theory |
| Exact diag and QMC disagree | Systematic error in one method | Debug: check photon truncation convergence, check QMC thermalization |

---

## 4. Mincut Phase Detection Validation

### 4.1 The Claim

Doc 02 (RuVector Phase Memory) claims that the minimum cut in a graph of parameter-space points, weighted by observable similarity, detects the physical phase boundary. This section provides the proof sketch and validation protocol.

### 4.2 Graph Construction

**Nodes:** Each explored parameter point p_i = (Δ_i, g_i) with measured observables O_i = (ψ_clock, ⟨n_ph⟩, S(Q), ...).

**Edges:** Connect points within a local neighborhood (e.g., k-nearest neighbors in parameter space, typically k = 6–10 for a 2D parameter grid).

**Edge weights:** Observable similarity:

```
w(i, j) = exp(-||O_i - O_j||² / (2σ²))

where:
  ||O_i - O_j||² = Σ_α (O_i^α - O_j^α)² / (var(O^α))   (normalized by variance)
  σ = bandwidth parameter (controls edge weight decay)
```

**Properties:**
- Within a single phase, neighboring points have similar observables → large edge weights
- Across a phase boundary, observables change rapidly → small edge weights between boundary-crossing pairs
- The minimum cut naturally separates nodes in different phases

### 4.3 Why Mincut Detects Phase Boundaries

**Theorem (informal):** If the observable vector O(g) is a smooth function of the control parameter g within each phase, and has a discontinuity (first-order) or a non-analyticity (second-order) at g = g_c, then the minimum cut of the weighted graph converges to the set of edges crossing g = g_c as the grid resolution increases.

**Proof sketch:**

1. Within a phase, neighboring points have O_i ≈ O_j, so w(i,j) ≈ 1 (large weight).
2. Across the boundary at g_c, there exists a pair (i,j) with i on one side, j on the other, where ||O_i - O_j|| is large → w(i,j) ≈ 0 (small weight).
3. A cut through the boundary has total weight ≈ (number of boundary-crossing edges) × exp(-Δ²/2σ²), where Δ is the observable jump.
4. Any cut NOT through the boundary must sever edges within a phase, each with weight ≈ 1, yielding much larger total weight.
5. Therefore the minimum cut passes through the boundary.

**For first-order transitions:** The observable jump Δ is finite and grows with system size → mincut weight decreases exponentially with N → clean detection.

**For second-order transitions:** Observables are continuous but derivatives diverge → mincut weight decreases as a power law → detection requires sufficient grid resolution.

**For crossovers:** No true non-analyticity → mincut weight does not decrease systematically with grid resolution → mincut will find a "best guess" boundary but it will shift with resolution.

### 4.4 Validation Protocol

```
PROTOCOL: Mincut Validation Against Known Phase Boundaries

Input:
  - Phase diagram computed by standard methods (exact diag or QMC)
    with phase labels at each grid point
  - Same grid with observables stored in RuVector

Step 1: Compute reference boundary
  From standard classification:
    boundary_ref = set of grid cells where adjacent cells have different phase labels

Step 2: Compute mincut boundary
  From RuVector coherence engine:
    Build weighted graph from observable vectors
    Compute minimum cut
    boundary_mincut = set of edges in the minimum cut

Step 3: Compare
  Metrics:
    a. Hausdorff distance between boundary_ref and boundary_mincut
       (should be < grid spacing for successful detection)
    b. Precision: fraction of mincut edges that cross the real boundary
       (target: > 0.9)
    c. Recall: fraction of real boundary edges found by mincut
       (target: > 0.8)

Step 4: Sensitivity analysis
  Vary σ (bandwidth) and k (neighbor count):
    a. σ too small → many spurious cuts within phases
    b. σ too large → boundary edges indistinguishable from interior edges
    c. Optimal σ: ≈ median observable difference between neighboring points within a phase

Step 5: Scaling test
  Repeat at increasing grid resolution (50×50, 100×100, 200×200):
    a. Boundary location should converge
    b. For first-order: mincut weight should decrease exponentially with resolution
    c. For second-order: mincut weight should decrease as power law

Step 6: First-order vs. second-order classification
  The mincut weight w_cut encodes transition information:
    w_cut ∝ exp(-Δ²/2σ²)     → first-order (Δ = observable jump)
    w_cut ∝ 1/L^{d-2+η}      → second-order (η = anomalous dimension)
    w_cut ≈ constant           → crossover

  By measuring w_cut at multiple resolutions, classify the transition order
  without reference to the standard analysis.
```

### 4.5 Expected Results for the SRC System

Based on the Ginzburg-Landau analysis (doc 08):

| Boundary | Transition Order | Mincut Behavior |
|----------|-----------------|-----------------|
| Normal → Clock | Second-order (Z₃ Potts class) | Power-law weight decay |
| Normal → SR | Second-order (Dicke class) | Power-law weight decay |
| Clock → SRC | Second-order (SR onset within clock) | Power-law weight decay |
| SRC at Z₂ line | First-order | Exponential weight decay |
| SR → SRC | Likely first-order (clock onset with photon jump) | Exponential weight decay |

The first-order transition at the Z₂ line is the cleanest test case for mincut validation: the observable jump is large and sharp, making it an unambiguous target.

### 4.6 Worked Example: N=12 Toy System

To make this concrete, here is the expected workflow for the smallest viable system:

```
System: N=12 triangular cluster, PBC, N_max=8
Grid: 50×50 in (Δ/Ω, g/Ω), total 2500 points
Solver: symmetry-reduced exact diag (~512 dim per sector)
Time: ~25 seconds total (10 ms per point)

Step 1: Compute all 2500 points → store in RuVector
Step 2: Classify phases from observables
Step 3: Build weighted graph (k=6 neighbors, σ auto-calibrated)
Step 4: Compute mincut → compare with direct classification

Expected outcome at N=12:
  - Clock boundary: detectable but broad (finite-size rounding)
  - SR boundary: detectable, moderate width
  - SRC region: may be small or absent at N=12
  - Z₂ first-order line: may not be clearly first-order at N=12
    (need N=18,24 to confirm)

This establishes the baseline. N=18 and N=24 should show:
  - Sharpening boundaries
  - Emerging SRC region
  - Clear first-order signal at Z₂ line
```

---

## 5. Sheaf Laplacian Analysis of Phase Transitions

### 5.1 Construction

The sheaf Laplacian extends the standard graph Laplacian by assigning vector-valued data (stalks) to nodes and linear maps (restriction maps) to edges:

**Stalk at node i:** The observable vector O_i ∈ R^d (d = number of observables).

**Restriction map on edge (i,j):** The identity map (trivial sheaf) — or a rotation matrix if observables transform non-trivially under symmetry.

**Sheaf Laplacian:**

```
L_sheaf = Σ_{(i,j)∈E} w(i,j) × (O_i - R_{ij} O_j)^T (O_i - R_{ij} O_j)
```

For the trivial sheaf (R_{ij} = I):

```
L_sheaf = Σ_{(i,j)∈E} w(i,j) × ||O_i - O_j||²
```

**Spectrum:** The eigenvalues of L_sheaf encode the coherence of the observable field over the parameter-space graph.

### 5.2 Phase Transition Signatures

| Eigenvalue behavior | Interpretation |
|---|---|
| Small gap (λ₁ → 0) | Near-degenerate observable configurations → approaching transition |
| Large gap (λ₁ >> 0) | Well-separated phases → deep in a single phase |
| Eigenvector localization | Eigenvector concentrated near boundary → boundary detection |
| Multiple small eigenvalues | Multiple distinct phases meeting → multicritical point |

### 5.3 Advantage over Mincut

The sheaf Laplacian provides richer information than mincut alone:

- **Mincut:** binary partition (which side of the boundary?)
- **Sheaf spectrum:** continuous measure of transition sharpness
- **Sheaf eigenvectors:** spatial structure of the boundary
- **Multiple eigenvalues:** detect multicritical points where three or more phases meet

For the SRC system, the multicritical point where Normal, Clock, SR, and SRC phases meet should appear as a point with multiple small sheaf Laplacian eigenvalues.

---

## 6. Integration with ruQu Phase Scanner

The finite-size scaling protocol and mincut validation integrate with the phase scanner (doc 01, section 2.5) as follows:

```
phase_scan(system, grid) → PhaseDiagram
  ↓
finite_size_repeat(system, sizes=[12,18,24], grid) → [PhaseDiagram × 3]
  ↓
scaling_analysis(diagrams) → ScalingReport
  ├── boundary_locations(L) → g_c extrapolation
  ├── binder_cumulants(L) → crossing / dip analysis
  ├── order_parameter_scaling(L) → exponent extraction
  └── coexistence_check(L) → SRC survival test
  ↓
mincut_validation(diagrams) → MinCutReport
  ├── boundary_comparison(ref, mincut) → Hausdorff distance
  ├── precision_recall → P/R metrics
  ├── weight_scaling → transition order classification
  └── sheaf_spectrum → multicritical detection
```

**Output:** A publication-quality phase diagram with:
- Phase boundaries extrapolated to thermodynamic limit
- Error bars from finite-size scaling uncertainty
- Transition order classification for each boundary segment
- Mincut validation confirming structural detection accuracy
- All results witness-logged in RuVector

---

*This document defines the finite-size scaling protocol and mincut validation for the Rydberg-cavity ruQu module.*
*Series index: [00-superradiant-clock-phase.md](00-superradiant-clock-phase.md)*
