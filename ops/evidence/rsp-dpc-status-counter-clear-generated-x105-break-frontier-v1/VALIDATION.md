# Validation

Focused evidence at creation time:

- DPC-focused core filter: 8 passed;
- exact public DPC/CPU/Break composition: 1 passed;
- inspection all-target tests: 29 passed across unit and CLI suites;
- stable `fn64_step_probe`: success, `no-window: ok`, `result: ok`;
- focused core/inspection Clippy with warnings denied: passed.

The final full-validation counts, exact candidate SHA/tree, clean-checkout
result, canonical result, Context-SHA, and public-gate markers are recorded in
the packet artifact after all gates complete.
