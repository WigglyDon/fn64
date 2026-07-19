# Delay-slot and exception ownership

Taken BEQL reuses `CpuDelaySlotContext`; no second owner exists. The ordinary
slot step retains existing effect, rejection, and exception behavior. In
particular, a slot exception records the BEQL PC in EPC, sets BD, enters the
existing exception vector, and does not apply normal Count cadence to the
faulting slot.

Not-taken BEQL creates no delay context, so the nullified word cannot own an
effect or exception.
