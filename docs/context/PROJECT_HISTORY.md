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

## Unresolved history

The stale local donor clone preserves an earlier two-commit repository shape but
does not establish accepted product authority. The unrecovered prior C++
inventory never became a deletion prerequisite. Private chat chronology and
unavailable earlier bundles are not reconstructed as live truth.
