# Unsupported branch in a delay slot

All six assigned identities are recognized while explicit delay context is
active and return the existing represented `Unsupported` outcome with category
`ControlFlowInDelaySlot`.

Application restores the full control-flow snapshot. Tests prove unchanged
`pc`, `next_pc`, context owner, Count, source/possible-link GPRs, and COP0
exception state. The matrix includes `BEQ`, `BNE`, `J`, `JAL`, `JR`, and
`JALR`, so both non-linking and linking rejection are direct runtime facts.

No chaining, annul, silent execution, silent ignore, recursive step, or
branch-likely behavior exists.
