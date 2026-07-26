# LQV Address Calculation

Planning:

1. reads an available scalar base;
2. takes its low 12 bits;
3. sign-extends the encoded seven-bit offset;
4. shifts that signed offset left four;
5. adds with local wrapping arithmetic;
6. retains the low 12 bits.

The represented subset requires element zero and a sixteen-byte-aligned result.
Aligned starts range through `0xFF0`, so the complete sixteen-byte range stays
inside 4 KiB DMEM.

The public operation uses architectural-zero `r0`, offset zero, and resolves
to `0x000..0x010`.
