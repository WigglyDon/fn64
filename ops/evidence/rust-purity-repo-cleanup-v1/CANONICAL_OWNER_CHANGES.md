# Canonical Owner Changes

Lane: `rust-purity-repo-cleanup-v1`

This map was established from tracked repository content before any deletion.
It distinguishes current mutable capability truth, durable history, stable
boundary law, process truth, and lane coordination.

| Fact or competing owners before cleanup | Canonical owner after cleanup | History destination | Inbound-link / manifest consequence | Action |
| --- | --- | --- | --- | --- |
| Detailed represented machine capability was copied across `rust/PARITY.md`, `rust/README.md`, root `README.md`, and `CURRENT_STATE.md`. | `rust/PARITY.md`, redefined as the current represented-machine capability ledger. | The completed transition stays in `PROJECT_HISTORY.md` and superseded/current entries in `DECISION_LOG.md`. | Existing `rust/PARITY.md` links remain stable; summaries become short links. | Rewrite the ledger and collapse the three competing detailed summaries. |
| Current product authority appeared in root law, current state, README pages, and the old ledger. | `AGENTS.md` owns standing law; `CURRENT_STATE.md` owns mutable phase and authority. | Retirement chronology remains in `PROJECT_HISTORY.md`. | Discovery pages name the owner instead of reproducing the transition. | Keep short stable summaries; remove migration-ledger labels. |
| Retired C++ archival truth appeared in `historical-cpp-reference.md`, `PROJECT_HISTORY.md`, `DECISION_LOG.md`, root `README.md`, and the old parity transcript. | `PROJECT_HISTORY.md` owns chronology; `DECISION_LOG.md` owns the retirement and parity-waiver decisions; Git owns retired source. | Same owners; no new history framework. | Remove the subsystem node from the index, matrix, manifest, README, decision links, and host-runtime link. | Preserve the unique accepted-absence list in `PROJECT_HISTORY.md`, then delete the redundant subsystem page. |
| Required verification detail appeared in README pages, build context, current state, and the old ledger. | `rust/verify-forward` owns executable order; `build-and-tooling.md` owns verification boundaries. | Gate-promotion chronology remains in `PROJECT_HISTORY.md` and D007. | README pages point to the executable and context owner. | Remove transition-gate comparisons and copied stage detail from the capability ledger. |
| Machine/host authority appeared in standing law, subsystem pages, and migration tables. | `AGENTS.md` owns standing law; active subsystem pages specialize stable boundaries. | Earlier C++ manifestations remain historical only. | Active subsystem pages continue to link current state and the capability ledger. | Keep the boundary pages; remove C++-comparison and parity-ledger wording where it implies current comparison. |
| Worker topology and lane state appear in process pages, lane pages, current state, and queue data. | `WORKTREE_PROVISIONING.md` owns topology process; lane registry and queue remain Master-owned coordination. | Canceled-lane residue remains explicit historical evidence. | No lane or queue coordination file changes in this Worker lane. | Keep current protocol fixtures; request Master coordination updates after candidate review. |
| Cargo workspace location is described by Rust docs and lane policy. | `rust/Cargo.toml` remains the physical owner under `rust/`; `rust/README.md` documents the current location. | D019 records why movement is deferred. | No manifest, wrapper, symlink, or forwarding metadata change. | Recommend `KEEP_RUST_WORKSPACE_UNDER_RUST`. |

## Selected dispositions

- `rust/PARITY.md`: `KEEP_AND_REDEFINE`.
- Detailed capability owner after cleanup: `rust/PARITY.md`.
- Cargo workspace recommendation: `KEEP_RUST_WORKSPACE_UNDER_RUST`.
- Fleet verifier behavior: unchanged; no focused fixture is applicable.

The legacy `rust/PARITY.md` path remains because it is already the recursive
discovery target and capability-link destination. Its name is historical, but
its title and contents will no longer present parity as current work. Keeping
the path avoids creating a second framework or churning valid links while
still deleting the transition transcript.
