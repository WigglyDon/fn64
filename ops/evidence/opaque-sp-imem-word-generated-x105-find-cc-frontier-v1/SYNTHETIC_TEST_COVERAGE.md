# Synthetic test coverage

The focused proof set covers construction, both direct aliases, causal state,
sentinel secrecy, coherence, replacement, known overwrite, explicit opaque
Lw rejection, AdEL/AdES precedence, ordinary and delay-slot cadence, closed
device/SP-DMEM surfaces, pending-transfer conflict, reset/bootstrap/rollback,
independent Machines, equality, and the generated 24-save composition.

The no-window step probe exposes 157 stable cases and adds exact markers for
the opaque saves, concrete saves, and unexecuted FindCC boundary. Final focused,
full, clean-checkout, and canonical counts are sealed in `VALIDATION.md`.
