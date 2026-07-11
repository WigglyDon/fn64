# Implementation decision

Selected decision: **INPUT_BOUNDARY_ONLY_PRODUCT**

- `LIVE_REPO_FACT` Current source has a narrow Machine-owned immutable input
  seam and an existing optional no-window CLI consumer.
- `EXTERNAL TECHNICAL EVIDENCE` The official RCP map describes a 1,984-byte
  Boot ROM and a 2 KiB full PIF map whose remaining 64 bytes are writable RAM.
- `LIVE_REPO_FACT` Current SP IMEM production accepts no firmware source and
  correctly rejects unknown bytes before mutation.
- `INFERENCE` An input-only product is sufficient now because these facts
  support exact ownership, structural validation, lifecycle, and failure
  semantics without inventing firmware effects.

Rejected alternatives:

- `SOURCE_BACKED_BOOT_STATE_MATERIALIZATION`: rejected because the accepted
  provenance evidence leaves the numeric IPL2 source subrange and complete
  `IPL2_length` inside the raw input `UNKNOWN`. Destination consumption
  `[0x000,0x020)` is known, but its exact input slice is not.
- `MINIMAL_IPL1_IPL2_EXECUTION`: rejected because current source has no PIF
  instruction-fetch/device path and the input boundary does not create pressure
  for a second executor, PIF device, SI, bus, or timing framework.
- `EVIDENCE_ONLY_PARTIAL`: rejected because the structural boundary, Machine
  ownership, validation, lifecycle, and host transfer are implementable and
  synthetically provable now.

`UNKNOWN`: exact firmware variant policy and source-backed SP IMEM production.
Absence of an authorized private firmware path affects authentic runtime
validation only; it did not cause the input-only architecture decision.
