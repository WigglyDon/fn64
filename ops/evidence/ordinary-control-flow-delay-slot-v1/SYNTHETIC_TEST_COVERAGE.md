# Synthetic test coverage

| Requirement | Focused proof |
| --- | --- |
| six pure plans | `control_flow_planning_captures_all_six_identities_without_mutation` |
| integrated pure producer | `current_pc_control_flow_producer_keeps_every_planned_fact_pre_mutation` |
| BEQ taken/untaken/positive | `control_flow_beq_taken_and_untaken_each_execute_one_delay_slot` |
| BEQ negative/zero/same register | `control_flow_beq_zero_same_register_and_negative_target_rules_are_explicit` |
| BNE taken/untaken/positive/negative | `control_flow_bne_taken_untaken_positive_and_negative_targets` |
| wrapping formulas | `control_flow_target_and_link_arithmetic_wraps_explicitly` |
| J region and one slot | `control_flow_jump_uses_pc_plus_four_region_and_executes_slot_once` |
| JAL link visible in slot | `control_flow_jal_writes_link_before_delay_slot_execution` |
| JR/JALR distinct, alias, rd=0 | `control_flow_jr_and_jalr_capture_old_source_before_link_write` |
| all-six inner rejection | `branch_in_delay_slot_rejects_all_six_identities_without_mutation` |
| overflow slot exception | `branch_delay_exception_arithmetic_overflow_uses_owner_epc_and_no_slot_count` |
| fetch-AdEL slot exception | `branch_delay_exception_instruction_fetch_adel_uses_explicit_test_staging` |
| data-AdEL untaken slot | `branch_delay_exception_data_adel_handles_untaken_branch_context` |
| reset/direct staging | `delay_slot_context_reset_and_direct_pc_staging_clear_stale_state` |
| bootstrap honesty | `control_flow_bootstrap_unknown_sources_and_link_lineage_reject_before_mutation` |
| public no-window composition | six `control-flow-*` step-probe markers |

All inputs are generated instruction words and zero/default storage. No private
ROM, PIF firmware, downloaded image, or proprietary byte payload is used.
