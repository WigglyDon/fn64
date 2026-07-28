# Vsub Next Frontier

The next selected RSP word is:

- local PC: `0x060`;
- raw word: `0x4A0D6B51`;
- identity: `Vsub v13,v13,v13`;
- destination: `v13`;
- both sources: `v13`;
- element: 0.

`v13`, the accumulator, VCC, VCO, and VCE remain unavailable. The exact word
is identified but explicitly rejected as
`VectorVsubUnsupported`. A complete Machine snapshot is equal before and after
the rejected attempt; processor turn remains RSP, PC/next remain
`0x060/0x064`, and RSP count remains 56.

No vector arithmetic, accumulator, or flag behavior executes.
