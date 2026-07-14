# Current-control and timing boundary

The product represents only the stored input and enable fields produced by an
exact CPU word store. The bounded source describes `0x40` as selecting
automatic current control, but fn64 does not represent the resulting analog
process, convergence, output, RDRAM electrical state, or hardware time.

The generated 8,000-iteration wait loop is CPU instruction-composition proof.
Its Count and branch cadence do not establish an RCP-cycle duration or
calibration completion. RI_CURRENT_LOAD remains a separate unavailable write;
no loop completion fact makes it succeed.
