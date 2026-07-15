# Validation

Candidate validation uses explicit pass-owned home-backed `CARGO_TARGET_DIR`
and `TMPDIR` on every Cargo invocation. It includes nonzero focused filters,
format, warnings-denied clippy, all Rust tests, both no-window probes, the full
forward gate, context/link/fleet/queue checks, patch reproduction, a clean
checkout, and post-integration canonical repetition.

Exact commands, counts, tested SHAs, Context-SHAs, markers, and cleanup results
are sealed in the Master artifact rather than committed as terminal-wall logs.
