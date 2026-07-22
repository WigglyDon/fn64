# Synthetic cartridge checksums

The fixture builder independently implements the pinned x105 checksum before
constructing the `Machine`. It uses public payload bytes, fixed public seed
`0x91`, multiplier `0x5D588B65`, and the exact public generated side-data bytes
at cartridge offsets `0x750..0x84F` that relocation makes available at RDRAM
physical `0x200..0x2FF`.

The initial accumulator word is `0xDF26F436`. The computed header words are:

- offset `0x10`: `0xFAD40ECC`;
- offset `0x14`: `0x1F137F19`.

Generated execution independently produces the same two words in its checksum
GPRs and takes the success path. Cartridge bytes remain immutable throughout.
No proof helper assigns guest registers or skips the guest loop.
