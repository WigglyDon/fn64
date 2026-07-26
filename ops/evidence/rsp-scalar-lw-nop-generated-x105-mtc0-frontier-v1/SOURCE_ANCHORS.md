# Source Anchors

- Nintendo Ultra64 RSP Programmer's Guide:
  <https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf>
  defines scalar `LW` with a sign-extended 16-bit offset, low-12-bit DMEM
  addressing, a 32-bit scalar result, no listed architectural exception, and
  load interlocking. It also defines `NOP` as no register/internal-state
  modification.
- Pinned public x105 IPL3 source:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>
  directly supplies the bounded sequence at local `0x00C..0x024`.

Only the public words needed for this boundary are recorded:
`0x8C040040`, `0x00000000`, `0x00000000`, and `0x40800000`.
No complete public RSP program or private input is copied here.
