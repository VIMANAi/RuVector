# Full Hamiltonian Derivation and Clock Term Competition

## The Mathematics of the Superradiant Clock Phase

---

## 1. Microscopic Hamiltonian

### 1.1 Full System Hamiltonian

The complete Hamiltonian for Rydberg atoms on a triangular lattice coupled to a single-mode optical cavity:

```
H = H_atom + H_cavity + H_coupling + H_drive
```

Each term:

**Atomic part** (Rydberg blockade interactions):

```
H_atom = Σ_{<i,j>} V_ij n_i n_j + Σ_{<<i,j>>} V'_ij n_i n_j

where:
  n_i = |r_i⟩⟨r_i|                   (Rydberg occupation number operator)
  V_ij = C₆ / |r_i - r_j|⁶          (van der Waals interaction)
  <i,j>  = nearest neighbors on triangular lattice
  <<i,j>> = next-nearest neighbors
```

For the triangular lattice with lattice constant a:
```
V_nn  = C₆ / a⁶                       (nearest neighbor)
V_nnn = C₆ / (√3 · a)⁶ = V_nn / 27    (next-nearest neighbor)
```

**Cavity part:**

```
H_cavity = ω_c a†a

where:
  ω_c = cavity resonance frequency
  a†, a = photon creation/annihilation operators
  [a, a†] = 1
```

**Light-matter coupling** (Jaynes-Cummings type, rotating wave approximation):

```
H_coupling = (g / √N) Σ_i (a† σ_i⁻ + a σ_i⁺)

where:
  g = single-atom vacuum Rabi frequency
  σ_i⁺ = |r_i⟩⟨g_i|    (raising operator: ground → Rydberg)
  σ_i⁻ = |g_i⟩⟨r_i|    (lowering operator: Rydberg → ground)
  1/√N prefactor = Dicke normalization for collective coupling
```

**Coherent drive:**

```
H_drive = Ω Σ_i (σ_i⁺ + σ_i⁻) - Δ Σ_i n_i

where:
  Ω = Rabi frequency of external drive
  Δ = detuning from atomic resonance
```

### 1.2 Dimensionless Form

Rescale all energies by Ω:

```
H/Ω = (V_nn/Ω) Σ_{<i,j>} n_i n_j
     + (ω_c/Ω) a†a
     + (g/Ω√N) Σ_i (a† σ_i⁻ + a σ_i⁺)
     + Σ_i (σ_i⁺ + σ_i⁻)
     - (Δ/Ω) Σ_i n_i
     + (V_nnn/Ω) Σ_{<<i,j>>} n_i n_j
```

**Control parameters:**
```
δ = Δ/Ω          (dimensionless detuning)
λ = g/(Ω√N)      (dimensionless collective coupling, or g/Ω)
ν = V_nn/Ω       (dimensionless interaction strength)
κ̃ = κ/g          (cavity decay relative to coupling)
```

The phase diagram is explored in the (δ, λ) plane at fixed ν and κ̃.

---

## 2. Effective Hamiltonian After Adiabatic Elimination

### 2.1 Bad-Cavity Limit (κ >> g)

When the cavity decay rate κ is much larger than the coupling g, the photon field adiabatically follows the atomic state. Eliminating the photon mode:

```
a_ss = -(g/√N) Σ_i σ_i⁻ / (iΔ_c + κ/2)

where Δ_c = ω_c - ω_drive (cavity-drive detuning)
```

Substituting back into H gives an effective atom-only Hamiltonian with an additional infinite-range interaction:

```
H_eff = H_atom + H_drive + H_cavity-mediated

H_cavity-mediated = -(g²/N) × (Δ_c / (Δ_c² + κ²/4)) × (Σ_i σ_i⁺)(Σ_j σ_j⁻)
                  = -J_∞ × (Σ_i σ_i⁺)(Σ_j σ_j⁻)
```

This is an **infinite-range XX coupling** between all atom pairs, mediated by virtual photon exchange. The coupling strength:

```
J_∞ = g² Δ_c / [N (Δ_c² + κ²/4)]
```

is attractive (ferromagnetic) when Δ_c > 0 (red-detuned cavity) and repulsive (antiferromagnetic) when Δ_c < 0 (blue-detuned).

### 2.2 Good-Cavity Limit (κ << g)

When the cavity has low losses, the photon mode cannot be adiabatically eliminated. The full Hamiltonian including the cavity Fock space must be treated. This is the regime where the SRC phase appears — the photon field is a dynamical degree of freedom, not a passive mediator.

In this limit, the cavity-mediated interaction retains quantum correlations:

```
H_full = H_atom + ω_c a†a + (g/√N) Σ_i (a† σ_i⁻ + a σ_i⁺) + H_drive
```

The photon number ⟨a†a⟩ becomes an order parameter in its own right.

---

## 3. Mapping to the Frustrated Lattice Gas

### 3.1 Hard-Core Boson Representation

The Rydberg blockade constraint (at most one excitation per blockade radius) maps the system to a constrained lattice gas. On the triangular lattice at half-filling, the relevant degree of freedom is the sublattice occupation.

