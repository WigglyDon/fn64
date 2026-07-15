# Reserved-bit policy

Decision: `RI_MODE_DEFINED_FIELDS_REPRESENTABLE`.

The public definition names only bits 3:0, so fn64 accepts a transfer word when
`word & !0x0000000F == 0`. All sixteen low-nibble combinations are stored as
fields. Operating-mode values one and three are numeric facts only; fn64 gives
them no undocumented names.

A nonzero bit 4 or higher returns `RiModeReservedBitsUnsupported` before
mutation. High 32 bits of the 64-bit source GPR are outside the `Sw` transfer
word and do not affect acceptance. This policy describes fn64's earned model,
not a hardware trap or ignore rule.
