# SP IMEM Bootstrap Provenance V1

Context role: active lane coordination memory.
Scope: causal reconstruction of the Machine-owned creation event for SP IMEM bytes `0x000..0x003` before the authentic `Lw` at `0xA4000044`.
Canonical for: this lane's purpose, topology, authority boundary, writable paths, proof, dependencies, overlap, and stop conditions.
Not canonical for: accepted product behavior, private ROM content, candidate acceptance, or canonical integration.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [represented-machine capability](../../rust/PARITY.md), [machine core](../context/subsystems/machine-core.md), [memory and cartridge](../context/subsystems/memory-and-cartridge.md), [inspection](../context/subsystems/inspection-and-evidence.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, candidate creation, source-owner discovery, context propagation, blocking, integration, frontier change, or retirement.

## Lane identity and state

- Lane ID: `sp-imem-bootstrap-provenance-v1`
- Purpose: identify and, only when evidence is source-clear, represent the
  Machine-owned creation event that makes SP IMEM bytes `0x000..0x003` known
  before the authentic `Lw` at `0xA4000044`.
- Supervisor role: SP IMEM Bootstrap Provenance Supervisor GPT
- Worker Codex worktree:
  `/home/don/fn64-worktrees/sp-imem-bootstrap-provenance-v1`
- Branch: `worker/sp-imem-bootstrap-provenance-v1`
- Accepted base source: canonical `main` after this registration commit is
  integrated and pushed.
- Governing Context-SHA: the exact committed post-registration value owned by
  the Master provisioning report and first executable packet.
- Selected topology: one provenance lane; ordinary control flow is deferred.
- Status: **PROVISIONED — AWAITING SUPERVISOR PACKET**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Provisioning exception: `NONE`
- Launch state: not started; Don must not launch Worker Codex before Master GPT
  issues the matching supervisor seed.

## Exact writable scope

- `rust/crates/fn64-core/src/lib.rs`
- `rust/crates/fn64-core/src/machine.rs`
- `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`
- `rust/crates/fn64-core/src/sp_imem.rs`
- `rust/crates/fn64-inspection/src/boot_probe.rs`
- `rust/crates/fn64-inspection/tests/boot_probe_cli.rs`
- `ops/evidence/sp-imem-bootstrap-provenance-v1/**`
- external lane logs and artifacts

Tests embedded in the listed Rust source files are owned with those files.
The lane may leave product source unchanged and add only its owned evidence
when a precise unavailable fact blocks honest implementation.

## Machine authority and prohibited authority

The lane starts with one question: what source-clear represented creation event
establishes SP IMEM bytes `0x000..0x003` before the CPU executes the `Lw` at
`0xA4000044`? Current classification is `UNKNOWN`. Reset-state creation, PIF
execution, a PIF-to-SP transfer, SP DMA, another bootstrap transfer, and an
unknown external hardware fact are hypotheses only until evidence selects one.

When valid evidence identifies the event, the lane may add the smallest
Machine-owned value/provenance creation rule, preserve four-byte knownness,
rerun the authentic load, and continue the bounded trace to its next honest
frontier.

Prohibited authority:

- no backing-zero assumption, title/cartridge-ID/ROM-digest selector, patch
  table, imported memory dump, replayed trace, direct host/probe SP IMEM staging,
  or fabricated byte value;
- no proprietary PIF/BIOS blob, broad PIF HLE profile, or SP DMA implementation
  without evidence that it owns this exact event;
- no generic bus, generalized memory map, game compatibility claim, graphical
  host, SDL/window/audio, deployment, or unrelated instruction family;
- no edits to `docs/**`, `tools/fleet/**`, `ops/fleet/**`, Cargo metadata,
  `rust/verify-forward`, another lane's evidence, user ROM files, Worker
  topology, or canonical `main`.

Master Codex alone owns branch/worktree topology, context propagation,
candidate integration, lane/queue reconciliation, and canonical push. Worker
authority is
`WORKER_ASSIGNED_WORKTREE_NO_BRANCH_MANAGEMENT_NO_PUSH_NO_INTEGRATION`.

## Proof, dependencies, and overlap

Required proof begins with a causal evidence record, considers every plausible
owner without promoting inference, and records `UNKNOWN` when competing causes
cannot be distinguished. Any implementation must prove production-inaccessible
test staging, lawful source bytes, exact offsets, value/provenance creation,
four-byte knownness, no mutation before preflight, authentic `Lw` commit or
precise rejection, complete Rust gates, and the highest honestly reached boot
checkpoint.

Private-ROM authority is not granted by provisioning. The future supervisor
seed may authorize bounded read, digest, structural validation, and no-window
execution of `/home/don/fn64/roms/test.z64`. The input must never be modified,
copied, moved, renamed, staged, committed, dumped, or packaged.

Dependencies: accepted partial product commits `dcb9f1bf` and `2de443f3`,
Machine-owned SP IMEM/knownness, complete aligned `Lw`, and the repeated BOOT-2
frontier at unknown offset zero.

Direct overlap inside this lane is deliberate: any source-backed creation event
may require `sp_imem.rs`, Machine lifecycle/application in `machine.rs`, the
named bootstrap creation point, and bounded probe reporting. No concurrent
product lane owns those paths. Ordinary control flow is deferred because its
complete public-step integration also requires `machine.rs` and bootstrap
GPR-knownness in `machine/cartridge_bootstrap.rs`.

Indirect overlap is future Master capability/context reconciliation after an
accepted candidate. Preferred integration order is product truth first, then
Master capability, lane, queue, evidence, and Context-SHA reconciliation.

Stop without fabrication when evidence cannot distinguish competing owners,
or when progress requires proprietary firmware, a game-specific hack, broad
PIF/SP-DMA emulation, speculative routing infrastructure, authority expansion,
destructive Git action, or contradictory accepted law. Retire after accepted
integration, a precise partial result, or an explicit Master stop decision.
The expected next milestone is a causal owner/value finding or a bounded
`UNKNOWN` result—not a predetermined BOOT-3 claim.
