# Generated InitCC call path

After the outer JAL and slot, public Machine execution reaches:

- `0xA400087C`, word `0x27BDFF60`, `Addiu sp,sp,-0xA0`, producing
  `sp=0xFFFFFFFFA4001EF0`;
- `0xA4000880`, word `0xAFB00040`, `Sw r16,0x40(sp)`, storing the RCP
  spacing word `0x00000400` at SP-IMEM offset `0xF30`;
- `0xA4000884`, word `0xAFB10044`, `Sw r17,0x44(sp)`, storing the
  first-responder base low word `0xA3F08000` at offset `0xF34`;
- `0xA4000888`, word `0x00008825`, `Or r17,r0,r0`;
- `0xA400088C`, word `0x00008025`, `Or r16,r0,r0`.

The first unsupported pressure is `0xA4000890`, word `0xAFA20000`,
`Sw r2,0(sp)`. Its effective address is `0xFFFFFFFFA4001EF0`, CPU address
`0xA4001EF0`, physical address `0x04001EF0`, and SP-IMEM offset `0xEF0`.
r2 contains zero but has retained `UnknownPifProduced` lineage, so the exact
result is `ValueSourceUnavailable` before mutation. No nested call is reached.
