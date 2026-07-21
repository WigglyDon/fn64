# Primary-cache initialization, IPL3 relocation, and PI frontier

This evidence records one synthetic public Machine::step composition from the
accepted x105 cache frontier at PC 0xA4000400 through cold primary-cache
initialization, SP control, the public IPL3 relocation, and relocated KSEG0
execution. It stops before the first PI MMIO store at PC 0x8000001C.

The composition begins at Count 246,984 and total commit 247,000. Exactly
5,367 further public steps commit, leaving Count 252,351 and total commit
252,367. No proof-side guest mutation, RSP execution, PI execution, private PIF
input, or commercial ROM input occurs.

The resulting synthetic milestone is
GENERATED-CACHE-INIT-AND-IPL3-RELOCATION-COMPLETE. Authentic cartridge
checkpoint BOOT-2 is unchanged.
