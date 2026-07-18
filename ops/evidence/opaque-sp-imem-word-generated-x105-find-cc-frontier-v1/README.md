# Opaque SP-IMEM word and FindCC boundary

This evidence records one bounded machine fact: an aligned CPU `Sw` whose
source cause is explicit but whose low 32 value bits are unavailable may commit
only to the existing SP-IMEM owner as one opaque aligned word. It records no
transferred bits and does not advance through the FindCC call.

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

Authentic execution remains BOOT-2. BOOT-3 and game compatibility are not
earned.
