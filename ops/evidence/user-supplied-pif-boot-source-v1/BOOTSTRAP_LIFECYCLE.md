# Bootstrap lifecycle

## Accepted transitions

1. `LIVE_REPO_FACT` Construction starts with firmware `Absent`.
2. `LIVE_REPO_FACT` Installation validates an owned candidate into a local
   immutable value, then performs one optional-field replacement.
3. `LIVE_REPO_FACT` Reset preserves the accepted firmware owner.
4. `LIVE_REPO_FACT` Cartridge-bootstrap staging records the current firmware
   state in `MachineCartridgeBootstrapState`, but creates a fresh all-Unknown
   SP IMEM.
5. `LIVE_REPO_FACT` Repeated bootstrap staging preserves the same firmware
   bytes and again clears stale SP IMEM backing/provenance.

## No-partial-mutation proof

- `LIVE_REPO_FACT` Malformed and unsupported candidates return before
  `Machine.pif_firmware` assignment.
- `RUNTIME_FACT` A complete Machine snapshot proves rejected replacement
  preserves prior accepted firmware, cartridge bytes, CPU/GPR/HI/LO/COP0,
  PC/next-PC/Count, RDRAM, SP DMEM, SP IMEM bytes/provenance, bootstrap state,
  reservation state, and power state.
- `RUNTIME_FACT` Repeated-bootstrap tests prove accepted input alone cannot
  make even SP IMEM offset zero known.
- `LIVE_REPO_FACT` Firmware validation and installation do not invoke
  `Machine::step`, advance Count, stage cadence, or author a checkpoint.

Rollback is not used; all fallible validation precedes mutation.
