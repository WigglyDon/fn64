# Vector Byte Element Order

For an available vector:

- `bytes[0]` is architectural byte element 0;
- `bytes[n]` is architectural byte element `n`;
- `bytes[15]` is architectural byte element 15.

The represented aligned element-zero LQV maps DMEM address `start + n`
directly to vector byte element `n`. A non-symmetric synthetic sixteen-byte
pattern proves the order. This byte-element representation does not claim or
pre-implement 16-bit vector arithmetic lanes.
