# No Cycle-Accuracy Claim

fn64 remains an instruction-level Machine.

Each Machine::step selects and attempts at most one CPU or RSP instruction.
RSP delay contexts persist across real intervening CPU-selected instruction
calls. SP DMA completes atomically at its triggering instruction boundary.
CPU Count and VI advance only on CPU-selected commits.

No branch pipeline cycles, load stalls, DMA busy cycles, partial transfer,
frequency ratio, host timer, wall clock, or cycle-accuracy claim exists.
