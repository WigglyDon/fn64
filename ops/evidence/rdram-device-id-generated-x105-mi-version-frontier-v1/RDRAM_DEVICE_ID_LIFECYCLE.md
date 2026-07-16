# RDRAM DEVICE_ID lifecycle

- construction/reset/complete cold-x105 bootstrap before stepping: unavailable;
- exact store: creates or replaces request and provenance;
- repeated complete bootstrap: clears request with owning `Rdram` replacement;
- failed bootstrap: preserves the complete prior Machine;
- independent Machines: independent request facts and provenance.

No unpressured hardware reset value is fabricated.
