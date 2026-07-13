# Aligned Sw generated x105 frontier

This evidence records the source-first selection and generated proof boundary
for one represented instruction family: aligned `Sw` to SP IMEM. The accepted
generated cold-x105 spine reaches `Sw` at CPU address `0xA4000050` after four
committed instructions. Its old `r9` base plus signed immediate `0xF010`
selects CPU address `0xA4001000`, physical SP IMEM `0x04001000`, local offset
zero.

The product scope is deliberately narrow: SP IMEM only, explicit CPU-store
provenance, AdES for unaligned addresses, current cadence and delay-slot
lineage, and fail-closed rejection elsewhere. All bytes and instruction fields
used for proof are generated. This is not authentic IPL execution, cartridge
execution, BOOT-3, or compatibility evidence.
