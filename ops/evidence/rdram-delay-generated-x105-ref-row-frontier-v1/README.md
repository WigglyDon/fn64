# RDRAM delay generated x105 REF_ROW frontier v1

This evidence records one synthetic public `Machine::step` composition increment.
The accepted x105 MI initialization write arms one exact private transfer; the
generated global RDRAM_DELAY store consumes it and creates one Machine-owned
broadcast configuration fact. The next global RDRAM_REF_ROW store remains
unsupported. This is not authentic firmware, cartridge, timing, readiness, or
per-module RDRAM proof.

Decisions:

- `EXACT_X105_MI_INIT_TO_RDRAM_DELAY_PAIR_ONLY`
- `RDRAM_DELAY_BROADCAST_CONFIGURATION_FACT_ONLY`
- `POST_TRANSFER_MI_READBACK_UNAVAILABLE_UNLESS_PRIMARY_SOURCE_PROVES_EXACT_STATE`

