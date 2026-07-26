# Public X105 Scalar LW Result

Pre-Lw: CPU `pc/next_pc = 0x80000010/0x80000014`, Count `252348`, CPU
committed count `252364`; RSP `pc/next_pc = 0x00C/0x010`, committed count `3`,
turn RSP.

`0x8C040040` commits once as `Lw r4,0x40(r0)`. Available bootstrap-owned bytes
`03 A0 48 20` produce Available `r4 = 0x03A04820` with exact
instruction/base/DMEM provenance. RSP `pc/next_pc` becomes `0x010/0x014`,
count becomes `4`, and turn becomes CPU. CPU Count, CPU committed count, VI,
DMEM, and unavailable `v12` are unchanged.
