# Exact value and rotation policy

Independent calculation:

- `RDRAM_DELAY(5, 7, 3, 1) = 0x28381808`
- `ROT16(0x28381808) = 0x18082838`

Only CPU low word `0x18082838` is accepted with the exact pending transfer.
Words such as `0x28381808`, adjacent values, zero, and all-ones reject before
mutation. The logical fact stores `0x28381808`; `0x18082838` remains provenance.

