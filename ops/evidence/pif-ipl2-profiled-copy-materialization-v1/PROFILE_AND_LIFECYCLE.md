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
firmware plus an explicit profile produces a complete replacement `SpImem`.
Accepted firmware without a profile and the no-firmware/no-profile state both
produce the prior all-Unknown replacement. A selected profile without accepted
firmware rejects before any represented mutation.

Every copied byte records `UserSuppliedPifFirmware { profile, source_offset }`.
All destination bytes outside the selected range remain zero-backed and
Unknown. The full consumed prefix `[0x000,0x020)` and current mutation-input
prefix `[0x000,0x02c)` lie inside every complete copied range.

Malformed or unsupported replacement input is rejected before accepted
firmware changes and preserves the independently selected profile. Full-state
snapshots prove firmware, profile, cartridge, SP IMEM bytes and provenance,
CPU, COP0, PC, next_pc, Count, RDRAM, SP DMEM, bootstrap state, and reservation
state are unchanged. A bootstrap source-range or missing-firmware failure is
likewise pre-mutation.

## Lifecycle

Firmware and profile can be installed in either order and remain independent
Machine-owned state. Reset preserves both while clearing bootstrap and SP IMEM
knownness. The next bootstrap rematerializes only when both are present.
Repeated bootstrap is deterministic. Replacing PAL or MPAL with the shorter
NTSC profile rebuilds SP IMEM and clears `[0x648,0x64c)` to Unknown, proving
stale bytes and provenance cannot survive. Separate Machines remain
independent.
