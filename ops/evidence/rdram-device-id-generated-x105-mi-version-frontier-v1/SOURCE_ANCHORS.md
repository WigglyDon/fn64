# Source anchors

Pinned public sources:

- N64-IPL `include/PR/rcp.h`, commit `928f59089c18a95cbffa59938a18fa6032c5d78c`: <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h>
- N64-IPL `src/ipl3.s`, same commit: <https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s>
- SDK corroboration: <https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm>

`EXTERNAL_TECHNICAL_EVIDENCE`: RDRAM base is `0x03F00000`; DEVICE_ID is base plus `0x04`; the global aperture adds `0x00080000`; the bounded source constructs `0x02000000 << 6`, writes it globally, and describes requested physical base `0x02000000`; MI_VERSION is MI base `0x04300000` plus `0x04`.

`UNKNOWN`: arbitrary DEVICE_ID fields, per-module readback, module presence/count, physical relocation completion, and every MI_VERSION value. No external source text or authentic instruction stream is vendored.
