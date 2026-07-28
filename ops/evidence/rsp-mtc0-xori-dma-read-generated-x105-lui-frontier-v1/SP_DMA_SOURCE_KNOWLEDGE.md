# SP DMA Source Knowledge

At the public boundary, singular RDRAM truth is:

| Offset | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Value | 25 | 29 | 00 | 04 | 15 | 1F | FF | E3 |

All bytes are source-known and in range. Rdram has no unavailable-byte variant;
out-of-range preflight is its exact rejection surface. No private input is used.
