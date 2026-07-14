# RI lifecycle

- Construction and general `Machine::reset` leave RI_SELECT, RI_CONFIG, and
  RI_CURRENT_LOAD unavailable.
- Complete supported cold-x105 bootstrap creates RI_SELECT zero from
  `ColdX105Entry` and leaves RI_CONFIG and RI_CURRENT_LOAD unavailable.
- RI_CONFIG `Sw` creates only configuration state.
- RI_CURRENT_LOAD `Sw` requires configuration and creates the event snapshot.
- Repeating complete cold bootstrap recreates RI_SELECT and clears stale
  configuration/event state and provenance.
- Failed bootstrap planning preserves all prior RI, CPU, COP0, memory, PC,
  Count, and delay-slot facts.
- Independent Machines own independent RI facts and provenance.

No NMI, warm-reset retention, or hardware-time lifecycle is represented.
