# Start state

Accepted v1 boundary, retained as an exact regression:

- PC / next PC: `0x80001000 / 0x80001004`;
- Count: `7,477,812`;
- committed steps: `7,477,828`;
- word: `0x24020042`;
- identity: `Addiu r2,r0,0x0042`;
- executions of that word: zero.

Runtime-v2 authoritative proof starts from the established public cold x105
bootstrap, not from this staged boundary. It repeats RDRAM bring-up, cache
initialization, relocation, PI DMA, x105 checksum, control teardown, and final
JR before executing the cartridge program.
