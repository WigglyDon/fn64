# Break run-start preservation

Break preserves the accepted consumed run-start lineage. It does not fabricate
a new start event, consume an additional start token, or create task-completion
state.

CPU SP-PC writes and later SP_STATUS commands retain their existing ownership
and lifecycle rules. Break provenance is historical instruction truth; it is
not a replacement for run-start ownership.

