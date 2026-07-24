# Instruction Pipeline

Context role: fetch/decode/identify/classify context.
Scope: the represented Rust current-PC instruction production path.
Canonical for: pipeline fact ownership and ordering.
Not canonical for: exhaustive instruction semantics.
Inherits: [root law](../../../AGENTS.md) and [core scope law](../../../rust/crates/fn64-core/AGENTS.md).
Current-state owner: [CURRENT_STATE.md](../CURRENT_STATE.md).
Related evidence: [rust/PARITY.md](../../../rust/PARITY.md) and `machine.rs` source/tests.
Update triggers: fetch targets, decode/identity ownership, selection, or action production changes.

The source-clear path is:

`current pc/context → target/provenance classification → one instruction fetch → one raw-field decode → one identity classification → contextual and source-knownness gates → ordinary-control-flow planning, no-effect/stopped/unsupported, represented memory/device planning, represented COP0/cache planning, or one CPU-local helper selection → classified action`.

Production does not apply machine mutation. Application does not refetch,
decode, or identify. The instruction word and decoded fields are fixed-width;
memory words are interpreted through explicit big-endian storage access.

Allowed dependencies flow from cartridge/storage and CPU address types into
Machine fetch, then pure decode/identify, then classification/application.
Forbidden dependencies include host paths, dynamic registries, probe policy,
private producer calls from inspection, and a generic all-future dispatcher.

Proof consists of source anchors, classification/fetch unit tests, focused step
tests, the 187-case step probe, and the bounded BOOT-2 trace. Read-only
current-instruction inspection exposes address, fields, identity, and Machine
source provenance without mutable state. Proof does not mean every recognized
identity executes. `Lw` is represented as one Machine-owned rule over direct RDRAM,
known SP IMEM, cartridge-bootstrap-staged SP DMEM, exactly the stored RI_SELECT
word at physical `0x0470000C`, and immutable MI_VERSION at
`0x04300004`. The SP-DMEM target records exact cartridge
provenance; RI_SELECT records its cold-entry source; unclassified backing,
unknown SP IMEM, unavailable RI_SELECT, and neighboring RI addresses reject
before mutation. The authentic first frontier remains that
represented rejection at `0xA4000044`, not absent decode or load semantics.
Integrated provenance evidence identifies the missing source category as
retained IPL2 firmware content. Explicit profiled bootstrap materialization can
now satisfy the known-byte gate from generated or user-supplied bytes; external
source knowledge alone cannot bypass it. Synthetic `Lw` success proves the
represented composition, not authentic boot. A separate generated-only NTSC
cold-x105 test proves the Machine bootstrap plan can source the inherited t3
operand before `Machine::step`; that source gate still rejects every unstaged
GPR and does not imply IPL2 execution. Generated-only continuation now commits the SP-IMEM load, the
earlier-missing
SP-DMEM load, logical transforms, four prefix SP-IMEM stores, BNE/BLTZ, both
ordinary slots, the bounded MTC0 trio, the stored RI_SELECT load, the cold BNE
and NOP slot, five high-SP-IMEM stack saves, the exact RI_CONFIG store, 8,000
four-instruction CPU-loop iterations, RI_CURRENT_LOAD update event, following
`Ori`, exact RI_SELECT write, both RI_MODE stores, and both bounded CPU wait
regions. The second BNE delay slot executes the `Ori` that constructs `0x10F`
on all 32 iterations. The exact MI_INIT_MODE `Sw` commits length 15 and
initialization mode true with CPU-store provenance, then generated `Lui`/`Ori`
construct `0x18082838`. The exact global RDRAM_DELAY `Sw` consumes the pending
transfer and stores logical fields 5/7/3/1. The exact global RDRAM_REF_ROW `Sw`
then stores raw zero with CPU provenance, the following `Lui` produces the
DEVICE_ID transfer word, and the exact global RDRAM_DEVICE_ID `Sw` stores a
bounded requested-base fact. Fourteen existing CPU-local commits construct the
setup state; exact MI_VERSION `Lw` returns `0x02020102`, and the generated
comparison selects RCP 2.0 through ordinary Bne/delay-slot semantics. Spacing
`0x400` and first-responder base `0xFFFFFFFFA3F08000` are CPU results. The
following exact zero store commits one bounded first-responder assignment
request; `Addiu` then constructs initial RDRAM_MODE address
`0xFFFFFFFFA3F0000C`. The next JAL at `0xA40001A0` replaces retained r31 with
PC+8 and exact JAL lineage, its Nop slot executes once, and five InitCCValue
entry instructions commit. Four inherited-unknown saves commit as opaque
SP-IMEM words and twenty known-source saves follow. FindCC JAL/Nop and setup
then commit. BEQL alone is represented among likely branches: available old
GPR values compare across all 64 bits; taken execution uses the existing one
slot, and not-taken execution annuls PC+4 with no slot execution, commit,
Count, effect, exception, or context. The generated BEQL is not taken;
TestCCValue and WriteCC then produce exact `0x46C0C0C0`; the physical
RDRAM_MODE `0x03F0000C` store commits one request through the existing BNE
slot. Existing and newly required source-clear identities then execute every
guest calibration, discovery, module-register, final-mapping, refresh, and
frame-teardown transition. At 247,000 commits, PC/next-PC are
`0xA4000400 / 0xA4000404`, Count is `246984`, and guest size is
`0x00400000`. Bounded MTC0 then stores zero TagLo/TagHi and CACHE op
`0x08/0x09` invalidates every primary I/D line. KSEG1 continues to fetch
directly. After public relocation, KSEG0 fetch requires CPU-owned I-cache
truth: a miss reads one complete 32-byte Machine-owned RDRAM line, applies one
fill before decode, and rolls it back if instruction application rejects; a
matching valid line returns its cached word without backing access. At 252,367
commits cached word `0xAC290000` begins exact PI programming. Subsequent closed
plans cover PI registers/status, one atomic DMA, exact final control targets,
and SP-DMEM/IMEM teardown stores. KSEG0 aligned `Lw` uses CPU-owned D-cache
hit/fill truth over one 16-byte RDRAM line; KSEG1 remains uncached. Generated
execution reaches the synthetic entry after JR/Nop. This proves deterministic
fixed-profile cache/PI/final-handoff composition.

