# Exact First Manual Value Policy

The first nominal current-control input is zero. The generated WriteCC path
masks it, XORs `0x3F`, scatters code `0x3F` as `0x00C0C0C0`, and combines it
with manual-path base flags `0x46000000`, yielding `0x46C0C0C0`. Nominal input
zero is evidence and CPU provenance, not a second stored field.
