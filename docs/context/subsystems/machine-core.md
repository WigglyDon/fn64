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
RI_SELECT, RI_CONFIG, and RI_CURRENT_LOAD event state, reset/power state, optional
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

Aligned `Sw` uses a separate Machine plan/application path for SP IMEM or the
four exact RI_MODE, RI_CONFIG, RI_CURRENT_LOAD, and RI_SELECT targets.
The plan resolves old base, alignment, direct target, old source value, exact
SP-IMEM span or destination-specific RI state, and CPU-store provenance before
application. Undefined RI_CONFIG high bits, unavailable RI_CONFIG for an
RI_CURRENT_LOAD event, and RI_SELECT words other than `0x14` reject during
planning. RI_MODE planning accepts its defined low-nibble fields and rejects
nonzero bits above bit 3. Application has no
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

The supported coupled handoff follows the same ownership rule. Machine first
plans accepted bytes, explicit `NTSC_PINNED`, x105 family, cold reset,
cartridge medium, PIF-version bit, all staged GPR values/sources, Status, and
completed-transfer control flow. Only a complete plan may replace runtime CPU
and memory state. PAL/MPAL or incomplete requests reject before mutation.

## Proof, integration, and limits

Accepted proof classes are core unit tests, focused `machine_step` tests, the
construction/reset probe, the 116-case step probe, the bounded BOOT-2
probe, and exact-source anchors. BOOT-2 proves one authentic cartridge-derived
`SpecialAdd` commit only. The integrated partial increment proves private
Machine-owned SP IMEM representation and complete aligned `Lw` for direct
RDRAM, known SP IMEM, and cartridge-bootstrap-staged SP DMEM. Explicit profile
materialization now gives generated or user-supplied firmware bytes a
production copy event; the authentic
no-firmware SP-IMEM load still rejects before mutation because byte zero is
unknown. Generated proof also establishes the bounded NTSC cold-x105 coupled
handoff and a 32,155-step generated composition through the stored RI_SELECT
read, cold BNE/NOP slot, high-SP-IMEM stack stores, exact RI_CONFIG store, and
8,000 generated CPU-loop iterations, the RI_CURRENT_LOAD event, following
`Ori`, exact RI_SELECT write, both RI_MODE stores, a four-iteration CPU wait,
and a 32-iteration CPU wait whose BNE delay slot constructs `0x10F`. It stops
at the MI_INIT_MODE store target miss and does not prove an
authentic firmware-executed handoff, RI
calibration or elapsed hardware time, RDRAM initialization, BOOT-3, full ISA,
game compatibility, renderer, audio, performance, or host integration.

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
