# Synthetic Test Coverage

Public synthetic tests cover:

- exact Mtc0 decode for control indices 0, 1, and 2;
- unsupported destinations, unavailable sources, and atomic rejection;
- exact Xori decode, 32-bit result, aliasing, r0 discard, and provenance;
- shared CPU/RSP SP read-DMA decoding, masks, preflight, application, address
  evolution, record fields, and capacity pressure;
- unavailable and out-of-range source/destination rejection without partial
  effects;
- DMEM knowledge transition and byte order with independently constructed
  non-symmetric bytes;
- preservation of scalar, vector, semaphore, CPU Count, VI, and processor
  selection truth;
- the public cold-x105 sequence through exact Lui rejection.

The no-window step probe independently drives only public `Machine::step` and
emits stable cases for every commit, source byte, destination byte, record
field, cadence transition, and rejected frontier.
