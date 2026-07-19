# Annulled-slot effect boundary

Focused proofs put GPR writes, concrete SP-IMEM writes, represented device
writes, unaligned loads and stores, and an unsupported identity at `P+4`.
When BEQL is not taken, none is observed: no destination changes, memory or
device mutation, exception, rejection, Count cadence, or committed-step entry
exists for the nullified word.

Nullification is not a Nop execution, pseudo-commit, hidden step, or temporary
delay context.
