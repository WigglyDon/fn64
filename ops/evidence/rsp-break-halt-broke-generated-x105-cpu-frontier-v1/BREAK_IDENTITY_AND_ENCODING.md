# Break identity and encoding

The represented identity is exactly:

- raw word: `0x0000000d`;
- major opcode: SPECIAL (`0x00`);
- function: BREAK (`0x0d`);
- 20-bit code field: zero.

Break has no scalar, vector, accumulator, control, or memory operand and no
architectural exception. A nonzero code field, an adjacent SPECIAL function,
and any word with a non-SPECIAL major opcode remain explicit rejection
boundaries.

