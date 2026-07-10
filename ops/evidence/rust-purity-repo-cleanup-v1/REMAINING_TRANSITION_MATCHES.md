# Remaining Transition Match Classification

The full case-sensitive and case-insensitive candidate-tree search is retained
in the external artifact. Counts below use source-line matches so repeated
terms on one line count once.

Two scopes keep the result interpretable:

- current surface: candidate files excluding canonical history/decision pages,
  lane coordination, and evidence directories;
- historical surface: `PROJECT_HISTORY.md`, `DECISION_LOG.md`, and the clearly
  labeled historical boot checkpoint.

| Match class | Current count | Historical count | Classification |
| --- | ---: | ---: | --- |
| C++ references | 11 | 21 | Current matches state retirement/absence or preserve one unchanged product-source comment; historical matches are chronology and decisions. None says C++ is present or runnable. |
| CMake commands | 0 | 0 | No command remains. |
| CMake references | 7 current boundary lines within the C++ set | 2 | Current lines state absence; historical lines identify the retired build era. |
| `fresh` commands | 0 | 0 | No match remains outside audit evidence. |
| migration terminology | 0 | 2 | Historical accepted-loss and cleanup-decision wording only. |
| parity terminology | 0 | 3 | Historical checkpoint/history/D017 wording only; literal uppercase `PARITY.md` path references are not terminology matches. |
| retired-binary command references (`fn64_selftest`, `build/fn64*`) | 0 | 0 | Removed from current/historical documentation. Earlier bytes remain in Git. |
| deleted `src/` path references | 0 | 6 | All six are inside explicitly historical owners; four name old subtrees, one labels their path class, and one says Git retains that history. |
| current `fn64_step_probe` references | 6 | 1 | Six refer to the current Rust probe. The one historical checkpoint reference is explicitly non-runnable old-era context. |

Additional candidate matches are classified as follows:

- `CURRENT_AND_CORRECT`: root law, README, current state, and build context say
  the retired implementation/build lane is absent; current probe and
  `PARITY.md` path references name live Rust owners.
- `CLEARLY_HISTORICAL`: project history, superseded/current decisions, and the
  historical boot checkpoint retain dated facts without runnable commands.
- `DURABLE_DECISION`: D006–D008, D015, D017, and D019 explain adoption,
  supersession, retirement, and cleanup.
- `VALID_FIXTURE`: packet/context fixtures retain current protocol semantics;
  the one obsolete discovery phrase was rewritten.
- `FALSE_POSITIVE`: `src/bin/...` paths under the current Rust inspection crate,
  `fixture-candidate` values, and uppercase `PARITY.md` filenames are not
  retired source, product candidates, or active comparison terminology.
- `LANE_EVIDENCE`: cleanup inventory and evidence quote pre-cleanup wording so
  reviewers can audit what changed; they do not own current product truth.
- `UNRESOLVED_STALE_REFERENCE`: none.
