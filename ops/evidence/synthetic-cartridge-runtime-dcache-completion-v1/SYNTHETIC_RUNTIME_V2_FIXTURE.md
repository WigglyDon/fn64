# Synthetic runtime-v2 fixture

Name: `synthetic-x105-cartridge-runtime-v2`.

- size: `0x00101000` bytes (`1,052,672`);
- entrypoint: `0x80001000`;
- first word: `0x24020042`;
- header checksums: `0x4077ADEF / 0x096B847A`;
- fixture SHA-256:
  `98b078fc27cc10120e71eeaafa188af2dbdbd594723e8598caf32a82117a84dc`.

Unused payload words follow the existing deterministic public rule:

`rotate_left(index * 0x045D9F3B, 7) XOR 0x9E3779B9`.

The 92-word program replaces only its bounded payload region. The independent
x105 checksum builder then recomputes header words before Machine construction.
Cartridge bytes remain immutable after construction. The digest is evidence
metadata only and is not product selection policy.
