# Second-wait delay slot

The pinned x105 source's 32-iteration loop ends each BNE with the next
source-level `Ori` as its ordinary delay slot; there is no intervening NOP in
the non-6101 path. The independently encoded layout is:

- `Addi` at `0xA400010C`;
- BNE at `0xA4000110`;
- `Ori r9,r0,0x010F` at `0xA4000114`.

The BNE schedules exactly one ordinary slot on every iteration, so the `Ori`
commits 32 times, including after the final untaken branch. It produces r9
`0x10F` without creating MI state or bus behavior.
