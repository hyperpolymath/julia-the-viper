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

-- Build all proofs
lean_lib JtvAll where
  roots := #[`JtvCore, `JtvTheorems]
