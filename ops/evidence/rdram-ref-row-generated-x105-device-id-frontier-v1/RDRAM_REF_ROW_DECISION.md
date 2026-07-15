# RDRAM REF_ROW decision

Decision: `RDRAM_REF_ROW_EXACT_X105_ZERO_WRITE_ONLY`.

The pinned register header defines the address, while the bounded x105 source
defines one global zero write and describes its intent. That supports an exact
raw-zero write fact. It does not support interpreted fields, reads, refresh
progress, timing, completion, or arbitrary values.
