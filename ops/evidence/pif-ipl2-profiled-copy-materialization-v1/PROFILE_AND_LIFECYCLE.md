# Profile And Lifecycle

## Exact mappings

| Explicit profile | Raw source | SP IMEM destination | Bytes |
| --- | --- | --- | ---: |
| `NTSC_PINNED` | `[0x0d4,0x71c)` | `[0x000,0x648)` | 1,608 |
| `PAL_PINNED` | `[0x0d4,0x720)` | `[0x000,0x64c)` | 1,612 |
| `MPAL_PINNED` | `[0x0d4,0x720)` | `[0x000,0x64c)` | 1,612 |

`LIVE_REPO_FACT`: the closed `PifIpl2Profile` enum owns these meanings. No
profile is inferred from firmware content, hash, path, filename, cartridge
metadata, title, region, product code, or digest.

## Creation and atomicity

The creation event is `Machine::stage_cartridge_bootstrap`. It preflights the
cartridge source and builds replacement SP DMEM, SP IMEM, CPU, and provenance
state in local values before replacing represented Machine state. Accepted
profiled firmware produces a complete replacement `SpImem`; absent firmware
produces the prior all-Unknown replacement.

Every copied byte records `UserSuppliedPifFirmware { profile, source_offset }`.
All destination bytes outside the selected range remain zero-backed and
Unknown. The full consumed prefix `[0x000,0x020)` and current mutation-input
prefix `[0x000,0x02c)` lie inside every complete copied range.

Malformed or unsupported replacement input is rejected before the accepted
firmware/profile pair changes. A bootstrap source-range failure preserves the
firmware, SP IMEM bytes and provenance, CPU, RDRAM, SP DMEM, checkpoint state,
and control flow.

## Lifecycle

Reset preserves the immutable firmware/profile input but clears bootstrap and
SP IMEM state. The next bootstrap rematerializes the selected copy. Repeated
bootstrap is deterministic. Replacing PAL/MPAL with the shorter NTSC profile
rebuilds SP IMEM and clears the former four-byte tail to Unknown, proving stale
bytes and provenance cannot survive. Separate Machines remain independent.
