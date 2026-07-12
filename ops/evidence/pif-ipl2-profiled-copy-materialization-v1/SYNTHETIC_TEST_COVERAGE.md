# Synthetic Test Coverage

All PIF-shaped fixtures are generated 1,984-byte patterns. No proprietary byte,
word, digest, disassembly, or expected firmware table is present.

Core proof covers:

- exact names, tokens, source/destination endpoints, and equal lengths for all
  three profiles;
- every copied byte and its exact source-offset provenance;
- every untouched byte remaining Unknown;
- no firmware/no profile, accepted firmware/no profile, profile/no firmware,
  and both install orders;
- malformed and unsupported input with full-state replacement rollback;
- host-buffer independence and failed-replacement atomicity;
- reset, repeated bootstrap, PAL-to-NTSC and MPAL-to-NTSC stale-tail clearing,
  and independent Machines;
- bootstrap failure preserving complete represented state; and
- a generated copied word consumed by `Lw` through public `Machine::step`.

Inspection proof covers:

- no-PIF behavior, accepted unprofiled input, explicit profile-without-input
  failure, missing values, unsupported `auto`, and an unreadable literal path;
- generated malformed and unsupported files both with and without a profile;
- all three explicit CLI profiles and exact reported ranges;
- deterministic output, no successful PIF path or byte dump, no environment or
  current-directory search, and no filename inference; and
- unchanged absent-input BOOT-2-shaped synthetic behavior.

Synthetic success proves these represented semantics only. It does not prove
authentic firmware behavior, a private-ROM checkpoint advance, or compatibility.
