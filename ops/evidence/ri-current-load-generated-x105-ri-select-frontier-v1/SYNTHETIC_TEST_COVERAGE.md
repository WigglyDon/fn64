# Synthetic test coverage

Generated unit and public-step proof covers:

- construction/reset/bootstrap/rebootstrap/failed-bootstrap lifecycle and
  independent Machines;
- required stored RI_CONFIG dependency and complete rejection rollback;
- any known source word, low-word capture, high-word discard, r0, and
  `rs == rt` old-value behavior;
- exact KSEG0/KSEG1 aliases and neighboring RI target misses;
- CPU-store provenance, no memory mutation, and RI_SELECT/RI_CONFIG
  preservation;
- sequential and ordinary-delay-slot cadence plus unaligned AdES;
- the 32,035-commit prefix, event commit 32,036, `Ori` commit 32,037, and the
  atomic RI_SELECT write stop.

All input bytes and instruction words are independently generated. No private
firmware or cartridge ROM participates.
