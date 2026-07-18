# Exact first-responder DEVICE_ID request and current-control frontier v1

This bounded Rust increment represents one exact generated x105 `Sw` of low
word zero to the RCP 2.0 first-responder `RDRAM_DEVICE_ID` aperture. The
Machine-owned fact is an assignment request, not a response or module fact.

Generated public `Machine::step` composition commits the store at step 32,184,
commits the following `Addiu` at step 32,185, and stops atomically at the JAL
to `InitCCValue`. The JAL cannot replace r31's retained bootstrap lineage, so
it rejects before writing its link or scheduling/executing its delay slot.
RDRAM_MODE is therefore not reached.

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`. BOOT-2 remains
the authentic checkpoint; BOOT-3 and compatibility remain unearned.
