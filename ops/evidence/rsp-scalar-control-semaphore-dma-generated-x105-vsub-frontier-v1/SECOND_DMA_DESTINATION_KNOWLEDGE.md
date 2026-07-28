# Second DMA Destination Knowledge

SpDmem remains the sole owner of DMEM backing, knowledge, and provenance.

Before the transfer, the complete range contains the accepted mixed prior
bootstrap/DMA knowledge. After the atomic transfer, every offset
`0x000..0xFFF` is:

`Available { value: matching Rdram source byte, source: SpDma { record_index: 1 } }`

This replaces prior `BootstrapUncovered`, `CartridgeBootstrap`, and first-DMA
knowledge through one ordinary owner-local write. No second byte map exists.
All 4096 post-transfer values equal their RDRAM source bytes.

The scalar registers retain their owned loaded values, and unavailable `v12`
is not retroactively changed by later DMEM mutation.
