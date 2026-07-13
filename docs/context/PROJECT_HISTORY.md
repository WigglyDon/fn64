# Project History

Context role: canonical architectural history.
Scope: major fn64 eras and direction changes.
Canonical for: chronology, superseded directions, and surviving invariants.
Not canonical for: current state or complete per-instruction provenance.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](CURRENT_STATE.md).
Related evidence: Git history, [boot checkpoint](../boot_spine_checkpoint.md), and [rust/PARITY.md](../../rust/PARITY.md).
Update triggers: a major architectural era begins, ends, or is reinterpreted by stronger evidence.

## Era 1 — repository skeleton (2026-03-16)

- Evidence: `LIVE_REPO_FACT`, commits `e425b35` and `88fae0a`.
- Prior direction: minimal repository identity and Fedora-oriented bootstrap.
- Revised direction: a runnable C++ machine/host path grew from the skeleton.
- Surviving law: Fedora is a workbench, not emulator identity.
- Remaining debt: architecture and proof ownership were not yet explicit.
- Status: superseded foundation.

## Era 2 — C++ machine and host growth (2026-03-16 through 2026-04-14)

- Evidence: `LIVE_REPO_FACT`, commits `4788b77` through `24fe880` and retained `src/` history.
- Direction: C++ owned ROM loading, machine execution, proof/bootstrap code, CLI plumbing, and SDL presentation.
- Trigger for revision: commit/source evidence shows monolith pressure and later boundary work; product meaning cannot be inferred from terse commit subjects alone.
- Surviving law: direct control, one small machine, and visible host plumbing.
- Remaining debt: product, proof, and host truth were coupled.
- Status: donor/reference.

## Era 3 — C++ boundary and no-window proof (2026-06-17 through 2026-07-04)

- Evidence: `LIVE_REPO_FACT`, commits `36dbee9` through `e66ad6e`.
- Revised direction: separate core, proof, CLI inspection/step probe, and SDL host; narrow public execution to `Machine::step`; harden no-ghost behavior; add explicit boot-adjacent boundaries.
- Trigger evidence: source and commits name core/proof/host separation, no-SDL inspection, step-only execution, and the boot-spine checkpoint.
- Surviving law: proof is an instrument, runtime smoke is not boot, machine truth stays machine-owned.
- Remaining debt at the time: C++ remained both historical implementation and comparison source.
- Status: retired source; preserved in Git history.

## Era 4 — Rust parallel machine construction (chronology partially unavailable)

- Evidence: `LIVE_REPO_FACT`, the complete Rust tree first enters Git in commit `8034b50`; older revisions of `rust/PARITY.md` in Git describe the preceding incremental passes as `WORKER_CLAIM` records.
- Direction: reconstruct represented machine ownership in Rust through small source-clear seams, culminating in public represented `Machine::step` and two no-window probes.
- Trigger evidence: the adopted source and tests prove the resulting ownership; Git does not preserve individual pre-adoption Rust seam commits.
- Surviving law: one owner per mutation, no generic future dispatcher, no host policy in the core.
- Remaining debt: historical per-seam authorship and timing before adoption are `UNKNOWN` in repository history.
- Status: superseded development method; resulting tree is current product truth.

## Era 5 — tracked Rust product and forward gate (2026-07-09)

- Evidence: `LIVE_REPO_FACT`, commits `8034b50` and `df0551f`.
- Revised direction: track the Rust workspace as product truth; make `rust/verify-forward` the default required lane; retain C++ checks only as optional frozen reference.
- Surviving law: deterministic no-window verification and narrow capability claims.
- Remaining debt at the time: C++ remained physically present.
- Status: current Rust product foundation; its transitional C++ retention policy is superseded by Era 7.

## Era 6 — repository-native project context and fleet infrastructure (2026-07-10)

- Evidence: `USER_DECISION`, packet `fn64-master-infrastructure-v1-2026-07-10-001`.
- Revised direction: suspend seam-090 deletion, make authority/context/packet/evidence state discoverable in the repository, and add small non-destructive fleet instruments.
- Surviving law: infrastructure must reduce cognitive load and must not become a second product.
- Remaining debt: context and tools require continued rent-paying review; no product feature is authorized by this era.
- Status: current infrastructure pass.

## Era 7 — direct C++ lane retirement (2026-07-10)

- Evidence: `USER_DECISION`, packet `fn64-master-remove-frozen-cpp-lane-v1-2026-07-10-001`, plus the resulting tracked-tree deletion.
- Prior direction: keep C++ as optional frozen reference until another inventory/deletion-readiness pass completed.
- Trigger: the product chose the more fundamental end state—one current implementation and an honestly smaller Rust machine—without requiring semantic parity.
- Revised direction: remove C++ machine, host, proof, CMake, and C++-only launch source from the current tree; use Git history as the only archive.
- Surviving law: one owner per fact, machine/host separation, explicit absence, lawful synthetic proof, and no compatibility claim from green gates.
- Accepted loss: unported C++ instruction, host, SDL, cartridge/bootstrap, device, CLI, and proof behavior is intentionally absent rather than migrated.
- Remaining debt: Rust is incomplete and headless; future behavior must be earned independently under current architecture.
- Status: current source-ownership era.

