# CP0 TagLo and TagHi

C0_TAGLO is register 28 and C0_TAGHI is register 29. Each is one optional raw
32-bit CPU-owned fact with the instruction PC, source GPR, and source lineage
that produced it.

| PC | Word | Destination | Raw value | Source |
| --- | --- | --- | --- | --- |
| 0xA4000400 | 0x4080E000 | C0_TAGLO | 0x00000000 | r0 / ArchitecturalZero |
| 0xA4000404 | 0x4080E800 | C0_TAGHI | 0x00000000 | r0 / ArchitecturalZero |

Each MTC0 consumes the old source low word, commits once, and advances Count
once. No MFC0 path or unrelated cache-test register is added.
