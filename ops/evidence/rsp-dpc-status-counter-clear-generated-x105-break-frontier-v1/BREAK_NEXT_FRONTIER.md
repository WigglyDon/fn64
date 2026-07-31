# Break next frontier

- local PC/next: `0x09c/0x0a0`;
- word: `0x0000000d`;
- identity: RSP Break;
- result: explicit `BreakUnsupported` rejection;
- token remains RSP;
- RSP count remains 1091.

The complete Machine equals its pre-rejection snapshot. SP halt, broke,
interrupt-on-break, and MI SP-pending truth do not change. Neither NOP at
`0x0a0` or `0x0a4` executes.
