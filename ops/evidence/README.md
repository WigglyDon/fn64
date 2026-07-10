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
