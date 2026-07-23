# Synthetic cartridge runtime and D-cache completion

This evidence records one public, deterministic, no-window composition from the
existing cold x105 bootstrap through a complete self-checking cartridge
program. Every guest transition uses public `Machine::step`.

The program executes its first word, exercises KSEG0 cached `Sw`, `Sb`, `Lw`,
and `Lbu`, forces three genuine dirty replacements, writes an uncached KSEG1
success mailbox, and completes two iterations of its stable success loop.
Failure code executes zero times.

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

Synthetic milestone: `SYNTHETIC-CARTRIDGE-RUNTIME-COMPLETE`.

Authentic checkpoint remains `BOOT-2`. This is public generated test input, not
authentic cartridge execution or a compatibility claim.
