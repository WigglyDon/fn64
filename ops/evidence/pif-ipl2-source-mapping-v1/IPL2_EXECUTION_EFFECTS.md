# IPL2 execution effects

## Scope

This file separates the retained-byte copy from dynamic IPL1/IPL2 behavior.
The findings are paraphrases of `SRC-RE-DECOMPALS-928F`, corroborated only where
the source register says so. They authorize no firmware execution in fn64.

## Effects relevant before IPL3

| Effect | Cause and inputs | Owner and address/state | Stage | Static or dynamic | Current relevance |
| --- | --- | --- | --- | --- | --- |
| IPL2 bytes in SP IMEM | IPL1 reads the selected raw source range and writes SP IMEM | CPU event; Machine-owned SP IMEM `[0x000, 0x648)` or `[0x000, 0x64C)` | IPL1 | Dynamic historically; source-backed materialization is possible | Required: x105 reads and mutates its low prefix |
| Stack pointer and stack traffic | IPL1 chooses a stack near SP IMEM end; IPL2 calls use it | CPU GPR `sp`; high SP IMEM outside the copied low range | IPL1 and IPL2 | Dynamic execution effect or explicit end-state materialization | `sp` is needed by x105 and is already represented by current bootstrap |
| Boot handoff registers | IPL2 derives `t3` as SP DMEM plus `0x40`; the final linked call leaves a negative KSEG1 return address in `ra` | CPU GPRs | IPL2 handoff | Dynamic control-flow result or explicit end-state materialization | `t3` supplies x105 DMEM reads; `ra` selects its tail branch; current bootstrap does not establish either |
| Region/reset/security register state | IPL2 reads a PIF-RAM seed/status field and applies regional build choices | CPU GPRs including `s3` through `s7` | IPL2 | Dynamic input-dependent computation | Not consumed by the retained-byte copy; required by common IPL3 after the x105 prelude |
| PI timing | IPL2 reads cartridge-header timing fields and writes PI configuration | Machine-owned PI state | IPL2 | Dynamic input-dependent device effect | Not required for the already staged DMEM bytes, but relevant to an authentic copy path |
| IPL3 staging | IPL2 copies cartridge bytes `[0x40, 0x1000)` into SP DMEM `[0x40, 0x1000)` | Machine-owned SP DMEM | IPL2 | Dynamic historically; already represented as a bounded bootstrap effect | Required for current IPL3 instruction/data reads |
| IPL3 checksum exchange | IPL2 computes a cartridge-dependent result, writes PIF RAM, requests verification, waits for acknowledgement, then requests RAM clearing | CPU, SI/PIF state, PIF RAM, and cartridge bytes | IPL2 | Dynamic and input/device-dependent | Current bootstrap does not model it; current bounded trace does not claim the verification occurred |
| IPL3 control transfer | IPL2 jumps to SP DMEM KSEG1 address `0xA4000040` | CPU program counter and GPR handoff | IPL2 end | Dynamic historically; entry state can be explicitly represented | Entry PC is already represented; related GPR provenance remains incomplete |

## Retained SP IMEM mutations

`EXTERNAL_TECHNICAL_EVIDENCE`: the pinned IPL2 uses high SP IMEM for stack
storage but does not write into its copied code range. Therefore the low
retained bytes are not changed by IPL2 in the examined builds.

`EXTERNAL_TECHNICAL_EVIDENCE`: after IPL3 begins, the x105 prelude reads the
retained words, combines them with staged DMEM data, and writes results back to
the same low SP IMEM area. Its tail adds writes through local end exclusive
`0x02C`.

`INFERENCE`, supported by these two source facts: the proposed raw-to-IMEM copy
is sufficient to reproduce the initial retained byte content for the pinned
profile, but not the complete post-IPL2 machine state.

## Would direct copy omit a required effect?

Yes, if direct copy were presented as a complete IPL1/IPL2 replacement. It
would omit at least the `t3` and `ra` handoff facts used by the bounded x105
path, along with PIF-RAM-derived saved registers and device/checksum effects.
It would not omit those effects if its authority were explicitly limited to
the IPL1 copy effect and all other unknown state continued to fail closed.

`SRC-EMU-MUPEN-9EB6` corroborates that an emulator can materialize selected boot
end effects without executing firmware. Its HLE also demonstrates why staging
only a convenient retained prefix is not an exact IPL1 copy: it covers a narrow
prefix rather than the full regional range and does not by itself establish the
current mutation frontier.

`SRC-EMU-CEN64-E064` independently chooses supplied-byte execution and separately
initializes PIF RAM. That implementation choice corroborates the existence of
state outside the raw copied slice, but it does not prove execution is required.

## Unknowns

- `UNKNOWN`: complete dynamic behavior of unexamined physical PIF revisions.
- `UNKNOWN`: a source-clear current fn64 representation for all PIF-RAM and
  security-handshake state.
- `UNKNOWN`: whether additional device-visible effects are required beyond the
  current bounded x105 trace before a broader authentic-boot claim.
- `UNKNOWN`: any universal rule selecting a regional copy mapping from a
  structurally valid 1,984-byte input.
