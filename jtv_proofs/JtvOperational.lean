/-
  Julia the Viper - Operational Semantics

  This file defines small-step and big-step operational semantics for JtV,
  complementing the denotational semantics in JtvCore.lean.

  Operational semantics provide an executable specification that:
  1. Models actual interpreter behavior
  2. Enables proofs about evaluation order
  3. Supports reasoning about non-termination (for Control Language)
-/

import JtvCore

-- ============================================================================
-- SECTION 1: SMALL-STEP OPERATIONAL SEMANTICS (Data Language)
-- ============================================================================

/--
  Configuration: (expression, state) pair
  Represents a point in evaluation
-/
structure DataConfig where
  expr : DataExpr
  state : State
  -- Note: no `deriving Repr` — State = String → Int has no Repr instance.

/--
  Small-step transition relation for Data expressions: e, σ ⟶ e', σ'
  Note: State never changes in Data Language (no side effects)
-/
inductive DataStep : DataConfig → DataConfig → Prop where
  | addLeft : ∀ e₁ e₁' e₂ σ,
      DataStep ⟨e₁, σ⟩ ⟨e₁', σ⟩ →
      DataStep ⟨DataExpr.add e₁ e₂, σ⟩ ⟨DataExpr.add e₁' e₂, σ⟩

  | addRight : ∀ (n₁ : Int) e₂ e₂' σ,
      DataStep ⟨e₂, σ⟩ ⟨e₂', σ⟩ →
      DataStep ⟨DataExpr.add (DataExpr.lit n₁) e₂, σ⟩ ⟨DataExpr.add (DataExpr.lit n₁) e₂', σ⟩

  | addEval : ∀ (n₁ n₂ : Int) σ,
      DataStep ⟨DataExpr.add (DataExpr.lit n₁) (DataExpr.lit n₂), σ⟩ ⟨DataExpr.lit (n₁ + n₂), σ⟩

  | varLookup : ∀ x σ,
      DataStep ⟨DataExpr.var x, σ⟩ ⟨DataExpr.lit (σ x), σ⟩

  | negStep : ∀ e e' σ,
      DataStep ⟨e, σ⟩ ⟨e', σ⟩ →
      DataStep ⟨DataExpr.neg e, σ⟩ ⟨DataExpr.neg e', σ⟩

  | negEval : ∀ (n : Int) σ,
      DataStep ⟨DataExpr.neg (DataExpr.lit n), σ⟩ ⟨DataExpr.lit (-n), σ⟩

notation:50 c₁ " ⟶ᴰ " c₂ => DataStep c₁ c₂

/-- Value predicate: an expression is a value if it's a literal -/
def DataExpr.isValue : DataExpr → Bool
  | lit _ => true
  | _ => false

/-- Multi-step transition (reflexive transitive closure) -/
inductive DataStepStar : DataConfig → DataConfig → Prop where
  | refl : ∀ c, DataStepStar c c
  | step : ∀ c₁ c₂ c₃, DataStep c₁ c₂ → DataStepStar c₂ c₃ → DataStepStar c₁ c₃

notation:50 c₁ " ⟶ᴰ* " c₂ => DataStepStar c₁ c₂

-- ============================================================================
-- SECTION 2: BIG-STEP OPERATIONAL SEMANTICS (Data Language)
-- ============================================================================

/--
  Big-step evaluation relation: (e, σ) ⇓ n
  Expression e in state σ evaluates to value n
-/
inductive DataBigStep : DataExpr → State → Int → Prop where
  | lit : ∀ n σ,
      DataBigStep (DataExpr.lit n) σ n

  | var : ∀ x σ,
      DataBigStep (DataExpr.var x) σ (σ x)

  | add : ∀ e₁ e₂ σ n₁ n₂,
      DataBigStep e₁ σ n₁ →
      DataBigStep e₂ σ n₂ →
      DataBigStep (DataExpr.add e₁ e₂) σ (n₁ + n₂)

  | neg : ∀ e σ n,
      DataBigStep e σ n →
      DataBigStep (DataExpr.neg e) σ (-n)

notation:50 "⟨" e ", " σ "⟩ ⇓ " n => DataBigStep e σ n

-- ============================================================================
-- SECTION 3: CONTROL LANGUAGE SEMANTICS
-- ============================================================================

/--
  Control configuration: (statement, state) pair
