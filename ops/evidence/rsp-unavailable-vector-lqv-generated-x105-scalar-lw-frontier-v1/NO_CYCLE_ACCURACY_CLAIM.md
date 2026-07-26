# No Cycle-Accuracy Claim

The CPU/RSP token and LQV result are instruction-level functional truth.
Successful represented CPU and RSP commits alternate deterministically while
RSP remains running. This is host-independent and explicitly not a hardware
frequency or cycle model.

No RSP vector-load stall count, pending-load queue, device duration, wall
clock, host timer, dual issue, or cycle ratio exists. LQV changes only one RSP
committed-instruction boundary.
