# Generated x105 composition

The proof uses a generated 1,984-byte PIF-shaped pattern, generated cartridge
storage, `NTSC_PINNED`, x105, cold reset, cartridge medium, explicit PIF version
zero, and independently encoded instruction fields.

The accepted first fifteen identities are reproduced. The next commits are:

16. MTC0 r0,Cause: IP1:IP0 become known zero; Count 15 -> 16.
17. MTC0 r0,Count: Count 16 -> write 0 -> cadence 1.
18. MTC0 r0,Compare: Compare 0; timer clear; Count 1 -> 2.
19. LUI: r8 becomes `0xFFFFFFFFA4700000`; Count 2 -> 3.

At `PC=0xA400008C`, the generated Lw selects virtual `0xA470000C`. It rejects
as a direct target miss, preserving `next_pc=0xA4000090`, Count 3, and all
Machine state. Total committed steps: 19.

This is synthetic composition only, not PIF/IPL execution, cartridge entry,
RI behavior, BOOT-3, or game compatibility.
