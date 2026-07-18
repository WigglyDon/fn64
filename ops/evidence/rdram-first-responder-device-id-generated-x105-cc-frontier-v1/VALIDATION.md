# Validation

Candidate validation completed with pass-owned Cargo and temporary paths:

- focused filters: `rdram` 107, `first_responder` 5,
  `rdram_device_id` 4, `device_id` 8, `mi_version` 8, `store_word` 12,
  `address` 46, `ades` 15, `delay_slot` 27, `jal` 3,
  `control_flow` 22, `machine_step` 12, `bootstrap` 39, `cold_x105` 9,
  `generated_x105` 1, `current_control` 1, `rdram_mode` 1,
  `rdram_ref_row` 4, `rdram_delay` 6, `mi_init_mode` 5,
  `mi_init_transfer` 1, `ri_mode` 7, `ri_select` 10, `ri_config` 7, and
  `ri_current_load` 7; all passed with zero failures;
- formatting check: passed;
- clippy: workspace/all targets, warnings denied, passed;
- `fn64-core`: 494 passed, 0 failed;
- `fn64-inspection` library: 16 passed, 0 failed;
- CLI integration: 11 passed, 0 failed;
- no-window machine probe: `result: ok`;
- no-window step probe: 155 stable cases and `result: ok`;
- complete Rust forward gate: `forward gate: ok`;
- context and local-link verifier: 15 checks, 0 errors, `result: ok`;
- fleet fixture suite: 52 passed, `result: ok`;
- integration queue: `integration-queue: ok`;
- pre-commit Context-SHA:
  `79bc946a09dd9ac8dc9c5870522fb8afa726bfb3c5602747f87075bcc94a5125`.

The exact candidate-SHA clean-checkout gate, post-integration canonical gate,
normal push proof, and artifact verification occur after this evidence commit
and are recorded in the sealed operation artifact and final report.
