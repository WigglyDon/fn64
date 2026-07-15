# RI hardware-effect and timing boundary

The product represents stored RI_MODE register fields and CPU-store lineage.
The bounded source describes zero as reset mode with both stop-active flags
disabled and `0x0E` as standby with both flags enabled.

It does not represent physical reset or standby completion, TX/RX stop
propagation, RDRAM electrical readiness, current calibration, elapsed RCP or
DRAM clocks, a CPU-to-RI timing relationship, platform clock, device tick,
hidden countdown, or transition-complete flag.

The four-iteration and 32-iteration loops prove CPU instruction composition
only. They neither authorize nor create later MI or RDRAM state.
