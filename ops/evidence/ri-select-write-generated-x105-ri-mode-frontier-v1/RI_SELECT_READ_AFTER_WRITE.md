# RI_SELECT read after write

The existing aligned-`Lw` route reads the stored RI_SELECT state; it does not
derive a value from reset kind or source order. After exact `Sw` stores
`0x00000014`, an exact direct RI_SELECT `Lw` loads the 32-bit word and the
existing sign-extension rule produces 64-bit `0x0000000000000014`.

The destination GPR receives `KnownInstructionResult` lineage from the `Lw`.
The read commits existing PC/next-PC and Count cadence once and has no RI side
effect: the stored value and `CpuStoreWord` source remain unchanged.
