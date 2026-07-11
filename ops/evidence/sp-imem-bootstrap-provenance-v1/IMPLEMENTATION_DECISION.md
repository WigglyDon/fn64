# Implementation decision

Selected decision: **USER_SUPPLIED_FIRMWARE_REQUIRED_FUTURE_LANE**

- `INFERENCE` The source event is identified: IPL1 copies proprietary IPL2 code
  bytes from PIF ROM to SP IMEM and the x105 IPL3 prelude consumes them as
  data.
- `INFERENCE` Exact values matter immediately. The first `Lw` writes the full
  firmware word to r8, and the following prelude combines full words with IPL3
  data before storing them back.
- `USER_DECISION` fn64 may not embed one word, an eight-word prefix, a complete
  IPL2 image, an encoded table, or copied emulator state.
- `WORKER_CLAIM` A separately authorized future lane is therefore required if
  authentic progress is to execute or otherwise consume user-supplied PIF
  firmware.

Rejected decisions:

- `GENERAL_MACHINE_OWNED_HLE_EFFECT`: rejected because there is no exact
  firmware-independent state effect at this frontier. Fast-forwarding the
  x105 IPL3 prelude would also simulate cartridge instructions instead of
  executing them through `Machine::step`.
- `SOURCE_CLEAR_BOOT_PROFILE`: rejected because x105 bootcode identity does
  not supply the PIF-region/revision-specific firmware words. A profile holding
  those values would be a disguised firmware image, and current Machine
  construction has no explicit PIF variant choice.
- `EVIDENCE_ONLY_PARTIAL`: rejected as the decision-tree label because the
  more specific future requirement is established. The current commit shape
  is nevertheless evidence-only and the Worker result is PARTIAL.

Future boundary request, not implementation authority:

- `WORKER_CLAIM` Host may someday own an explicit user-selected firmware path
  and file read only.
- `WORKER_CLAIM` Machine must own the resulting validated firmware bytes,
  explicit console/PIF variant configuration, reset mapping, execution, SP
  IMEM mutations, and provenance.
- `WORKER_CLAIM` Unsupported or absent firmware must remain explicit. Filename,
  title, cartridge ID, and whole-ROM digest must not select behavior.
- `UNKNOWN` The minimum coherent device/execution scope for real IPL1/IPL2 is a
  future architecture decision; this lane does not add a speculative PIF path,
  generic bus, SP DMA layer, or firmware loader.
