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
committed or bundled commercial/proprietary payloads, host pointers as machine
policy, renderer decisions, and an unearned generic bus or memory-map
framework.

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

Material ownership has three source-clear states. A structurally accepted
explicit byte vector is user-provided `PifFirmware`; generated public bytes are
public-synthetic `PifFirmware` only when a proof selects that constructor; and
no supplied bytes leave firmware unavailable. The optional low-level
user-cartridge composition accepts only the first state. Ordinary cartridge
execution instead selects `CleanRoomHle` and consumes no PIF state. Neither
route promotes unavailable material to public synthetic, treats unavailable
backing as zero, or attaches a host path to Machine or `SpImem` provenance.

Cold-x105 coupled handoff adds four independent explicit host spellings for
family, reset kind, boot medium, and PIF-version bit. They are transferred as
typed Machine inputs and never inferred from a filename, game identity,
cartridge digest, PIF contents, host region, or expected trace. The only
supported coupled path is `NTSC_PINNED` + x105 + cold + cartridge; PAL/MPAL
continue to support their byte-copy layouts but their coupled CPU handoff
requests fail closed.

One separately authorized no-window shell may read an explicit user-supplied
cartridge path and transfer owned bytes into the existing `Cartridge`
normalizer. The selected basename has no product meaning: filename, title, ID,
region, header checksums, digest, strings, and observed PCs cannot select
Machine behavior. Cartridge bytes remain immutable and local; no byte, hash,
header dump, string, or code excerpt enters source, public tests, evidence,
patches, or artifacts.

The named `Machine::stage_cartridge_bootstrap` creation point preflights the
normalized cartridge span `[0x40, 0x1000)`, stages it into the same SP DMEM
offsets, and records cartridge provenance. The bounded inspection host supplies
owned bytes; it never gives the core a file path. This narrow path is not PI
DMA, a general cartridge mapping, or a PIF/CIC implementation.

The separate default `Machine::stage_clean_room_cartridge_entry` path
preflights normalized cartridge source `[0x1000, 0x101000)`, the pinned KSEG0
entry, and the complete one-MiB physical RDRAM destination before mutation. It
constructs replacement RDRAM whose staging record owns exact cartridge and
destination spans plus `CleanRoomHle` cause. It stages no PIF/IPL/X105 bytes and
does not generalize cartridge mapping; later guest writes and DMA remain their
own causes.

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
known truth. Instruction fetch still has no SP-IMEM route. The reached
byte/halfword/word/doubleword load/store families also use the existing direct
RDRAM and CPU-cache owners as detailed by the capability ledger. Unknown
SP-DMEM/device writes, unearned registers, and generic routing remain closed.

Lineage is `lawful bytes → normalized layout → named address domain → preflight → storage mutation/read → narrow observable result`. Failed writes must leave no
ghost state. Synthetic instruction words and small generated fixtures are valid
proof. A user-local ROM may be used only by the separately authorized explicit
local probe and remains outside routine validation and evidence packaging.

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
PI timing, RSP execution, NMI, or generic MMIO route exists.

The separate immutable public runtime-v2 fixture retains the same size and
deterministic unused payload, overlays one original 92-word program, and
recomputes x105 header checksums `0x4077ADEF / 0x096B847A`. Guest KSEG0 cached
stores change only CPU cache bytes until a conflicting dirty replacement writes
one complete 16-byte line to the sole Rdram backing owner. Three such
writebacks create final words `0x11AA3344` at physical `0x00100000` and
`0x55667788` at `0x00102000`. Eight KSEG1 stores bypass D-cache and create the
success mailbox at `0x003FF000`; no separate mailbox owner exists. It does not
establish authentic SP IMEM, user-provided or commercial cartridge execution,
PIF/BIOS boot, SP DMA, controller protocol, game compatibility, or a complete
N64 memory system. Rollback/preflight exists only where the detailed ledger
says it is sealed.

The authorized user-cartridge path leaves the immutable local cartridge in its
existing owner while general atomic PI transfers populate Rdram. Two
preflighted RDRAM-to-SP transfers then copy physical
`[0x0012BAC0,0x0012BB00)` into DMEM `[0x0FC0,0x1000)` and
`[0x000060B0,0x00006498)` into IMEM `[0x0000,0x03E8)`. SP records transfer
metadata; SpDmem and SpImem remain the sole destination byte owners. The first
task-start command follows these genuine DMA effects and no RSP byte is
executed. No local cartridge content is persisted outside Machine memory or
the original user-owned input.

