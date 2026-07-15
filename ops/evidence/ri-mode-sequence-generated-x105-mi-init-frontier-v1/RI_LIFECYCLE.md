# RI lifecycle

- Construction and general reset leave all four represented RI facts
  unavailable.
- Complete cold-x105 bootstrap creates RI_SELECT zero from `ColdX105Entry` and
  leaves RI_CONFIG, RI_CURRENT_LOAD, and RI_MODE unavailable.
- Existing RI_CONFIG, RI_CURRENT_LOAD, and RI_SELECT stores leave RI_MODE
  unavailable.
- A successful RI_MODE store creates or replaces only RI_MODE and its source.
- Repeated complete bootstrap restores cold RI_SELECT, clears RI_CONFIG and
  RI_CURRENT_LOAD, and clears stale RI_MODE state/provenance.
- Failed bootstrap planning preserves all RI, CPU, COP0, memory, PC, Count,
  provenance, and delay-slot facts.
- Independent Machines own independent RI values and provenance.

No warm-reset, NMI-retention, or hardware-time lifecycle is represented.
