# Vector Register Owner Decision

The existing private `MachineRspExecutionState` nested in `Sp` owns the vector
unit. `MachineRspVectorUnitState` contains exactly 32 individually addressed
`MachineRspVectorRegisterState` slots.

`SpDmem` retains DMEM ownership. Vector provenance references DMEM knowledge
descriptors but owns neither DMEM bytes nor a duplicate DMEM knowledge map.
`Sp::pc` remains the singular current RSP PC; the nested RSP state retains its
separate next PC, count, delay, scalar, vector, and last-instruction truth.
