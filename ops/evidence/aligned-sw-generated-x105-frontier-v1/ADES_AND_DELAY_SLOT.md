# AdES and delay-slot ownership

The current CPU address owner maps an unaligned write to AdES, Cause exception
code 5. COP0 remains the only exception-state owner. Sequential faults record
the faulting instruction PC in EPC with BD false. Delay-slot faults record the
owning branch/jump PC with BD true. BadVAddr is the exact represented CPU
address.

A faulting store changes no memory or provenance, advances Count zero, does not
commit normal `pc/next_pc`, and does not commit the outer target. Successful
exception entry clears delay-slot context through the existing CPU owner. A
blocked entry leaves the complete pre-step state unchanged.
