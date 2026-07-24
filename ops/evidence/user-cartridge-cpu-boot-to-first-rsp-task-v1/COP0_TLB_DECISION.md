# COP0 and TLB decision

The existing CPU/COP0 owner now represents Index, Random, EntryLo0, EntryLo1,
Context, PageMask, Wired, EntryHi, EPC, and a private 32-entry TLB array.
`TLBR`, `TLBWI`, `TLBWR`, and `TLBP` operate on that state with source-defined
masks. MFC0/MTC0 expose only the concrete destinations reached by the bounded
runtime.

`ERET` clears EXL and transfers to EPC through the existing `pc` / `next_pc`
model. ERL remains rejected because ErrorEPC was not earned by this path.

The local runtime observed direct-segment execution and an indexed TLB write,
but did not pressure translated instruction or data access before task
submission. Therefore this pass owns TLB register/entry truth, not a broad
translated virtual-memory route.

CP0 Random advances on every committed instruction within the represented
Wired bound. Rejection and exception entry remain atomic.
