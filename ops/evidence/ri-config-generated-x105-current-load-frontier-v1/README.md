# RI_CONFIG write and generated x105 current-load frontier

Frontier decision: `RI_CONFIG_SW_FRONTIER_CONFIRMED`.

Field decision: `RI_CONFIG_DEFINED_FIELDS_REPRESENTABLE`.

The generated cold-x105 composition reaches aligned `Sw` at CPU address
`0xA4700004`, physical `0x04700004`, with old r8 equal to
`0xFFFFFFFFA4700000` and old r9 equal to `0x40`. Public RI definitions make
bits 5:0 the current-control input and bit 6 the enable. fn64 therefore stores
only those seven defined bits and rejects nonzero undefined high bits before
mutation.

This bounded state does not represent calibration, elapsed hardware time,
RDRAM initialization, or RI_CURRENT_LOAD. All composition bytes are generated
and all instruction words are independently field-encoded. The authentic
checkpoint remains BOOT-2.
