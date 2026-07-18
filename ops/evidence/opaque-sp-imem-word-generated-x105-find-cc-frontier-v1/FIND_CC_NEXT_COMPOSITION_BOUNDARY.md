# FindCC next composition boundary

The next instruction is word `0x0D000261` at PC `0xA40008F0`, decoded as
`Jal 0xA4000984`. Its prospective link is
`0xFFFFFFFFA40008F8`. The unexecuted delay slot at `0xA40008F4` is word
`0x00000000`, decoded as `Nop`.

This pass stops before the JAL. It does not claim FindCC, TestCCValue,
WriteCC, RDRAM_MODE, or calibration execution.
