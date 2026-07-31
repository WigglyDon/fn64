# Break halt and broke semantics

A successful exact Break atomically:

- sets `Sp` halt to true;
- sets `Sp` broke to true;
- preserves single-step;
- preserves interrupt-on-break;
- preserves all eight signal bits;
- preserves semaphore, address state, all three DMA records, and SP memory;
- commits one RSP instruction.

No CPU Count, CPU committed count, VI state, scalar/vector state, DPC state, or
memory byte changes during the RSP-selected call.

