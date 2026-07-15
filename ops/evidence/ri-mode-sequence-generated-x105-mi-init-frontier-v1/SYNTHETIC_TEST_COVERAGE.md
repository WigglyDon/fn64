# Synthetic test coverage

Generated unit and public-step proof covers:

- construction/reset/bootstrap/rebootstrap/failed-bootstrap lifecycle and
  independent Machines;
- all defined low-nibble field shapes, undefined-high-bit rejection, low-word
  transfer, high-word discard, knownness, `rs == rt`, and complete rollback;
- exact KSEG0/KSEG1 aliases, RI neighbor misses, and unaligned AdES;
- CPU-store provenance replacement and sibling RI/memory preservation;
- sequential and ordinary-delay-slot cadence;
- exact four-iteration and 32-iteration CPU loops, branch counts, and slots;
- the second-loop `Ori` slot producing `0x10F`;
- the 32,038-commit prefix through total commit 32,155 and atomic MI_INIT_MODE
  stop.

All input bytes and instruction words are independently generated. No private
firmware or cartridge ROM participates.
