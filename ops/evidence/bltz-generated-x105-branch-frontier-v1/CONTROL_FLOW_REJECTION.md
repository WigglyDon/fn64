# Control-flow rejection

`RegimmBltz` joins the existing control-flow-in-delay-slot rejection matrix.
Identity plus active delay context are sufficient to reject before reading the
source register, scheduling a target, or mutating state. The outer `pc`,
`next_pc`, delay context, Count, COP0, GPRs, memories, and provenance remain
unchanged.

An unknown bootstrap source outside that case is a separate Machine rejection:
the plan captures the operand, the bootstrap source ledger rejects it before
application, and the full captured control-flow state remains unchanged.
