# Memory And Cartridge

Context role: memory/cartridge architecture context.
Scope: normalized cartridge bytes, RDRAM, SP DMEM, and represented access seams.
Canonical for: byte ownership, address-domain boundaries, and legal fixture policy.
Not canonical for: detailed represented access capability or future bus design.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and [boot checkpoint](../../boot_spine_checkpoint.md).
Update triggers: byte ownership, normalization, address classification, or represented storage changes.

The Machine owns accepted cartridge bytes after normalization, RDRAM, SP DMEM,
and each represented state mutation. Hosts may supply bytes and paths but never
retain competing emulated truth. Source-layout normalization and storage
endianness must remain explicit and range checked.

Allowed direction is host bytes → cartridge normalization → Machine-owned
storage/classification. Forbidden directions include core filesystem access,
commercial/proprietary payloads, host pointers as machine policy, renderer
decisions, and an unearned generic bus or memory-map framework.

The named `Machine::stage_cartridge_bootstrap` creation point preflights the
normalized cartridge span `[0x40, 0x1000)`, stages it into the same SP DMEM
offsets, and records cartridge provenance. The bounded inspection host supplies
owned bytes; it never gives the core a file path. This narrow path is not PI
DMA, a general cartridge mapping, or a PIF/CIC implementation.

Lineage is `lawful bytes → normalized layout → named address domain → preflight → storage mutation/read → narrow observable result`. Failed writes must leave no
ghost state. Synthetic instruction words and small generated fixtures are valid
proof; user-local ROMs are outside routine inspection and evidence packaging.

Current integration includes represented cartridge facts, narrow bootstrap
staging, and one cartridge-derived instruction commit. It does not establish
authentic handoff, SP IMEM storage/routing, PIF/BIOS boot, controller protocol,
game compatibility, or a complete N64 memory system. Rollback/preflight exists
only where the detailed ledger says it is sealed.

Required validation: `./rust/verify-forward` plus focused cartridge/RDRAM tests.
Performance and large-ROM resource behavior are `UNKNOWN` without measurement.
The first source-clear pressure is the observed SP IMEM target at CPU address
`0xA4001000` plus complete aligned `Lw`; it still does not earn an
architecture-first bus abstraction.
