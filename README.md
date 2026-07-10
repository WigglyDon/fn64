# fn64

fn64 is a small, headless Nintendo 64 machine core under construction. The
current product source is the tracked Rust workspace under `rust/`.

The former C++ machine, host, proof, and CMake lane has been retired from the
current tree. Git history is its only archive. Unported C++ behavior is
intentionally absent rather than migrated, and no semantic-parity claim is
made.

## Required verification

From the repository root:

```sh
./rust/verify-forward
```

The repository-owned verifier runs, in order:

1. `cargo fmt --check`
2. `cargo clippy --all-targets -- -D warnings`
3. the complete Rust test suite
4. the no-window construction/reset probe
5. the no-window represented `Machine::step` probe

It ends with `forward gate: ok` when every stage passes. It invokes no CMake,
C++ binary, SDL/window/audio runtime, ROM, cartridge boot, or Git mutation.

## Current represented scope

`fn64-core` owns the represented `Machine`, cartridge bytes, RDRAM, SP DMEM,
CPU/COP0 subset, reset state, instruction-fetch classification, and the narrow
public `Machine::step` path.

`fn64-inspection` owns deterministic no-window setup, assertions, formatting,
and process exit:

- `fn64_machine_probe` covers construction/reset only.
- `fn64_step_probe` calls `Machine::step` for eight synthetic cases: committed
  CPU-local success, arithmetic overflow, SYNC, SYSCALL, BREAK, unsupported
  rollback, selected fetch AdEL, and source-clear rejection.

These proofs do not establish a complete N64, cartridge boot, game
compatibility, timing accuracy, or host-runtime support.

Still absent include branch/link/delay-slot execution, load/store execution,
COP0 instruction execution, ERET, LL/SC, interrupt processing, TLB/MMU, a broad
bus or memory map, device/MMIO routing, cartridge execution mapping,
PIF/BIOS bootstrap behavior, and SDL/window/audio runtime.

## Source ownership

- `rust/crates/fn64-core`: emulated machine truth
- `rust/crates/fn64-inspection`: no-window proof shell
- `rust/verify-forward`: required product verification owner
- `docs/context`: current architecture, history, and decisions
- `tools/fleet` and `ops`: repository coordination and evidence infrastructure

The machine core accepts bytes and owns emulated state. Future hosts may own
paths, input, presentation, audio-device plumbing, and event loops, but must not
own machine policy or hidden emulated truth.

## Legal boundary

fn64 does not ship commercial ROMs or proprietary BIOS/PIF blobs. User-provided
ROMs remain ignored local data and are not part of repository truth or routine
evidence. Small generated instruction words and synthetic byte fixtures are
allowed only for explicit reproducible proof.

## Historical context

The retired C++ eras and the direct retirement decision are recorded in
[`docs/context/PROJECT_HISTORY.md`](docs/context/PROJECT_HISTORY.md) and
[`docs/context/subsystems/historical-cpp-reference.md`](docs/context/subsystems/historical-cpp-reference.md).
Historical source remains available through Git history; no checked-in museum
or compatibility wrapper is retained.
