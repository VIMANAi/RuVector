# ADR-084: Critical Issues Audit — Open Bugs, Broken Packages, and Technical Debt

**Status**: Accepted
**Date**: 2026-03-06
**Authors**: RuVector Team
**Deciders**: ruv

## 1. Context

A review of all open GitHub issues (38 open as of 2026-03-06) reveals a pattern of broken npm packages, missing binaries, placeholder code shipped as production, and accumulating technical debt. This ADR categorizes the critical issues by severity, identifies systemic root causes, and prioritizes remediation.

## 2. Critical Issues (P0 — Broken/Unusable Published Packages)

These are packages users can `npm install` but that **do not work**.

### 2.1 ruvllm-wasm: Published Placeholder (#238)

**Package**: `@ruvector/ruvllm-wasm@0.1.0`
**Problem**: Contains only TypeScript stubs. `loadModel()` logs a message and returns a placeholder. `generate()` returns `"[Placeholder - build ruvllm-wasm crate for actual inference]"`. No WASM binary compiled or included.
**Impact**: Developers integrating into browser apps discover at runtime that inference is fake.
**Fix**: Either compile `crates/ruvllm-wasm` via `wasm-pack` and include the binary, or unpublish/deprecate the package with a clear warning.

### 2.2 ruvllm SIMD: Garbled Output (#103)

**Package**: `@ruvector/ruvllm@0.2.4`
**Problem**: SIMD inference produces garbled, unusable output (`=xmanybyon=...`). The `stats` command throws `Cannot read properties of undefined`. Package advertises "Self-learning LLM orchestration with SIMD inference" but does not produce coherent text.
**Impact**: Users expect a working local LLM; get garbage output.
**Fix**: Either implement real inference with a proper tokenizer/model or clarify that ruvllm is an orchestration layer (not a standalone LLM).

### 2.3 rvdna: All Native Binaries Missing (#165)

**Package**: `@ruvector/rvdna@0.1.1`
**Problem**: Declares 5 platform-specific optional dependencies — none are published to npm. `isNativeAvailable()` always returns `false`. Native-only functions (`fastaToRvdna`, `readRvdna`) throw on every platform.
**Impact**: The `.rvdna` binary format is inaccessible from Node.js.
**Fix**: Publish platform binaries via NAPI-RS CI, or remove native-only APIs from the public interface until binaries ship.

### 2.4 ONNX Embeddings: Hardcoded Dimension + Crash (#237)

**Package**: `examples/onnx-embeddings`
**Problem**: Dimension hardcoded to 384 (wrong for `all-mpnet-base-v2` which outputs 768). Unconditionally passes `token_type_ids` input — crashes on models that don't accept it.
**Impact**: Only works with models that happen to match 384 dimensions and accept `token_type_ids`.
**Fix**: Detect dimension from model output shape; conditionally include `token_type_ids` based on `input_names`.

### 2.5 rvf-node: Compression Silently Ignored (#225)

**Package**: `@ruvector/rvf-node@0.1.7`
**Problem**: TypeScript SDK accepts `compression: 'scalar'` in `RvfOptions` and correctly maps it via `mapCompressionToNative()`. But the N-API binding in Rust never reads the compression field — `..Default::default()` drops it silently.
**Impact**: Users configure compression expecting 4x memory savings; get zero compression. No error or warning.
**Fix**: Add `compression` field to the Rust `RvfOptions` struct in `rvf-node/src/lib.rs` and pass it through.

## 3. High Severity Issues (P1 — Build/Install Failures)

### 3.1 ARM64 Binaries Missing for attention + gnn (#110)

**Packages**: `@ruvector/attention@0.1.4`, `@ruvector/gnn@0.1.22`
**Problem**: ARM64 platform binaries not published in sync with main package releases. Every other version is missing ARM64 builds. Fails on Codespaces, Docker DevContainers on M1/M2 Macs, AWS Graviton.
**Fix**: Fix CI to publish ARM64 binaries atomically with x64 builds. Consider gating main package publish on successful platform builds.

