# Validation

Candidate validation uses the repository-local pass cache and a TMPDIR whose
path contains `authentic`.

- safe structural no-fallback/redaction checks: passed
- formatting: passed
- warnings-denied clippy: passed
- `fn64-core`: 647 passed
- `fn64-inspection` library: 16 passed
- `fn64_user_cartridge_probe` unit tests: 4 passed
- `boot_probe_cli`: 12 passed
- `user_cartridge_probe_cli`: 4 passed
- machine probe: `result: ok`
- step probe: `result: ok`
- public generated x105/Break proof: passed inside `fn64-core`
- complete forward gate: `forward gate: ok`

No private input was executed.
