# Next frontier

After thirteen generated commits, the next instruction at `0xA4000074` is
recognized as `RegimmBltz`. `Machine::step` does not yet represent REGIMM
control flow, so it returns `UnrepresentedInstruction` with PC/next-PC
`0xA4000074 / 0xA4000078`, Count `13`, registers, memory, provenance, COP0, and
delay context preserved.

This pass adds no second instruction family. The next product pressure is a
separately bounded REGIMM decision, not a reason to generalize control flow.
