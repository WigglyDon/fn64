# PI DMA and generated IPL3 final handoff

This evidence records the bounded product result from canonical
`b9bb59062d50d959ee6c581bc82acb5bf0cf4dff`. One public synthetic x105
composition begins at PC `0x8000001C`, completes the generated PI transfer,
checksum, final device-control writes, boot globals, SP-memory teardown, and
the final `JR` delay slot, then stops at PC `0x80001000` before the synthetic
entry instruction executes.

The result is machine truth only for the represented fixed path: PI completion
is one atomic effect with no timing; D-cache functionality is earned only for
aligned KSEG0 `Lw` from Machine-owned RDRAM; interrupt state is MI-owned; SP
control remains register truth without RSP execution. The cartridge is public
generated test input, not a commercial ROM and not product policy.

Classification: `SYNTHETIC_PUBLIC_MACHINE_STEP_COMPOSITION`.

Synthetic milestone: `GENERATED-IPL3-FINAL-HANDOFF-COMPLETE`.

Authentic checkpoint remains `BOOT-2`; no compatibility claim is made.
