# PI next frontier

The first PI access is deliberately not executed.

| Fact | Value |
| --- | --- |
| PC / next PC | 0x8000001C / 0x80000020 |
| Count / total commits | 252351 / 252367 |
| word / identity | 0xAC290000 / Sw r9,0(r1) |
| base | r1 = 0xFFFFFFFFA4600000 |
| base lineage | LUI at 0x80000014 |
| source | r9 = 0x0000000000001000 |
| source lineage | AND at 0x80000018 from r9 and r10 |
| effective / CPU / physical | 0xFFFFFFFFA4600000 / 0xA4600000 / 0x04600000 |
| target register | PI_DRAM_ADDR |
| transfer low word | 0x00001000 |
| current result | MachineStoreWordRejectionReason::DirectTargetMiss |

Pure store planning preserves the complete pre-PI Machine. PI registers and PI
DMA remain unimplemented.
