# Cache Initialization Next Frontier

RDRAM initialization is complete at:

- PC `0xA4000400`;
- next PC `0xA4000404`;
- Count `246984`;
- total committed steps `247000`;
- current word `0x4080E000`;
- identity `Cop0Mtc0`, source r0, destination COP0 register 28 (`C0_TAGLO`).

Pinned source begins cold cache initialization here and follows with another
tag register write and CACHE loops. The frontier instruction is inspected but
not executed. CACHE and general cache state remain unimplemented.

