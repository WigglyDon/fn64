# Synthetic test coverage

Focused product tests cover:

- four-counter ownership, independence, width, and unavailable lifecycle;
- reset, repeated bootstrap, failed-bootstrap rollback, SP PC, halt/run-start,
  and independent Machines;
- exact word/index decode and source-knownness;
- exact `0x240` command and the full bounded rejection matrix;
- nonzero selected-counter clear and unselected-counter preservation;
- exact per-counter provenance and repeated-clear replacement;
- one-instruction RSP cadence and full preservation;
- generated cold-x105 pre/post state;
- one real post-command CPU interleave;
- exact Break identity and full atomic rejection.

`fn64_step_probe` adds a stable no-window public-API case through the completed
write DMA, DPC command, one CPU call, and Break frontier.
