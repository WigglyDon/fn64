# RDRAM Mode Derived Field Policy

Read-only methods derive only source-named facts from the raw word:

- device enable: true (`0x02000000`);
- auto skip: true (`0x04000000`);
- current-control multiplier: true (`0x40000000`);
- current-control enable: false (`0x80000000` absent);
- encoded current-control code: `0x3F`, reconstructed from raw bits
  6, 14, 22, 7, 15, and 23.

No other bit receives product interpretation.
