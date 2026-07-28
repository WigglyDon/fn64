# Public X105 Post-Semaphore Lw

At loop exit, already represented scalar `Lw` at local PC `0x03C` reads
available DMEM bytes `25 29 00 04` from `[0,4)` and commits:

- destination: `r6`;
- value: `0x25290004`;
- provenance: exact local PC, four SpImem fetch descriptors, base `r0`
  architectural zero, signed offset zero, local DMEM address zero, and four
  Available SpDmem knowledge descriptors;
- post PC/next PC: `0x040/0x044`;
- post RSP count: 48.

`r4 = 0x03A04820` and unavailable pre-DMA `v12` remain unchanged.
