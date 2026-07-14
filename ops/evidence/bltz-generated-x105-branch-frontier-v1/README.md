# BLTZ generated x105 branch frontier

Frontier classification: `BLTZ_X105_FRONTIER_CONFIRMED`.

Product classification: `ACCEPTED — BLTZ AND GENERATED X105 BRANCH FRONTIER`.

The accepted generated NTSC cold-cartridge x105 composition commits thirteen
instructions and reaches `RegimmBltz` at CPU address `0xA4000074`. The encoded
source is r31, whose Machine-owned retained-link value is
`0xFFFFFFFFA4001550`; the existing full-width signed GPR comparison policy
therefore selects the taken path. The branch target is `0xA400007C` and its
ordinary delay slot at `0xA4000078` is an aligned `Sw` from r0 to SP IMEM local
offset `0x00C`.

The implemented identity reuses the existing full-GPR signed comparator and
ordinary branch plan/application. Generated public-step proof commits BLTZ as
step 14 and the zero-store slot as step 15, then stops atomically at recognized
but unrepresented `Cop0Mtc0` at `0xA400007C`. Every other REGIMM identity
remains unrepresented.

This directory records source-qualified facts and generated proof only. It
contains no private PIF or cartridge input, firmware word, copied instruction
stream, assembly block, or disassembly block. Synthetic progress does not
change the authentic BOOT-2 checkpoint.
