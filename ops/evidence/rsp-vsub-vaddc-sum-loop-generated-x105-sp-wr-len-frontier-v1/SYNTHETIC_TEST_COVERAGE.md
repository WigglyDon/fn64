# Synthetic Test Coverage

Focused public tests cover:

- all 256 Vsub borrow masks under unavailable self-alias cancellation;
- signed lane order, raw accumulator-low bits, positive/negative saturation;
- all 256 Vaddc carry masks and exact unsigned 17-bit sums;
- Available and Unavailable operand policies;
- independent accumulator/VCO/VCC/VCE construction and preservation;
- exact Bgez positive, zero, negative, unavailable, target, and delay behavior;
- delay context across CPU interleave and rejected slots;
- reset, bootstrap, failed bootstrap, SP-PC, halt/run-start, and Machine
  independence;
- exact public 256-iteration composition and SP_WR_LEN atomic rejection.

The complete core suite passes 634 tests. Inspection passes 16 library tests,
2 CLI integration tests, 11 binary tests, and both no-window probes.
