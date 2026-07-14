# Synthetic test coverage

Generated unit and public-step probe coverage includes:

- construction/reset/bootstrap/rebootstrap/failed-bootstrap lifecycle and
  independent Machines;
- field words `0x00`, `0x3f`, `0x40`, and `0x7f`;
- undefined-high-bit, unknown-source, target-miss, and full-state rollback;
- KSEG0/KSEG1 exact aliases and neighboring RI misses;
- old-source low-word, high-word discard, r0, and `rs == rt` behavior;
- CPU-store provenance, no memory mutation, and RI_SELECT preservation;
- sequential and ordinary-delay-slot cadence plus unaligned AdES;
- the exact 8,000-iteration loop and atomic RI_CURRENT_LOAD stop.

All instruction words and input bytes are independently generated. No private
firmware or cartridge ROM participates.
