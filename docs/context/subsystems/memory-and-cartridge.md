# Memory And Cartridge

Context role: memory/cartridge architecture context.
Scope: normalized cartridge bytes, RDRAM, SP DMEM, and represented access seams.
Canonical for: byte ownership, address-domain boundaries, and legal fixture policy.
Not canonical for: detailed represented access capability or future bus design.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and [boot checkpoint](../../boot_spine_checkpoint.md).
Update triggers: byte ownership, normalization, address classification, or represented storage changes.

The Machine owns accepted cartridge bytes after normalization, RDRAM, SP DMEM, SP IMEM,
and each represented state mutation. Hosts may supply bytes and paths but never
retain competing emulated truth. Source-layout normalization and storage
endianness must remain explicit and range checked.

Allowed direction is host bytes → cartridge normalization → Machine-owned
storage/classification. Forbidden directions include core filesystem access,
commercial/proprietary payloads, host pointers as machine policy, renderer
decisions, and an unearned generic bus or memory-map framework.

For an explicitly user-selected PIF firmware file, the host may own only the
path, file read/failure, and owned-byte transfer. The Machine must own accepted
bytes, validation/classification, reset/bootstrap lifecycle, SP IMEM production,
and provenance. The no-window probe now implements that input boundary with one
optional literal `--pif-rom` path and no default, search, download, bundled
fallback, reconstructed table, or firmware-derived profile. A separate
explicit `--pif-profile` spelling selects one Machine-owned pinned layout; the
host does not own layout meaning or infer a value. Machine accepts a
1,984-byte candidate structurally, rejects a 2,048-byte full-map shape as
unsupported, and rejects other lengths as malformed. Acceptance does not prove
authenticity. Firmware and profile installation remain independent and neither
alone produces SP IMEM state.

Cold-x105 coupled handoff adds four independent explicit host spellings for
family, reset kind, boot medium, and PIF-version bit. They are transferred as
typed Machine inputs and never inferred from a filename, game identity,
cartridge digest, PIF contents, host region, or expected trace. The only
supported coupled path is `NTSC_PINNED` + x105 + cold + cartridge; PAL/MPAL
continue to support their byte-copy layouts but their coupled CPU handoff
requests fail closed.

The named `Machine::stage_cartridge_bootstrap` creation point preflights the
normalized cartridge span `[0x40, 0x1000)`, stages it into the same SP DMEM
offsets, and records cartridge provenance. The bounded inspection host supplies
owned bytes; it never gives the core a file path. This narrow path is not PI
DMA, a general cartridge mapping, or a PIF/CIC implementation.

Aligned CPU `Lw` now reuses that exact bootstrap span as the sole production
knownness owner for direct SP-DMEM data reads. A complete word within
`[0x040,0x1000)` reports its exact source cartridge offset; backing below
`0x040` or without current bootstrap lineage rejects before mutation. The route
adds no SP-DMEM write, mirroring, device access, bus, or generalized map.

SP IMEM is exactly 4 KiB of private Machine-owned backing storage for physical
addresses `0x04001000..0x04001fff`. Construction and reset create zero backing
with every byte explicitly `Unknown`. Cartridge-bootstrap restaging builds a
replacement image and, when both inputs exist, copies the complete selected
range before assignment. Every copied byte receives user-supplied-PIF source
provenance; every other byte remains `Unknown`. An aligned big-endian word is
readable only when all four bytes have represented provenance. Test-only
staging remains distinct from this production creation event.

Aligned CPU `Sw` mutates direct RDRAM/SP-IMEM storage and the exact represented
RI, MI, global RDRAM, and generated RCP-2 module-register targets. Existing
narrow RI/MI/global-request semantics remain intact. Exact generated MI command
words enable/disable module-register reads; module DEVICE_TYPE, manufacturer,
and mode values are readable only while enabled. DEVICE_ID requests update
mapping metadata without moving the one backing store. Module RAS_INTERVAL is
`0x101C0A04`; RI_REFRESH is the raw readable word `0x001E3634` without timing
effects.

The current 4 MiB backing selects immutable profile
`fixed-standard-retail-4mib-two-module-digital-cc-v1`: two present 2 MiB
modules, DEVICE_TYPE `0xB0190000`, fixed NEC manufacturer `0x0500`, and no
enhanced-speed bit. During active manual calibration only, direct RDRAM reads
shape the response byte as `min(n + 1, 8)` one bits for nominal input `n`;
ordinary reads otherwise return backing bytes. Absent-module probes return zero
and never create a module. The profile is Machine-owned and capacity-derived,
never cartridge/host selected, and claims no analog or timing accuracy.

