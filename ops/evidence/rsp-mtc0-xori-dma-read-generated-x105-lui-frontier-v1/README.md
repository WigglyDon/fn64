# RSP MTC0, XORI, Read DMA, And LUI Frontier

Evidence class: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

This increment represents RSP `Mtc0` only for `SP_MEM_ADDR`, `SP_DRAM_ADDR`,
and `SP_RD_LEN`, plus exact 32-bit `Xori`. The read-length write reuses the
existing Sp-owned CPU-side read-DMA policy and commits one atomic eight-byte
RDRAM-to-DMEM transfer.

Public words `40800000 38030180 40830800 40801000` commit. RDRAM bytes
`25 29 00 04 15 1F FF E3` move from `[0x180,0x188)` to DMEM `[0,8)`. RSP count
reaches ten and `Lui r5,0x0020` (`3C050020`) rejects at local `0x028`.

No private input, scalar Lui/J, BREAK, DMA timing, generic COP0/DMA framework,
BOOT-3, or compatibility claim is included.
