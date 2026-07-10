# Validation

Superseded candidate: `6f189716ad401cbc9996ad57a23cef4a7c3da196`.

Repair candidate: `HEAD`, the commit containing this file. The exact immutable
repair SHA and complete post-commit command logs are in the repair archive
`UPLOAD_ME_fn64_real_cartridge_boot_spine_v1_repair1.tar.gz`.

The following commands passed against the candidate tree:

```sh
cd rust
PATH=/home/don/.cargo/bin:$PATH cargo test -p fn64-core machine_bootstrap_reset_state_lineage -- --nocapture
PATH=/home/don/.cargo/bin:$PATH cargo test -p fn64-core machine_bootstrap_unknown_gpr_source_rejection -- --nocapture
PATH=/home/don/.cargo/bin:$PATH cargo test -p fn64-core machine_bootstrap_known_special_add_commit -- --nocapture
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
earning BOOT-2. The first `SpecialAdd` changed destination GPR 9 from
zero/unknown to `0xFFFFFFFFA4001FF0`/known; the next known-base `Lw` stopped at
effective CPU address `0xA4001000`. Its exact invocation, metadata, complete
output, and digest are external only.

External log names include `targeted_reset_state_lineage.log`,
`targeted_unknown_source_rejection.log`, `targeted_known_special_add.log`,
`targeted_bootstrap_staging.log`, `targeted_cartridge_normalization.log`,
`targeted_boot_probe.log`, `machine_step.log`, `rustfmt.log`, `clippy.log`,
`verify_forward.log`, and `private_boot_probe.log`.

Compatibility claim: none.
