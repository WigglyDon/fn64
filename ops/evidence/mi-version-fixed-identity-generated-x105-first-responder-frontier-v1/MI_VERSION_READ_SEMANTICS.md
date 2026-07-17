# MI_VERSION read semantics

The sole supported target is physical `0x04300004` through direct aliases.
Aligned `Lw` returns `0x0000000002020102`, writes the destination unless it
is r0, records ordinary `KnownInstructionResult` lineage, and advances PC,
next_pc, and Count once.

The plan captures the old base before writeback, including when base and
destination alias. The read mutates no MI, RI, RDRAM, memory, reservation, or
host truth. MI_VERSION `Sw` and all other new MI reads remain unsupported.
