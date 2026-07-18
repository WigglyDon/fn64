# Machine Core

Context role: machine-core architecture context.
Scope: represented Rust `Machine` ownership and lifecycle.
Canonical for: machine authority, cause/mutation lineage, and integration boundary.
Not canonical for: the detailed capability ledger or current lane status.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and Rust tests.
Update triggers: Machine ownership, lifecycle, public execution, or state lineage changes.

## Mission and owner

`fn64-core::Machine` is the current production owner of each represented
machine instance: cartridge, CPU, RDRAM, SP DMEM, SP IMEM, minimal RI_MODE,
RI_SELECT, RI_CONFIG, RI_CURRENT_LOAD event, immutable MI_VERSION identity,
and MI initialization-mode state,
reset/power state, optional
structurally accepted immutable
PIF firmware input, optional explicit PIF IPL2 copy profile, explicit narrow
cold-handoff selector inputs, and narrow
machine-owned staging/inspection. It now also owns the narrow normalized
cartridge-bootstrap state, SP-DMEM provenance, and bootstrap GPR-knownness
ledger that earned BOOT-2. SP IMEM has separate backing bytes and per-byte
knownness; concrete reset zero is not an architecturally known value.
Long-term ownership stays with the smallest
host-independent core that actual hardware work earns.

Authority forbidden here includes file paths, CLI parsing, SDL/window/audio,
platform clocks, probe formatting, Git/fleet policy, commercial data, and
host-owned emulator decisions. `fn64-inspection` may depend on public core APIs;
the core must never depend on inspection.

## Lineage and consistency

Every represented step follows:

`synthetic or owned bytes → fetch address/target → instruction word → decoded identity → classified action → one mutation owner → represented result`.

For bootstrap execution, source provenance and consumed-GPR knownness precede
helper invocation. Unknown-source rejection restores staged control flow and
mutates no GPR, HI/LO, COP0, memory, or Count. A successful GPR write records
its producing instruction lineage.

Control-flow snapshots, staged sequential cadence, Count advancement, rollback,
and exception entry remain distinct owners. Ordinary control flow adds one
CPU-owned delay-slot context naming the owning branch/jump PC. Machine planning
and application schedule exactly one slot, preserve context on rejection, and
clear it after a successful slot or exception entry. A green helper test is not
public step integration. `Machine` construction/reset preserves instance
isolation; multiboxing means multiple independent instances.

Numeric values are explicit fixed-width CPU/address/storage types where earned;
RDRAM/SP words use source-clear big-endian access. An aligned `Lw` plan
preflights known bootstrap operands, address classification, alignment, and the
complete source word before application owns writeback, lineage, and cadence.
Direct SP-DMEM reads are additionally gated by the current cartridge-bootstrap
span and record the exact source cartridge offset; concrete but unclassified
backing rejects. No serialization format is a product contract yet.

Aligned `Sw` uses a separate Machine plan/application path for SP IMEM, the
four exact RI_MODE, RI_CONFIG, RI_CURRENT_LOAD, and RI_SELECT targets, exact
MI_INIT_MODE, the three exact global RDRAM writes, or the exact RCP 2.0
first-responder DEVICE_ID target.
The plan resolves old base, alignment, direct target, old source value, exact
SP-IMEM span or destination-specific RI state, and CPU-store provenance before
application. Undefined RI_CONFIG high bits, unavailable RI_CONFIG for an
RI_CURRENT_LOAD event, and RI_SELECT words other than `0x14` reject during
planning. RI_MODE planning accepts its defined low-nibble fields and rejects
nonzero bits above bit 3. MI_INIT_MODE planning accepts only `0x0000010F` and
constructs length 15, initialization mode true, and CPU-store provenance.
That exact MI plan also arms one private 16-byte pending transfer. While it is
pending, other represented successful stores reject; target misses stay closed
and preserve it. Global RDRAM_DELAY planning accepts only physical `0x03F80008`,
low word `0x18082838`, known lineage, and the exact 15/16 transfer. Its plan
stores logical fields 5/7/3/1 and packed configuration `0x28381808` with CPU
and consumed-MI provenance.
Global REF_ROW accepts only low word zero, and global DEVICE_ID accepts only
`0x80000000` as requested base `0x02000000`. Exact first-responder physical
`0x03F08004` accepts only low word zero with known lineage and no pending MI
transfer, then constructs a bounded initial-device-ID assignment request. It
is classified by exact address, not a register range; RCP 1.0 physical
`0x03F04004` remains unsupported.
Application has no
fallible operation: it mutates one selected owner, commits control flow once,
and advances Count once. Rejection restores the captured snapshot; AdES
delegates to existing sealed COP0 entry.

