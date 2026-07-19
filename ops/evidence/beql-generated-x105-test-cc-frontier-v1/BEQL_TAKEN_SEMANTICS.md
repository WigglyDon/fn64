# BEQL taken semantics

Equal available operands commit BEQL once, set `pc=P+4`, set `next_pc` to the
existing calculated target, establish the existing single delay context owned
by `P`, and advance Count once. The slot is not executed recursively. A second
public `Machine::step` executes it exactly once, advances Count once, transfers
to the target, and clears the delay context through existing cadence.

A slot exception retains existing EPC/BD ownership: EPC is the BEQL PC, BD is
true, and the faulting slot receives no normal Count cadence.
