# Exact RSP Lui

`Lui` is recognized only at major opcode `0x0F`. It has destination `rt` and a
16-bit immediate, consumes no scalar source, and produces
`zero_extend(immediate) << 16`.

The old destination is not an input. Destination `r0` discards only the write
while ordinary instruction cadence still commits. Provenance records local PC,
the exact four fetched SpImem bytes, and the immediate; the scalar register is
the sole owner of the result word.

Public `0x3C050020` at `0x028` produces available `r5 = 0x00200000`.
