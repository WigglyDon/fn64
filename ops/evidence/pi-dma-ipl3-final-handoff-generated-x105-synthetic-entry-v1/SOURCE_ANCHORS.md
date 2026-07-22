# Source anchors

Primary pinned inputs:

- N64-IPL x105 assembly at commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `src/ipl3.s`;
- N64-IPL RCP register header at the same commit, `include/PR/rcp.h`;
- the public SDK RCP header corroboration at ultra64.ca.

Locally cached audit copies were read only. Their SHA-256 values are:

- `ipl3.s`: `7b0699f20fab112ef4924f3c0c12b791f65f012b6d7ed3f43bbf07c89905ecbd`;
- `rcp.h`: `06af5a0a2538e344935694adf7d55a5f82eb53c9abf70a6fe89efae4351b105c`.

The pinned header defines PI base `0x04600000`, DRAM address at `+0x00`,
cartridge address at `+0x04`, RD length at `+0x08`, WR length at `+0x0C`, and
status at `+0x10` (cached `rcp.h` lines 682-700). It defines status read bits
DMA busy, I/O busy, and error, plus write bit `0x02` for interrupt clear
(lines 735-772). SP semaphore, MI DP clear, MI mask, AI status, and SI status
anchors appear at lines 206, 416, 438, 630, and 838.

The pinned x105 source programs PI at cached `ipl3.s` lines 688-705, polls the
status at lines 691 and 746, clears the SP semaphore at line 752, runs the
checksum at lines 792-843, performs final device clears at lines 868-888,
writes handoff globals at lines 893-914, tears down SP memory at lines
924-936, and jumps through the cartridge entrypoint at line 977.

Generated words and live `Machine::step` state, not source pseudo-instruction
spelling, are the execution authority.
