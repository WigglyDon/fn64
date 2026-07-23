# Source anchors

Primary cache semantics were revalidated against the SGI R4300 processor
specification:

- primary D-cache: direct mapped, 8 KiB, 16-byte lines, 512 lines;
- line states: invalid, valid clean, and valid dirty;
- KSEG0 direct addresses are cached;
- KSEG1 direct addresses are uncached;
- dirty victims are written back before replacement.

Primary source:

- <https://ultra64.ca/files/documentation/silicon-graphics/SGI_R4300_RISC_Processor_Specification_REV2.2.pdf>

Repository anchors:

- `rust/crates/fn64-core/src/cpu/cache.rs`: CPU-owned cache state and plans;
- `rust/crates/fn64-core/src/machine.rs`: public-step composition;
- `rust/crates/fn64-core/src/rdram.rs`: backing bytes and writeback provenance;
- `rust/crates/fn64-inspection/src/bin/fn64_step_probe.rs`: no-window proof.

The runtime program is independently encoded original test data. It contains no
commercial ROM, proprietary PIF, BIOS, or game code.
