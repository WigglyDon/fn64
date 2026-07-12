# Synthetic Test Coverage

All PIF-shaped fixtures are generated 1,984-byte patterns. No proprietary byte,
word, digest, disassembly, or expected firmware table is present.

Core proof covers:

- exact names, tokens, source/destination endpoints, and equal lengths for all
  three profiles;
- every copied byte and its exact source-offset provenance;
- every untouched byte remaining Unknown;
- absent, malformed, and unsupported input;
- host-buffer independence and failed-replacement atomicity;
- reset, repeated bootstrap, shorter-profile stale-tail clearing, and
  independent Machines;
- bootstrap failure preserving complete represented state; and
- a generated copied word consumed by `Lw` through public `Machine::step`.

Inspection proof covers:

- paired profile/path parsing, missing values, unsupported `auto`, and an
  unreadable literal path;
- generated malformed and unsupported files;
- all three explicit CLI profiles and exact reported ranges;
- deterministic output, no successful PIF path or byte dump, and no default
  filename search; and
- unchanged absent-input BOOT-2-shaped synthetic behavior.

Synthetic success proves these represented semantics only. It does not prove
authentic firmware behavior, a private-ROM checkpoint advance, or compatibility.
