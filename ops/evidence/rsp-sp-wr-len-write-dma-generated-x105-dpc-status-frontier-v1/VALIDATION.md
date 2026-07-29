# Validation

Pre-edit public frontier reproduction passed with one selected
`UnsupportedMtc0ControlRegister { register_index: 3 }` rejection and complete
Machine preservation.

The disposable source/destination audit proved 192 concrete non-opaque IMEM
bytes and 24 disjoint in-range RDRAM destination blocks. Product, clean
checkout, canonical, context, fleet, queue, and artifact validation results are
recorded by the final packet after completion.

Focused candidate results:

- exact public core gate: 1 passed, 0 failed;
- `sp_write_length`: 1 passed, 0 failed;
- `rsp_mtc0`: 5 passed, 0 failed;
- `mtc0`: 14 passed, 0 failed;
- inspection all targets: 16 library, 2 user-probe, and 11 CLI tests passed;
- complete no-window step probe: `result: ok`;
- context verifier: 15 checks, 0 errors;
- reconciled candidate Context-SHA:
  `86eee0976cd27608e9240bfc259df3615eaa366792aca7d2c7df3f656cd72b0c`.
