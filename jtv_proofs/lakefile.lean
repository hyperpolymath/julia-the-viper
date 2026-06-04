import Lake
open Lake DSL

package JtvProofs where
  -- Lean 4 package for JtV formal semantics proofs
  leanOptions := #[
    ⟨`pp.unicode.fun, true⟩,
    ⟨`autoImplicit, false⟩
  ]

@[default_target]
lean_lib JtvCore where
  -- Core definitions: semantic domains, evaluation functions
  roots := #[`JtvCore]

lean_lib JtvTheorems where
  -- Theorems: totality, determinism, security, algebraic properties
  roots := #[`JtvTheorems]

lean_lib JtvOperational where
  -- Operational semantics: small-step and big-step
  roots := #[`JtvOperational]

lean_lib JtvTypes where
  -- Type system: typing rules, purity, type soundness
  roots := #[`JtvTypes]

lean_lib JtvSecurity where
  -- Security properties: injection impossibility, sandboxing
  roots := #[`JtvSecurity]

lean_lib JtvExtended where
  -- Extended formal proofs: cancellation, equivalence, optimization, etc.
  roots := #[`JtvExtended]

lean_lib JtvReversibility where
  -- v2 grammar: reverse-block reversibility (single-op + list-level + bijection).
  roots := #[`JtvReversibility]

lean_lib JtvPurity where
  -- v2 grammar: purity_marker (@pure / @total) compositionality and stratification.
  roots := #[`JtvPurity]

lean_lib JtvControlSemantics where
  -- v2 grammar: print / reverseBlock execution semantics + trace soundness.
  roots := #[`JtvControlSemantics]

lean_lib JtvCalls where
  -- v2 grammar: function calls in Data context + Pure Function Rule teeth.
  roots := #[`JtvCalls]

lean_lib JtvTotalSemantics where
  -- v2 grammar: fuel-free total semantics for @total-respecting statements (Δ5).
  roots := #[`JtvTotalSemantics]

lean_lib JtvBool where
  -- v2 grammar: Bool sublanguage — pure, total, decidable (Δ4).
  roots := #[`JtvBool]

lean_lib JtvAxiomAudit where
  -- Build oracle: #print axioms for every headline theorem.
  -- Any `sorryAx` or unexpected axiom appearing here is a regression.
  roots := #[`JtvAxiomAudit]

-- Build all proofs
@[default_target]
lean_lib JtvAll where
  roots := #[`JtvCore, `JtvTheorems, `JtvOperational, `JtvTypes,
             `JtvSecurity, `JtvExtended, `JtvReversibility, `JtvPurity,
             `JtvControlSemantics, `JtvCalls, `JtvTotalSemantics,
             `JtvBool, `JtvAxiomAudit]
