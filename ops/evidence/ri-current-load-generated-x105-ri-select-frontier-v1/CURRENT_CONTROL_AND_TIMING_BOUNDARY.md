# Current-load and timing boundary

The product represents one CPU-triggered update event under the currently
stored RI_CONFIG fields. The bounded source uses the write after its CPU wait
loop to update current control, but fn64 records no current-control output,
analog measurement, convergence, calibration completion, RDRAM electrical
readiness, or hardware progress.

The preceding 8,000-iteration loop remains CPU composition proof only. No CPU
commit is converted into RCP cycles or DRAM clocks. There is no platform clock,
device tick, hidden countdown, completion flag, RI_SELECT side effect, or
RI_MODE side effect.
