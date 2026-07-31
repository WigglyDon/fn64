# Break MI SP-pending semantics

`Mi` remains the singular owner of SP-pending truth.

- interrupt-on-break false: preserve the prior pending value and provenance;
- interrupt-on-break true, pending false: set the one pending bit with exact
  Break provenance;
- interrupt-on-break true, pending true: retain one bit and the existing
  idempotent assertion provenance;
- interrupt-on-break false, pending true: preserve it; Break never clears it.

The RSP-selected Break call does not synchronize or recognize a CPU interrupt
and does not directly mutate COP0 Cause or Status.

