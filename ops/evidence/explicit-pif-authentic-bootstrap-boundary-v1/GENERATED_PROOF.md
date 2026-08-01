# Generated proof

All proof inputs are generated public data.

- Core tests distinguish absent, accepted explicit, and explicit public
  synthetic firmware states.
- Profiled-copy tests prove complete byte-exact atomic materialization,
  explicit/synthetic provenance, reset/rebootstrap lifecycle, source range,
  and no mutation for absent or rejected input.
- The public x105 Break proof installs generated raw-Boot-ROM-shaped bytes
  through the explicit byte API, composes the NTSC cold-x105 handoff, executes
  the CPU transformation, selects Mfc0, and commits terminal Break.
- User-cartridge CLI tests prove the host route stops on absent material, reads
  only an explicit PIF path, rejects malformed and unsupported generated files,
  leaks neither input path, ignores tempting current-directory/environment
  material, and never selects public synthetic firmware.
- Boot-probe CLI tests retain explicit-profile materialization, no-search, shape
  validation, and fixed PIF-path redaction.

No private cartridge or firmware material participated.
