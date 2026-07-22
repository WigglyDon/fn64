# PI interrupt ownership

PI completion does not own an interrupt bit. The existing `Mi` owner now
contains concrete pending and mask truth for SP, SI, AI, VI, PI, and DP. The
atomic DMA sets MI PI pending true. PI_STATUS command `0x00000002` at PC
`0x800001D4` clears it and records CPU-store provenance.

The generated final sequence leaves all six pending sources false and all six
mask bits false. Clear provenance is recorded for SP (`0x800001A0`), SI
(`0x800001B4`), AI (`0x800001BC`), DP (`0x800001C8`), and PI
(`0x800001D4`). The mask-clear provenance is PC `0x800001AC`.

This is register truth only. CPU interrupt delivery, MI interrupt reads, VI
behavior, DP execution, SI DMA, and AI DMA are not represented.
