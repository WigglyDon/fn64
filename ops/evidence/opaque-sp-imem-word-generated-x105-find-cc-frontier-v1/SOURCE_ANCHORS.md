# Source anchors

- Pinned public PIF/IPL2 source: `decompals/N64-IPL` commit
  `928f59089c18a95cbffa59938a18fa6032c5d78c`, `src/pifrom.s`, checksum call,
  `CalcChecksum`, and `VerifyAndRunIPL3`.
- Pinned public x105 source: the same commit, `src/ipl3.s`, `InitCCValue` save
  frame and first `FindCC` call.
- Generated-byte authority: the public `ipl3.X105.o` `.text` image produced at
  that pinned commit and the independently encoded repository fixture.

The public source establishes that checksum-derived `v0` survives the final
IPL2 transfer. It does not establish its value bits without unavailable
PIF/CIC seed and checksum state. No private firmware or ROM input was read.
