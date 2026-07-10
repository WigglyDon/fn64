# Current Project State

Context role: canonical current-state owner.
Scope: fn64 at the accepted repository head.
Canonical for: current phase, authority, verification, lanes, blockers, and capability boundaries.
Not canonical for: detailed machine parity, full history, or individual test evidence.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: this document.
Related evidence: [rust/PARITY.md](../../rust/PARITY.md) and [lane registry](../lanes/lane-registry.md).
Update triggers: accepted authority, capability, verification, lane, or retirement state changes.

## Current phase and authority

- `LIVE_REPO_FACT`: fn64 is one Git repository with a tracked Cargo workspace under `rust/` and a retained CMake/C++ lane at repository root.
- `LIVE_REPO_FACT`: Rust is the forward product implementation; `./rust/verify-forward` is the sole default required forward-product gate.
- `LIVE_REPO_FACT`: C++ source, CMake targets, no-window proofs, and an SDL host remain present as optional frozen reference truth.
- `USER_DECISION`: C++ deletion is inactive. The drafted seam-090 deletion request is superseded by the project-context and fleet-infrastructure pass.
- `USER_DECISION`: Master Codex alone provisions and verifies Worker Codex branches/worktrees and integrates worker commits; supervisors supply semantic packets only after provisioning.
- `WORKER_CLAIM`: a complete C++ truth inventory was previously reported, but no committed inventory evidence exists at the starting head; it is not promoted here.
- `USER_DECISION`: this pass may establish infrastructure/context authority but may not change represented machine behavior, add an emulator feature, or delete C++.

## Forward machine truth

- `LIVE_REPO_FACT`: `fn64-core` owns represented `Machine`, cartridge, RDRAM, SP DMEM, CPU, COP0 subset, instruction-fetch classification, reset, and machine-state staging/inspection surfaces.
- `LIVE_REPO_FACT`: public `Machine::step` is the sole represented execution entrance. Its current selector commits the sealed straight-line CPU-local helper families and reports no-effect, stopped, unsupported, arithmetic-overflow, fetch-address-error, and source-clear rejection outcomes.
- `LIVE_REPO_FACT`: branch, jump/link, load/store, COP0 instruction, ERET, LL/SC, interrupt, device/MMIO, and cartridge/bootstrap-related code exists in the repository's historical C++ lane and in various Rust readiness/state seams, but those categories are not selected by the current public Rust `Machine::step` spine.
- `LIVE_REPO_FACT`: `fn64_machine_probe` is deterministic construction/reset inspection only. `fn64_step_probe` calls `Machine::step` over eight synthetic cases: committed CPU-local success, arithmetic overflow, SYNC, SYSCALL, BREAK, unsupported rollback, selected fetch AdEL, and source-clear rejection.
- `LIVE_REPO_FACT`: Rust has no SDL/window/audio host or game runtime. No current Rust path claims cartridge execution, PIF/BIOS boot, or game compatibility.

The detailed, source-linked machine and migration ledger remains
[rust/PARITY.md](../../rust/PARITY.md). Its historical seam rows must be read as
dated records; its current policy sections take precedence over superseded rows.

## Verification and lanes

- Required gate: `./rust/verify-forward`.
- Optional reference checks: CMake configure/build plus C++ `fn64_selftest` and `fn64_step_probe`, only when an explicit reference task requires them.
- Active worker lanes: none found from live repository evidence at infrastructure bootstrap.
- Deferred lane: `cpp-reference-truth-reconstruction-v1` is `DEFERRED — NOT PROVISIONED`; it has no branch, worktree, launch command, candidate, or executed inventory work.
- Planned but inactive: any future C++ deletion must receive a new explicit product packet after live inventory truth is accepted.
- Parked donor/reference: a separate stale local clone exists outside the canonical repository; see the [lane registry](../lanes/lane-registry.md).

## Blockers and known unknowns

- `USER_DECISION`: no emulator feature or C++ deletion may start automatically after this infrastructure pass.
- `UNKNOWN`: the prior external C++ inventory's exact artifact and acceptance lineage are unavailable in committed repository truth.
- `UNKNOWN`: no current active supervisor/worker session can be inferred from Git worktrees alone.
- `LIVE_REPO_FACT`: the canceled `/tmp` C++ inventory topology and reserved future persistent topology were absent at the workflow-amendment preflight; residue classification is `NO_RESIDUE_FOUND`.
- `LIVE_REPO_FACT`: ignored user-local artifact directories exist outside repository truth; their contents are not inspected, executed, committed, or packaged.
