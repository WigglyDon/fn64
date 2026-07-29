# SP write-DMA source knowledge

All 192 selected public synthetic IMEM bytes are concrete and non-opaque.

- FNV-1a-64 digest: `78c6276297c6c565`
- first sixteen: `25 42 5f 7c 99 b6 d3 f0 0d 2a 47 64 81 9e bb d8`
- final sixteen: `15 32 4f 6c 89 a6 c3 e0 fd 1a 37 54 71 8e ab c8`
- provenance class: deterministic generated PIF-to-IMEM bootstrap bytes with
  exact source offsets

Unavailable, opaque, inconsistent, or out-of-range source truth rejects the
complete transfer before its first destination mutation.
