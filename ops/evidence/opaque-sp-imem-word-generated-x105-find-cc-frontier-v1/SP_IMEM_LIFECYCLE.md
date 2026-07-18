# SP-IMEM lifecycle

- Construction and reset: no opaque words.
- Complete cold-x105 bootstrap: concrete profiled replacement, no stale opaque
  words.
- Repeated complete bootstrap: the same replacement clears stale opaque state.
- Failed bootstrap: complete prior bytes, knowledge, and provenance survive.
- Independent Machines: independent `SpImem` owners, no mutable global state.
