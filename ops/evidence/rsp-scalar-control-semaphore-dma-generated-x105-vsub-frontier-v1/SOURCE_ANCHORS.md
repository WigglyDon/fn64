# Source Anchors

- [Nintendo Ultra64 RSP Programmer's Guide](https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf):
  scalar `Lui`/`Addi`, `Bltz`/`Bne`, branch-delay behavior, RSP COP0
  `SP_DMA_BUSY`, and SP DMA length/alignment semantics.
- [Pinned public x105 IPL3](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s):
  bounded local sequence `0x028..0x060` and its exact words.
- [Pinned RCP definitions](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h):
  SP control indices, including `SP_DMA_BUSY`.

Only the fourteen necessary RSP words, bounded loop facts, and bounded
begin/end anchors for the 4096-byte transfer are recorded. No complete public
RSP program or external document is copied.
