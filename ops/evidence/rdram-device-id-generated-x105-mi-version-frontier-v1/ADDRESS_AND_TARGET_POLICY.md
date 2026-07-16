# Address and target policy

Exact store target: physical `0x03F80004`, KSEG0 `0x83F80004`, KSEG1 `0xA3F80004`. Existing alignment precedes target/value policy. One literal classifier target is added, not a range or map. Direct RDRAM byte routing is unchanged.

There is no DEVICE_ID load. `MI_VERSION` at `0xA4300004` remains an aligned-Lw `DirectTargetMiss`.
