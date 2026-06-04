/-
  Julia the Viper — v2 Grammar: Purity layer.

  This module closes the loop on the v2 grammar's `purity_marker` annotation
  (`spec/grammar.ebnf:86-88`):

      purity_marker = "@pure"   (* No side effects, may not terminate *)
                    | "@total"  (* Guaranteed to terminate *)

  and the v2 Pure Function Rule quoted at `spec/grammar.ebnf:96-97`:

      "Only Pure Data Functions can be called from Data context"
      "Compiler MUST enforce: @pure functions cannot contain loops or IO"

  We strengthen the `ControlStmt.respectsPurity` predicate (already defined
  in `JtvTypes.lean`) with the compositionality and monotonicity theorems
  needed by any downstream purity checker.

  Important caveat (documented honestly): the present ControlStmt grammar
  models only `skip`, `assign`, `seq`, `ifThenElse`, `whileLoop`. There is
  no `print` constructor and no function-call constructor in `DataExpr`,
  so the @pure-vs-@impure distinction collapses to the @total-vs-@pure
  distinction at the Control level. The theorems below are stated for the
  full `Purity` lattice and hold regardless; extending `ControlStmt` with
  IO would only add cases to `respectsPurity`, not break the lemmas here.
-/

import JtvCore
import JtvTypes

-- ============================================================================
-- SECTION 1: Decomposition theorems for `respectsPurity`
-- ============================================================================

/-- **Theorem (Seq decomposition)**: `respectsPurity p (seq s₁ s₂)` is
    equivalent to both components respecting `p`. The forward direction
    captures the "if a sequence is pure then both parts are pure" intuition;
    the reverse direction captures compositionality. -/
theorem respectsPurity_seq_iff (s₁ s₂ : ControlStmt) (p : Purity) :
    (ControlStmt.seq s₁ s₂).respectsPurity p = true ↔
    s₁.respectsPurity p = true ∧ s₂.respectsPurity p = true := by
  constructor
  · intro h
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h
    exact h
  · intro ⟨h₁, h₂⟩
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true]
    exact ⟨h₁, h₂⟩

/-- **Theorem (ifThenElse decomposition)**: an if-then-else respects purity `p`
    iff both branches do. The condition (a `DataExpr`) is automatically Total
    by `data_is_total`, so it cannot affect Control purity. -/
theorem respectsPurity_ifThenElse_iff (e : DataExpr) (s₁ s₂ : ControlStmt)
    (p : Purity) :
    (ControlStmt.ifThenElse e s₁ s₂).respectsPurity p = true ↔
    s₁.respectsPurity p = true ∧ s₂.respectsPurity p = true := by
  constructor
  · intro h
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h
    exact h
  · intro ⟨h₁, h₂⟩
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true]
    exact ⟨h₁, h₂⟩

/-- **Theorem (skip is pure at every level)**: `skip` is trivially pure. -/
theorem respectsPurity_skip (p : Purity) :
    ControlStmt.skip.respectsPurity p = true := by
  simp [ControlStmt.respectsPurity]

/-- **Theorem (assign is pure at every level)**: a single assignment is
    purity-neutral because the data-side expression is by construction Total
    (`data_is_total`). The assignment introduces no loops and no IO. -/
theorem respectsPurity_assign (x : String) (e : DataExpr) (p : Purity) :
    (ControlStmt.assign x e).respectsPurity p = true := by
  simp [ControlStmt.respectsPurity]

-- ============================================================================
-- SECTION 2: While-loop characterisation of @total purity
-- ============================================================================

/-- **Theorem (whileLoop forbidden under @total)**: a while-loop never respects
    the @total purity level. This is the v2 grammar's strict rule
    "@total: Guaranteed to terminate" — and while loops cannot be proved
    terminating in general. -/
theorem whileLoop_not_total (e : DataExpr) (s : ControlStmt) :
    (ControlStmt.whileLoop e s).respectsPurity Purity.total = false := by
  simp [ControlStmt.respectsPurity]

/-- **Theorem (whileLoop allowed under @pure / @impure)**: while-loops are
    only forbidden under @total — they're allowed under @pure (may not
    terminate but no side effects) and @impure. The recursive body must also
    respect the same purity. -/
theorem whileLoop_pure_iff (e : DataExpr) (s : ControlStmt) (p : Purity)
    (hp : p ≠ Purity.total) :
    (ControlStmt.whileLoop e s).respectsPurity p = true ↔
    s.respectsPurity p = true := by
  cases p with
  | total => exact absurd rfl hp
  | pure => simp [ControlStmt.respectsPurity]
  | impure => simp [ControlStmt.respectsPurity]

-- ============================================================================
-- SECTION 3: Structural "no while loops anywhere" predicate
-- ============================================================================

/-- A direct structural predicate: the statement contains no `whileLoop`
    anywhere in its AST. -/
