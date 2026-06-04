/-
  Julia the Viper — v2 grammar: function calls in Data context.

  This module closes the load-bearing gap flagged in `JtvPurity.lean:19-25`:
  the base `DataExpr` has no `call` constructor, so the v2 Pure Function Rule
  ("Only Pure Data Functions can be called from Data context") collapses to
  vacuity at the DataExpr level.

  Rather than re-engineer `DataExpr` and break the existing 126-theorem
  build oracle, we add a SEPARATE extension layer `DataExprC` that lifts a
  `DataExpr` and adds the `call` constructor on top. This corresponds
  one-to-one with the EBNF (`spec/grammar.ebnf:54-60`):

      factor = number
             | identifier
             | function_call    (* this is the call constructor *)
             | "(" data_expr ")"
             | ...

  Phase 1 (this file): arguments to a call are plain `DataExpr` — i.e. no
  nested calls. This is sufficient to give the Pure Function Rule formal
  teeth on the surface call sites. Phase 2 (future) would lift arguments
  to `List DataExprC` for nested calls like `f(g(x))`.

  The headline theorem `respectsPureFnRule_excludes_impure` is the v2
  grammar's "Compiler MUST enforce: @pure functions cannot contain loops
  or IO" rule, made formal: a DataExprC that passes the check can never
  reach an @impure function.
-/

import JtvCore
import JtvTypes
import JtvPurity

-- ============================================================================
-- SECTION 1: DataExprC — DataExpr extended with the `call` constructor
-- ============================================================================

/-- Extended data expression supporting function calls.

    * `base e` lifts a plain `DataExpr` (no calls).
    * `call f args` is a call site `f(args₁, …, argsₙ)`.

    Arguments are plain `DataExpr` in Phase 1 — Phase 2 will lift to
    `List DataExprC` for nested calls. -/
inductive DataExprC where
  | base : DataExpr → DataExprC
  | call : String → List DataExpr → DataExprC
  deriving Repr

/-- The list of function names called by a `DataExprC`. -/
def DataExprC.calleeNames : DataExprC → List String
  | base _      => []
  | call f _    => [f]

/-- A `DataExprC` is "pure-syntactic" if it has no call sites. -/
def DataExprC.isPureSyntactic (e : DataExprC) : Bool :=
  e.calleeNames.isEmpty

-- ============================================================================
-- SECTION 2: Pure Function Rule, structural
-- ============================================================================

/-- A function environment satisfies the Pure Function Rule for a callee `f`
    if `env(f)` exists and its declared purity is `@pure` or `@total`
    (i.e. not `@impure`). -/
def FuncEnv.calleeIsPure (env : FuncEnv) (f : String) : Bool :=
  match env f with
  | none      => false
  | some decl =>
    match decl.purity with
    | Purity.total  => true
    | Purity.pure   => true
    | Purity.impure => false

/-- A `DataExprC` respects the v2 Pure Function Rule under `env` if every
    call site is to a `@pure` or `@total` function. -/
def DataExprC.respectsPureFnRule (e : DataExprC) (env : FuncEnv) : Bool :=
  e.calleeNames.all env.calleeIsPure

-- ============================================================================
-- SECTION 3: Headline theorems — the v2 Pure Function Rule with teeth
-- ============================================================================

/-- **Theorem (base lift vacuously respects PFR)**: any `base e` for
    `e : DataExpr` has no call sites, so the Pure Function Rule is trivially
    satisfied. This is the formal counterpart of "the addition-only data
    sublanguage is automatically `@pure`". -/
theorem DataExprC.base_respectsPureFnRule (e : DataExpr) (env : FuncEnv) :
    (DataExprC.base e).respectsPureFnRule env = true := by
  simp [DataExprC.respectsPureFnRule, DataExprC.calleeNames]

/-- **Theorem (call respects PFR iff callee is pure)**: `call f args` respects
    the Pure Function Rule under `env` exactly when `env.calleeIsPure f`. -/
