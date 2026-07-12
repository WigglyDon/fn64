# Required pre-IPL3 state

## Retained SP IMEM coverage

| Required item | Exact producer | Source coverage | Current representation | Result |
| --- | --- | --- | --- | --- |
| Retained bytes consumed at `[0x000, 0x020)` | IPL1 regional copy | Raw `[0x0D4, 0x0F4)`; 32 bytes because `0x0F4 - 0x0D4 = 0x20` | SP IMEM exists but accepted PIF bytes currently do not populate it | Fully source-covered for all three pinned profiles; not product-materialized |
| Retained mutation input/output through `[0x000, 0x02C)` | IPL1 regional copy supplies initial bytes; x105 later mutates them | Raw `[0x0D4, 0x100)`; 44 bytes because `0x100 - 0x0D4 = 0x2C` | The required low region remains unknown absent generated test staging | Fully source-covered for all three pinned profiles; not product-materialized |
| Full retained IPL2 copy | IPL1 regional copy | NTSC raw `[0x0D4, 0x71C)`; PAL/MPAL raw `[0x0D4, 0x720)` | No accepted-firmware-to-IMEM path | Exact only after an explicit supported profile is selected |

Convenience staging of one word, eight words, or only `[0x000, 0x020)` is
incomplete. The observed mutation frontier extends through `[0x000, 0x02C)`,
and the honest IPL1 effect is the full selected regional copy range.

## Registers and other machine state

| State | Causal producer | Required by | Current status | Honest disposition |
| --- | --- | --- | --- | --- |
| PC `0xA4000040` | IPL2 transfer to IPL3 in SP DMEM | IPL3 entry | `LIVE_REPO_FACT`: represented by cartridge bootstrap | Already represented; does not prove IPL2 ran |
| `sp` near the end of SP IMEM | IPL1 constant setup | x105 retained-range address calculation and IPL2 stack | `LIVE_REPO_FACT`: represented as a source-known bootstrap GPR | Already represented; keep provenance explicit |
| `t3 = 0xA4000040` at handoff | IPL2 final transfer setup | x105 reads staged DMEM relative to `t3` | `LIVE_REPO_FACT`: not source-known in current bootstrap | Requires separate source-backed end-state materialization or execution; copy alone cannot supply it |
| negative KSEG1 `ra` link value | IPL2 linked call into the final verification-and-run routine | x105 tail branch | `LIVE_REPO_FACT`: not source-known in current bootstrap | Requires separate source-backed control-flow result or execution |
| `s3` through `s7` | PIF-RAM input, reset state, regional IPL2 choices, and cartridge/security seed | common IPL3 after the x105 prelude | Not established by the retained copy; current bootstrap leaves unsupported GPRs unknown | Further evidence and explicit ownership required before broader progress |
| SP DMEM `[0x040, 0x1000)` | IPL2 cartridge copy from cartridge `[0x040, 0x1000)` | IPL3 instructions and x105 data | `LIVE_REPO_FACT`: already staged from the user cartridge | Already represented as materialization; does not prove IPL2 execution or checksum acceptance |
| PIF RAM seed/status and clearing | PIF/device initialization plus IPL2 request/response | IPL2 register derivation and verification lifecycle | Not represented as an authentic PIF lifecycle | Outside the raw PIF Boot ROM slice; remains a separate evidence/product boundary |
| PI configuration and SI/PIF status | IPL2 cartridge-header reads and device operations | authentic IPL2 copy and verification flow | Not required for the current pre-staged DMEM read path | Must not be claimed from byte materialization |

## Causal conclusion

`INFERENCE`, supported by the exact copy mapping and pinned x105 source: the
source slice completely supplies every retained SP IMEM byte currently known to
be consumed or mutated. It does not supply every register or device fact needed
to represent the entire pre-IPL3 lifecycle.

The narrow next product seam may therefore materialize only the selected IPL1
copy effect and remain fail-closed at later unknown state. Any lane that claims
a complete IPL2 handoff must additionally prove and own the dynamic items above.
