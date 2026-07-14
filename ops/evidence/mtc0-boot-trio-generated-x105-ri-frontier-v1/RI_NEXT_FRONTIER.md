# RI next frontier

The first post-trio unavailable fact is an aligned Lw from RI_SELECT:

- constructed base: `0xFFFFFFFFA4700000`;
- virtual CPU address: `0xA470000C`;
- direct physical address: `0x0470000C`;
- RI register offset: `0x0C`;
- current result: `MachineLoadWordRejectionReason::DirectTargetMiss`.

No RI state, RI_SELECT value, NMI selector, MMIO route, bus, generalized map,
or device registry was added. Those are distinct product decisions.