The triangular lattice has three sublattices (A, B, C). At filling f = 1/3, one sublattice is preferentially occupied. At f = 1/2, the system is frustrated: two of three sublattices are occupied, and the ground state is massively degenerate.

### 3.2 Classical Ground-State Degeneracy

For antiferromagnetic nearest-neighbor interactions on the triangular lattice at half-filling, the number of classical ground states grows exponentially with system size:

```
W(N) ∝ exp(S₀ × N)

where S₀ ≈ 0.3383 per site (Wannier, 1950)
```

This is the residual entropy of the triangular Ising antiferromagnet. At N = 24, the number of degenerate ground states is approximately:

```
W(24) ≈ e^(0.3383 × 24) ≈ e^8.12 ≈ 3,366
```

This massive degeneracy is what frustration creates, and what the cavity field lifts.

---

## 4. Ginzburg-Landau Theory and Clock Terms

### 4.1 Order Parameters

The SRC phase is described by two coupled order parameters:

**Clock order parameter** (complex scalar):

```
ψ = |ψ| e^{iθ}

where θ takes discrete values θ = 2πk/p for k = 0, 1, ..., p-1
p = 3 for Z₃ clock (sublattice ordering on triangular lattice)
p = 6 for Z₆ clock (finer structure)
```

In terms of sublattice densities n_A, n_B, n_C:

```
ψ = (1/N)(n_A + ω n_B + ω² n_C)

where ω = e^{2πi/3} (cube root of unity)
```

**Superradiant order parameter** (real scalar):

```
Φ = ⟨a⟩ / √N

Nonzero Φ signals macroscopic cavity field occupation.
```

### 4.2 Ginzburg-Landau Free Energy

The free energy in terms of ψ and Φ:

```
F[ψ, Φ] = F_clock[ψ] + F_SR[Φ] + F_coupling[ψ, Φ]
```

**Clock part:**

```
F_clock = r|ψ|² + u|ψ|⁴ + v₃(ψ³ + ψ*³) + v₆(ψ⁶ + ψ*⁶) + ...

where:
  r = reduced temperature / detuning (controls ordering)
  u = quartic interaction
  v₃ = threefold clock anisotropy (Z₃ symmetry breaking)
  v₆ = sixfold clock anisotropy (Z₆ symmetry breaking)
```

The key terms are v₃ and v₆. Their competition determines the nature of the clock phase:

- |v₃| >> |v₆|: strong Z₃ locking → three-state clock phase
- |v₃| << |v₆|: weak Z₃, strong Z₆ → six-state clock or quasi-continuous
- v₃ = 0: Z₂ symmetry line (the clock is symmetric under θ → -θ)

**Superradiant part:**

```
F_SR = r_SR Φ² + u_SR Φ⁴

where:
  r_SR ∝ (g_c - g)     (distance from superradiant transition)
  g_c = critical coupling strength
```

**Coupling between clock and superradiance:**

```
F_coupling = α Φ (ψ³ + ψ*³) + β Φ² |ψ|² + ...
```

**This is the crucial term.** The coupling α Φ (ψ³ + ψ*³) means:

1. Nonzero photon field Φ ≠ 0 acts as an explicit symmetry-breaking field for the Z₃ clock term
2. The photon field selects a specific clock orientation
3. This is NOT order-by-disorder (fluctuation selection) — it is **direct symmetry breaking by the cavity field**
4. The selection is robust because it comes from the order parameter coupling, not from the fluctuation spectrum

### 4.3 Phase Diagram from Ginzburg-Landau

Minimizing F[ψ, Φ] yields four phases:

```
Phase        |  |ψ|  |  Φ  |  Condition
─────────────┼───────┼──────┼──────────────────────
Normal       |   0   |  0   |  r > 0 and r_SR > 0
Clock        |  > 0  |  0   |  r < 0 and r_SR > 0
Superradiant |   0   | > 0  |  r > 0 and r_SR < 0
SRC          |  > 0  | > 0  |  r < 0 and r_SR < 0
```

The SRC phase requires BOTH r < 0 (clock ordering favored) AND r_SR < 0 (superradiance favored), plus the coupling α that locks them together.

### 4.4 First-Order Transition at Z₂ Line

When v₃ = 0 (the Z₂ symmetry line), the Ginzburg-Landau free energy has an additional Z₂ symmetry (θ → -θ). Crossing this line:

- The coupling term α Φ (ψ³ + ψ*³) = 2α Φ |ψ|³ cos(3θ) changes sign
- This forces a discontinuous jump in θ (from one clock sector to the opposite)
- Simultaneously, Φ exhibits a discontinuity (jumps with the clock order)
- The transition is first-order because both order parameters jump

**Observable signature:** Bimodal distribution of ψ at the transition, discontinuous ⟨n_ph⟩, latent heat in QMC simulations.

---

## 5. Ring Exchange Interactions

### 5.1 Origin

The paper [P1] identifies cavity-mediated **nonlocal ring exchange** as critical. On the triangular lattice, a ring exchange involves flipping all spins around an elementary triangle simultaneously:

