# SP write-DMA owner decision

`Sp` remains the sole owner of SP memory-address state, SP DRAM-address state,
DMA field decoding, DMA records, transfer preflight, atomic application, and
register evolution.

`SpImem` and `SpDmem` remain the singular source-byte and source-knowledge
owners. `Rdram` remains the singular destination-byte owner. No RSP-specific
DMA owner, generic DMA framework, bus, MMIO layer, or physical-memory map is
introduced.
