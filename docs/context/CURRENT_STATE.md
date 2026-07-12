# Current Project State

Context role: canonical current-state owner.
Scope: fn64 at the accepted repository head.
Canonical for: current phase, authority, verification, lanes, blockers, and capability boundaries.
Not canonical for: detailed machine behavior, full history, or individual test evidence.
Inherits: [repository standing law](../../AGENTS.md).
Current-state owner: this document.
Related evidence: [rust/PARITY.md](../../rust/PARITY.md), [project history](PROJECT_HISTORY.md), and [lane registry](../lanes/lane-registry.md).
Update triggers: accepted authority, capability, verification, lane, or retirement state changes.

## Current phase and authority

- `LIVE_REPO_FACT`: fn64 is one Git repository whose sole current product implementation is the tracked Cargo workspace under `rust/`.
- `LIVE_REPO_FACT`: no active C/C++ source, header, CMake build owner, C++ proof binary, C++ host shell, or C++-only launch script remains in the current tree.
- `USER_DECISION`: the former C++ machine, host, proof, and build lane is retired without semantic-equivalence or inventory prerequisites. Git history is its only archive; unported behavior is intentionally absent.
- `LIVE_REPO_FACT`: `./rust/verify-forward` is the sole required product gate and has no CMake, C++, ROM, SDL/window/audio, or Git dependency.
- `USER_DECISION`: Master Codex alone provisions Worker topology and integrates Worker commits.
- `USER_DECISION`: fn64 may accept one explicitly selected user-supplied PIF
  firmware file as local runtime input. The host owns only the path, file read,
  read failure, and owned-byte transfer; the Machine must own accepted bytes,
  validation, lifecycle, state production, provenance, and rejection.
- `LIVE_REPO_FACT`: the existing no-window `fn64_boot_probe` now accepts one
  optional literal `--pif-rom` path. It performs no search or fallback; the host
  reads that path and transfers owned bytes into Machine validation.

## Forward machine truth

- `LIVE_REPO_FACT`: `fn64-core` owns the represented machine and public
  `Machine::step`; `fn64-inspection` owns deterministic no-window proof
  plumbing only.
- `RUNTIME_FACT`: BOOT-2 is the highest earned cartridge checkpoint. One
  authentic private-ROM-derived `SpecialAdd` committed through `Machine::step`
  with complete represented value, provenance, `pc` / `next_pc`, and Count
  lineage.
- `LIVE_REPO_FACT`: each Machine now owns 4 KiB of SP IMEM with explicit
  construction/reset, byte knownness independent of zero backing, and a narrow
  CPU-data route for the represented physical range. Complete aligned `Lw`
  semantics cover direct RDRAM and known SP IMEM, including sign extension,
  alias/zero-register behavior, data AdEL, lineage, cadence, and rollback.
- `RUNTIME_FACT`: the authentic trace still stops at `Lw` at `0xA4000044`.
  Known r9 produces CPU address `0xA4001000`, but SP IMEM offset zero is
  `Unknown`, so the load rejects before mutation.
- `INFERENCE`: integrated source-qualified evidence identifies the hardware
  producer chain: IPL1 copies proprietary IPL2 firmware content into SP IMEM,
  CPU control enters IPL2 there, IPL2 stages cartridge IPL3 in SP DMEM, and the
  observed x105 entry consumes `[0x000, 0x020)` and initially mutates
  `[0x000, 0x02c)`. External observability does not authorize embedding the
  values or make them current Machine truth.
- `LIVE_REPO_FACT`: represented execution remains incomplete and headless.
  BOOT-3, authentic handoff, cartridge-entry/game execution, compatibility,
  graphics, window, and audio are not claimed.
- `LIVE_REPO_FACT`: ordinary `BEQ`, `BNE`, `J`, `JAL`, `JR`, and `JALR`
  execute through `Machine::step` with one CPU-owned delay-slot context.
  Taken and untaken branches both execute one slot; link, alias, Count,
  branch-in-delay-slot rejection, and delay-slot EPC/BD behavior are explicit.
  This is the first ordinary control-flow family, not complete MIPS control
  flow.
- `LIVE_REPO_FACT`: Machine structurally accepts an explicitly supplied
  1,984-byte raw-Boot-ROM-shaped input, rejects the 2,048-byte full-map shape as
  unsupported, and classifies other lengths as malformed. Acceptance proves no
  authenticity, revision, executability, or compatibility. Accepted bytes are
  immutable Machine input that survives reset and repeated bootstrap staging;
  they currently execute nothing and produce no known SP IMEM byte.
