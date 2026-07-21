# Primary cache geometry

The CPU owns one direct-mapped primary instruction cache and one direct-mapped
primary data cache per Machine.

| Cache | Capacity | Line size | Lines | Represented states |
| --- | ---: | ---: | ---: | --- |
| Primary instruction | 0x4000 bytes | 0x20 bytes | 512 | unavailable, invalid, valid-data-unavailable, valid with tag/data |
| Primary data | 0x2000 bytes | 0x10 bytes | 512 | unavailable, invalid, valid clean, valid dirty |

Construction and a fresh bootstrap leave lines unavailable rather than
inventing reset-invalid truth. The generated Index Store Tag loops establish
all 512 lines in each cache as invalid. The relocated KSEG0 fetch later replaces
I-cache line zero with one valid 32-byte RDRAM fill; I-cache lines 1 through 511
and every D-cache line remain invalid.
