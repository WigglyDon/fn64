# No Generic Branch Or DMA Framework Audit

The product delta contains:

- four exact scalar identities: Lui, Addi, Bltz, Bne;
- one private immutable branch cause inside the existing Sp::rsp owner;
- one owner-derived Mfc0 source: SP_DMA_BUSY;
- reuse of the existing Sp-owned read-DMA planner/applicator.

It does not add:

- a public RSP step API;
- recursive Machine::step;
- a generic processor or branch trait;
- a pipeline or scheduler;
- a generic COP0 bank;
- a second DMA engine;
- a bus, MMIO layer, physical map, device registry, timer, or clock.

Beq, Bgez, J-family control flow, other arithmetic, SP_DMA_FULL, write DMA,
BREAK, and vector arithmetic remain closed.
