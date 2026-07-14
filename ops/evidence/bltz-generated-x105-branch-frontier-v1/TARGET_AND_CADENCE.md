# Target and cadence

For the generated frontier word constructed from opcode `0x01`, rs `31`,
REGIMM subcode `0x00`, and immediate `0x0001`:

- instruction PC: `0xA4000074`;
- delay-slot PC: `0xA4000078`;
- branch target: `0xA400007C`;
- untaken successor: `0xA400007C` for this displacement;
- old r31: `0xFFFFFFFFA4001550`, therefore taken.

The equal target/fall-through addresses in this one generated vector do not
replace taken/untaken proof. Separate positive, negative, zero, wrapping, and
unequal target/fall-through tests discriminate the common target helper.

After the BLTZ commit, `pc=0xA4000078`, `next_pc=0xA400007C`, Count is
incremented once, and context names owner `0xA4000074`. The target executes
only after a successful slot. The slot advances Count once and clears context.
