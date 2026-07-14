# Synthetic test coverage

Core tests cover the exact RI constant and source, construction/reset
unavailability, complete cold-bootstrap creation, repeated bootstrap,
independent Machines, both direct aliases, zero load result, destination
lineage, cadence, no read side effect, neighboring misses, unavailable-state
rollback, and unaligned AdEL.

The extended generated composition covers the cold BNE, one NOP delay slot,
stack adjustment, five exact big-endian high-SP-IMEM stores, source-specific
`CpuStoreWord` provenance, untouched neighboring bytes, four address
constructions, the `0x40` immediate, and atomic RI_CONFIG rejection.

`fn64_step_probe` invokes public `Machine::step` for stable RI_SELECT,
alias, miss, AdEL, lifecycle, independent-Machine, unavailable, cold-branch,
delay-slot, stack-save, and post-RI_SELECT frontier markers.
