# Boot-Spine Architecture Checkpoint

Behavioral checkpoint:

- `554b01bac375cec696343ed9156dc2fc959f1af5`
- `core: name unavailable PIF ROM reset fetch seam`

This document seals the boot-adjacent machine truth earned at that commit. It is
not a boot plan and not a compatibility claim. Later documentation commits may
edit this file, but the behavioral checkpoint above names the source state being
described.

## Ownership Boundary

- `src/core` owns machine truth.
- `src/proof` owns proof and self-test instruments.
- `src/host/cli` owns no-window host tools.
- `src/host/sdl` owns SDL/window plumbing.
- Hosts do not own reset vectors, cartridge execution policy, memory-map policy,
  or emulator truth.
- `Machine` remains the unit of emulated truth.
- No public Bus exists.
- No full memory-map framework exists.

## Earned Boot-Adjacent Spine

### Cartridge Inspection

- Core owns normalized `Cartridge` bytes.
- The cartridge header entry word at normalized cartridge offset `0x08` is
  inspectable.
- The candidate IPL3 byte span `cart[0x00000040..0x00000fff]` is inspectable.
- The first candidate IPL3 word at normalized cartridge offset `0x40` is
  inspectable.
- z64, v64, and n64 normalization are proof-backed for this inspection seam.

These are cartridge byte facts only. They do not select pc/next_pc, stage bytes,
execute bytes, map cartridge CPU fetch, emulate PIF/CIC, or claim boot.

### Non-Boot Reset

- Reset pc is `0xbfc00000`.
- Reset next_pc is `0xbfc00004`.
- Reset does not use the cartridge header entry word.
- Reset does not stage cartridge bytes.
- Reset does not enter IPL3.

This is a named local reset-vector policy, not N64 reset/PIF boot.

### Unavailable PIF ROM Reset Fetch

- Reset fetch origin is `0xbfc00000`.
- The direct physical target is `0x1fc00000`.
- PIF ROM is unavailable and not modeled.
- The current reset step path is `kException` through the local direct-alias
  fetch-address AdEL exception seam.
- Post-step pc/next_pc are `0x80000180` / `0x80000184`.
- COP0 lineage is:
  - `EPC = 0xbfc00000`
  - `BadVAddr = 0xbfc00000`
  - `Cause = 0x00000010` (AdEL)
  - `Status.EXL` set
- Count does not advance for the reset-fetch failure.

No instruction executes from reset. The failure is not caused by cartridge,
SP DMEM, RDRAM, or header-entry bytes.

### SP DMEM Instruction Fetch

- Aligned CPU instruction fetch from Machine-owned SP DMEM is supported.
- The direct-alias entry around `0xa4000040` is proof-backed.
- SP DMEM fetch reads SP DMEM bytes, not RDRAM bytes.
- SP IMEM fetch remains rejected.

This is a narrow SP DMEM fetch seam. It is not RSP execution, COP2, PIF boot, or
general device execution.

### IPL3 Candidate Staging

- `Machine::stage_cartridge_ipl3_candidate_to_sp_dmem()` explicitly stages the
  candidate IPL3 byte span.
- Source: normalized cartridge `cart[0x00000040..0x00000fff]`.
- Destination: Machine-owned `SP DMEM[0x00000040..0x00000fff]`.
- Byte count: `0x0fc0`.
- Staging does not change pc, next_pc, or Count.
- Staging does not modify SP IMEM.
- Staging alone is non-entry.

Staging is an explicit machine mutation seam. It is not reset, load, boot, PIF
handoff, CIC behavior, or automatic IPL3 execution.

### Explicit IPL3 Candidate Entry

- `Machine::enter_sp_dmem_ipl3_candidate()` explicitly selects the candidate
  entry point.
- pc becomes `0xa4000040`.
- next_pc becomes `0xa4000044`.
- Entry does not stage bytes.
- Entry does not execute an instruction.
- Entry does not use the cartridge header entry word.
- Entry does not change Count or memory.

This is a candidate entry seam, not successful boot.

### No-Window Synthetic Probe

- `fn64_step_probe` links `fn64_core` only.
- The probe uses synthetic cartridge bytes generated in memory.
- The probe normalizes those bytes through the real cartridge loader, stages the
  candidate IPL3 span into SP DMEM, explicitly enters `0xa4000040`, and steps a
  bounded straight-line SP DMEM instruction sequence.
- The probe also shows reset returning to the non-boot reset vector and failing
  through the unavailable PIF ROM fetch seam.

This is synthetic staged candidate execution. It is not boot, not real IPL3, not
PIF/CIC emulation, and not compatibility evidence.

## Explicitly Unearned

- No boot.
- No PIF ROM bytes or execution.
- No PIF handoff.
- No CIC/security behavior.
- No automatic IPL3 execution.
- No real IPL3 execution claim.
- No cartridge CPU fetch/data mapping.
- No cartridge execution mapping.
- No SP IMEM fetch.
- No TLB.
- No COP1.
- No COP2/RSP execution.
- No cache behavior.
- No renderer.
- No host audio.
- No VI/RI device expansion.
- No public Bus.
- No full memory-map framework.
- No game compatibility claim.

## Next-Pressure Guidance

- Do not widen into random device/MMIO gardening.
- Do not add a public Bus before real pressure proves it belongs.
- Do not turn explicit stage/entry seams into automatic boot.
- Any future PIF handoff work must first name what is being modeled and must not
  imply proprietary PIF ROM/CIC behavior was implemented.
- Any future TLB work must be justified by an execution path that needs it, not
  by emulator completeness theater.
- Any future cartridge execution work must distinguish cartridge bytes, staged
  bytes, RDRAM bytes, and CPU-fetchable domains.
