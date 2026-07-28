# SP DMA Destination Knowledge

Before DMA, `[0,0x10)` has private zero backing but `BootstrapUncovered`
unavailable knowledge; backing zero is not truth. After record 0, `[0,8)` is
Available as `25 29 00 04 15 1F FF E3`, each sourced by `SpDma { record_index:
0 }`. `[8,0x10)` remains exactly unavailable. SpDmem stays the sole owner.
