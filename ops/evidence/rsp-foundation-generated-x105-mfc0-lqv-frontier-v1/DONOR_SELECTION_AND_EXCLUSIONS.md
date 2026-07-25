# Donor Selection And Exclusions

The preserved `master/direct-rsp-foundation-first-scalar-j-v1` lane remained
read-only. Its accepted reusable design was re-expressed through bounded edits
in this fresh lane.

Reused concepts and symbols:

| Donor symbol | Classification | Fresh-lane result | Required change |
|---|---|---|---|
| `MachineRspExecutionState` | `FOUNDATION_OWNER_REUSABLE` | `rsp.rs::MachineRspExecutionState` | removed scalar-J planning/application |
| scalar availability enums | `INITIAL_STATE_REUSABLE` | `rsp.rs` availability state | retained explicit unavailable values |
| `Sp::rsp` | `FOUNDATION_OWNER_REUSABLE` | `sp.rs::Sp::rsp` | preserved singular `Sp::pc` |
| task-start lineage shape | `RUN_START_LINEAGE_REUSABLE_AFTER_RENAME_OR_SPLIT` | `MachineRspRunStartState` | renamed to general halt-to-run evidence and separated from user-task truth |
| processor turn | `PROCESSOR_ARBITER_REUSABLE` | `MachineStepProcessor` private Machine token | removed J-specific path |
| selected RSP fetch | `FETCH_PREFLIGHT_REUSABLE` | `Machine::attempt_selected_rsp_instruction` | bounded decode to MFC0 and LQV frontier |
| SP-PC synchronization | `SP_PC_SYNCHRONIZATION_REUSABLE` | `MachineRspExecutionState::synchronize_pc_write` | no J delay ownership |

Explicit exclusions:

- all scalar-J opcode, target, planning, application, and delay-context work;
- user-task first-J probe logic and private-input endpoint assumptions;
- task-start naming for a general halt-to-run transition;
- any generated opcode shortcut or scheduler exception.

The donor was not formatted, staged, committed, reset, cleaned, or copied
wholesale.