Opaque SP-IMEM words retain cause/address truth without value bits. Known full
overwrite replaces them. Aligned `Lw` may transport canonical zero backing only
with the original unavailable lineage, so later consumers cannot treat it as
known truth. Instruction fetch still has no SP-IMEM route. `LBU`/`SB` are
narrowly represented over direct SP IMEM only. Unknown SP-DMEM/device writes,
unearned registers, and generic routing remain closed.

Lineage is `lawful bytes → normalized layout → named address domain → preflight → storage mutation/read → narrow observable result`. Failed writes must leave no
ghost state. Synthetic instruction words and small generated fixtures are valid
proof; user-local ROMs are outside routine inspection and evidence packaging.

Current integration includes the prior cartridge/bootstrap/SP/RI/MI/global
facts plus one capacity-derived fixed RDRAM profile, two module records,
deterministic digital current-control response, module-register read/write
state, mapping metadata, RI_REFRESH, and the guest-detected size word. `Rdram`
remains the only backing owner; module state never duplicates bytes. Lifecycle,
snapshots, equality, rollback, and independent Machines include the complete
profile/module/register state. Synthetic proof does not convert any private PIF
or ROM input into product truth.

Source-qualified
evidence identifies retained IPL2 firmware as
the external producer for the observed x105 prefix `[0x000, 0x020)` and initial
mutation range `[0x000, 0x02c)`. Explicit profiled copy now represents that
byte-transfer effect from lawful input, but no private PIF was used. Generated proof combines it atomically with the bounded NTSC
cold-x105 CPU
handoff and advances a generated 247,000-step composition through the stored
RI_SELECT read, cold BNE/NOP slot, five high-SP-IMEM saves, exact RI_CONFIG
store, 8,000 CPU-loop iterations, RI_CURRENT_LOAD event, following `Ori`, and
exact RI_SELECT write, both RI_MODE writes, both bounded CPU waits, the exact
MI_INIT_MODE write, delay-word construction, exact global RDRAM_DELAY commit,
raw-zero global RDRAM_REF_ROW commit, DEVICE_ID-value `Lui`, exact global
RDRAM_DEVICE_ID requested-base commit, fourteen CPU-local setup steps, the
MI_VERSION read, guest-selected RCP 2.0 branch and delay slot, spacing/base
setup, exact first-responder zero request, and the following RDRAM_MODE-address
`Addiu`. The generated JAL at `0xA40001A0` replaces retained r31 with PC+8,
its Nop slot executes once, and five InitCCValue entry instructions commit.
Four inherited-unknown r2-r5 saves then create opaque aligned SP-IMEM words;
twenty known-source saves follow without disturbing them. FindCC JAL/Nop,
BEQL annul, TestCCValue, and WriteCC then commit through public stepping. The
actual first manual word is `0x46C0C0C0`; its `Sw` at CPU `0xA3F0000C` /
physical `0x03F0000C` commits one request. WriteCC returns through JR/Nop and
execution continues through the full digital calibration, two-module discovery,
one absent probe, final mapping, module RAS configuration, RI_REFRESH, detected
4 MiB size store, and frame teardown. Exact SP status/PC state then brackets
the generated relocation from SP-DMEM local `[0x554,0x888)` into physical
RDRAM `[0x4,0x338)`. Existing owners perform all 205 known-word loads and
stores; no shadow buffer or byte owner is added. Relocated KSEG0 execution
fills one I-cache line and reads the public cartridge header. `Cartridge` then
remains sole owner of a complete public 0x101000-byte fixture while `Pi`
atomically transfers offsets `[0x1000,0x101000)` into the sole `Rdram` backing
range `[0x1000,0x101000)`. CPU D-cache reads cached copies only; PI does not
snoop cache state. Generated known stores overwrite all SP DMEM/IMEM words
with `0xA4002000`, replacing opaque truth through existing owners. No general
PI programming, RSP execution, PI/cache timing, NMI, dirty D-cache writeback,
or generic MMIO route exists. It does not establish authentic SP IMEM,
firmware/cartridge execution, PIF/BIOS boot, SP DMA, controller protocol, game
compatibility, or a complete N64 memory system. Rollback/preflight exists only
where the detailed ledger says it is sealed.

Required validation: `./rust/verify-forward` plus focused cartridge/RDRAM tests.
Performance and large-ROM resource behavior are `UNKNOWN` without measurement.
Pinned mapping evidence now identifies NTSC raw `[0x0d4,0x71c)` to SP IMEM
`[0x000,0x648)` and PAL/MPAL raw `[0x0d4,0x720)` to
`[0x000,0x64c)`. Shape-only input cannot select a mapping. The represented
Machine profile and full-range generated proof now cover the copy effect; the
remaining evidence pressure is profile-qualified PAL/MPAL and broader
pre-cartridge-entry state. Neither earns an architecture-first bus
abstraction.
