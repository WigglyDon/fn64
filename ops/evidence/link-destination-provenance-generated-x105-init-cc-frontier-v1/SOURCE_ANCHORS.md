# Source anchors

CPU source:

- R4300 RISC Processor Specification, revision 2.2, section 3.2.3.2: JALR
  reads `rs`, derives the target from that old value, and independently
  produces PC+8 for the link.
- The same specification identifies r31 as the implicit link register and
  one architectural delay slot.
- Existing accepted fn64 control-flow law applies the J/JAL target region from
  PC+4 and the link from PC+8.

Generated source:

- decompals/N64-IPL pinned at
  `928f59089c18a95cbffa59938a18fa6032c5d78c`.
- `src/ipl3.s:1046-1123`: InitCCValue saves state and calls FindCC.
- `src/ipl3.s:1129-1169`: FindCC initializes nominal CC state and calls
  TestCCValue after its initial branch.
- `src/ipl3.s:1178-1220`: TestCCValue passes manual mode to WriteCC.
- `src/ipl3.s:1297-1339`: WriteCC constructs the RDRAM_MODE word.

The pinned public object audit is accepted only from the converged downstream
region where it matches committed anchors: `0xA400087C/0x27BDFF60`,
`0xA40008F0/0x0D000261`, and
`0xA400099C/0x53400018`. Its drifted earlier main-body schedule is rejected
as generated-byte authority.

