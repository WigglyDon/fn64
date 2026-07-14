# Delay slot and exception lineage

Taken and untaken BLTZ both reuse the existing non-likely one-slot lifecycle.
The generated taken slot is aligned `Sw` from r0 using old r9
`0xFFFFFFFFA4001FF4` plus signed immediate `0xF018`. It writes one known
zero word at SP IMEM local `0x00C` with CPU-store provenance, commits once,
then transfers to `0xA400007C`.

Existing AdEL and AdES owners remain unchanged. When a represented slot faults,
EPC is the owning BLTZ PC, BD is true, BadVAddr is the exact fault address,
the BLTZ Count increment remains, the faulting slot advances Count zero, no
slot destination or memory mutation occurs, and no selected target/fall-through
commits. Successful exception entry clears the delay context; blocked entry
preserves the complete pre-step state.
