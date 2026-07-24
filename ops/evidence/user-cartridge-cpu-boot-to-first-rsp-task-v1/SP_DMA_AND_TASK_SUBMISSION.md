# SP DMA and task submission

`Sp` remains the sole control/DMA-register owner. `Rdram`, `SpDmem`, and
`SpImem` remain the sole byte owners.

The task was prepared by two atomic RDRAM-to-SP DMA records:

1. 64 bytes from RDRAM `[0x0012BAC0,0x0012BB00)` to DMEM
   `[0x0FC0,0x1000)`. The 12-bit local end wraps to `0x0000`.
2. 1,000 bytes from RDRAM `[0x000060B0,0x00006498)` to IMEM
   `[0x0000,0x03E8)`, represented as local addresses
   `[0x1000,0x13E8)`.

Both records contain one block and zero DRAM skip. Transfers were fully
preflighted before either byte owner mutated. Per-byte SP provenance refers to
the typed DMA record rather than duplicating source truth.

SP PC was `0x000`. The start command was `0x00000125`; halt transitioned from
true to false. MI SP pending remained false. The start establishes an
inspectable request, not completion.

No SP DMA timing, queue, RSP scalar/vector register, RSP fetch, task completion,
or DP result was created.
