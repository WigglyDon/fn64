# SP write-DMA field decoding

The shared SP decoder applies to raw word `0xFE817000`:

- block length: `(raw & 0x0ff8) + 8 = 8`
- block count: `((raw >> 12) & 0xff) + 1 = 24`
- DRAM skip: `(raw >> 20) & 0x0fff = 0xFE8`
- RDRAM block stride: `8 + 0xFE8 = 0xFF0`
- transferred byte count: `8 * 24 = 192`

No x105-specific field decoder exists.