-/
structure CtrlConfig where
  stmt : ControlStmt
  state : State

/--
  Small-step for Control statements (partial - may not terminate)
-/
inductive CtrlStep : CtrlConfig → CtrlConfig → Prop where
  | skip : ∀ σ,
      CtrlStep ⟨ControlStmt.skip, σ⟩ ⟨ControlStmt.skip, σ⟩

  | assign : ∀ x e σ n,
      DataBigStep e σ n →
      CtrlStep ⟨ControlStmt.assign x e, σ⟩ ⟨ControlStmt.skip, σ[x ↦ n]⟩

  | seqLeft : ∀ s₁ s₁' s₂ σ σ',
      CtrlStep ⟨s₁, σ⟩ ⟨s₁', σ'⟩ →
      CtrlStep ⟨ControlStmt.seq s₁ s₂, σ⟩ ⟨ControlStmt.seq s₁' s₂, σ'⟩

  | seqSkip : ∀ s₂ σ,
      CtrlStep ⟨ControlStmt.seq ControlStmt.skip s₂, σ⟩ ⟨s₂, σ⟩

  | ifTrue : ∀ e s₁ s₂ σ n,
      DataBigStep e σ n →
      n ≠ 0 →
      CtrlStep ⟨ControlStmt.ifThenElse e s₁ s₂, σ⟩ ⟨s₁, σ⟩

  | ifFalse : ∀ e s₁ s₂ σ,
      DataBigStep e σ 0 →
      CtrlStep ⟨ControlStmt.ifThenElse e s₁ s₂, σ⟩ ⟨s₂, σ⟩

  | whileTrue : ∀ e s σ n,
      DataBigStep e σ n →
      n ≠ 0 →
      CtrlStep ⟨ControlStmt.whileLoop e s, σ⟩
               ⟨ControlStmt.seq s (ControlStmt.whileLoop e s), σ⟩

  | whileFalse : ∀ e s σ,
      DataBigStep e σ 0 →
      CtrlStep ⟨ControlStmt.whileLoop e s, σ⟩ ⟨ControlStmt.skip, σ⟩

notation:50 c₁ " ⟶ᶜ " c₂ => CtrlStep c₁ c₂

/-- Multi-step for Control -/
inductive CtrlStepStar : CtrlConfig → CtrlConfig → Prop where
  | refl : ∀ c, CtrlStepStar c c
  | step : ∀ c₁ c₂ c₃, CtrlStep c₁ c₂ → CtrlStepStar c₂ c₃ → CtrlStepStar c₁ c₃

notation:50 c₁ " ⟶ᶜ* " c₂ => CtrlStepStar c₁ c₂

-- ============================================================================
-- SECTION 4: EQUIVALENCE THEOREMS
-- ============================================================================

/--
  **Theorem (Denotational-Big-Step Equivalence)**:
  Big-step semantics agrees with denotational semantics.
-/
theorem bigstep_denotational_equiv (e : DataExpr) (σ : State) (n : Int) :
    DataBigStep e σ n ↔ evalDataExpr e σ = n := by
  constructor
  · intro h
    induction h with
    | lit n' σ' => simp [evalDataExpr]
    | var x σ' => simp [evalDataExpr]
    | add e₁ e₂ σ' n₁ n₂ _ _ ih₁ ih₂ =>
      simp [evalDataExpr, ih₁, ih₂]
    | neg e' σ' n' _ ih =>
      simp [evalDataExpr, ih]
  · intro h
    induction e generalizing n with
    | lit m =>
      simp [evalDataExpr] at h
      subst h
      exact DataBigStep.lit m σ
    | var x =>
      simp [evalDataExpr] at h
      subst h
      exact DataBigStep.var x σ
    | add e₁ e₂ ih₁ ih₂ =>
      simp [evalDataExpr] at h
      have h₁ := ih₁ (evalDataExpr e₁ σ) rfl
      have h₂ := ih₂ (evalDataExpr e₂ σ) rfl
      rw [← h]
      exact DataBigStep.add e₁ e₂ σ _ _ h₁ h₂
    | neg e' ih =>
      simp [evalDataExpr] at h
      have h' := ih (evalDataExpr e' σ) rfl
      rw [← h]
      exact DataBigStep.neg e' σ _ h'

