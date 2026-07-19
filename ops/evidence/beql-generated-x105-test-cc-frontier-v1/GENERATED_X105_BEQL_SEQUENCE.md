# Generated x105 BEQL sequence

- `0xA4000998 0x299A0040`: `Slti r26,r12,64`, producing known `r26=1`.
- `0xA400099C 0x53400018`: `Beql r26,r0,0xA4000A00`; unequal and not taken.
- `0xA40009A0 0x00001025`: `Or r2,r0,r0`; architecturally nullified.
- `0xA40009A4 0x0D000284`: first executed successor, `Jal 0xA4000A10`.

The annul preserves all four outer opaque SP-IMEM words, all concrete save
words, device facts, current generated `r31` lineage, and `r2`'s
`UnknownPifProduced` classification.
