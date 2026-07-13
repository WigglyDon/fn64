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

`pre-step pc/next_pc/context → one snapshot → one fetch/decode/identify → ordinary-control-flow planning or sequential staging → one selected action → committed cadence, rollback, stop, rejection, or exception`.

Read-before-write and zero-register behavior stay explicit. `pc` / `next_pc`
and one CPU-owned context now represent a selected ordinary delay slot. Count
advances only through the committed-step owner. Exception actions restore or
preserve control flow before delegating to the sealed entry owner.

`BEQ`, `BNE`, `J`, `JAL`, `JR`, and `JALR` share one bounded Machine
planning/application family. Target and link arithmetic is explicit, including
PC+4 jump-region selection, PC+8 links, JALR alias read-before-write, and r0
discard. Taken and untaken branches schedule one slot. A slot exception uses
the owning branch/jump PC for EPC and sets BD; inner control flow is rejected
before mutation.

Machine-owned bootstrap state distinguishes concrete GPR storage from known
architectural state. Each selected CPU-local bootstrap instruction checks all
consumed GPR sources before helper invocation. The generated-only supported
NTSC cold-x105 handoff marks exactly t3, sp, ra, and s3-s7 with distinct
Machine-owned lineage; all other inherited GPRs remain unknown unless a later
instruction produces them. Its ra value is the complete retained link
`0xFFFFFFFFA4001550`, not merely the negative relation consumed by the first
x105 branch. The accepted authentic BOOT-2
`SpecialAdd` reads known r29 and r0, writes known r9=`0xFFFFFFFFA4001FF0`, and
commits cadence once. The following aligned `Lw` is represented through one
Machine-owned plan/application rule; it reads its old base before destination
write, sign-extends the loaded word, preserves GPR zero, records successful
destination lineage, and commits cadence once. Its authentic SP IMEM source is
unknown, so that instance rejects without mutation.

Coupled staging also owns Status=`0x34000000`,
PC/next-PC=`0xA4000040 / 0xA4000044`, and a clear delay-slot context. It does
not source Count, Compare, EPC, BadVAddr, Cause, timer state, or unrelated GPRs.
The private plan is complete before CPU replacement, so rejected profile or
missing-input paths retain the prior CPU, COP0, control-flow, and memory state.

Accepted proof: current source anchors, CPU/helper unit tests, focused Machine
step tests, and the synthetic step probe. Historical output cannot establish
current behavior by itself.
Current observability is deterministic state inspection; no instruction trace
format is yet a runtime product surface.

Required validation: `./rust/verify-forward`, plus focused instruction-family
tests for changes. Known unknowns include complete public-step ISA integration,
real timing, branch-likely/REGIMM/COP0 branches, nested control flow, other
load/store families, and performance. Next authority must be earned by a
bounded product packet, not a generic dispatcher.
