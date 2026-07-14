# Synthetic test coverage

Core tests cover exact MTC0 outcomes, low-word transfer, Cause masks and
knownness, read-only preservation, timer preservation, Count write-before-
cadence, wrapping and equality latch, Compare clear-before-cadence and relatch,
ordinary delay slots, malformed/unsupported/unknown/access rejection,
independent Machines, reset/restaging, and the generated RI frontier.

`fn64_step_probe` exercises the public `Machine::step` entrance and prints
stable markers for all required Cause, Count, Compare, delay-slot, rejection,
and post-trio frontier cases before ending with `result: ok`.

No test uses a private PIF image, ROM, copied firmware word, or authentic
instruction stream.
