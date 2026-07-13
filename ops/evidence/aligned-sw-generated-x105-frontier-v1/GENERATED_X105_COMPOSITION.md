# Generated x105 composition

The composition proof uses a generated 1,984-byte PIF-shaped pattern, a
generated cartridge buffer, explicit NTSC_PINNED/X105/cold/cartridge/version
selectors, and instruction words constructed from semantic fields. It begins
at `0xA4000040`, commits the accepted add/load/load/xor prefix, commits the new
SP-IMEM `Sw`, and then follows only already represented identities until the
next explicit frontier.

The generated first copied word is `0x81ABC000`; generated SP-DMEM data words
are `0x13579000`, `0x11223344`, and `0x89ABCDEF`. The committed identities are
`SpecialAdd`, `Lw`, `Lw`, `SpecialXor`, `Sw`, `Addi`, `Andi`, `Bne`, its
`Addi` delay slot, `Lw`, `Lw`, `Sw`, and `Sw`. The branch is untaken, but its
slot still commits.

The first store writes transformed low word `0x92FC5000` as bytes
`[0x92,0xFC,0x50,0x00]` at SP IMEM local `0x000`; later stores write
`0x11223344` at `0x004` and `0x89ABCDEF` at `0x008`. Each byte receives exact
CPU-store provenance. The final state is PC/next-PC
`0xA4000074 / 0xA4000078`, Count `13`, with recognized but unrepresented
`RegimmBltz` as the next frontier. Its rejected step preserves that state.

The proof is synthetic and is not cartridge boot.
