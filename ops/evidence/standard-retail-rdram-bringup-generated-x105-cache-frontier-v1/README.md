# Fixed Standard-Retail RDRAM Bring-up Evidence

This evidence records one synthetic, generated x105 composition executed only
through public `Machine::step`. Starting at PC `0xA4000A2C`, the represented
Machine completes the guest current-control loops, discovers two fixed-profile
2 MiB modules, configures and maps them, records `RI_REFRESH`, writes detected
size `0x00400000`, tears down the RDRAM frame, and stops before executing the
cache-specific `MTC0 zero,C0_TAGLO` word at `0xA4000400`.

The result is the explicit fn64 profile
`fixed-standard-retail-4mib-two-module-digital-cc-v1`. It is deterministic
digital Machine truth, not analog or electrical hardware-conformance evidence.
The authentic cartridge checkpoint remains BOOT-2 and no compatibility claim
is made.

