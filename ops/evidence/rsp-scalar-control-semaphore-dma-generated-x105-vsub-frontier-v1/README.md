# RSP Scalar Control, Semaphore, Second DMA, And Vsub Frontier

Evidence class: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

This increment represents exact RSP `Lui`, `Addi`, `Bltz`, and `Bne`,
including one independently owned RSP delay context which survives intervening
CPU-selected calls. The public semaphore loop progresses only because ordinary
guest CPU execution clears the Sp-owned semaphore. It then reuses the existing
scalar `Lw`, `Mtc0`, `Xori`, and shared atomic read-DMA seams.

The bounded public run copies RDRAM `[0x400,0x1400)` to all of DMEM, reads
`SP_DMA_BUSY` as idle after atomic completion, commits the not-taken busy-loop
branch and its `Xori` delay slot, and stops without mutation at identified
`Vsub v13,v13,v13` at local PC `0x060`.

No private input, host semaphore shortcut, vector arithmetic, BREAK, scalar J,
DMA timing, generic branch/DMA framework, BOOT-3, or compatibility claim is
included.
