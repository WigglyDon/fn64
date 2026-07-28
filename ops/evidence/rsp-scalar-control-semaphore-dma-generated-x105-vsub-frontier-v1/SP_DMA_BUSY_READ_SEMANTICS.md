# SP DMA Busy Read Semantics

Existing scalar `Mfc0` gains exactly control index 6, `SP_DMA_BUSY`.
Sp and its existing DMA records remain the source owner.

The current DMA model applies the complete transfer atomically when
`SP_RD_LEN` commits. No later instruction boundary has a pending transfer, so
the owner-derived read result is Available zero. The read has no source side
effect and uses ordinary Mfc0 destination/r0/provenance rules.

Public `0x40033000` at `0x054` writes `r3 = 0`. Control index 5
`SP_DMA_FULL` remains explicitly unsupported.
