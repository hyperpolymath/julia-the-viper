/-
  Julia the Viper - Denotational Semantics Core Definitions

  This file defines the semantic domains and evaluation functions for JtV's
  Data Language (the Total, addition-only expression sublanguage).

  The key insight: By grammatically separating Control (Turing-complete) from
  Data (Total), we achieve code injection impossibility as an architectural
  guarantee, not a runtime check.
-/

-- Semantic Domains

/-- State: maps variable names (strings) to integers -/
abbrev State := String → Int

/-- The empty state (all variables map to 0) -/
def State.empty : State := fun _ => 0

/-- State update: σ[x ↦ v] -/
def State.update (σ : State) (x : String) (v : Int) : State :=
  fun y => if y == x then v else σ y

notation:max σ "[" x " ↦ " v "]" => State.update σ x v

-- Abstract Syntax for the Data Language (Total sublanguage)

/-- Terms: the base syntactic elements -/
inductive Term where
  | lit : Int → Term           -- Integer literal n
  | var : String → Term        -- Variable reference x
  deriving Repr, DecidableEq

/-- Expressions: terms composed with addition -/
inductive Expr where
  | term : Term → Expr                    -- Single term t
  | add : Term → Term → Expr              -- Addition t₁ + t₂
  deriving Repr, DecidableEq

-- Evaluation Functions (Denotational Semantics)

/-- Term evaluation: ⟦t⟧ₜ(σ) -/
def evalTerm (t : Term) (σ : State) : Int :=
  match t with
  | Term.lit n => n        -- ⟦n⟧ₜ(σ) = n
  | Term.var x => σ x      -- ⟦x⟧ₜ(σ) = σ(x)

/-- Expression evaluation: ⟦e⟧ₑ(σ) -/
def evalExpr (e : Expr) (σ : State) : Int :=
  match e with
  | Expr.term t => evalTerm t σ                          -- ⟦t⟧ₑ(σ) = ⟦t⟧ₜ(σ)
  | Expr.add t₁ t₂ => evalTerm t₁ σ + evalTerm t₂ σ      -- ⟦t₁ + t₂⟧ₑ(σ) = ⟦t₁⟧ₜ(σ) + ⟦t₂⟧ₜ(σ)

-- Notation for semantic brackets
notation:max "⟦" t "⟧ₜ(" σ ")" => evalTerm t σ
notation:max "⟦" e "⟧ₑ(" σ ")" => evalExpr e σ

-- Extended AST for full Data Language (includes nested additions)

/-- Extended expression syntax supporting arbitrary addition chains -/
inductive DataExpr where
  | lit : Int → DataExpr              -- Integer literal
  | var : String → DataExpr           -- Variable reference
  | add : DataExpr → DataExpr → DataExpr  -- Addition
  | neg : DataExpr → DataExpr         -- Negation (for subtraction via addition)
  deriving Repr, DecidableEq

/-- Size of a DataExpr (for termination proofs) -/
def DataExpr.size : DataExpr → Nat
  | lit _ => 1
  | var _ => 1
  | add e₁ e₂ => 1 + e₁.size + e₂.size
  | neg e => 1 + e.size

/-- Evaluation of extended data expressions.
    Structural recursion on the DataExpr argument; keeping this definitionally
    reducing matters for the `example : evalDataExpr ... = 5 := rfl` checks
    and for any downstream `simp [evalDataExpr]` proof. -/
def evalDataExpr (e : DataExpr) (σ : State) : Int :=
  match e with
  | DataExpr.lit n => n
  | DataExpr.var x => σ x
  | DataExpr.add e₁ e₂ => evalDataExpr e₁ σ + evalDataExpr e₂ σ
  | DataExpr.neg e => -(evalDataExpr e σ)

notation:max "⟦" e "⟧ᴰ(" σ ")" => evalDataExpr e σ

-- ============================================================================
-- v2 GRAMMAR: Reversible operations and reverse-block statements
--
-- These were previously defined in JtvTheorems / JtvReversibility. Moved here
-- so that `ControlStmt` can directly embed `ReversibleStmt` via its
-- `reverseBlock` constructor — matching the EBNF (`spec/grammar.ebnf:22`)
-- which places `reverse_block` inside `control_stmt`.
-- ============================================================================

