# RDRAM DEVICE_ID next frontier

At PC `0xA4000130`, word `0xAD490004` decodes as Sw r9,4(r10). The old r9
value is `0xFFFFFFFF80000000`, produced by the LUI at `0xA400012C`; the low
transfer word is `0x80000000`. The effective, CPU, and physical addresses are
`0xFFFFFFFFA3F80004`, `0xA3F80004`, and `0x03F80004`. It rejects as
`DirectTargetMiss` with complete preservation. DEVICE_ID semantics are not
implemented or interpreted.
