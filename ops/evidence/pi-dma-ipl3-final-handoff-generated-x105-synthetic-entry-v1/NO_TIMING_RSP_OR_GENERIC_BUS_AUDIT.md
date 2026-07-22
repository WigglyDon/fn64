# No timing, RSP, or generic bus audit

The implementation contains no PI cycle timing, FIFO, asynchronous progress,
device clock, arbitration, or partial DMA state. Completion is one documented
atomic functional effect.

SP start, semaphore clear, and final halt are register/command truth only.
There is no RSP instruction execution, scalar or vector register file, SP DMA,
or semaphore handshake model.

The new target matches are concrete named addresses. There is no generic bus,
generic MMIO layer, generalized device registry, generalized physical-memory
map, cache hierarchy, PI domain-timing surface, SI DMA, AI DMA, VI behavior,
DP execution, or CPU interrupt delivery.

KSEG0 D-cache functionality is limited to aligned `Lw` from Machine-owned
RDRAM because that is the bounded generated pressure. Dirty stores and
writeback remain unearned.
