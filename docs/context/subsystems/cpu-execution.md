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

`BEQ`, `BNE`, non-linking/non-likely `BLTZ`, exact `BEQL`, `BNEL`, `BLEZL`,
`BGEZL`, linking/non-likely `BGEZAL`, `J`, `JAL`, `JR`, and `JALR` share one bounded Machine
planning/application family. BLTZ reuses the exact
full-GPR signed comparison already used by SLT/SLTI; it does not create an
execution-width owner. Target and link arithmetic is explicit, including PC+4
jump-region selection, PC+8 links, JALR alias read-before-write, and r0 discard.
Ordinary taken/untaken branches and taken likely branches schedule one slot;
not-taken likely branches annul PC+4 without execution or Count. A slot exception uses the owning
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
is unknown, so that instance rejects without mutation. One exact MI_VERSION
target reads immutable word `0x02020102` from the Mi owner; the destination
gets ordinary Lw provenance while device state stays unchanged.

Aligned `Sw` is a separate Machine-owned action. It reads the old base,
resolves alignment before source consumption, accepts direct RDRAM/SP IMEM and
the exact represented RI, MI, global RDRAM, or generated RCP-2 module-register
targets, then captures old source low word and exact
lineage. RI_CONFIG planning rejects undefined high bits and selects only its
input/enable fields. RI_CURRENT_LOAD planning requires stored RI_CONFIG and
snapshots those fields into one event. Success writes no GPR and advances
normal cadence once. RI_SELECT accepts only `0x14`, replaces its stored source,
and does not consult the two earlier RI facts as authorization; no RI route
changes memory. RI_MODE stores its defined low-nibble fields, rejects nonzero
bits above bit 3, and does not consult prior RI facts as authorization.
MI_INIT_MODE accepts only `0x0000010F`, stores length 15 plus initialization
mode true with source provenance, and arms one exact 16-byte pending transfer.
That transfer blocks other represented commits and is consumed only by global
RDRAM_DELAY low word `0x18082838`. Success stores logical fields 5/7/3/1 with
complete CPU/MI lineage and leaves post-transfer current MI state unavailable.
The exact first-responder route accepts only physical `0x03F08004` and low
word zero, then records a bounded requested initial device ID with CPU-store
provenance. It is not a ranged device route and creates no responder,
per-module, completion, or routing state.
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
RI_CURRENT_LOAD, RI_MODE, and RI_REFRESH state. The aligned-`Lw`
planner reads RI_SELECT physical `0x0470000C` and RI_REFRESH at `0x04700010`;
aligned `Sw` writes
RI_MODE physical `0x04700000`, RI_CONFIG physical `0x04700004`,
RI_CURRENT_LOAD physical `0x04700008`, or RI_SELECT physical `0x0470000C`, with
CPU-store provenance and ordinary cadence. The event consumes stored RI_CONFIG
without creating a hardware result. RI_SELECT exact-write replaces value and
source, and the existing `Lw` observes it without side effects. No path derives
state from reset kind or generalizes CPU device access. RI_CONFIG/
RI_CURRENT_LOAD/RI_MODE reads and general RI_SELECT programming remain absent.
RI_REFRESH raw word `0x001E3634` has no timing effect.

One private Machine-owned MI_VERSION identity is always available and remains
`0x02020102` through reset and bootstrap. Exact aligned `Lw` at physical
`0x04300004` returns it with ordinary cadence. A separate MI initialization
fact is unavailable at construction, reset, and cold bootstrap. Exact aligned `Sw` to physical
`0x04300000` creates length 15 / initialization mode true only for word
`0x0000010F`; other words, nearby MI addresses, and unknown sources reject
atomically. There is no other MI `Lw`, other MI register, physical next-write
replication or timing owner. Exact generated `0x2000`/`0x1000` commands toggle
one MI-owned RDRAM-register mode; module reads reject while it is disabled.

`MULTU` is the one newly represented multiply identity and uses unsigned
low-32-bit operands with the architecturally defined HI/LO word results. `LBU`
and `SB` are represented only over direct SP IMEM. Aligned opaque-word `Lw`
transports unavailable lineage using canonical zero backing during generated
frame teardown; the backing never becomes known truth and later consumers
still reject. These identities retain ordinary alias, zero-register,
source-knownness, delay-slot, exception, and Count ownership.

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
through the existing `Lui`/`Ori` identities, commits global RDRAM_DELAY and raw
zero global RDRAM_REF_ROW, executes the DEVICE_ID-value `Lui`, and commits the
exact global RDRAM_DEVICE_ID request without relocating bytes. Fourteen
CPU-local setup instructions then reach MI_VERSION. Its exact `Lw` returns
`0x02020102`; generated comparison against `0x01010101` takes the RCP 2.0
branch, executes its Nop slot once, selects spacing `0x400`, and builds
first-responder base `0xFFFFFFFFA3F08000`. The exact first-responder zero store
commits a bounded assignment request, and `Addiu` constructs
`0xFFFFFFFFA3F0000C` as the initial RDRAM_MODE address. The following JAL at
`0xA40001A0` replaces retained bootstrap r31 with PC+8 and exact JAL lineage;
its Nop slot executes once. Five InitCCValue entry instructions then commit.
The four aligned r2-r5 stores commit cause-known, value-unavailable words only
to SP IMEM; twenty following known-source saves commit through `0xA40008EC`.
`Jal 0xA4000984` and its Nop slot then commit. FindCC setup reaches exact BEQL
word `0x53400018`; complete available 64-bit operands compare unequal, so its
`0xA40009A0` slot is architecturally annulled with no execution, commit, Count,
effect, exception, or delay context. TestCCValue and WriteCC commit through
public stepping and construct `0x46C0C0C0`. The physical RDRAM_MODE
`0x03F0000C` store commits in the existing BNE slot as one request fact.
WriteCC restores ra/sp and returns through JR/Nop. From the accepted 32,266-step
state, 214,734 further public steps execute the complete deterministic digital
calibration and module-discovery/finalization path, RI_REFRESH, detected-size
store, and frame teardown. Total commits are 247,000 at PC/next-PC
`0xA4000400 / 0xA4000404`, Count `246984`. The next 5,367 public steps
commit zero TagLo/TagHi, 512 I-cache and 512 D-cache Index Store Tag
operations, exact SP control, 205 ordinary relocation loads/stores, the JR to
`0x80000004`, one real KSEG0 I-cache fill, and six relocated CPU-local or
cartridge-read instructions. PC/next-PC become
`0x8000001C / 0x80000020`, Count `252351`, total commits 252,367. The next
7,225,461 public steps program and complete the atomic PI copy, use general
`BGEZAL` plus existing scalar/control-flow identities, and execute the full
one-MiB checksum through KSEG0 D-cache `Lw`. Exact device clears, boot globals,
SP teardown, and JR/Nop reach `0x80001000 / 0x80001004`, Count `7477812`, total
commits 7,477,828, without executing the synthetic entry. This is synthetic
CPU/device composition, not authentic IPL2/cartridge or RSP execution, PI
timing, dirty D-cache behavior, or analog/cache timing accuracy. Known unknowns
include full ISA integration, real timing, unearned likely/REGIMM and broader
COP0/CACHE identities, NMI, generic MMIO, unrelated load/store families, and
performance.
Next authority must be earned by a bounded product packet, not a generic
dispatcher.
