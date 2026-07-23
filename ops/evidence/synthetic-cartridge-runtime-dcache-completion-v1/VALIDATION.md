# Validation

Focused product proofs completed before closure:

- complete `fn64-core`: 557 passed, 0 failed;
- exact runtime-v2 cold composition: 1 passed, 0 failed;
- cache ownership/dirty replacement focused proofs: 3 passed, 0 failed;
- stable no-window step probe: 187 cases, `result: ok`;
- inspection exact-output regression: 1 passed, 0 failed;
- `git diff --check`: clean.

The measured runtime result is:

`checksums=4077adef/096b847a attempts=7477116 committed=7477116
program_steps=77 final_count=7477100 final_commits=7477116
success_loop=0x80001124 icache_fills=10 icache_hits=67 dcache_lh=4
dcache_lm=2 dcache_sh=2 dcache_sm=1 writebacks=3 bypass=8`.

Formatting, clippy, full inspection/CLI suites, forward gate, context/fleet,
detached exact-SHA, canonical post-fast-forward, and artifact verification are
closure gates recorded in the external sealed report because their final SHA
cannot be self-referentially committed here.
