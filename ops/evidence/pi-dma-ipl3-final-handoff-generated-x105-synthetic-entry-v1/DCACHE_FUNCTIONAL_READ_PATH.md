# Functional D-cache read path

The CPU-owned primary D-cache remains direct mapped with 512 lines of 16
bytes. This pass adds functional aligned KSEG0 `Lw` from Machine-owned RDRAM:
a matching valid line supplies the big-endian word; an invalid or mismatched
line preflights and fills one full 16-byte line, records fill provenance, marks
it valid clean, and returns the requested word. KSEG1 remains uncached.

The generated 1 MiB checksum performs 262,144 word loads. Measured cache
behavior is 65,536 fills and 196,608 hits. At completion all 512 D-cache lines
are valid clean and none is dirty. There is no KSEG0 store, dirty writeback,
timing, or cache-coherence protocol in the bounded path.

PI DMA deliberately does not snoop either primary cache. This path begins with
all D-cache lines invalid, and the future entrypoint I-cache line remains
invalid, so no generic coherency behavior is needed or claimed.
