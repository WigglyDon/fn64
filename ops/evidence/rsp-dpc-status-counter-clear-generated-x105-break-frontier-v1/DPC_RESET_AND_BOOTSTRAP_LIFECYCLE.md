# DPC reset and bootstrap lifecycle

- Construction: all counters Unavailable.
- Reset: all counters restored Unavailable.
- Complete bootstrap: all counters restored Unavailable.
- Repeated bootstrap: identical state; stale clear provenance removed.
- Failed bootstrap: full previous Dpc state and provenance preserved.
- SP PC writes and halt/run-start transitions: Dpc preserved.
- Independent Machines: independent Dpc state.

The full Machine snapshot used by atomicity tests includes every counter state.
