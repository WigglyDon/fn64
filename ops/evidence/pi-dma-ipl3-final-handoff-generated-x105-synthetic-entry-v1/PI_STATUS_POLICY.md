# PI status policy

The status read word is derived only from PI-owned state: bit 0 is DMA busy,
bit 1 is I/O busy, and bit 2 is error. The MI-owned PI interrupt-pending bit is
not exposed by this read.

The bounded DMA starts and completes during the PI_WR_LEN store step, so busy
and error are false after commit. Consequently the generated pre-transfer
I/O-busy poll at PC `0x80000024` and post-transfer DMA-busy poll at PC
`0x8000009C` each load `0x00000000` through ordinary aligned `Lw` semantics.

The exact status store word `0x00000002` clears the MI PI pending bit while
preserving programmed addresses and the completion record. Reset word
`0x00000001` and all other status command values reject before mutation.
