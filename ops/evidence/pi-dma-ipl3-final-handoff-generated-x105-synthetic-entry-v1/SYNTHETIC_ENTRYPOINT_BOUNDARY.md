# Synthetic entrypoint boundary

Final Machine state is PC `0x80001000`, next_pc `0x80001004`, with no active
delay context. RDRAM physical `0x1000` and cartridge offset `0x1000` both hold
`0x24020042` (`Addiu r2,r0,0x0042`). Primary I-cache line index 128 remains
invalid before the first possible entry fetch.

The sentinel instruction has zero execution count. Reaching its address is a
synthetic authority boundary, not authentic cartridge execution. No private
PIF input, proprietary firmware, or commercial ROM was accessed.

Authentic checkpoint: `BOOT-2`.

Synthetic milestone: `GENERATED-IPL3-FINAL-HANDOFF-COMPLETE`.

Game compatibility claim: none.
