# RSP Delay Context Ownership

Sp::pc remains the singular current RSP PC owner. Sp::rsp owns `rsp.next_pc`,
the optional RSP delay context, scalar truth, branch provenance, and the RSP
committed count.

A committed branch:

1. moves current PC to `P + 4`;
2. sets `rsp.next_pc` to target or `P + 8`;
3. records exact immutable branch cause and taken truth;
4. increments the RSP count once;
5. selects CPU.

The slot is not executed recursively. A successful represented slot later
uses ordinary instruction effects, advances to the selected successor, clears
the context once, increments the RSP count once, and selects CPU.

Control flow attempted while this context is active remains unsupported.
