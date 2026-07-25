# SP PC Synchronization

SP PC write planning remains within the existing CPU direct-target store path.
A successful write applies the existing low-field mask/alignment policy to the
singular `Sp::pc`, then synchronizes `rsp.next_pc` to the local sequential
successor and clears stale RSP delay context.

It preserves scalar and unavailable-unit state, RSP committed count,
last-instruction provenance, run-start lineage, DMA state, and SP memory. It
does not create a run-start. Failed planning/application changes none of these
facts.
