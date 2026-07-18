# Opaque load boundary

Aligned `Lw` from an opaque SP-IMEM word rejects explicitly as
`SpImemWordOpaque` before destination mutation or normal Count cadence. This
also applies to destination `r0`; no unknown destination GPR is created and the
private sentinel is never returned.

Unaligned `Lw` retains AdEL precedence.
