# GPR provenance

Each known entry GPR has one source:

- r0: architectural zero;
- t3: final IPL2 SP-DMEM entry pointer and jr target;
- sp: IPL1 stack setup retained after balanced IPL2 frames;
- ra: NTSC_PINNED final IPL2 branch-and-link, including its instruction address;
- s3: explicit cartridge boot medium;
- s4: explicit NTSC PIF profile TV relation;
- s5: explicit cold reset kind;
- s6: explicit x105 family seed relation;
- s7: explicit PIF version bit plus NTSC regional mask zero.

The earlier general-reset stack-pointer label is replaced by the narrower
restored-IPL2 owner. Instruction results continue to replace destination
provenance through the existing known-instruction-result path.
