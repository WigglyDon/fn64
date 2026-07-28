# Second Shared SP Read DMA

The sequence at local PCs `0x040..0x050` reuses existing represented
`Mtc0`/`Xori` semantics and the singular Sp-owned shared read-DMA planner and
applicator:

- destination programming: DMEM local `0x000`;
- source programming: physical RDRAM `0x00000400`;
- raw `SP_RD_LEN`: `0x00000FFF`;
- decoded block length: 4096 bytes;
- block count: 1;
- DRAM skip: 0;
- source: RDRAM `[0x00000400,0x00001400)`;
- destination: DMEM `[0x000,0x1000)`.

The complete range is preflighted and applied atomically at the committing
`Mtc0 SP_RD_LEN` boundary. There is no second DMA algorithm in RSP code.
