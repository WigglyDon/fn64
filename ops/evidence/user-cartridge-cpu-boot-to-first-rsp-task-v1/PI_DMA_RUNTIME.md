# PI DMA runtime

The concrete per-Machine `Pi` owner now accepts source-defined programmed DRAM
and cartridge addresses plus variable PI write-length values. A write-length
commit preflights the complete Machine-owned cartridge source range and RDRAM
destination range, then applies one atomic cartridge-to-RDRAM transfer.

The local user runtime first committed this generalized path at PC
`0x8000488C`. Exact transfer summaries remain Machine-owned typed records and
are intentionally omitted here because the endpoint proof does not require
publishing cartridge-derived ranges.

PI timing registers for both domains retain masked raw register truth and CPU
provenance. Their first local domain-one programming began at `0x8000573C`.
They create no cycle model.

Completion sets MI-owned PI pending truth; PI_STATUS clear removes it. The
model retains no partial-copy progress, FIFO, timing, cache snooping, or
cartridge-write route.
