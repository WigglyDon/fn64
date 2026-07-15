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

`BEQ`, `BNE`, non-linking/non-likely `BLTZ`, `J`, `JAL`, `JR`, and `JALR`
share one bounded Machine planning/application family. BLTZ reuses the exact
full-GPR signed comparison already used by SLT/SLTI; it does not create an
execution-width owner. Target and link arithmetic is explicit, including PC+4
jump-region selection, PC+8 links, JALR alias read-before-write, and r0 discard.
Taken and untaken branches schedule one slot. A slot exception uses the owning
branch/jump PC for EPC and sets BD; inner control flow is rejected before
source consumption or mutation.

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
destination lineage, and commits cadence once. Its targets now include
cartridge-bootstrap-staged SP DMEM with exact source-offset provenance; other
concrete SP-DMEM backing is not treated as known. Its authentic SP IMEM source
is unknown, so that instance rejects without mutation.

Aligned `Sw` is a separate Machine-owned action. It reads the old base,
resolves alignment before source consumption, accepts direct SP IMEM or exact
RI_MODE/RI_CONFIG/RI_CURRENT_LOAD/RI_SELECT/MI_INIT_MODE only, then captures old source low word and exact
lineage. RI_CONFIG planning rejects undefined high bits and selects only its
input/enable fields. RI_CURRENT_LOAD planning requires stored RI_CONFIG and
snapshots those fields into one event. Success writes no GPR and advances
normal cadence once. RI_SELECT accepts only `0x14`, replaces its stored source,
and does not consult the two earlier RI facts as authorization; no RI route
changes memory. RI_MODE stores its defined low-nibble fields, rejects nonzero
bits above bit 3, and does not consult prior RI facts as authorization.
MI_INIT_MODE accepts only `0x0000010F`, stores length 15 plus initialization
mode true with source provenance, and does not authorize the following RDRAM
write.
Sequential or delay-slot
AdES delegates to COP0
with zero faulting-instruction Count; unknown sources and unsupported targets
restore the complete pre-step state.

Bounded `Cop0Mtc0` is a closed Machine-owned action for Cause software
pending, Count, and Compare only. It reads a known old source, transfers its
low word, and requires the source-backed cold-x105 kernel access state. Cause
updates only IP1/IP0; Count writes before normal cadence; Compare clears timer
pending before normal cadence. Unsupported contexts, encodings, and
destinations reject before mutation, and no general CP0 executor exists.

The same complete cold-x105 plan creates optional Machine-owned RI_SELECT zero
with `ColdX105Entry` provenance and clears optional RI_CONFIG,
RI_CURRENT_LOAD, and RI_MODE state. The aligned-`Lw`
planner reads only RI_SELECT physical `0x0470000C`; aligned `Sw` writes only
RI_MODE physical `0x04700000`, RI_CONFIG physical `0x04700004`,
RI_CURRENT_LOAD physical `0x04700008`, or RI_SELECT physical `0x0470000C`, with
CPU-store provenance and ordinary cadence. The event consumes stored RI_CONFIG
without creating a hardware result. RI_SELECT exact-write replaces value and
source, and the existing `Lw` observes it without side effects. No path derives
state from reset kind or generalizes CPU device access. RI_CONFIG/
RI_CURRENT_LOAD/RI_MODE reads, general RI_SELECT programming, calibration,
and timing remain absent.

One private Machine-owned MI initialization fact is unavailable at
construction, reset, and cold bootstrap. Exact aligned `Sw` to physical
`0x04300000` creates length 15 / initialization mode true only for word
`0x0000010F`; other words, nearby MI addresses, and unknown sources reject
atomically. There is no MI `Lw`, other MI register, physical next-write
replication, RDRAM-register state, or timing owner.

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
tests for changes. Generated composition commits the exact RI_CONFIG `Sw`,
installs wait count 8,000, executes 8,000 four-instruction loop iterations,
commits RI_CURRENT_LOAD, `Ori r9,r0,0x14`, exact RI_SELECT `Sw`, both RI_MODE
stores, the four-iteration wait, and the 32-iteration wait with ORI in every
BNE delay slot. It then commits exact MI_INIT_MODE, constructs `0x18082838`
through the existing `Lui`/`Ori` identities, and rejects the global
RDRAM_DELAY `Sw` as a direct target miss. This is CPU
composition, not elapsed RI time or calibration. Known
unknowns include complete public-step ISA integration, real timing,
branch-likely/other REGIMM and broader COP0 execution, every RI action except
the exact RI_SELECT read/`0x14` write, RI_CONFIG write, RI_CURRENT_LOAD event,
RI_MODE defined-field writes, and the exact MI_INIT_MODE write, NMI,
generic MMIO, nested control flow, other load/store families, and
performance. Next authority must be earned by a bounded product packet, not a generic dispatcher.
