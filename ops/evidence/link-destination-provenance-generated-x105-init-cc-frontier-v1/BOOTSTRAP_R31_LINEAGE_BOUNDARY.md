# Bootstrap r31 lineage boundary

Complete cold-x105 bootstrap truth is unchanged:

- r31 before the generated JAL: `0xFFFFFFFFA4001550`;
- lineage: retained PIF IPL2 bootstrap link.

That state remains true until the JAL commits. The correction does not clear,
relabel, or pre-convert it. The committing JAL replaces it with
`0xFFFFFFFFA40001A8` and exact JAL instruction-result provenance.

