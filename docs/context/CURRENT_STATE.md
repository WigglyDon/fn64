# Current Project State

Context role: canonical current-state owner.
Scope: fn64 at the accepted repository head.
Canonical for: current phase, authority, verification, lanes, blockers, and capability boundaries.
Not canonical for: detailed machine behavior, full history, or individual test evidence.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: this document.
Related evidence: [rust/PARITY.md](../../rust/PARITY.md), [project history](PROJECT_HISTORY.md), and [lane registry](../lanes/lane-registry.md).
Update triggers: accepted authority, capability, verification, lane, or retirement state changes.

## Current phase and authority

- `LIVE_REPO_FACT`: fn64 is one Git repository whose sole current product implementation is the tracked Cargo workspace under `rust/`.
- `LIVE_REPO_FACT`: no active C/C++ source, header, CMake build owner, C++ proof binary, C++ host shell, or C++-only launch script remains in the current tree.
- `USER_DECISION`: the former C++ machine, host, proof, and build lane is retired without semantic-equivalence or inventory prerequisites. Git history is its only archive; unported behavior is intentionally absent.
- `LIVE_REPO_FACT`: `./rust/verify-forward` is the sole required product gate and has no CMake, C++, ROM, SDL/window/audio, or Git dependency.
- `USER_DECISION`: Master Codex alone provisions Worker topology and integrates Worker commits.

## Forward machine truth

- `LIVE_REPO_FACT`: `fn64-core` owns the represented machine and public
  `Machine::step`; `fn64-inspection` owns deterministic no-window proof
  plumbing only.
- `LIVE_REPO_FACT`: represented execution is bounded and the product remains
  incomplete and headless. It has no cartridge or PIF/BIOS boot claim, game
  compatibility claim, or window/audio runtime.

The single detailed owner for represented capability and explicit absence is
the [represented-machine capability ledger](../../rust/PARITY.md). Stable
architecture boundaries live in the active subsystem pages; retirement
chronology lives in [project history](PROJECT_HISTORY.md).

## Verification and lanes

- Required gate: `./rust/verify-forward`.
- `real-cartridge-boot-spine-v1`: provisioned and awaiting its supervisor packet; no Worker implementation has started and no candidate exists.
- `rust-purity-repo-cleanup-v1`: provisioned and awaiting its supervisor packet; no Worker implementation has started and no candidate exists.
- Active C++ lanes: none.
- `cpp-reference-truth-reconstruction-v1`: canceled without provisioning; its reserved branch/worktree remain absent.
- The earlier seam-090 and inventory-first sequences are superseded by the direct Master retirement decision.
- Integration queue: both provisioned lanes are registered with unknown candidate/artifact values; no candidate commit exists.
- No cartridge-boot milestone, compatibility result, or repository-cleanup result has been earned by lane registration or provisioning.

## Blockers and known unknowns

- `LIVE_REPO_FACT`: the current Rust product remains deliberately incomplete and headless.
- `UNKNOWN`: performance, broad hardware compatibility, cartridge boot, game behavior, and host-runtime behavior are unmeasured or unavailable.
- `USER_DECISION`: retired C++ behavior does not define a product backlog. Any future Rust capability requires its own bounded product decision and proof.
- `LIVE_REPO_FACT`: ignored user-local assets remain outside repository truth and routine evidence.
