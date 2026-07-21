# KSEG0 instruction fetch and fill

KSEG1 fetch retains the existing uncached direct path. KSEG0 fetch consults the
CPU-owned primary I-cache.

On a valid matching tag, the requested big-endian word is returned only from
the cached line. On a definite invalid or tag-mismatched line, planning reads
exactly one 32-byte Machine-owned RDRAM line. Machine::step installs that line
before decode and rolls it back if the represented instruction application
rejects. Unavailable reset truth and valid-tag data-unavailable truth reject
before decoding placeholder bytes.

The first relocated request is PC 0x80000004. It misses line index zero, fills
physical RDRAM line 0x00000000..0x0000001F, and returns word 0x3C0BB000.
PC 0x80000008 then hits that same line and executes cartridge-header Lw word
0x8D690008. The I-cache, not the fixture, owns the fetched copy.
