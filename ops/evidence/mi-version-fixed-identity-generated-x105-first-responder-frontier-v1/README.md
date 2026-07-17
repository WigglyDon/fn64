# Fixed MI_VERSION identity and generated x105 first-responder frontier v1

This bounded Rust increment gives each `Machine` one immutable standard-retail
MI_VERSION word, `0x02020102`, owned by its existing `Mi`. One exact direct
aligned `Lw` reads it. Generated public `Machine::step` composition then
builds `0x01010101`, takes the guest CPU's RCP 2.0 branch, selects spacing
`0x400`, builds first-responder base `0xFFFFFFFFA3F08000`, and stops at the
unsupported non-global `RDRAM_DEVICE_ID` store.

Decisions: `MI_VERSION_FIXED_STANDARD_RETAIL_NUS_IDENTITY`,
`MI_VERSION_IMMUTABLE_PER_MACHINE`, `MI_VERSION_EXACT_DIRECT_LW_ONLY`,
`MI_VERSION_RAW_WORD_IS_SINGLE_CANONICAL_TRUTH`,
`MI_VERSION_FIELDS_ARE_DERIVED_READ_ONLY_FACTS`, and
`RCP_2_0_PATH_SELECTED_BY_GUEST_CPU_COMPARISON`.

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`. This is not
authentic firmware or cartridge execution. BOOT-2 remains the checkpoint.
