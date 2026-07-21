# SP control decision

One per-Machine Sp value owns the exact SP control truth exercised by x105:
halt, broke, SP interrupt pending, single-step, interrupt-on-break, SP PC low
field, and CPU-store provenance.

Only exact command words 0x000000CE and 0x000000AD and exact SP_PC word zero
are accepted. The start command records control-register truth only. It does
not execute an RSP core, vector state, scalar state, DMA, or a run loop.
