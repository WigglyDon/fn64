# Break SP owner decision

`Sp` remains the singular owner of halt, broke, single-step,
interrupt-on-break, signals, semaphore, addresses, and DMA records.
`Sp::rsp` remains the singular RSP execution and last-instruction owner.

Break changes the existing `Sp` status value and stores one immutable
`MachineRspBreakSource` causal record. It does not create a second status
register, task-completion flag, scheduler state, or host event.

