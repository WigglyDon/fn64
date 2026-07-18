# Generated InitCC call path

Reached after the new store:

1. `0xA400019C`, word `0x25F5000C`, `Addiu r21,r15,0x000C`; commits and
   produces r21=`0xFFFFFFFFA3F0000C` with instruction lineage.
2. `0xA40001A0`, word `0x0D00021F`, `Jal 0xA400087C`; rejects before mutation
   because r31 has retained PIF IPL2 bootstrap lineage rather than a writable
   generated-instruction lineage.

Architectural link would be `0xFFFFFFFFA40001A8`, but it is not written. The
NOP at `0xA40001A4` is not scheduled or executed. `InitCCValue`, `FindCC`,
`TestCCValue`, and `WriteCC` are source/byte-audited but not Machine-reached.
