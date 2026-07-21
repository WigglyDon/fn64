# RI_REFRESH Decision

The existing `Ri` owner stores raw `RI_REFRESH` and CPU-store provenance. For
two discovered 2 MiB modules, the guest combines base word `0x00063634` with
module mask `3 << 19`, producing `0x001E3634` at PC `0xA40003B0` from r9.

Derived read-only facts are module mask 3, optimize true, enable true,
refresh-bank false, dirty delay `0x36`, and clean delay `0x34`. The following
generated `Lw` reads the stored raw word. No clock or timing engine exists.