### 3.2 Misconfigured rvf Cargo.toml (#214)

**Problem**: `cargo install --git https://github.com/ruvnet/ruvector ruvllm-cli` fails with `package is a member of the wrong workspace`. `rvf-crypto/Cargo.toml` references the wrong workspace root.
**Impact**: Cannot install any crate from the repo via `cargo install --git`.
**Fix**: Fix workspace membership in `crates/rvf/Cargo.toml` or use proper `exclude` patterns.

## 4. Security Issues (P1)

### 4.1 Edge-Net: 5 Critical Security Flaws (#95)

**Location**: `examples/edge-net/`
**Grade**: ~60% production ready (B+)
**Critical findings**:
1. Weak PBKDF in Pi-Key — simple SHA-256 iteration instead of Argon2id/scrypt
2. Private key exposure — `export_secret_key()` returns raw key bytes
3. Signature verification unimplemented — `verify_event_signature()` returns `true` always
4. Session key derivation weakness
5. Forged events accepted as valid

**Status**: Marked as example code, but could be used as a starting point for production systems.

## 5. Technical Debt (P2)

### 5.1 Quality Engineering Findings (#56)

| Category | Count | Risk |
|----------|-------|------|
| `.unwrap()` calls | 725 | Potential panics in production |
| TODO/FIXME markers | 415 | Unfinished work scattered across codebase |
| Undocumented `unsafe` blocks | 277 | Difficult to audit for soundness |
| Duplicated distance functions | 800 lines | Maintenance burden, divergence risk |
| Cyclomatic complexity (Raft node.rs) | 19.3 | Unmaintainable hot path |
| ruvector-server tests | 0 | API users on untested code |
| ruvector-node tests | 0 | npm users on untested code |

### 5.2 AVX-512 Fallback (#47)

AVX-512 code paths exist in `crates/ruvector-postgres/src/distance/simd.rs` but fall back to AVX2 intrinsics. Potential 2-3x speedup left on the table for supported hardware.

## 6. Systemic Root Causes

| Pattern | Issues | Root Cause |
|---------|--------|------------|
| **Placeholder packages published** | #238, #103 | No pre-publish validation that core functionality works |
| **Missing platform binaries** | #165, #110 | NAPI-RS CI publishes x64 but skips ARM64 on some releases |
| **Silent option ignoring** | #225 | N-API binding structs not kept in sync with TypeScript types |
| **Workspace misconfiguration** | #214 | Nested workspace (`crates/rvf/`) conflicts with root workspace |
| **Hardcoded assumptions** | #237, #47 | Model-specific values (384 dim, AVX2) baked in instead of detected |

## 7. Recommended Priority Order

### Immediate (P0 — Stop the bleeding)

1. **Deprecate or fix ruvllm-wasm** (#238) — npm deprecate with warning, or compile WASM binary
2. **Deprecate or fix ruvllm SIMD** (#103) — clarify it's an orchestrator, not a standalone LLM
3. **Fix rvf-node compression** (#225) — add field to Rust struct (small, precise fix)
4. **Fix ONNX dimension/inputs** (#237) — conditional token_type_ids + dynamic dimension

### Short-term (P1 — Build reliability)

5. **Fix ARM64 CI** (#110) — gate main publish on all platform builds
6. **Publish rvdna binaries** (#165) — run NAPI-RS CI for rvdna
7. **Fix Cargo workspace** (#214) — resolve nested workspace conflict

### Medium-term (P2 — Quality)

8. **Reduce `.unwrap()` count** (#56) — prioritize hot paths (server, node)
9. **Add server/node tests** (#56) — critical coverage gap
10. **Edge-Net security** (#95) — fix signature verification, replace PBKDF

## 8. Metrics

| Metric | Value |
|--------|-------|
| Total open issues | 38 |
| P0 (broken packages) | 5 |
| P1 (build/security) | 3 |
| P2 (technical debt) | 4 |
| Enhancement requests | 15 |
| Documentation | 2 |
| External/feature requests | 9 |
