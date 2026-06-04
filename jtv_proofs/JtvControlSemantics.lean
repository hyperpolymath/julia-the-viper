/-
  Julia the Viper — v2 grammar: Control-level execution semantics.

  This module proves that the `execStmt` extensions for `print` and
  `reverseBlock` (Δ3 and Δ1) have *real* observational content — they are
  not vacuous additions. Specifically:

    * `execStmt_print_emits` shows that `print(e)` produces a trace entry
      equal to `evalDataExpr e σ` (the property "Data evaluation produces
      no trace entries" is now provably non-trivial).
    * `execStmt_reverseBlock_emits_no_trace` shows that reverse blocks
      run silently (no IO surface).
    * `execStmt_noIO_empty_trace` is the structural theorem connecting the
      typing predicate `ControlStmt.noIO` to the operational trace: if a
      typed statement has no IO, executing it produces the empty trace.

  Together these are the formal counterpart of the v2 Pure Function Rule's
  "no IO under @pure / @total" — the typing check is *sound* with respect
  to the operational trace.
-/

import JtvCore
import JtvTypes
import JtvPurity

-- ============================================================================
-- SECTION 1: `print` actually emits trace entries
-- ============================================================================

/-- **Theorem (print emits its evaluated arguments)**:
    `execStmt (print args) σ` returns the state unchanged and a trace
    equal to the list of evaluated arguments. This is the operational
    soundness of `print` — the IO surface is observable. -/
theorem execStmt_print_emits (args : List DataExpr) (σ : State) (fuel : Nat) :
    execStmt (ControlStmt.print args) σ (fuel + 1) =
    some (σ, args.map (fun e => evalDataExpr e σ)) := by
  simp [execStmt]

/-- **Corollary (print with a single argument emits a singleton trace)**:
    the most direct restatement — `print(e)` produces `[evalDataExpr e σ]`. -/
theorem execStmt_print_single (e : DataExpr) (σ : State) (fuel : Nat) :
    execStmt (ControlStmt.print [e]) σ (fuel + 1) =
    some (σ, [evalDataExpr e σ]) := by
  simp [execStmt]

-- ============================================================================
-- SECTION 2: `reverseBlock` emits no trace
-- ============================================================================

/-- **Theorem (reverse blocks are silent)**:
    `execStmt (reverseBlock body) σ` returns the body's forward-executed
    state and the empty trace. Reverse blocks have no IO surface; their
    only observable effect is the state transition. -/
theorem execStmt_reverseBlock_emits_no_trace
    (body : ReversibleStmt) (σ : State) (fuel : Nat) :
    execStmt (ControlStmt.reverseBlock body) σ (fuel + 1) =
    some (body.execForward σ, []) := by
  simp [execStmt]

-- ============================================================================
-- SECTION 3: noIO ⇒ empty trace (typing soundness for the IO surface)
-- ============================================================================

/-- **Theorem (noIO ⇒ empty trace, single-step cases)**:
    For every leaf-like `ControlStmt`, if its `noIO` predicate is `true`,
    then a successful one-step `execStmt` produces the empty trace. We
    prove this for the non-recursive cases (`skip`, `assign`, `print` is
    excluded by hypothesis, `reverseBlock`); the full recursive form for
    `seq`/`ifThenElse`/`whileLoop` is below. -/
theorem execStmt_skip_empty_trace (σ : State) (fuel : Nat) :
    execStmt ControlStmt.skip σ (fuel + 1) = some (σ, []) := by
  simp [execStmt]

theorem execStmt_assign_empty_trace (x : String) (e : DataExpr)
    (σ : State) (fuel : Nat) :
    execStmt (ControlStmt.assign x e) σ (fuel + 1) =
    some (σ[x ↦ evalDataExpr e σ], []) := by
  simp [execStmt]

-- ============================================================================
-- SECTION 3a: structural decomposition lemmas for `execStmt`
-- ============================================================================

/-- **Lemma (seq decomposition)**: a successful `execStmt` of `s₁; s₂` factors
    through a mid-state with concatenated traces. Used by the recursive
    `noIO ⇒ empty trace` theorem below. -/
theorem execStmt_seq_split
    (s₁ s₂ : ControlStmt) (σ σ' : State) (t : Trace) (fuel : Nat)
    (hexec : execStmt (ControlStmt.seq s₁ s₂) σ (fuel + 1) = some (σ', t)) :
    ∃ σ_mid t₁ t₂,
      execStmt s₁ σ fuel = some (σ_mid, t₁) ∧
      execStmt s₂ σ_mid fuel = some (σ', t₂) ∧
      t = t₁ ++ t₂ := by
  -- Renamed locals (sm/T1/T2) to avoid name-collision with the existential
  -- binders (σ_mid/t₁/t₂) during anonymous-constructor elaboration.
  -- `cases h :` substitutes `execStmt ...` throughout the goal, so after we
  -- provide witnesses, the corresponding conjuncts collapse to `rfl`.
  cases h₁ : execStmt s₁ σ fuel with
  | none => simp [execStmt, h₁] at hexec
  | some pair₁ =>
    obtain ⟨sm, T1⟩ := pair₁
    cases h₂ : execStmt s₂ sm fuel with
    | none => simp [execStmt, h₁, h₂] at hexec
    | some pair₂ =>
      obtain ⟨send, T2⟩ := pair₂
      simp [execStmt, h₁, h₂] at hexec
      obtain ⟨hσ, ht⟩ := hexec
      subst hσ
      -- After `cases h₁`, the 1st conjunct collapses to `rfl`; the 2nd
      -- needs `h₂` (since the existential's σ_mid binder kept the second
      -- subterm unsubstituted until `refine` plugged sm in).
      exact ⟨sm, T1, T2, rfl, h₂, ht.symm⟩

/-- **Lemma (while decomposition, condition-true branch)**: when the
    while condition is non-zero, a successful `execStmt` of `while e do s`
    factors through a single body execution followed by a recursive while
    execution, with concatenated traces. -/
theorem execStmt_while_split_true
    (e : DataExpr) (body : ControlStmt) (σ σ' : State) (t : Trace) (fuel : Nat)
    (hcond : evalDataExpr e σ ≠ 0)
    (hexec : execStmt (ControlStmt.whileLoop e body) σ (fuel + 1) = some (σ', t)) :
    ∃ σ_mid t₁ t₂,
      execStmt body σ fuel = some (σ_mid, t₁) ∧
      execStmt (ControlStmt.whileLoop e body) σ_mid fuel = some (σ', t₂) ∧
      t = t₁ ++ t₂ := by
  cases h₁ : execStmt body σ fuel with
  | none => simp [execStmt, h₁, hcond] at hexec
  | some pair₁ =>
    obtain ⟨sm, T1⟩ := pair₁
    cases h₂ : execStmt (ControlStmt.whileLoop e body) sm fuel with
    | none => simp [execStmt, h₁, h₂, hcond] at hexec
    | some pair₂ =>
      obtain ⟨send, T2⟩ := pair₂
      simp [execStmt, h₁, h₂, hcond] at hexec
      obtain ⟨hσ, ht⟩ := hexec
      subst hσ
      exact ⟨sm, T1, T2, rfl, h₂, ht.symm⟩

-- ============================================================================
-- SECTION 3b: noIO ⇒ empty trace (recursive form)
-- ============================================================================

/-- **Theorem (noIO + successful execution ⇒ empty trace)**:
    if `s.noIO = true` and `execStmt s σ fuel = some (σ', t)`, then `t = []`.
    This is the full operational soundness of the `noIO` typing predicate
    across every `ControlStmt` constructor — the type system's "no IO"
    claim is true at runtime.

    Proved by induction on `fuel` rather than on `s`: the `whileLoop`
    self-recursion calls `execStmt` again on the same statement with
    smaller fuel, which lines up with this induction principle. -/
theorem execStmt_noIO_empty_trace :
    ∀ (fuel : Nat) (s : ControlStmt) (σ : State) (σ' : State) (t : Trace),
      s.noIO = true →
      execStmt s σ fuel = some (σ', t) →
      t = [] := by
  intro fuel
  induction fuel with
  | zero =>
    intros s _σ _σ' _t _hio hexec
    simp [execStmt] at hexec
  | succ n ih =>
    intro s σ σ' t hio hexec
    cases s with
    | skip =>
      simp only [execStmt, Option.some.injEq, Prod.mk.injEq] at hexec
      exact hexec.2.symm
    | assign x e =>
      simp only [execStmt, Option.some.injEq, Prod.mk.injEq] at hexec
      exact hexec.2.symm
    | seq s₁ s₂ =>
      simp only [ControlStmt.noIO, Bool.and_eq_true] at hio
      obtain ⟨hio₁, hio₂⟩ := hio
      obtain ⟨σ_mid, t₁, t₂, h₁, h₂, ht⟩ :=
        execStmt_seq_split s₁ s₂ σ σ' t n hexec
      have ht₁ := ih s₁ σ σ_mid t₁ hio₁ h₁
      have ht₂ := ih s₂ σ_mid σ' t₂ hio₂ h₂
      rw [ht, ht₁, ht₂]; rfl
    | ifThenElse e s₁ s₂ =>
      simp only [ControlStmt.noIO, Bool.and_eq_true] at hio
      obtain ⟨hio₁, hio₂⟩ := hio
      simp only [execStmt] at hexec
      split at hexec
      · exact ih s₁ σ σ' t hio₁ hexec
      · exact ih s₂ σ σ' t hio₂ hexec
    | whileLoop e body =>
      simp only [ControlStmt.noIO] at hio
      -- Two sub-cases: condition true (recursive call) or false (terminates).
      by_cases hcond : evalDataExpr e σ ≠ 0
      · obtain ⟨σ_mid, t₁, t₂, h₁, h₂, ht⟩ :=
          execStmt_while_split_true e body σ σ' t n hcond hexec
        have hwhileIO : (ControlStmt.whileLoop e body).noIO = true := by
          simp [ControlStmt.noIO, hio]
        have ht₁ := ih body σ σ_mid t₁ hio h₁
        have ht₂ := ih (ControlStmt.whileLoop e body) σ_mid σ' t₂ hwhileIO h₂
        rw [ht, ht₁, ht₂]; rfl
      · -- Condition false: while terminates with state unchanged, empty trace.
        simp only [execStmt] at hexec
        rw [if_neg hcond] at hexec
        simp only [Option.some.injEq, Prod.mk.injEq] at hexec
        exact hexec.2.symm
    | print _ =>
      -- print has noIO = false; the hypothesis is vacuous.
      simp [ControlStmt.noIO] at hio
    | reverseBlock _ =>
      simp only [execStmt, Option.some.injEq, Prod.mk.injEq] at hexec
      exact hexec.2.symm

/-- **Theorem (`print` is rejected by `noIO`)**:
    a `print` statement has `noIO = false`, formally distinguishing it from
    every other constructor. This is the typing-side counterpart of
    `execStmt_print_emits`. -/
theorem print_not_noIO (args : List DataExpr) :
    (ControlStmt.print args).noIO = false := by
  simp [ControlStmt.noIO]

/-- **Theorem (`reverseBlock` passes `noIO`)**:
    reverse blocks are IO-free at the Control surface — the body is a
    `ReversibleStmt`, which has no print constructor. -/
theorem reverseBlock_noIO (body : ReversibleStmt) :
    (ControlStmt.reverseBlock body).noIO = true := by
  simp [ControlStmt.noIO]

/-- **Theorem (skip is IO-free)**. -/
theorem skip_noIO : ControlStmt.skip.noIO = true := by
  simp [ControlStmt.noIO]

/-- **Theorem (assign is IO-free)**. -/
theorem assign_noIO (x : String) (e : DataExpr) :
    (ControlStmt.assign x e).noIO = true := by
  simp [ControlStmt.noIO]

-- ============================================================================
-- SECTION 4: @total ⇒ noIO, the v2 typecheck contract
-- ============================================================================

/-- **Theorem (@total ⇒ noIO)**: a statement respecting @total purity is
    IO-free. This is the @total guarantee at the typing level: combining
    with `execStmt_print_emits` and `print_not_noIO`, no @total program
    can contain a `print`. -/
theorem total_implies_noIO (s : ControlStmt)
    (htotal : s.respectsPurity Purity.total = true) :
    s.noIO = true :=
  ((respectsPurity_total_iff_noWhileLoops_noIO s).mp htotal).2

/-- **Theorem (a @total program excludes `print`)**: if `s.respectsPurity total`
    is `true`, then `s` is not of the form `print args`. This is the
    v2 grammar's "@total forbids IO" rule made formal: any attempt to
    annotate a `print`-containing function as @total fails the typecheck. -/
theorem total_excludes_print (args : List DataExpr) :
    (ControlStmt.print args).respectsPurity Purity.total ≠ true := by
  intro h
  have hno := total_implies_noIO _ h
  rw [print_not_noIO] at hno
  exact Bool.false_ne_true hno

/-- **Theorem (@total + successful execution ⇒ empty trace)**: combines
    `total_implies_noIO` with `execStmt_noIO_empty_trace`. This is the v2
    grammar's headline operational soundness: a @total program *cannot*
    produce trace output, even when it runs to completion. -/
theorem execStmt_total_empty_trace
    (s : ControlStmt) (σ : State) (fuel : Nat) (σ' : State) (t : Trace)
    (htotal : s.respectsPurity Purity.total = true)
    (hexec : execStmt s σ fuel = some (σ', t)) :
    t = [] :=
  execStmt_noIO_empty_trace fuel s σ σ' t (total_implies_noIO s htotal) hexec
