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

Aligned CPU `Sw` now mutates only direct SP-IMEM words. A known source creates
four known big-endian bytes even when prior bytes were Unknown. Each selected
byte receives CPU-store provenance carrying instruction PC, source GPR, and
source lineage; neighboring value/provenance is unchanged. Reset clears the
runtime bytes to Unknown, and bootstrap restaging replaces overwritten bytes
inside the copied range with their original user-PIF provenance. RDRAM,
SP-DMEM, device, and other store targets remain unsupported.

Lineage is `lawful bytes → normalized layout → named address domain → preflight → storage mutation/read → narrow observable result`. Failed writes must leave no
ghost state. Synthetic instruction words and small generated fixtures are valid
proof; user-local ROMs are outside routine inspection and evidence packaging.

Current integration includes represented cartridge facts, narrow bootstrap
staging, one cartridge-derived instruction commit, SP IMEM storage, narrow
KSEG0/KSEG1 CPU-data routes to the represented SP memories, and aligned `Lw`
over direct RDRAM, known SP IMEM, or cartridge-staged SP DMEM. Source-qualified
evidence identifies retained IPL2 firmware as
the external producer for the observed x105 prefix `[0x000, 0x020)` and initial
mutation range `[0x000, 0x02c)`. Explicit profiled copy now represents that
byte-transfer effect from lawful input, but no private PIF was used. Generated
proof combines it atomically with the bounded NTSC cold-x105 CPU handoff and
advances a generated nineteen-step composition through BLTZ, its fourth
SP-IMEM `Sw`, the bounded MTC0 trio, and RI-address construction. The next
`Lw` at RI_SELECT rejects as a direct target miss; no RI storage or MMIO route
exists. It
does not establish authentic SP IMEM contents, firmware-executed handoff,
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
