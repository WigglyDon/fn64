# Generated WriteCC Return Path

After the request, public stepping executes:

- `0xA4000BC4 0x8FBF001C` — restore ra from known SP-IMEM `0xE9C`;
- `0xA4000BC8 0x27BD0028` — restore sp;
- `0xA4000BCC 0x03E00008` — JR to `0xA4000A24`;
- `0xA4000BD0 0x00000000` — one Nop delay slot;
- `0xA4000A24 0x0000F025` — initialize s8 to zero;
- `0xA4000A28 0x241AFFFF` — construct all-ones k0.

The JR link source is the loaded known stack word and the existing delay owner
clears after the Nop.
