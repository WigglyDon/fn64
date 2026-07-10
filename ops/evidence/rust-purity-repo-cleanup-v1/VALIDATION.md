# Validation

Resulting Context-SHA:
`418a4eabbf1aa2a56f00ca51198f1e7a71407a399f1da877c275c39bfd74b4a4`.

Tested candidate SHA: `SEE_EXTERNAL_POST_COMMIT_VALIDATION`.

The exact post-commit candidate SHA is recorded in the external artifact and
authoritative Worker final packet. A repository file cannot truthfully contain
the hash of the commit that contains the file itself; the accepted evidence
process requires that self-hash boundary to remain external.

| Command / stable name | Exit status | Concise outcome |
| --- | ---: | --- |
| `git --no-pager diff --check` | 0 | no whitespace errors |
| `./tools/fleet/context-sha --root /home/don/fn64-worktrees/rust-purity-repo-cleanup-v1 --machine` | 0 | 40 manifest paths; resulting digest above |
| `./tools/fleet/context-verify --root /home/don/fn64-worktrees/rust-purity-repo-cleanup-v1` | 0 | 15 checks, 0 errors, `result: ok` |
| one-shot candidate-tree Markdown local-link check | 0 | 0 broken links |
| `./tools/fleet/packet-verify tools/fleet/fixtures/packets/valid-context-delta.packet` | 0 | 12 checks, 0 errors |
| `./tools/fleet/test-fleet` | 0 | 52 checks, `result: ok` |
| `./tools/fleet/integration-queue check` | 0 | `integration-queue: ok` |
| `PATH=/home/don/.cargo/bin:$PATH ./rust/verify-forward` | 0 | format, clippy, 335 tests, both probes, `forward gate: ok` |

Focused verifier fixtures: not applicable — no verifier behavior changed.

Modified fleet executables: none. The full fleet suite's shell-syntax stage
validated all existing executables.

Product-source immutability, base ancestry, final cleanliness, and exact
post-commit candidate/Context-SHA pairing are recorded after commit in the
external validation record.