def ControlStmt.noWhileLoops : ControlStmt → Bool
  | ControlStmt.skip            => true
  | ControlStmt.assign _ _      => true
  | ControlStmt.seq s₁ s₂       => s₁.noWhileLoops && s₂.noWhileLoops
  | ControlStmt.ifThenElse _ s₁ s₂ => s₁.noWhileLoops && s₂.noWhileLoops
  | ControlStmt.whileLoop _ _   => false
  -- v2 additions:
  | ControlStmt.print _         => true   -- print has no loops itself
  | ControlStmt.reverseBlock _  => true   -- rev blocks have no while loops by syntax

/-- A direct structural predicate: the statement performs no IO. The only
    IO surface in v2 is `print`; everything else is internal computation.
    `reverseBlock` bodies cannot contain `print` (they are `ReversibleStmt`,
    not `ControlStmt`), so they are IO-free. -/
def ControlStmt.noIO : ControlStmt → Bool
  | ControlStmt.skip            => true
  | ControlStmt.assign _ _      => true
  | ControlStmt.seq s₁ s₂       => s₁.noIO && s₂.noIO
  | ControlStmt.ifThenElse _ s₁ s₂ => s₁.noIO && s₂.noIO
  | ControlStmt.whileLoop _ s   => s.noIO
  | ControlStmt.print _         => false
  | ControlStmt.reverseBlock _  => true

/-- **Theorem (@total ⇔ no while loops AND no IO)**: `respectsPurity total`
    exactly characterises the conjunction of "no while loops" and "no IO"
    anywhere in the AST. This is the structural formulation of the v2
    grammar's @total guarantee, now strengthened to include the v2
    `print` constructor as an IO source. -/
theorem respectsPurity_total_iff_noWhileLoops_noIO (s : ControlStmt) :
    s.respectsPurity Purity.total = true ↔
    s.noWhileLoops = true ∧ s.noIO = true := by
  induction s with
  | skip =>
    simp [ControlStmt.respectsPurity, ControlStmt.noWhileLoops,
          ControlStmt.noIO]
  | assign x e =>
    simp [ControlStmt.respectsPurity, ControlStmt.noWhileLoops,
          ControlStmt.noIO]
  | seq s₁ s₂ ih₁ ih₂ =>
    simp only [ControlStmt.respectsPurity, ControlStmt.noWhileLoops,
               ControlStmt.noIO, Bool.and_eq_true]
    constructor
    · intro ⟨h₁, h₂⟩
      obtain ⟨wl₁, io₁⟩ := ih₁.mp h₁
      obtain ⟨wl₂, io₂⟩ := ih₂.mp h₂
      exact ⟨⟨wl₁, wl₂⟩, ⟨io₁, io₂⟩⟩
    · intro ⟨⟨wl₁, wl₂⟩, ⟨io₁, io₂⟩⟩
      exact ⟨ih₁.mpr ⟨wl₁, io₁⟩, ih₂.mpr ⟨wl₂, io₂⟩⟩
  | ifThenElse e s₁ s₂ ih₁ ih₂ =>
    simp only [ControlStmt.respectsPurity, ControlStmt.noWhileLoops,
               ControlStmt.noIO, Bool.and_eq_true]
    constructor
    · intro ⟨h₁, h₂⟩
      obtain ⟨wl₁, io₁⟩ := ih₁.mp h₁
      obtain ⟨wl₂, io₂⟩ := ih₂.mp h₂
      exact ⟨⟨wl₁, wl₂⟩, ⟨io₁, io₂⟩⟩
    · intro ⟨⟨wl₁, wl₂⟩, ⟨io₁, io₂⟩⟩
      exact ⟨ih₁.mpr ⟨wl₁, io₁⟩, ih₂.mpr ⟨wl₂, io₂⟩⟩
  | whileLoop e body _ih =>
    simp [ControlStmt.respectsPurity, ControlStmt.noWhileLoops,
          ControlStmt.noIO]
  | print _ =>
    simp [ControlStmt.respectsPurity, ControlStmt.noWhileLoops,
          ControlStmt.noIO]
  | reverseBlock _ =>
    simp [ControlStmt.respectsPurity, ControlStmt.noWhileLoops,
          ControlStmt.noIO]

-- ============================================================================
-- SECTION 4: Purity lattice and monotonicity
-- ============================================================================

/-- **Theorem (Purity.le reflexivity)**. -/
theorem Purity_le_refl (p : Purity) : p.le p = true := by
  cases p <;> simp [Purity.le]

/-- **Theorem (Purity stratification)**: `total ≤ pure ≤ impure`. -/
theorem Purity_total_le_pure : Purity.total.le Purity.pure = true := by
  simp [Purity.le]

theorem Purity_pure_le_impure : Purity.pure.le Purity.impure = true := by
  simp [Purity.le]

theorem Purity_total_le_impure : Purity.total.le Purity.impure = true := by
  simp [Purity.le]

