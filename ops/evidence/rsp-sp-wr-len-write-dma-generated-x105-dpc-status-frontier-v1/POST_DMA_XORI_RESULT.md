# Post-DMA Xori result

After one actual CPU-selected interleave, existing RSP
`Xori r3,r0,0x0240` at local PC `0x094` commits once. It produces
`r3 = 0x00000240`, advances RSP PC/next-PC to `0x098/0x09C`, advances the RSP
count to 1090, and leaves the completed DMA record and destination bytes
unchanged.
