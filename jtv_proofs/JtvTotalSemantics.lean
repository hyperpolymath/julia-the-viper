/-
  Julia the Viper — v2 grammar: total semantics for @total-respecting statements.

  This module realises Δ5 from the v2 syntax/semantics/typecheck split: the
  operational semantics of `@total`-respecting `ControlStmt`s is *total* —
  it does not need a fuel parameter, because the grammar provably contains
  no `whileLoop` and no `print` (cf. `respectsPurity_total_iff_noWhileLoops_noIO`
  in `JtvPurity`).

  Concretely we define

      totalExec : (s : ControlStmt) → s.respectsPurity Purity.total = true →
                  State → State

  by structural recursion on `s`. The `whileLoop` and `print` arms are
  unreachable under the typing hypothesis, so they fall through to
  `False.elim`. The `reverseBlock` arm dispatches to `ReversibleStmt.execForward`.

  We then prove the headline operational soundness result:

      execStmt_eq_totalExec_when_some :
        execStmt s σ fuel = some (σ', t) →
        s.respectsPurity Purity.total = true →
        σ' = totalExec s h σ ∧ t = []

  This is the v2 contract: *whenever* the fueled semantics terminates on a
  @total program, the result agrees with the fuel-free total semantics.

  Plus an existence theorem: for every @total program there *exists* fuel
  at which `execStmt` succeeds, giving the full Δ5 bridge. The proof goes:

    * `execStmt_fuel_succ`        — successful at fuel n ⇒ at fuel n+1
    * `execStmt_fuel_le`          — successful at k₁ ⇒ at any k₂ ≥ k₁
    * `execStmt_terminates_for_total` — for every @total s, σ there's a
                                        fuel f with execStmt s σ f = some (totalExec s h σ, [])

  Together these say: `execStmt` on @total programs is fuel-free in
  spirit — it always terminates given enough fuel, and the answer is
  independent of which sufficient fuel was used.
-/

import JtvCore
import JtvTypes
import JtvPurity
import JtvControlSemantics

-- ============================================================================
-- SECTION 1: Helper lemmas to extract sub-purity hypotheses
-- ============================================================================

private theorem seq_total_left (s₁ s₂ : ControlStmt)
    (h : (ControlStmt.seq s₁ s₂).respectsPurity Purity.total = true) :
    s₁.respectsPurity Purity.total = true := by
  simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h
  exact h.1

private theorem seq_total_right (s₁ s₂ : ControlStmt)
    (h : (ControlStmt.seq s₁ s₂).respectsPurity Purity.total = true) :
    s₂.respectsPurity Purity.total = true := by
  simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h
  exact h.2

private theorem if_total_left (e : DataExpr) (s₁ s₂ : ControlStmt)
    (h : (ControlStmt.ifThenElse e s₁ s₂).respectsPurity Purity.total = true) :
    s₁.respectsPurity Purity.total = true := by
  simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h
  exact h.1

private theorem if_total_right (e : DataExpr) (s₁ s₂ : ControlStmt)
    (h : (ControlStmt.ifThenElse e s₁ s₂).respectsPurity Purity.total = true) :
    s₂.respectsPurity Purity.total = true := by
  simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h
  exact h.2

private theorem whileLoop_not_total_contra (e : DataExpr) (body : ControlStmt)
    (h : (ControlStmt.whileLoop e body).respectsPurity Purity.total = true) :
    False := by
  rw [whileLoop_not_total] at h
  exact Bool.false_ne_true h

private theorem print_not_total_contra (args : List DataExpr)
    (h : (ControlStmt.print args).respectsPurity Purity.total = true) :
    False := by
  simp [ControlStmt.respectsPurity] at h

-- ============================================================================
-- SECTION 2: The total semantics, by structural recursion on `s`
-- ============================================================================

/-- **`totalExec`**: fuel-free interpreter for `@total`-respecting `ControlStmt`s.
    The typing hypothesis `h` is consumed by the recursion to access the
    sub-statement hypotheses; the `whileLoop` and `print` cases are
    *unreachable* under `h` (proved by `whileLoop_not_total_contra` and
    `print_not_total_contra`), so they fall through to `False.elim`. -/
def totalExec : (s : ControlStmt) →
                s.respectsPurity Purity.total = true → State → State
  | ControlStmt.skip,            _, σ => σ
  | ControlStmt.assign x e,      _, σ => σ[x ↦ evalDataExpr e σ]
  | ControlStmt.seq s₁ s₂,       h, σ =>
      totalExec s₂ (seq_total_right s₁ s₂ h)
        (totalExec s₁ (seq_total_left s₁ s₂ h) σ)
  | ControlStmt.ifThenElse e s₁ s₂, h, σ =>
      if evalDataExpr e σ ≠ 0
      then totalExec s₁ (if_total_left e s₁ s₂ h) σ
      else totalExec s₂ (if_total_right e s₁ s₂ h) σ
  | ControlStmt.whileLoop e body, h, _ =>
      False.elim (whileLoop_not_total_contra e body h)
  | ControlStmt.print args,      h, _ =>
      False.elim (print_not_total_contra args h)
  | ControlStmt.reverseBlock body, _, σ => body.execForward σ

-- ============================================================================
-- SECTION 3: Constructor-by-constructor agreement lemmas
-- ============================================================================

/-- The skip case: a one-fuel `execStmt` returns the original state and
    empty trace, matching `totalExec`. -/
theorem totalExec_skip (σ : State) :
    totalExec ControlStmt.skip (by simp [ControlStmt.respectsPurity]) σ = σ := rfl

/-- The assign case. -/
theorem totalExec_assign (x : String) (e : DataExpr) (σ : State) :
    totalExec (ControlStmt.assign x e)
      (by simp [ControlStmt.respectsPurity]) σ = σ[x ↦ evalDataExpr e σ] := rfl

/-- The reverseBlock case: total semantics matches the v2 reverse-block
    forward execution. -/
theorem totalExec_reverseBlock (body : ReversibleStmt) (σ : State) :
    totalExec (ControlStmt.reverseBlock body)
      (by simp [ControlStmt.respectsPurity]) σ = body.execForward σ := rfl

-- ============================================================================
-- SECTION 4: Agreement with execStmt — whenever execStmt terminates
-- ============================================================================

/-- **Theorem (operational agreement)**: whenever the fueled `execStmt`
    terminates on a `@total`-respecting statement, the resulting state
    equals what `totalExec` would produce, and the trace is empty.

    Proof by strong induction on `fuel` (the `seq` case calls `execStmt`
    recursively with smaller fuel; the `ifThenElse` case dispatches
    to one branch with smaller fuel).

    The trace conclusion is the v2 IO soundness — already proved as
    `execStmt_total_empty_trace`; we re-derive it here as part of the
    agreement bundle for convenience. -/
theorem execStmt_eq_totalExec_when_some :
    ∀ (fuel : Nat) (s : ControlStmt)
      (h : s.respectsPurity Purity.total = true)
      (σ σ' : State) (t : Trace),
      execStmt s σ fuel = some (σ', t) →
      σ' = totalExec s h σ ∧ t = [] := by
  intro fuel
  induction fuel with
  | zero =>
    intros s _h _σ _σ' _t hexec
    simp [execStmt] at hexec
  | succ n ih =>
    intro s h σ σ' t hexec
    cases s with
    | skip =>
      simp only [execStmt, Option.some.injEq, Prod.mk.injEq] at hexec
      refine ⟨?_, hexec.2.symm⟩
      exact hexec.1.symm
    | assign x e =>
      simp only [execStmt, Option.some.injEq, Prod.mk.injEq] at hexec
      refine ⟨?_, hexec.2.symm⟩
      exact hexec.1.symm
    | seq s₁ s₂ =>
      have h₁ := seq_total_left s₁ s₂ h
      have h₂ := seq_total_right s₁ s₂ h
      obtain ⟨σ_mid, t₁, t₂, hex₁, hex₂, htsum⟩ :=
        execStmt_seq_split s₁ s₂ σ σ' t n hexec
      have ⟨hσ_mid, ht₁⟩ := ih s₁ h₁ σ σ_mid t₁ hex₁
      have ⟨hσ', ht₂⟩ := ih s₂ h₂ σ_mid σ' t₂ hex₂
      refine ⟨?_, ?_⟩
      · -- σ' = totalExec (seq s₁ s₂) h σ
        show σ' = totalExec s₂ h₂ (totalExec s₁ h₁ σ)
        rw [hσ', hσ_mid]
      · rw [htsum, ht₁, ht₂]; rfl
    | ifThenElse e s₁ s₂ =>
      have h₁ := if_total_left e s₁ s₂ h
      have h₂ := if_total_right e s₁ s₂ h
      simp only [execStmt] at hexec
      by_cases hcond : evalDataExpr e σ ≠ 0
      · rw [if_pos hcond] at hexec
        have ⟨hσ', ht⟩ := ih s₁ h₁ σ σ' t hexec
        refine ⟨?_, ht⟩
        show σ' = totalExec (ControlStmt.ifThenElse e s₁ s₂) h σ
        unfold totalExec
        rw [if_pos hcond]
        exact hσ'
      · rw [if_neg hcond] at hexec
        have ⟨hσ', ht⟩ := ih s₂ h₂ σ σ' t hexec
        refine ⟨?_, ht⟩
        show σ' = totalExec (ControlStmt.ifThenElse e s₁ s₂) h σ
        unfold totalExec
        rw [if_neg hcond]
        exact hσ'
    | whileLoop e body =>
      -- @total forbids while loops; we contradict h.
      exact absurd h (by rw [whileLoop_not_total]; exact Bool.false_ne_true)
    | print args =>
      -- @total forbids print; we contradict h.
      exact absurd h (by simp [ControlStmt.respectsPurity])
    | reverseBlock body =>
      simp only [execStmt, Option.some.injEq, Prod.mk.injEq] at hexec
      refine ⟨?_, hexec.2.symm⟩
      show σ' = totalExec (ControlStmt.reverseBlock body) h σ
      exact hexec.1.symm

/-- **Corollary**: whenever `execStmt` returns a value on a `@total` program,
    the state matches `totalExec`. -/
theorem execStmt_state_eq_totalExec
    (s : ControlStmt) (h : s.respectsPurity Purity.total = true)
    (σ σ' : State) (t : Trace) (fuel : Nat)
    (hexec : execStmt s σ fuel = some (σ', t)) :
    σ' = totalExec s h σ :=
  (execStmt_eq_totalExec_when_some fuel s h σ σ' t hexec).1

-- ============================================================================
-- SECTION 5: Fuel monotonicity
-- ============================================================================

-- Private one-step unfolding lemmas (all `rfl` since `execStmt` is defined by
-- structural recursion on `fuel = fuel' + 1` then case-split on `s`).
private theorem execStmt_seq_unfold (s₁ s₂ : ControlStmt) (σ : State) (fuel : Nat) :
    execStmt (ControlStmt.seq s₁ s₂) σ (fuel + 1) =
      (match execStmt s₁ σ fuel with
       | some (σ', t₁) =>
         match execStmt s₂ σ' fuel with
         | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
         | none => none
       | none => none) := rfl

private theorem execStmt_if_unfold (e : DataExpr) (s₁ s₂ : ControlStmt)
    (σ : State) (fuel : Nat) :
    execStmt (ControlStmt.ifThenElse e s₁ s₂) σ (fuel + 1) =
      (if evalDataExpr e σ ≠ 0
       then execStmt s₁ σ fuel
       else execStmt s₂ σ fuel) := rfl

private theorem execStmt_while_unfold (e : DataExpr) (body : ControlStmt)
    (σ : State) (fuel : Nat) :
    execStmt (ControlStmt.whileLoop e body) σ (fuel + 1) =
      (if evalDataExpr e σ ≠ 0
       then match execStmt body σ fuel with
            | some (σ', t₁) =>
              match execStmt (ControlStmt.whileLoop e body) σ' fuel with
              | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
              | none => none
            | none => none
       else some (σ, [])) := rfl

/-- **Lemma (fuel +1 monotonicity)**: a successful `execStmt` at fuel `n`
    remains successful (with the same result) at fuel `n+1`. Proved by
    induction on `fuel`, case-analysis on `s` per fuel level, with the IH
    applied to sub-`execStmt` calls in `seq`, `ifThenElse`, and `whileLoop`.

    This is the workhorse for combining sub-fuel witnesses in the
    completeness direction below. -/
theorem execStmt_fuel_succ :
    ∀ (fuel : Nat) (s : ControlStmt) (σ : State) (r : State × Trace),
      execStmt s σ fuel = some r → execStmt s σ (fuel + 1) = some r := by
  intro fuel
  induction fuel with
  | zero =>
    intros _s _σ _r hexec
    simp [execStmt] at hexec
  | succ n ih =>
    intro s σ r hexec
    cases s with
    | skip => exact hexec
    | assign x e => exact hexec
    | print args => exact hexec
    | reverseBlock body => exact hexec
    | seq s₁ s₂ =>
      rw [execStmt_seq_unfold] at hexec ⊢
      cases h₁ : execStmt s₁ σ n with
      | none =>
        rw [h₁] at hexec; exact absurd hexec (by simp)
      | some pair₁ =>
        obtain ⟨σm, t₁⟩ := pair₁
        rw [h₁] at hexec
        -- hexec : (match some (σm, t₁) with | some (σ', t₁) => match execStmt s₂ σ' n with ... | none => none) = some r
        -- iota-reduces to: (match execStmt s₂ σm n with | some (σ'', t₂) => some (σ'', t₁ ++ t₂) | none => none) = some r
        change (match execStmt s₂ σm n with
                | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
                | none => none) = some r at hexec
        have h₁' := ih s₁ σ (σm, t₁) h₁
        rw [h₁']
        change (match execStmt s₂ σm (n + 1) with
                | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
                | none => none) = some r
        cases h₂ : execStmt s₂ σm n with
        | none =>
          rw [h₂] at hexec; exact absurd hexec (by simp)
        | some pair₂ =>
          obtain ⟨σ', t₂⟩ := pair₂
          have h₂' := ih s₂ σm (σ', t₂) h₂
          rw [h₂']
          rw [h₂] at hexec
          exact hexec
    | ifThenElse e s₁ s₂ =>
      rw [execStmt_if_unfold] at hexec ⊢
      by_cases hc : evalDataExpr e σ ≠ 0
      · rw [if_pos hc] at hexec ⊢
        exact ih s₁ σ r hexec
      · rw [if_neg hc] at hexec ⊢
        exact ih s₂ σ r hexec
    | whileLoop e body =>
      rw [execStmt_while_unfold] at hexec ⊢
      by_cases hc : evalDataExpr e σ ≠ 0
      · rw [if_pos hc] at hexec ⊢
        cases h₁ : execStmt body σ n with
        | none =>
          rw [h₁] at hexec; exact absurd hexec (by simp)
        | some pair₁ =>
          obtain ⟨σm, t₁⟩ := pair₁
          rw [h₁] at hexec
          change (match execStmt (ControlStmt.whileLoop e body) σm n with
                  | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
                  | none => none) = some r at hexec
          have h₁' := ih body σ (σm, t₁) h₁
          rw [h₁']
          change (match execStmt (ControlStmt.whileLoop e body) σm (n + 1) with
                  | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
                  | none => none) = some r
          cases h₂ : execStmt (ControlStmt.whileLoop e body) σm n with
          | none =>
            rw [h₂] at hexec; exact absurd hexec (by simp)
          | some pair₂ =>
            obtain ⟨σ', t₂⟩ := pair₂
            have h₂' := ih (ControlStmt.whileLoop e body) σm (σ', t₂) h₂
            rw [h₂']
            rw [h₂] at hexec
            exact hexec
      · rw [if_neg hc] at hexec ⊢; exact hexec

/-- **Corollary (fuel ≤ monotonicity)**: a successful `execStmt` at fuel
    `k₁` remains successful (same result) at any fuel `k₂ ≥ k₁`. -/
theorem execStmt_fuel_le
    (s : ControlStmt) (σ : State) (k₁ k₂ : Nat) (hle : k₁ ≤ k₂)
    (r : State × Trace) (h : execStmt s σ k₁ = some r) :
    execStmt s σ k₂ = some r := by
  induction hle with
  | refl => exact h
  | step _ ih => exact execStmt_fuel_succ _ s σ r ih

-- ============================================================================
-- SECTION 6: Completeness — existence of sufficient fuel for @total programs
-- ============================================================================

/-- **Theorem (Δ5 completeness)**: for every @total-respecting `ControlStmt`
    and any initial state `σ`, there *exists* a fuel value at which
    `execStmt` succeeds and returns `(totalExec s h σ, [])`. Together with
    `execStmt_eq_totalExec_when_some` this gives the full Δ5 bridge:
    `execStmt` on @total programs is fuel-free in spirit — it always
    terminates given enough fuel, and the answer it gives is independent
    of which sufficient fuel was used. -/
theorem execStmt_terminates_for_total :
    ∀ (s : ControlStmt) (h : s.respectsPurity Purity.total = true) (σ : State),
      ∃ fuel, execStmt s σ fuel = some (totalExec s h σ, []) := by
  intro s
  induction s with
  | skip =>
    intro _ σ
    refine ⟨1, ?_⟩
    rfl
  | assign x e =>
    intro _ σ
    refine ⟨1, ?_⟩
    rfl
  | seq s₁ s₂ ih₁ ih₂ =>
    intro h σ
    have h₁ := seq_total_left s₁ s₂ h
    have h₂ := seq_total_right s₁ s₂ h
    obtain ⟨f₁, hf₁⟩ := ih₁ h₁ σ
    obtain ⟨f₂, hf₂⟩ := ih₂ h₂ (totalExec s₁ h₁ σ)
    refine ⟨max f₁ f₂ + 1, ?_⟩
    have hmax₁ : f₁ ≤ max f₁ f₂ := Nat.le_max_left _ _
    have hmax₂ : f₂ ≤ max f₁ f₂ := Nat.le_max_right _ _
    have hf₁' := execStmt_fuel_le s₁ σ f₁ (max f₁ f₂) hmax₁ _ hf₁
    have hf₂' := execStmt_fuel_le s₂ (totalExec s₁ h₁ σ) f₂ (max f₁ f₂) hmax₂ _ hf₂
    -- The execStmt (seq) at fuel max+1 unrolls to two sub-execStmt calls at fuel max.
    rw [execStmt_seq_unfold]
    rw [hf₁']
    -- Goal iota-reduces: match some (totalExec s₁ h₁ σ, []) with | some (σ', _) => match execStmt s₂ σ' ...
    change (match execStmt s₂ (totalExec s₁ h₁ σ) (max f₁ f₂) with
            | some (σ'', t₂) => some (σ'', [] ++ t₂)
            | none => none) = some (totalExec (ControlStmt.seq s₁ s₂) h σ, [])
    rw [hf₂']
    -- Goal iota-reduces: some (totalExec s₂ h₂ (totalExec s₁ h₁ σ), [] ++ []) = some (totalExec (seq s₁ s₂) h σ, [])
    -- And totalExec (seq s₁ s₂) h σ defeq totalExec s₂ h₂ (totalExec s₁ h₁ σ)
    rfl
  | ifThenElse e s₁ s₂ ih₁ ih₂ =>
    intro h σ
    have h₁ := if_total_left e s₁ s₂ h
    have h₂ := if_total_right e s₁ s₂ h
    by_cases hc : evalDataExpr e σ ≠ 0
    · obtain ⟨f, hf⟩ := ih₁ h₁ σ
      refine ⟨f + 1, ?_⟩
      simp only [execStmt]
      rw [if_pos hc]
      -- Goal: execStmt s₁ σ f = some (totalExec (ifThenElse e s₁ s₂) h σ, [])
      have heq : totalExec (ControlStmt.ifThenElse e s₁ s₂) h σ =
                 totalExec s₁ h₁ σ := by
        show (if evalDataExpr e σ ≠ 0
              then totalExec s₁ (if_total_left e s₁ s₂ h) σ
              else totalExec s₂ (if_total_right e s₁ s₂ h) σ) =
             totalExec s₁ h₁ σ
        rw [if_pos hc]
      rw [heq]; exact hf
    · obtain ⟨f, hf⟩ := ih₂ h₂ σ
      refine ⟨f + 1, ?_⟩
      simp only [execStmt]
      rw [if_neg hc]
      have heq : totalExec (ControlStmt.ifThenElse e s₁ s₂) h σ =
                 totalExec s₂ h₂ σ := by
        show (if evalDataExpr e σ ≠ 0
              then totalExec s₁ (if_total_left e s₁ s₂ h) σ
              else totalExec s₂ (if_total_right e s₁ s₂ h) σ) =
             totalExec s₂ h₂ σ
        rw [if_neg hc]
      rw [heq]; exact hf
  | whileLoop e body _ih =>
    intro h _σ
    exact absurd h (by rw [whileLoop_not_total]; exact Bool.false_ne_true)
  | print args =>
    intro h _σ
    exact absurd h (by simp [ControlStmt.respectsPurity])
  | reverseBlock body =>
    intro _ σ
    refine ⟨1, ?_⟩
    rfl
