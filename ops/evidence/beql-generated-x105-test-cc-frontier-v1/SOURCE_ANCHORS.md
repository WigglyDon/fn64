# Source anchors

Public sources only were used.

- R4300 specification revision 2.2:
  <https://ultra64.ca/files/documentation/silicon-graphics/SGI_R4300_RISC_Processor_Specification_REV2.2.pdf>.
  The branch-delay and EPC/BD rules are in the pipeline and exception sections.
- MIPS R4000 Microprocessor User's Manual:
  <https://techpubs.jurassic.nl/library/manuals/2000/007-2489-001/pdf/007-2489-001.pdf>.
  Appendix A, pages A-25 through A-26, defines opcode `010100`, full-GPR
  equality, `PC+4`-relative signed targeting, one taken delay slot, not-taken
  nullification, and no exception intrinsic to BEQL.
- Pinned public x105 source, commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>.
  Relevant anchors are `InitCCValue` at line 1046, `FindCC` at line 1129,
  `TestCCValue` at line 1178, `CC_MANUAL` at line 68, and `WriteCC` at line
  1297.
- Pinned public RCP header at the same commit:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h>.
  `RDRAM_MODE_REG` is `RDRAM_BASE_REG + 0x0C` at line 102.

No private PIF, BIOS, or cartridge bytes were accessed.
