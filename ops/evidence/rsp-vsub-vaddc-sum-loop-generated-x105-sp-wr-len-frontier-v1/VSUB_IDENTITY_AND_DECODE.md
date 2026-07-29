# Vsub Identity And Decode

Exact represented identity:

- COP2 vector-computation major form `0x12`;
- function `0x11`;
- fields `element`, `vt`, `vs`, and `vd`;
- authorized element: zero only.

Public word `0x4A0D6B51` decodes as `Vsub v13,v13,v13`, element zero.
Element zero maps lane `i` to lane `i`. Other elements reject before mutation.
`Vsubc` and every other vector arithmetic identity remain unsupported.
