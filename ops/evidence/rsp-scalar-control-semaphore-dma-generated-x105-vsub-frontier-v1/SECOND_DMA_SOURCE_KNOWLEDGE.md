# Second DMA Source Knowledge

Rdram remains the singular owner of the 8 MiB generated public Machine's
RDRAM bytes. The shared DMA preflight accepts the complete in-range source
`[0x400,0x1400)` as current Machine value truth.

Bounded identity:

- length: 4096 bytes;
- FNV-1a-64: `12B28969CC323A95`;
- first sixteen bytes:
  `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00`;
- last sixteen bytes:
  `8A 68 73 9F DD 19 DE 11 EF C9 3C 93 3E FA 9B 15`.

No private cartridge, proprietary PIF blob, user task, or complete range dump
was read or recorded.
