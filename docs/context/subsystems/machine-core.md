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
machine instance: cartridge, CPU, RDRAM, SP DMEM, SP IMEM, reset/power state,
optional structurally accepted immutable PIF firmware input, and narrow
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
and exception entry remain distinct owners. A green helper test is not public
step integration. `Machine` construction/reset preserves instance isolation;
multiboxing means multiple independent instances.

Numeric values are explicit fixed-width CPU/address/storage types where earned;
RDRAM/SP words use source-clear big-endian access. An aligned `Lw` plan
preflights known bootstrap operands, address classification, alignment, and all
four source bytes before application owns writeback, lineage, and cadence. No
serialization format is a product contract yet.

## Proof, integration, and limits

Accepted proof classes are core unit tests, focused `machine_step` tests, the
construction/reset probe, the eight-case step probe, the bounded BOOT-2 probe,
and exact-source anchors. BOOT-2 proves one authentic cartridge-derived
`SpecialAdd` commit only. The integrated partial increment proves private
Machine-owned SP IMEM representation and complete aligned `Lw` for direct
RDRAM and known SP IMEM; the authentic SP-IMEM load still rejects before
mutation because byte zero is unknown. It does not prove bootstrap handoff,
BOOT-3, timing, full ISA, game compatibility, renderer, audio, performance, or
host integration.

Runtime integration is headless/no-window only. Rollback exists for represented
unsupported/rejection paths. Observability is public read-only state plus probe
artifacts. Performance/resource truth is `UNKNOWN` unless separately measured.

Integrated evidence identifies the missing hardware producer as IPL1 copying
retained IPL2 firmware content into SP IMEM. That finding changes no Machine
state production: current SP IMEM bytes remain `Unknown`. The product now has
one explicit user-supplied firmware input boundary. After host byte transfer,
Machine owns structural validation, immutable accepted bytes, reset/bootstrap
persistence, classification, and rejection. Acceptance alone executes nothing
and produces no SP IMEM provenance.

Required validation: `./rust/verify-forward` and the narrow focused test for a
changed seam. Next authority requires an explicit product packet. Known unknowns
include unearned full machine scheduling, timing, broad memory/device routing,
host integration, the exact retained-IPL2 source mapping, and the later
implementation choice between source-backed materialization and minimal
firmware execution.
