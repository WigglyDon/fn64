# Module Layout and Device Type

Pinned `DEVICE_TYPE(1, 9, 11, 0, 0, 0, 0)` expands to `0xB0190000` and names a
two-bank, 2 MiB module. The fixed 4 MiB profile therefore owns two stable module
records, indices zero and one, each `0x00200000` bytes.

Module register bases after final placement are `0x03F00000` and `0x03F00800`.
Final data bases are `0x00000000` and `0x00200000`, yielding one linear range
through `0x003FFFFF`. Module metadata does not duplicate or relocate the single
RDRAM byte backing.
