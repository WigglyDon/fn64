# Link-destination input decision

Decision: `LINK_DESTINATION_PRIOR_STATE_IS_NOT_AN_EXECUTION_INPUT`.

JAL has no GPR source operand. Its target is derived from PC+4 and its link
from PC+8. Old r31 value, knownness, lineage, and bootstrap classification do
not participate in either result.

The pre-step snapshot may retain old r31 for rollback and proof. Snapshot
capture is not an execution prerequisite.

