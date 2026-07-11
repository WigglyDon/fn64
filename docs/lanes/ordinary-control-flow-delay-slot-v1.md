# Ordinary Control Flow And Delay Slot V1

Context role: active lane coordination memory.
Scope: the first complete ordinary MIPS control-flow family and delay-slot semantics through the existing Machine step spine.
Canonical for: this lane's purpose, topology, writable boundary, machine authority, proof, dependencies, overlap, stop conditions, and retirement condition.
Not canonical for: accepted product behavior, PIF/bootstrap behavior, candidate acceptance, or canonical integration.
Inherits: [repository standing law](../../AGENTS.md) and [worker worktree provisioning](../process/WORKTREE_PROVISIONING.md).
Current-state owner: [CURRENT_STATE.md](../context/CURRENT_STATE.md).
Related evidence: [CPU execution](../context/subsystems/cpu-execution.md), [instruction pipeline](../context/subsystems/instruction-pipeline.md), [exceptions and COP0](../context/subsystems/exceptions-and-cop0.md), [represented-machine capability](../../rust/PARITY.md), and [integration queue](integration-queue.md).
Update triggers: provisioning, packet launch, implementation decision, candidate creation, context propagation, blocking, integration, or retirement.

## Lane identity and state

- Lane ID: `ordinary-control-flow-delay-slot-v1`
- Goal ID: `fn64-ordinary-control-flow-and-delay-slot-v1`
- Purpose: implement `BEQ`, `BNE`, `J`, `JAL`, `JR`, and `JALR` as one
  coherent family while preserving the represented `pc` / `next_pc` model,
  exception ownership, rollback, and `Machine::step` authority.
- Supervisor role: Ordinary Control Flow And Delay Slot Supervisor GPT
- Worker Codex worktree:
  `/home/don/fn64-worktrees/ordinary-control-flow-delay-slot-v1`
- Branch: `worker/ordinary-control-flow-delay-slot-v1`
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

- `rust/crates/fn64-core/src/lib.rs`
- `rust/crates/fn64-core/src/machine.rs`
- `rust/crates/fn64-core/src/cpu.rs`
- `rust/crates/fn64-core/src/cpu/instruction.rs`
- `rust/crates/fn64-core/src/cpu/scalars.rs`
- `rust/crates/fn64-core/src/cpu/cop0.rs`
- `rust/crates/fn64-inspection/src/bin/fn64_step_probe.rs`
- tests embedded in the listed Rust source files
- `ops/evidence/ordinary-control-flow-delay-slot-v1/**`
- external lane logs and artifacts

The Worker must not modify PIF firmware ownership or validation,
`rust/crates/fn64-core/src/pif_firmware.rs`,
`rust/crates/fn64-core/src/machine/cartridge_bootstrap.rs`,
`rust/crates/fn64-core/src/sp_imem.rs`, boot-probe source or tests, Cargo
metadata, `rust/verify-forward`, `docs/**`, `tools/fleet/**`, `ops/fleet/**`,
another lane's evidence, private inputs, ignored user content, Git topology, or
canonical `main`.

## Machine authority and required semantics

Machine owns branch/jump planning, application, cadence, rollback, and public
step results. CPU scalar state owns explicit delay-slot context; COP0 remains
the sole owner of branch-delay exception EPC and BD state. Decode and identity
remain in the current instruction layer. The implementation must not create a
public `Cpu::step`, generic execute dispatcher, all-future step result, or
generic control-flow framework.

Required semantics include taken and untaken `BEQ`/`BNE`, signed branch target
calculation, `J`/`JAL` region construction, register targets, link addresses,
`JALR` source/destination aliasing, zero-register discard, read-before-write,
one delay-slot instruction exactly once, target only after that slot, and Count
once per committed instruction. A delay-slot exception must set BD, identify
the owning branch/jump in EPC, and prevent normal target commit.

The current `pc` / `next_pc` primitives are an earned interface. Live source
also proves a missing fact: sequential `pc` / `next_pc` alone cannot identify
the delay slot of an untaken branch, so explicit narrowly owned delay-slot
context is required. A branch or jump in a delay slot must be rejected or
classified explicitly when it cannot be represented honestly; it must not be
guessed.

Explicitly absent: branch-likely annul, REGIMM unless later proven necessary,
COP0 branches, ERET, interrupts, TLB/MMU, ROM-specific behavior, a private-ROM
test, a PIF test, a generic bus, and a generalized memory map.

## Proof, dependency, overlap, and closure

Synthetic generated proof must cover all six identities, taken/untaken and
positive/negative branch offsets, wrapping, jump regions, links and aliasing,
zero destination, delay-slot/target order, per-instruction Count, delay-slot
exception EPC/BD, no target commit after exception, branch-in-delay-slot
policy, rollback, and existing Machine-step regressions. The existing
`fn64_step_probe` may be extended; no second control-flow or boot probe is
authorized.

Dependency: current decode identities, `CpuControlFlowSnapshot`, scalar
stage/restore/commit primitives, Machine producer/applicator cadence, and COP0
exception entry. Private-ROM authority: `NONE`. Private-PIF authority: `NONE`.

Direct writable overlap with `pif-ipl2-source-mapping-v1`: `NONE`. That lane is
evidence-only in its own directory. Indirect overlap is limited to later Master
context reconciliation. Either accepted candidate may integrate independently;
product truth always integrates before its Master context reconciliation.

Stop when complete delay-slot semantics require an unearned generalized
framework, ownership becomes ambiguous, another active lane acquires a listed
path, private input becomes necessary, accepted exception law contradicts the
mission, destructive Git action would be required, or reviewability is lost.
Retire after accepted integration, a precise partial, or explicit Master stop.
Expected result: complete ordinary control flow with synthetic proof; no boot
checkpoint or compatibility claim.
