/-
  Julia the Viper — Axiom audit oracle.

  This module imports every JtV proof library and emits `#print axioms` for the
  headline theorems of each. A clean build of this file with no `sorry` and
  only the standard Lean kernel axioms (`propext`, `Classical.choice`,
  `Quot.sound`) is the build oracle for the JtV proof debt clearance.

  If a `sorryAx` or unexpected axiom appears in any of these prints, the proof
  has regressed.
-/

import JtvCore
import JtvTheorems
import JtvOperational
import JtvTypes
import JtvSecurity
import JtvExtended
import JtvReversibility
import JtvPurity
import JtvControlSemantics
import JtvCalls
import JtvTotalSemantics
import JtvBool

-- ============================================================================
-- Headline theorems — one per module — that must show only kernel axioms.
-- ============================================================================

-- JtvCore: no theorems, only definitions; nothing to audit here.

-- JtvTheorems
#print axioms dataExpr_totality
#print axioms free_vars_sufficient
#print axioms subst_correct
#print axioms constFold_correct
#print axioms rev_forward_backward
#print axioms no_infinite_dataExpr
#print axioms dataExpr_no_control

-- JtvOperational
#print axioms data_step_deterministic
#print axioms data_progress
#print axioms data_state_preservation
#print axioms data_terminates
#print axioms bigstep_denotational_equiv

-- JtvTypes
#print axioms data_is_total
#print axioms total_no_loops
#print axioms type_preservation
#print axioms typed_progress
#print axioms infer_sound
#print axioms purity_check_correct
#print axioms typing_coercion_refl
#print axioms coercion_refl

-- JtvSecurity
#print axioms no_vulnerable_constructs
#print axioms no_control_to_data_flow
#print axioms joinpoint_unidirectional
#print axioms no_reverse_joinpoints
#print axioms aold_complete
#print axioms string_not_executable
#print axioms data_language_sandboxed
#print axioms owasp_code_injection_mitigated
#print axioms data_evaluation_secure

-- JtvExtended
#print axioms add_left_cancel
#print axioms add_right_cancel
#print axioms neg_add_distrib
#print axioms sub_eq_add_neg
#print axioms size_pos
#print axioms subexpr_size_lt
#print axioms eval_compositional_add
#print axioms closed_context_independent
#print axioms semEquiv_refl
#print axioms semEquiv_symm
#print axioms semEquiv_trans
#print axioms semEquiv_add_cong
#print axioms semEquiv_neg_cong
#print axioms dead_code_elim
#print axioms simplify_add_zero
#print axioms simplify_zero_add
#print axioms simplify_add_neg_self
#print axioms simplify_neg_neg
#print axioms no_hidden_deps
#print axioms rev_composition_single
#print axioms rev_composition
#print axioms rev_totality
#print axioms strong_normalization
#print axioms confluence
#print axioms control_data_noninterference
#print axioms eval_functorial

-- JtvReversibility (v2 grammar — reverse-block reversibility)
#print axioms rev_composition_subAssign
#print axioms execListForward_nil
#print axioms execListForward_cons
#print axioms execListBackward_nil
#print axioms execListBackward_cons
#print axioms rev_forward_backward_state
#print axioms rev_backward_forward_state
#print axioms revOp_forward_is_bijection
#print axioms addAssign_then_subAssign_is_id
#print axioms subAssign_then_addAssign_is_id
#print axioms execForward_frame
#print axioms execBackward_frame
#print axioms eval_indep_of_other_update
#print axioms execForward_preserves_eval
#print axioms safe_later_ops_preserved
#print axioms rev_composition_list

-- JtvReversibility (v2 auto-inverse semantics — `RevOp.inverse`)
#print axioms revOp_inverse_involutive
#print axioms revOp_execForward_inverse
#print axioms revOp_execBackward_inverse
#print axioms revOp_inverse_safe
#print axioms revOp_forward_eq_backward_of_inverse

-- JtvReversibility (v2 grammar — conditional reverse blocks)
#print axioms ReversibleStmt.execForward_frame
#print axioms cond_preserved_by_frame_safe
#print axioms revIf_reversible
#print axioms revStmt_reversible

-- JtvReversibility (v2 grammar — ReversibleStmt structural + bijection)
#print axioms ReversibleStmt.size_pos
#print axioms ReversibleStmt.execBackward_frame
#print axioms revStmt_backward_forward
#print axioms revStmt_is_bijection

