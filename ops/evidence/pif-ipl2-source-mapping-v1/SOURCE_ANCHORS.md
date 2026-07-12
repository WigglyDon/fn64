# Source anchors

No source passage or firmware content is reproduced here. Each entry maps a
claim to a public, reviewable anchor.

## Official address and capacity facts

- Claim: the PIF Boot ROM occupies 1,984 bytes at physical
  `0x1FC00000..0x1FC007BF`, and SP IMEM occupies
  `0x04001000..0x04001FFF`.
  - Source ID: `SRC-OFFICIAL-RCP-1998`
  - Revision: SGI/Nintendo `rcp.h` Revision 1.21, 1998-07-31
  - Anchor: [RCP Boot ROM and SP memory definitions](https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm)
  - Status: direct
  - Variant scope: architecture address map
  - Limitation: no IPL1 copy range or region rule
- Claim: RSP IMEM and DMEM are each 4 KiB.
  - Source ID: `SRC-OFFICIAL-PROGMAN-52`
  - Revision: Nintendo 64 Programming Manual Version 5.2; publication date
    `UNKNOWN`
  - Anchor: [Chapter 3 section 3.3](https://jrra.zone/n64/doc/pro-man/pro03/03-03.htm)
  - Status: direct
  - Variant scope: architecture-wide
  - Limitation: no boot-copy mapping

## Pinned matching reconstruction

- Claim: IPL1 performs a direct aligned copy from `ipl2_rom` through
  `ipl2_rom_end`, starting at SP IMEM, then transfers control to SP IMEM.
  - Source ID: `SRC-RE-DECOMPALS-928F`
  - Revision: commit `928f59089c18a95cbffa59938a18fa6032c5d78c`
  - Anchor: [`src/pifrom.s` lines 96-124](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/pifrom.s#L96-L124)
  - Status: direct for the pinned reconstruction
  - Variant scope: common IPL1 in the pinned NTSC, PAL, and MPAL builds
  - Limitation: not universal physical-revision proof
- Claim: NTSC has `0xA4` bytes after `ipl2_rom_end`, while PAL and MPAL have
  `0xA0`, within a `0x7C0` total.
  - Source ID: `SRC-RE-DECOMPALS-928F`
  - Revision: same pinned commit
  - Anchor: [`src/pifrom.s` lines 668-682](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/pifrom.s#L668-L682)
  - Status: direct
  - Variant scope: the three pinned regional build targets
  - Limitation: physical hardware revision identities remain `UNKNOWN`
- Claim: IPL2 derives region/reset/security state, stages cartridge IPL3 in SP
  DMEM, performs checksum/PIF interactions, and transfers at `0xA4000040`.
  - Source ID: `SRC-RE-DECOMPALS-928F`
  - Revision: same pinned commit
  - Anchors: [`src/pifrom.s` lines 125-253](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/pifrom.s#L125-L253) and [lines 589-666](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/pifrom.s#L589-L666)
  - Status: direct for the pinned reconstruction
  - Variant scope: regional choices are conditional in the pinned builds
  - Limitation: public reconstructed behavior does not authorize fn64 execution
- Claim: the x105 prelude reads and rewrites retained low SP IMEM, consumes
  staged DMEM relative to `t3`, and branches using `ra`.
  - Source ID: `SRC-RE-DECOMPALS-928F`
  - Revision: same pinned commit
  - Anchor: [`src/ipl3.s` lines 70-105](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s#L70-L105)
  - Status: direct for source behavior; current eight-word frontier is separately
    revalidated repository truth
  - Variant scope: x105 build path
  - Limitation: no firmware words are retained in this lane

## Independent observation

- Claim: an NTSC-like debugger observation gives CPU source start
  `0xBFC000D4`, source end exclusive `0xBFC0071C`, and SP IMEM destination
  `0xA4001000`.
  - Source ID: `SRC-DEBUG-PIF12`
  - Revision: unversioned; accessed 2026-07-11
  - Anchor: [Console Protocols PIF Stage 1 and 2](https://sites.google.com/site/consoleprotocols/home/techinfo/lowlevel/pif12)
  - Status: direct observation for endpoints
  - Variant scope: the page's examined common NTSC setup
  - Limitation: its prose byte/instruction count conflicts with the endpoint
    arithmetic and is rejected; the page is not revision-pinned

## Independent emulator implementations

- Claim: one emulator uses boot HLE, materializing regional/register/device
  effects and only a narrow retained IMEM prefix rather than executing IPL1 or
  IPL2.
  - Source ID: `SRC-EMU-MUPEN-9EB6`
  - Revision: commit `9eb6a7cbefe663c0a7c527afc705f5dea5197d7c`
  - Anchor: [`bootrom_hle.c` lines 37-150](https://github.com/mupen64plus/mupen64plus-core/blob/9eb6a7cbefe663c0a7c527afc705f5dea5197d7c/src/device/pif/bootrom_hle.c#L37-L150)
  - Status: direct for implementation behavior; corroboration only
  - Variant scope: emulator-supported TV/CIC combinations
  - Limitation: HLE constants and shortcuts are not hardware specification
- Claim: another emulator stores supplied PIF bytes for CPU-visible execution
  and initializes PIF RAM separately.
  - Source ID: `SRC-EMU-CEN64-E064`
  - Revision: commit `e0641c8452a3ae8edcd2bf4e46794bb4eaafc076`
  - Anchors: [`device/device.c` lines 41-104](https://github.com/n64dev/cen64/blob/e0641c8452a3ae8edcd2bf4e46794bb4eaafc076/device/device.c#L41-L104), [`si/controller.c` lines 43-62](https://github.com/n64dev/cen64/blob/e0641c8452a3ae8edcd2bf4e46794bb4eaafc076/si/controller.c#L43-L62), and [lines 319-329](https://github.com/n64dev/cen64/blob/e0641c8452a3ae8edcd2bf4e46794bb4eaafc076/si/controller.c#L319-L329)
  - Status: direct for implementation behavior; corroboration only
  - Variant scope: emulator implementation
  - Limitation: execution choice neither supplies the regional copy arithmetic
    nor proves execution is required in fn64
