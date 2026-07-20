# RDRAM Mode Decision

Decision: `RDRAM_MODE_EXACT_FIRST_MANUAL_REQUEST_ONLY`.

Only known low word `0x46C0C0C0` at physical `0x03F0000C` is accepted. The
Machine records a request, not a physical register response. Every other word
and every other RDRAM_MODE aperture remains closed.
