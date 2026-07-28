# RSP XORI Identity

Opcode `0x0E` consumes old Available `rs`, zero-extends the 16-bit immediate,
and writes a 32-bit XOR result to `rt`. Aliasing is read-before-write; r0
discards its write but commits cadence. Public `0x38030180` produces
`r3=0x00000180`. No other logical identity is added.