The retirement decision intentionally waived semantic comparison and inventory
prerequisites. Git history is the only retired-source archive. In particular,
unported instruction families, cartridge/bootstrap experiments, device and DMA
behavior, CLI output, window/event-loop plumbing, and proof coverage are absent
from the current product. Green Rust tests do not imply that those historical
behaviors were carried forward. Restoring the retired implementation as a
product or reference lane requires a new explicit product decision.

## Era 8 — Rust-purity consolidation and authentic BOOT-2 (2026-07-10)

- Evidence: `USER_DECISION`, packet
  `fn64-master-integrate-boot2-and-rust-purity-v1-2026-07-10-001`;
  `LIVE_REPO_FACT`, worker commits `6f189716`, `8e5efc8e`, and `9cc16142`;
  `RUNTIME_FACT`, the bounded private no-window BOOT-2 trace.
- Prior direction: current capability remained buried inside a transition
  transcript while no authentic cartridge-derived instruction had a complete
  represented effect.
- Revised direction: consolidate current capability under one ledger, preserve
  C++ chronology only in history/decisions/Git, and accept one authentic
  cartridge-derived `SpecialAdd` commit as BOOT-2.
- Repair lesson: concrete zero storage is not architectural knownness. The
  first instruction became complete only after r29 and r0 gained source-backed
  known state, all other unstaged PIF GPRs stayed unknown, and rejection was
  proved pre-mutation.
- Exact boundary: BOOT-3 was not reached. The next frontier is `Lw` at
  `0xA4000044` targeting CPU address `0xA4001000`; SP IMEM storage/routing and
  complete aligned-load semantics remain absent.
- Surviving law: Machine owns bytes and state; the inspection shell owns paths
  and reporting; no game-specific patch, direct-entry bypass, generic bus, or
  compatibility claim was introduced.
- Status: current Rust product/capability era.

## Era 9 — SP IMEM and aligned-Lw partial frontier (2026-07-11)

- Evidence: `USER_DECISION`, packet
  `fn64-master-integrate-sp-imem-lw-partial-and-provision-wave-2-2026-07-11-001`;
  `LIVE_REPO_FACT`, worker commit `dcb9f1bf`; `RUNTIME_FACT`, the repeated
  bounded private no-window trace.
- Prior boundary: the authentic `Lw` frontier lacked SP IMEM representation,
  data routing, and aligned-load semantics.
- Accepted increment: each Machine owns 4 KiB of SP IMEM with separate backing
  bytes and knownness, and one complete aligned `Lw` rule covers direct RDRAM
  plus known SP IMEM with exception, lineage, cadence, and rollback proof.
- Honest partial result: the authentic `Lw` still does not commit. Offset zero
  remains unknown, so BOOT-2 remains the highest checkpoint.
- Next source question: the Machine-owned creation event for SP IMEM bytes
  `0x000..0x003` is `UNKNOWN`; history does not relabel it as reset, PIF, DMA,
  transfer, or firmware behavior without evidence.
- Surviving law: concrete storage value is distinct from represented
  architectural knowledge, and a truthful partial machine increment may land
  without a higher boot claim.
- Status: accepted current capability; the provenance question is resolved by
  Era 10 without a product-behavior change.

## Era 10 — retained IPL2 provenance boundary (2026-07-11)

- Evidence: `LIVE_REPO_FACT`, evidence-only worker commit `8db1b57c`;
  `INFERENCE`, pinned source reconstruction plus independent corroboration;
  `USER_DECISION`, explicit user-supplied PIF input authority.
- Causal finding: IPL1 places proprietary IPL2 content in SP IMEM, CPU control
  enters IPL2 there, and IPL2 stages cartridge IPL3 before entering it at
  `0xA4000040`. The observed x105 prelude consumes retained SP IMEM
  `[0x000, 0x020)` and initially mutates `[0x000, 0x02c)`.
- Product boundary: external observability does not authorize embedding code,
  constants, tables, or a firmware-derived profile. No Machine behavior or
  checkpoint changed; current bytes remain `Unknown` and BOOT-2 remains highest.
- Revised direction: fn64 may accept an explicit user-supplied PIF firmware
  file. Host authority ends at path/read/owned-byte transfer; the Machine owns
  validation, lifecycle, supported/unsupported state, SP IMEM production, and
  provenance.
- Remaining decision: source inspection must choose between source-backed state
  materialization, minimal firmware execution, an input-boundary partial, or an
  evidence-only partial. Broad PIF architecture is not preselected.
- Status: evidence boundary accepted; product implementation follows in a
  separately provisioned lane.

