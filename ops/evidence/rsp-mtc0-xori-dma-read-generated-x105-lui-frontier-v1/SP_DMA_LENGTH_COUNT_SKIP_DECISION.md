# SP DMA Length, Count, And Skip Decision

Existing decode is length `(raw & 0x0FF8)+8`, count
`((raw>>12)&0xFF)+1`, and skip `(raw>>20)&0x0FFF`. Raw zero therefore means
length 8, count 1, skip 0, transferred bytes 8. The CPU-side value domain is
not broadened.
