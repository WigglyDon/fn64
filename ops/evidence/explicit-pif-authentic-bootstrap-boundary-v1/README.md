# Explicit PIF material and authentic-bootstrap boundary v1

- `USER_DECISION`: authentic user-cartridge composition requires explicitly
  supplied PIF firmware bytes.
- `LIVE_REPO_FACT`: `PifFirmware` already distinguishes absent, structurally
  accepted explicit, and deliberately selected public-synthetic material.
- `LIVE_REPO_FACT`: `fn64_user_cartridge_probe` accepts one literal optional
  `--pif-rom` path and passes only owned bytes through existing public core APIs.
- `LIVE_REPO_FACT`: absent material stops before execution with
  `PIF_FIRMWARE_REQUIRED_FOR_AUTHENTIC_BOOT`; it never selects public synthetic
  material.
- `RUNTIME_FACT`: generated process tests prove explicit input, unavailable
  input, malformed and unsupported input, redaction, no search, and no fallback.
- `RUNTIME_FACT`: the complete Rust forward gate passes without a private
  cartridge or PIF input.
- `UNAVAILABLE`: no configured local explicit PIF reference existed for this
  pass, so authentic x105, cartridge-entry, BOOT-2, and user-task execution were
  not run.

Compatibility claim: none.

See [source anchors](SOURCE_ANCHORS.md), [material model](THREE_STATE_PIF_MATERIAL.md),
[host/core boundary](HOST_CORE_BOUNDARY.md), [bootstrap lineage](BOOTSTRAP_LINEAGE.md),
[generated proof](GENERATED_PROOF.md), [local-material result](LOCAL_MATERIAL.md),
[legal audit](LEGAL_AND_PRIVATE_INPUT_AUDIT.md), and [validation](VALIDATION.md).
