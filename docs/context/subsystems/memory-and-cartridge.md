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
fallback, reconstructed table, or firmware-derived profile. Machine accepts a
1,984-byte candidate structurally, rejects a 2,048-byte full-map shape as
unsupported, and rejects other lengths as malformed. Acceptance does not prove
authenticity and currently produces no SP IMEM state.

The named `Machine::stage_cartridge_bootstrap` creation point preflights the
normalized cartridge span `[0x40, 0x1000)`, stages it into the same SP DMEM
offsets, and records cartridge provenance. The bounded inspection host supplies
owned bytes; it never gives the core a file path. This narrow path is not PI
DMA, a general cartridge mapping, or a PIF/CIC implementation.

SP IMEM is exactly 4 KiB of private Machine-owned backing storage for physical
addresses `0x04001000..0x04001fff`. Construction, reset, and cartridge-bootstrap
restaging create zero backing with every byte explicitly `Unknown`. An aligned
big-endian word is readable only when all four bytes have represented
provenance. The current product has no production creation event for authentic
SP IMEM offset zero; test-only staging cannot be reached by production or the
boot probe.

Lineage is `lawful bytes → normalized layout → named address domain → preflight → storage mutation/read → narrow observable result`. Failed writes must leave no
ghost state. Synthetic instruction words and small generated fixtures are valid
proof; user-local ROMs are outside routine inspection and evidence packaging.

Current integration includes represented cartridge facts, narrow bootstrap
staging, one cartridge-derived instruction commit, SP IMEM storage, the narrow
KSEG0/KSEG1 CPU-data route to that range, and aligned `Lw` over direct RDRAM or
known SP IMEM. Source-qualified evidence identifies retained IPL2 firmware as
the external producer for the observed x105 prefix `[0x000, 0x020)` and initial
mutation range `[0x000, 0x02c)`, but does not establish product bytes. It does
not establish authentic SP IMEM contents, handoff,
PIF/BIOS boot, SP DMA, controller protocol, game compatibility, or a complete
N64 memory system. Rollback/preflight exists
only where the detailed ledger says it is sealed.

Required validation: `./rust/verify-forward` plus focused cartridge/RDRAM tests.
Performance and large-ROM resource behavior are `UNKNOWN` without measurement.
Pinned mapping evidence now identifies NTSC raw `[0x0d4,0x71c)` to SP IMEM
`[0x000,0x648)` and PAL/MPAL raw `[0x0d4,0x720)` to
`[0x000,0x64c)`. Shape-only input cannot select a mapping, so the next product
pressure is explicit Machine-owned profile selection and full-range synthetic
copy proof. The separate evidence pressure is complete pre-IPL3 handoff state;
neither earns an architecture-first bus abstraction.
