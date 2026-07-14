# Generated x105 composition

Inputs are a generated 1,984-byte PIF-shaped buffer, generated cartridge
bytes, NTSC_PINNED, X105, cold reset, cartridge medium, explicit PIF-version
bit zero, and independently field-encoded instruction words.

The first nineteen commits retain the accepted sequence through the MTC0 trio
and RI-base `Lui`. Starting at `PC=0xA400008C`, `next_pc=0xA4000090`, and
`Count=3`:

20. `Lw` reads RI_SELECT zero into r9; Count 4.
21. `Bne` compares r9 with r0, is untaken, and schedules one slot; Count 5.
22. `SpecialSll` encodes the generated NOP slot and commits the cold
    fall-through; Count 6.
23. `Addiu` changes sp from `0xFFFFFFFFA4001FF0` to
    `0xFFFFFFFFA4001FD8`; Count 7.
24-28. Five `Sw` commits save s3-s7 at SP-IMEM locals `0xFD8`, `0xFDC`,
    `0xFE0`, `0xFE4`, and `0xFE8`; Count 12.
29-32. Four `Lui` commits construct RI, RDRAM-global, RDRAM-unit, and MI
    addresses; Count 16.
33. `Ori` constructs generated RI_CONFIG current-control-auto value `0x40`;
    Count 17.

The generated BNE's nonselected synthetic target is `0xA40000D0`; its cold
fall-through is `0xA4000098`. At the stop, `PC=0xA40000C4`,
`next_pc=0xA40000C8`, `Count=17`, and 33 instructions have committed.

The next word is `Sw` to CPU `0xA4700004`, physical `0x04700004`, RI_CONFIG
offset `0x04`. It returns `MachineStoreWordRejectionReason::DirectTargetMiss`
with the complete pre-step state preserved. This remains synthetic-only proof.
