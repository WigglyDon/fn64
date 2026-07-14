# Atomicity and rollback

The coupled bootstrap computes all replacement CPU, SP memory, bootstrap, and
RI facts before assigning them to the Machine. Failed selector, firmware, or
cartridge preflight therefore cannot partially stage RI state.

For `Lw`, base knownness, effective address, alignment, direct translation,
exact target, RI availability, loaded word, and destination result are all
resolved before destination mutation. Unknown base, non-direct address,
direct target miss, unavailable RI_SELECT, and read failure preserve GPRs and
lineage, HI/LO, COP0, RDRAM, SP memories and provenance, RI state, PC, next_pc,
Count, and delay-slot context. Unaligned access enters the existing atomic AdEL
path without touching RI state.

The final generated `Sw` to RI_CONFIG rejects as a direct target miss before
any Machine fact changes.
