# Break reset and bootstrap lifecycle

Construction, reset, and complete bootstrap use the existing `Sp`, `Mi`, and
RSP owner replacement paths and clear stale Break provenance. Repeated complete
bootstrap is identical.

Failed bootstrap preserves complete prior Break-related `Sp`, `Mi`, RSP, DPC,
and Machine truth. Independent Machines hold independent status, pending, and
provenance state; no global mutable state is introduced.

