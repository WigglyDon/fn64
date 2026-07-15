# RI_MODE field decision

Decision: `RI_MODE_DEFINED_FIELDS_REPRESENTABLE`.

The pinned public RCP header identifies RI_MODE as one R/W word at physical
`0x04700000` and defines all represented bits:

- operating-mode bits: `word & 0x3`;
- stop-transmit-active: `(word & 0x4) != 0`;
- stop-receive-active: `(word & 0x8) != 0`.

The defined mask is `0x0000000F`. Field locations and R/W ownership are direct
source facts; combining them into the mask and rejecting bits above bit 3 are
bounded fn64 inferences. `word & !0xF != 0` returns
`RiModeReservedBitsUnsupported` before mutation. This is not a hardware-trap
claim.

The pinned x105 source separately calls operating mode zero reset mode with
both stop-active flags disabled and operating mode two standby with both flags
enabled. No semantic name is assigned to operating-mode values one or three,
and no electrical behavior follows from stored fields.