/-- **Theorem (Purity.le transitivity)**: the purity lattice is transitive. -/
theorem Purity_le_trans (p q r : Purity)
    (hpq : p.le q = true) (hqr : q.le r = true) : p.le r = true := by
  cases p <;> cases q <;> cases r <;> simp_all [Purity.le]

/-- **Theorem (purity monotonicity for `respectsPurity`)**:
    a more-restrictive purity check implies a less-restrictive one.
    Concretely: if a statement is @total, it is also @pure and @impure. -/
theorem respectsPurity_mono (s : ControlStmt) (p q : Purity)
    (hpq : p.le q = true) (h : s.respectsPurity p = true) :
    s.respectsPurity q = true := by
  induction s with
  | skip => simp [ControlStmt.respectsPurity]
  | assign _ _ => simp [ControlStmt.respectsPurity]
  | seq s₁ s₂ ih₁ ih₂ =>
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h ⊢
    exact ⟨ih₁ h.1, ih₂ h.2⟩
  | ifThenElse _ s₁ s₂ ih₁ ih₂ =>
    simp only [ControlStmt.respectsPurity, Bool.and_eq_true] at h ⊢
    exact ⟨ih₁ h.1, ih₂ h.2⟩
  | whileLoop _ body ih =>
    -- The interesting case: a whileLoop under total is `false`, so the
    -- hypothesis would be `false = true` (vacuous). Under pure or impure,
    -- it recurses.
    cases p with
    | total =>
      simp [ControlStmt.respectsPurity] at h
    | pure =>
      cases q with
      | total => simp [Purity.le] at hpq
      | pure => exact h
      | impure =>
        simp only [ControlStmt.respectsPurity] at h ⊢
        exact ih h
    | impure =>
      cases q with
      | total => simp [Purity.le] at hpq
      | pure => simp [Purity.le] at hpq
      | impure => exact h
  | print _ =>
    -- print under @total or @pure is false; only @impure permits it.
    cases p with
    | total => simp [ControlStmt.respectsPurity] at h
    | pure  => simp [ControlStmt.respectsPurity] at h
    | impure =>
      cases q with
      | total  => simp [Purity.le] at hpq
      | pure   => simp [Purity.le] at hpq
      | impure => simp [ControlStmt.respectsPurity]
  | reverseBlock _ =>
    -- reverseBlock is true at every purity level, so monotonicity is trivial.
    simp [ControlStmt.respectsPurity]

-- ============================================================================
-- SECTION 5: The v2 Pure Function Rule formalisation
-- ============================================================================

/-- **Definition (well-formed function declaration)**: per the v2 grammar's
    Pure Function Rule, a function with declared purity `p` must have a body
    that respects `p`. This is exactly `checkPurity` from `JtvTypes.lean`. -/
def FuncDecl.wellFormed (decl : FuncDecl) : Prop :=
  checkPurity decl = true

/-- **Theorem (wellFormed ⇔ body respects declared purity)**: the well-formed
    predicate is precisely the v2 grammar's static check. -/
theorem wellFormed_iff (decl : FuncDecl) :
    decl.wellFormed ↔ decl.body.respectsPurity decl.purity = true :=
  Iff.rfl

/-- **Theorem (a @total function has no while loops AND no IO in its body)**:
    the structural consequence of the @total purity marker. Derived by
    combining `wellFormed_iff` and `respectsPurity_total_iff_noWhileLoops_noIO`. -/
theorem total_function_no_while_loops_no_io (decl : FuncDecl)
    (htotal : decl.purity = Purity.total) (hwf : decl.wellFormed) :
    decl.body.noWhileLoops = true ∧ decl.body.noIO = true := by
  rw [wellFormed_iff] at hwf
  rw [htotal] at hwf
  exact (respectsPurity_total_iff_noWhileLoops_noIO decl.body).mp hwf

/-- Corollary: a @total function has no while loops in its body. -/
theorem total_function_no_while_loops (decl : FuncDecl)
    (htotal : decl.purity = Purity.total) (hwf : decl.wellFormed) :
    decl.body.noWhileLoops = true :=
  (total_function_no_while_loops_no_io decl htotal hwf).1

/-- Corollary: a @total function performs no IO. -/
theorem total_function_no_io (decl : FuncDecl)
    (htotal : decl.purity = Purity.total) (hwf : decl.wellFormed) :
    decl.body.noIO = true :=
  (total_function_no_while_loops_no_io decl htotal hwf).2

/-- **Theorem (@total ⇒ @pure)**: a function declared @total is automatically
    a valid @pure function — the formal counterpart of "Total ⊂ Pure" from
    the v2 purity lattice. -/
theorem total_implies_pure (decl : FuncDecl)
    (htotal : decl.purity = Purity.total) (hwf : decl.wellFormed) :
    decl.body.respectsPurity Purity.pure = true := by
  rw [wellFormed_iff, htotal] at hwf
  exact respectsPurity_mono decl.body Purity.total Purity.pure
    Purity_total_le_pure hwf
