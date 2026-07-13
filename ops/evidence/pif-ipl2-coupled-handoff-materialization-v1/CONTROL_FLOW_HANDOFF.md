# Control-flow handoff

The final IPL2 transfer computes t3 as SP DMEM plus `0x40`, executes `jr t3`,
and completes its delay instruction before the x105 entry. Therefore the
materialized entry is PC `0xA4000040`, next PC `0xA4000044`, with no active
delay-slot context.

The retained ra is older lineage: the final NTSC IPL2 `bal` at
`0xA4001548` links `PC + 8 = 0xA4001550`. `VerifyAndRunIPL3` does not replace
ra before its final jr. The x105 branch at `0xA4000074` consumes only the
negative signed relation, but the Machine retains the complete sign-extended
value and its link provenance.

No trace replay, host jump, or executed IPL2 is involved.
