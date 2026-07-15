# RI lifecycle

- Construction and general `Machine::reset` leave RI_SELECT, RI_CONFIG, and
  RI_CURRENT_LOAD unavailable.
- Complete supported cold-x105 bootstrap creates RI_SELECT zero from
  `ColdX105Entry` and leaves RI_CONFIG/RI_CURRENT_LOAD unavailable.
- RI_CONFIG and RI_CURRENT_LOAD stores preserve the cold RI_SELECT state.
- Exact RI_SELECT `Sw` replaces value with `0x14` and source with
  `CpuStoreWord`, preserving RI_CONFIG and RI_CURRENT_LOAD.
- Repeating complete cold bootstrap restores RI_SELECT zero and its cold source
  while clearing stale RI_CONFIG/current-load state and CPU provenance.
- Failed bootstrap planning preserves all prior RI, CPU, COP0, memory, PC,
  Count, and delay-slot facts.
- Independent Machines own independent values and sources.

No NMI, warm-reset retention, or hardware-time lifecycle is represented.
