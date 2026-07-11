# Known frontier

- `RUNTIME_FACT` PC: `0xA4000044`.
- `RUNTIME_FACT` Identity: `Lw`; represented operands are `rs=9`, `rt=8`,
  immediate `0xF010` (signed `-4080`).
- `RUNTIME_FACT` Known base r9: `0xFFFFFFFFA4001FF0`, sourced from the preceding
  authentic `SpecialAdd` instruction result.
- `RUNTIME_FACT` Effective 64-bit address: `0xFFFFFFFFA4001000`.
- `RUNTIME_FACT` Effective CPU address: `0xA4001000`.
- `RUNTIME_FACT` Target: SP IMEM, local offset `0x000`.
- `RUNTIME_FACT` Rejection: first unknown byte offset `0x000`; mutation: none.
- `UNKNOWN` Exact post-PIF SP IMEM word and its lawful provenance creator.
- `INFERENCE` The smallest next honest subsystem is a source-backed PIF/reset
  or bootstrap transfer fact that creates those SP IMEM bytes. This follows
  from complete `Lw` semantics reaching the represented storage and failing
  only its provenance preflight.
- `USER_DECISION` Proprietary PIF execution, broad PIF HLE, copied emulator
  state, or guessed bootstrap bytes are outside this lane and were not added.