Bounded `Cop0Mtc0` uses another closed plan/application path. The plan captures
the cold-x105 access proof, exact Cause/Count/Compare destination, known old
source, and low transfer word before mutation. Application performs the
destination-specific COP0 write before the existing committed cadence. It is
not a numeric CP0 map or a general register-write framework.

One private `Ri` owner stores optional RI_MODE, RI_SELECT, RI_CONFIG, and
RI_CURRENT_LOAD event state separately from bootstrap selectors. Construction
and general reset leave all unavailable.
The complete supported cold-x105 plan creates RI_SELECT zero with
`ColdX105Entry` provenance and leaves the other three facts unavailable; repeated
complete staging restores that cold value/source and clears stale RI_CONFIG,
RI_CURRENT_LOAD, RI_MODE, and CPU-store provenance. The aligned-`Lw`
plan reads only RI_SELECT at
physical `0x0470000C`. The aligned-`Sw` plan writes only RI_MODE at physical
`0x04700000`, RI_CONFIG at `0x04700004`, RI_CURRENT_LOAD at `0x04700008`, or
RI_SELECT at `0x0470000C`.
RI_CURRENT_LOAD requires and
snapshots stored RI_CONFIG fields while recording the transfer word and
CPU-store lineage. RI_SELECT accepts only exact `0x14`, replaces value/source
with CPU-store provenance, and leaves both siblings unchanged. RI_MODE stores
operating-mode bits 1:0 and the two stop-active bits with CPU-store provenance;
bits above bit 3 reject before mutation. No RI route mutates memory. No path
derives RI state from reset kind. RI_CONFIG/RI_CURRENT_LOAD/RI_MODE reads,
general RI_SELECT programming, current-control
output/processing/timing, NMI, generic MMIO, and a bus remain absent.

One private `Mi` owner separately stores immutable MI_VERSION word
`0x02020102`, optional MI initialization state, and one bounded pending
transfer. IO/RAC/RDP/RSP bytes derive from the one raw word. Construction,
reset, and complete cold-x105 bootstrap preserve identity while leaving mutable
state unavailable; repeated bootstrap clears stale mutable state and failed
bootstrap preserves all state. The
exact physical `0x04300000` store has no CPU read route and creates no other MI
fact. The pending transfer is consumed only by the exact x105 RDRAM_DELAY pair;
post-consumption current MI state becomes unavailable because exact readback is
not source-clear. Exact aligned `Lw` at physical `0x04300004` reads only the
immutable version with ordinary CPU lineage. This is not a general next-write
engine or MI register bank and represents no timing.

The existing `Rdram` remains the sole byte owner and separately stores optional
global/broadcast delay, raw REF_ROW, global DEVICE_ID relocation-request, and
first-responder DEVICE_ID assignment-request facts. The exact REF_ROW route
accepts only low word zero at physical `0x03F80014`, records CPU-store
provenance, and preserves the delay fact. Global DEVICE_ID physical
`0x03F80004` records only requested base `0x02000000`; first-responder physical
`0x03F08004` records only requested initial ID zero and the exact RCP 2.0
aperture. These writes change no RDRAM byte or route, create no module
inventory/presence/completion state, and have no CPU read route. Reset and
complete bootstrap clear all optional RDRAM facts; failed bootstrap preserves
them.

The supported coupled handoff follows the same ownership rule. Machine first
plans accepted bytes, explicit `NTSC_PINNED`, x105 family, cold reset,
cartridge medium, PIF-version bit, all staged GPR values/sources, Status, and
completed-transfer control flow. Only a complete plan may replace runtime CPU
and memory state. PAL/MPAL or incomplete requests reject before mutation.

## Proof, integration, and limits

