# Known frontier

- `RUNTIME_FACT` Execution address: `0xA4000044`.
- `RUNTIME_FACT` Identity: `Lw`; base r9 is known and the sign-extended
  immediate produces effective CPU address `0xA4001000`.
- `RUNTIME_FACT` Target: SP IMEM local offset `0x000`.
- `RUNTIME_FACT` Current outcome: first source byte is `Unknown`; rejection is
  before mutation, with BOOT-2 cadence unchanged.
- `INFERENCE` Actual source category: retained IPL2 firmware instruction bytes
  copied into SP IMEM by IPL1.
- `INFERENCE` Complete immediately consumed firmware range for the x105 prelude:
  `[0x000,0x020)`; complete prelude mutation range: `[0x000,0x02c)`.
- `UNKNOWN` Exact loaded word and provenance cannot become represented without
  user-supplied firmware or an independently authorized lawful source.
- `WORKER_CLAIM` Next product frontier is not another CPU instruction. It is a
  future explicit PIF-firmware input/execution architecture decision, including
  missing post-PIF register state such as the x105 prelude's t3 and ra sources.

Compatibility claim: none.
