# Current-Control Response-Test Boundary

Execution stops at PC/next-PC `0xA4000A2C/0xA4000A30`, Count 32250, commits
32266. The unexecuted word is `0xAE9A0000`, `Sw r26,0(r20)`. r20 is
`0xFFFFFFFFA0000000` from `Lui` at `0xA400015C`; r26 is all ones from
`Addiu` at `0xA4000A28`. Effective/CPU/physical addresses are
`0xFFFFFFFFA0000000`, `0xA0000000`, and `0x00000000`; transfer word would be
`0xFFFFFFFF`. Current target classification is direct RDRAM offset zero.

This instruction is inspected but never passed to `Machine::step` in the
bounded composition.
