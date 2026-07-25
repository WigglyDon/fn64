# RSP Owner Decision

`Sp` owns one private `MachineRspExecutionState` in `Sp::rsp`.

Ownership remains singular:

- `Sp::pc`: current 12-bit local RSP PC;
- `Sp`: halt, broke, single-step, interrupt-on-break, signals, semaphore,
  memory/DRAM address registers, DMA records, and run-start lineage;
- `Sp::rsp`: scalar availability, next PC, RSP delay context, RSP committed
  count, last-instruction provenance, and unavailable vector/accumulator state;
- `SpImem` and `SpDmem`: their respective bytes and knowledge;
- `Mi`: MI SP pending truth;
- `Machine`: only the private CPU/RSP turn.

There is no `Machine::rsp`, duplicated PC/control fact, generic processor
trait, device registry, or second execution entrance.

