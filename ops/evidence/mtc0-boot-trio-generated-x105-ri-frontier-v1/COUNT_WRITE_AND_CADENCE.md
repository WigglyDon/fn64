# Count write and cadence

MTC0 Count first installs the source low word. The existing committed-step
cadence then increments Count once and performs the existing post-increment
Count/Compare equality test.

Consequences proved with generated words:

- writing zero finishes at one;
- writing `0xFFFFFFFF` wraps to zero;
- high GPR bits do not enter Count;
- post-increment equality latches timer pending;
- rejection advances Count zero.

This remains fn64's instruction-boundary cadence model. It is not a claim of
half-PClock or cycle-accurate timing.
