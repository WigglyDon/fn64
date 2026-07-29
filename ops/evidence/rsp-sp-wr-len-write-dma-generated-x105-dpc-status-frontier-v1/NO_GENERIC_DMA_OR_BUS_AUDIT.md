# No generic DMA or bus audit

The change stays inside existing `Machine`, `Sp`, `SpImem`, `SpDmem`, and
`Rdram` seams. It adds no generic DMA framework, bus, MMIO layer, generalized
physical-memory map, device registry, processor trait, or public RSP-only step
entrance.