/-- A single reversible assignment: either `x += e` or `x -= e`. Corresponds
    to the v2 grammar's `reversible_assignment` (spec/grammar.ebnf:43-44). -/
inductive RevOp where
  | addAssign : String → DataExpr → RevOp
  | subAssign : String → DataExpr → RevOp
  deriving Repr

/-- Execute a reversible operation forward. -/
def RevOp.execForward (op : RevOp) (σ : State) : State :=
  match op with
  | addAssign x e => σ[x ↦ σ x + evalDataExpr e σ]
  | subAssign x e => σ[x ↦ σ x - evalDataExpr e σ]

/-- Execute a reversible operation backward (the auto-generated inverse). -/
def RevOp.execBackward (op : RevOp) (σ : State) : State :=
  match op with
  | addAssign x e => σ[x ↦ σ x - evalDataExpr e σ]
  | subAssign x e => σ[x ↦ σ x + evalDataExpr e σ]

/-- The target variable of a `RevOp`. -/
def RevOp.target : RevOp → String
  | RevOp.addAssign x _ => x
  | RevOp.subAssign x _ => x

/-- The expression of a `RevOp`. -/
def RevOp.expr : RevOp → DataExpr
  | RevOp.addAssign _ e => e
  | RevOp.subAssign _ e => e

/-- A reversible statement, corresponding to the v2 grammar's `reversible_stmt`
    (`spec/grammar.ebnf:42`). We use a tree-of-trees encoding for structural
    recursion: `revSkip` / `revAssign` / `revSeq` / `revIf` together cover
    sequences of `reversible_assignment`s with optional conditional dispatch. -/
inductive ReversibleStmt : Type where
  | revSkip   : ReversibleStmt
  | revAssign : RevOp → ReversibleStmt
  | revSeq    : ReversibleStmt → ReversibleStmt → ReversibleStmt
  | revIf     : DataExpr → ReversibleStmt → ReversibleStmt → ReversibleStmt
  deriving Repr

/-- Forward execution of a reversible statement. -/
def ReversibleStmt.execForward : ReversibleStmt → State → State
  | ReversibleStmt.revSkip,            σ => σ
  | ReversibleStmt.revAssign op,       σ => RevOp.execForward op σ
  | ReversibleStmt.revSeq s₁ s₂,       σ => s₂.execForward (s₁.execForward σ)
  | ReversibleStmt.revIf cond thn els, σ =>
    if evalDataExpr cond σ ≠ 0
    then thn.execForward σ
    else els.execForward σ

/-- Backward execution: the dual. -/
def ReversibleStmt.execBackward : ReversibleStmt → State → State
  | ReversibleStmt.revSkip,            σ => σ
  | ReversibleStmt.revAssign op,       σ => RevOp.execBackward op σ
  | ReversibleStmt.revSeq s₁ s₂,       σ => s₁.execBackward (s₂.execBackward σ)
  | ReversibleStmt.revIf cond thn els, σ =>
    if evalDataExpr cond σ ≠ 0
    then thn.execBackward σ
    else els.execBackward σ

/-- The (over-approximated) set of variables a reversible statement may
    write to. -/
def ReversibleStmt.writes : ReversibleStmt → List String
  | ReversibleStmt.revSkip            => []
  | ReversibleStmt.revAssign op       => [op.target]
  | ReversibleStmt.revSeq s₁ s₂       => s₁.writes ++ s₂.writes
  | ReversibleStmt.revIf _ thn els    => thn.writes ++ els.writes

-- ============================================================================
-- Control Language AST
-- ============================================================================

/-- An execution trace: the list of values emitted by `print` statements,
    in evaluation order. This is the formal model of v2's IO surface. -/
abbrev Trace := List Int

/-- Control statements: the Turing-complete imperative fragment.
    v2 additions (`spec/grammar.ebnf:22, 36, 41`):
      * `print`        — the only IO constructor; `print(e₁, …, eₙ)`
      * `reverseBlock` — a `reverse { … }` block embedded as control flow -/
