# RI_CURRENT_LOAD ownership

`LIVE_REPO_FACT`: the existing private per-`Machine` `Ri` owner now carries
three optional facts: RI_SELECT, RI_CONFIG, and the last represented
RI_CURRENT_LOAD update event. The event stores only the consumed RI_CONFIG
input/enable snapshot, the low transfer word as store evidence, and exact CPU
store lineage.

Construction and reset create no event. There is no numeric current-load
register, analog value, output, completion flag, elapsed time, RDRAM-ready
state, host flag, global state, register array, MMIO trait, bus, or device
registry. Public access is immutable observation only.
