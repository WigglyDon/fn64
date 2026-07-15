# Synthetic test coverage

Independently encoded tests cover ownership, KSEG aliases, low-word behavior,
exact fields and provenance, missing/wrong/unknown input rejection, pending
effect guarding, one-use consumption, ordinary and delay-slot cadence, ordinary
and BD AdES, bootstrap/reset lifecycle, independent Machines, closed read and
neighbor routes, sibling RI and memory preservation, and the generated 32159
step REF_ROW frontier. All composition uses public `Machine::step`.