## Era 11 — explicit user-supplied PIF input boundary (2026-07-11)

- Evidence: `LIVE_REPO_FACT`, Worker commits `7d0fd689`, `63aea796`, and
  `1fa8aa17`; `USER_DECISION`, accepted classification
  `INPUT_BOUNDARY_ONLY_PRODUCT`; `RUNTIME_FACT`, focused and complete Rust gates
  plus the unchanged private no-firmware BOOT-2 regression.
- Accepted increment: the existing no-window boot probe accepts one optional
  literal `--pif-rom` path, performs no search or fallback, and transfers owned
  bytes into Machine-owned structural validation.
- Machine boundary: one 1,984-byte raw-Boot-ROM-shaped candidate is accepted
  structurally, a 2,048-byte full-map shape is explicitly unsupported, and
  other lengths are malformed. Accepted immutable bytes persist across reset
  and repeated bootstrap; rejected replacement has no partial mutation.
- Honest limitation: structural shape is not authenticity, firmware revision,
  executability, or compatibility. Accepted bytes execute nothing, produce no
  SP IMEM state, and do not advance the authentic trace beyond BOOT-2.
- Next evidence pressure: identify the exact numeric and variant-qualified
  mapping from accepted input bytes to retained IPL2 content before selecting
  source-backed materialization or minimal execution.
- Status: accepted current input-boundary product capability.

## Era 12 — ordinary control flow and profile-specific IPL2 mapping (2026-07-12)

- Evidence: `LIVE_REPO_FACT`, control-flow commits `60cfc832`, `e46816b9`, and
  `01b06e5a`; `RUNTIME_FACT`, focused tests, direct step probe, and full Rust
  gate; `EXTERNAL_TECHNICAL_EVIDENCE`, mapping commit `2ee4b3c7` and pinned
  public-source anchors.
- Accepted product increment: `BEQ`, `BNE`, `J`, `JAL`, `JR`, and `JALR`
  execute through `Machine::step` with one CPU-owned delay slot, explicit link
  and Count rules, complete branch-in-slot rollback, and EPC/BD exception
  lineage. A region-crossing shared-helper test proves J/JAL use PC+4 high
  bits.
- Accepted evidence increment: pinned NTSC raw `[0x0d4,0x71c)` maps to SP
  IMEM `[0x000,0x648)`; pinned PAL and MPAL raw `[0x0d4,0x720)` map to
  `[0x000,0x64c)`. Structural input shape cannot select a profile, and
  unexamined physical PIF revisions remain unknown.
- Honest limitation: accepted firmware still produces no SP IMEM state. A
  profile-specific copy is only one IPL1 effect and does not establish the
  complete IPL2 handoff; minimal execution remains unearned. BOOT-2 is
  unchanged.
- Status: both independent Wave 3 candidates accepted and integrated without
  rewriting Worker history.

## Era 13 — explicit profiled IPL2 copy materialization (2026-07-12)

- Evidence: `LIVE_REPO_FACT`, eight Worker commits from `0a095487` through
  `a2a8ca51`; `RUNTIME_FACT`, focused tests, patch-tree reproduction, artifact
  verification, clean-checkout proof, and the complete Rust gate;
  `USER_DECISION`, accepted classification `PROFILED COPY MATERIALIZATION
  PRODUCT`.
- Accepted product increment: inspection owns exact `--pif-profile` spellings,
  while Machine owns distinct NTSC/PAL/MPAL meanings and atomically copies the
  complete selected firmware slice at `stage_cartridge_bootstrap`. Copied bytes
  become known with source-offset/profile provenance; untouched bytes remain
  `Unknown`.
- Lifecycle: firmware and profile installation remain independent and
  non-materializing. Both persist across reset; repeated bootstrap constructs a
  replacement image, including stale PAL/MPAL-tail clearing when NTSC is later
  selected.
- Proof boundary: generated bytes prove exact ranges and one source-backed
  `Machine::step` `Lw`. No private PIF or cartridge ROM was read, no PIF
  execution or complete handoff was represented, and BOOT-2 remains highest.
- Workflow record: the raw report sealed in the artifact passed line-integrity
  checks, while the chat-delivered packet was terminal-wrapped and
  noncompliant. This is a `WORKFLOW_DELIVERY_DEFECT`, not a product semantic
  defect.
- Handoff continuation: evidence candidate `c24ab78c` remains unaccepted. Its
  first focused repair must be retried after quota/context recovery and must
  profile-qualify retained IPL2 r31/ra.
- Status: profiled-copy lane accepted and integrated; handoff evidence lane
  remains active and blocked for `FOCUSED_REPAIR_1_RETRY`.

## Unresolved history

The stale local donor clone preserves an earlier two-commit repository shape but
does not establish accepted product authority. The unrecovered prior C++
inventory never became a deletion prerequisite. Private chat chronology and
unavailable earlier bundles are not reconstructed as live truth.
