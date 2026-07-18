# Opaque store semantics

Planning preserves base knownness, old-base capture, effective-address
arithmetic, alignment/AdES precedence, direct-address normalization, exact
target classification, pending-MI-transfer preflight, and ordinary immutable
plan/apply structure.

Unavailable source bits may commit only when the already-supported target is
an aligned SP-IMEM word. Application canonicalizes four private bytes, replaces
one word's knowledge atomically, and advances ordinary or delay-slot cadence
once. No GPR, device, other memory, or host state changes.
