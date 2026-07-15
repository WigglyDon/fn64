# RDRAM_DELAY next frontier

Independent public-macro calculation:

- `RDRAM_DELAY(5,7,3,1) = 0x28381808`;
- `ROT16(0x28381808) = 0x18082838`.

Public definitions give RDRAM base `0x03F00000`, global configuration offset
`0x00080000`, and RDRAM_DELAY offset `0x08`. The next effective CPU address is
therefore `0xA3F80008`, physical address `0x03F80008`.

At PC `0xA4000124`, `Sw r9,8(r10)` uses r10
`0xFFFFFFFFA3F80000` and r9 `0x0000000018082838`. It returns
`MachineStoreWordRejectionReason::DirectTargetMiss` and preserves PC,
next_pc, Count, GPR lineage, MI, RI, COP0, memory, delay context, and host
state. RDRAM_DELAY is the next frontier and is not implemented.
