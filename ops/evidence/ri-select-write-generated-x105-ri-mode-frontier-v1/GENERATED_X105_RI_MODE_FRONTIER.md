# Generated x105 RI_MODE frontier

`RUNTIME_FACT`: independently encoded generated instructions reproduce the
accepted 32,037-commit prefix at PC/next-PC
`0xA40000E4 / 0xA40000E8`, Count 32,021. Commit 32,038 stores r9 low word
`0x00000014` to RI_SELECT physical `0x0470000C` and replaces its cold source
with CPU-store provenance.

After the commit, PC/next-PC are `0xA40000E8 / 0xA40000EC`, Count is 32,022,
RI_CONFIG remains input zero/enable true, and RI_CURRENT_LOAD remains its
stored event. The next independently encoded instruction is `Sw r0,0(r8)` at
PC `0xA40000E8`, targeting CPU `0xA4700000`, physical `0x04700000`, RI_MODE.
It rejects as `DirectTargetMiss` with complete pre-step state preserved.

This is synthetic public-`Machine::step` composition only, not authentic
firmware or cartridge execution.
