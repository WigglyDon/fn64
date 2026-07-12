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
Arithmetic overflow does not invent BadVAddr. Fetch AdEL and aligned-`Lw`
data-AdEL planning write only their sealed fields. An unaligned `Lw` enters the
existing load address-error owner with its exact BadVAddr and no destination
write. Count and normal cadence do not advance on represented fault entry.
Bootstrap unknown-GPR rejection is not an exception: it restores staged
control flow and leaves COP0 and Count unchanged before helper invocation.

For the represented ordinary-control-flow family, CPU-owned delay-slot context
names the owning branch/jump PC. Arithmetic overflow, instruction-fetch AdEL,
and unaligned-`Lw` data-AdEL entry from that slot set Cause.BD, write the owner
PC to EPC, leave the faulting slot Count unchanged, prevent target commit, and
clear context after successful entry. A branch/jump in the slot is unsupported
rollback, not an exception.

Forbidden authority includes full COP0 claims, TLB/MMU, generic all-future
exception objects, host interruption, real timing, PIF boot, and inferred
behavior from retired-source names. Numerical exception codes and bits are
explicit in source; no serialization compatibility is promised.

Accepted proof is focused state-transition testing and the overflow/fetch-AdEL
plus delay-slot-exception probe cases. Current observability is the public
read-only CPU surface. Full interrupt delivery, ERET integration, nested
exception completeness, and performance remain bounded by the detailed
capability ledger and public step selection; do not infer them from readiness
code.

Required validation: `./rust/verify-forward` plus the narrow exception test.
Next authority requires a bounded source-proven exception source or field.
