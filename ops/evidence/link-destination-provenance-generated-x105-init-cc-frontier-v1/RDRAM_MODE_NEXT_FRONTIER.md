# RDRAM_MODE next frontier

RDRAM_MODE is not the immediate machine frontier in this pass.

The exact earlier source-knownness frontier is:

- PC/next_pc: `0xA4000890/0xA4000894`;
- Count/commits before attempted step: `32176/32192`;
- word: `0xAFA20000`;
- identity: `Sw r2,0(r29)`;
- base r29: `0xFFFFFFFFA4001EF0`, known generated Addiu result;
- source r2: value zero, retained `UnknownPifProduced` lineage;
- effective/CPU/physical addresses:
  `0xFFFFFFFFA4001EF0/0xA4001EF0/0x04001EF0`;
- target: SP IMEM offset `0xEF0`;
- result: `ValueSourceUnavailable` before mutation.

Therefore RDRAM_MODE is `NOT YET REACHED DUE TO EARLIER CPU FRONTIER`. The
store source-knownness rule is preserved rather than bypassed.
