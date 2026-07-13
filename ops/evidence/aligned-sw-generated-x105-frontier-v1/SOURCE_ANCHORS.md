# Source anchors

## Product sources

- `rust/crates/fn64-core/src/cpu/instruction.rs`: existing `Sw` decode and
  identity.
- `rust/crates/fn64-core/src/cpu/address.rs`: direct aliases, write alignment,
  AdES name, and Cause code 5.
- `rust/crates/fn64-core/src/cpu/cop0.rs`: atomic data-address-error entry.
- `rust/crates/fn64-core/src/machine.rs`: public step, aligned-Lw planning,
  cadence, delay snapshots, and generated-x105 frontier.
- `rust/crates/fn64-core/src/sp_imem.rs`: byte storage, knownness, PIF-copy
  provenance, and big-endian known-word reads.

## External technical sources

- NEC VR4300 User's Manual, primary architecture reference, accessed
  2026-07-13: word-store low-word behavior, natural alignment, and store
  address error.
  <https://www.bitsavers.org/components/nec/mips/1995_NEC_VR4300_MIPS_RISC_Microprocessor_Users_Manual.pdf>
- decompals/N64-IPL, commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `src/ipl3.s`, bounded x105
  entry region, accessed 2026-07-13: identity order and operand relationships
  through the first store.
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L70-L105>

External code, instruction words, assembly blocks, disassembly, firmware, and
cartridge bytes copied into fn64: no.
