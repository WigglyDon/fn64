# RDRAM_MODE Sequence

Post-start mode writes are exact generated-family words:

- module zero: 31 manual writes (the first nominal-zero write precedes the
  start), then 34 automatic writes;
- module one: 32 manual and 34 automatic writes;
- absent aperture: 256 manual writes and one automatic zero-result write;
- global aperture: one `0xC4000000` request at PC `0xA4000290`.

Manual nominal inputs zero through seven yield scores 10 through 80 on present
modules. Automatic candidates zero through seven execute during conversion.
Both final module words are `0xC6808080`, with automatic nominal input seven.
The raw word is the stored truth; fields are derived.

