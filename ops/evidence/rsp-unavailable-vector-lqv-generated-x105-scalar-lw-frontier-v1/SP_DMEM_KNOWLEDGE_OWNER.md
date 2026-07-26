# SP DMEM Knowledge Owner

`SpDmem` remains the only owner of DMEM backing bytes, per-byte availability,
per-byte provenance, replacement, CPU-store effects, and represented DMA
effects. `Machine` and `Sp::rsp` observe this state; neither owns a shadow
knowledge map or a second byte array.

`MachineSpDmemByteKnowledge` is either:

- `Available { value, source }`; or
- `Unavailable { source }`.

Available sources are cartridge bootstrap, CPU store word, represented SP DMA,
and test-only generated-machine staging. Unavailable sources distinguish
construction/reset storage from storage omitted by a bounded bootstrap
replacement.
