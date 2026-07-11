# User-supplied PIF boot source v1

- `USER_DECISION` Selected product decision:
  `INPUT_BOUNDARY_ONLY_PRODUCT`.
- `LIVE_REPO_FACT` `fn64_boot_probe` accepts one optional literal
  `--pif-rom <path>` and performs no firmware search or default-path lookup.
- `LIVE_REPO_FACT` The host reads only that explicit path. `Machine` validates
  and owns accepted bytes in a private immutable `PifFirmware` value.
- `LIVE_REPO_FACT` Accepted firmware is preserved across reset and repeated
  cartridge-bootstrap staging.
- `LIVE_REPO_FACT` Acceptance produces no SP IMEM bytes; construction, reset,
  and bootstrap staging keep all SP IMEM provenance `Unknown`.
- `RUNTIME_FACT` Generated tests prove absent, unreadable, malformed,
  unsupported, accepted, reset, repeated-bootstrap, no-search, and no-partial-
  mutation behavior without proprietary content.
- `UNKNOWN` The exact numeric IPL2 source subrange inside a selected raw PIF
  Boot ROM and a supported real-firmware variant classifier remain unearned.

Compatibility claim: none.
