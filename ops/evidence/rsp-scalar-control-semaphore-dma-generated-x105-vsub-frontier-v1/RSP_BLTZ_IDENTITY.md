# Exact RSP Bltz

`Bltz` is the REGIMM encoding with the exact BLTZ `rt` selector. It requires an
Available scalar source and is taken exactly when source bit 31 is set.

Public `0x04A0001B` at `0x02C` has delay slot `0x030` and target `0x09C`.
The public source is nonnegative, so every bounded public execution is not
taken. Synthetic tests separately prove negative, zero, and positive source
conditions and the exact target.

An unavailable source and any other REGIMM selector reject atomically.