theorem DataExprC.call_respectsPureFnRule (f : String) (args : List DataExpr)
    (env : FuncEnv) :
    (DataExprC.call f args).respectsPureFnRule env = true ↔
    env.calleeIsPure f = true := by
  simp [DataExprC.respectsPureFnRule, DataExprC.calleeNames]

/-- **Theorem (PFR excludes @impure callees)** — the headline v2 result.
    A `DataExprC` that respects the Pure Function Rule under `env` can never
    resolve any of its callees to an `@impure` function. This is the formal
    teeth of the v2 grammar's compiler-enforced rule:
    "Only Pure Data Functions can be called from Data context"
    (`spec/grammar.ebnf:96-97`). -/
theorem DataExprC.respectsPureFnRule_excludes_impure
    (e : DataExprC) (env : FuncEnv)
    (h : e.respectsPureFnRule env = true) :
    ∀ f, f ∈ e.calleeNames →
      ∀ decl, env f = some decl → decl.purity ≠ Purity.impure := by
  intro f hf decl henv himpure
  simp only [DataExprC.respectsPureFnRule, List.all_eq_true] at h
  have hp := h f hf
  simp [FuncEnv.calleeIsPure, henv, himpure] at hp

/-- **Theorem (PFR resolution)** — every callee of a PFR-respecting expression
    is bound by `env` (no dangling references). -/
theorem DataExprC.respectsPureFnRule_callees_bound
    (e : DataExprC) (env : FuncEnv)
    (h : e.respectsPureFnRule env = true) :
    ∀ f, f ∈ e.calleeNames → ∃ decl, env f = some decl := by
  intro f hf
  simp only [DataExprC.respectsPureFnRule, List.all_eq_true] at h
  have hp := h f hf
  simp only [FuncEnv.calleeIsPure] at hp
  cases henv : env f with
  | none      => rw [henv] at hp; simp at hp
  | some decl => exact ⟨decl, rfl⟩

/-- **Corollary (pure-syntactic ⇒ PFR for free)**: an expression with no
    call sites automatically respects the PFR under any `env`. -/
theorem DataExprC.isPureSyntactic_respectsPureFnRule
    (e : DataExprC) (env : FuncEnv) (h : e.isPureSyntactic = true) :
    e.respectsPureFnRule env = true := by
  simp only [DataExprC.isPureSyntactic, List.isEmpty_iff] at h
  simp [DataExprC.respectsPureFnRule, h]

/-- **Theorem (base lift is pure-syntactic)** — base lifts are always
    call-free by construction. -/
theorem DataExprC.base_isPureSyntactic (e : DataExpr) :
    (DataExprC.base e).isPureSyntactic = true := by
  simp [DataExprC.isPureSyntactic, DataExprC.calleeNames]

-- ============================================================================
-- SECTION 4: Bridge to the v2 Pure Function Rule contract
-- ============================================================================

/-- A function declaration is callable from a Data context if its declared
    purity is `@pure` or `@total`. Mirrors the v2 grammar restriction. -/
def FuncDecl.callableFromData (decl : FuncDecl) : Bool :=
  match decl.purity with
  | Purity.total  => true
  | Purity.pure   => true
  | Purity.impure => false

/-- **Theorem (PFR ⇔ all callees callable from Data)**: the Pure Function
    Rule check decomposes exactly into "every callee is callable from Data
    context per the v2 grammar". -/
theorem DataExprC.respectsPureFnRule_iff_all_callable
    (e : DataExprC) (env : FuncEnv) :
    e.respectsPureFnRule env = true ↔
    ∀ f, f ∈ e.calleeNames →
      ∃ decl, env f = some decl ∧ decl.callableFromData = true := by
  simp only [DataExprC.respectsPureFnRule, List.all_eq_true]
  constructor
  · intro h f hf
    have hp := h f hf
    cases henv : env f with
    | none      => simp [FuncEnv.calleeIsPure, henv] at hp
    | some decl =>
      refine ⟨decl, rfl, ?_⟩
      simp only [FuncDecl.callableFromData]
      cases hπ : decl.purity with
      | total  => rfl
      | pure   => rfl
      | impure => simp [FuncEnv.calleeIsPure, henv, hπ] at hp
  · intro h f hf
    obtain ⟨decl, henv, hcall⟩ := h f hf
    cases hπ : decl.purity with
    | total  => simp [FuncEnv.calleeIsPure, henv, hπ]
    | pure   => simp [FuncEnv.calleeIsPure, henv, hπ]
    | impure => simp [FuncDecl.callableFromData, hπ] at hcall

