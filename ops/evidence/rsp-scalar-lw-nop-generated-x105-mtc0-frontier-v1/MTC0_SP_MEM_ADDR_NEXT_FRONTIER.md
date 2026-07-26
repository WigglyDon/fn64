# MTC0 SP_MEM_ADDR Next Frontier

At local `0x018`, public word `0x40800000` is identified as
`Mtc0 r0,SP_MEM_ADDR`. Source `r0` is Available zero from
`ArchitecturalZero`.

The selected RSP call rejects with `Mtc0Unsupported`. Turn remains RSP,
RSP `pc/next_pc` remains `0x018/0x01C`, RSP count remains `6`, and
`SP_MEM_ADDR`, DMA state, `r4`, `v12`, CPU counts, VI, and complete Machine
state are unchanged. No instruction at or after `0x01C` executes.
