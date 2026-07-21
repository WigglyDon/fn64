# Source Anchors

Primary source is decompals N64-IPL commit
`928f59089c18a95cbffa59938a18fa6032c5d78c`:

- [x105 IPL3 source](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/src/ipl3.s)
- [RCP register definitions](https://github.com/decompals/N64-IPL/blob/928f59089c18a95cbffa59938a18fa6032c5d78c/include/PR/rcp.h)

Corroboration:

- [Nintendo 64 introductory hardware manual](https://ultra64.ca/files/documentation/online-manuals/man/kantan/step1/2-4.html)
- [SDK RCP header rendering](https://ultra64.ca/files/documentation/online-manuals/man/header/rcp.htm)
- [OKI MSM5718B70 device description, page 19](https://www.alldatasheet.com/html-pdf/11246/OKI/MSM5718B70/1021/19/MSM5718B70.html)
- [OKI MSM5718B70 manufacturer/device material, page 27](https://www.alldatasheet.com/html-pdf/11246/OKI/MSM5718B70/1453/27/MSM5718B70.html)

Relevant pinned-source anchors are `ipl3.s` lines 22-69 for register-word
construction, 268-480 for discovery/finalization/refresh/size storage,
1046-1123 for `InitCCValue`, 1129-1169 for `FindCC`, 1178-1220 for
`TestCCValue`, 1230-1288 for `ConvertManualToAuto`, 1297-1339 for `WriteCC`,
and 1347-1389 for `ReadCC`. `rcp.h` lines 417-418 define MI clear/set RDRAM
register mode and line 804 defines `RI_REFRESH`.

Generated instruction words in the repository fixture, not source-level
pseudo-instruction spelling, are execution authority.
