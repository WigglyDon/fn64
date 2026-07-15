# Synthetic test coverage

Generated unit and public-step proof covers:

- construction/reset/bootstrap/rebootstrap/failed-bootstrap lifecycle and
  independent Machines;
- exact `0x14` acceptance, representative unsupported words, low-word transfer,
  high-word discard, knownness, and complete rollback;
- exact KSEG0/KSEG1 aliases, RI_MODE/neighbor misses, and unaligned AdES;
- CPU-store provenance and RI_CONFIG/RI_CURRENT_LOAD/memory preservation;
- sequential and ordinary-delay-slot cadence;
- read-after-write value, destination lineage, and no read side effect;
- the 32,037-commit prefix, RI_SELECT commit 32,038, and atomic RI_MODE stop.

All input bytes and instruction words are independently generated. No private
firmware or cartridge ROM participates.
