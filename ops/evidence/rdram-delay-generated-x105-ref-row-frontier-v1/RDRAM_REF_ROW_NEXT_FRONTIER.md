# RDRAM_REF_ROW next frontier

At PC `0xA4000128`, word `0xAD400014` decodes as Sw r0,20(r10).
r10 is `0xFFFFFFFFA3F80000`; r0 is architectural zero. The effective address is
`0xFFFFFFFFA3F80014`, CPU address `0xA3F80014`, physical address `0x03F80014`,
and transfer word zero. It rejects as `DirectTargetMiss`, preserving the
consumed-transfer absence and complete RDRAM-delay fact. REF_ROW is not
implemented.

