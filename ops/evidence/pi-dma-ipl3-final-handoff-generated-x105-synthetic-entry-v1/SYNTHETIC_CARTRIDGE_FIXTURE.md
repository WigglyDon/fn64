# Public synthetic cartridge fixture

The generated fixture is `0x00101000` bytes and is constructed before
`Machine` creation. It is entirely public synthetic data. Its header entry
word at offset `0x08` is `0x80001000`; its payload occupies offsets
`0x1000..0x00100FFF`.

Each payload word at zero-based word index `i` is generated as:

`rotate_left(i * 0x045D9F3B, 7) XOR 0x9E3779B9`

with wrapping 32-bit multiplication. The first payload word is then replaced
by `0x24020042`, the encoding of `Addiu r2,r0,0x0042`. The fixture is never
modified after generated execution begins.

The exact generated fixture measured for this evidence is 1,052,672 bytes and
has SHA-256
`98c697835c854ff5d79e050b6234c9f572cc9efe63ef4ba598736811ec4d8876`.
That digest is proof metadata only; no filename, title, ID, region, or digest
is consulted by product behavior. The raw fixture is intentionally excluded
from the artifact.
