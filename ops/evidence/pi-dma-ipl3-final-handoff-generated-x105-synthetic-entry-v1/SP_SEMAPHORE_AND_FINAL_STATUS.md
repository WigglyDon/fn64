# SP semaphore and final status

At PC `0x800000B0`, generated `Sw r0,0x1C(r1)` writes zero to physical
SP_SEMAPHORE `0x0404001C`. The `Sp` owner records a clear semaphore and exact
CPU provenance. This is register truth only: it does not claim RSP acquisition,
observation, synchronization, or instruction execution.

At PC `0x800001A0`, generated SP_STATUS command `0x00AAAAAE` sets halt,
clears broke and SP interrupt, clears single-step and interrupt-on-break, and
clears signal bits 0 through 7. Final `halt` is true and all signals are false.

No RSP scalar/vector state, DMA, semaphore read, or RSP execution was added.
