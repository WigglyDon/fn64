# RI_SELECT write decision

Decision: `RI_SELECT_EXACT_X105_VALUE_ONLY`.

Direct public evidence identifies RI_SELECT as one R/W word at physical
`0x0470000C`. Its field comment assigns the same `[2:0]` range to both receive
and transmit selection, so it does not establish two independently representable
fields. The pinned x105 source directly constructs `0x10 | 4`, writes the
resulting `0x00000014` to RI_SELECT, and describes the operation as enabling
TX/RX select.

The accepted transfer word is exactly `0x00000014`. The Machine stores that
word with the instruction PC, source GPR, and prior Machine-owned GPR lineage.
Every other low transfer word rejects before mutation as fn64's unsupported
boundary. This is not a claim that the hardware traps or rejects other values.
The high 32 bits of the 64-bit GPR are outside the existing `Sw` transfer word
and do not affect acceptance.

The exact address, R/W relation, generated value, source description, and next
RI_MODE store are direct evidence. Treating the exact word as a bounded stored
state is the product decision. General receive/transmit fields, reserved-bit
behavior, physical-revision variation, electrical behavior, and arbitrary
RI_SELECT programming remain unavailable.
