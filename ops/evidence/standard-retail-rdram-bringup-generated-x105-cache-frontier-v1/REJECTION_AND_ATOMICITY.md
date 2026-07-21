# Rejection and Atomicity

Planning completes source-knownness, effective address, alignment, destination,
MI mode/transfer requirements, generated-family mode validation, module
presence, fixed RAS word, and exact profile-derived refresh word before
mutation. Existing AdEL/AdES and delay-slot EPC/BD owners remain unchanged.

Unavailable sources, unsupported words, disabled register mode, absent named
registers, unsupported addresses, partial writes into opaque words, and
unrepresented identities reject atomically. Absent-module calibration writes
are explicit no-response hardware events only while that mapped absent probe is
active; they create no module and mutate no backing bytes.

No failure path advances normal Count or loses active delay context. Snapshot
tests include complete RDRAM profile/module state, MI mode, RI refresh, concrete
and opaque SP memory, CPU, devices, reservations, cartridge, and host-owned
absence.
