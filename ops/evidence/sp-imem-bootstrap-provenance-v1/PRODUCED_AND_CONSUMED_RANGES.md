# Produced and consumed SP IMEM ranges

| Stage | Range | Access shape | Evidence and meaning |
| --- | --- | --- | --- |
| IPL1 | `[0x000, IPL2_length)` | contiguous word copy | `INFERENCE` Pinned `pifrom.s` copies the complete source-symbol span into SP IMEM. Numeric full length remains `UNKNOWN` under the firmware boundary. |
| x105 IPL3 prelude | `[0x000, 0x020)` | eight aligned contiguous reads | `INFERENCE` The loop reads one word, advances by four, and terminates on the eighth source word for the corroborated x105 state. |
| x105 IPL3 prelude | `[0x000, 0x020)` | eight aligned contiguous write-backs | `INFERENCE` Each consumed firmware word is combined with IPL3 data and written back to the same offset. |
| x105 IPL3 prelude | `[0x020, 0x02c)` | three aligned contiguous writes | `INFERENCE` The post-loop tail writes three further words before entering common IPL3. |

- `INFERENCE` Complete initial SP IMEM consumption relevant to the exposed
  sequence is exactly `[0x000, 0x020)`.
- `INFERENCE` Complete SP IMEM mutation by that prelude is exactly
  `[0x000, 0x02c)`.
- `INFERENCE` Reads are contiguous, word-aligned, and data-dependent in the
  general source algorithm; the corroborated x105 prefix terminates at word
  index seven.
- `INFERENCE` The sequence writes back into SP IMEM; it does not merely inspect
  the first word.
- `INFERENCE` At least one retained IPL2 word varies with PIF/console region in
  public HLE corroboration. The exact PIF revision matrix is `UNKNOWN`.
- `INFERENCE` Cartridge boot/CIC family selects whether the x105 IPL3 prelude
  consumes this residue. The residue producer itself is the console PIF
  firmware, not the cartridge title or whole-ROM identity.
- `UNKNOWN` A broader later SP IMEM consumer is outside the currently exposed
  prelude and is not claimed.

`WORKER_CLAIM` Representing only offset zero would be incomplete. Representing
the eight-word prefix as constants would still be forbidden because it would
embed proprietary IPL2 code, and it would omit the prelude's subsequent writes.
