# DPC_STATUS next frontier

The next word is `0x40835800` at local PC `0x098`:
`Mtc0 r3,DPC_STATUS`, source r3 `0x00000240`, control index 11.

It is identified but remains unsupported. Rejection must preserve the completed
SP write DMA and complete Machine state, keep processor turn RSP, retain
PC/next-PC `0x098/0x09C`, and retain RSP count 1090. No Dpc or Rdp owner is
created, and Break at `0x09C` is not executed.
