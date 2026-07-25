# Validation

Candidate validation is recorded in the final artifact and commit evidence.
Required closure includes:

- focused RSP foundation, run-start, arbiter, fetch, MFC0, semaphore,
  SP-DRAM-address, LQV, lifecycle, and cadence tests;
- complete core and inspection tests;
- formatting and warnings-denied clippy for all targets;
- CLI integration tests;
- 190-case no-window step probe and machine probe;
- complete `rust/verify-forward`;
- context/local-link, fleet, and integration-queue gates;
- exact-candidate clean-checkout validation;
- public cold-x105 two-MFC0/LQV gate on candidate, clean checkout, and
  canonical after fast-forward;
- donor path/diffstat/full-index patch digest preservation.

Final counts, SHAs, Context-SHA, and literal success markers are not asserted
here before those gates complete.
