# DPC counter state representation

Each counter is independently:

- `Available { value: u32, source }`; or
- `Unavailable { source }`.

Available values must satisfy `value & !0x00ff_ffff == 0`. Unavailable states
store no backing value. A malformed available value is rejected during
command planning before any mutation.
