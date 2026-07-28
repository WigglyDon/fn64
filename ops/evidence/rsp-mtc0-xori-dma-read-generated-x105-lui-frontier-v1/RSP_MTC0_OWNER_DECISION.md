# RSP MTC0 Owner Decision

`Sp::rsp` owns scalar availability and instruction lineage. Existing `Sp`
fields remain the only SP register/DMA-record owners. `SpDmem` remains the only
destination knowledge owner and `Rdram` the only source-byte owner. No
RSP-specific register bank, memory map, or second byte owner exists.