- `EXTERNAL_TECHNICAL_EVIDENCE`: pinned NTSC, PAL, and MPAL IPL
  reconstructions share raw source start `0x0d4` and SP IMEM destination zero,
  but NTSC ends at `0x71c` (`0x648` bytes) while PAL and MPAL end at `0x720`
  (`0x64c` bytes). The current 1,984-byte structural shape cannot select that
  profile, and unexamined physical PIF revisions remain `UNKNOWN`.

The single detailed owner for represented capability and explicit absence is
the [represented-machine capability ledger](../../rust/PARITY.md). Stable
architecture boundaries live in the active subsystem pages; retirement
chronology lives in [project history](PROJECT_HISTORY.md).

## Verification and lanes

- Required gate: `./rust/verify-forward`.
- `real-cartridge-boot-spine-v1`: completed, integrated through Master
  authority, and closed at accepted worker candidate `8e5efc8e`.
- `rust-purity-repo-cleanup-v1`: completed, integrated through Master
  authority, and closed at accepted worker candidate `9cc16142`.
- Active C++ lanes: none.
- `cpp-reference-truth-reconstruction-v1`: canceled without provisioning; its reserved branch/worktree remain absent.
- The earlier seam-090 and inventory-first sequences are superseded by the direct Master retirement decision.
- `boot-frontier-sp-imem-lw-v1`: candidate `dcb9f1bf` was independently
  verified, integrated as a truthful partial product increment, and closed
  **PARTIAL — INTEGRATED**. It earned SP IMEM plus aligned `Lw`; it did not
  advance the authentic trace beyond BOOT-2.
- `sp-imem-bootstrap-provenance-v1`: evidence-only candidate `8db1b57c` was
  independently verified and integrated **PARTIAL — EVIDENCE INTEGRATED;
  PRODUCT SOURCE UNAVAILABLE**. It changed no product behavior and earned no
  higher checkpoint.
- `user-supplied-pif-boot-source-v1`: complete three-commit candidate
  `1fa8aa17` was independently verified and integrated **ACCEPTED —
  SOURCE-BOUNDARY PRODUCT**. No private PIF input was used and BOOT-2 did not
  advance.
- `ordinary-control-flow-delay-slot-v1`: complete three-commit candidate
  `01b06e5a` was independently verified and integrated **ACCEPTED**. The shared
  J/JAL target helper passed a region-crossing PC+4 discriminator, the direct
  step probe passed, and no boot checkpoint changed.
- `pif-ipl2-source-mapping-v1`: evidence-only candidate `2ee4b3c7` was
  independently verified and integrated **ACCEPTED — VARIANT-SPECIFIC SOURCE
  MAPPING**. It changed no product behavior and used no private input.
- Integration queue: empty after both reviewed Wave 3 entries were closed.
- `USER_DECISION`: the evidence-only mapping lane may run concurrently with the
  synthetic ordinary-control-flow product lane. Neither may edit coordination
  state, and neither receives private-ROM or private-PIF authority.
- Repository-purity cleanup is complete for its accepted non-product scope.
- Product Acceleration Wave 1 selected one combined frontier because
  storage/routing, aligned `Lw`, bootstrap knownness, Machine step application,
  and boot-probe continuation shared direct source ownership. Its accepted
  partial result is now product truth; no compatibility fact was earned.

## Blockers and known unknowns

- `LIVE_REPO_FACT`: the current Rust product remains deliberately incomplete and headless.
- `UNKNOWN`: performance, broad hardware compatibility, BOOT-3 and later boot
  behavior, game behavior after handoff, and host-runtime behavior remain
  unmeasured or unavailable.
- `LIVE_REPO_FACT`: fn64 has an explicit PIF-firmware input, structural
  validation, immutable Machine ownership, and reset/bootstrap persistence. It
  still has no authentic firmware classification, firmware execution, or
  source-backed SP IMEM production. The authentic `Lw` still rejects because
  its first SP IMEM source byte is `Unknown`.
- `LIVE_REPO_FACT`: accepted firmware still has no SP IMEM production effect.
  A future copy effect requires an explicit pinned NTSC, PAL, or MPAL Machine
  profile; it may not infer one from byte shape, content, filename, cartridge,
  or digest.
- `UNKNOWN`: the complete pre-IPL3 handoff state, including source-qualified
  ownership for `t3`, `ra`, other consumed GPRs, COP0, and device facts. The
  mapping evidence does not prove that minimal IPL2 execution is required.
- `USER_DECISION`: retired C++ behavior does not define a product backlog. Any future Rust capability requires its own bounded product decision and proof.
- `LIVE_REPO_FACT`: ignored user-local assets remain outside repository truth and routine evidence.
