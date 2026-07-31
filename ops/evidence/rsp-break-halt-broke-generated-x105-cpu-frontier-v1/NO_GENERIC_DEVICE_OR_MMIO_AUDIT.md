# No generic device or MMIO audit

The changed product seams are bounded additions to existing RSP decode,
`Sp` status application, and `Mi` SP-pending assertion. There is no device
registry, generic bus, generic MMIO layer, generalized physical map, processor
framework, pipeline, recursive `Machine::step`, dual commit, or public
`step_rsp`.

