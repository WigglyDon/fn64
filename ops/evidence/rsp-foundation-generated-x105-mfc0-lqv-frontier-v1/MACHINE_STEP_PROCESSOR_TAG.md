# Machine Step Processor Tag

`Machine::step` remains the sole public represented execution entrance.

Successful RSP execution returns:

`MachineRepresentedStepOutcome::RspCommitted { outcome }`

Every outcome exposes `processor() -> MachineStepProcessor`. Existing CPU
outcome payloads remain intact and classify as `Cpu`; the bounded scalar MFC0
outcome classifies as `Rsp`.

Selected-RSP rejection is owned by
`MachineRepresentedStepError::RspRejected` and also identifies the RSP
processor. One call reports one selected processor and commits at most one
instruction.
