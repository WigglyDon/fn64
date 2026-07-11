# User-Supplied PIF Boot Source V1

Context role: active lane coordination memory.
Scope: explicit user-supplied PIF firmware input, Machine ownership/validation, and the smallest source-backed bootstrap mechanism demanded by retained IPL2 state.
Canonical for: this lane's purpose, topology, authority boundary, legal boundary, writable paths, proof, dependencies, overlap, and stop conditions.
Not canonical for: accepted product behavior, private firmware content, candidate acceptance, or canonical integration.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [represented-machine capability](../../rust/PARITY.md), [provenance evidence](../../ops/evidence/sp-imem-bootstrap-provenance-v1/README.md), [machine core](../context/subsystems/machine-core.md), [memory and cartridge](../context/subsystems/memory-and-cartridge.md), [host boundary](../context/subsystems/host-runtime.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, candidate creation, implementation-path selection, context propagation, blocking, integration, frontier change, or retirement.

## Lane identity and state

- Lane ID: `user-supplied-pif-boot-source-v1`
- Goal ID: `fn64-user-supplied-pif-firmware-source-backed-bootstrap-v1`
- Purpose: establish the smallest lawful source-backed Machine mechanism by
  which explicitly user-supplied PIF firmware bytes can provide the retained
  IPL2 content required by the authentic x105 path.
- Supervisor role: User-Supplied PIF Boot Source Supervisor GPT
- Worker Codex worktree:
  `/home/don/fn64-worktrees/user-supplied-pif-boot-source-v1`
- Branch: `worker/user-supplied-pif-boot-source-v1`
- Accepted base source: canonical `main` after the final Master provisioning
  state is integrated and pushed; the literal SHA is owned by the provisioning
  manifest and first executable packet.
- Governing Context-SHA: the exact committed post-provisioning value owned by
  the Master provisioning report and first executable packet. The lane page is
  itself a context node and therefore cannot embed its own digest.
- Status: **REGISTERED — MASTER PROVISIONING PENDING**
- Provisioning state: pending Master verification
- Provisioning exception: `NONE`
- Launch state: no command is executable until Master provisioning succeeds and
  Master GPT issues the matching supervisor seed.

## Exact writable scope

- `rust/crates/fn64-core/src/lib.rs`
- `rust/crates/fn64-core/src/machine.rs`
- `rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`
- `rust/crates/fn64-core/src/sp_imem.rs`
- `rust/crates/fn64-core/src/pif_firmware.rs` only when a new narrow immutable
  firmware owner is source-clear
- `rust/crates/fn64-inspection/src/boot_probe.rs`
- `rust/crates/fn64-inspection/src/bin/fn64_boot_probe.rs`
- `rust/crates/fn64-inspection/tests/boot_probe_cli.rs`
- tests embedded in the listed source files
- `ops/evidence/user-supplied-pif-boot-source-v1/**`
- external lane logs and artifacts

Forbidden paths include `docs/**`, `tools/fleet/**`, `ops/fleet/**`,
`rust/PARITY.md`, `rust/README.md`, `rust/verify-forward`, every Cargo manifest
and lockfile, another lane's evidence, user-local ROM or firmware files, ignored
user content, Worker topology, and canonical `main`. A later literal Master
amendment is required to widen this set.

## Product authority and implementation decision

`USER_DECISION`: fn64 may accept explicitly user-supplied PIF firmware bytes.
The host may own only a literal path, opening/reading it, reporting read failure,
and transferring owned bytes. The Machine must own accepted bytes,
validation/classification, supported or unsupported variant state when needed,
reset/bootstrap lifecycle, SP IMEM production, byte provenance, execution or
source-backed materialization policy, rejection, and resulting state.

The lane must select from evidence rather than assume:

- `SOURCE_BACKED_BOOT_STATE_MATERIALIZATION`;
- `MINIMAL_IPL1_IPL2_EXECUTION`;
- `INPUT_BOUNDARY_ONLY_PARTIAL`; or
- `EVIDENCE_ONLY_PARTIAL`.

One committed `Lw` would advance the trace but does not itself earn BOOT-3.
Full PIF, SI, DMA, interrupt, bus, or memory-map architecture is forbidden
unless the current causal path proves it necessary.

## Legal and private-input boundary

fn64 does not ship, download, search for, reconstruct, encode, compress, or
provide acquisition/dumping instructions for PIF ROM, IPL1, or IPL2 content.
No firmware bytes, words, copied disassembly, derived table, or normalized
private asset may enter source, tests, logs, evidence, commits, or artifacts.
Synthetic tests use generated non-firmware patterns.

No title, filename, cartridge ID, region string, firmware hash, or full-ROM
digest may select game behavior. A digest may identify bounded evidence only.
There is no hidden default path, automatic filesystem search, bundled fallback,
network download, global firmware singleton, plugin, compatibility database,
game profile, device registry, generic bus, generalized memory map, or second
boot probe.

Provisioning grants no private-ROM or private-PIF read authority. A future
supervisor packet may authorize one explicit local path for bounded no-window
verification. The input must never be modified, copied, moved, renamed, staged,
committed, dumped, or packaged. Absence of a real firmware file may produce a
truthful input-boundary partial.

## Proof, dependency, overlap, and closure

Required proof covers literal-path parsing/failure, owned-byte transfer,
Machine ownership and validation, absent/unsupported input, lifecycle,
synthetic-only tests, complete consumed SP IMEM range, provenance, no partial
mutation, full Rust gates, forbidden-content scans, and the highest honestly
reached checkpoint. Runtime trace advancement is required only when an
authorized private firmware input is actually available.

Dependency: evidence-only candidate `8db1b57c`, Master reconciliation
`ba8a1be78080387ff8b2cf93c77579854d2181d1`, Machine-owned SP IMEM/knownness,
complete aligned `Lw`, and the unchanged BOOT-2 frontier.

Direct overlap inside this lane is deliberate across `machine.rs`,
`machine/cartridge_bootstrap.rs`, `sp_imem.rs`, public bootstrap results, and
boot-probe plumbing. `ordinary-control-flow-delay-slot-v1` is deferred because
it needs the same Machine/bootstrap integration surfaces. Preferred integration
order is product truth first, then Master capability, lane, queue, evidence,
and Context-SHA reconciliation.

Stop without fabrication when progress requires bundled/reconstructed firmware,
a game-specific selector, probe-authored bytes, unapproved private input, broad
PIF architecture without current pressure, a generic bus/map, security or
legal expansion, destructive Git action, or contradictory accepted law.
Retire after accepted integration, a precise partial, or an explicit Master
stop. The expected next milestone is a lawful source boundary and, only when
available and source-clear, an authentic trace advance to its next frontier.
