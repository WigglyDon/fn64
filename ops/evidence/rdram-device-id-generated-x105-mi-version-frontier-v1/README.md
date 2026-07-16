# RDRAM DEVICE_ID generated x105 MI_VERSION frontier v1

This evidence records one bounded Rust product increment:

- exact global/broadcast `RDRAM_DEVICE_ID` `Sw` accepts low word `0x80000000`;
- `Machine` records a request for physical base `0x02000000` with CPU provenance;
- accepted RDRAM delay and refresh-row facts remain unchanged;
- RDRAM bytes and address routing remain unchanged;
- generated public `Machine::step` composition reaches unsupported `MI_VERSION`.

Decisions: `RDRAM_DEVICE_ID_EXACT_X105_BROADCAST_WORD_ONLY`, `RDRAM_DEVICE_ID_REQUESTED_BASE_ADDRESS_FACT_ONLY`, `RDRAM_DEVICE_RELOCATION_EFFECT_UNAVAILABLE`, and `MI_VERSION_READ_NEXT_FRONTIER_ONLY`.

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`. This is not authentic firmware or cartridge execution and earns no checkpoint beyond BOOT-2.
