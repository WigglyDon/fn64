# SP Write Length Frontier

At the exact frontier:

- RSP PC/next/count/turn: `0x090/0x094/1088/RSP`
- word: `0x40831800`
- identity: `Mtc0 r3,SP_WR_LEN`
- scalar source: r3 Available `0xFE817000`
- destination control index: 3
- DMA record count: 2

One selected RSP step rejects with unsupported Mtc0 control register index 3.
Complete Machine equality proves no SP register write, write-length state,
DMA record, RDRAM/DMEM/IMEM change, scalar/vector/control change, CPU/device
change, or CPU fallback. SP-to-RDRAM DMA is not implemented.
