# Instruction Pipeline

Context role: fetch/decode/identify/classify context.
Scope: the represented Rust current-PC instruction production path.
Canonical for: pipeline fact ownership and ordering.
Not canonical for: exhaustive instruction semantics.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and `machine.rs` source/tests.
Update triggers: fetch targets, decode/identity ownership, selection, or action production changes.

The source-clear path is:

`current pc/context → target/provenance classification → one instruction fetch → one raw-field decode → one identity classification → contextual and bootstrap source-knownness gates → ordinary-control-flow planning, no-effect/stopped/unsupported, aligned-Lw planning, aligned-Sw planning for SP IMEM or exact RI_MODE/RI_CONFIG/RI_CURRENT_LOAD/RI_SELECT, bounded-MTC0 planning, or one CPU-local helper selection → classified action`.

Production does not apply machine mutation. Application does not refetch,
decode, or identify. The instruction word and decoded fields are fixed-width;
memory words are interpreted through explicit big-endian storage access.

Allowed dependencies flow from cartridge/storage and CPU address types into
Machine fetch, then pure decode/identify, then classification/application.
Forbidden dependencies include host paths, dynamic registries, probe policy,
private producer calls from inspection, and a generic all-future dispatcher.

Proof consists of source anchors, classification/fetch unit tests, focused step
tests, the 116-case step probe, and the bounded BOOT-2 trace. Read-only
current-instruction inspection exposes address, fields, identity, and Machine
source provenance without mutable state. Proof does not mean every recognized
identity executes. `Lw` is represented as one Machine-owned rule over direct RDRAM,
known SP IMEM, cartridge-bootstrap-staged SP DMEM, and exactly the stored RI_SELECT word
at physical `0x0470000C`. The SP-DMEM target records exact cartridge
provenance; RI_SELECT records its cold-entry source; unclassified backing,
unknown SP IMEM, unavailable RI_SELECT, and neighboring RI addresses reject
before mutation. The authentic first frontier remains that
represented rejection at `0xA4000044`, not absent decode or load semantics.
Integrated provenance evidence identifies the missing source category as
retained IPL2 firmware content. Explicit profiled bootstrap materialization can
now satisfy the known-byte gate from generated or user-supplied bytes; external
source knowledge alone cannot bypass it. Synthetic `Lw` success proves the
represented composition, not authentic boot. A separate generated-only NTSC
cold-x105 test proves the Machine bootstrap plan can source the inherited t3
operand before `Machine::step`; that source gate still rejects every unstaged
GPR and does not imply IPL2 execution. Generated-only continuation now commits the SP-IMEM load, the
earlier-missing
SP-DMEM load, logical transforms, four prefix SP-IMEM stores, BNE/BLTZ, both
ordinary slots, the bounded MTC0 trio, the stored RI_SELECT load, the cold BNE
and NOP slot, five high-SP-IMEM stack saves, the exact RI_CONFIG store, 8,000
four-instruction CPU-loop iterations, RI_CURRENT_LOAD update event, following
`Ori`, exact RI_SELECT write, both RI_MODE stores, and both bounded CPU wait
regions. The second BNE delay slot executes the `Ori` that constructs `0x10F`
on all 32 iterations. It stops atomically when the MI_INIT_MODE `Sw` target
classification misses; this proves no RI timing, calibration, or MI process.

The MTC0 producer accepts only zero low bits, Cause/Count/Compare, the
source-backed cold-x105 access scope, and a known old source. Its immutable
plan resolves all fallible facts before destination-specific COP0 mutation and
existing cadence application. No numeric CP0 register map or generic writer is
introduced.

The `Sw` producer checks base knownness, computes address, selects AdES before
source-value consumption, rejects every target except direct SP IMEM or exact
RI_MODE/RI_CONFIG/RI_CURRENT_LOAD/RI_SELECT, and only then captures source value/lineage and
constructs a closed destination plan. RI_CONFIG planning rejects undefined
high bits; RI_CURRENT_LOAD planning requires stored RI_CONFIG and snapshots its
fields; RI_SELECT planning accepts only low word `0x14` and creates exact
CPU-store provenance; RI_MODE planning stores bits 1:0 and bits 2/3 while
rejecting nonzero bits above bit 3. Application neither reclassifies nor
discovers a new failure.

Ordinary `BEQ`, `BNE`, non-linking/non-likely `BLTZ`, `J`, `JAL`, `JR`, and
`JALR` identities now select one bounded Machine-owned action before sequential
staging. BLTZ alone reuses the existing full-GPR signed comparator; no generic
REGIMM executor exists. A control-flow identity inside a represented slot
selects explicit unsupported rollback. Application schedules or clears the
CPU-owned slot context; it does not refetch or re-identify.

Required validation: `./rust/verify-forward` and relevant focused filters.
Known unknowns include future public-step integration categories, branch-likely
and other REGIMM/control-flow families, broader COP0 instruction execution,
general RI_SELECT programming/other RI actions/MI/NMI and generic MMIO, nested control-flow
behavior, broad fetch mapping, and instruction timing.
