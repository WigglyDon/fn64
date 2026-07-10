# Removed Paths

## `docs/context/subsystems/historical-cpp-reference.md`

- Why obsolete: it described no active subsystem boundary and duplicated the
  retirement status, waived comparison prerequisite, Git-history archive, and
  intentionally absent behavior already owned by history and decisions.
- Unique truth destination: the explicit accepted-absence list and restoration
  boundary are preserved under Era 7 in
  `docs/context/PROJECT_HISTORY.md`; D008 and D017 remain in
  `docs/context/DECISION_LOG.md`.
- Link repair: root `README.md`, `docs/INDEX.md`, `CONTEXT_MATRIX.md`,
  `DECISION_LOG.md`, and `host-runtime.md` now point to the surviving owners or
  omit the inactive node.
- Manifest repair: its entry was removed from
  `docs/context/CONTEXT_MANIFEST.json`; the surviving history and decision
  owners remain manifest nodes.
- Validation: `context-sha`, `context-verify`, tracked-reference search, and
  local-link validation.

No placeholder, zero-byte, retired-source directory, or empty migration path
was found or removed.
