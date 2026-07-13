# Generated composition proof

The proof constructs all inputs locally:

- a 1,984-byte PIF-shaped pattern `0xA5 + index * 29` modulo 256;
- explicit `NTSC_PINNED`, `X105`, cold, cartridge, and PIF-version-zero inputs;
- a generated 4 KiB cartridge with a synthetic valid header;
- individually encoded instruction identities, not a copied IPL3 stream;
- generated SP-DMEM data word `0x13579BDF` at cartridge/local offset `0x084`.

Starting from the represented handoff at `pc=0xA4000040`, the public
`Machine::step` path commits:

1. `SpecialAdd`, copying the known stack relation;
2. `Lw` from generated profile-copied SP IMEM at `0xA4001000`;
3. `Lw` from cartridge-staged SP DMEM at `0xA4000084`;
4. `SpecialXor`, producing explicit instruction-result lineage.

The final committed state is `pc=0xA4000050`, `next_pc=0xA4000054`, Count `4`,
and four committed instructions. The independently encoded next identity is
aligned `Sw`; it remains unrepresented and rejects without mutation.

Separate generated cases prove unclassified SP-DMEM rejection and delay-slot
AdEL with EPC `0xA4000040`, BD set, `BadVAddr=0xA4000085`, and no Count advance
for the faulting load. This is synthetic composition proof, not cartridge boot.
