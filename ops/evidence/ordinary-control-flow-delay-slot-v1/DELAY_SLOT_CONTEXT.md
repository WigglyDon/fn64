# Delay-slot context

Owner: `Cpu` control-flow state in `cpu/scalars.rs`.

Field: `Option<CpuDelaySlotContext>`, where the present value contains exactly
`branch_or_jump_pc: u32`.

State transitions (`B` is branch/jump PC, `S=B+4`, and `N` is target or
untaken fall-through):

| Point | `pc` | `next_pc` | context | Count delta |
| --- | --- | --- | --- | --- |
| before branch plan | `B` | `S` | none | 0 |
| after branch commit | `S` | `N` | owner `B` | +1 |
| before slot execution | `S` | `N` | owner `B` | 0 |
| after successful slot | `N` | `N+4` wrapping | none | +1 |
| after rejected inner control flow | `S` | `N` | owner `B` | 0 |
| after slot exception | exception vector | vector+4 | none | 0 for slot |

- Reset constructs a CPU with no context.
- `Machine::stage_cpu_pc` clears stale context and stages sequential `next_pc`.
- `CpuControlFlowSnapshot` captures and restores context with `pc/next_pc`.
- Both taken and untaken conditional branches schedule context; it is not
  inferred from `pc/next_pc`.
- A test-only CPU owner can stage context for an otherwise unnatural fetch-AdEL
  setup. No public mutable backdoor was added.
- The probe only observes `Machine::cpu_delay_slot_context`; it never authors
  production delay-slot truth.
