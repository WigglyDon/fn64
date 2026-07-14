# Generated x105 wait loop

`RUNTIME_FACT`: independently encoded generated instructions reproduce the
accepted 33-commit prefix. The RI_CONFIG store is commit 34 and produces input
zero, enable true, PC/next-PC `0xA40000C8 / 0xA40000CC`, and Count 18. Generated
`Addiu r17,r0,8000` is commit 35 and leaves Count 19.

The loop at `0xA40000CC` is generated as NOP, `Addi s1,s1,-1`, BNE back to the
loop, and one NOP delay slot. Exactly 8,000 iterations commit 32,000
instructions: 7,999 taken branches, one untaken branch, and 8,000 executed
slots. Final s1 is zero, PC/next-PC are `0xA40000DC / 0xA40000E0`, Count is
32,019, total committed steps are 32,035, and RI_CONFIG is unchanged.

This proves CPU composition and cadence only. It does not relate CPU commits to
RCP cycles or prove analog calibration progress.
