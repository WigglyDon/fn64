# MI lifecycle

- Construction and general reset: MI initialization state unavailable.
- Complete cold-x105 bootstrap before CPU stepping: unavailable.
- Exact CPU write: state and provenance become available.
- Repeated complete cold-x105 bootstrap: stale state and provenance clear.
- Failed bootstrap: the complete prior Machine, including MI, is preserved.
- Independent Machines: values and provenance are independent.

No reset value, NMI retention, warm-reset retention, ticking, or timing state
is fabricated.
