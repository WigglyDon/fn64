# Machine lineage

## Current represented path

`LIVE_REPO_FACT` normalized cartridge bytes
→ `Machine::stage_cartridge_bootstrap`
→ cartridge `[0x040,0x1000)` staged in SP DMEM
→ SP IMEM replaced with zero backing plus `Unknown` provenance
→ represented stack pointer and architectural zero
→ public `Machine::step`
→ authentic `SpecialAdd` commit
→ known r9 effective-address base
→ represented `Lw` planning
→ SP IMEM offset zero
→ `Unknown` rejection before mutation.

## Hardware causality missing from the product

`INFERENCE` console PIF ROM IPL1 bytes
→ CPU execution at reset vector
→ word copy of IPL2 firmware into SP IMEM
→ CPU execution of IPL2 from SP IMEM
→ IPL2 stages and verifies cartridge IPL3 in SP DMEM
→ retained IPL2 prefix remains in SP IMEM
→ x105 IPL3 reads SP IMEM `[0x000,0x020)` as data
→ x105 IPL3 rewrites `[0x000,0x02c)`.

## Ownership consequence

- `INFERENCE` SP IMEM is still Machine-owned emulated truth, but the source
  bytes must originate from explicit firmware input or represented firmware
  execution, not from cartridge staging, reset backing, host policy, or probe
  setup.
- `WORKER_CLAIM` Current `Machine::stage_cartridge_bootstrap` is an HLE
  post-PIF creation point. It cannot honestly publish firmware residue while
  receiving only cartridge bytes and no PIF variant/input.
- `UNKNOWN` The future creation point may precede the current staging function
  or replace its post-PIF shortcut when real firmware execution is configured.
  That product topology remains unearned.
