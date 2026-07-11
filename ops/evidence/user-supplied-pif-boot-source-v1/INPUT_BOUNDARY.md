# Input boundary

- `LIVE_REPO_FACT` CLI option: `--pif-rom <path>` on the existing
  `fn64_boot_probe` executable.
- `LIVE_REPO_FACT` The option is optional. Without it, the Machine firmware
  state is explicitly `Absent` and existing BOOT-2 fail-closed behavior
  remains available.
- `LIVE_REPO_FACT` The binary owns argument parsing, opening the exact literal
  path, reading bytes, and reporting an unreadable-path failure.
- `LIVE_REPO_FACT` The binary passes an owned `Vec<u8>` to
  `Machine::install_pif_firmware`; it does not validate layout or write SP
  IMEM.
- `LIVE_REPO_FACT` Successful output reports only accepted/absent state,
  structural classification, and size. It reports no successful firmware path
  and no firmware byte.
- `LIVE_REPO_FACT` Missing option values exit with usage status 2. Unreadable
  explicit paths exit 1 without consulting another path.
- `LIVE_REPO_FACT` Public additions are the Machine installation/state query,
  validation/state enums, structural size constants, and the optional-firmware
  boot-probe library entry. Existing `run_boot_probe` remains the absent-input
  wrapper.

Host authority ends at owned-byte transfer. Machine authority begins at
validation.
