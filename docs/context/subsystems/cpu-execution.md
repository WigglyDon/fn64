# CPU Execution

Context role: CPU execution architecture context.
Scope: Rust CPU state, helpers, and Machine-owned execution cadence.
Canonical for: CPU mutation ownership and execution-layer dependency rules.
Not canonical for: exhaustive instruction support or retired implementation behavior.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md), `cpu.rs`, and `machine.rs` tests.
Update triggers: helper selection, public execution, cadence, or CPU state ownership changes.

## Mission and authority

CPU code owns represented GPR, HI/LO, PC/next-PC, COP0 subset, instruction
identity, and narrowly selected local helper mutation. `Machine::step` is the
sole public represented execution entrance and owns composition/application.
There is no public `Cpu::step` or generic all-future executor.

CPU may depend on decoded fixed-width fields and owned state. It must not depend
on host time, renderer/input/audio, probe assertions, Git context, or generic
device registries. Machine cadence may call sealed CPU mutation primitives;
helpers must not call back into Machine policy.

## Cadence and cause

`pre-step pc/next_pc → one snapshot → one sequential staging → one fetch/decode/identify → one selected action → committed cadence, rollback, stop, rejection, or exception`.

Read-before-write and zero-register behavior stay explicit. `pc` / `next_pc`
models delay-slot-relevant state without claiming unselected branch execution.
Count advances only through the committed-step owner. Exception actions restore
or preserve control flow before delegating to the sealed entry owner.

Machine-owned bootstrap state distinguishes concrete GPR storage from known
architectural state. Each selected CPU-local bootstrap instruction checks all
consumed GPR sources before helper invocation. The accepted BOOT-2
`SpecialAdd` reads known r29 and r0, writes known r9=`0xFFFFFFFFA4001FF0`, and
commits cadence once; the following `Lw` remains unrepresented.

Accepted proof: current source anchors, CPU/helper unit tests, focused Machine
step tests, and the synthetic step probe. Historical output cannot establish
current behavior by itself.
Current observability is deterministic state inspection; no instruction trace
format is yet a runtime product surface.

Required validation: `./rust/verify-forward`, plus focused instruction-family
tests for changes. Known unknowns include complete public-step ISA integration,
real timing, full delay-slot semantics, aligned load execution, and
performance. Next authority must be earned by a bounded product packet, not a
generic dispatcher.
