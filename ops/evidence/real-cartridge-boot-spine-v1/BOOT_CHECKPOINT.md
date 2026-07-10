# Boot checkpoint

Highest checkpoint: **BOOT-2 — ROM-DERIVED INSTRUCTION COMMITTED**.

BOOT-0 is earned because a deterministic private runtime artifact was accepted
by content-based byte-order classification, normalized by `fn64-core`, and
passed structural header and full IPL3-span validation. Exact private metadata
is external only.

BOOT-1 is earned because `Machine::stage_cartridge_bootstrap` copies normalized
cartridge offsets `0x40..0x1000` into Machine-owned SP DMEM at the same offsets,
records cartridge provenance, creates the represented reset subset, and stages
`pc / next_pc` as `0xA4000040 / 0xA4000044`. The operation guesses no PIF/CIC
GPR, COP0, or device state.

BOOT-2 is earned because public `Machine::step` fetched the instruction at
`0xA4000040` from the staged cartridge range, identified it as `SpecialAdd`,
committed the CPU-local path, advanced `pc / next_pc` to
`0xA4000044 / 0xA4000048`, and advanced represented Count from 0 to 1. The
source trace identifies the cartridge staging lineage without recording the
instruction word or a ROM byte range.

BOOT-3 is not claimed. Machine behavior did not reach the cartridge-declared
program entry. BOOT-4 and BOOT-5 are therefore also not claimed. No graphics or
externally meaningful game-visible output was reached.

Compatibility claim: none.
