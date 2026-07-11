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
- Status: **ACCEPTED — SOURCE-BOUNDARY PRODUCT**
- Provisioning state: completed and integrated through Master authority
- Provisioning exception: `NONE`
- Launch state: Worker execution completed; no further Worker command is active.

## Accepted result

- Complete Worker range: `7d0fd68961c559af546f7be0aa15f10612d3347d`,
  `63aea79687abf30f1d6b7096141c6b00183ce31f`, and
  `1fa8aa1789666a38c8a22661a7b0e829ec241c41`.
- Product classification: `INPUT_BOUNDARY_ONLY_PRODUCT`.
- Artifact: `/tmp/UPLOAD_ME_fn64_user_supplied_pif_boot_source_v1.tar.gz`,
  SHA-256 `b59304f1b97f45d0b23effbe4ec54c1853fdc7be9d6722cf5c745d2192ed0450`.
- Integrated product SHA: `1fa8aa1789666a38c8a22661a7b0e829ec241c41`;
  final canonical reconciliation identity belongs to the Master report.
- Earned truth: one explicit optional `--pif-rom` path, host-owned literal read
  and owned-byte transfer, Machine-owned structural validation and immutable
  input, explicit absent/malformed/unsupported/accepted states, reset and
  repeated-bootstrap persistence, and atomic rejection.
- Explicit absence: accepted firmware executes nothing and produces no known
  SP IMEM byte. No authenticity, revision, compatibility, or BOOT-3 fact was
  earned; BOOT-2 remains highest.
- Private inputs: the authorized private-ROM no-firmware regression passed; no
  private PIF path was authorized, searched, read, hashed, copied, staged,
  committed, or packaged.
- Worker branch push: none. Worker topology remains preserved.
- Exact next blocker: the numeric and variant-qualified mapping from accepted
  PIF Boot ROM bytes to retained IPL2 SP IMEM content.

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

The completed supervisor packet authorized one explicit private-ROM path only
for the no-firmware BOOT-2 regression. It authorized no private PIF path. No
input was modified, copied, moved, renamed, staged, committed, dumped, or
packaged.

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
boot-probe plumbing. The implementation lane is now closed; Product Wave 3
re-audits ordinary-control-flow ownership against the integrated source.

Stop without fabrication when progress requires bundled/reconstructed firmware,
a game-specific selector, probe-authored bytes, unapproved private input, broad
PIF architecture without current pressure, a generic bus/map, security or
legal expansion, destructive Git action, or contradictory accepted law.
Retirement condition is satisfied by accepted Master integration. Any
source-backed materialization or firmware execution requires a separately
provisioned lane after the source-mapping evidence result.
