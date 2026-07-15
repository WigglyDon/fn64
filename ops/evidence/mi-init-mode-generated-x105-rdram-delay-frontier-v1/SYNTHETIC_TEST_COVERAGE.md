# Synthetic test coverage

Focused Rust proof covers construction, cold bootstrap, exact KSEG0/KSEG1
stores, high-word truncation, resulting fields, source provenance,
read-before-write alias pressure, representative unsupported words, unknown
source, neighboring target, no CPU load route, sequential and delay-slot
cadence, ordinary and delay-slot AdES, RI/memory preservation, repeated and
failed bootstrap, reset, and independent Machines.

The existing `fn64_step_probe` uses public `Machine::step`, retains every prior
case, adds stable MI owner/write/rejection/lifecycle cases, commits the exact
generated MI write, and proves the RDRAM-delay miss without per-instruction
loop output.
