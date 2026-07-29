# Public x105 SP write-DMA result

The accepted pre-instruction boundary is RSP PC/next-PC `0x090/0x094`, RSP
count 1088, processor turn RSP, scalar r3 `Available(0xFE817000)`, selected
IMEM local address `0x120`, physical RDRAM address `0x002FB1F0`, and two prior
DMA records.

The product gate must prove one Mtc0 commit, one third `SpToRdram` record, 192
equal source/destination bytes, RSP PC/next-PC `0x094/0x098`, RSP count 1089,
and CPU selection without CPU cadence changes during the RSP commit.
