# Validation

## Self-identification boundary

A tracked file cannot contain the hash of the commit that contains it, and an
archive member cannot contain the final hash of the archive that contains it.
This durable record therefore names the exact starting facts and repeatable
commands. The exact post-commit candidate SHA, resulting Context-SHA, final
worktree/index state, artifact SHA-256, archive listing, and post-commit command
logs are recorded in the external validation record, checksum sidecar, and
authoritative Worker final packet.

## Topology preflight

- Repository root:
  `/home/don/fn64-worktrees/pif-ipl2-source-mapping-v1`
- Branch: `worker/pif-ipl2-source-mapping-v1`
- Initial HEAD: `c085bed4a599b5d5ff20898894b89ab2ed78dd44`
- Initial Context-SHA:
  `b3fc9cfeff64cd407d5ddb4686c236e05e14018571ae7d7504480f9ffcd09279`
- Initial `git status --short`: empty
- Initial `git diff --cached --quiet`: exit 0
- Lane doctor: `READY`, exit 0, using the exact packet command and expected
  branch, base, lane ID, and Context-SHA
- Provisioning mutation: none

## Initial complete-gate results

These results were obtained after the substantive evidence files were authored.
The complete evidence tree, including this record, is checked again before
commit; the candidate commit is checked again afterward.

| Exact command | Exit | Result |
| --- | ---: | --- |
| `./tools/fleet/context-sha --root /home/don/fn64-worktrees/pif-ipl2-source-mapping-v1 --machine` | 0 | committed context; 45 paths; digest `b3fc9cfeff64cd407d5ddb4686c236e05e14018571ae7d7504480f9ffcd09279`; 0 dirty context paths; `result=ok` |
| `./tools/fleet/context-verify --root /home/don/fn64-worktrees/pif-ipl2-source-mapping-v1` | 0 | 15 checks, 0 errors, `result: ok` |
| `./tools/fleet/test-fleet` | 0 | 52 passed, `result: ok` |
| `./tools/fleet/integration-queue check` | 0 | `integration-queue: ok` |
| `PATH=/home/don/.cargo/bin:$PATH ./rust/verify-forward` | 0 | format and clippy passed; 367 core, 12 inspection, and 8 CLI tests passed; machine and step probes passed; `forward gate: ok` |

The Rust gate proves only that evidence work did not break the current Rust
product. It does not prove the mapping, firmware authenticity, IPL1 execution,
IPL2 execution, BOOT-3, or cartridge compatibility. The initial gate created
`rust/target`; it was removed, its absence was confirmed, and no unknown file
was removed.

## Canonical Markdown link check

The canonical current repository command is:

```text
./tools/fleet/context-verify --root /home/don/fn64-worktrees/pif-ipl2-source-mapping-v1
```

It was identified from the local-link stage in current
`tools/fleet/context-verify` and from the broken-link fleet fixture, not invented
as a substitute. Its initial result was exit 0 with 15 checks and 0 errors.
Because the context manifest does not include this evidence lane, a separate
lane-only audit applies the same local-link rules to every lane Markdown file.
That supplemental audit reported `lane_markdown_links: 0 broken local links`,
exit 0, on the complete evidence tree.

## TSV and arithmetic audit

The deterministic TSV audit uses tab as the sole separator, compares each
literal required header, checks 14 columns in `SOURCE_MAPPING.tsv`, 11 columns
in `VARIANT_MATRIX.tsv`, and 11 columns in
`EXTERNAL_SOURCE_REGISTER.tsv`, and requires column 10 of every source row to
equal literal `no`.

For every non-`UNKNOWN` mapping row, the arithmetic audit evaluates:

```text
source_end_exclusive - source_start == length_bytes
destination_end_exclusive - destination_start == length_bytes
source_start >= 0 && source_end_exclusive <= 0x7C0
destination_start >= 0 && destination_end_exclusive <= 0x1000
destination_start <= 0x000 && destination_end_exclusive >= 0x020
destination_start <= 0x000 && destination_end_exclusive >= 0x02C
```

Complete-tree result, exit 0: all required headers were exact, all row widths
were equal, every source-register copy field was `no`, three exact mapping rows
passed all six arithmetic/bounds/coverage checks, and the explicitly unknown
row contained no guessed numeric value.

## Forbidden-content audit

The complete-tree audit is restricted to
`ops/evidence/pif-ipl2-source-mapping-v1/` and checks:

- no symlinks;
- only `.md` and `.tsv` member extensions;
- no likely firmware or cartridge-image extensions;
- no long delimited hexadecimal byte or word sequences;
- no assembly/disassembly instruction blocks;
- no source fragments;
- no unresolved drafting or template marker;
- no private absolute path. The required assigned worktree root and later
  external artifact path are the only allowed absolute local paths;
- no proprietary bytes, words, hashes, filenames, or private input.

Public source commit IDs, the repository Context-SHA, CPU addresses, numeric
offsets, HTTPS anchors, and the explicitly assigned worktree path are expected
evidence metadata, not firmware content.

Complete-tree result, exit 0: no symlink, disallowed member extension,
ROM-like extension, long delimited hexadecimal sequence, assembly block,
source fragment, unresolved marker, credential pattern, unexpected local path,
or private-input evidence was found.

## Commit and final validation boundary

Before staging, the worker inspects status, unstaged whitespace, stat,
name-status, and full diff. Only paths below
`ops/evidence/pif-ipl2-source-mapping-v1/` may be staged. Cached whitespace,
stat, name-status, and full diff are inspected before commit.

After the one bounded evidence commit, the external validation record supplies:

- exact candidate HEAD and commit subject;
- base ancestry and left/right divergence;
- exact resulting Context-SHA and context verification;
- fleet, integration queue, canonical link, supplemental lane link, TSV,
  forbidden-content, and Rust forward-gate logs;
- exact tested candidate SHA;
- post-gate `rust/target` removal;
- final clean worktree and index;
- `/tmp/UPLOAD_ME_fn64_pif_ipl2_source_mapping_v1.tar.gz`, its adjacent SHA-256
  sidecar, archive member count, listing validation, and member-name audit.
