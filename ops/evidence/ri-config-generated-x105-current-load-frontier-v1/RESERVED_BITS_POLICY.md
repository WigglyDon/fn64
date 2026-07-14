# Reserved-bits policy

The public register definition names only bits 5:0 and bit 6. Therefore fn64
accepts a transfer word only when `word & !0x0000007f == 0`. A nonzero undefined
high bit returns `RiConfigReservedBitsUnsupported` before RI or cadence
mutation.

This is an honest product boundary for undefined behavior. It is not a claim
that VR4300 or RI hardware traps, ignores, or otherwise rejects such a write.
