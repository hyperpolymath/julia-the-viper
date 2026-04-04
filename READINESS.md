# READINESS.md — julia-the-viper

<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->

**Current Grade:** C

## Component Readiness Assessment

| Field | Value |
|-------|-------|
| Component | julia-the-viper (JTV) |
| Version | 0.1.0 |
| Date | 2026-04-04 |
| Assessor | Claude (automated) + Jonathan D.A. Jewell |
| CRG Grade | **C+** (approaching B) |
| Release Stage | Alpha (home context) |

## Test Evidence Summary

- **339 passing tests** across 6 test suites
- **41 property-based tests** (proptest) covering all 7 number systems
- **35 security/boundary tests** validating Harvard Architecture separation
- **20 contract tests** verifying addition-only, purity, and reversibility invariants
- **2 fuzz targets** (libfuzzer) for parser safety
- **2 benchmark suites** (parser throughput + interpreter performance)
- **80K Lean 4 proofs** (security, type soundness, algebraic properties)
- **4.7K Idris2 ABI proofs** (dependent type definitions)
- **51 conformance programs** (30 valid + 21 invalid)

## Blitz Matrix (16 Test Categories)

```
         UT   P2P  E2E  BLD  EXE  REF  LCY  SMK  PBT  MUT  FUZ  CTR  REG  CHS  CMP  PRF
jtv-core  v    v    ~    v    v    v    v    v    v    -    v    v    ~    n/a   -    v
```

Legend: v = pass, ~ = partial, - = missing, n/a = not applicable

## 14 Aspect Dimensions

| Aspect | Grade | Notes |
|--------|-------|-------|
| Security | A | Grammatical injection impossibility (Lean proof + 35 tests) |
| Safety | A | Addition-only + purity enforced at type level, no dangerous patterns |
| Functionality | B | 339 tests, 2 conformance gaps (string/range) |
| Performance | B | Benchmarked, baselines not yet tracked in CI |
| Dependability | C | Overflow handled, no crash recovery tests |
| Maintainability | B | Modular crates, deep annotation, `just doctor` |
| Reproducibility | C | flake.nix present, not fully tested |
| Interoperability | C | WASM target exists, FFI untested |
| Portability | C | Linux CI only |
| Observability | C | Debug crate exists |
| Usability | C | LSP exists, untested |
| Versability | D | No version matrix |
| Accessibility | N/A | CLI tool |
| Privacy | N/A | No user data |

## Formal Proof Coverage

| Property | Prover | File | Status |
|----------|--------|------|--------|
| Code injection impossibility | Lean 4 | JtvSecurity.lean | Proven (Theorem 2.3) |
| Type soundness | Lean 4 | JtvTypes.lean | Proven |
| Data language totality | Lean 4 | JtvTheorems.lean | Proven |
| Operational semantics | Lean 4 | JtvOperational.lean | Proven |
| Algebraic properties | Lean 4 | JtvExtended.lean | Proven |
| Harvard Architecture separation | Idris2 | Types.idr | Proven |
| Tropical semiring laws | Idris2 | TropicalSemiring.idr (007/) | Proven |
| CNO identity | Idris2 | CNO.idr (007/) | Proven (0 sorry) |
| Reversibility round-trip | Rust proptest | property_tests.rs | 4 properties |

## Gaps to CRG B

1. Mutation testing (cargo-mutants, >80% kill rate)
2. Fix 2 conformance failures (string in data_expr, for-in range syntax)
3. Version compatibility testing
4. Cross-platform CI (macOS, Windows)
5. Zig FFI integration tests
6. Coverage reporting

## Specialist JTV Properties Covered

| Property | Test Type | Count | Coverage |
|----------|-----------|-------|----------|
| Addition-only invariant | Contract + Grammar | 20 | DataExpr enum exhaustive, grammar rejects */÷/% |
| 7 number systems | Property + Unit | 49+ | All types: commutativity, associativity, identity, inverse |
| Harvard boundary | P2P + Security | 35 | Data cannot contain control, control reads data one-way |
| Reversibility identity | Property + Contract | 8+ | forward+reverse=identity across types and multi-variable |
| Purity lattice | Contract | 7 | Total < Pure < Impure, enforcement tested |
| Code injection impossibility | Security + Lean proof | 10+ | No eval/exec, strings are data, keywords reserved |
| Overflow safety | Property | 4 | Never panics on arbitrary i64 input |
