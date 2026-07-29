# Vector Rejection And Atomicity

Planning resolves fetch, exact identity, element boundary, source state,
accumulator/control invariants, branch state, and destination action before
mutation.

Malformed or nonzero-element Vsub/Vaddc, unavailable Bgez source, malformed
Bgez selector, control flow in an active RSP slot, fetch failure, and the
SP_WR_LEN frontier preserve the complete pre-commit Machine. A rejected delay
slot preserves its already committed branch and post-branch/pre-slot state.
Selected RSP rejection receives no CPU fallback.

No rejected vector operation partially changes its destination, accumulator,
VCO, VCC, VCE, PC, count, turn, or provenance.
