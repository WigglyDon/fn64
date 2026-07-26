# Scalar LW Alignment Boundary

Only four-byte-aligned local DMEM addresses are represented. A misaligned
address rejects atomically before any scalar, PC, count, turn, memory, or
device mutation.

This does not claim `AdEL`, another exception, or unaligned hardware behavior.
Those semantics remain unearned.
