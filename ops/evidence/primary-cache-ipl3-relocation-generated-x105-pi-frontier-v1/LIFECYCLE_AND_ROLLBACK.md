# Lifecycle and rollback

Construction and a fresh complete bootstrap contain no fabricated CP0 tag,
cache-line, or SP-control truth. Generated software writes the tags and
invalidates the caches. Repeated complete bootstrap replaces CPU/SP runtime
state, clearing stale tag writes, cache operations/fills, SP commands, and SP
PC provenance while preserving the immutable fixed RDRAM profile.

Failed bootstrap preserves the complete prior Machine, including cache arrays,
CP0 tag facts, SP control, RDRAM bytes, and relocation bytes. Independent
Machines own independent arrays and control facts.

I-cache fill application retains the prior selected line for rollback if the
subsequent represented instruction application rejects. Every other rejected
plan is mutation-free.
