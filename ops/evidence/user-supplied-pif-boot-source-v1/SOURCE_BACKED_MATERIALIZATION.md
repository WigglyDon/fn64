# Source-backed materialization decision

Not implemented.

- `INFERENCE` Hardware causality is source-clear at the category level: IPL1
  copies an IPL2 subrange from PIF Boot ROM into SP IMEM at offset zero.
- `INFERENCE` The currently exposed x105 sequence consumes destination
  `[0x000,0x020)` and initially mutates `[0x000,0x02c)`.
- `UNKNOWN` The exact numeric source start/end inside the accepted 1,984-byte
  raw Boot ROM and the complete numeric IPL2 length remain unearned in current
  product evidence.
- `WORKER_CLAIM` Copying an assumed prefix, staging only offset zero, or using
  a firmware-derived table would guess the missing mapping and would be
  incomplete or proprietary.
- `LIVE_REPO_FACT` Accepted firmware therefore produces no SP IMEM byte. Every
  byte remains `Unknown`, including the complete currently consumed range and
  mutation-input range.

Exact next requirement: a later evidence/product packet must establish a
source-clear input range and supported layout, or authorize the smallest
firmware execution path that derives it, before any production provenance is
added.
