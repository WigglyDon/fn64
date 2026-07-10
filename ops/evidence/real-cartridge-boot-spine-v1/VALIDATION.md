# Validation

Candidate: `HEAD`, the commit containing this file. The exact immutable tested
SHA and complete command logs are in the external archive.

The following commands passed against the candidate tree:

```sh
cd rust
PATH=/home/don/.cargo/bin:$PATH cargo test machine_cartridge_bootstrap -- --nocapture
PATH=/home/don/.cargo/bin:$PATH cargo test boot_probe -- --nocapture
PATH=/home/don/.cargo/bin:$PATH cargo test normalizes_supported_source_layouts_and_loads_cartridge_bytes -- --nocapture
PATH=/home/don/.cargo/bin:$PATH cargo test rejects_unsupported_or_malformed_rom_inputs -- --nocapture
PATH=/home/don/.cargo/bin:$PATH cargo test machine_step -- --nocapture
PATH=/home/don/.cargo/bin:$PATH cargo fmt --all -- --check
PATH=/home/don/.cargo/bin:$PATH cargo clippy --all-targets -- -D warnings
cd ..
PATH=/home/don/.cargo/bin:$PATH ./rust/verify-forward
```

The no-window probe was also run directly against the deterministically
selected private runtime artifact with `--max-steps 100000`. It exited zero at
the explicit `Lw` frontier after two attempted steps and one committed step,
earning BOOT-2. Its exact invocation, metadata, complete output, and digest are
external only.

External log names include `targeted_machine_cartridge_bootstrap.log`,
`targeted_boot_probe.log`, `cartridge_normalization.log`,
`cartridge_rejection.log`, `machine_step.log`, `rustfmt.log`, `clippy.log`,
`verify_forward.log`, and `private_boot_probe.log`.

Compatibility claim: none.
