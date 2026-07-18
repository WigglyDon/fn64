# RDRAM_MODE next frontier

RDRAM_MODE is not the immediate machine frontier in this pass.

The exact earlier CPU frontier is:

- PC/next_pc: `0xA400099C/0xA40009A0`;
- Count/commits before attempted step: `32208/32224`;
- word: `0x53400018`;
- identity: `Beql r26,r0,0xA4000A00`;
- source r26: `1`, from Slti at `0xA4000998`;
- source r0: `0`, ArchitecturalZero;
- result: `MachineRepresentedStepError::UnrepresentedInstruction`.

Therefore RDRAM_MODE is `NOT YET REACHED DUE TO EARLIER CPU FRONTIER`.

