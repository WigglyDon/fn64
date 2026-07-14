# RI_CONFIG ownership

`LIVE_REPO_FACT`: the existing private per-`Machine` `Ri` owner now carries
two optional, independent facts: RI_SELECT and RI_CONFIG. RI_CONFIG stores only
the defined current-control input and enable fields plus CPU-store lineage. It
does not store a fabricated raw register word, another RI register, calibration
state, elapsed time, or a host flag.

The only public addition is immutable observation of the represented fields and
source. There is no mutable RI surface, register array, numeric dispatch table,
MMIO trait, bus, or device registry.
