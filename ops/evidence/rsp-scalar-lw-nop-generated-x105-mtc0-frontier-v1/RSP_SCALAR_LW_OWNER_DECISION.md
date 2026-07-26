# Scalar LW Owner Decision

`Sp::rsp` remains the singular owner of scalar values, availability,
provenance, RSP successor PC, last instruction, and committed count. `Sp::pc`
remains the singular current RSP PC. `SpDmem` and `SpImem` retain memory and
instruction-byte ownership.

No scalar register file, memory knowledge map, or mutable result owner was
added elsewhere.
