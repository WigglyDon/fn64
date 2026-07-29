# Vaddc Identity And Decode

Exact represented identity:

- COP2 vector-computation major form `0x12`;
- function `0x14`;
- fields `element`, `vt`, `vs`, and `vd`;
- authorized element: zero only.

Public word `0x4A0E6B54` decodes as `Vaddc v13,v13,v14`, element zero.
Other elements reject before mutation. `Vadd`, `Vsubc`, and all other vector
arithmetic identities remain unsupported.
