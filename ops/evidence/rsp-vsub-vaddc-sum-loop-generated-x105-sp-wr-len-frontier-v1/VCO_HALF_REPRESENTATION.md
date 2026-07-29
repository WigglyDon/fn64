# VCO Half Representation

VCO is one `Sp::rsp`-owned state with two independently known halves:

- `carry_or_borrow`: low eight bits;
- `not_equal`: upper eight bits.

Each half is Available with an eight-bit value and immutable source or
Unavailable with an exact cause. Both begin Unavailable from
`ConstructionOrReset`.

`Vsub` consumes only the low half and clears both halves to Available zero.
`Vaddc` consumes neither old half, replaces the low half with carry truth, and
sets the upper half to Available zero.
