# Exceptions And COP0

Context role: represented exception/COP0 context.
Scope: Rust COP0 subset and sealed exception-entry ownership.
Canonical for: exception mutation lineage and authority limits.
Not canonical for: a general exception framework or full COP0 behavior.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and focused core tests.
Update triggers: represented COP0 fields, exception source, entry, or return ownership changes.

CPU/COP0 owns represented Status, Cause, EPC, BadVAddr, Count/Compare, and
sealed mutation primitives. Machine action application classifies the event and
delegates to that owner; producers must not enter exception state.

Lineage is `faulting cause/address → source-specific plan → control-flow preservation/rollback → sealed exception entry → PC/next-PC and represented COP0 fields → outcome evidence`.
Arithmetic overflow does not invent BadVAddr. Fetch AdEL writes only its sealed
fields. Count and normal cadence do not advance on represented fault entry.

Forbidden authority includes full COP0 claims, TLB/MMU, generic all-future
exception objects, host interruption, real timing, PIF boot, and inferred
behavior from retired-source names. Numerical exception codes and bits are
explicit in source; no serialization compatibility is promised.

Accepted proof is focused state-transition testing and the overflow/fetch-AdEL
probe cases. Current observability is the public read-only CPU surface. Full
interrupt delivery, ERET integration, delay-slot exception fidelity, and
performance remain bounded by the detailed capability ledger and public step
selection; do not infer them from readiness code.

Required validation: `./rust/verify-forward` plus the narrow exception test.
Next authority requires a bounded source-proven exception source or field.
