# RSP Run-Start Lineage

`MachineRspRunStartState` records the general causal transition from SP halt to
run.

`Pending` owns:

- the committing CPU SP-status store provenance, including instruction PC,
  source GPR, source lineage, and addresses;
- the exact status command;
- the current SP PC selected as the start PC.

The first successfully committed RSP instruction converts Pending to
`Consumed`, adding its local PC and semantic identity. Rejection preserves the
state. A later genuine halt true-to-false transition replaces it; idempotent
clear-halt does not.

The lineage is inspectable evidence, not execution authorization. General RSP
execution remains controlled by represented halt truth and the Machine-owned
processor turn.
