# PIF IPL2 Source Mapping V1

Context role: active lane coordination memory.
Scope: evidence-only numeric and variant-qualified mapping from structurally accepted raw PIF Boot ROM bytes to retained IPL2 SP IMEM content.
Canonical for: this lane's purpose, topology, evidence authority, writable boundary, required questions, proof, dependencies, overlap, stop conditions, and retirement condition.
Not canonical for: Rust product behavior, private firmware content, candidate acceptance, or canonical integration.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [input-boundary evidence](../../ops/evidence/user-supplied-pif-boot-source-v1/README.md), [bootstrap provenance](../../ops/evidence/sp-imem-bootstrap-provenance-v1/README.md), [represented-machine capability](../../rust/PARITY.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, evidence creation, candidate creation, mapping classification, blocking, integration, or retirement.

## Lane identity and state

- Lane ID: `pif-ipl2-source-mapping-v1`
- Goal ID: `fn64-map-pif-boot-rom-to-retained-ipl2-sp-imem-v1`
- Purpose: determine whether the complete retained IPL2 input consumed from SP
  IMEM by the observed x105 IPL3 entry can be produced from one source-clear
  numeric slice of the accepted 1,984-byte raw PIF Boot ROM shape.
- Supervisor role: PIF IPL2 Source Mapping Supervisor GPT
- Worker Codex worktree:
  `/home/don/fn64-worktrees/pif-ipl2-source-mapping-v1`
- Branch: `worker/pif-ipl2-source-mapping-v1`
- Accepted base source: the exact canonical Wave 3 registration state owned by
  the Master provisioning report and first executable packet.
- Governing Context-SHA: the exact committed provisioning value owned by the
  Master report and first executable packet. This context page cannot embed its
  own digest.
- Status: **PROVISIONED — AWAITING SUPERVISOR PACKET**
- Provisioning state: `MASTER_PROVISIONED_VERIFIED`
- Provisioning exception: `NONE`
- Launch state: persistent Worker topology was created and verified by Master;
  no command is executable until Master GPT issues the matching supervisor
  seed.

## Exact writable and forbidden scope

Worker-writable repository scope is exactly:

- `ops/evidence/pif-ipl2-source-mapping-v1/**`
- external lane logs and artifacts

The Worker must not modify Rust source or tests, Cargo metadata,
`rust/verify-forward`, `docs/**`, `tools/fleet/**`, `ops/fleet/**`, another
lane's evidence, private inputs, ignored user content, Git topology, or
canonical `main`.

The lane may read current fn64 source and public technical references. It may
not search local storage for PIF firmware, inspect private firmware or ROMs,
copy firmware bytes or code, vendor external source, or reproduce firmware
words, assembly, or disassembly.

## Evidence mission and required proof

The evidence pass must establish or leave explicit:

1. exact raw-PIF source start, end, and copied length;
2. exact SP IMEM destination and lifecycle;
3. whether region, revision, PIF, or console variant changes the mapping;
4. whether `[0x000, 0x020)` and the x105 initial mutation-input range
   `[0x000, 0x02c)` are completely contained;
5. whether source-backed materialization preserves the actual IPL1
   machine-visible effect;
6. whether another pre-IPL3 machine-visible effect makes copy-only
   materialization incomplete;
7. whether minimal IPL1/IPL2 execution is genuinely required; and
8. the smallest honestly earned follow-on product lane.

Evidence priority is current fn64 source and integrated evidence, official or
contemporaneous documentation, pinned clean-room or reverse-engineered source,
and independent emulator corroboration. Informal sources are leads only.
External observability is not fn64 product authority.

The final result must use exactly one classification:

- `SOURCE_BACKED_MATERIALIZATION_PROVEN`;
- `MINIMAL_FIRMWARE_EXECUTION_REQUIRED`;
- `VARIANT_SPECIFIC_MAPPING_REQUIRES_EXPLICIT_MACHINE_PROFILE`;
- `PARTIAL_MAPPING_ONLY`; or
- `UNKNOWN`.

Materialization is proved only with exact source/destination ranges, lifecycle,
variant scope, more than one evidence class, and no omitted pre-IPL3 effect
needed by the current trace. Difficulty finding offsets does not prove that
execution is required.

## Authority, dependency, overlap, and closure

Authority is evidence-only. No Machine, host, validation, SP IMEM, bootstrap,
checkpoint, or compatibility behavior may change. Private-ROM authority:
`NONE`. Private-PIF authority: `NONE`.

Dependency: accepted input-boundary product commit `1fa8aa17`, Master
reconciliation `0d16dfebc5b7c74228d0416ae562227cfd1fedc3`, and the integrated
retained-IPL2 provenance record.

Direct writable overlap with `ordinary-control-flow-delay-slot-v1`: `NONE`.
The mapping lane owns only its evidence directory; the control-flow lane owns
registered Rust, step-probe, and separate evidence paths. Indirect overlap is
limited to later Master context reconciliation. Either accepted candidate may
integrate independently; source mapping precedes any future PIF
materialization/execution lane.

Stop when evidence cannot distinguish owners or variants, when progress would
require private firmware, copied proprietary content, a game-specific policy,
destructive Git action, or expanded product authority. Retire after accepted
integration, a precise partial, or explicit Master stop. Expected result: an
exact mapping classification and a bounded recommendation, with BOOT-2 and
product behavior unchanged.
