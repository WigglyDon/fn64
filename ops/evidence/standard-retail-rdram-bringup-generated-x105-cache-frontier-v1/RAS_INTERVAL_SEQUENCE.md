# RAS Interval Sequence

Generated PC `0xA4000258` executes twice. It writes raw `0x101C0A04` through
source GPR r8 to physical addresses `0x03F00018` and `0x03F00818`.

Each module retains that raw word and exact CPU-store provenance. No refresh,
timing, delay, or electrical consequence is inferred from it.
