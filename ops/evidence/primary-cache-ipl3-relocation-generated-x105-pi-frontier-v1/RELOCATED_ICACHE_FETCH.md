# Relocated I-cache fetch

The generated JR at 0xA4000504 targets 0x80000004. Its delay-slot SP start
store at 0xA4000508 commits once before control arrives at the target.

The first KSEG0 fetch at 0x80000004 fills I-cache line zero from physical
RDRAM line zero, which contains the just-relocated bytes. The requested word is
0x3C0BB000, LUI r11,0xB000. The following word 0x8D690008 is an I-cache hit and
loads the synthetic Machine-owned cartridge header at CPU 0xB0000008.

This proves cached execution of copied public bytes. It is not authentic
firmware execution or cartridge handoff.
