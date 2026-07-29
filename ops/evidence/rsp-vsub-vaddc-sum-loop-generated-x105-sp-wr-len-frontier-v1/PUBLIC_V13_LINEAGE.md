# Public V13 Lineage

The first product vector arithmetic fact is one Unavailable v13 caused by Vsub
at local PC `0x060`, exact fetched IMEM provenance, self-alias, and the
Unavailable construction/reset borrow half.

Every subsequent Vaddc at local PC `0x070` creates one immutable cause record
whose old source-a state points to the previous v13 source. The final chain has
exactly 256 Vaddc nodes terminating at the one Vsub node. Shared immutable
links avoid exponential copying; no symbolic expression or unavailable vector
bytes exist.

No bounded instruction after the loop reads or stores v13.
