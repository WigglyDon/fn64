# Generated x105 JAL sequence

Accepted pre-state:

- PC/next_pc: `0xA40001A0/0xA40001A4`;
- Count/commits: `32169/32185`;
- word: `0x0D00021F`;
- identity: `Jal 0xA400087C`;
- old r31: `0xFFFFFFFFA4001550`;
- old lineage: retained PIF IPL2 link.

Expected committed state:

- r31: `0xFFFFFFFFA40001A8`;
- lineage: JAL at `0xA40001A0`, no GPR sources;
- active delay owner: `0xA40001A0`.

The delay-slot word is `0x00000000` and executes exactly once.
