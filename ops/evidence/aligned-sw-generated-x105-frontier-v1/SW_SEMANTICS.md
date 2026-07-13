# Sw semantics

The selected family reads the old `rs` base and, only for an aligned supported
SP-IMEM target, the old `rt` source. The effective address is the old 64-bit
base plus the sign-extended 16-bit immediate using wrapping arithmetic. The
stored value is exactly the low 32 bits of old `rt`, split in N64 big-endian
byte order.

`rs == rt` uses the same pre-mutation value for both roles. Architectural zero
is a known zero base and source. Success writes no GPR, commits staged
`pc/next_pc` once, and advances Count once.
