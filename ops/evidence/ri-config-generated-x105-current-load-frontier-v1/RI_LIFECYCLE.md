# RI lifecycle

- Construction and general `Machine::reset` leave RI_SELECT and RI_CONFIG
  unavailable.
- Complete supported cold-x105 bootstrap creates RI_SELECT zero with
  `ColdX105Entry` provenance and leaves RI_CONFIG unavailable.
- A successful exact RI_CONFIG `Sw` creates its fields and CPU-store source.
- Repeating the complete cold bootstrap recreates RI_SELECT and clears stale
  RI_CONFIG state and provenance.
- Failed bootstrap planning preserves all prior RI, CPU, COP0, and memory
  state.
- Independent Machines own independent RI state and provenance.

No NMI or warm-reset retention rule is represented.
