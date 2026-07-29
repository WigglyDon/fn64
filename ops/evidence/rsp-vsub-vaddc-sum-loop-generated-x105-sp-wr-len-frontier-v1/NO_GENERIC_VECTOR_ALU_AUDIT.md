# No Generic Vector ALU Audit

The core adds two exact private plan/application paths selected only by exact
Vsub and Vaddc decoding. It adds no operation table, vector-ALU trait, generic
COP2 register bank, generic element mapper, vector store, or other arithmetic
identity.

Nonzero elements, Vadd, Vsubc, and all other vector arithmetic remain explicit
unsupported boundaries.
