# Profiled PIF IPL2 Copy Materialization V1

Context role: active lane coordination memory.
Scope: explicit-profile materialization of the pinned IPL1 raw-PIF-to-SP-IMEM copy effect.
Canonical for: this lane's topology, authority, writable boundary, dependencies, proof, overlap, stop conditions, and retirement condition.
Not canonical for: accepted product behavior, private firmware identity, candidate acceptance, or canonical integration.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [PIF source mapping](../../ops/evidence/pif-ipl2-source-mapping-v1/README.md), [memory and cartridge](../context/subsystems/memory-and-cartridge.md), [represented-machine capability](../../rust/PARITY.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, implementation decision, candidate creation, validation, blocking, integration, or retirement.

## Lane identity and state

- Lane ID: `pif-ipl2-profiled-copy-materialization-v1`
- Goal ID: `fn64-profiled-user-pif-ipl2-copy-materialization-v1`
- Supervisor role: Profiled PIF IPL2 Copy Materialization Supervisor GPT
- Worker Codex worktree:
  `/home/don/fn64-worktrees/pif-ipl2-profiled-copy-materialization-v1`
- Branch: `worker/pif-ipl2-profiled-copy-materialization-v1`
- Accepted base source: the exact canonical Wave 4 registration commit owned by
  the Master provisioning report and first executable packet.
- Governing Context-SHA: the exact committed Wave 4 registration value owned by
  the Master report and first executable packet. This page cannot embed its own
  digest.
- Status: **ACCEPTED — PROFILED COPY MATERIALIZATION PRODUCT**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Provisioning exception: `NONE`
- Launch state: closed; do not relaunch without a new Master packet.

Accepted candidate:
`a2a8ca51b9a5c4c7b80a8d9f4fc39cf60942239c`, preserving all eight Worker
commits from `0a095487` through `a2a8ca51`. The verified artifact is
`/home/don/Downloads/UPLOAD_ME_fn64_pif_ipl2_profiled_copy_materialization_v1.tar.gz`
with SHA-256
`b0bc3859838bbf09d66d8631d2ae9a2fb3b6c2f95df5d02439b4952a87c344d0`.
Master independently reproduced the patch tree and reran focused plus complete
Rust gates. The artifact's raw report passed line-integrity checks; the
chat-delivered Worker packet was not protocol-compliant and is recorded as a
`WORKFLOW_DELIVERY_DEFECT`, not a product defect. No Worker branch was pushed
and no private input was used.

## Mission and exact profile law

Materialize one explicitly selected pinned IPL1 copy effect from already
accepted user-supplied 1,984-byte raw PIF Boot ROM bytes into Machine-owned SP
IMEM. The supported profiles and exact mappings are:

- `NTSC_PINNED`: raw `[0x0d4,0x71c)` to SP IMEM `[0x000,0x648)`;
- `PAL_PINNED`: raw `[0x0d4,0x720)` to SP IMEM `[0x000,0x64c)`;
- `MPAL_PINNED`: raw `[0x0d4,0x720)` to SP IMEM `[0x000,0x64c)`.

Host authority is one explicit profile token plus the existing literal
`--pif-rom` path/read/owned-byte transfer. Host must not infer a profile.
Machine owns profile meaning, firmware ownership, source slice, complete atomic
destination mutation, per-byte user-PIF provenance, reset/bootstrap lifecycle,
and failure. Bytes outside the copied range remain `Unknown` unless another
represented source owns them.

No private PIF or cartridge input authority is granted. Tests use generated
1,984-byte patterns only. No hidden/default/platform/content/filename/cartridge
or digest-derived profile is allowed.

## Exact writable and forbidden scope

Worker-writable repository paths are exactly:

- `rust/crates/fn64-core/src/lib.rs`
- `rust/crates/fn64-core/src/pif_firmware.rs`
- `rust/crates/fn64-core/src/machine.rs`
- `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`
- `rust/crates/fn64-core/src/sp_imem.rs`
- `rust/crates/fn64-inspection/src/boot_probe.rs`
- `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`
- `rust/crates/fn64-inspection/tests/boot_probe_cli.rs`
- tests embedded in the listed Rust source files
- `ops/evidence/pif-ipl2-profiled-copy-materialization-v1/**`
- external lane logs and artifacts

Every other repository path is forbidden. In particular, the Worker must not
modify CPU/control-flow files, Cargo metadata, `rust/verify-forward`, `docs/**`,
`tools/fleet/**`, `ops/fleet/**`, another lane's evidence, private inputs,
ignored user content, Git topology, or canonical `main`.

## Required proof and non-goals

Generated proof must cover every profile's first/last copied byte, equal source
and destination lengths, no mutation outside the selected destination, profile
absence/unsupported input, malformed firmware, atomic failure, reset, repeated
bootstrap, stale-provenance clearing, per-byte provenance, untouched unknown
bytes, independent Machines, host-buffer independence, explicit CLI profile,
and no-search behavior. A synthetic `Machine::step` test may prove an `Lw`
consumes a generated copied word.

This lane does not execute IPL1 or IPL2, synthesize `t3`, `sp`, `ra`, PIF RAM,
PI/SI state, or CPU handoff, implement PIF commands/DMA/interrupts/bus/memory
map, use proprietary bytes, promise trace advancement, earn BOOT-3, or claim
compatibility.

## Dependency, overlap, integration, and closure

Dependencies: accepted input boundary `1fa8aa17`, accepted source mapping
`2ee4b3c7`, integrated ordinary control flow `01b06e5a`, and the current
Machine/bootstrap/SP-IMEM lifecycle. Direct writable overlap with
`pif-ipl2-handoff-state-mapping-v1`: `NONE`; that lane writes only its own
evidence directory. Indirect overlap is later Master context reconciliation.
Either candidate may be reviewed independently; product truth integrates
before its Master documentation reconciliation.

Stop for profile ambiguity, partial-mutation risk, a need for private firmware,
an unearned device/executor framework, overlapping ownership, destructive Git,
or loss of reviewability. Retire after accepted integration, a precise partial,
or explicit Master stop. Expected next milestone: source-backed profiled SP
IMEM copy production with synthetic proof, still separate from complete IPL2
handoff and BOOT-3.
