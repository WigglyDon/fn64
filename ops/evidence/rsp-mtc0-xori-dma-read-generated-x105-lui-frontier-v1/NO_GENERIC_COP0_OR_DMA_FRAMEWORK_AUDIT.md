# No Generic COP0 Or DMA Framework Audit

The product adds no general COP0 bank. Exact Mtc0 decode accepts only
`SP_MEM_ADDR`, `SP_DRAM_ADDR`, and `SP_RD_LEN`; other control indices remain
closed.

The product adds no generic DMA framework, bus, MMIO layer, generalized
physical map, device registry, queue, clock, or processor trait. A small
private `Machine` helper extraction lets CPU SP-register writes and RSP Mtc0
share the pre-existing `Sp`-owned read-DMA policy.

`SP_WR_LEN`, SP-to-RDRAM DMA, scalar Lui/J, BREAK, RDP, graphics, and audio
remain outside this increment.
