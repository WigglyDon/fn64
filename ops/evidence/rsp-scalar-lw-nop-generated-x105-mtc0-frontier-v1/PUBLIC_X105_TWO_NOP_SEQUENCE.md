# Public X105 Two-NOP Sequence

The exact public sequence commits:

1. local `0x010`, word `0x00000000`: RSP PC/next `0x014/0x018`, count `5`;
2. local `0x014`, word `0x00000000`: RSP PC/next `0x018/0x01C`, count `6`.

Ordinary CPU instructions rotate the token:

- `0x80000010`, word `0x354AFFFF`, `Ori r10,r10,0xFFFF`: old r10
  `0x1FFF0000`, new r10 `0x1FFFFFFF`, exact instruction-result lineage;
- `0x80000014`, word `0x3C01A460`, `Lui r1,0xA460`: new r1
  `0xFFFFFFFFA4600000`, exact instruction-result lineage;
- `0x80000018`, word `0x012A4824`, `SpecialAnd r9,r9,r10`: old r9
  `0xFFFFFFFF80001000` and old r10 `0x1FFFFFFF`, new r9 `0x00001000`, exact
  two-source instruction-result lineage.

Each commits once with unavailable CPU delay context and no device effect.
CPU Count reaches `252351` and CPU committed count reaches `252367`. `r4`,
`v12`, semaphore, SP state, and RSP count remain exact during every CPU call.
