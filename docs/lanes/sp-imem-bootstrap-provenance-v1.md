# SP IMEM Bootstrap Provenance V1

Context role: retired lane coordination memory.
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
- Status: **PARTIAL — EVIDENCE INTEGRATED; PRODUCT SOURCE UNAVAILABLE**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Provisioning exception: `NONE`
- Launch state: completed Worker evidence pass; no further Worker repair or
  relaunch is authorized by this closure.

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

The lane identified the source-qualified hardware event: IPL1 copies
proprietary IPL2 content into SP IMEM, CPU control enters IPL2 there, IPL2
stages cartridge IPL3 in SP DMEM, and the observed x105 entry consumes retained
SP IMEM `[0x000, 0x020)` before initially mutating `[0x000, 0x02c)`. External
observability did not create product authority or lawful Machine bytes, so no
product implementation was made.

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

Accepted proof is the source-qualified causal record in
`ops/evidence/sp-imem-bootstrap-provenance-v1/`, exact candidate `8db1b57c`,
and verified artifact SHA-256
`032d52c033ead5d44dd0cef370b4a3c67cfaf378e97956e7989bb5dc8198dd47`.
The evidence contains no firmware words, copied assembly, private ROM bytes, or
product-source change. BOOT-2 remains highest and the authentic `Lw` does not
commit.

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

Closure record:

- Candidate: `8db1b57c2fc1447c228df7aa192090eda3c64ee8`
- Parent: `eb1e4d12b193256923aa3fa0b741c1dacf67a17b`
- Result: evidence-only partial; no product behavior change
- Consumed range: `[0x000, 0x020)`
- Initial mutation range: `[0x000, 0x02c)`
- Worker push: none
- Worker worktree/branch: preserved
- Integration: Worker commit preserved unchanged; exact Master reconciliation
  SHA is recorded by the following registration commit and final artifact
- Next decision: explicitly user-supplied PIF firmware is authorized, with
  implementation delegated to a separately provisioned product lane
