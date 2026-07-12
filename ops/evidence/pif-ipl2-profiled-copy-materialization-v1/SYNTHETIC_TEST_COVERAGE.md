# Synthetic Test Coverage

`USER_DECISION`: all PIF-shaped fixtures are generated 1,984-byte patterns. No proprietary byte,
word, digest, disassembly, or expected firmware table is present.

`LIVE_REPO_FACT`: core tests cover:

- exact semantic names, source/destination endpoints, and equal lengths for all
  three profiles, with no CLI spellings in core;
- every copied byte and its exact source-offset provenance;
- every untouched byte remaining Unknown;
- no firmware/no profile, accepted firmware/no profile, profile/no firmware,
  both install orders, direct order equivalence, repeated staging, reset, and
  restaging;
- malformed and unsupported input with full-state replacement rollback;
- host-buffer independence and failed-replacement atomicity;
- reset, repeated bootstrap, PAL-to-NTSC and MPAL-to-NTSC stale-tail clearing,
  and independent Machines;
- bootstrap failure preserving complete represented state; and
- a generated copied word consumed by `Lw` through public `Machine::step`.

`LIVE_REPO_FACT`: inspection tests cover:

- no-PIF behavior, accepted unprofiled input, explicit profile-without-input
  failure, missing values, unsupported `auto`, unknown aliases, and an
  unreadable literal path;
- generated malformed and unsupported files both with and without a profile;
- all three explicit CLI profiles and exact reported ranges;
- deterministic output, no successful PIF path or byte dump, no environment or
  current-directory search, and no filename inference; and
- unchanged absent-input BOOT-2-shaped synthetic behavior.

`RUNTIME_FACT`: the exact focused and full-gate results at product-source commit
`4eaa33d9fc59182d8e69a24edb39ee3be9ff8797` are recorded in
`VALIDATION.md`.

`INFERENCE`: because the named tests use generated input and exercise only
Machine copy materialization plus the existing `Lw` path, their success proves
represented copy semantics but not authentic firmware behavior, a private-ROM
checkpoint advance, or compatibility. Those authentic outcomes remain
`UNKNOWN`.
