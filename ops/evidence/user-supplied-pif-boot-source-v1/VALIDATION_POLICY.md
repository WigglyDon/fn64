# Validation policy

`EXTERNAL TECHNICAL EVIDENCE`: the official RCP map describes a 2 KiB PIF
physical address space split into a 1,984-byte read-only Boot ROM and 64 bytes
of writable PIF RAM.

| State | Owner | Rule |
| --- | --- | --- |
| absent | `Machine` | no `--pif-rom` bytes were transferred; firmware state is `Absent` |
| unreadable | host | the one explicit path cannot be read; no fallback is tried |
| malformed | `Machine` | transferred length is neither the exact 1,984-byte raw Boot ROM nor the specifically named 2 KiB full-address-space layout |
| unsupported | `Machine` | transferred length is exactly 2 KiB and therefore includes the 64-byte writable PIF RAM tail; fn64 does not treat that tail as immutable firmware |
| accepted | `Machine` | transferred length is exactly 1,984 bytes; bytes are owned unchanged and classified `RawBootRom` |

- `LIVE_REPO_FACT` Current validation preserves every byte of an accepted
  1,984-byte candidate; acceptance is structural classification only.
- `LIVE_REPO_FACT` Content, digest, filename, ROM identity, region, and revision
  do not select acceptance or behavior.
- `LIVE_REPO_FACT` Two different generated 1,984-byte patterns receive the
  same classification.
- `UNKNOWN` Authenticity and console/PIF variant compatibility cannot be
  classified without a source-clear non-hash rule. This product deliberately
  does not invent one.
- `WORKER_CLAIM` Acceptance proves only the explicit raw-byte boundary, not
  authentic firmware compatibility or boot execution.
