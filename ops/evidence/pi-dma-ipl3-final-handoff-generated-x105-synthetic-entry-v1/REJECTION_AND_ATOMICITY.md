# Rejection and atomicity

Focused complete-state proofs cover PI_WR_LEN before either programmed address,
unsupported length, unsupported cartridge domain, out-of-range cartridge and
RDRAM spans, upper PI_DRAM_ADDR bits, unknown source lineage, unaligned PI
access, unsupported PI_STATUS reset, and unsupported PI_RD_LEN. They also
cover unavailable KSEG0 D-cache backing and unsupported final control words.

All non-exception rejections occur before PC, next_pc, Count, GPR lineage,
device, cache, memory, interrupt, reservation, or host mutation. AdEL and AdES
continue through the existing exception owner with exact BadVAddr, EPC, BD,
and no normal faulting-instruction cadence.

Successful device commands and PI DMA commit once through the immutable store
plan/application seam. The generated final path has no rejected or partially
committed step.
