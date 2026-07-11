# SP IMEM knownness

- `LIVE_REPO_FACT` Backing value and architectural knownness are separate
  arrays owned by `SpImem`.
- `LIVE_REPO_FACT` Production provenance currently has one category:
  `Unknown`. A second category, `GeneratedMachineTestStaging`, exists only
  under `cfg(test)` and is reached only through a Machine-owned test seam.
- `LIVE_REPO_FACT` A word is readable only when all four consumed bytes have
  known provenance. Failure reports the first unknown local offset.
- `RUNTIME_FACT` Tests prove default zero backing is unknown, partial
  knownness cannot satisfy a word read, and bootstrap/reset erase generated
  test staging.
- `UNKNOWN` The authentic post-PIF word at SP IMEM offset `0x000` has no
  represented source in current product truth.
- `INFERENCE` Because the authentic trace needs that exact word and no
  source-clear creation fact exists, treating backing zero as known would be
  fabricated machine state. This inference is supported by the live
  byte-provenance policy and the runtime rejection at the exact offset.
- `LIVE_REPO_FACT` No title, cartridge ID, digest, patch table, host write,
  imported emulator state, or proprietary PIF/BIOS byte can establish SP IMEM
  knownness.
