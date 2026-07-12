# Control-flow ownership

| Fact | Exact owner | Proof | Limit |
| --- | --- | --- | --- |
| Public execution entrance | `machine.rs`, `Machine::step` | focused `control_flow` tests and step probe | no second step API |
| Identity-specific plan | `machine.rs`, `produce_ordinary_control_flow_step_action` | plan-capture and current-PC no-mutation tests | six assigned identities only |
| Link/control-flow application | `machine.rs`, `apply_ordinary_control_flow_step_action` | link, alias, cadence, and slot tests | no generic branch executor |
| `pc`, `next_pc`, delay context | `cpu/scalars.rs`, `Cpu` | scalar snapshot/restore/commit tests | no pipeline framework |
| Zero-register discard | inspected `cpu/registers.rs`, `Cpu::set_gpr` | `JALR rd=0` plus existing register tests | unchanged owner |
| Count/Compare latch | `cpu/cop0.rs`, `advance_count_for_committed_step` | branch and slot Count assertions; JAL Compare latch | no interrupt delivery |
| Decode/identity | inspected `cpu/instruction.rs` | existing decode tests and six-identity planning test | recognition alone is not execution |
| Exception entry | `cpu/cop0.rs` | three delay-slot exception classes | represented local exceptions only |
| No-window proof | `fn64_step_probe.rs` | deterministic case markers | proof shell owns no machine state |

`LIVE_REPO_FACT`: the concurrently active PIF mapping lane owns no listed
source path. No additional writable-path exception was used.
