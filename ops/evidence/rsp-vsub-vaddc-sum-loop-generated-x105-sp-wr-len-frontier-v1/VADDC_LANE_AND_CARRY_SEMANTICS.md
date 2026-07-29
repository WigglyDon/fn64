# Vaddc Lane And Carry Semantics

For every element-zero lane:

`sum17 = unsigned(vs[i]) + unsigned(vt[i])`

The destination and accumulator-low receive `sum17 & 0xFFFF`. VCO
carry/borrow bit `i` receives bit 16. The upper VCO half becomes Available
zero. Old VCO is not an input. Results are not clamped and no exception is
raised.

Independent synthetic tests cover exact low results, big-endian lane order,
source/destination aliasing, and all 256 carry-byte patterns.
