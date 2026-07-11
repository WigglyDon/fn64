# No search and no default

- `LIVE_REPO_FACT` The only firmware filesystem call is
  `std::fs::read(path)` where `path` comes from the explicit `--pif-rom` value.
- `LIVE_REPO_FACT` No environment variable, current-directory probe, home
  directory, emulator directory, filename list, fallback, or download exists.
- `RUNTIME_FACT` A CLI test places a generated structurally accepted file at a
  tempting `pifdata.bin` name in the process working directory, omits
  `--pif-rom`, and observes `pif_firmware_input: absent`.
- `RUNTIME_FACT` An unreadable explicit generated path fails with that exact
  path and does not try another source.
- `LIVE_REPO_FACT` Successful probe output includes
  `pif_firmware_search: none` and `pif_firmware_default_path: none`.

No private filesystem location was searched while producing this evidence.
