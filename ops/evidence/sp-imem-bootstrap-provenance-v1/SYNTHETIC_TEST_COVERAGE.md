# Synthetic test coverage

- `WORKER_CLAIM` Product code changed: no. No artificial product test was
  added to make unavailable firmware content appear represented.
- `RUNTIME_FACT` Existing SP IMEM tests prove exact capacity, unknown-zero
  construction, four-byte knownness, first-unknown reporting, bounds, and N64
  big-endian assembly using generated values only.
- `RUNTIME_FACT` Existing Machine lifecycle tests prove construction/reset and
  repeated cartridge-bootstrap staging erase generated test values and restore
  `Unknown` provenance.
- `RUNTIME_FACT` Existing `Lw` tests prove direct RDRAM and generated-known SP
  IMEM commits, sign extension, alias and zero-register behavior, destination
  lineage, cadence, data-AdEL, and no-partial-mutation rejection.
- `RUNTIME_FACT` Existing authentic-frontier-shaped tests prove the exact
  offset-zero rejection without embedding a private ROM or firmware word.
- `RUNTIME_FACT` Existing boot-probe library and CLI tests prove deterministic
  bounded reporting and no-window exit policy.
- `WORKER_CLAIM` No fixture contains the private input's identity, bootcode
  digest, title, cartridge ID, instruction sequence, SP IMEM dump, or firmware
  data.

The external artifact owns exact command logs and test counts for the immutable
candidate.
