# PI DMA atomicity

The PI_WR_LEN step first proves register availability, exact supported length,
domain mapping, nonoverflowing source and destination bounds, cartridge-byte
availability, and RDRAM capacity. Only then does application copy the complete
payload, record the completion, and set MI PI pending.

On success, PC, next PC, Count, and the committed-step total advance once for
the store. PI busy and error state are already false when the step returns. No
intermediate progress is visible and no timing, arbitration, FIFO, or cycle
claim is made.

All failure cases preserve complete Machine state, including cartridge and
RDRAM bytes, both primary caches, PI registers, MI interrupts, SP state, CPU
state and lineage, COP0, reservations, and host-owned state. Unaligned stores
retain existing AdES and delay-slot ownership.
