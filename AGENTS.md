# fn64 Standing Law

Scope: the complete fn64 repository.

fn64 builds the smallest honest Nintendo 64 machine core that can breathe. Rust
is the sole current product implementation. The retired C++ machine, host,
proof, and CMake lane exists only in Git history; it must not be restored as a
product lane without an explicit product decision.

The machine core owns emulated truth. Host code owns file paths, CLI parsing,
windows, input, audio-device plumbing, presentation, and platform event loops.
One fact has one owner. Unsupported, unavailable, and not-yet-earned behavior
must stay explicit. A `Machine` instance is the unit of emulated truth; hidden
global state, host-owned emulator policy, and platform clocks inside stepping
are forbidden.

Prefer concrete code, no-window proof, plain artifacts, and inspectable failure.
Do not turn proof code into the runtime path. Do not claim cartridge boot, game
compatibility, or host-runtime support from synthetic bytes or green tests.
Commercial ROMs and proprietary BIOS/PIF blobs must never be read for routine
work, committed, copied, or packaged.

## Agent discovery

Before work, read [the context index](docs/INDEX.md), then follow every scoped
`AGENTS.md` from this root to the working directory. Current project truth is
owned by [CURRENT_STATE.md](docs/context/CURRENT_STATE.md), accepted decisions
by [DECISION_LOG.md](docs/context/DECISION_LOG.md), and detailed Rust migration
truth by [rust/PARITY.md](rust/PARITY.md).

Workers do not push, deploy, mutate canonical `main`, discard unknown work, or
expand product authority. Stop for contradictory accepted law, data-loss risk,
security risk, or a product/authority decision—not for routine Git mechanics or
an ordinary compile failure.

Master Codex alone [provisions and verifies worker worktrees and branches](docs/process/WORKTREE_PROVISIONING.md)
and integrates worker commits. Worker Codex may not manage worktrees or
branches and must stop without modification when its repository root, assigned
branch, starting base, cleanliness, index, or Context-SHA differs from its
Master-provisioned packet.

Update this file only when standing project law or root discovery changes.
