# Generated RCP 2.0 branch

`0x02020102 != 0x01010101`, so the guest Bne is taken. The Nop delay slot
executes once. Post-slot PC/next_pc are `0xA4000190/0xA4000194`, Count is
`32165`, and committed steps are `32181`.

Then:

- `0xA4000190 0x24100400 Addiu r16,r0,0x0400` selects spacing `0x400`.
- `0xA4000194 0x35718000 Ori r17,r11,0x8000` uses unchanged
  r11 `0xFFFFFFFFA3F00000` and builds
  `0xFFFFFFFFA3F08000`.

This is CPU composition, not stored MI branch state.
