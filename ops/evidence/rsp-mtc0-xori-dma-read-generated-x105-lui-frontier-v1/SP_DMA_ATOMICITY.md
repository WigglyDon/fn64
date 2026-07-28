# SP DMA Atomicity

The shared read-DMA path separates immutable preflight from mutation. It
decodes the read-length word, validates the programmed addresses, proves all
eight RDRAM source offsets, proves all eight DMEM destination offsets, and
checks DMA-record capacity before application.

Only a complete plan may copy bytes, append a record, evolve addresses, and
commit the RSP instruction. Rejections for an unavailable source byte,
out-of-range source or destination, unsupported length, or record-capacity
pressure preserve the complete `Machine`: no partial byte copy, record,
register write, PC advance, count increment, or processor fallback occurs.
