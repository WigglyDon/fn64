# RSP Vsub, Vaddc, Bgez, And SP Write Frontier

Evidence class: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

This bounded product evidence records exact element-zero `Vsub`, exact
element-zero `Vaddc`, exact scalar `Bgez`, the 256-iteration public x105 vector
sum loop, the seven existing post-loop scalar/control commits, and atomic
rejection of `Mtc0 r3,SP_WR_LEN` at local PC `0x090`.

`Sp::rsp` remains the single owner of vector registers, the sliced accumulator,
VCO, VCC, VCE, scalar registers, next PC, delay context, and RSP commit count.
`Sp::pc` remains the current-PC owner. The evidence uses only generated public
input and public `Machine::step`; it contains no private firmware, commercial
cartridge, user-task microcode, complete program, or complete memory dump.

The result remains BOOT-2 synthetic machine evidence. It is not BOOT-3, vector
ISA completeness, SP write DMA, RDP execution, graphics, audio, or game
compatibility.
