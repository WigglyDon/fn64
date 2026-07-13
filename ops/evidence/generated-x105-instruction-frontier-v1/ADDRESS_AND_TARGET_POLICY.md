# Address and target policy

`Lw` retains the existing effective-address policy: a known old base GPR plus a
sign-extended 16-bit displacement, with represented wrapping arithmetic. The
32-bit CPU address must be naturally word-aligned and must translate through a
direct KSEG0 or KSEG1 alias.

The added target is only physical SP DMEM `[0x04000000,0x04001000)`, with a
complete four-byte word ending no later than local offset `0x0FFC`. The CPU
aliases therefore include `[0x84000000,0x84001000)` and
`[0xA4000000,0xA4001000)`. There is no mirroring.

Target classification is narrow:

- direct RDRAM and SP IMEM retain their existing behavior;
- SP-DMEM offsets wholly within the cartridge-bootstrap copy span
  `[0x040,0x1000)` are readable with cartridge provenance;
- concrete SP-DMEM backing outside that span is explicitly unclassified and
  rejects before data access or architectural mutation;
- non-direct addresses, target misses, and incomplete words reject through the
  existing load-word boundary.

No bus, generalized memory map, device registry, or SP-DMEM write route is
introduced.
