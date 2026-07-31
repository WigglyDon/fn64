# Break provenance

`MachineRspBreakSource` retains:

- local instruction PC and prior RSP next PC;
- exact four-byte `SpImem` fetch provenance;
- raw word and exact Break identity;
- pre-Break halt, broke, interrupt-on-break, and MI SP-pending truth;
- prior MI SP-pending provenance;
- whether the conditional MI assertion path was selected.

The same bounded cause supplies `MachineMiRspBreakInterruptSource` when an MI
assertion is required. Values remain owned only by `Sp`, `Sp::rsp`, and `Mi`;
the provenance records causality rather than duplicate mutable truth.

