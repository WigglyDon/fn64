# Validation plan

The evidence arithmetic was checked against the pinned sources and two
independent emulator implementations. TSVs have fixed headers and no unresolved
placeholder. Product validation will use only generated data and the explicit
temporary Cargo target named by the Master packet.

Required closing markers are formatting pass, clippy with warnings denied,
focused handoff tests, all Rust tests, `fn64_machine_probe result: ok`,
`fn64_step_probe result: ok`, and `forward gate: ok`. Context, links, fleet,
queue, clean-checkout, patch reproduction, manifest, and forbidden-content
audits are recorded in the final Master artifact.
