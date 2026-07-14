# Generated x105 composition

The proof uses one generated 1,984-byte PIF-shaped input, generated cartridge
bytes, `NTSC_PINNED`, x105, cold reset, cartridge medium, and an explicit zero
PIF-version bit. Instruction words are encoded independently from semantic
fields; no authentic instruction stream is copied.

Starting at `pc/next_pc=0xA4000040/0xA4000044`, Count zero, public
`Machine::step` commits:

1. `SpecialAdd`
2. SP-IMEM `Lw`
3. cartridge-staged SP-DMEM `Lw`
4. `SpecialXor`
5. SP-IMEM `Sw`
6. `Addi`
7. `Andi`
8. untaken `Bne`
9. its `Addi` delay slot
10. `Lw`
11. `Lw`
12. `Sw`
13. `Sw`
14. taken `RegimmBltz` on retained r31
15. its aligned `Sw` delay slot from r0

BLTZ leaves r31 and its retained-link provenance unchanged, sets
`pc/next_pc=0xA4000078/0xA400007C`, advances Count to 14, and names
`0xA4000074` as the slot owner. The slot writes four known zero bytes to SP
IMEM local `[0x00C,0x010)` with CPU-store provenance, clears the context,
advances Count to 15, and leaves `pc/next_pc=0xA400007C/0xA4000080`.

The next generated word identifies as `Cop0Mtc0` with rd 13 (Cause). It is
unrepresented and preserves the complete pre-step state. This proof is
synthetic composition only, not PIF/IPL or cartridge execution.
