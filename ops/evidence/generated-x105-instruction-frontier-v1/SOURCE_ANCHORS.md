# Source anchors

## Pinned x105 identity order

- Source: decompals/N64-IPL.
- Revision: `928f59089c18a95cbffa59938a18fa6032c5d78c`.
- Anchor: `src/ipl3.s`, lines 70-105.
- Use: instruction identities, operand relationships, and the bounded order
  through the first retained-word mutation.
- Scope: pinned x105 reconstruction only.
- Copied code or bytes: no.

The source comment associated with the t3-relative load is arithmetically
stale. The represented handoff supplies t3 `0xA4000040`; adding immediate
`0x44` selects `0xA4000084`, not `0xA4000088`. This evidence follows the
instruction operands and address arithmetic, not the stale comment.

## Architectural load and exception rules

- Source: NEC MIPS VR4300 Microprocessor User's Manual, 1995.
- Anchor: MIPS III load/store instruction details and Cause exception-code
  table.
- Use: sign-extended 16-bit displacement, natural word alignment, AdEL, and
  branch-delay EPC/BD lineage.
- Copied code or bytes: no.

## Current fn64 owners

- `rust/crates/fn64-core/src/machine.rs`: `Machine::step`, aligned `Lw`
  planning/application, direct target selection, cadence, and rollback.
- `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`: cartridge-to-SP-
  DMEM copy span and source-known GPR lineage.
- `rust/crates/fn64-core/src/sp_dmem.rs`: 4 KiB SP-DMEM storage and big-endian
  reads.
- `rust/crates/fn64-core/src/cpu/cop0.rs`: AdEL exception entry, BadVAddr,
  EPC/BD, vectoring, and delay-context clearing.
