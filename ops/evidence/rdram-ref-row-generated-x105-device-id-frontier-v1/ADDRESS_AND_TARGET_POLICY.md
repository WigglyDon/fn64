# Address and target policy

- Physical target: `0x03F80014`.
- KSEG0 alias: `0x83F80014`.
- KSEG1 alias: `0xA3F80014`.

Only the exact aligned direct address classifies as REF_ROW. Non-global
REF_ROW, REF_INTERVAL, neighboring RDRAM registers, and DEVICE_ID remain direct
target misses. Unaligned addresses enter the existing AdES path first.
