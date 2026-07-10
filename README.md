# fn64

fn64 is a small, headless Nintendo 64 machine core under construction. The
tracked Cargo workspace under `rust/` is the sole current product
implementation. fn64 is not yet a complete N64 emulator.

The former C++ machine, host, proof, and CMake lane is retired. Git history is
its source archive, and unported behavior is intentionally absent. The durable
reasoning is recorded in [project history](docs/context/PROJECT_HISTORY.md) and
the [decision log](docs/context/DECISION_LOG.md).

## Required verification

From the repository root:

```sh
./rust/verify-forward
```

This is the sole required product gate. It is Rust-only and no-window; the
[build and tooling context](docs/context/subsystems/build-and-tooling.md)
defines its proof boundary.

## Current shape

- `rust/crates/fn64-core` owns represented emulated truth inside each
  `Machine`.
- `rust/crates/fn64-inspection` owns deterministic no-window proof plumbing,
  not machine state.
- `rust/verify-forward` owns the required verification sequence.

The single detailed owner for represented machine capability and explicit
absence is the [capability ledger](rust/PARITY.md). The
[current-state page](docs/context/CURRENT_STATE.md) owns project phase and
authority; subsystem pages own stable architecture boundaries.

Current proof reaches BOOT-2 only: one authentic private-ROM-derived
`SpecialAdd` commits through `Machine::step` before the explicit `Lw` frontier.
It does not establish bootstrap handoff, cartridge entry, BOOT-3, timing
accuracy, game compatibility, graphics, or a window/audio runtime.

## Ownership boundary

The machine core accepts bytes and owns emulated state. A future host may own
paths, input, presentation, audio-device plumbing, and platform event loops,
but must not own machine policy, platform-clock stepping, or hidden emulated
truth.

## Legal boundary

Commercial ROMs and proprietary BIOS/PIF blobs are not repository content.
User-provided ROMs remain ignored local data and stay outside routine evidence.
Small generated instruction words and synthetic byte fixtures are permitted
only for explicit reproducible proof.

Start repository discovery at [docs/INDEX.md](docs/INDEX.md).
