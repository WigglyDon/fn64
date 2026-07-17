# Generated x105 MI_VERSION sequence

| PC | Word | Instruction | Result |
|---|---|---|---|
| `0xA400016C` | `0x8C300004` | `Lw r16,4(r1)` | r16 = `0x0000000002020102` |
| `0xA4000170` | `0x3C110101` | `Lui r17,0x0101` | r17 = `0x0000000001010000` |
| `0xA4000174` | `0x36310101` | `Ori r17,r17,0x0101` | r17 = `0x0000000001010101` |
| `0xA4000178` | `0x16110005` | `Bne r16,r17,0xA4000190` | taken |
| `0xA400017C` | `0x00000000` | `Nop` | delay slot once |

Before the Lw: PC/next_pc `0xA400016C/0xA4000170`, Count `32160`,
committed steps `32176`. After it: `0xA4000170/0xA4000174`, Count
`32161`, committed steps `32177`. Old r16 is zero with
`UnknownPifProduced` lineage.