The new public generated RSP path does not change byte ownership. `SpImem`
remains the sole instruction-byte and knowledge owner; selected RSP fetch reads
one known big-endian word and retains its four provenance records.
`SpDmem` remains the sole local-data backing, per-byte knowledge, and
provenance owner. Construction/reset backing is unavailable value truth;
complete cartridge bootstrap leaves `[0x000,0x040)` unavailable and makes
`[0x040,0x1000)` available with exact cartridge-source offsets. CPU stores and
represented SP DMA make only their destination bytes available. The two scalar
MFC0 instructions mutate no memory. Aligned full-register LQV observes sixteen
knowledge entries without copying DMEM ownership: all-available input produces
an available vector while any unavailable byte produces a whole-register
unavailable result. No RSP cache, shadow memory, new DMA behavior, or
generalized memory router was added.

Exact aligned scalar RSP LW observes four consecutive `SpDmem` knowledge
entries without taking byte ownership. It requires all four to be Available
and coherent, constructs one big-endian 32-bit scalar result, and records their
sources in scalar provenance. Unavailable bytes reject rather than producing
an unavailable scalar value. The public bytes at `0x040..0x044` remain
bootstrap-owned `03 A0 48 20`; the Lw read changes neither DMEM backing nor
knowledge. Both following NOPs leave memory unchanged.

Exact RSP MTC0 does not add a memory owner. `SP_MEM_ADDR` and `SP_DRAM_ADDR`
program their existing `Sp` state without transferring bytes. `SP_RD_LEN`
reuses the private owner-local read-DMA policy already reached from CPU
SP-register writes. The public raw-zero length preflights source-known RDRAM
`[0x180,0x188)`, then atomically copies `25 29 00 04 15 1F FF E3` into DMEM
`[0,8)`. `Rdram` remains the source-byte owner, `SpDmem` remains the
destination-byte/knowledge owner, and `Sp` records transfer causality. The
eight destinations become Available with exact `SpDma` record provenance;
DMEM `[8,16)` remains unavailable. No partial transfer, shadow memory, DMA
queue, busy-duration truth, or retroactive mutation of pre-DMA `v12` exists.

The later public sequence uses the same singular owners for a second transfer.
Raw read length `0xFFF` preflights complete Rdram-owned source
`[0x400,0x1400)` and complete SpDmem destination `[0,0x1000)`. One atomic
application appends DMA record one, copies all 4096 bytes, and replaces every
DMEM knowledge entry with Available `SpDma { record_index: 1 }` truth. Existing
register evolution ends at local zero and RDRAM `0x1400`. The prior record,
scalar r4/r6, unavailable pre-DMA v12, and semaphore remain unchanged. Failure
preflight produces no partial byte copy, record, or address evolution. A later
MFC0 SP_DMA_BUSY read derives idle zero because no transfer persists beyond
the committing instruction boundary; it adds no timing state.

Exact Vsub and Vaddc do not move or duplicate DMEM ownership. Each aligned
Lqv observes the existing Available `SpDmem` truth and writes only `Sp::rsp`
vector state. The 256-address sequence is `0xFF0` through `0x000` by `-0x10`;
final v14 reflects the last Available read at DMEM zero. Cause-known
unavailable v13/accumulator/VCO results contain no hidden memory bytes. The
post-loop SP_MEM_ADDR and SP_DRAM_ADDR writes program only existing `Sp`
state. SP_WR_LEN selects complete Available/non-opaque IMEM `[0x120,0x1e0)`.
The Sp owner preflights all 24 eight-byte RDRAM destinations before atomically
copying 192 bytes, preserving IMEM truth, and appending record two. Rdram
retains singular destination-byte ownership; the record owns causality rather
than duplicate bytes. Final local/RDRAM addresses are `0x11e0/0x00313070`.
No persistent DMA duration, queue, partial progress, or unrelated memory
mutation exists. The later exact DPC_STATUS counter-clear command and the
subsequent committed Break leave RDRAM, DMEM, IMEM, cartridge bytes, and all
three DMA records unchanged.

Required validation: `./rust/verify-forward` plus focused cartridge/RDRAM tests.
Performance and large-ROM resource behavior are `UNKNOWN` without measurement.
Pinned mapping evidence now identifies NTSC raw `[0x0d4,0x71c)` to SP IMEM
`[0x000,0x648)` and PAL/MPAL raw `[0x0d4,0x720)` to
`[0x000,0x64c)`. Shape-only input cannot select a mapping. The represented
Machine profile and full-range generated proof now cover the copy effect; the
remaining evidence pressure is profile-qualified PAL/MPAL and broader
pre-cartridge-entry state. Neither earns an architecture-first bus
abstraction.
