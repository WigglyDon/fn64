# Exact Scalar LW Identity

Only major opcode `0x23` is decoded as scalar `Lw { base_gpr, destination_gpr,
signed_offset }`. Public word `0x8C040040` is exactly
`Lw r4,0x40(r0)`.

`Lb`, `Lbu`, `Lh`, `Lhu`, other scalar loads, and scalar stores remain
explicitly unsupported. No generic scalar-memory framework was introduced.
