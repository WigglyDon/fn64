# Synthetic test coverage

Implemented product proof:

- independent explicit input installation and reset persistence;
- complete NTSC GPR values and sources;
- PIF version zero and one;
- PAL and MPAL fail-closed without partial mutation;
- missing firmware, profile, family, reset kind, medium, and version;
- malformed and unsupported PIF replacement preservation;
- exact Status and no active delay context;
- repeated bootstrap and independent Machines;
- generated instruction consumption of a coupled GPR;
- generated copied SP-IMEM Lw and existing provenance;
- all unstaged GPRs remain source-Unknown;
- no fabricated Count, Compare, EPC, BadVAddr, or timer provenance;
- CLI parsing and no-search behavior using temporary generated files.

Focused filters `cold_x105`, `cartridge_bootstrap`, `boot_probe`, and the
direct `boot_probe_cli` target exercise these rules. The complete gate covers
the existing SP IMEM, aligned `Lw`, ordinary-control-flow, reset, and
Machine-step regressions.

Synthetic proof demonstrates the represented rule only. It is not authentic
firmware compatibility, executed IPL2, BOOT-3, or game compatibility.
