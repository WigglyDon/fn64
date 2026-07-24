# Validation

Focused validation completed before evidence reconciliation:

- redacted probe parser and basename tests: 2 passed;
- BEQL focused regression: 7 passed;
- generated x105 final-handoff regression: 1 passed;
- complete fn64-core suite: 594 passed.

The full workspace, clippy, probes, forward gate, context, fleet, queue,
detached exact-SHA, canonical, patch-reproduction, and artifact results are
recorded after their final commands complete. No local user-ROM result is used
as a standard CI dependency.
