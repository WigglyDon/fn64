# Exact BEQL annul and generated TestCC frontier

This evidence records one exact CPU identity and one generated composition
boundary. `BEQL` compares two available 64-bit GPR values. A taken branch uses
the existing single delay-slot owner; a not-taken branch architecturally
nullifies `PC+4` and commits only the branch. The public `Machine::step`
composition then reaches the first non-global `RDRAM_MODE` store as an atomic
`DirectTargetMiss`.

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

No RDRAM_MODE effect, current-control calibration, responder presence, module
discovery, BOOT-3 checkpoint, or compatibility claim is made.
