# No Cycle Accuracy Claim

Every public `Machine::step` selects and attempts at most one CPU or RSP
instruction. RSP branch and delay-slot commits are distinct calls separated by
the existing deterministic CPU interleave.

The product adds no cycle counter, frequency ratio, pipeline, stall, wall
clock, host timer, hidden batching, or recursive stepping. CPU Count and VI
advance only during CPU-selected calls. The composition is deterministic
instruction-boundary truth, not hardware timing.
