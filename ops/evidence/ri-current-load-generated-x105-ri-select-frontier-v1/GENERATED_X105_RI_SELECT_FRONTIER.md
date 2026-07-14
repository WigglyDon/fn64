# Generated x105 RI_SELECT frontier

`RUNTIME_FACT`: independently encoded generated instructions reproduce the
accepted 32,035-commit prefix at PC/next-PC `0xA40000DC / 0xA40000E0`, Count
32,019. Commit 32,036 stores r0 to RI_CURRENT_LOAD: the event snapshots
RI_CONFIG input zero and enable true, and leaves PC/next-PC
`0xA40000E0 / 0xA40000E4`, Count 32,020.

Commit 32,037 is independently encoded `Ori r9,r0,0x14`. It produces r9
`0x14`, PC/next-PC `0xA40000E4 / 0xA40000E8`, and Count 32,021. The following
aligned `Sw r9,0x0c(r8)` computes CPU `0xA470000C`, physical `0x0470000C`, and
rejects as `DirectTargetMiss` with the complete pre-step state preserved.

This is synthetic public-`Machine::step` composition only, not authentic
firmware or cartridge execution.
