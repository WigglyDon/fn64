# SP DMA Timing Boundary

Represented:

- address and length register writes;
- complete range preflight;
- one atomic 4096-byte copy;
- one immutable typed DMA record;
- exact SpDmem knowledge/provenance;
- existing register evolution;
- owner-derived idle `SP_DMA_BUSY` at later instruction boundaries.

Not represented:

- transfer cycles;
- persistent busy/full duration;
- queue occupancy;
- partial progress;
- arbitration timing;
- RSP stalls;
- frequency ratios;
- wall clock or host scheduling.

No queue, countdown, clock, hidden service event, extra Machine::step, or extra
RSP commit was introduced.
