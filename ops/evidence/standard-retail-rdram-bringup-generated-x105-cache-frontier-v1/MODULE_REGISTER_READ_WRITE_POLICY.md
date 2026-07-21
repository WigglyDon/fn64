# Module Register Read/Write Policy

The `Rdram` owner dispatches only named generated-path registers at RCP 2.0
spacing: DEVICE_TYPE/CONFIG (`+0x00`), DEVICE_ID (`+0x04` through the existing
first-responder aperture), RDRAM_MODE (`+0x0C`), RAS_INTERVAL (`+0x18`), and
DEVICE_MANUF (`+0x24`). It is not an arbitrary register array.

DEVICE_TYPE and manufacturer are immutable profile reads. RDRAM_MODE stores one
raw generated-family word per selected module and derives fields. RAS_INTERVAL
accepts the exact fixed-profile word. Register reads require MI register mode.
Absent register apertures do not create modules.

