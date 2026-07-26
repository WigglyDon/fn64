# Scalar-LW Next Frontier

After public LQV, one CPU-selected instruction commits:

- PC `0x8000000C`;
- word `0x3C0A1FFF`;
- identity `Lui r10,0x1FFF`;
- post `PC/next_pc = 0x80000010/0x80000014`;
- CPU Count/commit = `252348/252364`;
- no device or RSP state change;
- next turn RSP.

The next RSP selection sees public word `0x8C040040` at local `0x00C`:
`Lw r4,0x40(r0)`. Base `r0` is available architectural zero; DMEM
`0x040..0x044` is available cartridge-bootstrap truth; old `r4` is
unavailable. The instruction rejects as `ScalarLwUnrepresented`, preserves
the entire Machine including unavailable `v12`, and leaves turn RSP.
