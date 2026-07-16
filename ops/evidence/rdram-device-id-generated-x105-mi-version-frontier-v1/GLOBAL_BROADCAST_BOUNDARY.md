# Global/broadcast boundary

Only physical `0x03F80004` and direct aliases `0x83F80004`/`0xA3F80004` are represented. Non-global `0x03F00004`, adjacent registers, and all other RDRAM-register addresses remain target misses.

The state records a global-aperture write; it does not claim any module received or completed it.
