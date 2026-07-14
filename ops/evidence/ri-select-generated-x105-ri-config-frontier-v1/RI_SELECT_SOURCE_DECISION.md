# RI_SELECT source decision

Decision: `RI_SELECT_COLD_ENTRY_ZERO_ONLY`.

Direct evidence establishes that RI_SELECT is one R/W RI register at physical
`0x0470000C`, that the bounded x105 path reads it before initialization, that a
nonzero value selects the NMI path, and that the cold path later writes
`0x14`. Nintendo's reset note establishes that NMI resets the CPU without
resetting the RCP.

The zero at cold entry is therefore a bounded control-flow inference tied to
the already explicit cold-x105 selectors. None of the inspected primary
sources states a generic RI_SELECT power-on reset value. Machine construction
and `Machine::reset` consequently leave the state unavailable; the complete
coupled cold-x105 bootstrap is the sole creation point for known zero with
source `ColdX105Entry`.

The later load consumes stored Machine state. It does not recompute a value
from reset kind, profile, expected branch direction, or host state. NMI
retention explains why the register is real state but does not authorize NMI
product behavior.
