# PIF IPL2 Handoff State Mapping V1

Context role: retired historical lane and donor-state memory.
Scope: evidence-only mapping of every pre-IPL3 Machine-state fact consumed before cartridge code overwrites it.
Canonical for: this lane's topology, evidence authority, writable boundary, required questions, dependencies, overlap, stop conditions, and retirement condition.
Not canonical for: Rust product behavior, private firmware content, candidate acceptance, or canonical integration.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [PIF source mapping](../../ops/evidence/pif-ipl2-source-mapping-v1/README.md), [bootstrap provenance](../../ops/evidence/sp-imem-bootstrap-provenance-v1/README.md), [represented-machine capability](../../rust/PARITY.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, evidence creation, classification, blocking, integration, or retirement.

## Lane identity and state

- Lane ID: `pif-ipl2-handoff-state-mapping-v1`
- Goal ID: `fn64-map-pre-ipl3-handoff-state-v1`
- Supervisor role: PIF IPL2 Handoff State Mapping Supervisor GPT
- Worker Codex worktree:
  `/home/don/fn64-worktrees/pif-ipl2-handoff-state-mapping-v1`
- Branch: `worker/pif-ipl2-handoff-state-mapping-v1`
- Accepted base source: the exact canonical Wave 4 registration commit owned by
  the Master provisioning report and first executable packet.
- Governing Context-SHA: the exact committed Wave 4 registration value owned by
  the Master report and first executable packet. This page cannot embed its own
  digest.
- Status: **RETIRED — UNACCEPTED HISTORICAL DONOR**
- Historical provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Provisioning exception: `NONE`
- Launch state: retired; no supervisor, Worker, retry, or relaunch is active or
  authorized.

Unaccepted candidate:
`c24ab78c9a4b93fe79b660f3428d06a6a570c4dd`. Its stale artifact
`/tmp/UPLOAD_ME_fn64_pif_ipl2_handoff_state_mapping_v1.tar.gz` has SHA-256
`f1d864415d64c51a03ebcd3890b92a2f5ebc6b1676decd6d11ba80b9047fd2c6`
and is not accepted or promoted. The first repair attempt stopped before
modification because tmpfs user-quota headroom was exhausted. The later
context-propagation merge `96840e996208d35baabbfd6ffe921f01272699c9` and
preserved branch/worktree remain historical donor state.

The unresolved historical evidence defect is exact: reconstruct retained IPL2 r31/ra
separately for `NTSC_PINNED`, `PAL_PINNED`, and `MPAL_PINNED`, distinguishing
the signed relation directly consumed by the first x105 branch from the
complete retained link address as control-flow provenance. A direct Master pass
may inspect these claims as leads only; it does not resume or accept this lane.

## Evidence mission

Map every Machine-state fact consumed by the observed x105 IPL3 path before its
first cartridge-code write. The lane must identify source owners for `t3`,
`sp`, `ra`, every additional first-use GPR, COP0 facts, RDRAM, SP DMEM, SP IMEM,
PI, SI, PIF RAM, and other device state. It must distinguish IPL1, IPL2,
PIF-RAM, cartridge-header, device, region/revision, and CIC/bootcode causes.

The result must classify exactly one of:

- `HANDOFF_STATE_MATERIALIZATION_PROVEN`;
- `MIXED_NARROW_STATE_EFFECTS_REQUIRED`;
- `MINIMAL_IPL2_EXECUTION_REQUIRED`;
- `PARTIAL_HANDOFF_MAPPING`; or
- `UNKNOWN`.

Minimal execution is earned only by proof that independently named
source-backed effects cannot preserve required causality. Difficulty locating a
fact is not that proof.

## Exact writable and forbidden scope

Worker-writable repository scope is exactly:

- `ops/evidence/pif-ipl2-handoff-state-mapping-v1/**`
- external lane logs and artifacts

All Rust source/tests, Cargo metadata, `rust/verify-forward`, `docs/**`,
`tools/fleet/**`, `ops/fleet/**`, another lane's evidence, private inputs,
ignored user content, Git topology, and canonical `main` are forbidden.

The lane may read current fn64 source and pinned public technical sources. It
must not search local storage, inspect a private PIF or cartridge ROM, copy
firmware/source code, vendor external trees, or reproduce assembly,
disassembly, firmware words, or binaries.

## Required proof, dependency, overlap, and closure

Durable evidence must include first-use GPR, COP0, memory/device, and variant
matrices; materialization-versus-execution analysis; a pinned external-source
register; copyright boundary; source anchors; and exact validation. Every
inference names its supporting facts, and unexamined variants remain
`UNKNOWN`.

Dependencies: accepted retained-IPL2 provenance `8db1b57c`, accepted input
boundary `1fa8aa17`, accepted source mapping `2ee4b3c7`, and current source.
Direct writable overlap with `pif-ipl2-profiled-copy-materialization-v1`:
`NONE`; indirect overlap is only later Master product-topology and context
reconciliation. Either candidate may be reviewed independently. The profiled
copy may land first, but this evidence lane must not assume its product result.

Private-ROM authority: `NONE`. Private-PIF authority: `NONE`. Stop when
evidence cannot distinguish owners, progress requires proprietary content or a
game-specific rule, authority overlaps, destructive Git would be required, or
reviewability is lost. Retire after accepted integration, a precise partial, or
explicit Master stop. Expected next milestone: the smallest source-qualified
product lane for post-copy handoff state, with minimal execution still
unearned unless proved.