```
Ring exchange on triangle (i,j,k):

K_ring (|↑↓↑⟩⟨↓↑↓| + h.c.)  on triangle (i,j,k)
```

The cavity mediates this process through virtual photon exchange:

```
|g_i r_j g_k⟩ ⟨r_i g_j r_k| via intermediate cavity states
```

### 5.2 Effective Hamiltonian with Ring Exchange

```
H_eff = H_Ising + H_ring + H_cavity

H_Ising = V Σ_{<i,j>} n_i n_j - Δ Σ_i n_i

H_ring = K Σ_{△} (P_△ + P_△†)

where:
  P_△ = ring permutation operator on triangle △
  K ∝ g² / (ω_c - Δ)   (cavity-mediated ring exchange strength)
```

The ring exchange K competes with the direct Ising interaction V. When K is comparable to V, the phase diagram becomes rich, and the SRC phase emerges from this competition.

### 5.3 Dimer Language

At half-filling on the triangular lattice, each ground-state configuration maps to a close-packed dimer covering. The cavity-mediated ring exchange corresponds to dimer resonance:

```
| ═ |   ↔   | | |     (horizontal dimer ↔ two vertical dimers)
| | |       | ═ |     around a hexagonal plaquette
```

The SRC phase corresponds to a specific dimer ordering (columnar or staggered) selected by the cavity coupling, coexisting with macroscopic photon occupation.

---

## 6. Numerical Implementation Notes

### 6.1 Hamiltonian Construction in ruQu

The Hamiltonian builder must construct H as a sparse matrix in a symmetry-adapted basis.

**Step 1: Enumerate basis states**

```
For N atoms with photon cutoff N_max:
  basis = { |s₁ s₂ ... s_N; n_ph⟩ : s_i ∈ {g, r}, n_ph ∈ {0, ..., N_max} }
  subject to optional constraints:
    - blockade: n_i n_j = 0 for r_ij < r_blockade
    - filling: Σ n_i = N_fill
    - symmetry sector: state transforms as irrep ρ under lattice symmetry
```

**Step 2: Apply Hamiltonian terms**

For each basis state |φ⟩:
```
H_Ising |φ⟩ = (Σ V_ij n_i n_j - Δ Σ n_i) |φ⟩           (diagonal)
H_cavity |φ⟩ = ω_c n_ph |φ⟩                                (diagonal)
H_coupling |φ⟩ = (g/√N) Σ_i [√(n_ph+1) |flip_i_up; n_ph+1⟩
                              + √n_ph |flip_i_down; n_ph-1⟩]  (off-diagonal)
H_drive |φ⟩ = Ω Σ_i [|flip_i⟩]                              (off-diagonal)
```

**Step 3: Symmetry projection**

Project into momentum sector k and point group irrep ρ:

```
P_{k,ρ} = (1/|G|) Σ_{g∈G} χ_ρ(g)* × exp(-ik·t_g) × T_g

where T_g is the symmetry operation, χ_ρ its character, t_g its translation
```

### 6.2 Observable Extraction

After solving for the ground state |Ψ₀⟩:

```
⟨n_ph⟩ = ⟨Ψ₀| a†a |Ψ₀⟩ = Σ_{n=0}^{N_max} n × |⟨n|Ψ₀⟩|²

ψ_clock = (1/N) Σ_i ⟨Ψ₀| n_i |Ψ₀⟩ × ω^{sub(i)}

where sub(i) ∈ {0, 1, 2} is the sublattice index of site i
and ω = e^{2πi/3}

S(Q) = (1/N) |Σ_i ⟨Ψ₀| n_i |Ψ₀⟩ × e^{iQ·r_i}|²
```

For the SRC phase:
- |ψ_clock| should be O(1) (not vanishingly small)
- ⟨n_ph⟩/N should be O(0.01–0.1) or larger
- S(Q) should peak at the ordering wavevector Q = K or K' point of the Brillouin zone

---

## 7. Connection to ruQu Observables Module

The Hamiltonian derivation directly maps to the ruQu observable pipeline (doc 01, section 2.4):

| GL Parameter | ruQu Observable | Measurement Method |
|-------------|----------------|-------------------|
| |ψ| (clock magnitude) | `measure_clock_order` | Sublattice magnetization |
| θ (clock angle) | `arg(measure_clock_order)` | Phase of complex ψ |
| Φ (SR order) | `measure_photon_number → sqrt(⟨n_ph⟩/N)` | Photon sector projection |
| v₃/v₆ ratio | Structure factor peak analysis | Compare S(Q) at K and M points |
| K (ring exchange) | Energy difference between dimer orderings | Exact diag in dimer sector |

The GL free energy parameters (r, u, v₃, v₆, r_SR, α) can be extracted numerically by fitting the GL form to computed observables across the parameter range. This is how ruQu validates the Ginzburg-Landau description of [P1].

---

*This document provides the full Hamiltonian derivation and clock term competition analysis.*
*Series index: [00-superradiant-clock-phase.md](00-superradiant-clock-phase.md)*
