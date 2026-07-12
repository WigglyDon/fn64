# Materialization versus execution

## Classification

`VARIANT_SPECIFIC_MAPPING_REQUIRES_EXPLICIT_MACHINE_PROFILE`

The pinned NTSC mapping is raw `[0x0D4, 0x71C)` to SP IMEM
`[0x000, 0x648)`, while the pinned PAL and MPAL mapping is raw
`[0x0D4, 0x720)` to SP IMEM `[0x000, 0x64C)`. The one-word length
difference is part of the pinned regional source structure. Current fn64
validation accepts only the common 1,984-byte shape and cannot honestly select
one mapping. A future source-backed copy therefore needs an explicit
Machine-owned profile with no inferred default.

## Comparison

| Question | Source-backed materialization | Minimal firmware execution |
| --- | --- | --- |
| Raw bytes | Uses only the explicit user-supplied accepted bytes | Also requires explicit user-supplied bytes; fn64 may not embed firmware |
| IPL1 copy | Can reproduce the exact selected copy as a direct Machine state transition | Would produce the copy by executing IPL1 |
| Variant selection | Must be an explicit supported Machine profile | Still needs an honest firmware/device profile; execution does not make shape-only identification valid |
| Retained range | Full selected copy is representable; isolated-word staging is rejected | Produced by the copy loop if all prerequisites execute correctly |
| Dynamic GPRs | Must be separately source-backed and materialized, otherwise remain unknown | May be produced by execution if PIF RAM, devices, and inputs are also represented |
| PIF RAM and device effects | Must be separately modeled or left explicitly unsupported | Execution alone is insufficient without device state and request/response behavior |
| Current authority | A narrow copy-only lane can remain fail-closed after materializing SP IMEM | No current evidence lane authorizes a firmware execution path |

## Current causal requirement

The retained byte ranges needed by x105 are completely inside every established
regional copy. Materializing the full selected copy preserves the causal fact
that user-supplied PIF source bytes, not embedded fn64 constants, produce the
initial SP IMEM state. It must be described as materialization of the IPL1 copy
effect, not as IPL1 or IPL2 execution.

Copy-only materialization is not a complete IPL2 handoff. In particular, it
does not produce the `t3` DMEM pointer or the `ra` branch input used by x105,
and it does not produce PIF-RAM-derived saved registers. Those omissions are
acceptable only while Machine stepping stops honestly when the next required
fact is unavailable.

## Why the other classifications do not apply

- `SOURCE_BACKED_MATERIALIZATION_PROVEN` does not apply to the complete
  pre-IPL3 state: shape-only input cannot select the differing regional copy,
  and copy-only materialization omits dynamic handoff facts.
- `MINIMAL_FIRMWARE_EXECUTION_REQUIRED` does not apply: the sources prove that
  more than bytes are needed for a complete handoff, but do not prove those end
  effects can only be represented by execution. One independent emulator uses
  HLE materialization, while another executes supplied bytes. That is an
  implementation choice, not proof of necessity.
- `PARTIAL_MAPPING_ONLY` does not apply to the pinned NTSC, PAL, and MPAL
  reconstructions: their source start, end, length, destination, and lifecycle
  are exact. The limitation is variant selection, which has its own required
  classification.
- `UNKNOWN` does not apply to those pinned mappings. It does remain the correct
  label for unexamined physical PIF revisions.

## Smallest next lane

The smallest later product lane is a Master-owned
`pif-ipl2-profiled-copy-materialization-v1` lane. Its sole new truth would be:
given an explicit supported Machine profile and structurally accepted
user-supplied raw bytes, `Machine` materializes the corresponding complete IPL1
copy range into SP IMEM with source provenance and reset stability.

Strict non-goals: no automatic profile detection; no filename, hash, content,
region, or compatibility database; no IPL1/IPL2 execution; no PIF-RAM or device
handshake; no synthesis of `t3`, `ra`, or saved registers without separate
evidence; no trace-advance promise; no BOOT-3; no game compatibility.

Further evidence is required before a product lane may claim a complete IPL2
handoff or progress beyond the next unavailable dynamic fact. It is not required
for a copy-only, profile-explicit, fail-closed materialization lane.
