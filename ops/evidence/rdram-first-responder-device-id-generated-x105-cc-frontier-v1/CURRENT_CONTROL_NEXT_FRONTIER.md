# Current-control next frontier

Exact first unsupported pressure:

- PC/next-PC: `0xA40001A0/0xA40001A4`;
- Count: `32169`;
- committed steps: `32185`;
- word: `0x0D00021F`;
- identity: `Jal 0xA400087C` (`InitCCValue`);
- existing r31: `0xFFFFFFFFA4001550` with retained PIF IPL2 lineage;
- proposed link: `0xFFFFFFFFA40001A8`, not committed;
- rejection: `BootstrapLinkLineageUnavailable { destination_gpr: 31 }`;
- delay slot: not scheduled or executed.

Source arithmetic predicts the later first manual MODE word `0x46C0C0C0` to
CPU `0xA3F0000C` / physical `0x03F0000C`, but public Machine execution does not
reach it. RDRAM_MODE and calibration remain unimplemented.
