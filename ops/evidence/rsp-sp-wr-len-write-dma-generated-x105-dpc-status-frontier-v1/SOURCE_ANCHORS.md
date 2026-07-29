# Source anchors

- Nintendo Ultra64 RSP Programmer's Guide, DMA section and SP control-register
  table: <https://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf>
- Public x105 source pinned at commit `928f59089c18a95cbffa59938a18fa6032c5d78c`:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>
- Pinned RCP register definitions:
  <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h>

The guide identifies control index 3 as DMA write length, selects IMEM with
SP memory-address bit 12, and defines the length/count/DRAM-skip fields. The
repository's existing SP owner remains authoritative for masks and register
evolution.
