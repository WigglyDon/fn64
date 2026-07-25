# Legal And No-Hack Audit

The implementation and proof use public architecture documentation, pinned
public x105 source, pinned public register definitions, and independently
generated synthetic bytes.

Source and tests contain no:

- user cartridge, private PIF, or proprietary microcode bytes;
- title, filename, ID, region, checksum, digest, or microcode signature
  policy;
- PC, phase, command, function, or instruction-pattern scheduler bypass;
- inspection-written guest register, PC, SP memory, or device truth;
- public `step_rsp`, recursive step, hidden dual commit, or fallback;
- generic processor, bus, MMIO, physical-map, or device framework.

The public x105 words are independently public facts. No user-derived input was
read, copied, hashed, committed, or packaged for this pass.
