# Synthetic test coverage

Core proofs cover construction, KSEG0/KSEG1 aliases, arbitrary high source
bits, exact request/provenance, read-before-write rejection stress, all required
nonzero values, unknown source, narrow RCP2/RCP1/neighbor routing, pending MI
conflict, no hidden prior authorization, sibling preservation, ordinary and
delay-slot cadence/AdES, replacement, reset/bootstrap/failure lifecycle, and
Machine independence.

The generated composition uses public `Machine::step` only and proves the
request commit, following Addiu, and atomic JAL frontier. The no-window step
probe exposes stable cases for all three facts.