Accepted proof classes are core unit tests, focused `machine_step` tests, the
construction/reset probe, the 155-case step probe, the bounded BOOT-2
probe, and exact-source anchors. BOOT-2 proves one authentic cartridge-derived
`SpecialAdd` commit only. The integrated partial increment proves private
Machine-owned SP IMEM representation and complete aligned `Lw` for direct
RDRAM, known SP IMEM, and cartridge-bootstrap-staged SP DMEM. Explicit profile
materialization now gives generated or user-supplied firmware bytes a
production copy event; the authentic
no-firmware SP-IMEM load still rejects before mutation because byte zero is
unknown. Generated proof also establishes the bounded NTSC cold-x105 coupled
handoff and a 32,185-step generated composition through the stored RI_SELECT
read, cold BNE/NOP slot, high-SP-IMEM stack stores, exact RI_CONFIG store, and
8,000 generated CPU-loop iterations, the RI_CURRENT_LOAD event, following
`Ori`, exact RI_SELECT write, both RI_MODE stores, a four-iteration CPU wait,
and a 32-iteration CPU wait whose BNE delay slot constructs `0x10F`. The exact
MI_INIT_MODE store then creates length 15 / initialization mode true; a
following `Lui`/`Ori` pair constructs `0x18082838`; global RDRAM_DELAY then
commits the 5/7/3/1 fact and consumes the transfer. Global RDRAM_REF_ROW stores
raw zero, the following `Lui` constructs `0xFFFFFFFF80000000`, and global
RDRAM_DEVICE_ID records requested base `0x02000000` without moving bytes or
routing. Fourteen CPU-local setup commits then reach the MI_VERSION load;
`0x02020102` makes the guest comparison take RCP 2.0, the Nop delay slot
executes once, and setup selects spacing `0x400` plus first-responder base
`0xFFFFFFFFA3F08000`. Exact non-global RDRAM_DEVICE_ID physical `0x03F08004`
then records a bounded zero assignment request. The following `Addiu` produces
initial RDRAM_MODE address `0xFFFFFFFFA3F0000C`. The next generated JAL at
`0xA40001A0` rejects atomically because retained r31 IPL2 link lineage cannot
be replaced under the current source-backed gate; no link or delay slot
commits. It does not prove an authentic firmware-executed handoff, responder
presence/completion, RI calibration or elapsed hardware time, RDRAM_MODE or
RDRAM initialization, BOOT-3, full ISA, game compatibility, renderer, audio,
performance, or host integration.

Runtime integration is headless/no-window only. Rollback exists for represented
unsupported/rejection paths. Observability is public read-only state plus probe
artifacts. Performance/resource truth is `UNKNOWN` unless separately measured.

Integrated evidence identifies the hardware producer as IPL1 copying retained
IPL2 firmware content into SP IMEM. The product has one explicit user-supplied
firmware input boundary. After host byte transfer, Machine owns structural
validation, immutable accepted bytes, reset/bootstrap persistence,
classification, and rejection. Acceptance alone executes nothing and produces
no SP IMEM provenance.

Integrated mapping evidence establishes three pinned copy profiles: NTSC raw
`[0x0d4,0x71c)` to SP IMEM
`[0x000,0x648)`, and PAL/MPAL raw `[0x0d4,0x720)` to
`[0x000,0x64c)`. An explicit profile is required; shape-only acceptance does
not select one. Machine now owns the profile meanings and atomically copies the
complete selected range at bootstrap into a replacement SP IMEM image. Copied
bytes are known with source provenance and every other byte remains `Unknown`.
Copying those bytes alone does not establish coupled CPU handoff state. The
bounded product now adds one explicit NTSC cold cartridge x105 path: t3, sp,
profile-qualified ra, s3-s7, Status, PC/next-PC, and cleared delay context.
Other profiles, reset kinds, media, IPL3 families, and physical PIF revisions
remain unsupported or unknown.

Required validation: `./rust/verify-forward` and the narrow focused test for a
changed seam. Next authority requires an explicit product packet. Known unknowns
include unearned full machine scheduling, timing, broad memory/device routing,
host integration, broader handoff state, and whether any later fact requires
minimal firmware execution.