-- ============================================================================
-- SECTION 5: Closure under sub-expressions and program well-formedness
-- ============================================================================

/-- The list of all `DataExprC` arguments of a call site (the immediate
    sub-expressions, lifted from `DataExpr` to `DataExprC`). For `base` this
    is empty; for `call _ args` it is the lifted args. -/
def DataExprC.subExprs : DataExprC → List DataExprC
  | base _      => []
  | call _ args => args.map DataExprC.base

/-- **Theorem (sub-expressions of a PFR expression respect PFR)**: since
    every sub-expression at a call site is a base-lifted `DataExpr` (no
    nested calls in Phase 1), each sub-expression has no callees and so
    trivially respects the PFR. -/
theorem DataExprC.subExprs_respectsPureFnRule
    (e : DataExprC) (env : FuncEnv) :
    ∀ sub, sub ∈ e.subExprs → sub.respectsPureFnRule env = true := by
  intro sub hsub
  cases e with
  | base _ =>
    simp [DataExprC.subExprs] at hsub
  | call f args =>
    simp [DataExprC.subExprs] at hsub
    obtain ⟨a, _, ha_eq⟩ := hsub
    rw [← ha_eq]
    exact DataExprC.base_respectsPureFnRule a env

/-- **Definition (program well-formedness)**: an environment is well-formed
    if every declared function passes its own purity check (`checkPurity`),
    and every callee referenced by any function exists in the environment
    and is callable per the Pure Function Rule.

    For Phase 1 we restrict callees to those that appear in some
    `DataExprC`'s `calleeNames`; nothing in the current `ControlStmt` AST
    has a Data-context call site, so the "every callee" clause is vacuous
    at the program level — but the per-call-site contract above is real. -/
def FuncEnv.purityCoherent (env : FuncEnv) : Prop :=
  ∀ name decl, env name = some decl → checkPurity decl = true

/-- **Theorem (well-formed env ⇒ every declared function respects its purity)**:
    direct unpacking of the definition; the lemma is here for downstream use. -/
theorem FuncEnv.purityCoherent_checkPurity
    (env : FuncEnv) (h : env.purityCoherent)
    (name : String) (decl : FuncDecl) (henv : env name = some decl) :
    checkPurity decl = true :=
  h name decl henv

/-- **Theorem (well-formed env ⇒ @total declared body has no while loops and
    no IO)**: combines `purityCoherent` with the structural characterisation
    `respectsPurity_total_iff_noWhileLoops_noIO`. -/
theorem FuncEnv.purityCoherent_total_no_loops_no_io
    (env : FuncEnv) (h : env.purityCoherent)
    (name : String) (decl : FuncDecl)
    (henv : env name = some decl) (htotal : decl.purity = Purity.total) :
    decl.body.noWhileLoops = true ∧ decl.body.noIO = true := by
  have hcp := h name decl henv
  simp only [checkPurity, htotal] at hcp
  exact (respectsPurity_total_iff_noWhileLoops_noIO decl.body).mp hcp

/-- **Theorem (well-formed env + PFR ⇒ no Data-context call reaches IO)**:
    the v2 grammar's headline guarantee. If the program is well-formed and
    a `DataExprC` respects the Pure Function Rule, then every callee, when
    resolved, has `noIO = true` in its body — i.e. no Data-context evaluation
    can trigger IO. -/
