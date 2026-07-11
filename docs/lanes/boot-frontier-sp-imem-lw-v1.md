# Boot Frontier SP IMEM And Lw V1

Context role: completed partial lane coordination memory.
Scope: the authentic BOOT-2 frontier spanning SP IMEM representation, narrow CPU-data routing, aligned `Lw`, and bounded trace continuation.
Canonical for: this lane's purpose, topology, authority boundary, overlap decision, proof, and integration conditions.
Not canonical for: accepted product behavior, private ROM content, candidate acceptance, or canonical integration.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [machine core](../context/subsystems/machine-core.md), [CPU execution](../context/subsystems/cpu-execution.md), [memory and cartridge](../context/subsystems/memory-and-cartridge.md), [inspection](../context/subsystems/inspection-and-evidence.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, candidate creation, context propagation, blocking, integration, frontier change, or retirement.

## Lane identity and state

- Lane ID: `boot-frontier-sp-imem-lw-v1`
- Purpose: resolve the current authentic private-ROM frontier without hacks by composing Machine-owned SP IMEM, the narrow observed CPU-data route, complete aligned `Lw`, source/result lineage, and bounded no-window trace continuation.
- Supervisor role: SP IMEM And Authentic Boot Frontier Supervisor GPT
- Worker Codex worktree: `/home/don/fn64-worktrees/boot-frontier-sp-imem-lw-v1`
- Branch: `worker/boot-frontier-sp-imem-lw-v1`
- Accepted base source: `5f77d2df6005fe34ebb20f4751c2980ff73c57f1`.
- Registration Context-SHA: `aa529390c16cbae1ee073d0ecee1aa29626acd998dfa385775cc8a921fd49a21`; the literal committed post-registration Context-SHA belongs in the Master provisioning report and first executable packet.
- Selected topology: `COMBINED`
- Status: **PARTIAL — INTEGRATED**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Launch state: completed Worker pass; no relaunch or repair is authorized by this closure.

## Exact writable scope

- `rust/crates/fn64-core/src/lib.rs`
- `rust/crates/fn64-core/src/cpu.rs`
- `rust/crates/fn64-core/src/cpu/address.rs`
- `rust/crates/fn64-core/src/cpu/cop0.rs`
- `rust/crates/fn64-core/src/cpu/instruction.rs`
- `rust/crates/fn64-core/src/cpu/registers.rs`
- `rust/crates/fn64-core/src/cpu/scalars.rs`
- `rust/crates/fn64-core/src/machine.rs`
- `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`
- `rust/crates/fn64-core/src/rdram.rs`
- `rust/crates/fn64-core/src/sp_dmem.rs`
- `rust/crates/fn64-core/src/sp_imem.rs` when created as the Machine-owned storage owner
- `rust/crates/fn64-inspection/src/boot_probe.rs`
- `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`
- `rust/crates/fn64-inspection/tests/boot_probe_cli.rs`
- `ops/evidence/boot-frontier-sp-imem-lw-v1/**`
- external lane logs and artifacts

Tests embedded in the listed Rust source files are owned with those files.
Cargo manifests, `Cargo.lock`, and `rust/verify-forward` are outside this
lane; a real dependency need must return through Master authority instead of
being assumed.

## Machine authority and prohibitions

The lane may represent 4 KiB Machine-owned SP IMEM, explicit construction and
reset, the narrow observed `0xA4001000` CPU-data route, complete aligned
big-endian `Lw`, sign extension, read-before-write aliasing, zero-register
behavior, bootstrap source/result lineage, data AdEL, no-partial-mutation
rejection, and exactly-once committed cadence. `Machine::step` remains the
sole public execution entrance.

The lane may continue the bounded real trace only for source-reached,
reviewable semantic families that preserve current ownership. It must stop at
BOOT-3/BOOT-4, a new large hardware subsystem, proprietary-firmware pressure,
genuine multi-owner routing pressure, an unreviewable expansion, or a stable
unsupported loop/exception frontier.

Prohibited authority:

- no edits to `README.md`, `AGENTS.md`, `docs/**`, `tools/fleet/**`,
  `ops/fleet/**`, `rust/README.md`, `rust/PARITY.md`, `rust/AGENTS.md`,
  `rust/verify-forward`, Cargo metadata, lane registry, or integration queue;
- no edits to another lane's evidence, Worker topology, canonical `main`, or
  user ROM files;
- no title/cartridge-ID/ROM-hash selector, patch table, ROM mutation,
  instruction skip, direct-entry staging, imported state/trace, host-authored
  success, silent MMIO zero, swallowed exception, or broad PIF register
  profile;
- no proprietary PIF/BIOS blob, generic bus, generalized memory-map framework,
  SDL/window/audio, graphics, compatibility claim, or deployment.

Master Codex alone owns branch/worktree topology, context propagation,
candidate integration, and canonical push. Worker authority is
`WORKER_ASSIGNED_WORKTREE_NO_BRANCH_MANAGEMENT_NO_PUSH_NO_INTEGRATION`.

## Proof, private input, dependencies, and overlap

Required proof covers storage size/construction/reset; exact address and local
offset classification; aligned/bounded big-endian reads; `Lw` decode,
effective-address arithmetic, alignment, sign extension, base/destination
aliasing, GPR zero, bootstrap knownness/result lineage, exception/rejection
rollback, and exactly-once cadence. It also reruns the bounded private trace and
reports only the highest honestly reached checkpoint and first frontier.

Private-ROM authority is not granted by provisioning. The future supervisor
seed may authorize read, digest, structural validation, and no-window execution
of `/home/don/fn64/roms/test.z64`. The input must never be modified, copied,
moved, renamed, staged, committed, dumped, or packaged.

Dependencies: accepted BOOT-2 source and the existing RDRAM data-alignment,
data-AdEL, decode/identity, control-flow snapshot, bootstrap-knownness, and
public inspection seams.

Direct overlap internal to the frontier is deliberate: SP IMEM routing and
`Lw` both require `machine.rs` data-action/application ownership; `Lw`
also requires the bootstrap knownness ledger and boot-probe continuation.
There is no concurrent product lane. Indirect overlap is limited to future
Master context reconciliation after an accepted candidate.

Integration order: this lane's product truth first, then Master capability,
lane, queue, evidence, and Context-SHA reconciliation. Required integration
proof includes source review, focused synthetic tests, `./rust/verify-forward`,
the authorized private no-window trace, and clean-checkout reproduction when
runtime input is used.

Stop before modification on topology or Context-SHA mismatch. Stop on ROM/legal
leakage, game-specific behavior, ownership conflict, speculative abstraction,
unbounded scope, or contradictory accepted law. Retire after accepted
integration or an explicit Master decision.

## Integrated partial result

- Candidate: `dcb9f1bfac971a5a637f4c168aa57c9d0228ea0c`.
- Parent: `5f77d2df6005fe34ebb20f4751c2980ff73c57f1`.
- Product-integration SHA: the candidate itself, fast-forwarded unchanged onto
  the Master integration branch; the containing reconciliation/canonical SHA is
  owned by the external Master artifact because this page cannot name its own
  containing commit.
- Verified worker artifact source:
  `/tmp/UPLOAD_ME_fn64_boot_frontier_sp_imem_lw_v1.tar.gz`.
- Durable verified worker artifact:
  `/tmp/fn64-final-artifacts/UPLOAD_ME_fn64_boot_frontier_sp_imem_lw_v1_fca9c7e0.tar.gz`.
- Artifact SHA-256:
  `fca9c7e0608617490da38b8054a56716de16372e00929cb584b85fe5de88debb`.
- Authorized private input identity:
  `c916ab315fbe82a22169bff13d6b866e9fddc907461eb6b0a227b82acdf5b506`,
  `33554432` bytes; no input content is tracked or packaged.
- Accepted product truth: private Machine-owned SP IMEM with explicit unknown
  backing provenance, narrow CPU-data routing, and complete aligned `Lw` over
  direct RDRAM and known SP IMEM.
- Authentic result: BOOT-2 remains highest. The `Lw` at `0xA4000044` rejects
  before mutation because SP IMEM offset zero is unknown.
- Unresolved frontier: identify the exact Machine-owned creation event for
  bytes `0x000..0x003`; owner and value remain `UNKNOWN`.
- Worker repair: not requested. Worker push: not performed. Worker worktree and
  branch remain preserved.
