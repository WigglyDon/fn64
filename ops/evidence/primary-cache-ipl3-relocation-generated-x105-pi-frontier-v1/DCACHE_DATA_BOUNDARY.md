# D-cache data boundary

The bounded generated path pressures all 512 primary D-cache tag lines but no
KSEG0 data load or store before the PI frontier. Functional primary D-cache
fill, hit, dirty, and writeback data flow is therefore not earned.

The implemented truth is the direct-mapped geometry, explicit reset-unknown
state, Index Store Tag transition, tag provenance, and the final all-invalid
array. Existing KSEG1 and currently represented uncached data routes remain
unchanged.
