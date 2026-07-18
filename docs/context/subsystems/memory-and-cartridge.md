# Memory And Cartridge

Context role: memory/cartridge architecture context.
Scope: normalized cartridge bytes, RDRAM, SP DMEM, and represented access seams.
Canonical for: byte ownership, address-domain boundaries, and legal fixture policy.
Not canonical for: detailed represented access capability or future bus design.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and [boot checkpoint](../../boot_spine_checkpoint.md).
Update triggers: byte ownership, normalization, address classification, or represented storage changes.

The Machine owns accepted cartridge bytes after normalization, RDRAM, SP DMEM, SP IMEM,
and each represented state mutation. Hosts may supply bytes and paths but never
retain competing emulated truth. Source-layout normalization and storage
endianness must remain explicit and range checked.

Allowed direction is host bytes → cartridge normalization → Machine-owned
storage/classification. Forbidden directions include core filesystem access,
commercial/proprietary payloads, host pointers as machine policy, renderer
decisions, and an unearned generic bus or memory-map framework.

For an explicitly user-selected PIF firmware file, the host may own only the
path, file read/failure, and owned-byte transfer. The Machine must own accepted
bytes, validation/classification, reset/bootstrap lifecycle, SP IMEM production,
and provenance. The no-window probe now implements that input boundary with one
optional literal `--pif-rom` path and no default, search, download, bundled
fallback, reconstructed table, or firmware-derived profile. A separate
explicit `--pif-profile` spelling selects one Machine-owned pinned layout; the
host does not own layout meaning or infer a value. Machine accepts a
1,984-byte candidate structurally, rejects a 2,048-byte full-map shape as
unsupported, and rejects other lengths as malformed. Acceptance does not prove
authenticity. Firmware and profile installation remain independent and neither
alone produces SP IMEM state.

Cold-x105 coupled handoff adds four independent explicit host spellings for
family, reset kind, boot medium, and PIF-version bit. They are transferred as
typed Machine inputs and never inferred from a filename, game identity,
cartridge digest, PIF contents, host region, or expected trace. The only
supported coupled path is `NTSC_PINNED` + x105 + cold + cartridge; PAL/MPAL
continue to support their byte-copy layouts but their coupled CPU handoff
requests fail closed.

The named `Machine::stage_cartridge_bootstrap` creation point preflights the
normalized cartridge span `[0x40, 0x1000)`, stages it into the same SP DMEM
offsets, and records cartridge provenance. The bounded inspection host supplies
owned bytes; it never gives the core a file path. This narrow path is not PI
DMA, a general cartridge mapping, or a PIF/CIC implementation.

Aligned CPU `Lw` now reuses that exact bootstrap span as the sole production
knownness owner for direct SP-DMEM data reads. A complete word within
`[0x040,0x1000)` reports its exact source cartridge offset; backing below
`0x040` or without current bootstrap lineage rejects before mutation. The route
adds no SP-DMEM write, mirroring, device access, bus, or generalized map.

SP IMEM is exactly 4 KiB of private Machine-owned backing storage for physical
addresses `0x04001000..0x04001fff`. Construction and reset create zero backing
with every byte explicitly `Unknown`. Cartridge-bootstrap restaging builds a
replacement image and, when both inputs exist, copies the complete selected
range before assignment. Every copied byte receives user-supplied-PIF source
provenance; every other byte remains `Unknown`. An aligned big-endian word is
readable only when all four bytes have represented provenance. Test-only
staging remains distinct from this production creation event.

Aligned CPU `Sw` now mutates direct SP-IMEM words, exact RI_MODE fields, exact
RI_CONFIG state, an exact RI_CURRENT_LOAD event, exact bounded RI_SELECT state,
the exact bounded MI_INIT_MODE state, exact global RDRAM_DELAY/REF_ROW/DEVICE_ID
facts, or the exact RCP 2.0 first-responder DEVICE_ID request.
A known SP-IMEM source creates four known big-endian bytes even when prior
bytes were Unknown. Each selected byte receives CPU-store provenance carrying
instruction PC, source GPR, and source lineage; neighboring value/provenance is
unchanged. The exact RI_CONFIG route at physical `0x04700004` stores only
current-control input bits 5:0, enable bit 6, and the same bounded CPU-store
lineage; it changes no memory and rejects undefined high bits. RI_CURRENT_LOAD
at physical `0x04700008` requires the stored configuration, snapshots its
fields, and records transfer-word/CPU lineage as an update event without a
hardware output. RI_SELECT at physical `0x0470000C` accepts only exact x105
word `0x14`, replaces its value/source with CPU-store lineage, and preserves
both sibling facts. RI_MODE at physical `0x04700000` stores defined bits 3:0
with CPU-store lineage and rejects nonzero high bits before mutation. Reset
clears runtime SP bytes to Unknown and all RI facts to
unavailable. Complete
bootstrap restaging replaces copied SP bytes, recreates RI_SELECT zero, and
clears stale RI_CONFIG, RI_CURRENT_LOAD, RI_MODE, CPU-written select state, and
MI initialization state while preserving immutable MI_VERSION
`0x02020102`. MI_INIT_MODE at physical `0x04300000` accepts only
low word `0x0000010F`, stores length 15 and initialization mode true with
CPU-store provenance, and changes no memory or RI fact. Construction, reset,
and complete cold bootstrap leave that MI fact unavailable; repeated complete
bootstrap clears it, failed bootstrap preserves it, and independent Machines
do not share it. The exact MI write also arms one pending 16-byte transfer.
Only physical global `0x03F80008` and low word `0x18082838` consume it, storing
logical delay fields 5/7/3/1 and packed configuration `0x28381808` in the
existing sole `Rdram` owner without changing RDRAM bytes. Current MI readback
then becomes unavailable. Physical global `0x03F80014` separately accepts only
low word zero and stores a raw REF_ROW global-aperture fact with CPU provenance
while preserving the delay configuration. It interprets no refresh field and
changes no RDRAM byte. Physical global `0x03F80004` accepts only word
`0x80000000` and records requested base `0x02000000` without relocation or
routing effects. Exact RCP 2.0 first-responder physical `0x03F08004` accepts
only low word zero and records requested initial device ID zero with CPU-store
provenance. It does not require prior global RDRAM facts, prove a responder,
or complete an assignment. RCP 1.0 physical `0x03F04004`, other non-global
apertures, RDRAM_MODE, SP-DMEM, other MI/RDRAM registers, device reads,
general RI_SELECT programming, and other store targets remain unsupported.
Exact aligned `Lw` at physical `0x04300004` reads the immutable version;
the other MI read surface remains closed.

