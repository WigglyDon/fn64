# BLTZ ownership

- `cpu/instruction.rs` owns REGIMM decode, the exact `RegimmBltz` identity,
  and the existing full-width signed-GPR comparison helper.
- `machine.rs::produce_ordinary_control_flow_step_action` owns the immutable
  identity-specific plan: old r31 value, condition, target, fall-through,
  source register, and control-flow snapshot.
- `machine.rs::ordinary_control_flow_rejection` owns bootstrap source
  knownness before application.
- `machine.rs::apply_ordinary_control_flow_step_action` and the existing CPU
  control-flow owner schedule one delay slot and advance Count once.
- CPU scalar state owns `pc`, `next_pc`, and the branch-PC delay context.
- Existing COP0 paths own any exception raised by the later slot instruction.

No generic REGIMM dispatcher, branch predicate framework, mode system, link
owner, annul owner, or second public step entrance is added.