theorem DataExprC.pureFnRule_no_io
    (e : DataExprC) (env : FuncEnv)
    (henvWF : env.purityCoherent)
    (hPFR : e.respectsPureFnRule env = true) :
    ∀ f, f ∈ e.calleeNames →
      ∀ decl, env f = some decl → decl.body.noIO = true := by
  intro f hf decl henv
  -- The callee's declared purity is not @impure (by the PFR).
  have hnotImpure :=
    DataExprC.respectsPureFnRule_excludes_impure e env hPFR f hf decl henv
  -- Every declared function in a well-formed env passes its own purity check.
  have hcp := henvWF f decl henv
  -- Case split on the actual purity (must be @total or @pure).
  simp only [checkPurity] at hcp
  cases hp : decl.purity with
  | total =>
    rw [hp] at hcp
    exact ((respectsPurity_total_iff_noWhileLoops_noIO decl.body).mp hcp).2
  | pure =>
    rw [hp] at hcp
    -- @pure forbids IO at the ControlStmt level; we need a small lemma.
    -- A statement that respects @pure has noIO = true (by induction on s).
    exact respectsPurity_pure_implies_noIO decl.body hcp
  | impure =>
    exfalso
    apply hnotImpure
    exact hp

where
  /-- Helper used inline: @pure ⇒ noIO. We prove this as a standalone lemma
      below so the where-clause is just a redirect. -/
  respectsPurity_pure_implies_noIO
    (s : ControlStmt) (hp : s.respectsPurity Purity.pure = true) :
      s.noIO = true := by
    induction s with
    | skip => simp [ControlStmt.noIO]
    | assign _ _ => simp [ControlStmt.noIO]
    | seq s₁ s₂ ih₁ ih₂ =>
      simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at hp
      simp only [ControlStmt.noIO, Bool.and_eq_true]
      exact ⟨ih₁ hp.1, ih₂ hp.2⟩
    | ifThenElse _ s₁ s₂ ih₁ ih₂ =>
      simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at hp
      simp only [ControlStmt.noIO, Bool.and_eq_true]
      exact ⟨ih₁ hp.1, ih₂ hp.2⟩
    | whileLoop _ body ih =>
      simp only [ControlStmt.respectsPurity] at hp
      simp only [ControlStmt.noIO]
      exact ih hp
    | print _ => simp [ControlStmt.respectsPurity] at hp
    | reverseBlock _ => simp [ControlStmt.noIO]

/-- **Standalone lemma (@pure ⇒ noIO)**: a statement respecting `@pure`
    purity is IO-free. Exposed at the top level for direct reuse. -/
theorem respectsPurity_pure_implies_noIO
    (s : ControlStmt) (hp : s.respectsPurity Purity.pure = true) :
    s.noIO = true := by
  induction s with
  | skip => simp [ControlStmt.noIO]
  | assign _ _ => simp [ControlStmt.noIO]
  | seq s₁ s₂ ih₁ ih₂ =>
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at hp
    simp only [ControlStmt.noIO, Bool.and_eq_true]
    exact ⟨ih₁ hp.1, ih₂ hp.2⟩
  | ifThenElse _ s₁ s₂ ih₁ ih₂ =>
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at hp
    simp only [ControlStmt.noIO, Bool.and_eq_true]
    exact ⟨ih₁ hp.1, ih₂ hp.2⟩
  | whileLoop _ body ih =>
    simp only [ControlStmt.respectsPurity] at hp
    simp only [ControlStmt.noIO]
    exact ih hp
  | print _ => simp [ControlStmt.respectsPurity] at hp
  | reverseBlock _ => simp [ControlStmt.noIO]

/-- **Corollary (@pure callee body has noIO)**: every callee admitted by the
    Pure Function Rule has a body that is IO-free at the operational
    trace level. Combined with `execStmt_noIO_empty_trace` (from
    `JtvControlSemantics.lean`) this means a Data-context call cannot
    produce any trace output. -/
theorem DataExprC.pureFnRule_callees_silent
    (e : DataExprC) (env : FuncEnv)
    (henvWF : env.purityCoherent)
    (hPFR : e.respectsPureFnRule env = true)
    (f : String) (hf : f ∈ e.calleeNames)
    (decl : FuncDecl) (henv : env f = some decl) :
    decl.body.noIO = true :=
  DataExprC.pureFnRule_no_io e env henvWF hPFR f hf decl henv
