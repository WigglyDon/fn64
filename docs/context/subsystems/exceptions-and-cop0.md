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
write, including when the address would otherwise select SP DMEM. Alignment is
decided before target/source access. Count and normal cadence do not advance on
represented fault entry.
An unaligned MI_VERSION `Lw` uses the same sealed AdEL route in ordinary and
delay-slot contexts; it does not mutate immutable identity or destination.
An unaligned `Sw` enters the same sealed data-address-error owner with write
kind, AdES code 5, and exact BadVAddr. Alignment is resolved before store-source
consumption. No SP-IMEM byte or provenance changes, no RI_MODE, RI_CONFIG,
RI_CURRENT_LOAD, RI_SELECT, pending MI-transfer, RDRAM-delay, RDRAM REF_ROW, or
RDRAM DEVICE_ID request
or first-responder DEVICE_ID assignment-request state changes,
no normal cadence commits, and Count advances zero times. The exact RI, MI,
and global RDRAM_DELAY/REF_ROW/DEVICE_ID or exact RCP 2.0 first-responder
DEVICE_ID write routes add no device-specific exception.
Bootstrap unknown-GPR rejection is not an exception: it restores staged
control flow and leaves COP0 and Count unchanged before helper invocation.

The explicit generated-only NTSC cold-x105 handoff is the sole bootstrap path
that sources inherited COP0 state. It stages Status=`0x34000000` with named
IPL1 cold-boot lineage and a clear delay-slot context. Count, Compare, EPC,
BadVAddr, Cause, software/timer pending state, Config, and PRId receive no
handoff-source claim. Missing or unsupported handoff input rejects before any
COP0 or control-flow mutation.

The same source-backed cold-x105 state is the only represented MTC0 access
scope. Cause register 13 writes only IP1/IP0 (`0x00000300`) and makes that
two-bit software-pending state known while preserving exception code, BD, and
timer pending. Count register 9 installs its low word before committed cadence.
Compare register 11 installs its low word and clears timer pending before
committed cadence; the existing post-increment Count/Compare equality owner
may relatch pending. This is pending-state representation only: no interrupt
delivery, masking, or general CP0 register access is implied.

For the represented ordinary-control-flow family, CPU-owned delay-slot context
names the owning branch/jump PC. Arithmetic overflow, instruction-fetch AdEL,
unaligned-`Lw` data-AdEL, and unaligned-`Sw` data-AdES entry from that slot set
Cause.BD, write the owner PC to EPC, leave the faulting slot Count unchanged,
prevent target commit, and clear context after successful entry. A branch/jump
in the slot is unsupported rollback, not an exception.

Non-linking/non-likely BLTZ uses that same owner without adding exception
semantics. Focused generated proof covers taken BLTZ with an AdES slot and
untaken BLTZ with an AdEL slot: EPC is the BLTZ PC, BD is set, BadVAddr is
exact, the branch Count remains, the faulting slot adds zero, and neither
target nor fall-through commits. Bounded `Cop0Mtc0` in an ordinary slot commits
its destination once and then uses the existing slot cadence; it has no
intrinsic represented exception. Other COP0 instructions and destinations
remain unrepresented.

Generated SP-DMEM-shaped delay-slot proof uses fault address `0xA4000085`,
owner EPC `0xA4000040`, Cause.BD set, and zero Count delta for the faulting
load. It reuses the existing exception entry and adds no COP0 field or policy.

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
