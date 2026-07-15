# Hardware-effect boundary

Represented truth is register state only: length 15, initialization mode true,
and exact CPU-store provenance.

The public source explains an effect on repetition of the next bus write. This
pass does not represent that effect. There is no pending-write buffer,
replicated transaction, 16-byte write, RDRAM-register mutation, MI/RDRAM
clock, completion flag, hidden countdown, readiness fact, module discovery,
or electrical behavior.

The following RDRAM_DELAY store rejects closed; MI state does not authorize it.
