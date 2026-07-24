# Public source anchors

No user-ROM content was used as product documentation. Architectural decisions
were checked against public sources:

- [R4300 RISC Processor Specification, revision 2.2](https://ultra64.ca/files/documentation/silicon-graphics/SGI_R4300_RISC_Processor_Specification_REV2.2.pdf)
  for primary-cache states and operations, integer semantics, COP0 registers,
  interrupt/exception ownership, ERET, and the four represented TLB
  instructions.
- [Pinned N64 IPL R4300 header](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/R4300.h)
  for public SDK instruction and cache-operation spellings.
- [Pinned N64 IPL RCP header](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h)
  for MI, PI, SP, SI, VI, and AI addresses, masks, DMA fields, and command
  meanings.

The R4300 source establishes a direct-mapped write-back primary D-cache and
the source relationship for Index/Hit invalidate and writeback operations. It
also defines TLB register ownership, TLBR/TLBWI/TLBWR/TLBP, interrupt
recognition through Status/Cause, and ERET control transfer.

The pinned RCP header establishes:

- SP DMA length/count/skip fields and SP PC's low 12-bit field;
- VI vertical-interrupt/current semantics and source-named register masks;
- AI control, DAC-rate, and bitrate fields;
- PI domain timing registers, status, and length register ownership.

No source establishes RSP execution, video/audio presentation, controller
presence beyond the explicit neutral profile, or compatibility. Those truths
remain unavailable.
