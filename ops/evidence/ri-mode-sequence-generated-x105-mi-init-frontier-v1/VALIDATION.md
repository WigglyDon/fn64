# Validation

The candidate is validated with exact home-backed Cargo target and TMPDIR
paths on every Cargo invocation. Required proof includes format,
warnings-denied clippy, nonzero focused tests, the full Rust forward gate,
both direct probes, context/fleet/queue and Markdown-link checks, a fresh clean
checkout, and post-integration canonical repetition.

Exact logs, tested SHAs, Context-SHAs, storage audit, clean-checkout proof,
artifact checks, and generated-path cleanup are sealed in the Master artifact
rather than committed as terminal-wall output.
