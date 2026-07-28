# SP DRAM ADDR Write Semantics

The existing singular Sp DRAM-address state applies mask `0x00FFFFF8`.
Public r3 `0x180` programs physical `0x180` with Mtc0 source index 1. No DMA
starts and no RDRAM or SP-memory byte changes at this instruction.
