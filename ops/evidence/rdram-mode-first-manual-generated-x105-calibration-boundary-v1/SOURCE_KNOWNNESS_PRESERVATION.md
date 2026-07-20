# Source-Knownness Preservation

The new target accepts only known source lineage and does not weaken address,
arithmetic, branch, jump-register, load, store, SP-DMEM, or device-command
knownness. Unknown values remain unavailable rather than becoming zero or a
symbolic value.
