# Atomicity and rollback

Planning captures control flow, validates the base source, computes and checks
the address, classifies the target, validates the source only when the target
is supported, preflights the exact four-byte span, and constructs provenance
before mutation. The resulting application is infallible until cadence.

Production rejection restores the captured control-flow state. AdES restores
that state before COP0 entry. No rejection or exception path changes GPRs,
HI/LO, memory, memory provenance, Count, or accepted inputs. The existing
atomic COP0 entry rejects nested entry without partial exception mutation.
