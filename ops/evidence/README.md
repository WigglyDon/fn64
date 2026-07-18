# fn64 Evidence Schemas

Context role: canonical machine-readable evidence-schema index.
Scope: fn64 reset, step, no-window, performance, and host observations.
Canonical for: evidence record shapes and explicit unavailable-state vocabulary.
Not canonical for: machine implementation, proof results, or release acceptance.
Inherits: [operations scope law](../AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../../docs/context/CURRENT_STATE.md).
Related evidence: [evidence/release process](../../docs/process/EVIDENCE_AND_RELEASE.md).
Update triggers: a represented observation, required lineage field, or evidence family changes.

The files in `schemas/` are dependency-free JSON Schema documents. They define
artifact structures only; this pass does not instrument the core or claim that
every evidence family has a producer. Every record names its producer, source
commit, Context-SHA, unavailable facts, and artifact digest.

Canonical schemas:

- `machine-reset-snapshot.schema.json`
- `machine-step-trace.schema.json`
- `no-window-verification.schema.json`
- `performance-observation.schema.json`
- `host-runtime-observation.schema.json`

Current durable integration records:

- [BOOT-2 and Rust-purity integration](master-integrate-boot2-and-rust-purity-v1/README.md)
- [SP IMEM and aligned-Lw partial integration](master-integrate-sp-imem-lw-partial-and-provision-wave-2/README.md)
- [SP IMEM bootstrap provenance evidence](sp-imem-bootstrap-provenance-v1/README.md)
- [User-supplied PIF input-boundary evidence](user-supplied-pif-boot-source-v1/README.md)
- [Ordinary control flow and one delay slot](ordinary-control-flow-delay-slot-v1/README.md)
- [Profile-specific PIF IPL2 source mapping](pif-ipl2-source-mapping-v1/README.md)
- [Profiled PIF IPL2 copy materialization](pif-ipl2-profiled-copy-materialization-v1/README.md)
- [Cold x105 coupled handoff reconstruction and materialization](pif-ipl2-coupled-handoff-materialization-v1/README.md)
- [Generated x105 SP-DMEM load frontier](generated-x105-instruction-frontier-v1/README.md)
- [Aligned Sw generated x105 store frontier](aligned-sw-generated-x105-frontier-v1/README.md)
- [BLTZ generated x105 branch frontier](bltz-generated-x105-branch-frontier-v1/README.md)
- [Bounded MTC0 boot trio and generated x105 RI frontier](mtc0-boot-trio-generated-x105-ri-frontier-v1/README.md)
- [Cold-entry RI_SELECT read and generated x105 RI_CONFIG frontier](ri-select-generated-x105-ri-config-frontier-v1/README.md)
- [RI_CONFIG write and generated x105 RI_CURRENT_LOAD frontier](ri-config-generated-x105-current-load-frontier-v1/README.md)
- [RI_CURRENT_LOAD event and generated x105 RI_SELECT frontier](ri-current-load-generated-x105-ri-select-frontier-v1/README.md)
- [Exact RI_SELECT write and generated x105 RI_MODE frontier](ri-select-write-generated-x105-ri-mode-frontier-v1/README.md)
- [RI_MODE sequence and generated x105 MI init frontier](ri-mode-sequence-generated-x105-mi-init-frontier-v1/README.md)
- [Exact MI_INIT_MODE write and generated x105 RDRAM delay frontier](mi-init-mode-generated-x105-rdram-delay-frontier-v1/README.md)
- [Exact x105 MI transfer, global RDRAM delay, and REF_ROW frontier](rdram-delay-generated-x105-ref-row-frontier-v1/README.md)
- [Exact global RDRAM REF_ROW zero write and generated x105 DEVICE_ID frontier](rdram-ref-row-generated-x105-device-id-frontier-v1/README.md)
- [Exact global RDRAM DEVICE_ID request and generated x105 MI_VERSION frontier](rdram-device-id-generated-x105-mi-version-frontier-v1/README.md)
- [Fixed MI_VERSION identity and generated x105 first-responder frontier](mi-version-fixed-identity-generated-x105-first-responder-frontier-v1/README.md)
- [Exact first-responder DEVICE_ID request and generated x105 current-control frontier](rdram-first-responder-device-id-generated-x105-cc-frontier-v1/README.md)

The lineage contract is:

`cause → lawful bytes/input → address/subsystem → state change → observable result → artifact digest`.

Synthetic data must identify its generated origin. Commercial ROMs,
proprietary BIOS/PIF blobs, credentials, Git objects, build products, and Cargo
caches are forbidden. An `unavailable` status is a truthful result, especially
for the currently absent Rust host runtime and unmeasured performance.

The schemas are reviewable contracts, not automated proof of architecture.
Release manifests are checked with `tools/fleet/evidence-manifest check`; JSON
Schema validation may be added only when an existing dependency can own it
without enlarging product tooling.
