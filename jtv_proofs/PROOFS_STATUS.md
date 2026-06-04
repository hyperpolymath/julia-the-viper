<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
-->

# JtV Proof Status

A mechanised formalisation of Julia the Viper's v1 + v2 semantics in **Lean 4.12.0**.
Build oracle: `lake build` — must complete with zero `sorryAx`, zero `Classical.choice`.

## Headline

| Metric | Value |
|---|---|
| Audited theorems / definitions | **164** |
| Depend on no axioms | 42 |
| Depend on `propext` only | 75 |
| Depend on `propext + Quot.sound` | 47 |
| Depend on `Classical.choice` | **0** |
| Depend on `sorryAx` | **0** |
| Lean libraries (`lean_lib`) | 12 |

The build oracle `JtvAxiomAudit.lean` prints axiom dependencies for every
headline theorem; any regression to `sorryAx` or `Classical.choice` is a
hard failure.

## Modules

| Module | Topic | EBNF anchor |
|---|---|---|
| `JtvCore` | Semantic domains, evaluation, AST | `spec/grammar.ebnf:7-44` |
| `JtvTheorems` | Totality, security, algebra | – |
| `JtvOperational` | Small-step + big-step Data semantics | – |
| `JtvTypes` | 7 number-system types, purity lattice, type soundness | `spec/grammar.ebnf:99-106` |
| `JtvSecurity` | Code-injection impossibility, OWASP-aligned | `spec/grammar.ebnf:153-163` |
| `JtvExtended` | Cancellation, semantic equivalence, optimisation | – |
| `JtvReversibility` | v2 reverse blocks: bijection, inverses, RevTyping | `spec/grammar.ebnf:40-45, 178-185` |
| `JtvPurity` | v2 `@pure` / `@total` compositionality + lattice | `spec/grammar.ebnf:86-88, 96-97` |
| `JtvControlSemantics` | v2 `print` / `reverseBlock` trace soundness | `spec/grammar.ebnf:22, 36, 41` |
| `JtvCalls` | v2 function calls in Data context, Pure Function Rule | `spec/grammar.ebnf:54-60, 93-97` |
| `JtvBool` | v2 Bool sublanguage (pure / total / decidable) | `spec/grammar.ebnf:67-79` |
| `JtvTotalSemantics` | v2 fuel-free `totalExec` + execStmt bridge (Δ5) | – |
| `JtvAxiomAudit` | Build oracle — `#print axioms` for every headline | – |

## v2 Grammar Coverage by Δ-slice

The v2 grammar adds reversible blocks, IO, purity stratification, and Bool
expressions on top of v1. Each Δ-slice is formalised against the EBNF:

- **Δ1** — `print` + `reverseBlock` constructors → `JtvCore.ControlStmt`
- **Δ3** — Operational semantics for print/reverseBlock → `JtvControlSemantics`
- **Δ4** — Bool sublanguage → `JtvBool`
- **Δ5** — Fuel-free total semantics + bridge → `JtvTotalSemantics`
  - `execStmt_eq_totalExec_when_some` (soundness)
  - `execStmt_terminates_for_total` (completeness, via fuel monotonicity)
- **Δ6 / Δ7** — RevTyping inductive judgment + reversibility for typed RevStmts → `JtvReversibility`
- **PFR** — Pure Function Rule for Data-context calls → `JtvCalls`
  - `DataExprC.respectsPureFnRule_excludes_impure` (headline)
  - `DataExprC.pureFnRule_no_io` (well-formed + PFR ⇒ no IO)

## Open Work

**Mid-priority (real EBNF surface, currently unmodelled):**

- Function-call **evaluation** semantics — `DataExprC.call` is structurally typed
  but has no `evalDataExprWithEnv` evaluator yet
- `module_decl` + `import_stmt` — namespace + program scaffolding
- `for_stmt`, `return_stmt`
- Control-expression layer: `comparison_expr`, `logical_expr` (`==`, `<`, `&&`, `||`)
  beyond the partial Bool surface in `JtvBool`
- DataExprC: lift `List DataExpr` args to `List DataExprC` for nested calls

**Low-priority (advertised but vacuous):**

- 7-number-system tower: `JtvType` declares Rational / Complex / Hex / Binary / Symbolic
  but `evalDataExpr` only returns `Int`
- List / tuple / string literals

**Documentation gap:**

- `academic/proofs/QUANTUM_REVERSIBILITY.md:591` flags nested
  `reverse { reverse { … } }` blocks as TODO — `ReversibleStmt` is structurally
  capable but no headline theorem yet.

## Echo-types audit

Per estate convention, every proof slice audits the canonical
`hyperpolymath/echo-types` library before introducing new typing infrastructure.
**Status for this slice**: L3 echo obligations recorded as **not relevant** —
JtV proofs are local to its own Harvard-Architecture model and do not yet
participate in cross-repo echo typing.

## Running the oracle

```bash
cd jtv_proofs
lake build
```

A clean run prints axiom dependencies for every theorem in `JtvAxiomAudit`;
the build fails if any `sorryAx` or unexpected axiom appears.
