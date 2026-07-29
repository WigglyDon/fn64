# SP_WR_LEN identity and decode

The exact public word is `0x40831800`: RSP `Mtc0 r3,SP_WR_LEN`.

- COP0 transfer selector: Mtc0
- scalar source: r3
- control index: 3
- source value: `0xFE817000`
- scalar destination: none

Only control index 3 is added. Existing indices 0, 1, and 2 retain their
accepted behavior; all other Mtc0 destinations remain closed.

