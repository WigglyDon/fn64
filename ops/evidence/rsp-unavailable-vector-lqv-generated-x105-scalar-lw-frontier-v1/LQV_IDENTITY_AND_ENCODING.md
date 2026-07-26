# LQV Identity And Encoding

The new identity is exact RSP vector-memory `Lqv` in major opcode
LWC2 (`0x32`). Decode captures scalar base, vector destination, byte element,
and signed seven-bit offset, and requires the LQV sub-operation.

Public word `0xC80C2000` decodes as:

- base `r0`;
- destination `v12`;
- element `0`;
- encoded offset `0`;
- identity `Lqv v12[0],0(r0)`.

Other vector loads and all vector stores are named only for explicit rejection.
Scalar `Lw` is identified separately as the next unrepresented frontier.
