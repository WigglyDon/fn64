# No Cycle Accuracy Claim

The private Machine token alternates successful CPU and RSP instruction
commits 1:1 while halt is false. This is a deterministic host-independent
product cadence chosen to keep instruction ownership and mutation visible.

It is not an R4300/RSP clock ratio, device-cycle model, hardware scheduler,
wall-clock relationship, or timing-accuracy claim. CPU Count and VI remain on
their accepted CPU-only cadence. RSP owns only its separate committed
instruction count.
