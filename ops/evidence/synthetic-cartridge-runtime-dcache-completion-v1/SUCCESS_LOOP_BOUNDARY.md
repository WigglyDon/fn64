# Success-loop boundary

- success-loop PC: `0x80001124`;
- loop word: `0x08000449` (`J 0x80001124`);
- delay slot: `0x80001128`, word `0x00000000`;
- completed iterations: `2`;
- final PC / next PC: `0x80001124 / 0x80001128`;
- active delay context: unavailable;
- final Count: `7,477,100`;
- final committed steps: `7,477,116`.

Program Count and commit deltas are both `77`. Mailbox, test words, cartridge,
cache contents, devices, and provenance remain unchanged across the two loop
iterations; only ordinary PC, next-PC, and Count cadence changes.