Lineage is `lawful bytes → normalized layout → named address domain → preflight → storage mutation/read → narrow observable result`. Failed writes must leave no
ghost state. Synthetic instruction words and small generated fixtures are valid
proof; user-local ROMs are outside routine inspection and evidence packaging.

Current integration includes represented cartridge facts, narrow bootstrap
staging, one cartridge-derived instruction commit, SP IMEM storage, optional
cold-entry or exact CPU-written RI_SELECT, CPU-written RI_CONFIG,
RI_CURRENT_LOAD event, CPU-written RI_MODE field facts, one exact CPU-written
immutable MI_VERSION identity, MI initialization-mode fact/pending transfer,
one global broadcast-delay fact, one raw-zero global REF_ROW fact, one global
DEVICE_ID relocation-request fact, one RCP 2.0 first-responder DEVICE_ID
assignment-request fact, and cause-known value-unavailable aligned SP-IMEM
word truth,
narrow KSEG0/KSEG1 CPU-data routes to the
represented SP memories, and aligned `Lw` over direct RDRAM, known SP IMEM,
cartridge-staged SP DMEM, exact physical RI_SELECT `0x0470000C`, or exact
MI_VERSION `0x04300004`.
Opaque SP-IMEM words retain exact store PC, source GPR/lineage, and addresses
but expose no word or byte value. Their canonical private zero backing is not
machine truth. A known aligned word overwrite replaces opaque state; aligned
Lw rejects before destination mutation, and instruction fetch has no SP-IMEM
route that could decode backing bytes. Unknown SP-DMEM and device command
stores remain closed.

Source-qualified
evidence identifies retained IPL2 firmware as
the external producer for the observed x105 prefix `[0x000, 0x020)` and initial
mutation range `[0x000, 0x02c)`. Explicit profiled copy now represents that
byte-transfer effect from lawful input, but no private PIF was used. Generated proof combines it atomically with the bounded NTSC
cold-x105 CPU
handoff and advances a generated 32,216-step composition through the stored
RI_SELECT read, cold BNE/NOP slot, five high-SP-IMEM saves, exact RI_CONFIG
store, 8,000 CPU-loop iterations, RI_CURRENT_LOAD event, following `Ori`, and
exact RI_SELECT write, both RI_MODE writes, both bounded CPU waits, the exact
MI_INIT_MODE write, delay-word construction, exact global RDRAM_DELAY commit,
raw-zero global RDRAM_REF_ROW commit, DEVICE_ID-value `Lui`, exact global
RDRAM_DEVICE_ID requested-base commit, fourteen CPU-local setup steps, the
MI_VERSION read, guest-selected RCP 2.0 branch and delay slot, spacing/base
setup, exact first-responder zero request, and the following RDRAM_MODE-address
`Addiu`. The generated JAL at `0xA40001A0` replaces retained r31 with PC+8,
its Nop slot executes once, and five InitCCValue entry instructions commit.
Four inherited-unknown r2-r5 saves then create opaque aligned SP-IMEM words;
twenty known-source saves follow without disturbing them. Execution stops
before the FindCC JAL at `0xA40008F0`; later current-control code and
RDRAM_MODE are not reached;
no general RI_SELECT programming, other RI write/read, other MI register/read,
general MI replication, other RDRAM-register behavior, per-module state,
calibration/timing process, NMI, or
generic MMIO route exists. It does not establish authentic SP IMEM contents,
firmware-executed handoff,
PIF/BIOS boot, SP DMA, controller protocol, game compatibility, or a complete
N64 memory system. Rollback/preflight exists
only where the detailed ledger says it is sealed.

Required validation: `./rust/verify-forward` plus focused cartridge/RDRAM tests.
Performance and large-ROM resource behavior are `UNKNOWN` without measurement.
Pinned mapping evidence now identifies NTSC raw `[0x0d4,0x71c)` to SP IMEM
`[0x000,0x648)` and PAL/MPAL raw `[0x0d4,0x720)` to
`[0x000,0x64c)`. Shape-only input cannot select a mapping. The represented
Machine profile and full-range generated proof now cover the copy effect; the
remaining evidence pressure is profile-qualified PAL/MPAL and broader
pre-cartridge-entry state. Neither earns an architecture-first bus
abstraction.
