# CACHE Index Store Tag

The represented CACHE surface contains exactly the primary instruction and
primary data Index Store Tag operations.

| Generated PC | Raw word | op | Target | Line-index rule |
| --- | --- | ---: | --- | --- |
| 0xA4000408 | 0xBD080000 | 0x08 | primary instruction cache | address bits selected by 32-byte geometry |
| 0xA4000428 | 0xBD090000 | 0x09 | primary data cache | address bits selected by 16-byte geometry |

Planning requires a known base, cacheable direct KSEG0 effective address,
available TagLo and TagHi, zero supported TagHi reserved state, a supported
primary operation, and a representable TagLo primary state. Primary state zero
is invalid, state two is valid clean/exclusive, and state three is valid
dirty/exclusive. Reserved primary state one rejects before mutation for either
primary cache. Application mutates exactly one selected cache line and records
instruction, base lineage, effective address, line index, tag states, and delay
owner. It changes no backing memory and no GPR.

The generated zero tags make the selected line invalid. Unsupported CACHE
operations and unavailable sources reject before mutation.
