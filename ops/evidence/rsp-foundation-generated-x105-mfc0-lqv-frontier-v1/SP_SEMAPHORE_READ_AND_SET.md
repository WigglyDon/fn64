# SP Semaphore Read And Set

`Sp` owns one source-defined semaphore bit and its provenance. Construction
and reset establish clear/zero.

`Mfc0 SP_SEMAPHORE` atomically:

- captures the old bit;
- returns 0 when old clear or 1 when old set;
- sets the semaphore;
- records the instruction PC, fetched-IMEM source, old bit, and prior
  semaphore source;
- writes the result to `rt` unless `rt` is r0.

A repeated read returns one and remains set. An r0 destination still performs
the read-and-set. Any rejection preserves both the bit and its provenance.
