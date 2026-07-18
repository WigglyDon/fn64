# Generated x105 concrete saves

Twenty ordinary known-source `Sw` instructions follow. The generated public
`Machine::step` proof checks each exact full source value and lineage before
the existing concrete big-endian store commits its low word.

| PC | Source | Full value | Lineage | Offset |
|---|---:|---:|---|---:|
| `0xA40008A0` | r6 | `0xFFFFFFFFA3F00000` | `Lui @ 0xA4000150` | `0xF00` |
| `0xA40008A4` | r7 | `0xFFFFFFFFA0000000` | `Lui @ 0xA4000154` | `0xF04` |
| `0xA40008A8` | r8 | `0xFFFFFFFFA4700000` | `Lui @ 0xA40000B0` | `0xF08` |
| `0xA40008AC` | r9 | `0xFFFFFFFF80000000` | `Lui @ 0xA400012C` | `0xF0C` |
| `0xA40008B0` | r10 | `0xFFFFFFFFA3F80000` | `Lui @ 0xA40000B4` | `0xF10` |
| `0xA40008B4` | r11 | `0xFFFFFFFFA3F00000` | `Lui @ 0xA40000B8` | `0xF14` |
| `0xA40008B8` | r12 | `0xFFFFFFFFA4300000` | `Lui @ 0xA40000BC` | `0xF18` |
| `0xA40008BC` | r13 | `0x0000000000000000` | `Addu r0,r0 @ 0xA4000134` | `0xF1C` |
| `0xA40008C0` | r14 | `0x0000000000000000` | `Addu r0,r0 @ 0xA4000138` | `0xF20` |
| `0xA40008C4` | r15 | `0xFFFFFFFFA3F00000` | `Lui @ 0xA400013C` | `0xF24` |
| `0xA40008C8` | r24 | `0x0000000000000000` | `Addu r0,r0 @ 0xA4000140` | `0xF28` |
| `0xA40008CC` | r25 | `0xFFFFFFFFA3F00000` | `Lui @ 0xA4000144` | `0xF2C` |
| `0xA40008D0` | r18 | `0x0000000000000000` | `Addu r0,r0 @ 0xA4000158` | `0xF38` |
| `0xA40008D4` | r19 | `0x0000000000000000` | `CartridgeBootMedium` | `0xF3C` |
| `0xA40008D8` | r20 | `0xFFFFFFFFA0000000` | `Lui @ 0xA400015C` | `0xF40` |
| `0xA40008DC` | r21 | `0xFFFFFFFFA3F0000C` | `Addiu r15 @ 0xA400019C` | `0xF44` |
| `0xA40008E0` | r22 | `0xFFFFFFFFA0000000` | `Lui @ 0xA4000148` | `0xF48` |
| `0xA40008E4` | r23 | `0x0000000000000000` | `Addu r0,r0 @ 0xA400014C` | `0xF4C` |
| `0xA40008E8` | r30 | `0xFFFFFFFFA4001F90` | `Addu r29,r0 @ 0xA4000164` | `0xF50` |
| `0xA40008EC` | r31 | `0xFFFFFFFFA40001A8` | `Jal @ 0xA40001A0` | `0xF54` |

The four opaque words at `0xEF0..0xEFC` remain unchanged. The final state is
PC `0xA40008F0`, next PC `0xA40008F4`, Count `32200`, committed step `32216`.
