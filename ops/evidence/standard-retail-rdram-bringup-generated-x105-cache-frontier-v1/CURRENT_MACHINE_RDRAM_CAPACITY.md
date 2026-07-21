# Current Machine RDRAM Capacity

The existing Machine-owned RDRAM backing is exactly `0x00400000` bytes. No
resize was performed. That owned length selects the immutable 4 MiB profile:
two present modules of `0x00200000` bytes each.

The profile constructor also defines the source-clear 8 MiB alternative as
four 2 MiB modules and rejects every other capacity. There is no host selector;
the current product still constructs only its existing 4 MiB backing.
