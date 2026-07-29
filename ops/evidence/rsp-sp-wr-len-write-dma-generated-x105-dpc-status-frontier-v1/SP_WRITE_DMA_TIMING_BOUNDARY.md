# SP write-DMA timing boundary

fn64 represents one immutable plan, one atomic 192-byte application, one typed
record, and exact register evolution.

It does not represent persistent DMA busy/full duration, a queue, partial
progress, arbitration, RSP stalls, transfer cycles, a clock, or host service
events. The transfer completes at the committing Mtc0 boundary.
