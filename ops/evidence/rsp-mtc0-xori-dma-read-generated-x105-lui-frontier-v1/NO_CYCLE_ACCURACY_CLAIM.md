# No Cycle Accuracy Claim

fn64 continues to model one selected processor instruction per successful
`Machine::step`. The SP read DMA is a complete functional effect at the
committing Mtc0 instruction boundary.

No cycle duration, busy/full interval, arbitration, stall, frequency ratio,
pipeline, countdown, wall clock, or host timer is represented. CPU Count and
VI continue to advance only on CPU-selected commits. The evidence therefore
makes no cycle-accuracy claim.
