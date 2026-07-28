# SP MEM ADDR Write Semantics

The existing Sp state applies mask `0x1FF8`; bit 12 selects IMEM/DMEM and DMA
addresses are eight-byte aligned. Public r0 writes raw zero, selecting DMEM
offset zero with Mtc0 source index 0. It starts no DMA. The later transfer
advances interpreted local address to `0x008` while retaining original raw word
and programming source.
