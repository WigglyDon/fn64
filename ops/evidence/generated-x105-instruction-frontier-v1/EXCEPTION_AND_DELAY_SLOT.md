# Exception and delay-slot behavior

Natural word alignment is established before target or source-data access. An
unaligned SP-DMEM-shaped `Lw` therefore enters the existing AdEL path without
requiring a fabricated memory value.

The existing COP0 exception owner supplies:

- exception code `4` (AdEL);
- `BadVAddr` from the represented faulting CPU address;
- sequential EPC, or the owning branch/jump PC when the load is in a delay
  slot;
- Cause.BD from the existing CPU delay-slot context;
- the existing exception vector and EXL policy;
- zero Count advance for the faulting load;
- no destination write and no normal target/fall-through commit.

A successful load in an ordinary delay slot uses the existing single-slot
cadence and clears that context after commit. A blocked exception entry or a
pre-execution rejection preserves the complete pre-step state through the
existing sealed planning boundary. No exception framework was added.
