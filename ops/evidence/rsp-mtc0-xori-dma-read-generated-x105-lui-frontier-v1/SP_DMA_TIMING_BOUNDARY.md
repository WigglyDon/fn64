# SP DMA Timing Boundary

The represented operation is one instruction-boundary functional fact:
programming `SP_RD_LEN` commits one complete, preflighted, atomic DMA.

Not represented are transfer cycles, persistent `DMA_BUSY` or `DMA_FULL`
duration, queue occupancy, double buffering, partial progress, bus
arbitration, RSP stalls, CPU/RSP frequency ratios, wall-clock time, or host
scheduling. There is no countdown, service event, extra `Machine::step`, or
extra RSP committed instruction.

This is not a hardware-timing or cycle-accuracy claim.
