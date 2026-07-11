# Known frontier

- `RUNTIME_FACT` Existing no-firmware execution address: `0xA4000044`.
- `RUNTIME_FACT` Identity: `Lw`; effective CPU address: `0xA4001000`.
- `RUNTIME_FACT` Target: SP IMEM offset zero.
- `RUNTIME_FACT` Current source byte: `Unknown`; rejection occurs before
  mutation with BOOT-2 cadence unchanged.
- `LIVE_REPO_FACT` Optional accepted firmware changes only Machine input state;
  it does not alter this frontier.
- `UNKNOWN` Exact raw-firmware source range needed to produce retained IPL2
  destination `[0x000,0x020)`.
- `UNKNOWN` Authentic result with a later explicitly authorized private PIF
  input.

Exact next product requirement: source-clear firmware-to-SP-IMEM production or
a separately justified minimal execution decision, followed by one literal
path authorization for bounded no-window runtime validation.
