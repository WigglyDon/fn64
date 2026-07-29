# SP write-DMA register evolution

The existing `Sp` record policy evolves addresses once after complete transfer
application:

- local selected address: `0x1120 -> 0x11E0`
- physical RDRAM address: `0x002FB1F0 -> 0x00313070`

The SP memory-address state retains its original programmed transfer word and
source lineage while its owner-local interpreted address advances. The DRAM
address becomes one `DmaAdvance` state pointing at record index 2 and the
RSP-Mtc0 trigger.
