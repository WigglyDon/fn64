# Atomicity and rollback

Planning validates exact encoding, closed destination, cold-x105 access scope,
source knownness, source value, and transfer word before application. The
applicator has no fallible work.

Generated snapshots prove complete preservation for:

- unknown source GPR;
- unsupported CP0 destination;
- nonzero reserved encoding bits;
- bootstrap state outside the bounded CP0 access scope.

The rejected state includes GPR values and provenance, HI/LO, COP0 including
software-pending knownness and timer pending, memory and provenance, PC,
next_pc, Count, delay context, cartridge/bootstrap state, and reservations.
