# Public X105 DMA-Busy Loop

At local PC `0x054`, `Mfc0 r3,SP_DMA_BUSY` commits Available zero. At `0x058`,
`Bne r3,r0,0x054` compares two complete zero values and is not taken.

The branch still stages delay slot `0x05C`. One intervening CPU-selected call
preserves the exact RSP delay context. `Xori r3,r0,0x0FF0` then commits once,
producing `r3 = 0x00000FF0`, clearing the delay context, and advancing:

- current SP PC: `0x060`;
- `rsp.next_pc`: `0x064`;
- RSP committed count: 56.

DMA records, DMEM, scalar `r4`/`r6`, unavailable `v12`, and semaphore truth
remain unchanged.
