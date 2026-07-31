# No generic device or MMIO audit

The implementation adds one concrete `Dpc` field to `Machine` and one exact
RSP Mtc0 match arm. It adds no device registry, register array, generic bus,
MMIO trait, generalized physical-address map, generic processor framework, or
public RSP-only step.
