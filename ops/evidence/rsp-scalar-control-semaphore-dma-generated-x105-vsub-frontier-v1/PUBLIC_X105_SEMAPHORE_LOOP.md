# Public X105 Semaphore Loop

The bounded public run starts with `r5 = 0x00200000` and executes only through
Machine::step.

- selected attempts after Lui until local `0x03C`: 73;
- intervening CPU commits: 37;
- `Bltz` commits: 9, all not taken;
- `Mfc0 SP_SEMAPHORE` commits: 9;
- semaphore results: `1 1 1 1 1 1 1 1 0`;
- failed acquisitions: 8;
- successful acquisitions: 1;
- semaphore-loop `Bne` commits: 9;
- `Addi` delay-slot commits: 9;
- final `r5`: `0x001FFFF7`;
- final semaphore: set;
- final RSP PC/next PC: `0x03C/0x040`;
- final loop-exit RSP committed count: 47.

No iteration is skipped, batched, or short-circuited. The timeout `Bltz`
remains source-real but is never taken in this public run.
