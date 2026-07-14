# Bounded MTC0 boot trio and generated x105 RI frontier

Frontier decision: `MTC0_CAUSE_COUNT_COMPARE_TRIO_CONFIRMED`.

The accepted generated composition reaches consecutive word-form MTC0
instructions at `0xA400007C`, `0xA4000080`, and `0xA4000084`, all sourced by
architectural r0 and targeting Cause, Count, and Compare respectively. The
common transfer, access, destination side effects, and cadence are bounded
without a general CP0 register bank.

After the trio, represented `Lui` constructs `0xFFFFFFFFA4700000`; the next
aligned `Lw` selects virtual `0xA470000C`, physical `0x0470000C`, RI_SELECT
offset `0x0C`. RI/MMIO is absent, so that load is the fail-closed next frontier.

All proof inputs are generated and all instruction words are independently
encoded by semantic fields. This is synthetic composition, not authentic PIF,
IPL, cartridge execution, BOOT-3, interrupt delivery, RI behavior, or game
compatibility.