-- JtvPurity (v2 grammar — @pure / @total compositionality and stratification)
#print axioms respectsPurity_seq_iff
#print axioms respectsPurity_ifThenElse_iff
#print axioms respectsPurity_skip
#print axioms respectsPurity_assign
#print axioms whileLoop_not_total
#print axioms whileLoop_pure_iff
#print axioms respectsPurity_total_iff_noWhileLoops_noIO
#print axioms Purity_le_refl
#print axioms Purity_total_le_pure
#print axioms Purity_pure_le_impure
#print axioms Purity_total_le_impure
#print axioms Purity_le_trans
#print axioms respectsPurity_mono
#print axioms wellFormed_iff
#print axioms total_function_no_while_loops
#print axioms total_function_no_while_loops_no_io
#print axioms total_function_no_io
#print axioms total_implies_pure

-- JtvReversibility (v2 grammar — RevTyping inductive judgment, Δ6/Δ7)
#print axioms revTyping_implies_safe
#print axioms safe_implies_revTyping
#print axioms revTyping_iff_safe
#print axioms revStmt_reversible_typed
#print axioms revStmt_is_bijection_typed

-- JtvControlSemantics (v2 grammar — Δ1 + Δ3 trace soundness)
#print axioms execStmt_print_emits
#print axioms execStmt_print_single
#print axioms execStmt_reverseBlock_emits_no_trace
#print axioms execStmt_skip_empty_trace
#print axioms execStmt_assign_empty_trace
#print axioms print_not_noIO
#print axioms reverseBlock_noIO
#print axioms skip_noIO
#print axioms assign_noIO
#print axioms total_implies_noIO
#print axioms total_excludes_print
#print axioms execStmt_seq_split
#print axioms execStmt_while_split_true
#print axioms execStmt_noIO_empty_trace
#print axioms execStmt_total_empty_trace

-- JtvCalls (v2 grammar — function calls in Data context + Pure Function Rule)
#print axioms DataExprC.base_respectsPureFnRule
#print axioms DataExprC.call_respectsPureFnRule
#print axioms DataExprC.respectsPureFnRule_excludes_impure
#print axioms DataExprC.respectsPureFnRule_callees_bound
#print axioms DataExprC.isPureSyntactic_respectsPureFnRule
#print axioms DataExprC.base_isPureSyntactic
#print axioms DataExprC.respectsPureFnRule_iff_all_callable
#print axioms DataExprC.subExprs_respectsPureFnRule
#print axioms FuncEnv.purityCoherent_checkPurity
#print axioms FuncEnv.purityCoherent_total_no_loops_no_io
#print axioms DataExprC.pureFnRule_no_io
#print axioms respectsPurity_pure_implies_noIO
#print axioms DataExprC.pureFnRule_callees_silent

-- JtvTotalSemantics (v2 grammar — Δ5 fuel-free total semantics)
#print axioms totalExec
#print axioms totalExec_skip
#print axioms totalExec_assign
#print axioms totalExec_reverseBlock
#print axioms execStmt_eq_totalExec_when_some
#print axioms execStmt_state_eq_totalExec
#print axioms execStmt_fuel_succ
#print axioms execStmt_fuel_le
#print axioms execStmt_terminates_for_total

-- JtvBool (v2 grammar — Δ4 Bool sublanguage: pure / total / decidable)
#print axioms BoolExpr.size_pos
#print axioms boolExpr_totality
#print axioms boolExpr_deterministic
#print axioms boolExpr_state_unchanged
#print axioms boolExpr_free_vars_sufficient
#print axioms boolExpr_closed_state_indep
#print axioms evalBoolExpr_nonzero_legacy
#print axioms boolExpr_not_not
#print axioms boolExpr_deMorgan_and
#print axioms boolExpr_deMorgan_or
#print axioms boolExpr_and_comm
#print axioms boolExpr_or_comm
#print axioms boolExpr_and_self
#print axioms boolExpr_or_self
#print axioms boolExpr_no_control
#print axioms BoolExpr.size_not_lt
#print axioms BoolExpr.size_and_left
#print axioms BoolExpr.size_and_right
#print axioms BoolExpr.size_or_left
#print axioms BoolExpr.size_or_right