Runtime-v2 adds no alternate pipeline. The ordinary DirectRdram plan retains
the old operands and segment; KSEG0 `Sw`/`Sb` produce CPU-owned write-allocate
plans, KSEG0 `Lw`/`Lbu` consume clean or dirty line truth, and KSEG1 bypasses
cache. A miss plan includes the complete fill and any dirty victim; application
commits one atomic Rdram writeback before replacement. Public stepping executes
the 92-word program, all seven comparison branches, its KSEG1 mailbox stores,
and two success-loop J/Nop iterations. This proves neither PI/cache timing, RSP
execution, authentic IPL2/user cartridge execution, nor generic device routing.

The authorized user-cartridge probe uses this same pipeline from cold
construction through the first SP start request. It adds no inspection-owned
execution: scalar, COP0/TLB-register, cache, interrupt, PI, SI, VI, AI, and SP
frontiers are implemented as general Machine actions and then advanced by
public stepping. The first SP_STATUS command that changes halt true to false
commits before the probe stops; no RSP fetch or instruction application occurs.

The MTC0 producer accepts a known old source and the exact represented
destination set: Index, EntryLo0/1, Context, PageMask, Wired, EntryHi, Status,
Cause, EPC, Count, Compare, and TagLo/TagHi. Its immutable plan resolves
fallible facts before destination-specific COP0 mutation and cadence.
Matching reached MFC0 sources, bounded TLB register operations, and ERET share
Cpu ownership without creating a generic register writer or translated memory
route.

The `Sw` producer checks base knownness, computes address, selects AdES before
source-value consumption, rejects every target except direct RDRAM/SP memories
or the exact represented RI, MI, PI, SP, SI, AI, VI, global RDRAM, and
generated RCP-2 module targets,
and only then captures source value/lineage and
constructs a closed destination plan. RI_CONFIG planning rejects undefined
high bits; RI_CURRENT_LOAD planning requires stored RI_CONFIG and snapshots its
fields; RI_SELECT planning accepts only low word `0x14` and creates exact
CPU-store provenance; RI_MODE planning stores bits 1:0 and bits 2/3 while
rejecting nonzero bits above bit 3; MI_INIT_MODE planning accepts only low word
`0x0000010F` and creates length 15 / initialization mode true plus one bounded
transfer with exact CPU-store provenance. While pending, other represented
commits reject. Global RDRAM_DELAY planning requires exact address, low word
`0x18082838`, and that transfer, and creates logical configuration `0x28381808`
with consumed lineage. First-responder planning matches only physical
`0x03F08004`, requires known low word zero and no pending transfer, and creates
a request fact rather than module state. Initial-mode planning matches only
physical `0x03F0000C`, known low word `0x46C0C0C0`, and no pending transfer;
its fields derive from one raw request. Application neither reclassifies nor
discovers a new failure.

After alignment, direct normalization, and exact destination classification,
an explicitly unavailable store source may form an opaque plan only for an
already-supported aligned SP-IMEM word. The plan carries cause and address but
no value bits; application canonicalizes private backing and installs one
coherent owner state. Unknown effective addresses, SP-DMEM stores, device
commands, and unsupported targets retain their rejection paths. Aligned Lw
from opaque SP IMEM may transport canonical backing with the original
unavailable lineage; the backing remains non-truth and later consumers reject.

Ordinary `BEQ`/`BNE`/`BLTZ`, exact likely `BEQL`/`BNEL`/`BLEZL`/`BGEZL`,
`J`, `JAL`, `JR`, and `JALR` identities select one bounded Machine-owned action before sequential
staging. BLTZ alone reuses the existing full-GPR signed comparator; no generic
REGIMM executor exists. A control-flow identity inside a represented slot
selects explicit unsupported rollback. Application schedules or clears the
CPU-owned slot context; it does not refetch or re-identify. JAL planning has no
GPR source and does not inspect prior r31. JALR planning captures old `rs`
before application and never treats unrelated prior `rd` as an input.

Required validation: `./rust/verify-forward` and relevant focused filters.
Known unknowns include future public-step integration categories, unearned
branch-likely/REGIMM members, COP0/CACHE operations beyond the detailed ledger,
translated TLB access, NMI, generic MMIO, broad fetch mapping, analog device
behavior, and instruction timing.
