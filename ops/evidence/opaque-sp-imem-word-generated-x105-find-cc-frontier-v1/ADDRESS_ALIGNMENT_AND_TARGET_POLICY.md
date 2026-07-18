# Address, alignment, and target policy

The rule is destination-owned, not PC-, register-, function-, or x105-owned.
It requires an aligned word and the existing exact SP-IMEM classification.
KSEG0 and KSEG1 direct aliases normalize through the existing policy.

Unknown effective-address bases remain rejected. Unaligned stores enter the
existing AdES path before opaque planning. SP-DMEM and every device target stay
closed to unknown command words.
