# V1 regression

The accepted v1 fixture remains separate and byte-for-byte governed by its
existing builder.

Its authoritative proof still reaches:

- PC / next PC: `0x80001000 / 0x80001004`;
- Count: `7,477,812`;
- committed steps: `7,477,828`;
- entry word: `0x24020042`;
- entry executions: zero.

Runtime-v2 overlays a separate 92-word program and recomputes separate checksum
words. It does not replace or silently mutate v1.
