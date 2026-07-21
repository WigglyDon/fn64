# Validation

Final candidate-tree results:

- focused cache filter: 7 passed;
- focused SP-control filter: 1 passed;
- focused KSEG0 filter: 10 passed;
- focused relocation filter: 1 passed;
- exact generated PI-frontier proof: 1 passed;
- cargo fmt --check: passed;
- warnings-denied all-target clippy: passed;
- fn64-core: 538 passed, 0 failed;
- fn64-inspection library: 16 passed, 0 failed;
- CLI integration: 11 passed, 0 failed;
- doc tests: 0 tests, passed;
- no-window machine probe: construct/reset/no-window/result all ok;
- no-window step probe: 174 stable cases, result ok;
- complete Rust forward gate: forward gate ok;
- Context-SHA: 87e4a7ec05f0b7674f1a44e851cc11c9b33b319b9b62dc510bc1828f9bfe3e34;
- context/local-link verification: 15 checks, 0 errors, result ok;
- fleet verification: 52 passed, result ok;
- integration queue: ok;
- git diff --check: passed;
- repository rust/target: absent.

The first exact-name test command selected zero tests because the harness
requires a module-qualified name under --exact; the corrected filter selected
and passed one test. The first formatting check identified only rustfmt
layout and was followed by cargo fmt plus a clean rerun. The first post-cache
step probe identified stale KSEG0 proof-fixture assumptions. Generic synthetic
cases now execute from the existing uncached KSEG1 alias, while the
exception-vector case establishes invalid-cache truth through public MTC0 and
CACHE steps before fetching its KSEG0 vector. Product cache behavior was not
weakened. All final gates above were rerun after that reconciliation.

Exact committed-candidate, detached clean-checkout, post-fast-forward
canonical, patch-reproduction, and archive checks are recorded in the delivery
artifact after their immutable SHAs exist.
