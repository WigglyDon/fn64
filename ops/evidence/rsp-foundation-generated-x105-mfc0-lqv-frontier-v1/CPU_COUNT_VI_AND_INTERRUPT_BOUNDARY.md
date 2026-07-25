# CPU Count, VI, And Interrupt Boundary

CPU-selected calls retain the accepted path:

- MI/COP0 synchronization and CPU interrupt recognition;
- CPU fetch/plan/apply or rejection;
- COP0 Count cadence;
- CPU committed-instruction accounting;
- VI cadence and post-step interrupt synchronization.

RSP-selected calls do none of those CPU-only operations. RSP success or
rejection advances neither COP0 Count, CPU committed-step count, nor VI.
`Sp::rsp` owns a separate successful RSP instruction count. A future
RSP-produced interrupt remains outside this pass and would be observed by the
CPU only at a later CPU-selected boundary.
