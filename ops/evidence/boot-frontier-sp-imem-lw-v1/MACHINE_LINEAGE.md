# Machine lineage

## Synthetic known-word commit

`USER_DECISION` Generated tests may stage source-backed bytes only through a
Machine-owned test seam.

`RUNTIME_FACT` The proved chain is:

generated test word
-> Machine test staging
-> SP IMEM bytes plus `GeneratedMachineTestStaging` provenance
-> direct CPU address classification at `0xA4001000`
-> aligned big-endian word extraction
-> `Lw` destination mutation
-> `KnownInstructionResult` GPR lineage
-> one control-flow commit and one Count increment
-> `MachineRepresentedStepOutcome::LoadWordCommitted`

## Authentic rejection

`RUNTIME_FACT` The authentic chain is:

private cartridge bytes
-> Machine-owned cartridge bootstrap staging in SP DMEM
-> authentic `SpecialAdd` commit and known r9 lineage
-> `Lw` effective address `0xFFFFFFFFA4001000`
-> SP IMEM offset `0x000`
-> first consumed byte has `Unknown` provenance
-> rejection before mutation
-> deterministic boot-probe frontier

`UNKNOWN` The missing cause is a source-clear represented creator for the
post-PIF SP IMEM word. The probe does not author it and the product does not
guess it.
