# PI DMA request

The generated request is fully concrete:

- PI_DRAM_ADDR low 24 bits: `0x00001000`;
- PI_CART_ADDR: `0x10001000`, domain 1 address 2;
- PI_WR_LEN raw word: `0x000FFFFF`;
- interpreted size: raw plus one, `0x00100000` bytes;
- direction: cartridge to RDRAM.

The triggering instruction is `Sw r10,0x0C(r1)` at PC `0x80000054`. Its
programmed-register values and source lineages were produced by ordinary
generated CPU instructions. The immutable plan validates both ranges and all
arithmetic before any byte or interrupt mutation.

No generic DMA-direction framework, PI queue, visible partial transfer, or
controller clock was added.
