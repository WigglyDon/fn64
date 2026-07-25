# Synthetic Test Coverage

Public synthetic tests cover:

- nested ownership and explicit unavailable initial state;
- construction/reset, repeated and failed bootstrap, and independent Machines;
- singular SP-PC write synchronization;
- Pending/Consumed general run-start lifecycle and separation from task truth;
- one-processor-per-call tagging and successful alternation;
- no fallback and complete rejection atomicity;
- CPU Count, CPU committed count, VI, and interrupt-recognition boundaries;
- known big-endian IMEM fetch plus byte provenance;
- unknown, opaque, malformed, unaligned, single-step, and unsupported fetch or
  decode paths;
- exact MFC0 decode, old-destination independence, r0 discard, scalar
  provenance, sequential/wrapping local PC cadence, and separate RSP count;
- semaphore old-value read-and-set, repeated read, r0 destination, and
  rollback;
- SP DRAM-address cold/current masked read and no side effect;
- LQV semantic frontier identification and full-state rejection;
- complete public cold-x105 composition to the exact two-MFC0/LQV boundary.

No committed test contains user-derived RSP words.
