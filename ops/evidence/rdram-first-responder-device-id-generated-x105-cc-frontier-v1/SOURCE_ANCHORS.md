# Source anchors

Pinned public revision: decompals/N64-IPL
`928f59089c18a95cbffa59938a18fa6032c5d78c`.

- `include/PR/rcp.h:96-104`: RDRAM base `0x03F00000`, DEVICE_ID offset `0x04`,
  MODE offset `0x0C`.
- `include/PR/rcp.h:123`: global aperture offset `0x00080000`.
- `src/ipl3.s:206-219`: `InitialDeviceID` is t6/r14 and is initialized to zero;
  `InitialRegBase` is t7/r15 at the RDRAM base.
- `src/ipl3.s:268-274`: loop1 requests that the first responder take the first
  available device ID, constructs the initial MODE address, then calls
  `InitCCValue`.
- `src/ipl3.s:1046-1123`: `InitCCValue` source body.
- `src/ipl3.s:1129-1169`, `1178-1220`, and `1297-1339`: `FindCC`,
  `TestCCValue`, and `WriteCC` source bodies.

SDK header corroboration:
<https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm>.

The sources identify an intended write. They do not prove responder presence,
assignment completion, module count, per-module readback, or changed routing.