inductive ControlStmt where
  | assign : String → DataExpr → ControlStmt          -- x = e
  | seq : ControlStmt → ControlStmt → ControlStmt     -- s₁ ; s₂
  | ifThenElse : DataExpr → ControlStmt → ControlStmt → ControlStmt
  | whileLoop : DataExpr → ControlStmt → ControlStmt
  | skip : ControlStmt
  | print : List DataExpr → ControlStmt               -- v2: print(e₁, …, eₙ)
  | reverseBlock : ReversibleStmt → ControlStmt       -- v2: reverse { … }
  deriving Repr

-- Note: We intentionally do NOT define a general evaluator for ControlStmt
-- because while loops may not terminate. The fueled `execStmt` below is
-- partial in the same way; `print` and `reverseBlock` are total.

/-- Single-step execution of Control statements with a fuel bound for loops.
    Returns both the resulting state and an output trace; non-`print`
    statements emit the empty trace. -/
def execStmt (s : ControlStmt) (σ : State) (fuel : Nat) : Option (State × Trace) :=
  match fuel with
  | 0 => none  -- Out of fuel (potential non-termination)
  | fuel' + 1 =>
    match s with
    | ControlStmt.skip => some (σ, [])
    | ControlStmt.assign x e => some (σ[x ↦ evalDataExpr e σ], [])
    | ControlStmt.print args =>
        some (σ, args.map (fun e => evalDataExpr e σ))
    | ControlStmt.reverseBlock body =>
        some (body.execForward σ, [])
    | ControlStmt.seq s₁ s₂ =>
        match execStmt s₁ σ fuel' with
        | some (σ', t₁) =>
            match execStmt s₂ σ' fuel' with
            | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
            | none => none
        | none => none
    | ControlStmt.ifThenElse e s₁ s₂ =>
        if evalDataExpr e σ ≠ 0
        then execStmt s₁ σ fuel'
        else execStmt s₂ σ fuel'
    | ControlStmt.whileLoop e s =>
        if evalDataExpr e σ ≠ 0
        then match execStmt s σ fuel' with
             | some (σ', t₁) =>
                 match execStmt (ControlStmt.whileLoop e s) σ' fuel' with
                 | some (σ'', t₂) => some (σ'', t₁ ++ t₂)
                 | none => none
             | none => none
        else some (σ, [])

-- Free variables (for state independence analysis)

/-- Free variables in a term -/
def Term.freeVars : Term → List String
  | lit _ => []
  | var x => [x]

/-- Free variables in an expression -/
def Expr.freeVars : Expr → List String
  | term t => t.freeVars
  | add t₁ t₂ => t₁.freeVars ++ t₂.freeVars

/-- Free variables in an extended data expression -/
def DataExpr.freeVars : DataExpr → List String
  | lit _ => []
  | var x => [x]
  | add e₁ e₂ => e₁.freeVars ++ e₂.freeVars
  | neg e => e.freeVars

-- Substitution (for compositional reasoning)

/-- Substitute a variable with a value in a data expression -/
def DataExpr.subst (e : DataExpr) (x : String) (v : Int) : DataExpr :=
  match e with
  | lit n => lit n
  | var y => if y == x then lit v else var y
  | add e₁ e₂ => add (e₁.subst x v) (e₂.subst x v)
  | neg e => neg (e.subst x v)

-- Constants for common expressions

def DataExpr.zero : DataExpr := DataExpr.lit 0
def DataExpr.one : DataExpr := DataExpr.lit 1

-- Example expressions for testing

example : evalTerm (Term.lit 42) State.empty = 42 := rfl
example : evalTerm (Term.var "x") (State.empty["x" ↦ 10]) = 10 := rfl
example : evalExpr (Expr.add (Term.lit 2) (Term.lit 3)) State.empty = 5 := rfl

example : evalDataExpr (DataExpr.add (DataExpr.lit 2) (DataExpr.lit 3)) State.empty = 5 := rfl
example : evalDataExpr (DataExpr.neg (DataExpr.lit 5)) State.empty = -5 := rfl
