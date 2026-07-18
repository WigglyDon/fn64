# Generated x105 opaque saves

The generated prologue begins at PC `0xA4000890`, Count `32176`, committed
step `32192`, with `sp = 0xFFFFFFFFA4001EF0`. Four ordinary `Sw` instructions
save r2-r5 to offsets `0xEF0`, `0xEF4`, `0xEF8`, and `0xEFC`.

Each source is `UnknownPifProduced`. The existing numeric backing is not
transferred truth. The exact committed states are:

| Store PC | Source | Effective address | CPU address | Physical address | Offset | Post PC / next PC | Count | Commits |
|---|---|---|---|---|---|---|---:|---:|
| `0xA4000890` | r2 | `0xFFFFFFFFA4001EF0` | `0xA4001EF0` | `0x04001EF0` | `0xEF0` | `0xA4000894 / 0xA4000898` | 32177 | 32193 |
| `0xA4000894` | r3 | `0xFFFFFFFFA4001EF4` | `0xA4001EF4` | `0x04001EF4` | `0xEF4` | `0xA4000898 / 0xA400089C` | 32178 | 32194 |
| `0xA4000898` | r4 | `0xFFFFFFFFA4001EF8` | `0xA4001EF8` | `0x04001EF8` | `0xEF8` | `0xA400089C / 0xA40008A0` | 32179 | 32195 |
| `0xA400089C` | r5 | `0xFFFFFFFFA4001EFC` | `0xA4001EFC` | `0x04001EFC` | `0xEFC` | `0xA40008A0 / 0xA40008A4` | 32180 | 32196 |

Each state records its own store PC, source GPR, common
`UnknownPifProduced` source classification, and exact addresses. Each private
four-byte sentinel is zero, but no byte or word value is exposed as machine
truth.
