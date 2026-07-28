# SP DMA Register Evolution

The public transfer begins with singular `Sp` register truth:

- SP memory selection: DMEM, local address `0x000`;
- SP DRAM address: physical `0x00000180`;
- SP read-length raw word: `0x00000000`.

The shared owner decodes one eight-byte block with zero skip. Atomic
application advances the local address to `0x008` and the physical RDRAM
address to `0x00000188` exactly once. The memory-address programming source
remains the RSP `Mtc0` at local PC `0x018`; the DRAM-address state becomes the
typed `DmaAdvance` source tied to DMA record zero and the `SP_RD_LEN` trigger.

No persistent busy/full interval, partial register advance, or second register
owner exists.