/--
  **Theorem (Small-Step Determinism for Data)**:
  Data language small-step is deterministic.
-/
theorem data_step_deterministic (c c₁ c₂ : DataConfig) :
    DataStep c c₁ → DataStep c c₂ → c₁ = c₂ := by
  intro h₁ h₂
  induction h₁ generalizing c₂ with
  | addEval n₁ n₂ σ =>
    cases h₂ with
    | addEval => rfl
    | addLeft _ _ _ _ h => cases h
    | addRight _ _ _ _ h => cases h
  | varLookup x σ =>
    cases h₂
    rfl
  | negEval n σ =>
    cases h₂ with
    | negEval => rfl
    | negStep _ _ _ h => cases h
  | addLeft e₁ e₁' e₂ σ hs ih =>
    cases h₂ with
    | addLeft _ e₁'' _ _ hs' =>
      have heq : (⟨e₁', σ⟩ : DataConfig) = ⟨e₁'', σ⟩ := ih _ hs'
      injection heq with h_expr _
      subst h_expr
      rfl
    | addRight _ _ _ _ _ => cases hs
    | addEval => cases hs
  | addRight n₁ e₂ e₂' σ hs ih =>
    cases h₂ with
    | addLeft _ _ _ _ hs' => cases hs'
    | addRight _ _ e₂'' _ hs' =>
      have heq : (⟨e₂', σ⟩ : DataConfig) = ⟨e₂'', σ⟩ := ih _ hs'
      injection heq with h_expr _
      subst h_expr
      rfl
    | addEval => cases hs
  | negStep e e' σ hs ih =>
    cases h₂ with
    | negStep _ e'' _ hs' =>
      have heq : (⟨e', σ⟩ : DataConfig) = ⟨e'', σ⟩ := ih _ hs'
      injection heq with h_expr _
      subst h_expr
      rfl
    | negEval => cases hs

-- ============================================================================
-- SECTION 5: PROGRESS AND PRESERVATION (Data Language)
-- ============================================================================

/--
  **Theorem (Progress for Data)**:
  Every non-value Data expression can take a step.
-/
theorem data_progress (e : DataExpr) (σ : State) :
    e.isValue = true ∨ ∃ e', DataStep ⟨e, σ⟩ ⟨e', σ⟩ := by
  induction e with
  | lit n => left; simp [DataExpr.isValue]
  | var x => right; exact ⟨DataExpr.lit (σ x), DataStep.varLookup x σ⟩
  | add e₁ e₂ ih₁ ih₂ =>
    right
    rcases ih₁ with hv₁ | ⟨e₁', hs₁⟩
    · -- e₁ is a value
      cases e₁ with
      | lit n₁ =>
        rcases ih₂ with hv₂ | ⟨e₂', hs₂⟩
        · -- e₂ is a value
          cases e₂ with
          | lit n₂ => exact ⟨DataExpr.lit (n₁ + n₂), DataStep.addEval n₁ n₂ σ⟩
          | _ => simp [DataExpr.isValue] at hv₂
        · -- e₂ steps
          exact ⟨DataExpr.add (DataExpr.lit n₁) e₂', DataStep.addRight n₁ e₂ e₂' σ hs₂⟩
      | _ => simp [DataExpr.isValue] at hv₁
    · -- e₁ steps
      exact ⟨DataExpr.add e₁' e₂, DataStep.addLeft e₁ e₁' e₂ σ hs₁⟩
  | neg e' ih =>
    right
    rcases ih with hv | ⟨e'', hs⟩
    · -- e' is a value
      cases e' with
      | lit n => exact ⟨DataExpr.lit (-n), DataStep.negEval n σ⟩
      | _ => simp [DataExpr.isValue] at hv
    · -- e' steps
      exact ⟨DataExpr.neg e'', DataStep.negStep e' e'' σ hs⟩

/--
  **Theorem (Preservation - State Invariance)**:
  Data language evaluation never modifies state.
-/
theorem data_state_preservation (c₁ c₂ : DataConfig) :
    DataStep c₁ c₂ → c₁.state = c₂.state := by
  intro h
  induction h with
  | addLeft _ _ _ _ _ ih => exact ih
  | addRight _ _ _ _ _ ih => exact ih
  | addEval _ _ _ => rfl
  | varLookup _ _ => rfl
  | negStep _ _ _ _ ih => exact ih
  | negEval _ _ => rfl

-- ============================================================================
-- SECTION 6: TERMINATION THEOREMS (Operational)
-- ============================================================================

/-
  **Theorem (Data Termination via Small-Step)**:
  Every Data expression reaches a value in finite steps.
  (Stated and proved as `data_terminates` below, after the context-lift lemmas.)
-/

/-- Generic congruence-friendly form of DataStepStar over a context, parametrised
    by a function on configurations and a witness that the function lifts single
    steps. Avoids the "indices not variables" induction obstacle on the
    context-specific lemmas. -/
private theorem dataStepStar_lift
    (f : DataConfig → DataConfig)
    (hstep : ∀ c c', DataStep c c' → DataStep (f c) (f c'))
    (c₁ c₂ : DataConfig) (h : DataStepStar c₁ c₂) :
    DataStepStar (f c₁) (f c₂) := by
  induction h with
  | refl _ => exact DataStepStar.refl _
  | step a b _ hs _ ih => exact DataStepStar.step (f a) (f b) _ (hstep a b hs) ih

/-- Lift DataStepStar through addLeft context. -/
theorem dataStepStar_addLeft (e₁ e₁' e₂ : DataExpr) (σ : State) :
    DataStepStar ⟨e₁, σ⟩ ⟨e₁', σ⟩ →
    DataStepStar ⟨DataExpr.add e₁ e₂, σ⟩ ⟨DataExpr.add e₁' e₂, σ⟩ := by
  intro h
  -- We only lift over configs whose state remains σ; the lift function below is
  -- only meaningful on such configs, and data_state_preservation guarantees
  -- that DataStepStar walks only stay in state σ.
  let f : DataConfig → DataConfig := fun c => ⟨DataExpr.add c.expr e₂, c.state⟩
  have hstep : ∀ c c', DataStep c c' → DataStep (f c) (f c') := by
    intro c c' hs
    have hstate : c.state = c'.state := data_state_preservation c c' hs
    cases c with
    | mk ec sc =>
      cases c' with
      | mk ec' sc' =>
        simp at hstate
        subst hstate
        exact DataStep.addLeft ec ec' e₂ sc hs
  exact dataStepStar_lift f hstep ⟨e₁, σ⟩ ⟨e₁', σ⟩ h

/-- Lift DataStepStar through addRight context. -/
theorem dataStepStar_addRight (n₁ : Int) (e₂ e₂' : DataExpr) (σ : State) :
    DataStepStar ⟨e₂, σ⟩ ⟨e₂', σ⟩ →
    DataStepStar ⟨DataExpr.add (DataExpr.lit n₁) e₂, σ⟩ ⟨DataExpr.add (DataExpr.lit n₁) e₂', σ⟩ := by
  intro h
  let f : DataConfig → DataConfig :=
    fun c => ⟨DataExpr.add (DataExpr.lit n₁) c.expr, c.state⟩
  have hstep : ∀ c c', DataStep c c' → DataStep (f c) (f c') := by
    intro c c' hs
    have hstate : c.state = c'.state := data_state_preservation c c' hs
    cases c with
    | mk ec sc =>
      cases c' with
      | mk ec' sc' =>
        simp at hstate
        subst hstate
        exact DataStep.addRight n₁ ec ec' sc hs
  exact dataStepStar_lift f hstep ⟨e₂, σ⟩ ⟨e₂', σ⟩ h

/-- Lift DataStepStar through neg context. -/
theorem dataStepStar_neg (e e' : DataExpr) (σ : State) :
    DataStepStar ⟨e, σ⟩ ⟨e', σ⟩ →
    DataStepStar ⟨DataExpr.neg e, σ⟩ ⟨DataExpr.neg e', σ⟩ := by
  intro h
  let f : DataConfig → DataConfig := fun c => ⟨DataExpr.neg c.expr, c.state⟩
  have hstep : ∀ c c', DataStep c c' → DataStep (f c) (f c') := by
    intro c c' hs
    have hstate : c.state = c'.state := data_state_preservation c c' hs
    cases c with
    | mk ec sc =>
      cases c' with
      | mk ec' sc' =>
        simp at hstate
        subst hstate
        exact DataStep.negStep ec ec' sc hs
  exact dataStepStar_lift f hstep ⟨e, σ⟩ ⟨e', σ⟩ h

/-- Transitivity of DataStepStar -/
theorem dataStepStar_trans (c₁ c₂ c₃ : DataConfig) :
    DataStepStar c₁ c₂ → DataStepStar c₂ c₃ → DataStepStar c₁ c₃ := by
  intro h₁₂ h₂₃
  induction h₁₂ with
  | refl _ => exact h₂₃
  | step a b c hs _ ih => exact DataStepStar.step a b c₃ hs (ih h₂₃)

theorem data_terminates (e : DataExpr) (σ : State) :
    ∃ (n : Int), DataStepStar ⟨e, σ⟩ ⟨DataExpr.lit n, σ⟩ := by
  induction e with
  | lit n => exact ⟨n, DataStepStar.refl _⟩
  | var x =>
    exact ⟨σ x, DataStepStar.step _ _ _ (DataStep.varLookup x σ) (DataStepStar.refl _)⟩
  | add e₁ e₂ ih₁ ih₂ =>
    obtain ⟨n₁, h₁⟩ := ih₁
    obtain ⟨n₂, h₂⟩ := ih₂
    exact ⟨n₁ + n₂,
      dataStepStar_trans _ _ _
        (dataStepStar_addLeft e₁ (DataExpr.lit n₁) e₂ σ h₁)
        (dataStepStar_trans _ _ _
          (dataStepStar_addRight n₁ e₂ (DataExpr.lit n₂) σ h₂)
          (DataStepStar.step _ _ _ (DataStep.addEval n₁ n₂ σ) (DataStepStar.refl _)))⟩
  | neg e' ih =>
    obtain ⟨n, h⟩ := ih
    exact ⟨-n,
      dataStepStar_trans _ _ _
        (dataStepStar_neg e' (DataExpr.lit n) σ h)
        (DataStepStar.step _ _ _ (DataStep.negEval n σ) (DataStepStar.refl _))⟩

-- ============================================================================
-- SECTION 7: NON-TERMINATION (Control Language)
-- ============================================================================

/--
  Example: `while 1 { skip }` is an infinite loop.
  This demonstrates that Control Language can diverge.
-/
def infiniteLoop : ControlStmt :=
  ControlStmt.whileLoop (DataExpr.lit 1) ControlStmt.skip

/--
  **Observation**: The Control Language may not terminate.
  This is by design - it's Turing-complete.

  We can prove that infiniteLoop never reaches skip by showing
  that every step leads to another step.
-/
theorem infinite_loop_steps (σ : State) :
    ∃ c', CtrlStep ⟨infiniteLoop, σ⟩ c' := by
  refine ⟨⟨ControlStmt.seq ControlStmt.skip infiniteLoop, σ⟩, ?_⟩
  exact CtrlStep.whileTrue (DataExpr.lit 1) ControlStmt.skip σ 1
    (DataBigStep.lit 1 σ) (by decide)

-- ============================================================================
-- SECTION 8: HARVARD ARCHITECTURE INVARIANT
-- ============================================================================

/--
  **Structural Invariant**: Control steps can modify state, Data steps cannot.

  This is the operational manifestation of the Harvard Architecture:
  - Data Language: Pure computation (state-preserving)
  - Control Language: Stateful computation (state-modifying)
-/

theorem data_is_pure (c₁ c₂ : DataConfig) (h : c₁ ⟶ᴰ c₂) : c₁.state = c₂.state :=
  data_state_preservation c₁ c₂ h

/-- Control Language CAN modify state: demonstrated with a specific state where
    `x` becomes 42 (≠ 0 = `State.empty "x"`). -/
example : ∃ σ' : State, σ' ≠ State.empty ∧
    CtrlStep ⟨ControlStmt.assign "x" (DataExpr.lit 42), State.empty⟩ ⟨ControlStmt.skip, σ'⟩ := by
  refine ⟨State.empty["x" ↦ 42], ?_, ?_⟩
  · intro h
    have hpoint : (State.empty["x" ↦ 42]) "x" = State.empty "x" := by rw [h]
    have hbeq : ("x" == "x") = true := by decide
    simp only [State.update, State.empty, hbeq, if_true] at hpoint
    exact absurd hpoint (by decide)
  · exact CtrlStep.assign "x" (DataExpr.lit 42) State.empty 42 (DataBigStep.lit 42 State.empty)
