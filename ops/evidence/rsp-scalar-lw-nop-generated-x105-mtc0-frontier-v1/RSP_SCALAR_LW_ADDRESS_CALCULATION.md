# Scalar LW Address Calculation

Planning requires an Available scalar base. It consumes the old 32-bit base,
sign-extends the encoded 16-bit offset, adds with wrapping scalar arithmetic,
and retains the low 12 bits as the local DMEM address.

The bounded surface then requires four-byte alignment. Aligned local starts
through `0xFFC` have one complete four-byte range inside the 4 KiB owner.
Misalignment is an explicit unsupported fn64 boundary, not an architectural
trap claim.
