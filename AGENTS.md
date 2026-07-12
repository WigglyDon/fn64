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

## Project packet transport

Emit every chat-delivered project packet in one uninterrupted Markdown fenced
code block tagged `text`. Put the complete payload inside that single block,
including any routing title, `BEGIN PROJECT PACKET`, headers, body, and
`END PROJECT PACKET`; put no prose, citation, heading, note, explanation, or
commentary outside it. Never split one packet across blocks or nest fenced code
blocks. Keep commands, diffs, examples, and literal output as plain text inside
the transport block, and make `END PROJECT PACKET` the final payload line.

The Markdown fence is transport-only, not part of the packet payload. Store a
packet in a `.txt` file or archive member as raw payload without that fence.
Every packet that requests another packet reply must carry this rule forward
through Worker, repair, supervisor-review, and Master handoffs. Unless a later
authorized project instruction explicitly replaces it, `copy-paste-ready`
means exactly this single-block transport.

## Agent discovery

Before work, read [the context index](docs/INDEX.md), then follow every scoped
`AGENTS.md` from this root to the working directory. Current project truth is
owned by [CURRENT_STATE.md](docs/context/CURRENT_STATE.md), accepted decisions
by [DECISION_LOG.md](docs/context/DECISION_LOG.md), and detailed
represented-machine capability by [rust/PARITY.md](rust/PARITY.md).

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
