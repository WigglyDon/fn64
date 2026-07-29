# SP write-DMA destination block map

For block `n` in `0..24`:

`destination_start = 0x002FB1F0 + n * 0xFF0`

Each block contains eight bytes. The first range is
`0x002FB1F0..0x002FB1F8`; the second starts at `0x002FC1E0`; the final range is
`0x00312080..0x00312088`. All 24 ranges are disjoint.

The complete exact map is in `PUBLIC_X105_SP_WRITE_DMA_BLOCKS.tsv`.

