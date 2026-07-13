# Selected instruction semantics

The selected bounded capability is the existing aligned `Lw` family routed to
the existing SP-DMEM owner. It is not a new instruction identity.

- Decode and identity remain owned by the existing CPU instruction layer.
- `Machine::step` remains the sole public execution entrance.
- Both base-register value and source classification are read before mutation.
- The 16-bit immediate is sign-extended and added with the represented wrapping
  address rule.
- Word alignment is checked before data-target classification or data access.
- A successful word is read in N64 big-endian order and sign-extended into the
  64-bit GPR representation.
- Destination `r0` remains discarded through the existing GPR applicator.
- Success commits GPR lineage, `pc`/`next_pc`, and Count exactly once.

No store identity, SP-DMEM write, device route, bus, or generalized memory map
is added.
