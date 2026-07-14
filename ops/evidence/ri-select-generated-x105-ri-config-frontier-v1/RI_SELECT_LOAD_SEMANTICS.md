# RI_SELECT load semantics

The existing `Lw` planner reads the old known base GPR, adds the sign-extended
16-bit immediate with the current wrapping rule, and checks word alignment
before target access. Exact physical `0x0470000C` selects RI_SELECT only when
the Machine-owned state is available.

The read returns the stored 32-bit word without side effects. Existing `Lw`
sign extension supplies the 64-bit GPR result, the destination receives
`KnownInstructionResult` lineage, normal PC/next_pc cadence commits once, and
Count advances once. The generated zero remains zero after sign extension.

Unavailable state returns `RiSelectUnavailable` before mutation. Unaligned
access uses the existing AdEL path. Other RI offsets remain direct target
misses.
