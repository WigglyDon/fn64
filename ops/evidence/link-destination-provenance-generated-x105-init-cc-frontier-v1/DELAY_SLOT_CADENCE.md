# Delay-slot cadence

Generated expected cadence:

- JAL at `0xA40001A0`: PC becomes `0xA40001A4`, next_pc becomes
  `0xA400087C`, Count `32170`, commits `32186`;
- Nop at `0xA40001A4`: PC becomes `0xA400087C`, next_pc becomes
  `0xA4000880`, Count `32171`, commits `32187`;
- the existing delay owner is cleared after the slot.

A control-flow instruction encountered in an active delay slot remains
unsupported before any link write and advances Count zero.
