# Current-control source arithmetic

Pinned source predicts the later first manual test:

- nominal a0: zero;
- manual a1: `CC_MANUAL = 2`;
- low-byte mask: zero;
- XOR with `0x3F`: `0x3F`;
- base mode flags: `0x46000000`;
- scattered CC bits: `0x00C0C0C0`;
- predicted transfer: `0x46C0C0C0`;
- predicted target: CPU `0xA3F0000C`, physical `0x03F0000C`.

These remain source-level predictions only. Machine execution stops earlier
at Beql and does not reach TestCCValue, WriteCC, or RDRAM_MODE.
