# RI_SELECT ownership

The existing private per-`Machine` `Ri` owner continues to carry one optional
RI_SELECT fact. Its bounded values are cold-entry zero or exact CPU-written
`0x00000014`; it is not a raw register bank.

The source is either `ColdX105Entry` or `CpuStoreWord` with instruction PC,
source GPR, and prior Machine-owned GPR lineage. A successful CPU store replaces
both value and source. Construction/reset leave RI_SELECT unavailable, complete
cold-x105 bootstrap creates zero, and rebootstrap restores that zero while
clearing stale CPU-store provenance. Independent Machines remain independent.

There is no host setter, global state, general RI value, RI_MODE state, NMI
retention, register array, MMIO trait, bus, or device registry.
