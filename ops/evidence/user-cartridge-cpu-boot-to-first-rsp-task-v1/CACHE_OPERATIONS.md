# Cache operations

The existing CPU-owned direct-mapped primary caches remain the only cache
owners. RDRAM remains the only backing-byte owner.

The runtime earned:

- primary I/D Index Invalidate;
- primary D Index Writeback Invalidate;
- primary D Hit Writeback;
- primary I/D Hit Invalidate;
- cached halfword and doubleword access through the existing KSEG0 data cache.

Dirty writeback plans validate the complete RDRAM target before mutation. No
partial writeback, fill, invalidation, or Count cadence occurs on rejection.
KSEG1 remains uncached. PI and SP DMA remain non-snooping; guest cache
operations make visibility explicit.

No secondary cache, timing, miss penalty, write buffer, parity, speculative
fetch, or generic coherence layer was introduced.
