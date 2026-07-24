# CPU identities added

The user-cartridge path directly executed these newly represented general
identities:

- `Blez` and `RegimmBgez`;
- `Lb`, `Lh`, `Lhu`, `Ld`, `Sh`, and `Sd`;
- `SpecialDiv`, `SpecialDmultu`, and `SpecialDdivu`;
- `Cop0Tlbwi` and `Cop0Eret`;
- `Cop1Cfc1` and `Cop1Ctc1`.

`Cop0Tlbr`, `Cop0Tlbwr`, and `Cop0Tlbp` were implemented as one coherent CP0
TLB-register set and proved with public focused tests. No user-runtime claim is
made for those operations because they are absent from the bounded local
ledger.

Each integer identity consumes complete old source values before writes,
preserves zero-register behavior and alias ordering, and uses the existing
Count and delay-slot owners. DIV/DDIVU/DMULTU own their HI/LO results. Loads and
stores preserve alignment exception precedence and segment-selected cached or
uncached behavior.

No PC, function, instruction-sequence, filename, title, ID, checksum, or digest
whitelist exists.
