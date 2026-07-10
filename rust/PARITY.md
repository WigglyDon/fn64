# fn64 Rust Machine And Historical Parity Ledger

Current authority: direct retirement of the frozen C++ lane, 2026-07-10.

The Rust workspace is the sole current product
implementation for the cartridge/ROM, Machine/RDRAM/CPU construction ownership, CPU
GPR access/mutation, CPU scalar-state staging, COP0 construction/access
parity, COP0 derived-read/mutation readiness audit, RDRAM raw byte/u16_be/u32_be/u64_be storage
read access, raw RDRAM byte/u16_be/u32_be/u64_be write access and raw write-family
sealing, raw RDRAM read-width family sealing, Machine-owned CPU/RDRAM
reservation construction/default-state ownership/parity sealing, private CPU/RDRAM
reservation staging/setup and staging parity sealing, private CPU/RDRAM reservation
invalidation behavior, RDRAM byte-write decision, RDRAM raw byte-write parity
sealing, RDRAM raw u16_be write decision, RDRAM raw u16_be write parity
sealing, RDRAM raw u32_be write decision, RDRAM raw u32_be write parity
seal, RDRAM raw u64_be write decision/parity seal, raw write-family seal,
raw read-width family decision/parity seal, raw read/write storage-family
status, CPU load/store readiness audit, direct CPU-address-to-RDRAM
classification decision/seal, CPU-addressed direct RDRAM value access
decision/seal, pure CPU data-address alignment contract, pure CPU
data-address-error exception-class selection, and narrow CPU-owned data
address-error exception-entry mutation, Machine-owned direct RDRAM CPU data
access preflight/value-access composition plus direct RDRAM CPU-data
target-rejection to address-error entry, Machine reset for the represented
non-boot power-on state, no-window machine construction/reset probe, CPU
step readiness audit, raw CPU instruction-word decode representation, pure
CPU instruction identity classification, and direct RDRAM CPU instruction-word
fetch plus CPU instruction-fetch target classification, represented SP DMEM
storage, read-only SP DMEM CPU instruction-word fetch, explicit-address CPU
instruction-word fetch over represented direct RDRAM and SP DMEM targets, and
current-PC CPU instruction-word fetch wrapper, pure instruction-fetch fault
to address-error selection plan, and narrow instruction-fetch address-error
entry mutation, pure step fetch-fault action classification/rethrow
readiness, unsupported-step outcome readiness for unknown identities and the
source-clear known-unimplemented subset, crate-private CPU control-flow
snapshot/restore rollback-readiness, crate-private pre-execute sequential
next-PC staging readiness, crate-private committed-step control-flow commit
readiness, pure committed-step cadence planning, crate-private COP0 Count
advancement with timer-pending latch readiness, crate-private SPECIAL shift
GPR writeback execution readiness for source-clear 32-bit and 64-bit shift
identities, crate-private SPECIAL bitwise logical GPR writeback execution
readiness for source-clear register-register logical identities, crate-private
SPECIAL HI/LO transfer execution readiness for source-clear scalar/GPR
transfer identities, crate-private SPECIAL non-trapping integer GPR writeback
execution readiness for source-clear word arithmetic, doubleword arithmetic,
and compare identities, crate-private SPECIAL trapping integer
readiness/writeback with overflow outcome for source-clear signed arithmetic
identities, crate-private immediate trapping integer readiness/writeback with
overflow outcome for source-clear `ADDI`/`DADDI` signed immediate arithmetic,
narrow arithmetic-overflow exception entry readiness, stopped-step outcome
readiness for source-clear SYSCALL/BREAK identities, no-effect executed-step
outcome readiness for the source-clear SYNC identity, and crate-private
CPU-local executed-helper selection and invocation seams plus Machine-owned
CPU-local invocation outcome to future step-action planning and committed
success cadence composition plus arithmetic-overflow exception application,
non-CPU-local frontier application, classified action application, current-PC
classified action production, and represented-category `Machine::step`
composition, the deterministic no-window Rust `fn64_step_probe`, the narrow
Machine-owned `stage_cpu_pc` inspection staging surface, the seam 086
single-owner/visibility consolidation audit, seam 087 repository adoption, and
the seam 088 verification-policy promotion only.
Rust is implementation material, not fn64's product identity. C++ source paths,
targets, commands, equivalence statements, and proof names in older rows are
historical Git anchors only: they are not current files, runnable gates, parity
requirements, or claims that retired behavior was migrated.

## Current Post-Retirement Authority

| Current fact | Owner/status |
| --- | --- |
| Product source | The tracked Rust workspace is the sole current implementation. |
| Required gate | `rust/verify-forward`, with five Rust-only stages. |
| Retired C++ lane | Absent from the current tree; Git history is the only archive. |
| Parity prerequisite | Intentionally waived by `USER_DECISION`. |
| Unported C++ behavior | `INTENTIONALLY_ABSENT_AFTER_CPP_RETIREMENT`; not migrated. |
| Capability boundary | Rust remains incomplete, headless, and makes no cartridge-boot, game, SDL/window/audio, or broad compatibility claim. |

All later seam tables remain useful as dated reconstruction records. Any C++
command shown there is historical and unavailable in the current checkout.

## Seam 088 Forward Verification Policy

This section records the seam-088 transition that established the Rust gate.
The post-retirement authority section above supersedes its transitional C++
retention status.

| Verification fact | Current owner/status |
| --- | --- |
| Required default forward gate | `rust/verify-forward`; the sole executable owner of the required command order. |
| Required stages | Rust formatting, clippy with warnings denied, the complete Rust test suite, `fn64_machine_probe`, and `fn64_step_probe`. |
| Construction/reset probe | `fn64_machine_probe` remains deterministic, no-window, and construction/reset-only. |
| Represented step probe | `fn64_step_probe` calls `Machine::step` and covers the eight accepted represented cases. |
| C++ reference checks | Historical seam-088 state; removed from the current tree by the direct retirement decision. |
| C++ deletion readiness | Superseded; parity/inventory prerequisites were intentionally waived. |
| Behavior/API effect | None. No Rust core, inspection-probe, C++, CMake, public-API, or represented machine behavior changed. |

The normal entry point remains `./rust/verify-forward`. Rust remains a bounded,
incomplete N64 implementation with no cartridge boot, game compatibility, or
SDL/window/audio runtime claim.

## Seam 087 Repository Adoption

The complete 24-file Rust workspace accepted through seam 086 is now tracked
repository product truth under `rust/`. Adoption preserves the existing Cargo
workspace and crate layout, includes `Cargo.lock` as reproducibility metadata,
and keeps `rust/.gitignore` as the single owner of `/target/` exclusion. No
Machine behavior, public API, probe case, Rust source layout, C++ source, C++
test, CMake file, or gate policy changed in this adoption pass.

The tracked workspace remains a bounded, incomplete N64 machine-core
implementation. The two Rust no-window probes and represented `Machine::step`
do not claim cartridge boot, PIF/BIOS behavior, game compatibility, or an
SDL/window/audio runtime. At the seam 087 checkpoint, C++ remained present as
frozen reference truth and its checks had not yet been demoted; the seam 088
policy above now makes them optional.

## Seam 086 Current Step-Spine Audit

This section is the authoritative current step-spine state for seams 084 through
086 and supersedes point-in-time absence wording in older incremental ledger
rows. Seam 087 changes repository representation only; C++ remains frozen
reference truth. At that checkpoint its checks were still undemoted; the seam
088 policy above supersedes that historical gate status.

| Owned fact | Current Rust owner | Audit result |
| --- | --- | --- |
| Public represented execution entrance | `Machine::step` | Sole public execution entrance; no `Cpu::step` or generic `execute_cpu_instruction` exists. |
| Current-PC production | `Machine::produce_current_pc_classified_step_action` | Captures one control-flow snapshot, stages sequential `next_pc` once, fetches once, decodes once, identifies once, classifies represented frontiers, selects/invokes at most one CPU-local helper, and does not apply the action. |
| Classified application | `Machine::apply_classified_step_action` | Delegates exactly once to the CPU-local or non-CPU-local applicator and owns no fetch/decode/identify/invocation work. |
| Committed control flow | `Cpu::commit_staged_step_control_flow` | Single mutation primitive; sets `pc` from the captured pre-step `next_pc` and preserves the already-staged sequential `next_pc`. |
| Committed Count | `Cpu::advance_count_for_committed_step` delegating to private `Cop0::advance_count_for_committed_step` | Single runtime mutation primitive; CPU-local success, SYNC, SYSCALL, and BREAK invoke it once, while rollback/rejection/exception paths do not. |
| Unsupported/rejection rollback | `Cpu::restore_control_flow` | Single `pc`/`next_pc` restore primitive; unsupported restores in application, while source-clear production rejections restore before returning. |
| Arithmetic-overflow entry | `Machine::apply_cpu_local_arithmetic_overflow_exception` -> `Cpu::enter_arithmetic_overflow_exception` | Restores the snapshot, delegates exception mutation, does not commit cadence or Count, and does not write BadVAddr. |
| Instruction-fetch AdEL entry | `Machine::apply_non_cpu_local_step_frontier_action` -> `Machine::enter_instruction_fetch_address_error_exception` -> CPU/COP0 entry | Producer restores speculative staging before returning the selected action; application delegates the sealed AdEL mutation without Count advancement. |
| Public represented result | `MachineRepresentedStepOutcome` and `MachineRepresentedStepError` plus their payload/cadence descriptor types | Honest represented categories only; no all-future result abstraction. |
| Public PC staging | `Machine::stage_cpu_pc` -> `Cpu::stage_pc` | Kept. Establishes `pc = value` and `next_pc = value.wrapping_add(4)` without mutable CPU/COP0 exposure, policy, fetch, or execute. Its only current production consumer is `fn64_step_probe`, at four synthetic-state call sites. |
| Probe/core boundary | `fn64-inspection/src/bin/fn64_step_probe.rs` | Uses public Machine/RDRAM/CPU-inspection surfaces and calls only `Machine::step` for execution; formatting, assertions, case policy, and exit status stay outside `fn64-core`. |
| Internal planning visibility | Step/frontier classifiers, cadence classifier, and `MachineStepFetchFaultAction` | Narrowed to crate-private after the audit found no external consumer. Public outcome/error and externally inspected descriptor types remain public. |

The probe still covers CPU-local committed success, CPU-local arithmetic
overflow, SYNC no-effect, SYSCALL stopped, BREAK stopped, unsupported rollback,
selected instruction-fetch AdEL, and source-clear fetch rejection. It uses only
generated/synthetic state and ends with `result: ok`.

Seam 087 repository-adoption prerequisite for gate promotion: SATISFIED

## Current Verification Lanes

| Lane | Owner | Policy/result expected |
| --- | --- | --- |
| Required Rust forward lane | `rust/verify-forward` | Required by default; all five Rust stages pass and end with `forward gate: ok`. |
| Focused represented-step check | Direct Cargo filter | Supplementary source-focused check; existing `machine_step` tests pass. |
| Retired C++ history | Git history and the historical context documents | No current target or command; no parity requirement. |

## Behavior Parity Table

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Empty/default cartridge state | `src/core/cartridge.hpp` `Cartridge::Cartridge() = default`; members `source_layout_`, `image_`, `metadata_` | `rust/crates/fn64-core/src/cartridge.rs` `impl Default for Cartridge` | Yes | `default_cartridge_is_empty_big_endian_without_metadata`; C++ source inspection | Both default to big-endian source layout, empty bytes, zero/empty metadata. |
| Accepted byte orders | `src/core/rom.cpp` `detect_rom_source_layout` | `cartridge/byte_order.rs` `detect_rom_source_layout` | Yes | `normalizes_supported_source_layouts_and_loads_cartridge_bytes`; `fn64_selftest` synthetic ROM ingress demo | Accepted headers are exactly `80 37 12 40`, `37 80 40 12`, and `40 12 37 80`. |
| `.z64` normalization | `src/core/rom.cpp` `normalize_rom_bytes` case `kBigEndian` | `cartridge/byte_order.rs` `normalize_rom_bytes` case `BigEndian` | Yes | `normalizes_supported_source_layouts_and_loads_cartridge_bytes` | Native bytes are preserved as canonical big-endian cartridge order. |
| `.v64` normalization | `src/core/rom.cpp` `normalize_rom_bytes` case `kByteSwapped16` | `cartridge/byte_order.rs` `normalize_rom_bytes` case `ByteSwapped16` | Yes | `normalizes_supported_source_layouts_and_loads_cartridge_bytes` | Adjacent bytes are swapped in every 16-bit chunk. |
| `.n64` normalization | `src/core/rom.cpp` `normalize_rom_bytes` case `kLittleEndian32` | `cartridge/byte_order.rs` `normalize_rom_bytes` case `LittleEndian32` | Yes | `normalizes_supported_source_layouts_and_loads_cartridge_bytes` | Bytes are reversed within every 32-bit chunk. |
| Rejected/unknown byte order behavior | `src/core/rom.cpp` `detect_rom_source_layout` | `cartridge/byte_order.rs` `detect_rom_source_layout` and `CartridgeLoadError::UnsupportedHeaderByteLayout` | Yes at behavior level | `rejects_unsupported_or_malformed_rom_inputs` | C++ returns `false` and an error string; Rust returns an explicit error enum whose display text mirrors the C++ message. |
| ROM zero-length input behavior | `src/core/rom.cpp` `normalize_rom_image` | `cartridge.rs` `normalize_rom_image` and `CartridgeLoadError::HeaderTooSmall` | Yes at behavior level | `rejects_unsupported_or_malformed_rom_inputs` | Public C++ gate rejects before layout detection as smaller than the complete 0x40-byte header. Rust does the same. |
| ROM too-small input behavior | `src/core/rom.cpp` `normalize_rom_image` | `cartridge.rs` `normalize_rom_image` and `CartridgeLoadError::HeaderTooSmall` | Yes at behavior level | `rejects_unsupported_or_malformed_rom_inputs` | Rust covers empty, 3-byte, and 0x3f-byte synthetic inputs. |
| ROM size multiple-of-4 check | `src/core/rom.cpp` `normalize_rom_image` | `cartridge.rs` `normalize_rom_image` and `CartridgeLoadError::SizeNotMultipleOf4` | Yes at behavior level | `rejects_unsupported_or_malformed_rom_inputs` | Both reject after complete-header check and before normalization. |
| Header magic validation | `src/core/rom.cpp` `parse_rom_metadata` | `cartridge/metadata.rs` `parse_rom_metadata` and `CartridgeLoadError::NormalizedHeaderMagicMismatch` | Yes at behavior level | `rejects_unsupported_or_malformed_rom_inputs`; source inspection | Public tests reach unknown-layout rejection; magic mismatch is kept as explicit metadata-parse behavior matching C++ source. |
| Metadata numeric extraction | `src/core/rom.cpp` `parse_rom_metadata`, `read_be_u32` | `cartridge/metadata.rs` `parse_rom_metadata`, `read_be_u32` | Yes | `normalizes_supported_source_layouts_and_loads_cartridge_bytes` | Header magic, clock rate, entry point, release address, CRC1, and CRC2 are read from normalized big-endian offsets. |
| Metadata/name extraction | `src/core/rom.cpp` `read_ascii_field`; `parse_rom_metadata` | `cartridge/metadata.rs` `read_ascii_field`; `parse_rom_metadata` | Yes for current ASCII proof data | `normalizes_supported_source_layouts_and_loads_cartridge_bytes`; `extracts_metadata_name_fields_like_cpp_ascii_reader` | Both stop at NUL, replace non-printable ASCII with `?`, and trim trailing spaces. Rust uses ASCII printability explicitly; C++ uses `std::isprint` on `unsigned char`. |
| Cartridge ownership of normalized bytes | `src/core/cartridge.cpp` `load_cartridge`; `Cartridge` private members | `cartridge.rs` `load_cartridge`; `Cartridge` private fields | Yes | `normalizes_supported_source_layouts_and_loads_cartridge_bytes` | Both store normalized bytes, source layout, and metadata in the cartridge object. |
| Entry point/IPL3 inspection | `src/core/cartridge.cpp` `inspect_cartridge_entry` | `cartridge.rs` `inspect_cartridge_entry` | Yes | `entry_inspection_reports_available_and_unavailable_spans`; `fn64_selftest` cartridge entry inspection demo | Header entry word at `0x08`, first IPL3 candidate word at `0x40`, and full candidate span availability are mirrored. |
| Range-checked reads | `src/core/cartridge.cpp` `Cartridge::read_u8`, `require_readable_range` | `cartridge.rs` `Cartridge::read_u8`, `CartridgeReadError::OutOfRange` | Yes at behavior level | `cartridge_read_is_range_checked`; `fn64_selftest` synthetic read guard demo | Both allow reads inside the normalized image and reject out-of-range reads. |
| Range read ending exactly at cartridge length | `src/core/cartridge.cpp` `Cartridge::read_u8` | `cartridge.rs` `Cartridge::read_u8` | Yes | `cartridge_read_is_range_checked` | Last valid byte `size - 1` succeeds; first invalid byte `size` is rejected. |
| Out-of-range read behavior | `src/core/cartridge.cpp` `require_readable_range` throws `std::out_of_range` | `cartridge.rs` `CartridgeReadError::OutOfRange` | Yes at behavior level; API differs | `cartridge_read_is_range_checked`; source inspection | Rust returns `Result` instead of throwing. Display text mirrors the C++ out-of-range message. |
| `load_cartridge` failure output reset | `src/core/cartridge.cpp` `load_cartridge` assigns empty big-endian `Cartridge` to `out_cartridge` on failure | `cartridge.rs` `load_cartridge` returns `Err`; `Cartridge::default` represents the same empty state | API differs; behavior documented | `default_cartridge_is_empty_big_endian_without_metadata`; source inspection | Rust uses a bytes-in/`Result` API and does not mutate an out parameter. This is Rust API shape, not emulator truth. |
| Error representation | C++ `bool` plus `std::string& error`, and exceptions for `read_u8` | Rust `Result` with `CartridgeLoadError` / `CartridgeReadError` | Rust-only helper, no emulator truth | Rust tests compare variants and display text | Rust makes failures explicit with enums. This does not add emulator behavior. |
| Host file path ownership | `src/host/cli/inspect_main.cpp` and `src/host/sdl/app.cpp` read files before calling core `load_cartridge` | No Rust core file path API | Yes boundary preserved | Source inspection | Rust core accepts bytes only and owns no filesystem policy. |
| CPU/RDRAM/Machine execution | `src/core/machine.hpp`, `src/core/machine.cpp`, `src/core/machine_cpu.cpp` | No generic Rust execute, step, RDRAM range writes/runtime, bus, or memory-map implementation; crate-private SPECIAL shift, bitwise logical, HI/LO transfer, non-trapping integer, trapping integer, and immediate trapping integer helpers only | Not yet earned for generic execute/step | C++ gates only | Rust mirrors construction/default-state ownership facts, raw RDRAM byte/u16_be/u32_be/u64_be storage reads and writes, direct CPU-address-to-RDRAM classification, direct RDRAM value access, pure raw instruction-word field decode, pure instruction identity classification, instruction-fetch target classification, direct RDRAM CPU instruction-word fetch, read-only SP DMEM CPU instruction-word fetch, explicit-address instruction fetch over represented targets, current-PC instruction fetch wrapper, pure instruction-fetch fault address-error selection, narrow instruction-fetch address-error entry mutation, pure step fetch-fault action classification, unsupported-step outcome classification for unknown and source-clear known-unimplemented identities, stopped-step outcome classification for source-clear SYSCALL/BREAK identities, no-effect executed-step outcome classification for source-clear SYNC, crate-private CPU control-flow snapshot/restore, pre-execute next-PC staging, committed-step control-flow commit, pure committed-step cadence planning, crate-private COP0 Count advancement with timer-pending latch, crate-private SPECIAL shift, bitwise logical, HI/LO transfer, non-trapping integer, trapping integer, and immediate trapping integer execution/readiness, and narrow arithmetic-overflow exception entry only. Generic execution, step, generic exception machinery, and runtime memory behavior remain explicit non-goals. |
| Direct CPU-address-to-RDRAM classification | `src/core/machine.cpp` `translate_direct_cpu_physical_address`, `translate_cpu_rdram_address`, `translate_cpu_physical_rdram_address`; `src/core/machine.hpp` `CpuAddress`, `RdramOffset` | `rust/crates/fn64-core/src/cpu/address.rs` `CpuAddress`, `RdramOffset`, `CpuAddressTarget`, `classify_direct_rdram_address` | Equivalent for direct RDRAM subset | `cargo test`; source inspection | Rust mirrors KSEG0/KSEG1 direct RDRAM alias classification and RDRAM span bounds. It does not implement `require_cpu_data_target`, SP/MMIO/device targets, CPU load/store, exceptions, or a bus. |
| Direct CPU-addressed RDRAM value access | `src/core/machine_cpu.cpp` `read_cpu_memory_u8/u16_be/u32_be/u64_be` and `write_cpu_memory_u8/u16_be/u32_be/u64_be` RDRAM target arms; `src/core/machine.cpp` raw RDRAM read/write helpers | `machine.rs` `Machine::read_direct_rdram_u8/u16_be/u32_be/u64_be`, `Machine::write_direct_rdram_u8/u16_be/u32_be/u64_be`; `DirectRdramAccessError` | Equivalent for direct RDRAM subset; C++ broader target resolver intentionally absent | Direct value-access tests; source inspection | Rust resolves only direct KSEG0/KSEG1 RDRAM spans through the sealed classifier, then calls raw RDRAM reads/writes. This is not load/store instruction behavior, sign extension, GPR writeback, exception delivery, memory map, bus, device, DMA, or LL/SC. |
| Pure CPU data-address alignment contract | `src/core/machine_cpu.cpp` instruction cases check low address bits before read/write helper calls; `fail_unaligned_*_memory_access` tags data read/write intent | `cpu/address.rs` `CpuDataWidth`, `CpuDataAccessKind`, `CpuDataAlignmentError`, `check_cpu_data_alignment` | Equivalent for pure low-bit contract; exceptions intentionally absent | CPU address alignment tests; source inspection | Byte accepts all low bits; halfword/word/doubleword require low bits 0 modulo 2/4/8. Rust records read/write kind for future AdEL/AdES mapping but does not enter exceptions or mutate COP0. |
| Pure CPU data address-error exception selection | `src/core/machine_cpu.cpp` `step_cpu_instruction` maps `kDataRead` to `kCop0ExceptionCodeAddressErrorLoad` and `kDataWrite` to `kCop0ExceptionCodeAddressErrorStore` before exception entry | `cpu/address.rs` `CpuAddressErrorKind`, `CpuDataAddressError`, `select_cpu_data_address_error` | Equivalent for pure selection | CPU data address-error selection tests; source inspection | Rust maps read alignment faults to AdEL/code 4 and write alignment faults to AdES/code 5 while preserving address, width, and access kind. The selection value itself remains pure; seam 039 owns the narrow entry mutation. |
| CPU data address-error exception entry | `src/core/machine_cpu.cpp` `local_synchronous_exception_entry_allowed`, `local_delay_slot_synchronous_exception_entry_allowed`, and `enter_local_address_error_exception` | `cpu/cop0.rs` `Cpu::enter_data_address_error_exception`; `CpuAddressErrorExceptionEntryError` | Equivalent for narrow data address-error entry | CPU data address-error entry tests; source inspection | Rust consumes sealed `CpuDataAddressError`, requires the same EXL-clear ordinary/delay-slot cadence, writes BadVAddr, exception code, EPC, branch-delay flag, Status.EXL, PC, and next PC. It does not execute instructions, write GPRs, or touch RDRAM. |
| Machine-owned direct RDRAM CPU data access preflight and target rejection | `src/core/machine_cpu.cpp` aligned instruction cases check alignment before `read_cpu_memory_*`/`write_cpu_memory_*`; helpers resolve targets; `step_cpu_instruction` maps `kCpuRdramAddressRejected` data read/write faults to AdEL/AdES | `machine.rs` `Machine::read_direct_rdram_cpu_data_u8/u16_be/u32_be/u64_be`, `Machine::write_direct_rdram_cpu_data_u8/u16_be/u32_be/u64_be`, `MachineDirectRdramCpuDataAccessError` | Equivalent for direct RDRAM subset, alignment address-error preflight, and direct target-rejection address-error entry | Direct RDRAM CPU data preflight/rejection tests; source inspection | Rust composes sealed alignment, AdEL/AdES selection, narrow address-error entry, direct RDRAM classification, and raw value access. Lower-level direct APIs still return direct-access errors. This is not CPU load/store instruction behavior, sign/zero extension, GPR writeback, memory map, bus, device, DMA, LL/SC, or execution. |
| Machine reset | `src/core/machine.hpp` / `.cpp` `Machine::reset_to_non_boot_power_on_state` | `machine.rs` `Machine::reset` | Equivalent for represented Machine-owned reset state; full C++ reset state intentionally absent | Machine reset tests; source inspection | Rust resets CPU scalar/GPR/COP0 state, RDRAM bytes, CpuRdramReservation, and `powered_on`, while preserving Cartridge. SP/PIF/device shadows are not represented and remain out of scope. |
| Rust no-window machine probe | `src/proof/selftest_main.cpp` no-window proof runner, `src/proof/bootstrap_data.cpp` construction/reset proof subset, `src/host/cli/step_probe_main.cpp` no-window operational probe role | `rust/crates/fn64-inspection/src/lib.rs` `run_machine_probe`; `rust/crates/fn64-inspection/src/bin/fn64_machine_probe.rs` | Equivalent for construction/reset inspection subset; C++ step/execution probe responsibilities intentionally absent | `cargo test`; `cargo run -p fn64-inspection --bin fn64_machine_probe`; C++ source inspection | The Rust probe constructs a Machine from synthetic in-memory cartridge bytes, inspects sealed construction facts, dirties sealed represented state, calls `Machine::reset`, inspects reset facts, prints deterministic no-window output, and exits. It adds no CPU step, execution, memory map, bus, device, DMA, SDL/window runtime, ROM path policy, or C++ integration. |
| CPU step readiness audit | `src/core/machine.hpp` `Machine::step_cpu_instruction`; `src/core/machine_cpu.cpp` private fetch/decode/identify/execute path; `src/host/cli/step_probe_main.cpp` | Represented Rust `Machine::step` exists only for currently sealed categories; `rust/PARITY.md` records the remaining readiness map | Narrow represented-category composition only | Source inspection; C++ gates; Rust gates | C++ step remains broader than Rust because interrupts, branch/load/store/COP0/ERET/LL/SC, bus/device routing, and full probe compatibility remain absent. Rust intentionally adds no fake full-step or generic execute API. |
| Raw CPU instruction-word decode | `src/core/machine.hpp` `CpuInstructionWord`, `DecodedCpuInstructionWord`; `src/core/machine_cpu.cpp` `decode_cpu_instruction_word` | `cpu/instruction.rs` `CpuInstructionWord`, `CpuInstructionFields`, `decode_cpu_instruction_word` | Equivalent for raw field extraction subset | Decode tests; source inspection | Rust decodes an already-formed `u32` instruction word into raw fields. It does not fetch, convert endian order, execute, step, mutate state, sign/zero extend as semantics, or form branch/jump targets. |
| CPU instruction identity classification | `src/core/machine.hpp` `CpuInstructionIdentity`; `src/core/machine_cpu.cpp` `identify_cpu_instruction` | `cpu/instruction.rs` `CpuInstructionIdentity`, `identify_cpu_instruction` | Equivalent for pure identity classification | Identity tests; source inspection | Rust classifies already-decoded raw fields through the same primary, SPECIAL, REGIMM, COP0, coarse coprocessor/cache/load/store, and unknown identity boundaries. It does not fetch, execute, step, interpret operands, mutate state, sign/zero extend, or enter exceptions. |
| CPU instruction-fetch target classification | `src/core/machine_cpu.cpp` `fetch_cpu_instruction_word` target split; `src/core/machine.cpp` direct alias/RDRAM/SP/PIF helpers | `machine.rs` `Machine::classify_cpu_instruction_fetch_target`; `MachineCpuInstructionFetchTarget`; `MachineCpuInstructionFetchTargetError` | Equivalent target split, no read behavior | Instruction-fetch target classification tests; source inspection | Rust names aligned DirectRdram, SP DMEM, unavailable PIF reset, non-direct unsupported, direct-target miss, and unaligned fetch cases without reading memory, entering exceptions, or creating a bus/map. |
| Direct RDRAM CPU instruction-word fetch | `src/core/machine_cpu.cpp` `fetch_cpu_instruction_word` direct-RDRAM path; `src/core/machine.cpp` direct alias/RDRAM translation; `read_rdram_u32_be` | `machine.rs` `Machine::fetch_direct_rdram_cpu_instruction_word`; `MachineDirectRdramCpuInstructionFetchError` | Equivalent for direct RDRAM subset, different public shape | Direct RDRAM instruction-fetch tests; source inspection | Rust takes an explicit `CpuAddress`, checks 4-byte instruction alignment first, resolves only direct KSEG0/KSEG1 RDRAM, reads exactly one big-endian u32, and returns `CpuInstructionWord`. It does not fetch from SP DMEM/PIF, mutate PC/next PC/Count/COP0/GPR/RDRAM/reservation, enter exceptions, decode, identify, execute, step, or create a memory map/bus. |
| SP DMEM represented storage and CPU instruction-word fetch | `src/core/machine.hpp` `sp_dmem_`, `kSpMemorySizeBytes`; `src/core/machine.cpp` reset zero-fill; `src/core/machine_cpu.cpp` `read_sp_memory_u32_be`, SP DMEM branch in `fetch_cpu_instruction_word` | `sp_dmem.rs` `SpDmem`, `SpDmemOffset`, `SpDmem::read_u32_be`; `machine.rs` `sp_dmem`, `Machine::fetch_sp_dmem_cpu_instruction_word` | Equivalent for represented SP DMEM storage and read-only instruction-word fetch | SP DMEM storage/fetch tests; source inspection | Rust owns 4 KiB zero-filled SP DMEM and forms one big-endian `CpuInstructionWord` from a classified SP DMEM offset. Public SP DMEM writes, SP registers/status/control, SP DMA, SP IMEM, PIF/reset bytes, step-owned full fetch, and step remain absent. |
| Explicit-address CPU instruction-word fetch over represented targets | `src/core/machine_cpu.cpp` `fetch_cpu_instruction_word` alignment check, target split, direct RDRAM read branch, SP DMEM read branch, unavailable PIF reset branch, non-direct rejection, and direct-target-miss rejection | `machine.rs` `Machine::fetch_cpu_instruction_word_at`; `MachineCpuInstructionFetchError` | Equivalent for explicit-address represented target composition; C++ current-PC source intentionally absent | Explicit-address instruction-fetch tests; source inspection | Rust takes a caller-supplied `CpuAddress`, classifies it, dispatches DirectRdram to `Machine::fetch_direct_rdram_cpu_instruction_word`, dispatches SP DMEM to `Machine::fetch_sp_dmem_cpu_instruction_word`, and returns named errors for Unaligned, NonDirectUnsupported, DirectTargetMiss, and PifResetUnavailable. It does not read current PC, decode, identify, enter exceptions, mutate PC/Count/COP0/GPR/RDRAM/SP DMEM/reservation, execute, step, or create a memory map/bus. |
| Current-PC CPU instruction-word fetch wrapper | `src/core/machine_cpu.cpp` `fetch_cpu_instruction_word` reads `cpu_pc()` before the same fetch target/source split | `machine.rs` `Machine::fetch_current_cpu_instruction_word` | Equivalent current-PC wrapper, no step behavior | Current-PC fetch tests; source inspection | Rust reads the represented `Cpu::pc()`, converts it to `CpuAddress`, and delegates to `Machine::fetch_cpu_instruction_word_at`. It does not read `next_pc`, advance PC/next PC/Count, decode, identify, execute, enter exceptions, or create a memory map/bus. |
| Instruction-fetch fault to address-error selection | `src/core/machine_cpu.cpp` `step_cpu_instruction` catch block maps selected fetch faults to `kCop0ExceptionCodeAddressErrorLoad` before local entry | `machine.rs` `select_cpu_instruction_fetch_address_error`; `MachineInstructionFetchAddressErrorPlan` | Equivalent for pure AdEL/code 4 selection only | Instruction-fetch fault selection tests; source inspection | Rust maps `Unaligned`, `DirectTargetMiss`, and `PifResetUnavailable` fetch errors to a pure AdEL plan preserving the fetch address as future BadVAddr. `NonDirectUnsupported` and source-specific lower errors remain non-converting. No fetch API enters exceptions, mutates COP0/PC/Count/GPR, or implements step. |
| Step fetch-fault action classification | `src/core/machine_cpu.cpp` `step_cpu_instruction` fetch `MachineFault` catch either enters local AdEL for selected faults or rethrows | `machine.rs` `MachineStepFetchFaultAction`; `classify_step_fetch_fault_action` | Equivalent for pure action classification only | Step fetch-fault action tests; source inspection | Rust classifies already-returned fetch errors as either `EnterAddressError(plan)` for selected AdEL faults or `Rethrow(fetch_error)` for non-converting faults. It does not add `Machine::step`, a step result type, exception entry, PC/Count cadence, decode, identify, execute, memory map, or bus behavior. |
| Unsupported-step outcome classification | `src/core/machine_cpu.cpp` unknown identity defaults, known identity fallthrough defaults, and invalid COP0 register arms return `kUnsupported`; `step_cpu_instruction` reports `kUnsupported` after restoring PC/next PC | `machine.rs` `MachineStepUnsupportedInstruction`; `classify_step_unsupported_instruction` | Equivalent for unknown plus source-clear known-unimplemented subset only | Unsupported-instruction outcome tests; source inspection | Rust names unknown identities, coarse COP0/COP1/COP2/COP3, CACHE, coprocessor memory identities, and invalid COP0 MFC0/MTC0 register forms. ERET unsupported context, implemented COP0 forms, execution-owned rollback trigger, PC/next PC cadence, Count cadence, and full step result shape remain absent. |
| Stopped-step outcome classification | `src/core/machine_cpu.cpp` `execute_cpu_instruction` returns `CpuInstructionExecutionResult::kStopped` for `kSpecialSyscall` and `kSpecialBreak`; `step_cpu_instruction` commits normal PC/Count cadence before returning `CpuInstructionStepResult::kStopped` | `machine.rs` `MachineStepStoppedInstruction`; `classify_step_stopped_instruction` | Equivalent pure readiness for SYSCALL/BREAK only | Stopped-instruction outcome tests; source inspection | Rust names the source-clear stop identities without execute, cadence commit, Count mutation, syscall/break exception behavior, host stop/runtime behavior, or a step result type. `SYNC` is represented only by the separate no-effect executed-readiness classifier. |
| No-effect executed-step outcome classification | `src/core/machine_cpu.cpp` `execute_cpu_instruction` returns `CpuInstructionExecutionResult::kExecuted` directly for `kSpecialSync`; `step_cpu_instruction` commits normal PC/Count cadence before returning `kStepped` | `machine.rs` `MachineStepNoEffectExecutedInstruction`; `classify_step_no_effect_executed_instruction` | Equivalent pure readiness for SYNC only | No-effect executed-instruction outcome tests; source inspection | Rust names the source-clear no-effect executed identity without execute, cadence commit, Count mutation, side effects, or a step result type. NOP/SLL is intentionally excluded from the no-effect classifier because seam 064 owns it through the normal SPECIAL shift writeback path. |
| SPECIAL 32-bit shift GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kSpecialSll`, `kSpecialSrl`, `kSpecialSra`, `kSpecialSllv`, `kSpecialSrlv`, and `kSpecialSrav`; helpers `read_cpu_gpr_word`, `write_cpu_gpr_word_sign_extended_result`, `variable_shift_amount_u32`, `arithmetic_shift_right_u32` | `cpu/instruction.rs` `Cpu::execute_special_shift_instruction`; `CpuSpecialShiftExecutedInstruction`; `CpuSpecialShiftExecutionError` | Equivalent crate-private helper for represented 32-bit SPECIAL shift subset | SPECIAL shift execution tests; source inspection | Rust reads source GPR word values before destination writeback, fixed shifts use `sa`, variable shifts use `rs & 0x1f`, SRA/SRAV perform arithmetic right shift over the 32-bit word, and results are sign-extended to a CPU register value before writing through the sealed zero-register rule. It does not fetch, decode, identify, mutate PC/next PC/Count/COP0/RDRAM/SP DMEM/reservation, enter exceptions, commit cadence, branch, add generic execute, or add step. |
| SPECIAL 64-bit shift GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kSpecialDsll`, `kSpecialDsrl`, `kSpecialDsra`, `kSpecialDsll32`, `kSpecialDsrl32`, `kSpecialDsra32`, `kSpecialDsllv`, `kSpecialDsrlv`, and `kSpecialDsrav`; helpers `read_cpu_gpr_value`, `write_cpu_gpr_value`, `variable_shift_amount_cpu_value`, `arithmetic_shift_right_cpu_value` | `cpu/instruction.rs` `Cpu::execute_special_shift_instruction`; `CpuSpecialShiftExecutedInstruction`; `CpuSpecialShiftExecutionError` | Equivalent crate-private helper for represented 64-bit SPECIAL shift subset | 64-bit SPECIAL shift execution tests; source inspection | Rust reads full 64-bit source GPR values before destination writeback, fixed shifts use `sa`, `*32` shifts use `sa + 32`, variable shifts use `rs & 0x3f`, DSRA/DSRA32/DSRAV perform arithmetic right shift over the signed 64-bit value, and results are written through the sealed zero-register rule. It does not fetch, decode, identify, mutate PC/next PC/Count/COP0/RDRAM/SP DMEM/reservation, enter exceptions, commit cadence, branch, add generic execute, or add step. |
| SPECIAL bitwise logical GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kSpecialAnd`, `kSpecialOr`, `kSpecialXor`, and `kSpecialNor`; helpers `read_cpu_gpr_value` and `write_cpu_gpr_value` | `cpu/instruction.rs` `Cpu::execute_special_bitwise_logical_instruction`; `CpuSpecialBitwiseLogicalExecutedInstruction`; `CpuSpecialBitwiseLogicalExecutionError` | Equivalent crate-private helper for represented SPECIAL logical subset | SPECIAL bitwise logical execution tests; source inspection | Rust reads full 64-bit `rs` and `rt` values before destination writeback, computes AND/OR/XOR/NOR over the full GPR value, and writes through the sealed zero-register rule. Immediate logical instructions remain separate. It does not fetch, decode, identify, mutate PC/next PC/Count/COP0/RDRAM/SP DMEM/reservation, enter exceptions, commit cadence, branch, add generic execute, or add step. |
| SPECIAL HI/LO transfer execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kSpecialMfhi`, `kSpecialMthi`, `kSpecialMflo`, and `kSpecialMtlo`; helpers `cpu_hi`, `cpu_lo`, `write_cpu_hi`, `write_cpu_lo`, `read_cpu_gpr_value`, and `write_cpu_gpr_value` | `cpu/instruction.rs` `Cpu::execute_special_hi_lo_transfer_instruction`; `CpuSpecialHiLoTransferExecutedInstruction`; `CpuSpecialHiLoTransferExecutionError` | Equivalent crate-private helper for represented SPECIAL HI/LO transfer subset | SPECIAL HI/LO transfer execution tests; source inspection | Rust moves full-width values between HI/LO and GPR state: MFHI writes `rd = HI`, MFLO writes `rd = LO`, MTHI writes `HI = rs`, and MTLO writes `LO = rs`. It preserves zero-register writes for MFHI/MFLO and leaves multiply/divide separate. It does not fetch, decode, identify, mutate PC/next PC/Count/COP0/RDRAM/SP DMEM/reservation, enter exceptions, commit cadence, branch, add generic execute, or add step. |
| SPECIAL non-trapping integer GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kSpecialAddu`, `kSpecialSubu`, `kSpecialDaddu`, `kSpecialDsubu`, `kSpecialSlt`, and `kSpecialSltu`; helpers `read_cpu_gpr_word`, `write_cpu_gpr_word_sign_extended_result`, `read_cpu_gpr_value`, `write_cpu_gpr_value`, `signed_cpu_value_less_than`, `unsigned_cpu_value_less_than`, and `cpu_value_from_bool` | `cpu/instruction.rs` `Cpu::execute_special_non_trapping_integer_instruction`; `CpuSpecialNonTrappingIntegerExecutedInstruction`; `CpuSpecialNonTrappingIntegerExecutionError` | Equivalent crate-private helper for represented SPECIAL non-trapping integer subset | SPECIAL non-trapping integer execution tests; source inspection | Rust reads full `rs` and `rt` source values before destination writeback. ADDU/SUBU operate on low 32-bit words with wrapping arithmetic and sign-extend the word result; DADDU/DSUBU use full-width wrapping arithmetic; SLT/SLTU write 1 or 0 from signed/unsigned full-width comparison. Trapping ADD/SUB/DADD/DSUB and overflow exceptions remain separate. It does not fetch, decode, identify, mutate PC/next PC/Count/COP0/RDRAM/SP DMEM/reservation, enter exceptions, commit cadence, branch, add generic execute, or add step. |
| SPECIAL trapping integer readiness/writeback and overflow entry | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kSpecialAdd`, `kSpecialSub`, `kSpecialDadd`, and `kSpecialDsub`; `fail_signed_arithmetic_overflow`; `step_cpu_instruction` signed-overflow catch; `enter_local_signed_overflow_exception` | `cpu/instruction.rs` `Cpu::execute_special_trapping_integer_instruction`; `CpuSpecialTrappingIntegerExecutionOutcome`; `CpuSpecialTrappingIntegerOverflow`; `cpu/cop0.rs` `Cpu::enter_arithmetic_overflow_exception` | Equivalent for represented helper and narrow entry only | SPECIAL trapping integer and arithmetic-overflow entry tests; source inspection | Rust reads `rs`/`rt` before writeback. ADD/SUB use low 32-bit signed arithmetic and sign-extended successful results; DADD/DSUB use full-width signed overflow detection and full-width successful results. Overflow returns a narrow outcome before writeback; the separate overflow entry sets Cause code 12, EPC, branch-delay flag, EXL, and vector without BadVAddr or Count mutation. Nothing wires overflow outcomes to exception entry, step, commit, Count, generic execute, or generic exception dispatch. |
| Immediate trapping integer readiness/writeback | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kAddi` and `kDaddi`; `i16_from_u16_bits`; `sign_extend_u16_to_cpu_value`; `fail_signed_arithmetic_overflow`; `step_cpu_instruction` signed-overflow catch | `cpu/instruction.rs` `Cpu::execute_immediate_trapping_integer_instruction`; `CpuImmediateTrappingIntegerExecutionOutcome`; `CpuImmediateTrappingIntegerOverflow`; `cpu/cop0.rs` existing `Cpu::enter_arithmetic_overflow_exception` | Equivalent for represented helper; entry reused but unwired | Immediate trapping integer tests; source inspection | Rust reads `rs` before writeback. ADDI interprets the raw immediate as signed `i16`, uses low-word signed arithmetic, and sign-extends successful word results; DADDI sign-extends the raw immediate to a full CPU value and uses full-width signed overflow detection. Overflow returns a narrow outcome before writeback. Generic immediate semantics, overflow-to-entry wiring, step, commit, Count, generic execute, and generic exception dispatch remain absent. |
| Immediate non-trapping integer GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kAddiu` and `kDaddiu`; `sign_extend_u16_to_u32`; `sign_extend_u16_to_cpu_value`; `write_cpu_gpr_word_sign_extended_result`; `write_cpu_gpr_value` | `cpu/instruction.rs` `Cpu::execute_immediate_non_trapping_integer_instruction`; `CpuImmediateNonTrappingIntegerExecutedInstruction`; `CpuImmediateNonTrappingIntegerExecutionError` | Equivalent for represented helper | Immediate non-trapping integer tests; source inspection | Rust reads `rs` before writeback. ADDIU sign-extends raw `immediate_u16` to a word operand, uses low-word wrapping arithmetic, and sign-extends the successful word result; DADDIU sign-extends raw `immediate_u16` to a full CPU value and uses full-width wrapping arithmetic. No overflow outcome or exception entry is produced. ADDI/DADDI, SLTI/SLTIU, ANDI/ORI/XORI, LUI, generic immediate semantics, step, commit, Count, generic execute, and memory/bus/device routing remain separate. |
| Immediate comparison GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kSlti` and `kSltiu`; `sign_extend_u16_to_cpu_value`; `signed_cpu_value_less_than`; `unsigned_cpu_value_less_than`; `cpu_value_from_bool`; `write_cpu_gpr_value` | `cpu/instruction.rs` `Cpu::execute_immediate_comparison_instruction`; `CpuImmediateComparisonExecutedInstruction`; `CpuImmediateComparisonExecutionError` | Equivalent for represented helper | Immediate comparison tests; source inspection | Rust reads `rs` before writeback. SLTI sign-extends raw `immediate_u16` to a full CPU value and performs signed full-width comparison; SLTIU uses the same sign-extended immediate value and performs unsigned full-width comparison. Both write `rt` as 1 or 0. ADDI/DADDI, ADDIU/DADDIU, ANDI/ORI/XORI, LUI, generic immediate semantics, step, commit, Count, generic execute, and memory/bus/device routing remain separate. |
| Immediate bitwise logical GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` cases `kAndi`, `kOri`, and `kXori`; raw `immediate_u16` cast to `CpuRegisterValue`; `write_cpu_gpr_value` | `cpu/instruction.rs` `Cpu::execute_immediate_bitwise_logical_instruction`; `CpuImmediateBitwiseLogicalExecutedInstruction`; `CpuImmediateBitwiseLogicalExecutionError` | Equivalent for represented helper | Immediate bitwise logical tests; source inspection | Rust reads `rs` before writeback. ANDI/ORI/XORI zero-extend the raw `immediate_u16` only inside this instruction family, operate over full 64-bit GPR values, and write `rt` through sealed GPR semantics. LUI, SLTI/SLTIU, ADDI/DADDI/ADDIU/DADDIU, generic immediate semantics, generic zero-extension semantics, step, commit, Count, generic execute, and memory/bus/device routing remain separate. |
| Upper-immediate LUI GPR writeback execution | `src/core/machine_cpu.cpp` `execute_cpu_instruction` case `kLui`; raw `immediate_u16` shifted left 16 as `std::uint32_t`; `write_cpu_gpr_word_sign_extended_result` | `cpu/instruction.rs` `Cpu::execute_upper_immediate_instruction`; `CpuUpperImmediateExecutedInstruction`; `CpuUpperImmediateExecutionError` | Equivalent for represented helper | Upper-immediate LUI tests; source inspection | Rust ignores `rs`, shifts raw `immediate_u16` left 16 into a word result, sign-extends that 32-bit word to a CPU value, and writes `rt` through sealed GPR semantics. ANDI/ORI/XORI, SLTI/SLTIU, ADDI/DADDI/ADDIU/DADDIU, generic immediate semantics, generic upper-immediate semantics, step, commit, Count, generic execute, and memory/bus/device routing remain separate. |
| CPU-local executed-helper selection | `src/core/machine_cpu.cpp` `execute_cpu_instruction` identity cases for SYNC and already sealed CPU-local helper families return or prepare `kExecuted`/overflow outcome without pre-selection context checks; branch/load/store/COP0/ERET/LL/SC/trap/multiply/divide/stopped/unsupported cases remain separate | `cpu/instruction.rs` `CpuLocalExecutedHelperSelection`, `CpuLocalExecutedHelperFamily`, and `select_cpu_local_executed_helper` | Equivalent for pure identity-to-family selection only | Selector tests; Machine no-mutation test; source inspection | Rust names only already sealed helper families. It does not call execution helpers, mutate state, fetch/decode/identify internally, commit cadence, advance Count, process exceptions, or create generic execute/step machinery. |
| CPU-local executed-helper invocation | `src/core/machine_cpu.cpp` `execute_cpu_instruction` directly invokes already represented CPU-local helper cases and returns `kExecuted`; signed arithmetic overflow is thrown from execute and converted to exception entry only by `step_cpu_instruction` | `cpu/instruction.rs` `Cpu::invoke_cpu_local_executed_helper`; `CpuLocalExecutedHelperInvocationOutcome`; `CpuLocalExecutedHelperInvocationError` | Equivalent for selected already sealed CPU-local helper families only | Invocation tests; Machine preservation test; source inspection | Rust takes already decoded fields plus a source-clear selection, calls exactly one sealed CPU-local helper, and returns `Executed` or `ArithmeticOverflow`. It never fetches, decodes, identifies, calls Machine, commits cadence, advances Count, enters arithmetic-overflow exception entry, touches memory/reservation, or creates generic execute/step machinery. |
| CPU-local invocation outcome step-action planning | `src/core/machine_cpu.cpp` `step_cpu_instruction` maps `kExecuted` to normal commit/Count cadence and maps signed-overflow thrown from execute to local overflow exception entry without committing or advancing Count | `machine.rs` `MachineCpuLocalInvocationStepActionPlan`; `classify_cpu_local_invocation_step_action` | Equivalent pure planning for already produced local invocation outcomes only | Step-action plan tests; source inspection | Rust maps successful local invocation to the already sealed committed cadence plan and maps arithmetic overflow to future arithmetic-overflow exception-entry planning. Invocation errors are Rust-side rejections. The classifier does not call execution helpers, commit cadence, advance Count, enter exceptions, fetch/decode/identify, mutate state, or create generic step machinery. |
| Committed CPU-local success cadence composition | `src/core/machine_cpu.cpp` `step_cpu_instruction` commits non-branch-likely `kExecuted` by assigning `cpu_pc_ = current_next_pc` and then calling `advance_cop0_count_after_committed_instruction()` | `machine.rs` `Machine::apply_cpu_local_committed_success_cadence`; `MachineCpuLocalCommittedSuccessCadence` | Equivalent narrow mutation for an already-classified successful CPU-local action only | Committed success cadence tests; source inspection | Rust accepts only `CommitControlFlowAndAdvanceCount`, commits through `Cpu::commit_staged_step_control_flow`, then advances Count through `Cpu::advance_count_for_committed_step`. It performs no fetch/decode/identify/selection/invocation, no exception entry, and no generic step/result behavior. |
| CPU-local arithmetic-overflow exception application | `src/core/machine_cpu.cpp` `step_cpu_instruction` catches signed arithmetic overflow, restores `current_pc/current_next_pc`, enters the local signed-overflow exception when source-clear, and returns `kException` without Count advancement | `machine.rs` `Machine::apply_cpu_local_arithmetic_overflow_exception`; `MachineCpuLocalArithmeticOverflowException` | Equivalent narrow mutation for an already-classified overflow action only | Arithmetic-overflow application tests; source inspection | Rust accepts only `EnterArithmeticOverflowException`, restores the provided `CpuControlFlowSnapshot`, then calls `Cpu::enter_arithmetic_overflow_exception`. It does not commit normal cadence, advance Count, fetch/decode/identify/select/invoke, or create generic exception/step machinery. |
| CPU-local step-action application composition | `src/core/machine_cpu.cpp` `step_cpu_instruction` maps local execution success to normal committed cadence and signed arithmetic overflow to local exception entry, while unsupported/rejected paths do not use this success/overflow path | `machine.rs` `Machine::apply_cpu_local_step_action`; `MachineCpuLocalStepActionApplication` | Equivalent narrow composition for already-classified CPU-local actions only | CPU-local step-action application tests; source inspection | Rust delegates success to `Machine::apply_cpu_local_committed_success_cadence`, delegates overflow to `Machine::apply_cpu_local_arithmetic_overflow_exception`, and rejects invocation errors without mutation. It does not fetch/decode/identify/select/invoke helpers, add `Machine::step`, add generic execute, or add generic step-result machinery. |
| Non-CPU-local step-frontier action application composition | `src/core/machine_cpu.cpp` `step_cpu_instruction` maps SYNC/no-effect, SYSCALL/BREAK stopped, unsupported, and selected fetch-fault actions to distinct cadence/exception branches | `machine.rs` `Machine::apply_non_cpu_local_step_frontier_action`; `MachineNonCpuLocalStepFrontierAction`; `MachineNonCpuLocalStepFrontierApplication` | Equivalent narrow composition for already-classified non-CPU-local frontier actions only | Non-CPU-local frontier application tests; source inspection | Rust applies no-effect SYNC through committed cadence, stopped SYSCALL/BREAK through stopped committed cadence, unsupported through snapshot restore without Count, selected fetch faults through the sealed instruction-fetch AdEL entry, and fetch rejections without mutation. It does not compose CPU-local actions, fetch/decode/identify/select/invoke helpers, add `Machine::step`, add generic execute, or add generic step-result machinery. |
| Classified step-action application composition | `src/core/machine_cpu.cpp` `step_cpu_instruction` separates CPU-local executed success/overflow application from no-effect/stopped/unsupported/fetch-fault frontier application after the corresponding categories are known | `machine.rs` `Machine::apply_classified_step_action`; `MachineClassifiedStepAction`; `MachineClassifiedStepActionApplication` | Equivalent narrow composition for already-classified sealed categories only | Classified step-action application tests; source inspection | Rust delegates CPU-local actions only to `Machine::apply_cpu_local_step_action` and non-CPU-local frontier actions only to `Machine::apply_non_cpu_local_step_frontier_action`. Delegated invocation/fetch rejections return without mutation. The seam adds no future categories, fetch/decode/identify/classification, CPU-local helper selection/invocation, `Machine::step`, generic execute, generic dispatcher, or generic step result. |
| Current-PC classified step-action production | `src/core/machine_cpu.cpp` `step_cpu_instruction` captures `current_pc`/`current_next_pc`, fetches from current PC, decodes/identifies, stages sequential `next_pc`, executes or classifies represented categories, and maps outcomes before final cadence/exception application | `machine.rs` `Machine::produce_current_pc_classified_step_action`; `MachineCurrentPcClassifiedStepAction`; `MachineCurrentPcClassifiedStepActionError` | Equivalent narrow production for currently sealed categories only | Current-PC classified step-action tests; source inspection | Rust captures control flow before staging, stages `next_pc` exactly once, fetches once through the current-PC fetch API, decodes once, identifies once, passes that identity through the represented frontier classifiers and CPU-local selector, invokes only one selected CPU-local helper, maps the result to a classified action, and returns without applying it. Allowed mutation is limited to speculative `next_pc` staging and successful CPU-local helper writeback. It does not commit cadence, advance Count, enter exceptions, call the public step recursively, add generic execute/future categories, or add memory-map/bus/device routing. |
| Represented Machine step composition | `src/core/machine_cpu.cpp` `Machine::step_cpu_instruction` composes current-PC production with outcome application for represented paths | `machine.rs` `Machine::step`; `MachineRepresentedStepOutcome`; `MachineRepresentedStepError` | Equivalent narrow composition for currently sealed categories only | Machine step tests; source inspection | Rust `Machine::step` calls the seam 083 producer, then the seam 082 applicator, then converts the internal application result to a represented-category outcome/error. It covers CPU-local committed success, arithmetic-overflow exception, SYNC no-effect commit, SYSCALL/BREAK stopped commit, unsupported rollback, selected instruction-fetch AdEL entry, and source-clear fetch/invocation/unrepresented rejections. It does not duplicate fetch/decode/identify/select/invoke logic or action application logic, add `Cpu::step`, generic `execute_cpu_instruction`, generic `MachineStepResult`, future placeholders, branch/load/store/COP0/ERET/LL/SC, memory map, bus, or device routing. |
| Unsupported-instruction control-flow rollback readiness | `src/core/machine_cpu.cpp` `step_cpu_instruction` captures `current_pc`/`current_next_pc`, stages speculative `cpu_next_pc_`, and restores `cpu_pc_`/`cpu_next_pc_` when execution returns `kUnsupported` | `cpu/scalars.rs` `CpuControlFlowSnapshot`, `Cpu::capture_control_flow`, `Cpu::restore_control_flow` | Equivalent for the pc/next_pc primitive only | Control-flow snapshot/restore tests; source inspection | Rust can capture and restore only `pc` and `next_pc`. This primitive alone does not implement execute result handling, normal PC cadence, Count cadence, rollback of COP0/GPR/RDRAM/SP DMEM/reservation/Cartridge, generic rollback, savestates, or full Machine step behavior. |
| Pre-execute sequential next-PC staging readiness | `src/core/machine_cpu.cpp` `step_cpu_instruction` assigns `cpu_next_pc_ = sequential_instruction_address(current_next_pc)` before `execute_cpu_instruction` for non-ERET identities | `cpu/scalars.rs` `Cpu::stage_next_sequential_pc_for_step` | Equivalent for the pre-execute next-PC staging primitive only | Step next-PC staging tests; source inspection | Rust advances only `next_pc` by four with wrapping `u32` arithmetic and leaves `pc` unchanged. It does not fetch, decode, identify, execute, tick Count, commit PC, handle branch-likely annul, process interrupts/ERET/exceptions, or add step. |
| Committed-step control-flow commit readiness | `src/core/machine_cpu.cpp` `step_cpu_instruction` assigns `cpu_pc_ = current_next_pc` after non-annul `kExecuted`/`kStopped` execution and leaves `cpu_next_pc_` as already staged or execution-mutated | `cpu/scalars.rs` `Cpu::commit_staged_step_control_flow` | Equivalent for the pc/next_pc primitive only | Control-flow commit tests; source inspection | Rust sets `pc` from a pre-step snapshot's `next_pc` and preserves the already-staged `next_pc`. It does not call Count advancement, fetch, decode, identify, execute, branch, enter exceptions, process interrupts, ERET, or add step. |
| Committed-step cadence plan | `src/core/machine_cpu.cpp` `step_cpu_instruction` maps source-visible outcome paths to commit/restore/vector/return-before-cadence control flow and Count advance/no-advance decisions | `machine.rs` `MachineStepCadencePlan`, `MachineStepControlFlowAction`, `MachineStepCountAction`, `classify_machine_step_cadence` | Equivalent pure plan only | Cadence plan tests; source inspection | Rust names the cadence actions without mutating state. The plan itself does not add `Machine::step`, a generic step result, Count mutation, ERET, branch-likely annul, interrupt handling, exception handling, execute, memory map, or bus behavior. |
| COP0 Count advancement with timer-pending latch | `src/core/machine_cpu.cpp` `advance_cop0_count_after_committed_instruction` increments `cop0_count_` and sets `cop0_timer_interrupt_pending_` if the post-increment Count equals Compare | `cpu/cop0.rs` `Cpu::advance_count_for_committed_step` | Equivalent crate-private primitive | Count advancement tests; source inspection | Rust wraps Count by one using `u32`, checks Compare after increment, latches timer-pending only on equality, preserves existing pending when no equality occurs, and does not clear pending. It does not process interrupts, enter exceptions, commit pc/next PC, execute instructions, or create step/timer machinery. Compare writes remain absent. |
| Cartridge staging / PI DMA / IPL3 SP DMEM entry | `src/core/machine.cpp`; `src/core/machine_cpu.cpp` | No Rust implementation | Not yet earned | C++ gates only | Explicit non-goal for this pass. |
| SDL/window/runtime behavior | `src/host/sdl/app.cpp`, `src/host/sdl/main.cpp` | No Rust implementation | Not yet earned | Not run | Rust remains sidecar-only; no host runtime was added. |

## Seam 002 Audit Changes

- Removed the unused Rust-only `Cartridge::load` associated helper. The plain
  free function `load_cartridge(bytes)` remains the Rust seam entry point.
- Strengthened Rust tests for empty input, 0x3f-byte too-small input, exact
  boundary reads, out-of-range display text, and metadata trimming/nonprintable
  handling.
- No C++ source files were changed.

## Machine Construction/Default-State Parity Table

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Machine type exists | `src/core/machine.hpp` `class Machine` | `rust/crates/fn64-core/src/machine.rs` `pub struct Machine` | Equivalent for type presence only | Source inspection; `cargo test` | Rust introduces the sidecar Machine home in `fn64-core`; it is not integrated into C++. |
| Machine default construction behavior | `src/core/machine.hpp` has `explicit Machine(Cartridge cartridge)` and no default constructor | `machine.rs` has `Machine::from_cartridge(cartridge)` and no `Default` impl | Equivalent | Source inspection; `machine_from_empty_cartridge_is_powered_on_and_owns_cartridge` | C++ Machine construction requires a Cartridge. Rust intentionally does not add `Machine::default` or `Machine::empty`. |
| Machine cartridge ownership | `src/core/machine.hpp` private `Cartridge cartridge_`; `src/core/machine.cpp` `Machine::Machine(Cartridge)` and `Machine::cartridge()` | `machine.rs` private `cartridge: Cartridge`; `Machine::from_cartridge`; `Machine::cartridge` | Equivalent for ownership/access | `machine_from_empty_cartridge_is_powered_on_and_owns_cartridge`; `machine_from_loaded_cartridge_preserves_cartridge_facts`; `fn64_selftest` construction demo | Both Machines own the cartridge passed at construction and expose read-only cartridge inspection. |
| Empty/default cartridge relationship | C++ callers can construct `Machine(Cartridge{})`; proof uses `std::make_unique<Machine>(Cartridge{})` | `Machine::from_cartridge(Cartridge::default())` | Equivalent for owned-cartridge subset | `machine_from_empty_cartridge_is_powered_on_and_owns_cartridge`; source inspection | This proves the relationship between Machine construction and the already-sealed default Cartridge state. |
| Loaded cartridge relationship | C++ proof loads a synthetic ROM into `Cartridge`, then constructs `Machine(std::move(cartridge))` | Rust test loads a synthetic ROM with `load_cartridge`, then constructs `Machine::from_cartridge(cartridge)` | Equivalent for owned-cartridge facts | `machine_from_loaded_cartridge_preserves_cartridge_facts`; C++ `run_machine_construction_isolation_demo` source inspection | Rust only observes cartridge facts through the owned cartridge; it does not stage or execute bytes. |
| Construction powered-on flag | `src/core/machine.cpp` `Machine::Machine` calls `reset_to_non_boot_power_on_state`; `powered_on_ = true`; `powered_on()` | `machine.rs` `Machine::from_cartridge` sets `powered_on: true`; `powered_on()` | Equivalent for flag value | `machine_from_empty_cartridge_is_powered_on_and_owns_cartridge`; `machine_from_loaded_cartridge_preserves_cartridge_facts`; C++ source inspection | Construction and `Machine::reset` both set the local powered-on flag true. |
| C++ reset-state relationship | `src/core/machine.cpp` `reset_to_non_boot_power_on_state` zeroes RDRAM/SP/PIF/CPU/device shadows and sets reset PC/next PC | `Machine::reset` resets represented CPU, RDRAM, reservation, and powered-on state; no SP/PIF/device fields exist | Equivalent for represented state; C++ exists beyond Rust scope | Source inspection; reset tests | Rust does not claim full C++ reset parity because SP/PIF/device shadows are still absent. |
| CPU ownership | `src/core/machine.hpp` CPU PC/next PC, HI/LO, 32 GPRs, COP0 fields, and CPU step helpers | `rust/crates/fn64-core/src/cpu.rs` `Cpu`; `machine.rs` private `cpu: Cpu`; `Machine::cpu` | Equivalent for construction/default-state ownership only | `machine_from_cartridge_owns_cpu_construction_state`; CPU construction tests; source inspection | Rust owns pure raw instruction-word decode and identity classification values under `Cpu`; explicit-address and current-PC instruction fetch are Machine-owned. Rust does not mirror CPU step helpers, Machine-level broad CPU staging, execute, or LL/SC behavior. Private CPU/RDRAM reservation invalidation is owned under Machine, not Cpu. |
| RDRAM ownership | `src/core/machine.hpp` `rdram_` 4 MiB array plus read/write helpers | `rust/crates/fn64-core/src/machine.rs` private `rdram: Rdram`; `Machine::rdram`; raw/direct/direct-CPU-data read/write methods; `rdram.rs` raw read-width methods and private storage mutation | Equivalent for construction ownership, size, raw byte/u16_be/u32_be/u64_be reads, reservation-aware raw byte/u16_be/u32_be/u64_be writes, direct CPU-addressed RDRAM value access, direct RDRAM CPU data preflight composition, and target-rejection address-error entry | Machine construction tests; RDRAM read-width tests; raw write tests; direct value-access tests; direct CPU data preflight/rejection tests; source inspection | Pure raw reads remain RDRAM-owned. Reservation-aware raw/direct/direct-CPU-data writes remain Machine-owned. Direct CPU data access is Machine-owned because it may mutate Cpu/COP0/control-flow on alignment faults or direct target rejection and RDRAM/reservation on aligned successful writes. Rust still has no range access, staging, PI DMA, full memory-map, bus, device routing, or CPU load/store instruction behavior. |
| SP memory / PIF RAM / local device shadows | `src/core/machine.hpp` `sp_dmem_`, `sp_imem_`, `pif_ram_`, SP/MI/PI/AI/SI/COP0 local fields | `sp_dmem.rs` `SpDmem` only; no Rust SP IMEM, PIF RAM, SP/MI/PI/AI/SI device state, DMA, or MMIO APIs | Equivalent for represented SP DMEM storage/fetch only; other C++ state intentionally absent | Source inspection; SP DMEM tests; C++ gates only | Rust represents the SP DMEM byte storage needed for instruction fetch readiness. It does not add SP device/register/status/control behavior or any broader device reset. |
| Reset API | `src/core/machine.hpp` / `.cpp` `reset_to_non_boot_power_on_state()` | `machine.rs` `Machine::reset` | Equivalent for represented Machine-owned reset subset | Reset tests; source inspection | The Rust name is narrower and does not imply boot, PIF/BIOS execution, device reset, bus, memory map, or step readiness. |
| Step/execution relationship | `src/core/machine.hpp` / `src/core/machine_cpu.cpp` `step_cpu_instruction()` and private execution helpers | Represented Rust `Machine::step` exists separately; no generic execute API, `Cpu::step`, or full step compatibility exists | Narrow represented-category step only | Source inspection; C++ gates only | Construction ownership does not claim branch/load/store/COP0/ERET/LL/SC, bus/device routing, or full C++ step compatibility. |
| Cartridge staging / PI DMA / IPL3 SP DMEM entry | `src/core/machine.cpp`; `src/core/machine_cpu.cpp` staging and PI DMA methods | No Rust staging or DMA APIs | Not in scope | Source inspection; C++ gates only | Rust Machine does not stage cartridge bytes or select execution entry points. |
| Host/window/renderer relationship | Host files construct/use C++ Machine; SDL lives in `src/host/sdl`, not `src/core` | No Rust host/window/renderer API | Not in scope | Source inspection | Rust remains platform-free and sidecar-only. |
| `Machine::from_cartridge` name | C++ constructor syntax `Machine(Cartridge)` | Rust associated constructor `Machine::from_cartridge` | Rust-only helper, no emulator truth | Source inspection; Rust tests | This is Rust API shape for the same construction relationship, not new emulator behavior. |

## Seam 003 Audit Changes

- Added `rust/crates/fn64-core/src/machine.rs` as the minimal Rust-sidecar
  Machine construction home.
- Exported `Machine` from `rust/crates/fn64-core/src/lib.rs`.
- Rust Machine initially owned only `Cartridge` and a
  construction-observable `powered_on` flag in seam 003. Later seams added
  construction-only `Rdram` and `Cpu` ownership.
- No Rust reset, step, fetch, decode, execute, staging, bus, memory-map,
  renderer, SDL, or host behavior was added.
- No C++ source files were changed.

## CPU/RDRAM Construction Ownership Decision Table

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Machine powered_on construction flag | `src/core/machine.cpp` `Machine::Machine` calls `reset_to_non_boot_power_on_state`; `powered_on_ = true`; `powered_on()` | `rust/crates/fn64-core/src/machine.rs` `Machine::from_cartridge`, `powered_on` field, `powered_on()`; `Machine::reset` | Equivalent for construction/reset flag value | `machine_from_empty_cartridge_is_powered_on_and_owns_cartridge`; `machine_from_loaded_cartridge_preserves_cartridge_facts`; reset tests; C++ `require_non_boot_reset_power_on_state`; source inspection | Construction and reset both leave `powered_on` true for the represented non-boot power-on state. |
| C++ CPU type exists | C++ CPU state is inside `src/core/machine.hpp` `class Machine`; no standalone CPU type exists | `rust/crates/fn64-core/src/cpu.rs` `Cpu` | Rust-only structural helper, no standalone C++ type truth | Source inspection | C++ has Machine-owned CPU fields, not a separate CPU class. Rust `Cpu` groups the earned construction fields without claiming a C++ type match. |
| Rust CPU type exists | No C++ standalone CPU type; C++ CPU state is Machine-owned fields | `cpu.rs` `pub struct Cpu` | Equivalent for represented construction state; type shape differs | CPU construction tests; source inspection | The Rust type is allowed sidecar structure for construction/default-state ownership only. |
| CPU default construction state | `src/core/machine.cpp` `reset_to_non_boot_power_on_state` sets CPU PC/next PC, HI/LO, GPRs, and COP0 construction fields | `cpu.rs` `Cpu::new`; `cpu/cop0.rs` `Cop0::new`; `Machine::reset` replaces CPU with `Cpu::new()` | Equivalent for PC, next PC, HI/LO, 32 GPR values, and listed COP0 fields | `new_cpu_starts_at_cpp_non_boot_pc_pair`; `new_cpu_zeroes_integer_register_state`; `new_cpu_zeroes_cpp_cop0_construction_state`; reset tests; C++ reset source | CPU reset is only exposed through `Machine::reset`; no standalone `Cpu::reset` API was added. |
| CPU register ownership | `src/core/machine.hpp` `cpu_hi_`, `cpu_lo_`, `cpu_gprs_`; `src/core/machine_cpu.cpp` inspect/write helpers | `cpu.rs` private `hi`, `lo`, `gprs`; `cpu/scalars.rs` scalar access/staging methods; `cpu/registers.rs` `Cpu::gpr`, `Cpu::set_gpr`; `Machine::reset` | Equivalent for GPR/scalar storage, narrow mutation, and represented reset clearing | CPU construction, GPR mutation, scalar staging, SPECIAL shift/logical/HI-LO transfer/non-trapping integer/trapping integer/immediate trapping integer execution, and reset tests; source inspection | Rust adds no instruction writeback or execution behavior outside the narrow SPECIAL shift, bitwise logical, HI/LO transfer, non-trapping integer, trapping integer, and immediate trapping integer helpers. |
| CPU pc / next_pc construction/reset values | `Machine::kNonBootResetVectorPc`, `Machine::kNonBootResetVectorNextPc`; `reset_to_non_boot_power_on_state` | `cpu.rs` `NON_BOOT_RESET_VECTOR_PC`, `NON_BOOT_RESET_VECTOR_NEXT_PC`, `Cpu::new`; `cpu/scalars.rs` `Cpu::pc`, `Cpu::next_pc`; `Machine::reset` | Equivalent | `new_cpu_starts_at_cpp_non_boot_pc_pair`; reset tests; README non-boot reset vector section | Rust mirrors `0xbfc00000` and `0xbfc00004` for construction and represented Machine reset. |
| C++ RDRAM type/storage exists | `src/core/machine.hpp` private `std::array<std::uint8_t, kRdramSizeBytes> rdram_` | `rust/crates/fn64-core/src/rdram.rs` `pub struct Rdram` private `bytes: Vec<u8>` | Equivalent for owned byte storage behavior | `default_rdram_has_cpp_construction_size`; `default_rdram_storage_is_zero_filled`; source inspection | C++ has Machine-owned storage rather than a standalone RDRAM type. Rust uses a sidecar RDRAM owner to mirror construction storage only. |
| Rust RDRAM type/storage exists | C++ Machine-owned `rdram_` storage | `rdram.rs` `Rdram`; `Rdram::read_u8/u16_be/u32_be/u64_be`; `machine.rs` `Machine::write_rdram_u8/u16_be/u32_be/u64_be`; `Machine::read_direct_rdram_u8/u16_be/u32_be/u64_be`; `Machine::write_direct_rdram_u8/u16_be/u32_be/u64_be` | Equivalent for construction storage, raw byte/u16_be/u32_be/u64_be reads, Machine-level raw byte/u16_be/u32_be/u64_be writes, and direct CPU-addressed RDRAM value access | Rust RDRAM tests; raw read-width tests; raw write tests; direct value-access tests | Rust keeps pure storage inspection on `Rdram`, reservation-aware raw/direct byte/u16_be/u32_be/u64_be writes on `Machine`, and direct read values on `Machine`. Range, mapping, DMA, and CPU load/store instruction behavior remain absent. |
| RDRAM default size | `src/core/machine.hpp` `kRdramSizeBytes = 4 * 1024 * 1024`; `Machine::rdram_size_bytes()` | `rdram.rs` `RDRAM_SIZE_BYTES`; `Rdram::size_bytes` | Equivalent | `default_rdram_has_cpp_construction_size`; Machine tests | Both expose 4 MiB construction-owned RDRAM size. |
| RDRAM zero/default contents | `src/core/machine.cpp` `reset_to_non_boot_power_on_state` calls `rdram_.fill(0)`; proof checks selected words are zero | `rdram.rs` `Rdram::default` allocates zero-filled storage; `Machine::reset` replaces represented RDRAM with that default | Equivalent for construction and represented reset zero-fill | `default_rdram_storage_is_zero_filled`; reset tests; C++ `require_non_boot_reset_power_on_state` | Rust tests check constructed storage is zero and reset clears prior represented RDRAM writes. Range access and mapped/runtime memory behavior remain absent. |
| Machine ownership of CPU | `src/core/machine.hpp` Machine owns CPU fields directly | `machine.rs` private `cpu: Cpu`; `Machine::cpu` | Equivalent for construction/default-state ownership only | `machine_from_cartridge_owns_cpu_construction_state`; source inspection | C++ stores fields directly on `Machine`; Rust owns them through a sidecar `Cpu` value. No execution connection is added. |
| Machine ownership of RDRAM | `src/core/machine.hpp` private `rdram_`; `Machine::rdram_size_bytes`; private raw helpers | `machine.rs` private `rdram: Rdram`; `Machine::rdram`; `Machine::write_rdram_u8`; `Machine::write_rdram_u16_be`; `Machine::write_rdram_u32_be`; `Machine::write_rdram_u64_be`; `rdram.rs` read-width methods | Equivalent for construction ownership, size, raw byte/u16_be/u32_be/u64_be reads, reservation-aware raw byte writes, reservation-aware raw u16_be writes, reservation-aware raw u32_be writes, and reservation-aware raw u64_be writes | Machine tests; RDRAM tests; raw read-width tests; raw byte-write tests; raw u16_be write tests; raw u32_be write tests; raw u64_be write tests; source inspection | Rust Machine owns RDRAM; pure reads live on `Rdram` and reservation-aware byte/u16_be/u32_be/u64_be write seams live on `Machine`. It still has no range, mapped, DMA, or CPU-visible memory behavior. |
| Reset relationship | `src/core/machine.cpp` constructor calls `reset_to_non_boot_power_on_state` | `Machine::from_cartridge` constructs represented reset state directly; `Machine::reset` restores it later | Equivalent for represented state, different construction shape | Source inspection; reset tests | Rust still has no SP/PIF/device reset state. |
| Step/execution relationship | `src/core/machine.hpp` / `src/core/machine_cpu.cpp` `step_cpu_instruction` and private fetch/decode/identify/execute helpers | Represented Rust `Machine::step` exists separately; no generic execute API, `Cpu::step`, or full step compatibility exists | Narrow represented-category step only | Source inspection; C++ gates only | CPU/RDRAM construction ownership does not claim branch/load/store/COP0/ERET/LL/SC, bus/device routing, or full C++ step compatibility. |
| Rust `Rdram::size_bytes` | C++ has `Machine::rdram_size_bytes()` for current Machine-owned RDRAM size | `rdram.rs` `Rdram::size_bytes` | Rust-only helper, no emulator truth beyond construction-size inspection | Rust tests | The method exists only to inspect the construction-owned sidecar storage size. |

## CPU Construction/Default-State Parity Table

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ CPU type exists or does not exist | `src/core/machine.hpp` `class Machine` owns CPU fields directly; no standalone `Cpu` class exists | `rust/crates/fn64-core/src/cpu.rs` `pub struct Cpu` | Rust-only structural helper, no standalone C++ type truth | Source inspection | The type shape differs. Rust `Cpu` is the sidecar ownership home for construction CPU facts only. |
| Rust Cpu type exists | C++ Machine-owned CPU fields in `src/core/machine.hpp` | `cpu.rs` `Cpu` | Equivalent for represented construction/default-state facts only | `cargo test` CPU construction tests | Rust `Cpu` is not a CPU execution engine and is not a C++ replacement. |
| CPU ownership location in C++ | `src/core/machine.hpp` private `cpu_pc_`, `cpu_next_pc_`, `cpu_hi_`, `cpu_lo_`, `cpu_gprs_`, COP0 fields | N/A | C++ exists | Source inspection | C++ keeps CPU state directly inside `Machine`. |
| CPU ownership location in Rust | C++ Machine-owned fields | `machine.rs` private `cpu: Cpu`; `Machine::cpu`; `cpu.rs` private CPU fields | Equivalent for construction/default-state ownership only | `machine_from_cartridge_owns_cpu_construction_state` | Rust Machine owns CPU directly through a plain value. |
| Machine ownership of CPU | `src/core/machine.hpp` `Machine` private CPU fields; `src/core/machine.cpp` constructor initializes them | `machine.rs` `Machine::from_cartridge` initializes `cpu: Cpu::new()`; `Machine::reset` replaces represented CPU with `Cpu::new()` | Equivalent for construction/default-state ownership and represented reset | `machine_from_cartridge_owns_cpu_construction_state`; reset tests; C++ construction proof | No CPU/RDRAM execution connection, cartridge execution mapping, or step API is added. |
| CPU default construction | `src/core/machine.cpp` `Machine::Machine` calls `reset_to_non_boot_power_on_state()` | `cpu.rs` `Cpu::new`; `machine.rs` `Machine::from_cartridge`; `Machine::reset` | Equivalent for represented CPU construction/reset fields | CPU construction tests; reset tests; C++ `require_non_boot_reset_power_on_state` | Rust mirrors construction values and restores them through Machine-owned reset without exposing standalone `Cpu::reset`. |
| CPU register storage ownership | `src/core/machine.hpp` `cpu_hi_`, `cpu_lo_`, `std::array<CpuRegisterValue, kCpuGprCount> cpu_gprs_` | `cpu.rs` private `hi`, `lo`, `gprs: [u64; CPU_GPR_COUNT]` | Equivalent for construction storage, narrow scalar staging, and narrow GPR mutation | `new_cpu_zeroes_integer_register_state`; GPR, scalar mutation, and SPECIAL shift/logical/HI-LO transfer/non-trapping integer/trapping integer/immediate trapping integer execution tests; source inspection | Rust adds GPR access/mutation plus PC, next PC, HI, LO scalar staging, narrow SPECIAL shift/logical/non-trapping/trapping integer destination writeback, narrow immediate trapping integer destination writeback, and HI/LO transfer writeback. It still has no broad instruction writeback. |
| CPU general-register construction values | `src/core/machine.cpp` `cpu_gprs_.fill(0)` | `cpu.rs` `Cpu::new` sets `gprs: [0; CPU_GPR_COUNT]`; `cpu/registers.rs` `Cpu::gpr` | Equivalent | `new_cpu_zeroes_integer_register_state`; C++ proof checks GPRs 0, 1, 8, 31 | Rust test checks all 32 construction values are zero. |
| CPU zero-register construction value | `src/core/machine_cpu.cpp` `read_cpu_gpr_value(0)` returns 0; construction also zeroes storage; `write_cpu_gpr_value(0, value)` returns without storing | `cpu/registers.rs` `Cpu::gpr(0)` returns `Some(0)`; `Cpu::set_gpr(0, value)` returns `Ok(())` without storing | Equivalent for construction value and narrow GPR mutation | `new_cpu_zeroes_integer_register_state`; `gpr_zero_write_is_ignored_without_changing_other_state`; C++ proof checks GPR 0 | This is not instruction writeback, branch/link, or delay-slot behavior. |
| CPU pc construction value | `src/core/machine.hpp` `kNonBootResetVectorPc`; `src/core/machine.cpp` `cpu_pc_ = kNonBootResetVectorPc` | `cpu.rs` `NON_BOOT_RESET_VECTOR_PC`; `Cpu::new`; `cpu/scalars.rs` `Cpu::pc` | Equivalent | `new_cpu_starts_at_cpp_non_boot_pc_pair`; README non-boot reset vector section | Value is `0xbfc00000`. |
| CPU next_pc construction value | `src/core/machine.hpp` `kNonBootResetVectorNextPc`; `src/core/machine.cpp` `cpu_next_pc_ = kNonBootResetVectorNextPc` | `cpu.rs` `NON_BOOT_RESET_VECTOR_NEXT_PC`; `Cpu::new`; `cpu/scalars.rs` `Cpu::next_pc` | Equivalent | `new_cpu_starts_at_cpp_non_boot_pc_pair`; README non-boot reset vector section | Value is `0xbfc00004`. |
| CPU HI/LO construction values | `src/core/machine.cpp` `cpu_hi_ = 0`, `cpu_lo_ = 0` | `cpu.rs` `Cpu::new`; `cpu/scalars.rs` `Cpu::hi`, `Cpu::lo` | Equivalent | `new_cpu_zeroes_integer_register_state`; C++ proof checks HI/LO | Values are zero. |
| COP0 construction fields | `src/core/machine.cpp` zeroes `cop0_count_`, `cop0_compare_`, `cop0_status_`, `cop0_software_interrupt_pending_`, `cop0_epc_`, `cop0_bad_vaddr_`, `cop0_exception_code_`, and false flags | `cpu/cop0.rs` private `Cop0` fields and `Cpu::cop0_*` read-only accessors | Equivalent for construction values only | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Rust adds no COP0 execution, exception delivery, timer behavior, or register-write behavior. |
| CPU/RDRAM reservation construction/staging/invalidation/reset field | `src/core/machine.hpp` `CpuRdramReservation cpu_rdram_reservation_`; `src/core/machine.cpp` `clear_cpu_rdram_reservation()`, `set_cpu_rdram_reservation()`, and `invalidate_cpu_rdram_reservation_for_write()` | `machine.rs` private `cpu_rdram_reservation`; `machine/rdram_reservation.rs` `CpuRdramReservation::new`, `stage`, `invalidate_for_rdram_write`; `Machine::reset` | Equivalent behavior, different ownership shape for the private helper subset and represented reset clear | Reservation construction, staging, invalidation, raw write, and reset tests; source inspection | Rust owns construction/default state, private staging/setup state, private invalidation state, and Machine reset clearing. LL/SC, DMA, and range writes remain absent. |
| CPU staging helpers | `src/core/machine.hpp` `stage_cpu_pc`, `stage_cpu_next_pc`, `stage_cpu_hi`, `stage_cpu_lo`, `stage_cpu_gpr` | `cpu/scalars.rs` `Cpu::stage_pc`, `Cpu::stage_next_pc`, `Cpu::stage_hi`, `Cpu::stage_lo`; `cpu/registers.rs` `Cpu::set_gpr`; `Machine::stage_cpu_pc`; `Machine::reset` restores represented reset values | Equivalent state semantics for the represented subset | Source inspection; GPR/scalar/reset tests; Rust step probe | Rust keeps CPU scalar staging semantics and exposes one narrow Machine forwarding operation for deterministic inspection: `Machine::stage_cpu_pc` sets `pc` and the sequential wrapping `next_pc`. No mutable CPU/COP0 access, generic state injection, or execution mutation API is added. |
| Whether construction calls reset | `src/core/machine.cpp` `Machine::Machine` calls `reset_to_non_boot_power_on_state()` | `Machine::from_cartridge` constructs `Cpu::new`, `Rdram::default`, `CpuRdramReservation::new`, and `powered_on: true` directly; `Machine::reset` restores that represented state later | Equivalent represented state, different construction shape | Source inspection; C++ construction proof; reset tests | Rust does not call `Machine::reset` during construction and does not claim SP/PIF/device reset parity. |
| Reset relationship | `src/core/machine.hpp` / `.cpp` public `reset_to_non_boot_power_on_state()` | `machine.rs` `Machine::reset` | Equivalent for represented Machine-owned reset state | Source inspection; reset tests | Rust reset covers CPU scalar/GPR/COP0, RDRAM, reservation, and powered_on state while preserving Cartridge; C++ SP/PIF/device reset remains absent. |
| Step/execution relationship | `src/core/machine.hpp` `step_cpu_instruction`; `src/core/machine_cpu.cpp` fetch/decode/identify/execute helpers | Represented Rust `Machine::step` exists separately; no generic execute API, `Cpu::step`, or full step compatibility exists | Narrow represented-category step only | Source inspection; C++ gates only | CPU construction/default-state ownership does not claim branch/load/store/COP0/ERET/LL/SC, bus/device routing, or full C++ step compatibility. |
| Rust `Cpu::gpr` out-of-range behavior | C++ `inspect_cpu_gpr` throws via `fail_cpu_gpr_index` on invalid index | `cpu/registers.rs` `Cpu::gpr` returns `Option<u64>` | Rust-only helper, no emulator truth | Source inspection | This keeps Rust inspection explicit without adding emulator behavior. |

## CPU Construction Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ standalone CPU type absence | `src/core/machine.hpp` has no standalone `Cpu`; CPU state lives in `class Machine` | `rust/crates/fn64-core/src/cpu.rs` `pub struct Cpu` | Equivalent construction state, different ownership shape | Source inspection | Rust `Cpu` is a sidecar ownership refinement, not C++ type-layout parity. |
| C++ Machine-owned CPU fields | `src/core/machine.hpp` `cpu_pc_`, `cpu_next_pc_`, `cpu_hi_`, `cpu_lo_`, `cpu_gprs_`, COP0 fields | `cpu.rs` private `pc`, `next_pc`, `hi`, `lo`, `gprs`, and private `cop0: Cop0` from `cpu/cop0.rs` | Equivalent construction state, different ownership shape | CPU construction tests; source inspection | C++ stores CPU state directly in `Machine`; Rust groups construction CPU state in `Cpu`. |
| Rust Cpu type as sidecar ownership refinement | C++ Machine fields listed above | `cpu.rs` `Cpu`; `rust/crates/fn64-core/src/lib.rs` CPU exports | Equivalent construction state, different ownership shape | `cargo test`; source inspection | Valid only because the ledger names C++ `Machine` fields as product truth. |
| Rust Machine owning Cpu | `src/core/machine.hpp` `Machine` private CPU fields; `src/core/machine.cpp` constructor initializes them | `machine.rs` private `cpu: Cpu`; `Machine::from_cartridge`; `Machine::cpu` | Equivalent construction state, different ownership shape | `machine_from_cartridge_owns_cpu_construction_state` | Rust Machine owns CPU construction state directly through a plain value. |
| CPU default construction/reset | `src/core/machine.cpp` `Machine::Machine` calls `reset_to_non_boot_power_on_state()`; CPU assignments in that function | `cpu.rs` `Cpu::new`; `machine.rs` `Machine::from_cartridge`; `Machine::reset` | Equivalent for represented CPU state | CPU construction tests; reset tests; C++ `require_non_boot_reset_power_on_state` | Rust mirrors construction results directly and restores them through Machine-owned reset. |
| PC construction value | `src/core/machine.hpp` `kNonBootResetVectorPc`; `src/core/machine.cpp` `cpu_pc_ = kNonBootResetVectorPc` | `cpu.rs` `NON_BOOT_RESET_VECTOR_PC`, `Cpu::new`; `cpu/scalars.rs` `Cpu::pc` | Equivalent | `new_cpu_starts_at_cpp_non_boot_pc_pair`; README non-boot reset vector section | Value is `0xbfc00000`. |
| next PC construction value | `src/core/machine.hpp` `kNonBootResetVectorNextPc`; `src/core/machine.cpp` `cpu_next_pc_ = kNonBootResetVectorNextPc` | `cpu.rs` `NON_BOOT_RESET_VECTOR_NEXT_PC`, `Cpu::new`; `cpu/scalars.rs` `Cpu::next_pc` | Equivalent | `new_cpu_starts_at_cpp_non_boot_pc_pair`; README non-boot reset vector section | Value is `0xbfc00004`. |
| HI construction value | `src/core/machine.cpp` `cpu_hi_ = 0`; `src/core/machine_cpu.cpp` `inspect_cpu_hi` | `cpu.rs` `hi`, `Cpu::new`; `cpu/scalars.rs` `Cpu::hi` | Equivalent | `new_cpu_zeroes_integer_register_state`; C++ construction proof | Value is zero. |
| LO construction value | `src/core/machine.cpp` `cpu_lo_ = 0`; `src/core/machine_cpu.cpp` `inspect_cpu_lo` | `cpu.rs` `lo`, `Cpu::new`; `cpu/scalars.rs` `Cpu::lo` | Equivalent | `new_cpu_zeroes_integer_register_state`; C++ construction proof | Value is zero. |
| GPR storage ownership | `src/core/machine.hpp` `std::array<CpuRegisterValue, kCpuGprCount> cpu_gprs_` | `cpu.rs` `gprs: [u64; CPU_GPR_COUNT]` | Equivalent construction state, different ownership shape | `new_cpu_zeroes_integer_register_state`; source inspection | Seam 008 adds only narrow GPR access/mutation for this storage. |
| GPR count | `src/core/machine.hpp` `kCpuGprCount = 32` | `cpu.rs` `CPU_GPR_COUNT = 32` | Equivalent | `new_cpu_exposes_cpp_gpr_count_boundary`; `gpr_write_invalid_index_is_explicit_rust_api_safety` | Rust exposes the boundary with explicit Rust API safety for invalid indices. |
| GPR construction values | `src/core/machine.cpp` `cpu_gprs_.fill(0)` | `cpu.rs` `Cpu::new` initializes `gprs` with zeros | Equivalent | `new_cpu_zeroes_integer_register_state`; C++ proof checks GPRs 0, 1, 8, 31 | Rust test checks all 32 GPR construction values. |
| zero-register construction value | `src/core/machine_cpu.cpp` `read_cpu_gpr_value(0)` returns zero; construction storage is zeroed | `cpu/registers.rs` `Cpu::gpr(0)` returns `Some(0)` after construction | Equivalent | `new_cpu_zeroes_integer_register_state`; `gpr_zero_write_is_ignored_without_changing_other_state`; C++ construction proof | Seam 008 mirrors the explicit C++ zero-register write-ignore rule for narrow GPR mutation. |
| COP0 construction fields | `src/core/machine.cpp` zeroes `cop0_count_`, `cop0_compare_`, `cop0_status_`, `cop0_software_interrupt_pending_`, `cop0_epc_`, `cop0_bad_vaddr_`, `cop0_exception_code_`; false flags for timer pending and branch delay | `cpu/cop0.rs` private `Cop0` fields plus construction inspectors on `Cpu` | Equivalent | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Rust adds no COP0 register writes, interrupt delivery, ERET, or Count step wiring; seam 062 owns only the narrow Count helper. |
| powered_on relationship to CPU construction | `src/core/machine.cpp` constructor path sets `powered_on_ = true` before CPU fields | `machine.rs` `powered_on: true` and `cpu: Cpu::new()` in same constructor | Equivalent for construction observation only | Machine construction tests; C++ `require_non_boot_reset_power_on_state` | `powered_on` is Machine construction state, not CPU execution state. |
| CPU/RDRAM reservation construction/staging/invalidation/reset state | `src/core/machine.hpp` `CpuRdramReservation cpu_rdram_reservation_`; `src/core/machine.cpp` `clear_cpu_rdram_reservation()` during construction/reset, `set_cpu_rdram_reservation()`, and `invalidate_cpu_rdram_reservation_for_write()` | `machine/rdram_reservation.rs` `CpuRdramReservation::new`, `stage`, `invalidate_for_rdram_write`; `machine.rs` private `cpu_rdram_reservation`; `Machine::reset` | Equivalent behavior, different ownership shape for the private helper subset and represented reset clear | Reservation construction, staging, invalidation, raw byte write, raw u16_be write, raw u32_be write, raw u64_be write, and reset tests; source inspection | Reservation construction, private staging/setup, private invalidation, and Machine reset clearing are earned. Raw byte writes invalidate with width `1`; raw u16_be writes invalidate with width `2`; raw u32_be writes invalidate with width `4`; raw u64_be writes invalidate with width `8`. LL/SC, load/store, DMA write behavior, range writes, and execution remain unearned. |
| Whether construction calls reset | `src/core/machine.cpp` `Machine::Machine` calls `reset_to_non_boot_power_on_state()` | `Machine::from_cartridge` initializes represented reset fields directly; `Machine::reset` restores them later | Equivalent represented state, different construction shape | Source inspection; reset tests | Rust construction does not call the reset method and does not claim unrepresented SP/PIF/device reset state. |
| Reset relationship | `src/core/machine.hpp` / `.cpp` public `reset_to_non_boot_power_on_state()` | `machine.rs` `Machine::reset` | Equivalent for represented Machine-owned reset state | Source inspection; C++ gates; reset tests | Constructor-reset linkage is mirrored as state equivalence, but full C++ reset parity is blocked by absent SP/PIF/device owners. |
| Step/execution relationship | `src/core/machine.hpp` `step_cpu_instruction`; `src/core/machine_cpu.cpp` private fetch/decode/identify/execute helpers | Represented Rust `Machine::step` exists separately; no generic execute API, `Cpu::step`, or full step compatibility exists | Narrow represented-category step only | Source inspection; C++ gates only | CPU construction parity does not claim branch/load/store/COP0/ERET/LL/SC, bus/device routing, or full C++ step compatibility. |
| Rust `Cpu::gpr` boundary return | C++ `inspect_cpu_gpr` throws for invalid index via `fail_cpu_gpr_index` | `cpu/registers.rs` `Cpu::gpr` returns `Option<u64>` | Rust-only helper, no emulator truth | `new_cpu_exposes_cpp_gpr_count_boundary`; source inspection | This is explicit Rust inspection shape, not emulator behavior. |

## Reset-State Ownership Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ reset operation exists | `src/core/machine.hpp` `Machine::reset_to_non_boot_power_on_state`; `src/core/machine.cpp` implementation | `machine.rs` `Machine::reset` | Equivalent for represented Machine-owned state | Source inspection; reset tests | C++ reset is public and construction calls it. Rust exposes the smaller product name `reset`. |
| Rust reset operation scope | C++ public `Machine::reset_to_non_boot_power_on_state()` clears represented and unrepresented owners | `Machine::reset` resets CPU, RDRAM, SP DMEM, reservation, and powered-on state while preserving Cartridge | Equivalent for represented state; C++ exists beyond Rust scope | Reset tests | Rust does not add SP IMEM/PIF/device state just to claim full C++ reset parity. |
| Whether construction calls reset | `src/core/machine.cpp` `Machine::Machine(Cartridge)` calls `reset_to_non_boot_power_on_state()` | `Machine::from_cartridge` directly constructs `Cpu::new`, `Rdram::default`, `CpuRdramReservation::new`, and `powered_on: true` | Equivalent result for represented state, different construction shape | Source inspection; Machine construction tests; reset tests | Rust construction does not call `Machine::reset`, but both produce the same represented state. |
| Reset callers in current no-window gates | `src/proof/bootstrap_data.cpp` `run_machine_construction_isolation_demo`, `run_non_boot_reset_vector_step_demo`, `require_cartridge_ipl3_staging_reset_remains_non_boot`; `src/host/cli/step_probe_main.cpp` staged IPL3 probe | `fn64-inspection` `run_machine_probe` remains construction/reset-only; the separate `fn64_step_probe` owns represented-step inspection | Equivalent for construction/reset inspection subset; represented step has a separate narrow probe | C++ frozen gates; `cargo test`; both Rust probe commands | The machine probe makes no step claim. The Rust step probe covers only the eight represented categories and does not replace broader C++ load/store, cartridge staging, SP-DMEM IPL3, COP0, branch, or boot-adjacent scenarios. |
| PC reset value | `src/core/machine.hpp` `kNonBootResetVectorPc`; `src/core/machine.cpp` `cpu_pc_ = kNonBootResetVectorPc` | `cpu.rs` `NON_BOOT_RESET_VECTOR_PC`; `Cpu::new`; `Machine::reset` | Equivalent | `machine_reset_restores_represented_non_boot_power_on_state`; CPU construction tests | Value is `0xbfc00000`. |
| next PC reset value | `src/core/machine.hpp` `kNonBootResetVectorNextPc`; `src/core/machine.cpp` `cpu_next_pc_ = kNonBootResetVectorNextPc` | `cpu.rs` `NON_BOOT_RESET_VECTOR_NEXT_PC`; `Cpu::new`; `Machine::reset` | Equivalent | Reset tests; CPU construction tests | Value is `0xbfc00004`. |
| HI/LO reset behavior | `src/core/machine.cpp` `cpu_hi_ = 0`, `cpu_lo_ = 0` | `Cpu::new`; `Machine::reset` | Equivalent | Reset tests | `Machine::reset` clears staged HI/LO by replacing the represented CPU state. |
| GPR reset behavior | `src/core/machine.cpp` `cpu_gprs_.fill(0)`; `src/core/machine_cpu.cpp` zero-register read/write policy | `Cpu::new`; `Cpu::gpr`; `Machine::reset` | Equivalent | Reset tests; GPR tests | All GPRs read zero after reset, including register zero. |
| COP0 reset behavior | `src/core/machine.cpp` zeroes Count, Compare, Status, software pending, EPC, BadVAddr, exception code; clears timer pending and branch-delay flags | `Cop0::new`; `Cpu::new`; `Machine::reset` | Equivalent for represented COP0 subset | Reset tests; COP0 construction tests | Reset clears prior narrow address-error entry state. No generic exception, ERET, interrupt, or Count step wiring is added. |
| powered_on reset behavior | `src/core/machine.cpp` `powered_on_ = true`; `Machine::powered_on()` | `Machine::reset` sets `powered_on = true`; `Machine::powered_on` | Equivalent | Reset tests | There is still no power-off transition. |
| cartridge ownership/preservation across reset | `src/core/machine.hpp` `Cartridge cartridge_`; `src/core/machine.cpp` reset function does not assign `cartridge_` | `Machine::reset` leaves `cartridge` unchanged | Equivalent | `machine_reset_restores_represented_non_boot_power_on_state` | Loaded cartridge bytes and metadata survive reset. |
| RDRAM reset behavior | `src/core/machine.cpp` `rdram_.fill(0)` | `Machine::reset` replaces RDRAM with `Rdram::default` zero-filled storage | Equivalent byte behavior | Reset tests; RDRAM tests | Prior raw/direct writes are cleared and size remains 4 MiB. |
| SP memory reset behavior | `src/core/machine.cpp` `sp_dmem_.fill(0)`, `sp_imem_.fill(0)` | `Machine::reset` resets represented `SpDmem`; no Rust SP IMEM state | Equivalent for SP DMEM; C++ exists beyond Rust scope | SP DMEM reset tests; source inspection | SP DMEM reset is earned. SP IMEM remains intentionally absent. |
| PIF RAM reset behavior | `src/core/machine.cpp` `pif_ram_.fill(0)` | No Rust PIF RAM state | C++ exists, Rust intentionally absent | Source inspection; C++ gates only | PIF RAM/ROM behavior is not earned in Rust. |
| Local device shadow reset behavior | `src/core/machine.cpp` clears SP, MI, PI, AI, and SI local register/status shadows | No Rust SP register/status/control, MI, PI, AI, or SI state | C++ exists, Rust intentionally absent | Source inspection; C++ gates only | Device shadows are unearned Rust state and are tied to MMIO/DMA behavior. |
| CPU/RDRAM reservation reset behavior | `src/core/machine.cpp` calls `clear_cpu_rdram_reservation()`; `src/core/machine.cpp` reservation helpers | `Machine::reset` assigns `CpuRdramReservation::new()` | Equivalent | Reset tests; reservation tests | Reset clears valid staged reservation state to `{ valid: false, offset: 0, width: 0 }`. |
| Boot/staging relationship | `src/core/machine.hpp` comments; `src/proof/bootstrap_data.cpp` `require_cartridge_ipl3_staging_reset_remains_non_boot`; `src/host/cli/step_probe_main.cpp` reset after staged IPL3 candidate | No Rust boot or cartridge/SP staging API; `Machine::reset` only restores represented local non-boot state | Not in scope beyond represented reset | `fn64_selftest`; `fn64_step_probe`; reset tests | Rust reset does not stage bytes, execute PIF/CIC/IPL3, or select an execution handoff. |
| PI DMA relationship | `src/core/machine.cpp` reset clears PI local shadows; PI DMA behavior lives in C++ Machine data path | No Rust PI/DMA state | C++ exists, Rust intentionally absent | Source inspection; C++ gates only | Reset's PI shadow clearing is not enough to earn Rust PI or DMA behavior. |
| fetch/decode/identify/execute/step relationship | `src/core/machine_cpu.cpp` `step_cpu_instruction`; reset-vector fetch enters unavailable PIF ROM exception path in C++ proofs | Represented `Machine::step` exists separately; no generic execute/full step API exists, and reset-vector PIF behavior remains absent | Not in reset scope | `run_non_boot_reset_vector_step_demo`; `fn64_step_probe` | Reset sets PC/next PC only; reset-vector PIF bytes and full reset-fetch behavior remain absent. |
| host/runtime/renderer relationship | `src/host/cli/inspect_main.cpp` reports observation-only reset model; `src/host/sdl/app.cpp` reports reset model but SDL runtime is not part of core reset seam | No Rust host/runtime/renderer API | Not in scope | Source inspection | Rust remains platform-free and sidecar-only. |
| Rust-only reset error type | N/A | No reset error type | Rust-only API safety, no emulator truth | Source inspection | Reset cannot fail for the represented source-clear state, so no lifecycle/error framework was added. |

### Seam 042 Audit Changes

- Added `Machine::reset` as the named Machine-owned reset transition for the
  represented non-boot power-on state.
- Reset replaces represented CPU state with `Cpu::new`, replaces RDRAM with
  zero-filled `Rdram::default`, clears `CpuRdramReservation`, sets
  `powered_on = true`, and preserves the owned Cartridge.
- Added reset tests for CPU scalar state, all GPRs, zero register, represented
  COP0 fields after prior address-error entry, RDRAM clearing after writes,
  reservation clearing, cartridge preservation, powered-on behavior, and
  repeatability.
- Added no standalone `Cpu::reset`, reset error type, boot/PIF/BIOS execution,
  step/fetch/decode/execute behavior, memory map, bus, device/MMIO reset, DMA,
  LL/SC instruction behavior, host shell, SDL, renderer, or C++ integration.
- No C++ source files were changed.

## No-Window Machine Probe Readiness Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Historical C++ self-test no-window runner | `CMakeLists.txt` `fn64_selftest`; `src/proof/selftest_main.cpp` `main`; `src/proof/bootstrap.cpp` `run_bootstrap_demos` | `fn64-inspection` does not replace the retired full self-test | Retired history beyond Rust scope | Historical command/source inspection only | The deleted C++ self-test ran broader instruction/execution proofs. Those proofs were not migrated. |
| C++ construction/reset proof subset | `src/proof/bootstrap_data.cpp` `run_machine_construction_isolation_demo`; `require_non_boot_reset_power_on_state` | `rust/crates/fn64-inspection/src/lib.rs` `run_machine_probe`; `assert_represented_power_on_state`; `assert_probe_cartridge` | Equivalent for represented construction/reset facts | `cargo test`; Rust no-window probe command | Rust observes powered-on state, PC/next PC reset constants, HI/LO zero, representative GPR zero state, represented COP0 zero/false state, RDRAM size and zero-fill, and Cartridge preservation. |
| Historical C++ step probe role | `CMakeLists.txt` `fn64_step_probe`; `src/host/cli/step_probe_main.cpp` | Separate Rust `fn64_machine_probe` and `fn64_step_probe` binaries own current construction/reset and represented-step inspection | Narrower represented subset only | Historical source anchors plus current Rust probes | The broader C++ check was retired and is not runnable in the current tree. Its unported cases are intentionally absent. |
| Historical C++ inspect CLI role | `CMakeLists.txt` `fn64_inspect`; `src/host/cli/inspect_main.cpp` | No Rust ROM-path inspect CLI | Intentionally absent | Historical source anchor only | Rust probes use synthetic in-memory bytes and do not add path policy or a broad host shell. |
| Rust probe crate ownership | N/A | `rust/crates/fn64-inspection/Cargo.toml`; `src/lib.rs`; `src/bin/fn64_machine_probe.rs` | Rust-only repo hygiene, no emulator truth | `cargo test`; probe command | Process exit status, deterministic stdout/stderr, and probe sequencing are kept outside `fn64-core`. |
| Core truth ownership | `src/core/machine.*`, `src/core/cartridge.*`, `src/core/machine_cpu.cpp` | `fn64-core` `Machine`, `Cpu`, `Rdram`, `Cartridge`, `Machine::reset` | Equivalent for already-sealed represented facts | Rust tests; source inspection | `fn64-inspection` depends on `fn64-core` and does not move Machine truth into CLI/process code. |
| Probe construction input | C++ proof/step probe use synthetic in-memory cartridge construction for some no-window scenarios | `make_synthetic_probe_cartridge_bytes` plus `load_cartridge` | Equivalent for synthetic no-window construction | Probe tests; source inspection | No commercial ROM, BIOS/PIF blob, path input, or cartridge execution mapping is introduced. |
| Construction facts inspected | C++ construction/reset proof checks powered-on reset state, CPU reset fields, RDRAM zero bytes, and Cartridge isolation | `run_machine_probe` construction assertions | Equivalent for represented facts | Probe tests; probe command | Rust checks sealed construction facts only; it does not infer unrepresented SP/PIF/device state. |
| Reset after represented mutation | C++ reset proof and step probe reset after staged state | `dirty_represented_machine_state`; `Machine::reset`; reset assertions | Equivalent for represented reset facts | `probe_observes_reset_after_prior_rdram_and_exception_mutation`; probe command | Rust dirties only already-sealed RDRAM/direct-RDRAM/address-error-entry state, then verifies reset restores represented facts and preserves Cartridge. |
| RDRAM reset observation | `Machine::reset_to_non_boot_power_on_state` fills RDRAM with zero; C++ proof reads selected words | `run_machine_probe` reads first/last byte and a u64 after reset | Equivalent for represented RDRAM zero-fill | Probe tests; reset tests | No range API, DMA, memory map, or device reset is added. |
| Cartridge preservation observation | C++ reset leaves `cartridge_` unchanged | `assert_probe_cartridge` before and after `Machine::reset` | Equivalent | Probe tests; reset tests | Synthetic cartridge bytes and metadata are preserved across reset. |
| Deterministic probe output | C++ no-window tools print stable text for their own roles | `MACHINE_PROBE_OUTPUT`; `fn64_machine_probe` | Equivalent for Rust probe role | `probe_output_is_stable_plain_no_window_text`; probe command | Output is plain text: `fn64 machine probe`, `construct: ok`, `reset: ok`, `no-window: ok`, `result: ok`. |
| Probe success/failure process behavior | C++ CLI tools return zero on success and nonzero on thrown failures/usage errors | `fn64_machine_probe` exits `0` on success, `1` on probe failure, `2` on usage error | Equivalent process convention for Rust probe | Probe command; source inspection | This is process plumbing, not machine-core truth. |
| SDL/window/runtime behavior | The retired C++ SDL host was separate from no-window gates | No Rust SDL/window dependency or runtime | Intentionally absent | Cargo manifests; probe command | The Rust probe requires no SDL and opens no window. |
| CPU step/fetch/decode/identify/execute | C++ self-test and step probe call `step_cpu_instruction` extensively | Represented Rust `Machine::step` exists in `fn64-core`; the separate Rust step probe calls it for eight represented categories, while `Cpu::step`, a generic execute API, and full C++ probe replacement remain absent | Narrow represented-category step only | Rust step probe; source inspection; tests | `fn64_machine_probe` remains construction/reset-only; `fn64_step_probe` does not replace broader C++ step/execution responsibilities. |
| Instruction/load/store/sign-extension/GPR writeback behavior | C++ proof and step paths exercise broad instruction semantics | The Rust step probe covers only sealed represented writeback/no-effect/stopped/unsupported/fetch-fault categories; no load/store execution is probed | Narrow represented subset only | Rust tests; source inspection | Probe policy does not add instruction behavior or expand the represented Machine surface. |
| Memory map/bus/device/DMA behavior | C++ Machine owns broader data/device paths used by step proofs | No Rust probe behavior | Not in scope | Rust tests; source inspection | The probe does not create target dispatch, devices, MMIO, DMA, or a bus. |
| Broad host shell/path policy | The retired C++ `fn64_inspect` accepted a ROM path; the retired SDL host owned runtime concerns | No Rust path-based host shell | Intentionally absent | Historical source anchors; current probe command | The Rust binary is a no-window proof artifact, not a general host framework. |
| Recommended next seam | Remaining C++ no-window responsibilities require CPU step/execution ownership | `rust_parallel_core_seam_044_cpu_step_readiness_audit` | Ready for next seam | This ledger | The Rust no-window probe now covers construction/reset; seam 044 maps the remaining step/execution role still covered only by C++ gates. |

### Seam 043 Audit Changes

- Added the `fn64-inspection` workspace crate outside `fn64-core`.
- Added `fn64_machine_probe`, a deterministic no-window Rust probe binary.
- The probe constructs a Machine from synthetic in-memory cartridge bytes,
  inspects sealed construction facts, dirties only already-sealed represented
  state, calls `Machine::reset`, inspects sealed reset facts, prints stable
  plain text, and exits zero on success.
- Kept CLI/process/output/exit-status ownership out of `fn64-core`; Machine,
  CPU, RDRAM, Cartridge, and reset truth remain in `fn64-core`.
- Added no CPU step, fetch, decode, execute, instruction behavior, load/store
  instruction semantics, GPR writeback, sign/zero extension, memory map, bus,
  device/MMIO, DMA, LL/SC instruction behavior, SDL/window runtime, broad host
  shell, ROM path policy, C++ integration, or C++ source changes.

## CPU Step Readiness Audit

| Concept/path | C++ owner file/function | Rust owner/status | Readiness | Blockers/notes |
| --- | --- | --- | --- | --- |
| Public step entry | `src/core/machine.hpp` `Machine::step_cpu_instruction`; `src/core/machine_cpu.cpp` implementation | Rust `Machine::step` exists only for represented categories; no `Cpu::step`, generic execute, or placeholder full-step API exists | Narrow represented-category composition earned | C++ step remains broader. Rust step composes sealed producer/application seams for represented categories and rejects or excludes unrepresented categories. |
| Step result shape | `Machine::CpuInstructionStepResult` values `kStepped`, `kStopped`, `kUnsupported`, `kInterrupted`, `kException` | `MachineRepresentedStepOutcome` and `MachineRepresentedStepError` cover only represented categories | Narrow Rust shape earned; generic all-future result absent | Rust does not include placeholders for interrupts, branch/load/store/COP0/ERET/LL/SC, devices, or full C++ probe compatibility. |
| Fetch source | `fetch_cpu_instruction_word()` reads from current `cpu_pc()` | Rust `Machine::fetch_current_cpu_instruction_word` reads represented `Cpu::pc()` and delegates to explicit-address fetch | Equivalent wrapper; step fault handling absent | Fetch is not data load. Step fault handling remains a future seam. |
| Direct RDRAM instruction fetch | `fetch_cpu_instruction_word()` translates direct CPU aliases and calls `read_rdram_u32_be` | `Machine::fetch_direct_rdram_cpu_instruction_word`; composed by `Machine::fetch_cpu_instruction_word_at` and `Machine::fetch_current_cpu_instruction_word` | Earned for direct RDRAM subset, explicit-address composition, and current-PC wrapper | Direct fetch tests; explicit-address fetch tests; current-PC fetch tests | Direct RDRAM instruction fetch is sealed. PIF fetch bytes, exception conversion, and step rollback remain absent. |
| SP DMEM instruction fetch | `fetch_cpu_instruction_word()` can fetch from local SP DMEM direct alias when the target is SP DMEM | `Machine::fetch_sp_dmem_cpu_instruction_word` | Earned for explicit SP DMEM offset fetch | C++ no-window proofs exercise SP DMEM candidate execution; Rust tests cover read-only SP DMEM word formation. |
| Unavailable PIF reset fetch | `fetch_cpu_instruction_word()` treats the non-boot reset-vector physical target as unavailable PIF ROM and step converts it to local AdEL when allowed | No Rust PIF ROM/reset fetch behavior | Blocked by instruction-fetch exception seam | Rust reset sets PC/next PC only. It does not fetch from the reset vector or enter fetch-time exceptions through step. |
| Instruction word decode | `decode_cpu_instruction_word(raw)` extracts opcode, rs, rt, rd, sa, funct, immediate_u16, immediate_i16, and jump_target | `cpu/instruction.rs` `CpuInstructionWord`, `CpuInstructionFields`, `decode_cpu_instruction_word` | Earned for raw unsigned field subset | Decode tests; source inspection | Rust mirrors raw field extraction and intentionally leaves C++ `immediate_i16` signed interpretation for a future immediate-semantics seam. |
| Instruction identity classification | `identify_cpu_instruction(decoded)` maps opcode/funct/rt/rs/raw ERET to `CpuInstructionIdentity` | `cpu/instruction.rs` `CpuInstructionIdentity`; `identify_cpu_instruction` | Earned for pure identity classification | Identity tests; source inspection | Rust mirrors the full source-clear identity family and unknown boundaries without fetch, execute, step, operand interpretation, or mutation. |
| Execute dispatch | `execute_cpu_instruction(identity, decoded)` mutates CPU, RDRAM, COP0, reservation, SP/MMIO/device shadows, and control-flow state | Rust has no generic execute dispatch; represented CPU-local helper selection/invocation exists only below `Machine::step` composition | Generic execute blocked | Execution owns unrepresented load/store, branch, COP0, LL/SC, trap, multiply/divide, and device paths. |
| Ordinary PC cadence | `step_cpu_instruction()` commits `pc = old next_pc` and advances Count after successful normal execution | Represented success/stopped/no-effect paths commit through sealed applicators | Narrow represented categories earned | Branch-likely annul, interrupts, ERET, and unrepresented branch/load/store/COP0 categories remain absent. |
| Branch/link/delay-slot cadence | `step_cpu_instruction()` comments and execute path handle delay slots, branch-likely not-taken annul, link writeback, and control-transfer target faults | No Rust branch/link/delay-slot behavior | Blocked | These are instruction-execution semantics, not scalar storage facts. |
| Unsupported rollback | `step_cpu_instruction()` restores `cpu_pc_` and `cpu_next_pc_` for unsupported identities and execution-time failures | Represented unsupported identities restore captured `pc`/`next_pc` through the classified action applicator | Narrow represented subset earned | Context-coupled unsupported and broader execution-time failures remain absent. |
| Local exception conversion | `step_cpu_instruction()` converts earned fetch/data/control-transfer faults into local COP0 exception entry when local guards allow | Narrow Rust data address-error entry exists only below instruction step | Blocked by step exception context | Rust has no fetch/control-transfer exception path and no generic step-owned exception conversion. |
| Interrupt pre-fetch entry | `try_enter_local_interrupt()` may return `kInterrupted` before fetch | No Rust interrupt behavior | Blocked by interrupt/COP0 seam | Interrupt entry is explicitly outside current Rust scope. |
| ERET path | `step_cpu_instruction()` handles `kCop0Eret` before normal speculative PC movement | No Rust ERET instruction behavior | Blocked by COP0 instruction seam | Current Rust COP0 mutation is limited to reset and narrow data address-error entry. |
| COP0 Count cadence | `step_cpu_instruction()` advances Count after committed instructions, not after local exception entry | Represented committed `Machine::step` paths call the sealed Count helper through applicators | Narrow represented categories earned | Interrupt delivery and Compare writes remain absent. |
| CPU data load/store instruction path | C++ execute cases call CPU memory helpers and perform sign/zero extension, GPR writeback, stores, and reservation effects | Rust direct RDRAM CPU-data value access exists below instruction semantics | Blocked by load/store instruction seam | Direct RDRAM CPU-data access is not load/store instruction behavior. |
| LL/SC instruction path | C++ execute cases own LL/SC reservation setup/match/writeback behavior | Rust has private reservation construction/staging/invalidation only | Blocked by LL/SC seam | Raw/direct writes invalidate reservations, but LL/SC instruction semantics remain absent. |
| Device/MMIO/DMA paths | C++ data target dispatch can touch SP memory, SP/MMIO, MI, AI, PI, SI, and DMA-related local state | No Rust devices, MMIO, DMA, bus, or memory map | Blocked by device target seams | Step cannot be honest while execute may route to unrepresented targets. |
| Historical `fn64_selftest` role | `src/proof/selftest_main.cpp` ran bootstrap arithmetic/data/trap/control demos through `step_cpu_instruction` | Rust tests cover only sealed Rust seams | `INTENTIONALLY_ABSENT_AFTER_CPP_RETIREMENT` | Historical source anchor only. |
| Historical `fn64_step_probe` RDRAM scenario | `src/host/cli/step_probe_main.cpp` staged LUI/ORI/SW/LW/ORI/SB/LBU/BREAK words | Rust `fn64_step_probe` covers only represented categories | `INTENTIONALLY_ABSENT_AFTER_CPP_RETIREMENT` | Rust does not implement or claim the retired SW/LW/SB/LBU proof behavior. |
| Historical cartridge-staging scenario | `stage_cartridge_bytes_to_rdram`, staged PC, ORI, BREAK | No Rust cartridge-to-RDRAM execution staging | `INTENTIONALLY_ABSENT_AFTER_CPP_RETIREMENT` | Current synthetic cartridge bytes are construction/reset proof only. |
| Historical staged IPL3 scenario | cartridge-to-SP staging, SP DMEM execution, MFC0 Count reads, reset-fetch exception | Only narrow Rust SP DMEM storage/fetch and Count state exist | `INTENTIONALLY_ABSENT_AFTER_CPP_RETIREMENT` | No staging, COP0 instruction, reset-vector, or boot claim exists. |
| Current Rust no-window proof scope | Historical construction/reset and represented-step comparison anchors | `fn64-inspection` machine and step probes | Current Rust facts only | Both Rust probes are deterministic/no-window; retired broader behavior is not implied. |
| Placeholder/fake step API | N/A | Intentionally absent | Documentation only | A fake step API would overclaim execution readiness. Seam 044 adds documentation only. |

### Step Candidate Seam Map

| Candidate seam | Source clarity | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| CPU instruction word decode representation | Source-clear: `decode_cpu_instruction_word(raw)` is pure field extraction | Implemented in seam 045 | Low | Complete |
| CPU instruction identity classification | Source-clear pure table in `identify_cpu_instruction` | Implemented in seam 046 | Medium | Complete |
| Instruction fetch representation | Coupled to direct RDRAM fetch, SP DMEM fetch, unavailable PIF reset fetch, and fetch-time AdEL | Direct RDRAM fetch, SP DMEM fetch, target classification, explicit-address fetch, and current-PC wrapper are sealed; PIF bytes/fetch exception ownership are absent | Medium-high | Partially complete; step-owned full fetch blocked |
| Step result/error type only | Source-visible result enum exists | Real step consumer absent | Medium | Not recommended; would be placeholder API |
| Minimal PC/next PC step cadence | Source-visible comments exist | Requires execution result, stop, branch-likely, exception, interrupt, ERET, Count, and rollback ownership | High | Blocked |
| Rust no-window step probe replacement | Source-visible C++ probe exists | Requires a separate represented-step probe audit before changing `fn64_machine_probe` or adding a Rust step probe | Medium | Future seam |

### Seam 044 Audit Changes

- Audited C++ `Machine::step_cpu_instruction`, `fetch_cpu_instruction_word`,
  `decode_cpu_instruction_word`, `identify_cpu_instruction`, and the C++
  no-window `fn64_step_probe` role.
- This historical seam added no Rust runtime behavior, no step API, no fetch API,
  no decode API, no execute API, and no placeholder result type. Later seams
  earned represented-category `Machine::step` without changing the full-step
  and generic-execute boundaries recorded here.
- Confirmed `fn64_machine_probe` replaces only construction/reset no-window
  inspection, not C++ step/execution proof responsibilities.
- Recommended `rust_parallel_core_seam_045_cpu_instruction_word_decode_representation_decision_and_seal`
  as the next implementation seam because C++ decode is pure raw instruction-word
  field extraction and does not require fetch, memory map, bus, execute, or step.

## CPU Instruction Word Decode Representation Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ raw instruction word type | `src/core/machine.hpp` `using CpuInstructionWord = std::uint32_t` | `cpu/instruction.rs` `CpuInstructionWord(u32)` | Equivalent value width, different ownership shape | Decode tests; source inspection | Rust wraps the raw `u32` to keep instruction words distinct from CPU addresses and RDRAM offsets. |
| C++ decoded field owner | `src/core/machine.hpp` private `DecodedCpuInstructionWord` | `cpu/instruction.rs` `CpuInstructionFields` | Equivalent for public raw field subset | Decode tests | Rust keeps decode under the CPU owner instead of C++ private `Machine` because the operation is pure CPU instruction-word representation. |
| Decode function | `src/core/machine_cpu.cpp` `Machine::decode_cpu_instruction_word(CpuInstructionWord raw)` | `cpu/instruction.rs` `decode_cpu_instruction_word(CpuInstructionWord)` | Equivalent for raw unsigned field subset | Decode tests; source inspection | Rust takes an already-formed raw word and performs no fetch, endian conversion, identify, execute, step, or mutation. |
| Raw word preservation | `DecodedCpuInstructionWord::raw = raw` | `CpuInstructionFields::raw` | Equivalent | `raw_word_is_preserved_exactly` | Raw bits are returned unchanged. |
| Opcode bits | `(raw >> 26) & 0x3f` | `CpuInstructionFields::opcode()` | Equivalent | `all_one_word_decodes_cpp_field_masks`; `individual_field_extraction_uses_cpp_bit_positions` | Bits 31..26. |
| rs bits | `(raw >> 21) & 0x1f` | `CpuInstructionFields::rs()` | Equivalent | Decode tests | Bits 25..21. |
| rt bits | `(raw >> 16) & 0x1f` | `CpuInstructionFields::rt()` | Equivalent | Decode tests | Bits 20..16. |
| rd bits | `(raw >> 11) & 0x1f` | `CpuInstructionFields::rd()` | Equivalent | Decode tests | Bits 15..11. |
| sa bits | `(raw >> 6) & 0x1f` | `CpuInstructionFields::sa()` | Equivalent | Decode tests | Bits 10..6. Rust uses C++ `sa` naming. |
| funct bits | `raw & 0x3f` | `CpuInstructionFields::funct()` | Equivalent | Decode tests | Bits 5..0. |
| Raw immediate bits | `static_cast<std::uint16_t>(raw & 0xffff)` | `CpuInstructionFields::immediate_u16()` | Equivalent | Decode tests | Bits 15..0 are preserved as raw unsigned bits. |
| Jump target bits | `raw & 0x03ffffff` | `CpuInstructionFields::jump_target()` | Equivalent | Decode tests | Bits 25..0. No jump target address formation is performed. |
| C++ signed immediate cache | `DecodedCpuInstructionWord::immediate_i16 = i16_from_u16_bits(immediate_u16)` | No Rust signed immediate accessor | C++ exists, Rust intentionally absent | Source inspection | The pass earns raw field extraction only. Signed immediate use remains future instruction/immediate semantics. |
| All-zero decode | Source formula yields zeros | `CpuInstructionFields` accessors | Equivalent | `all_zero_word_decodes_all_fields_to_zero` | Synthetic raw word only. |
| All-one decode | Source masks yield max field values | `CpuInstructionFields` accessors | Equivalent | `all_one_word_decodes_cpp_field_masks` | Proves field masks. |
| Representative R-type raw fields | C++ decode extracts fields before identify | Rust decode extracts the same raw fields | Equivalent | `representative_r_type_word_extracts_raw_fields` | No R-type instruction identity or execution is claimed. |
| Representative I-type raw fields | C++ decode extracts fields before identify | Rust decode extracts the same raw fields | Equivalent | `representative_i_type_word_extracts_raw_fields` | No immediate sign/zero extension or load/store semantics are claimed. |
| Representative J-type raw fields | C++ decode extracts target bits before identify | Rust decode extracts target bits | Equivalent | `representative_j_type_word_extracts_raw_target` | No jump target address formation is claimed. |
| No state mutation | C++ decode only fills a local struct | Rust decode returns a value object only | Equivalent | `decoding_does_not_mutate_cpu_machine_or_rdram_state` | No CPU, Machine, RDRAM, PC/next PC, GPR, or COP0 mutation occurs. |
| Fetch/endian relationship | C++ fetch forms `CpuInstructionWord` before decode | No Rust fetch or endian conversion in decode | Not in scope | Source inspection | Decode accepts an already-formed `u32`; fetch/read byte order is a later seam. |
| Identify relationship | `identify_cpu_instruction(decoded)` is separate from decode | Rust identity classification exists as a separate pure function | Equivalent layering | Source inspection; identity tests | Decode itself still performs no identity classification; seam 046 owns the next pure layer. |
| Execute/step relationship | `step_cpu_instruction` calls fetch, decode, identify, execute | Represented `Machine::step` exists separately; no generic execute/full step is owned by decode | Not in decode scope | Source inspection | Decode representation does not itself imply execution readiness. |

### Decode Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| CPU instruction identity classification | C++ `identify_cpu_instruction` is source-visible and now has a decoded-field input owner in Rust | Implemented in seam 046 | Medium | Complete |
| Instruction fetch representation | Still coupled to direct RDRAM fetch, SP DMEM fetch, unavailable PIF reset fetch, and fetch-time AdEL | Decode and identity are sealed; instruction-fetch target ownership, SP/PIF decisions remain | Medium-high | Recommended audit |
| Minimal execute subset | Requires identity, operand read/write, PC cadence, exceptions, and instruction-specific semantics | Decode plus identity plus execution prerequisites | High | Blocked |
| Step API | Requires fetch, identify, execute, rollback, exceptions, interrupts, Count, and stop/unsupported behavior | Multiple future seams | High | Blocked |

### Seam 045 Audit Changes

- Added `rust/crates/fn64-core/src/cpu/instruction.rs`.
- Added `CpuInstructionWord`, `CpuInstructionFields`, and
  `decode_cpu_instruction_word` for pure raw `u32` instruction-word field
  extraction.
- Exported the narrow decode representation through `cpu.rs` and `lib.rs`.
- Added synthetic decode tests for raw preservation, zero/all-one masks,
  individual field bit positions, representative R/I/J raw words, and no CPU /
  Machine / RDRAM mutation.
- Updated `rust/README.md` to list `cpu::instruction` as an earned owner and to
  distinguish raw decode representation from absent fetch, identify, execute,
  and step behavior.
- Added no fetch, endian conversion, instruction identity classification,
  execute, step, instruction writeback, branch/jump target formation,
  sign/zero-extension semantics, load/store semantics, memory map, bus, device,
  DMA, SDL/window runtime, host shell, or C++ source changes.

## CPU Instruction Identity Classification Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ identity enum | `src/core/machine.hpp` private `Machine::CpuInstructionIdentity` | `cpu/instruction.rs` `CpuInstructionIdentity` | Equivalent identity set, different ownership shape | Identity tests; source inspection | Rust keeps the pure identity names under the CPU instruction owner instead of C++ private `Machine` because no Machine state is read or mutated. |
| Identify function | `src/core/machine_cpu.cpp` `Machine::identify_cpu_instruction(const DecodedCpuInstructionWord&)` | `cpu/instruction.rs` `identify_cpu_instruction(CpuInstructionFields)` | Equivalent | Identity tests; source inspection | The Rust function consumes already-decoded raw fields and returns only an identity enum. |
| Primary opcode classification | C++ `switch (instruction.opcode)` direct cases | Rust primary-opcode `match` direct cases | Equivalent | `primary_opcode_identity_classification_matches_cpp_switch` | Covers J/JAL, branches, arithmetic immediates, direct load/store identities, D-width identities, cache, LL/SC identities, and coarse coprocessor memory identities named by C++. |
| SPECIAL/funct classification | C++ opcode `0x00` switches on `instruction.funct` | Rust opcode `0x00` matches `funct()` | Equivalent | `special_opcode_uses_funct_field_and_unknown_matches_cpp_switch` | SPECIAL uses funct, not immediate, target, or execution-time operands. |
| REGIMM/rt classification | C++ opcode `0x01` switches on `instruction.rt` | Rust opcode `0x01` matches `rt()` | Equivalent | `regimm_opcode_uses_rt_field_and_unknown_matches_cpp_switch` | REGIMM uses rt, including branch/link/likely and trap identities. |
| COP0 subidentity classification | C++ opcode `0x10` switches on `instruction.rs`; exact raw word `0x42000018` selects ERET | Rust opcode `0x10` matches `rs()` and exact raw bits for ERET | Equivalent | `cop0_identity_classification_uses_rs_and_exact_eret_raw_word` | MFC0, MTC0, ERET, and coarse COP0 are identity-only. No COP0 mutation or ERET behavior is added. |
| COP1/COP2/COP3 classification | C++ opcodes `0x11`, `0x12`, `0x13` return coarse identities | Rust returns `Cop1`, `Cop2`, `Cop3` | Equivalent | Primary-opcode test | Coarse unsupported decode boundaries only; no FPU/RSP/coprocessor behavior. |
| CACHE classification | C++ opcode `0x2f` returns `kCache` | Rust returns `Cache` | Equivalent | Primary-opcode test | Identity only; no cache state, coherence, or cache op behavior. |
| LL/SC identity classification | C++ opcodes `0x30`, `0x34`, `0x38`, `0x3c` return `LL`, `LLD`, `SC`, `SCD` identities | Rust returns `Ll`, `Lld`, `Sc`, `Scd` | Equivalent | Primary-opcode test | Identity only; no LL/SC instruction behavior or reservation match/writeback. |
| Unknown primary behavior | C++ default returns `kUnknownPrimary` | Rust returns `UnknownPrimary` | Equivalent | `unknown_primary_opcode_matches_cpp_default` | Unknown identity does not create an error or exception. Step later reports unsupported. |
| Unknown SPECIAL behavior | C++ SPECIAL default returns `kSpecialUnknown` | Rust returns `SpecialUnknown` | Equivalent | SPECIAL unknown test | Unknown identity does not create an error or exception. |
| Unknown REGIMM behavior | C++ REGIMM default returns `kRegimmUnknown` | Rust returns `RegimmUnknown` | Equivalent | REGIMM unknown test | Unknown identity does not create an error or exception. |
| NOP handling | C++ raw zero decodes opcode `0`, funct `0`, then identifies as `kSpecialSll` | Rust raw zero identifies as `SpecialSll` | Equivalent | `nop_classifies_as_special_sll_like_cpp` | There is no separate NOP identity. |
| Operand interpretation absent | C++ identify does not use signed immediates, branch offsets, jump address formation, GPR values, memory, or COP0 state | Rust identify uses only raw decoded fields and exact ERET raw bits | Equivalent | `identity_classification_does_not_interpret_operands` | Signed immediate interpretation, branch offsets, jump target address formation, load/store width effects, and operand reads are future instruction semantics. |
| No mutation | C++ identify returns an enum only | Rust identify returns an enum only | Equivalent | `identifying_does_not_mutate_cpu_machine_or_rdram_state` | No CPU, Machine, RDRAM, PC/next PC, GPR, COP0, reservation, or cartridge mutation. |
| Fetch/endian out of scope | C++ fetch forms the raw word before decode and identify | Rust identify accepts `CpuInstructionFields` only | Not in scope | Source inspection | No instruction fetch or endian conversion is added. |
| Raw field decode relationship | C++ identify consumes `DecodedCpuInstructionWord` | Rust identify consumes `CpuInstructionFields` from seam 045 | Equivalent layering | Decode and identity tests | This seam does not change raw field extraction. |
| Execute/step out of scope | C++ step calls fetch, decode, identify, and execute, then commits/rolls back state | Represented `Machine::step` exists separately; no generic execute/full step is owned by identity classification | Not in identity scope | Source inspection | Identity classification does not itself imply execution readiness. |
| Memory map/bus/device out of scope | C++ execute may route identified load/store/device instructions later | No Rust memory map, bus, device, or DMA behavior | Not in scope | Source inspection | Identity names load/store/device-adjacent instruction classes only. It does not route memory. |

### Identity Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| CPU instruction fetch readiness audit | C++ step fetches before decode/identify, and fetch has direct RDRAM, SP DMEM, unavailable PIF reset-fetch, alignment, and exception boundaries | Decode and identity are sealed; direct RDRAM value access and data address-error entry are sealed, but SP/PIF/fetch exception seams are not | Medium-high | Recommended |
| Minimal execute subset | Execution consumes identity and decoded fields, but mutates CPU/RDRAM/COP0/reservation/control-flow and may route devices | Decode and identity | High | Blocked |
| Step API | Requires fetch, decode, identity, execute, rollback, exceptions, interrupts, Count, and stop/unsupported behavior | Multiple future seams | High | Blocked |

### Seam 046 Audit Changes

- Added `CpuInstructionIdentity` and `identify_cpu_instruction` to
  `rust/crates/fn64-core/src/cpu/instruction.rs`.
- Exported the identity enum/function through `cpu.rs` and `lib.rs`.
- Mirrored the full C++ source-clear identity family: primary opcodes,
  SPECIAL/funct identities, REGIMM/rt identities, COP0 MFC0/MTC0/ERET/coarse
  identity, COP1/COP2/COP3 coarse identities, CACHE, LL/SC identity names, and
  unknown primary/SPECIAL/REGIMM boundaries.
- Preserved C++ NOP treatment: raw zero identifies as `SpecialSll`, not a
  separate NOP identity.
- Added synthetic identity tests for switch boundaries, unknowns, NOP, COP0
  ERET exact raw-word classification, operand non-interpretation, and no
  Machine/Cpu/RDRAM/COP0 mutation.
- Added no fetch, endian conversion, execute, step, instruction writeback,
  branch/jump target formation, sign/zero-extension semantics, load/store
  semantics, operand reads, memory map, bus, device, DMA, SDL/window runtime,
  host shell, or C++ source changes.

## Machine CPU Instruction Fetch Readiness and Direct RDRAM Fetch Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ fetch owner | `src/core/machine.hpp` private `Machine::fetch_cpu_instruction_word`; `src/core/machine_cpu.cpp` implementation | Current-PC wrapper, explicit-address represented-target fetch, selected-fault AdEL selection, and narrow entry mutation exist; no step-owned full equivalent | Not yet earned for step-owned full fetch | Source inspection; C++ gates | C++ fetch is private to Machine and reads from current `cpu_pc()`. Rust mirrors the PC read, represented target/value layer, selected-fault address-error plan, and separate narrow entry method; step result/rethrow handling remains absent. |
| Rust direct fetch owner | C++ direct RDRAM branch inside `fetch_cpu_instruction_word` | `machine.rs` `Machine::fetch_direct_rdram_cpu_instruction_word` | Equivalent for direct RDRAM subset, different public shape | `direct_rdram_instruction_fetch_*` tests | Rust source-specific fetch takes an explicit `CpuAddress` and fetches only from direct KSEG0/KSEG1 RDRAM. The current-PC wrapper is a separate Machine-owned composition seam. |
| Instruction word type | `src/core/machine.hpp` `using CpuInstructionWord = std::uint32_t` | `cpu/instruction.rs` `CpuInstructionWord(u32)` | Equivalent value width, different ownership shape | Fetch and decode tests | Fetch returns an already-formed raw instruction word. Decode and identity remain separate CPU-owned pure layers. |
| Alignment check ordering | `fetch_cpu_instruction_word` checks `(pc & 0x3) != 0` before any target translation | `fetch_direct_rdram_cpu_instruction_word` checks `cpu_address.value() & 0x3` before direct RDRAM access | Equivalent for direct subset | `direct_rdram_instruction_fetch_checks_alignment_before_target_rejection` | Unaligned fetch returns Rust API error data. It does not enter AdEL/COP0; C++ exception conversion is step-owned. |
| Direct CPU alias translation | `translate_direct_cpu_physical_address` accepts direct segment top bits `0x80000000` or `0xa0000000` and masks with `0x1fffffff` | `classify_direct_rdram_address` through `direct_rdram_offset` | Equivalent for KSEG0/KSEG1 RDRAM subset | Address classification tests plus fetch tests | Rust reuses the sealed direct RDRAM classifier; no TLB, bus, cartridge mapping, or full memory map is added. |
| RDRAM span translation | `translate_cpu_physical_rdram_address(physical_address, 4, out)` rejects width 0, oversized widths, and offsets past `kRdramSizeBytes - 4` | `classify_direct_rdram_address(cpu_address, 4)` | Equivalent | `direct_rdram_instruction_fetch_uses_last_valid_word_boundary` | Last valid instruction fetch offset is `RDRAM_SIZE_BYTES - 4`; exact end and past end are rejected. |
| Big-endian word formation | `fetch_cpu_instruction_word` direct RDRAM branch calls `read_rdram_u32_be` | `read_direct_rdram_u32_be` returns a u32, wrapped by `CpuInstructionWord::new` | Equivalent | `direct_rdram_instruction_fetch_reads_kseg0_and_kseg1_big_endian_words` | Fetch forms exactly one big-endian u32 instruction word. No endian conversion occurs in decode or identity. |
| KSEG0 direct RDRAM fetch | Direct `pc` translating to physical RDRAM returns `read_rdram_u32_be` | `fetch_direct_rdram_cpu_instruction_word(kseg0(...))` | Equivalent for direct subset | Fetch tests | Explicit CPU address input, no PC mutation. |
| KSEG1 direct RDRAM fetch | Direct `pc` translating to physical RDRAM returns `read_rdram_u32_be` | `fetch_direct_rdram_cpu_instruction_word(kseg1(...))` | Equivalent for direct subset | Fetch tests | Explicit CPU address input, no PC mutation. |
| Unsupported/non-direct address behavior | Non-direct fetch failure uses `kCpuRdramAddressRejected` with `kInstructionFetch` intent; step may rethrow if not a direct-target miss | Rust returns `MachineDirectRdramCpuInstructionFetchError::DirectRdram` | Equivalent for direct-fetch API safety only | `direct_rdram_instruction_fetch_rejects_non_direct_and_pif_reset_without_exception_entry` | Rust direct fetch reports rejection but does not reproduce C++ exception throwing or step intent taxonomy. This is Rust-only API safety, no emulator truth beyond rejected direct fetch. |
| Direct target miss behavior | C++ direct non-RDRAM/non-SP/non-PIF fetch uses `kInstructionFetchDirectTargetMiss`; step can convert selected misses to AdEL | Rust direct fetch returns `DirectRdram` rejection | Equivalent for direct RDRAM subset; step conversion absent | Fetch tests; source inspection | The lower helper remains a direct-access API. It does not enter address-error exceptions. |
| SP DMEM instruction fetch | `fetch_cpu_instruction_word` checks `translate_cpu_physical_sp_memory_address(..., 4, ...)` and reads SP DMEM only | `sp_dmem.rs` `SpDmem::read_u32_be`; `machine.rs` `Machine::fetch_sp_dmem_cpu_instruction_word`; `Machine::fetch_cpu_instruction_word_at`; `Machine::fetch_current_cpu_instruction_word` | Equivalent for read-only SP DMEM instruction-word fetch, explicit-address composition, and current-PC wrapper | SP DMEM fetch tests; explicit-address fetch tests; current-PC fetch tests; source inspection; `fn64_step_probe` | Rust forms an instruction word from an explicit classified SP DMEM offset and composes it through explicit-address and current-PC fetch. Fetch-fault selection and separate narrow entry mutation are sealed separately. |
| SP IMEM instruction fetch | C++ translation can identify SP IMEM, but fetch accepts only `sp_kind == kSpDmem` | No Rust SP IMEM fetch | C++ exists, Rust intentionally absent | Source inspection | This pass does not add SP target routing. |
| Unavailable PIF reset fetch | `is_unavailable_pif_rom_reset_fetch(pc, physical_address)` names reset PC `0xbfc00000` to physical `0x1fc00000`; helper throws unavailable PIF ROM reset fetch miss | `MachineCpuInstructionFetchTarget::PifResetUnavailable`; `MachineCpuInstructionFetchError::PifResetUnavailable` | Equivalent unavailable naming only; bytes intentionally absent | `fn64_step_probe` reset non-boot probe; target/error tests; source inspection | Rust `Machine::reset` still sets represented PC/next PC only. It names the unavailable fetch but does not fetch PIF/BIOS bytes or fake a blob. |
| Fetch error to AdEL conversion | `step_cpu_instruction` catches unaligned fetch and direct-target miss faults and calls local address-error entry when guards allow | `select_cpu_instruction_fetch_address_error`; `MachineInstructionFetchAddressErrorPlan`; `Machine::enter_instruction_fetch_address_error_exception` | Equivalent for selection and narrow local entry mutation | Instruction-fetch fault selection and entry tests; source inspection | Rust maps source-clear selected fetch faults to AdEL/code 4, preserves BadVAddr input, and enters only through the separate ordinary local entry method. Step result conversion/rethrow remains absent. |
| PC/next PC source | C++ full fetch reads `cpu_pc()` and does not read `cpu_next_pc_` for the fetch address | `Machine::fetch_current_cpu_instruction_word` reads `Cpu::pc()` and delegates to explicit-address fetch | Equivalent current-PC wrapper | Current-PC fetch tests | The wrapper does not advance PC or next PC. |
| PC/next PC mutation | C++ fetch itself is `const`; step mutates PC/next PC after fetch/execute or exception conversion | Rust direct fetch performs no PC/next PC mutation | Equivalent for fetch subset | `direct_rdram_instruction_fetch_preserves_machine_cpu_rdram_and_reservation_state` | Step cadence remains absent. |
| Count cadence | C++ fetch itself does not advance Count; step advances Count after committed instruction paths | Rust direct fetch performs no Count mutation | Equivalent for fetch subset | Fetch no-mutation test | Count progression remains absent. |
| State mutation | C++ direct fetch reads storage only; no storage/reservation/GPR/COP0 mutation in the direct read branch | Rust direct fetch reads only, returning `CpuInstructionWord` | Equivalent for direct subset | Fetch no-mutation test | RDRAM, reservation, CPU, COP0, GPR, and Cartridge facts are preserved. |
| Decode composition | C++ step calls decode after fetch | Rust tests may call `decode_cpu_instruction_word` on the fetched word | Equivalent layering | `fetched_direct_rdram_instruction_word_can_be_decoded_and_identified_by_sealed_cpu_layers` | Fetch does not decode internally. |
| Identity composition | C++ step calls identify after decode | Rust tests may call `identify_cpu_instruction` after decode | Equivalent layering | Fetch/decode/identity composition test | Fetch does not identify internally. |
| Execute/step out of scope | C++ step fetches, decodes, identifies, executes, and then commits/rolls back state | Represented `Machine::step` exists separately; no generic execute/full step is owned by direct fetch | Not in fetch scope | Source inspection | Direct fetch does not itself imply CPU step readiness. |
| Memory map/bus out of scope | C++ fetch has local target checks, not a Rust bus | No Rust memory map or bus | Not in scope | Source inspection | Direct fetch only uses sealed direct RDRAM classification and read access. |

### Fetch Source Readiness Map

| Fetch source/path | C++ behavior | Rust status | Blocker | Recommended status |
| --- | --- | --- | --- | --- |
| Direct KSEG0/KSEG1 RDRAM | Alignment check, direct translation, RDRAM width-4 bounds, big-endian `read_rdram_u32_be` | Target classified, direct RDRAM fetch implemented, explicit-address composition sealed, current-PC wrapper sealed, selected fault AdEL plan sealed, and narrow entry mutation sealed | Step-owned result/rethrow remains absent | Direct RDRAM instruction fetch sealed |
| SP DMEM | Direct physical SP memory translation, SP DMEM-only read as big-endian u32 | Target classified, read-only SP DMEM instruction fetch sealed, explicit-address composition sealed, current-PC wrapper sealed, selected fault AdEL plan sealed, and narrow entry mutation sealed | Step-owned result/rethrow remains absent | SP DMEM instruction fetch sealed |
| SP IMEM | Translation helper can identify SP IMEM but C++ instruction fetch does not accept it | Target miss classified | Must not infer unsupported source behavior beyond C++ branch | Direct target miss sealed |
| PIF/reset-vector | Reset PC physical target is named unavailable PIF ROM reset fetch and becomes step-time AdEL when allowed | Unavailable target classified, fetch absent, and pure AdEL selection sealed | No PIF ROM/blob behavior and no fetch-fault entry mutation | Target classification sealed; selection sealed |
| Unsupported direct target | Throws direct-target miss intent for step to handle or reject | Direct target miss classified | Step conversion absent | Target classification sealed |
| Non-direct CPU address | Throws instruction-fetch rejection intent, not the direct-target miss handled by step | Non-direct unsupported classified | Full C++ fault taxonomy and step rethrow behavior absent | Target classification sealed |

### Fetch Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| CPU instruction fetch target classification | C++ full fetch has a source-clear target split across DirectRdram, SP DMEM, unavailable PIF reset fetch, direct-target miss, and non-direct rejection | Direct RDRAM fetch sealed; decode/identity sealed | Medium | Sealed in seam 048 |
| SP DMEM storage/read readiness | Needed before SP DMEM instruction fetch can read words | Target classification sealed; explicit SP storage owner added in seam 049 | Medium | Sealed in seam 049 |
| Fetch-fault AdEL selection | C++ conversion happens in `step_cpu_instruction`, not in fetch | Current-PC fetch and fetch error taxonomy | Medium | Sealed in seam 052 for pure selection |
| Explicit-address instruction fetch over represented targets | Direct RDRAM and SP DMEM source-specific reads now exist; PIF reset remains unavailable by name | Target classification, direct RDRAM fetch, SP DMEM fetch | Medium | Sealed in seam 050 |
| Current-PC instruction fetch wrapper | C++ full fetch reads `cpu_pc()` | Explicit-address fetch composition and careful fault taxonomy | Medium | Sealed in seam 051 |
| Full `fetch_cpu_instruction_word` equivalent | Requires fetch fault-to-step exception/rethrow behavior in addition to represented target reads and pure selection | Multiple future seams | High | Blocked |
| CPU step | Requires fetch, decode, identity, execute, rollback, interrupts, exceptions, Count, and stop behavior | Multiple future seams | High | Blocked |

### Seam 047 Audit Changes

- Added `Machine::fetch_direct_rdram_cpu_instruction_word` and
  `MachineDirectRdramCpuInstructionFetchError` to
  `rust/crates/fn64-core/src/machine.rs`.
- Exported the narrow fetch error through `lib.rs`.
- Added synthetic tests for KSEG0/KSEG1 direct RDRAM fetch, big-endian word
  formation, last-valid word boundary, alignment-before-target-rejection
  ordering, non-direct/PIF-reset rejection without exception entry, composition
  with sealed decode/identity, and no Machine/Cpu/RDRAM/COP0/reservation
  mutation.
- That seam added no current-PC fetch, SP DMEM fetch, PIF/reset-vector fetch,
  fetch-time AdEL conversion, decode inside fetch, identity inside fetch,
  execute, step, PC/next PC cadence, Count cadence, instruction writeback,
  CPU load/store instruction behavior, memory map, bus, device/MMIO routing,
  DMA, SDL/window runtime, host shell, or C++ source changes.

## Machine CPU Instruction Fetch Target Classification Decision and Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ target split owner | `src/core/machine_cpu.cpp` `Machine::fetch_cpu_instruction_word`; `src/core/machine.cpp` direct alias, RDRAM, SP memory, and unavailable PIF reset helpers | `machine.rs` `Machine::classify_cpu_instruction_fetch_target` | Equivalent target split, no read behavior | Target classification tests; source inspection | Rust names the fetch target/rejection split without reading memory or using current `Cpu::pc`. |
| Rust target representation | C++ branches inside `fetch_cpu_instruction_word` | `MachineCpuInstructionFetchTarget::{DirectRdram, SpDmem, PifResetUnavailable}` | Equivalent behavior, different ownership shape | Target classification tests | The Rust type is Machine-owned because it names Machine storage/source targets. |
| Rust rejection representation | C++ `MachineFaultKind::kUnalignedInstructionFetch`; `kCpuRdramAddressRejected` with `kInstructionFetch` or `kInstructionFetchDirectTargetMiss` intents | `MachineCpuInstructionFetchTargetError::{Unaligned, NonDirectUnsupported, DirectTargetMiss}` | Equivalent target classification shape; Rust-only API safety | Target classification tests | Rust returns values instead of throwing. It does not enter AdEL or mutate COP0. |
| Alignment ordering | `fetch_cpu_instruction_word` checks `(pc & 0x3) != 0` before direct translation | `classify_cpu_instruction_fetch_target` returns `Unaligned` before target classification | Equivalent | `cpu_instruction_fetch_target_distinguishes_fetch_rejection_kinds` | This mirrors C++ ordering and keeps unaligned addresses distinct from target misses. |
| Direct alias translation | `translate_direct_cpu_physical_address` accepts top bits `0x80000000`/`0xa0000000`, then masks with `0x1fffffff` | `cpu/address.rs` crate-private `translate_direct_cpu_physical_address` reused by Machine classifier | Equivalent | Target classification tests plus direct-address tests | The helper remains crate-private; no broad physical-address API is exported. |
| Direct RDRAM target | `translate_cpu_physical_rdram_address(physical, 4, out)` succeeds and fetch reads RDRAM | `MachineCpuInstructionFetchTarget::DirectRdram { offset }` | Equivalent target classification | `cpu_instruction_fetch_target_classifies_kseg0_and_kseg1_direct_rdram` | Classification names the target only. `Machine::fetch_direct_rdram_cpu_instruction_word` remains the direct RDRAM read seam. |
| Last valid RDRAM fetch span | Width 4 accepts offset `kRdramSizeBytes - 4` | Direct target with `RDRAM_SIZE_BYTES - 4` | Equivalent | `cpu_instruction_fetch_target_uses_width_four_rdram_span_boundaries` | Exact end and past-end aligned direct addresses become direct target misses. |
| SP DMEM target | `translate_cpu_physical_sp_memory_address(..., 4, ...)` returns `kSpDmem`; fetch reads SP DMEM | `MachineCpuInstructionFetchTarget::SpDmem { offset: SpDmemOffset }` | Equivalent target classification | `cpu_instruction_fetch_target_classifies_sp_dmem_fetch_target` | Seam 048 named the target only; seam 049 now owns represented SP DMEM storage and read-only instruction-word fetch from this offset. |
| SP IMEM direct address | Shared C++ SP helper can identify `kSpImem`, but fetch accepts only `sp_kind == kSpDmem` | `DirectTargetMiss` | Equivalent for fetch classification | Target classification tests; source inspection | Rust does not create SP IMEM fetch behavior. |
| Unavailable PIF reset fetch | `is_unavailable_pif_rom_reset_fetch(pc, physical)` checks `0xbfc00000 -> 0x1fc00000`; helper throws unavailable PIF ROM reset fetch | `MachineCpuInstructionFetchTarget::PifResetUnavailable` | Equivalent target naming only | `cpu_instruction_fetch_target_names_unavailable_pif_reset_fetch`; C++ step probe | Rust names the unavailable source. It does not fetch PIF/BIOS bytes and does not fake a blob. |
| Non-direct unsupported fetch | Direct translation failure throws `kCpuRdramAddressRejected` with `kInstructionFetch` intent | `NonDirectUnsupported` | Equivalent rejection classification | Target classification tests | This remains distinct from direct-target miss because C++ step only converts the direct-target-miss intent to AdEL in the local path. |
| Direct target miss | Direct alias succeeds but RDRAM, SP DMEM, and PIF reset unavailable checks do not accept the target | `DirectTargetMiss` | Equivalent rejection classification | Target classification tests | This covers aligned RDRAM span past end, SP IMEM, and other unsupported direct physical targets. |
| Memory access | C++ target decisions happen before read; only accepted target branches read later | Rust classifier reads no RDRAM/SP/PIF/cartridge bytes | Equivalent for classification layer | No-mutation classification test; source inspection | The classifier is a static Machine method and does not take `&self`. |
| Machine/Cpu mutation | C++ fetch target checks are before step commit; step owns exception/PC/Count changes | Rust classifier performs no Machine/Cpu/COP0/RDRAM/reservation mutation | Equivalent for classification layer | `cpu_instruction_fetch_target_classification_preserves_machine_state` | Selected-fault AdEL planning and separate narrow entry mutation are sealed separately; classification remains pure. |
| Direct fetch API relationship | Direct RDRAM branch in C++ fetch is still the only read behavior mirrored in Rust | `Machine::fetch_direct_rdram_cpu_instruction_word` unchanged | Equivalent behavior preserved | Direct fetch focused tests | The new classifier does not change lower-level direct RDRAM value APIs or CPU-data APIs. |
| Full instruction fetch | C++ full fetch reads current `cpu_pc()`, can read RDRAM or SP DMEM, and reports faults to step | Current-PC wrapper, explicit-address represented-target fetch, selected-fault AdEL planning, and separate narrow entry mutation exist; no step-owned full equivalent | Not yet earned for step-owned full fetch | Source inspection | Step result/rethrow and normal cadence remain future seams. PIF reset remains a named unavailable error without bytes. |
| Decode/identity/execute/step | C++ step decodes, identifies, executes, and commits/rolls back after fetch | No Rust behavior added here | Not in scope | Source inspection | Target classification does not decode, identify, execute, or step. |
| Memory map/bus/device routing | C++ fetch uses local helper branches, not a Rust bus | No Rust memory map or bus | Not in scope | Source inspection | The target enum is not a memory map and does not route device/MMIO accesses. |

### Fetch Target Classification Status

| Target case | Rust status | Notes |
| --- | --- | --- |
| Direct RDRAM | Represented as `DirectRdram { offset }` | Source-clear and sealed for target classification. |
| SP DMEM | Represented as `SpDmem { offset: SpDmemOffset }` | Target classification sealed; read-only instruction-word fetch is sealed separately in seam 049. |
| PIF/reset unavailable | Represented as `PifResetUnavailable` | Named only; no PIF/BIOS/blob behavior exists. |
| Direct-target miss | Represented as `DirectTargetMiss` | Covers aligned direct targets not accepted by RDRAM, SP DMEM, or PIF reset unavailable checks. |
| Non-direct unsupported | Represented as `NonDirectUnsupported` | Kept separate from direct-target miss to mirror C++ access intent distinction. |
| Unaligned fetch | Represented as `Unaligned` before target classification | Mirrors C++ alignment-first ordering. |

### Seam 048 Audit Changes

- Added `MachineCpuInstructionFetchTarget`,
  `MachineCpuInstructionFetchTargetError`, and
  `Machine::classify_cpu_instruction_fetch_target` to
  `rust/crates/fn64-core/src/machine.rs`.
- Made `translate_direct_cpu_physical_address` crate-private in
  `cpu/address.rs` so the Machine-owned classifier can reuse sealed direct-alias
  truth without exposing a public physical-address API.
- Exported the target and error types through `lib.rs`.
- Added tests for KSEG0/KSEG1 direct RDRAM target offsets, width-4 RDRAM
  boundaries, SP DMEM target naming, unavailable PIF reset target naming,
  unaligned/non-direct/direct-target-miss rejection, and no Machine/Cpu/RDRAM/
  COP0/reservation mutation.
- Did not change `Machine::fetch_direct_rdram_cpu_instruction_word`.
- Seam 048 added no current-PC fetch, SP DMEM storage/read behavior,
  PIF/reset-vector fetch behavior, cartridge/PIF boot behavior, fetch-time AdEL
  selection or entry, current-PC fetch, decode/identify inside classification, execute,
  step, memory map, bus, device/MMIO routing, DMA, SDL/window runtime, host shell,
  or C++ source changes.

### Fetch Target Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| SP DMEM storage/read readiness for instruction fetch | Target classification names SP DMEM, and seam 049 adds read-only SP DMEM instruction fetch | Machine-owned SP memory representation | Medium | Sealed in seam 049 |
| Explicit-address instruction fetch over represented targets | C++ full fetch has DirectRdram and SP DMEM read paths; both source-specific reads are now sealed | Fetch target classification and source-specific reads | Medium | Sealed in seam 050 |
| Current-PC fetch wrapper | C++ full fetch reads `cpu_pc()`; Rust source-specific fetches still take explicit address/offset inputs | Fetch target classification and represented target reads | Medium | Sealed in seam 051 |
| PIF/reset unavailable fetch fault conversion | C++ step converts the named unavailable reset fetch to local AdEL when allowed | Pure selection sealed; entry context still absent | Medium-high | Selection sealed in seam 052; mutation blocked |
| Full `fetch_cpu_instruction_word` equivalent | Requires step fault conversion/rethrow behavior in addition to represented target reads | Multiple future seams | High | Blocked |
| CPU step | Still requires full fetch, decode/identity composition, execution, exceptions, Count, and PC cadence | Multiple future seams | High | Blocked |

## Machine SP DMEM Storage Readiness and Instruction Fetch Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ SP DMEM storage owner | `src/core/machine.hpp` `std::array<std::uint8_t, kSpMemorySizeBytes> sp_dmem_` | `sp_dmem.rs` `SpDmem`; `machine.rs` private `sp_dmem: SpDmem` | Equivalent behavior, different ownership shape | `default_sp_dmem_has_cpp_storage_size`; Machine construction tests; source inspection | Rust represents only SP DMEM storage, not SP registers/status/control, DMA, or RSP execution. |
| SP DMEM size | `Machine::kSpMemorySizeBytes = 4 * 1024` | `SP_DMEM_SIZE_BYTES = 4 * 1024` | Equivalent | SP DMEM storage tests | Size is exactly 4096 bytes. |
| SP DMEM offset type | C++ uses `std::uint32_t sp_memory_offset` from `translate_cpu_physical_sp_memory_address` | `SpDmemOffset(u32)` | Equivalent behavior, different ownership shape | Target classification and fetch tests | The Rust newtype prevents confusing SP DMEM offsets with RDRAM offsets or CPU addresses. |
| Construction/reset zero-fill | `reset_to_non_boot_power_on_state` calls `sp_dmem_.fill(0)`; constructor calls reset | `SpDmem::default`; `Machine::from_cartridge`; `Machine::reset` | Equivalent for represented SP DMEM | Machine reset tests | Reset clears prior test-staged SP DMEM bytes. Cartridge remains preserved; no boot/PIF staging is performed. |
| SP DMEM target classification | `fetch_cpu_instruction_word` accepts `translate_cpu_physical_sp_memory_address(..., 4, ...) && sp_kind == kSpDmem` | `Machine::classify_cpu_instruction_fetch_target` returns `SpDmem { offset: SpDmemOffset }` | Equivalent target classification | Target classification tests | Seam 049 changes only the offset representation. Classification still performs no memory read. |
| SP DMEM offset rules | C++ helper rejects width 0, width over 4096, physical below `0x04000000`, and offsets past `4096 - width` | `translate_cpu_physical_sp_dmem_instruction_fetch_address` uses width 4 and `SP_DMEM_SIZE_BYTES - width` | Equivalent for instruction fetch target width | Target classification tests | Last valid fetch offset is `0xffc`; `0x1000` is rejected as a direct target miss. |
| Big-endian u32 storage read | `read_sp_memory_u32_be` composes bytes as `offset`, `offset+1`, `offset+2`, `offset+3` into high-to-low u32 bits | `SpDmem::read_u32_be`; `Machine::fetch_sp_dmem_cpu_instruction_word` | Equivalent | `sp_dmem_instruction_fetch_reads_one_big_endian_word`; `sp_dmem_u32_be_read_observes_big_endian_storage_order` | Fetch forms exactly one `CpuInstructionWord` from the u32. |
| Last valid SP DMEM u32 fetch | C++ target translation accepts width-4 span ending at byte 4095 | `fetch_sp_dmem_cpu_instruction_word(SpDmemOffset::new(0xffc))` succeeds | Equivalent | `sp_dmem_instruction_fetch_uses_width_four_span_boundary` | The storage read returns `CpuInstructionWord::new(value)`. |
| Span-past-end rejection | C++ target translation rejects offsets above `4096 - 4`; raw private reads assume a valid translated offset | `SpDmem::read_u32_be` and Machine fetch return `SpDmemReadError`/`MachineSpDmemCpuInstructionFetchError` | Equivalent behavior, Rust-only API safety | SP DMEM boundary tests | Rust returns explicit errors instead of unchecked indexing/throwing. This does not add emulator truth. |
| Unaligned SP DMEM CPU address ordering | C++ full fetch checks `(pc & 0x3)` before direct/SP target checks | `classify_cpu_instruction_fetch_target` returns `Unaligned` before SP DMEM target classification | Equivalent | `cpu_instruction_fetch_target_distinguishes_fetch_rejection_kinds` | `fetch_sp_dmem_cpu_instruction_word` takes an already-classified offset and does not add a second alignment rule. |
| Public SP DMEM writes | C++ has private `write_sp_memory_*` used by CPU data/device/DMA paths | No public Rust SP DMEM write API | C++ exists, Rust intentionally absent | Source inspection | A `#[cfg(test)] pub(crate)` staging method seeds synthetic bytes only for proof; CPU/device writes remain absent. |
| SP IMEM | C++ has `sp_imem_` and can translate SP IMEM for data targets; instruction fetch accepts only DMEM | No Rust SP IMEM storage/read behavior | C++ exists, Rust intentionally absent | Source inspection | SP IMEM is a separate future seam if needed. |
| SP device/register/DMA behavior | C++ has SP MMIO and DMA helpers around SP memory | No Rust SP device, register, status, control, or DMA behavior | Not in scope | Source inspection | Representing SP DMEM bytes does not earn device semantics. |
| RDRAM/reservation effects | C++ SP DMEM fetch reads SP storage only | `fetch_sp_dmem_cpu_instruction_word` reads SP DMEM only | Equivalent | `sp_dmem_instruction_fetch_preserves_machine_state` | RDRAM bytes and `CpuRdramReservation` are unchanged. |
| CPU/COP0/PC effects | C++ fetch read branch does not mutate CPU state; step owns PC/Count/COP0 changes | Rust SP DMEM fetch does not mutate CPU, COP0, PC, next PC, or Count | Equivalent for fetch read branch | SP DMEM no-mutation test | Pure fetch-fault AdEL selection is separate; entry mutation and step remain absent. |
| Decode/identity composition | C++ step decodes and identifies after fetch | Tests compose `decode_cpu_instruction_word` and `identify_cpu_instruction` after fetch | Equivalent layering | SP DMEM composition test | SP DMEM fetch does not decode or identify internally. |
| PIF/reset fetch | C++ names unavailable PIF ROM reset fetch separately | Target classification and explicit-address fetch return `PifResetUnavailable`; no bytes are fetched | Equivalent unavailable naming only | Source inspection; PIF reset target/error tests | No PIF/BIOS blobs, boot execution, or cartridge substitute bytes are added. |
| Full instruction fetch/current PC | C++ full fetch reads `cpu_pc()` and can reach RDRAM, SP DMEM, or PIF/unavailable errors | Current-PC wrapper, explicit-address represented-target fetch, selected-fault AdEL planning, and separate narrow entry mutation exist | Equivalent for current-PC represented-target fetch, pure selection, and narrow entry | Source inspection; current-PC, selection, and entry tests | Source-specific direct RDRAM and SP DMEM reads compose through explicit-address fetch, and the current-PC wrapper supplies `Cpu::pc()`. |

### SP DMEM Fetch Status

| Topic | Status | Notes |
| --- | --- | --- |
| SP DMEM represented storage | Sealed | Machine-owned 4 KiB byte storage, zero-filled on construction/reset. |
| SP DMEM instruction-word fetch | Sealed | Explicit `SpDmemOffset` input, one big-endian u32, returns `CpuInstructionWord`. |
| Public SP DMEM writes | Intentionally absent | Only test-local staging exists; CPU data writes, device writes, DMA writes, and cartridge IPL3 staging are not ported. |
| `Machine::classify_cpu_instruction_fetch_target` | Changed only in representation | `SpDmem` now carries `SpDmemOffset`; target behavior is unchanged. |
| `Machine::fetch_direct_rdram_cpu_instruction_word` | Unchanged | Direct RDRAM fetch remains sealed and separate. |
| Full fetch/step replacement | Not yet earned | Step fault conversion is still future work. |

### Seam 049 Audit Changes

- Added `sp_dmem.rs` with `SpDmem`, `SpDmemOffset`, `SpDmemReadError`, and
  `SP_DMEM_SIZE_BYTES`.
- Made `Machine` own represented SP DMEM storage, construct it zero-filled, and
  reset it to zero-filled state.
- Updated `MachineCpuInstructionFetchTarget::SpDmem` to carry `SpDmemOffset`.
- Added `Machine::fetch_sp_dmem_cpu_instruction_word` and
  `MachineSpDmemCpuInstructionFetchError`.
- Added tests for SP DMEM construction/reset size and zero-fill, big-endian u32
  word formation, last-valid and span-past-end offsets, decode/identity
  composition outside fetch, unaligned CPU-address classification ordering, and
  no CPU/COP0/RDRAM/reservation/Cartridge mutation.
- Added no public SP DMEM write API, SP IMEM, SP device registers/status/control,
  SP DMA, PIF/reset fetch, cartridge/PIF boot behavior, general instruction
  fetch, current-PC fetch, fetch-time AdEL selection or entry, execute, step, memory map,
  bus, device routing, host shell, SDL/window runtime, or C++ source changes.

### SP DMEM Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Explicit-address instruction fetch over represented targets | Direct RDRAM and SP DMEM source-specific instruction-word reads are now sealed; the target classifier already names unavailable PIF reset and rejection cases | Direct RDRAM fetch, SP DMEM fetch, target classifier | Medium | Sealed in seam 050 |
| Current-PC instruction fetch wrapper | C++ full fetch reads `cpu_pc()` | Explicit-address fetch composition plus careful fault taxonomy | Medium | Sealed in seam 051 |
| PIF/reset unavailable fetch error handling | Full fetch must preserve the named unavailable reset-vector behavior | Target classifier names `PifResetUnavailable`; no PIF bytes; pure selection sealed | Medium-high | Bytes and entry still absent |
| Fetch-fault AdEL entry mutation | C++ step, not fetch, converts selected fetch faults to local address-error entry | Pure selection sealed; step exception context absent | High | Blocked |
| CPU step | Still needs fetch composition, execute, PC/Count cadence, rollback, interrupts, and exceptions | Multiple future seams | High | Blocked |

## Machine Explicit-Address Instruction Fetch Over Represented Targets Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ fetch owner | `src/core/machine_cpu.cpp` `Machine::fetch_cpu_instruction_word` | `machine.rs` `Machine::fetch_cpu_instruction_word_at`; `Machine::fetch_current_cpu_instruction_word` | Equivalent for explicit-address composition and current-PC wrapper, different public shape | Explicit-address and current-PC fetch tests; source inspection | C++ reads `cpu_pc()` internally. Rust keeps explicit-address fetch separate and adds a current-PC wrapper below step. |
| Error owner | C++ throws `MachineFault` variants from `fail_unaligned_instruction_fetch`, `fail_cpu_direct_rdram_address`, and `fail_unavailable_pif_rom_reset_fetch` | `MachineCpuInstructionFetchError` | Equivalent behavior, Rust-only API safety | Error taxonomy tests | Rust returns a typed error instead of throwing. This does not add emulator truth. |
| Ordering | C++ checks `(pc & 0x3)` first, then direct translation/target checks, then source read | `fetch_cpu_instruction_word_at` calls `classify_cpu_instruction_fetch_target` before any source read, then dispatches accepted represented targets | Equivalent | No-mutation and named-rejection tests | Unaligned, non-direct, direct-target-miss, and PIF-reset-unavailable cases return before RDRAM/SP reads. |
| Direct RDRAM dispatch | C++ direct RDRAM target reads `read_rdram_u32_be(rdram_address)` | `fetch_cpu_instruction_word_at` dispatches `DirectRdram` to `fetch_direct_rdram_cpu_instruction_word(cpu_address)` | Equivalent for represented direct RDRAM source | KSEG0/KSEG1 explicit fetch tests | The lower-level direct RDRAM fetch API remains unchanged. |
| SP DMEM dispatch | C++ SP DMEM target reads `read_sp_memory_u32_be(kSpDmem, sp_memory_offset)` | `fetch_cpu_instruction_word_at` dispatches `SpDmem { offset }` to `fetch_sp_dmem_cpu_instruction_word(offset)` | Equivalent for represented SP DMEM source | SP DMEM explicit fetch tests | The lower-level SP DMEM fetch API remains unchanged. |
| PIF reset unavailable | C++ identifies reset-vector PIF ROM fetch as unavailable and throws without producing bytes | `PifResetUnavailable { cpu_address }` | Equivalent unavailable naming only | PIF reset error test; source inspection | Rust does not fake PIF/BIOS bytes and does not use cartridge bytes as a substitute. |
| Non-direct unsupported | Direct translation failure throws a CPU RDRAM address rejection with instruction-fetch intent | `NonDirectUnsupported { cpu_address }` | Equivalent rejection classification | Non-direct error test | Rust keeps this separate from direct-target miss. |
| Direct-target miss | Direct alias succeeds but RDRAM/SP DMEM/PIF reset checks do not accept the target | `DirectTargetMiss { cpu_address }` | Equivalent rejection classification | Direct-target-miss tests | This includes aligned RDRAM span past end and other unsupported direct physical targets. |
| Unaligned fetch | C++ checks low bits before target checks | `Unaligned { cpu_address }` | Equivalent | Unaligned error test | No source read occurs on unaligned explicit-address fetch. |
| Big-endian source reads | C++ direct RDRAM and SP DMEM source branches form u32 from big-endian bytes | Existing source-specific Rust fetch APIs form `CpuInstructionWord` from big-endian u32 reads | Equivalent | Direct RDRAM and SP DMEM fetch tests | The explicit-address API does not duplicate byte composition; it delegates to sealed source fetches. |
| Decode/identity relationship | C++ decodes and identifies after fetch in step | `fetch_cpu_instruction_word_at` returns `CpuInstructionWord` only | Equivalent layering | Composition test decodes/identifies outside fetch | Fetch does not decode or identify internally. |
| PC / next PC / Count | C++ fetch itself reads current PC but step owns cadence and Count changes | Rust explicit-address fetch does not read PC; `fetch_current_cpu_instruction_word` reads PC but mutates no PC, next PC, or Count | Equivalent for below-step fetch composition | No-mutation tests | Step cadence remains absent. |
| CPU/COP0/GPR effects | C++ fetch faults are consumed later by step exception handling | Rust explicit-address fetch does not mutate CPU, COP0, or GPR state and does not enter exceptions | Equivalent below exception-entry layer | No-mutation test | Fetch-fault AdEL selection and separate narrow entry mutation are sealed separately; fetch APIs remain non-mutating. |
| RDRAM/SP DMEM/reservation effects | C++ fetch reads only accepted source storage | Rust explicit-address fetch performs no RDRAM or SP DMEM mutation and no reservation invalidation | Equivalent | No-mutation test | This is read-only instruction-word formation. |
| Current-PC fetch | C++ full fetch reads `cpu_pc()` | `Machine::fetch_current_cpu_instruction_word` | Equivalent wrapper | Current-PC fetch tests | The wrapper reads represented `Cpu::pc()` and delegates to explicit-address fetch. |
| Fetch-time AdEL conversion | `step_cpu_instruction` converts selected fetch faults to local AdEL when allowed | `select_cpu_instruction_fetch_address_error`; `Machine::enter_instruction_fetch_address_error_exception` as a separate operation | Equivalent for selection and narrow entry; not called by fetch | Selection and entry tests; source inspection | Fetch APIs remain non-mutating and step remains absent. |
| Memory map / bus / device routing | C++ fetch uses local direct RDRAM/SP/PIF checks, not a Rust bus | No Rust memory map or bus | Not in scope | Source inspection | The explicit-address API dispatches only already represented targets. |

### Seam 050 Audit Changes

- Added `MachineCpuInstructionFetchError` to name explicit-address fetch
  failures for unaligned, non-direct unsupported, direct-target miss,
  unavailable PIF reset fetch, and lower-level represented-source errors.
- Added `Machine::fetch_cpu_instruction_word_at(CpuAddress)` to compose the
  sealed target classifier with sealed direct RDRAM and SP DMEM instruction-word
  fetch APIs.
- Exported `MachineCpuInstructionFetchError` through `fn64-core`.
- Added tests for KSEG0/KSEG1 direct RDRAM fetch, SP DMEM fetch, decode/identity
  composition outside fetch, named rejection behavior, and no PC/next PC/Count/
  GPR/COP0/RDRAM/SP DMEM/reservation/Cartridge mutation.
- Did not change `Machine::fetch_direct_rdram_cpu_instruction_word`,
  `Machine::fetch_sp_dmem_cpu_instruction_word`, or
  `Machine::classify_cpu_instruction_fetch_target`.
- Added no current-PC fetch, CPU step, fetch/decode/identify/execute loop,
  instruction execution, instruction writeback, fetch-time AdEL conversion,
  generic exception conversion, PIF/BIOS blob behavior, cartridge boot behavior,
  SP device behavior, SP DMA, memory map, bus, device routing, SDL/window
  runtime, host shell, or C++ source changes.

### Explicit-Address Fetch Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Current-PC instruction fetch wrapper | C++ full fetch reads `cpu_pc()` before the same target/source split; explicit-address fetch is now sealed | Explicit-address fetch and careful fault taxonomy | Medium | Sealed in seam 051 |
| PIF/reset unavailable fetch error handling | Current reset PC reaches the unavailable PIF path | Target classifier and explicit-address fetch error | Medium-high | Needs future pass |
| Fetch-fault AdEL selection | C++ step converts selected fetch faults, not fetch itself | Fetch error taxonomy | Medium | Sealed in seam 052 for pure selection |
| CPU step | Still needs fetch, decode/identity composition, execution, Count/PC cadence, rollback, interrupts, and exceptions | Multiple future seams | High | Blocked |
| Memory map / bus audit | C++ local fetch/data helpers still are not a Rust bus | Step/fetch/data access readiness | High | Not recommended before fetch-time fault handling |

## Machine Current-PC Instruction Fetch Wrapper Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Current fetch address source | `src/core/machine_cpu.cpp` `Machine::fetch_cpu_instruction_word` uses `const CpuAddress pc = cpu_pc()` | `machine.rs` `Machine::fetch_current_cpu_instruction_word` uses `self.cpu.pc()` and `CpuAddress::new` | Equivalent | Current-PC fetch tests; source inspection | C++ uses current PC, not next PC, as the fetch address. |
| Delegation to represented target fetch | C++ fetch applies the existing target/source split after reading PC | `fetch_current_cpu_instruction_word` delegates to `fetch_cpu_instruction_word_at` | Equivalent wrapper | Current-PC fetch tests | The wrapper adds no separate target logic and no new error type. |
| Reset PC / unavailable PIF | C++ non-boot reset PC `0xbfc00000` reaches unavailable PIF reset fetch | `fetch_current_cpu_instruction_word` returns `MachineCpuInstructionFetchError::PifResetUnavailable` at construction/reset PC | Equivalent unavailable naming only | `current_pc_instruction_fetch_uses_reset_pc_and_reports_pif_unavailable` | Rust still does not fetch PIF/BIOS bytes or fake a blob. |
| Direct RDRAM current-PC fetch | C++ current PC in direct RDRAM reads one big-endian RDRAM instruction word | Wrapper delegates to explicit-address DirectRdram path | Equivalent | `current_pc_instruction_fetch_reads_direct_rdram_and_sp_dmem_targets` | KSEG0 and KSEG1 direct RDRAM PC values are covered. |
| SP DMEM current-PC fetch | C++ current PC in SP DMEM direct alias reads one big-endian SP DMEM instruction word | Wrapper delegates to explicit-address SpDmem path | Equivalent | Current-PC SP DMEM fetch test | This is still storage read-only, not SP device behavior. |
| Unaligned current PC | C++ checks `(pc & 0x3)` before target classification | Wrapper returns the same `Unaligned` error as explicit-address fetch | Equivalent | Current-PC rejection-equivalence test | No source read or exception entry occurs. |
| Non-direct unsupported current PC | C++ direct translation failure reports instruction-fetch rejection | Wrapper returns the same `NonDirectUnsupported` error as explicit-address fetch | Equivalent | Current-PC rejection-equivalence test | No memory map or bus is created. |
| Direct-target miss current PC | C++ direct alias outside accepted RDRAM/SP/PIF reset target reports direct-target miss | Wrapper returns the same `DirectTargetMiss` error as explicit-address fetch | Equivalent | Current-PC rejection-equivalence test | Pure AdEL selection is sealed separately; exception entry remains step-owned and absent. |
| Decode and identity relationship | C++ step decodes and identifies after fetch | Wrapper returns `CpuInstructionWord`; tests decode/identify outside the wrapper | Equivalent layering | Current-PC composition test | The wrapper does not decode or identify internally. |
| PC / next PC / Count mutation | C++ fetch is `const`; step owns PC/next PC and Count cadence | Wrapper uses `&self` and mutates no PC, next PC, or Count | Equivalent | Current-PC no-mutation test | This is not PC cadence and not step. |
| CPU / COP0 / GPR mutation | C++ fetch fault conversion is later in step | Wrapper mutates no CPU, COP0, or GPR state and enters no exception | Equivalent below step exception layer | Current-PC no-mutation test | Fetch-fault AdEL selection and narrow entry mutation are separate explicit operations. |
| RDRAM / SP DMEM / reservation mutation | C++ fetch reads accepted source storage only | Wrapper performs no RDRAM/SP DMEM mutation and no CpuRdramReservation invalidation | Equivalent | Current-PC no-mutation test | Fetch is read-only. |
| `next_pc` as fetch address | C++ does not use `cpu_next_pc_` for `fetch_cpu_instruction_word` address | No Rust next-PC fetch API | Not in scope | Source inspection | Branch/delay-slot fetch semantics remain absent. |
| Step relationship | C++ `step_cpu_instruction` wraps fetch, decode, identify, execute, commit/rollback, and fetch-fault exception conversion | Represented `Machine::step` exists separately; current-PC fetch remains only one component | Not in fetch-wrapper scope | Source inspection; gates | Current-PC fetch alone does not imply step readiness. |
| Memory map / bus / device routing | C++ fetch uses local target checks | No Rust memory map or bus | Not in scope | Source inspection | Current-PC fetch delegates to the already sealed narrow target/source layer. |

### Seam 051 Audit Changes

- Added `Machine::fetch_current_cpu_instruction_word`, a read-only Machine-owned
  wrapper that reads represented `Cpu::pc()`, converts it to `CpuAddress`, and
  delegates to `Machine::fetch_cpu_instruction_word_at`.
- Added tests for construction/reset PC unavailable PIF behavior, KSEG0/KSEG1
  direct RDRAM current-PC fetch, SP DMEM current-PC fetch, error equivalence with
  explicit-address fetch, decode/identity composition outside the wrapper, and
  no PC/next PC/Count/GPR/COP0/RDRAM/SP DMEM/reservation/Cartridge mutation.
- Did not change `Machine::fetch_cpu_instruction_word_at`,
  `Machine::fetch_direct_rdram_cpu_instruction_word`,
  `Machine::fetch_sp_dmem_cpu_instruction_word`, or
  `Machine::classify_cpu_instruction_fetch_target`.
- Added no PC advancement, next PC advancement, Count cadence, CPU step,
  fetch/decode/identify/execute loop, instruction execution, instruction
  writeback, fetch-time AdEL conversion, generic exception conversion, PIF/BIOS
  blob behavior, cartridge boot behavior, SP device behavior, SP DMA, memory map,
  bus, device routing, SDL/window runtime, host shell, or C++ source changes.

## Machine Instruction-Fetch Address-Error Entry Mutation Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Selection owner | `src/core/machine_cpu.cpp` `step_cpu_instruction` catch block after `fetch_cpu_instruction_word` | `machine.rs` `select_cpu_instruction_fetch_address_error` | Equivalent for pure selection | Instruction-fetch fault selection tests | Rust still does not call this from any fetch API. |
| Entry owner | `src/core/machine_cpu.cpp` `step_cpu_instruction` guarded call to `enter_local_address_error_exception` | `machine.rs` `Machine::enter_instruction_fetch_address_error_exception`; `cpu/cop0.rs` `Cpu::enter_instruction_fetch_address_error_exception` | Equivalent for narrow ordinary fetch AdEL entry | Instruction-fetch address-error entry tests | Machine consumes the Machine-owned fetch plan; CPU/COP0 mutation stays crate-private and narrow. |
| Selected exception kind | `kCop0ExceptionCodeAddressErrorLoad = 4` | `MachineInstructionFetchAddressErrorPlan::exception_kind() == AddressErrorLoad`; `cause_exception_code() == 4` | Equivalent | `instruction_fetch_fault_selection_maps_source_clear_faults_to_adel` | Instruction fetch faults are read-side address errors only. |
| Faulting address / BadVAddr | C++ passes `fault.cpu_address()` to `enter_local_address_error_exception` as `bad_vaddr` | `MachineInstructionFetchAddressErrorPlan::bad_vaddr()` and `Cpu::enter_instruction_fetch_address_error_exception` write `cop0.bad_vaddr` | Equivalent | Selection and entry tests | BadVAddr is the faulting fetch CPU address. |
| Unaligned fetch conversion and entry | C++ maps `kUnalignedInstructionFetch` with `kInstructionFetch` intent to AdEL when local synchronous entry is allowed | `MachineCpuInstructionFetchError::Unaligned` maps to `Unaligned`; `Machine::enter_instruction_fetch_address_error_exception` enters AdEL | Equivalent | Selection tests; entry mutation test | Unaligned fetch PC can enter only through the ordinary sequential PC/next PC guard. |
| Direct-target miss conversion and entry | C++ maps `kCpuRdramAddressRejected` with `kInstructionFetchDirectTargetMiss` intent to AdEL when local synchronous entry is allowed | `MachineCpuInstructionFetchError::DirectTargetMiss` maps to `DirectTargetMiss`; entry method enters AdEL | Equivalent | Selection tests; entry mutation test | Includes aligned direct aliases not accepted by represented fetch targets. |
| Unavailable PIF reset conversion and entry | C++ unavailable PIF reset fetch throws `kCpuRdramAddressRejected` with `kInstructionFetchDirectTargetMiss` intent and step enters AdEL when allowed | `MachineCpuInstructionFetchError::PifResetUnavailable` maps to `PifResetUnavailable`; entry method enters AdEL | Equivalent | Selection tests; entry mutation test; `fn64_step_probe` source inspection | Rust still does not fetch PIF/BIOS bytes. |
| Non-direct unsupported fetch | C++ non-direct fetch rejection uses `kInstructionFetch` intent and is not converted by the fetch catch block | `MachineCpuInstructionFetchError::NonDirectUnsupported` returns `MachineInstructionFetchAddressErrorPlanError` | Equivalent non-converting boundary | Non-converting selection test | Blank/raw/non-direct fetch rejection remains outside local AdEL selection. |
| Lower source-specific errors | C++ step sees fetch faults through the full fetch path, not direct source-specific helper errors | `DirectRdram` and `SpDmem` source errors return `MachineInstructionFetchAddressErrorPlanError` | Rust-only API safety, no emulator truth | Non-converting selection test | These lower errors are not the C++ step catch categories. |
| EPC source | C++ passes `current_pc` as `faulting_pc` and `branch_delay=false`, so EPC becomes current PC | Entry method uses current represented CPU PC as EPC | Equivalent for ordinary fetch entry | Entry mutation test | Delay-slot fetch entry remains blocked because C++ fetch catch uses the ordinary guard only. |
| Branch-delay flag | C++ fetch-fault conversion passes `branch_delay=false`; it does not use the delay-slot address-error arm | Entry method writes `cop0.exception_branch_delay = false` | Equivalent | Entry mutation test | Branch/delay-slot execution behavior remains absent. |
| Status.EXL handling | C++ requires `local_synchronous_exception_entry_allowed(current_pc, current_next_pc)` and then sets EXL in entry | Entry method requires sequential PC/next PC and EXL clear, then sets Status.EXL | Equivalent | Entry mutation and blocked-context tests | EXL-already-set returns `CpuAddressErrorExceptionEntryError` without mutation. |
| PC / next PC vectoring | C++ entry sets `kLocalInterruptVectorPc = 0x80000180` and next PC `0x80000184` | Entry method sets PC `0x80000180` and next PC `0x80000184` | Equivalent | Entry mutation test | No BEV-dependent vector selection exists in the inspected source for this local entry. |
| BEV/vector handling | C++ local address-error entry uses fixed local vector constants; no BEV branch in this path | Rust uses the same fixed local vector constants | Equivalent for local entry | Source inspection; entry mutation test | Broad exception-vector/BEV behavior remains absent. |
| EXL-already-set behavior | C++ `local_synchronous_exception_entry_allowed` rejects when EXL is set; fetch fault is rethrown by step | Entry method returns `CpuAddressErrorExceptionEntryError` and mutates nothing | Equivalent below step result handling | Blocked-context test | Rust has no step rethrow/result behavior. |
| Non-sequential PC/next PC behavior | C++ fetch-fault catch only uses `local_synchronous_exception_entry_allowed`; non-sequential delay-slot context is not converted | Entry method rejects non-sequential PC/next PC and mutates nothing | Equivalent below step result handling | Blocked-context test | Delay-slot fetch-fault entry is intentionally absent. |
| Count cadence | C++ fetch-fault conversion occurs before committed-instruction Count advancement and returns `kException`; Count does not advance | Entry method does not mutate Count | Equivalent for narrow entry | Entry mutation test | Normal Count cadence remains step-owned. |
| Fetch API behavior | C++ fetch throws; step catches selected faults later | Rust `fetch_cpu_instruction_word_at` and `fetch_current_cpu_instruction_word` still return fetch errors | Equivalent below step layer | Existing fetch tests plus selection no-mutation test | Fetch APIs do not enter exceptions. |
| Decode / identify / execute relationship | C++ fetch-fault conversion occurs before decode and identify | Entry method consumes a selected fetch fault plan only | Equivalent below execution layer | Source inspection; entry tests | No fetch, decode, identify, execute, or step API is added. |
| RDRAM / SP DMEM / reservation effects | C++ local address-error entry does not mutate memory or reservation state | Entry method mutates only CPU/COP0/control-flow state | Equivalent | Entry mutation test | No RDRAM/SP DMEM mutation or CpuRdramReservation invalidation occurs. |
| Memory map / bus / devices | C++ uses local fetch target checks | No Rust map, bus, or device routing | Not in scope | Source inspection | Entry does not broaden target routing. |

### Seam 052 And 053 Audit Changes

- Added `MachineInstructionFetchAddressErrorSource`,
  `MachineInstructionFetchAddressErrorPlan`,
  `MachineInstructionFetchAddressErrorPlanError`, and
  `select_cpu_instruction_fetch_address_error` in `machine.rs`.
- Mapped only source-clear C++ step-convertible fetch errors to AdEL/code 4:
  `Unaligned`, `DirectTargetMiss`, and `PifResetUnavailable`.
- Preserved `NonDirectUnsupported`, lower direct-RDRAM source errors, and lower
  SP DMEM source errors as non-converting fetch faults.
- Added tests for convertible selection, non-converting preservation, future
  BadVAddr input preservation, source-category preservation, and no
  Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge
  mutation during selection.
- Added `Machine::enter_instruction_fetch_address_error_exception` in
  `machine.rs` and crate-private
  `Cpu::enter_instruction_fetch_address_error_exception` in `cpu/cop0.rs`.
- The entry method consumes an already selected fetch address-error plan and
  mutates only BadVAddr, Cause ExcCode, EPC, branch-delay flag, Status.EXL, PC,
  and next PC according to the source-clear ordinary local AdEL entry path.
- Added tests for actual entry of `Unaligned`, `DirectTargetMiss`, and
  `PifResetUnavailable`, Count no-change, non-sequential PC/next PC blocking,
  EXL-already-set blocking, fetch APIs remaining non-mutating, and no
  RDRAM/SP DMEM/reservation/GPR/Cartridge mutation during entry.
- Did not change `Machine::fetch_current_cpu_instruction_word`,
  `Machine::fetch_cpu_instruction_word_at`,
  `Machine::fetch_direct_rdram_cpu_instruction_word`,
  `Machine::fetch_sp_dmem_cpu_instruction_word`, or
  `Machine::classify_cpu_instruction_fetch_target`.
- Added no CPU step, normal PC/next PC cadence, Count cadence, fetch-time
  exception conversion inside fetch APIs, fetch/decode/identify/execute loop,
  instruction execution, generic exception machinery, PIF/BIOS blob behavior,
  memory map, bus, device routing, SDL/window runtime, host shell, or C++
  source changes.

## Machine Step Fetch-Fault Action Classification Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Step fetch fault owner | `src/core/machine_cpu.cpp` `step_cpu_instruction` fetch `MachineFault` catch block | `machine.rs` `classify_step_fetch_fault_action`; `MachineStepFetchFaultAction` | Equivalent for pure action classification | Step fetch-fault action tests; source inspection | Rust classifies an already-returned fetch error; it does not fetch or step. |
| Convertible action shape | C++ selected fetch faults call `enter_local_address_error_exception` and return `kException` if the local ordinary guard allows | `MachineStepFetchFaultAction::EnterAddressError(MachineInstructionFetchAddressErrorPlan)` | Equivalent action label, no mutation | `step_fetch_fault_action_classifies_convertible_faults_as_adel_entry` | The action contains the already-sealed AdEL/code 4 plan. Entry remains a separate explicit method. |
| Rethrow action shape | C++ fetch catch rethrows faults not handled by the two selected catch arms | `MachineStepFetchFaultAction::Rethrow(MachineCpuInstructionFetchError)` | Equivalent for Rust fetch-error return shape | `step_fetch_fault_action_classifies_non_converting_faults_as_rethrow` | Rust returns a typed error instead of throwing; this is Rust-only API safety and does not add emulator truth. |
| Unaligned fetch action | `kUnalignedInstructionFetch` with `kInstructionFetch` intent selects local AdEL when allowed | `Unaligned` -> `EnterAddressError` with source `Unaligned` | Equivalent | Step fetch-fault action test | The faulting fetch address is preserved for BadVAddr. |
| Direct-target miss action | `kCpuRdramAddressRejected` with `kInstructionFetchDirectTargetMiss` intent selects local AdEL when allowed | `DirectTargetMiss` -> `EnterAddressError` with source `DirectTargetMiss` | Equivalent | Step fetch-fault action test | Direct target miss remains separate from non-direct unsupported fetch. |
| Unavailable PIF reset action | `fail_unavailable_pif_rom_reset_fetch` throws the direct-target-miss intent that step handles as AdEL when allowed | `PifResetUnavailable` -> `EnterAddressError` with source `PifResetUnavailable` | Equivalent unavailable naming only | Step fetch-fault action test; source inspection | Rust still does not fetch PIF/BIOS bytes or fake reset-vector contents. |
| Non-direct unsupported action | C++ non-direct fetch rejection uses `kInstructionFetch` intent and is not handled by the selected catch arms | `NonDirectUnsupported` -> `Rethrow` | Equivalent non-converting boundary | Step fetch-fault action test | This remains an unsupported/rethrowable fetch outcome below step. |
| Lower direct RDRAM source errors | C++ step reaches RDRAM source errors through full fetch, not lower direct helper APIs | `DirectRdram { source }` -> `Rethrow` | Rust-only API safety, no emulator truth | Step fetch-fault action test | Lower source errors stay outside C++ step-convertible fetch categories. |
| Lower SP DMEM source errors | C++ step reaches SP DMEM source errors through full fetch, not lower SP DMEM helper APIs | `SpDmem { source }` -> `Rethrow` | Rust-only API safety, no emulator truth | Step fetch-fault action test | Lower source errors stay outside C++ step-convertible fetch categories. |
| Faulting address preservation | C++ uses `fault.cpu_address()` as the AdEL BadVAddr input or rethrow fault address | `MachineStepFetchFaultAction::cpu_address()` delegates to the underlying fetch error or plan | Equivalent | Step fetch-fault action tests | The classifier preserves the same address for both action classes. |
| Width preservation | C++ instruction fetch uses width 4 in the fault text and target checks | `MachineStepFetchFaultAction::width() == 4` | Equivalent | Step fetch-fault action tests | This is instruction fetch width only, not data access width. |
| Mutation behavior | C++ classification is implicit in a catch block; mutation occurs only if entry is called | Classifier returns an action value and mutates no Machine/Cpu/COP0/PC/next PC/Count/RDRAM/SP DMEM/reservation/Cartridge state | Equivalent for pure readiness boundary | `step_fetch_fault_action_performs_no_machine_mutation` | Fetch APIs still do not call entry. |
| Step result shape | C++ has `CpuInstructionStepResult::{kStepped,kStopped,kUnsupported,kInterrupted,kException}` | No Rust `MachineStepResult` or `MachineStepError` | Not yet earned | Source inspection | Full result shape is still coupled to interrupts, ERET, execute result, rollback, Count cadence, PC cadence, unsupported instructions, and stop/trap behavior. |
| Fetch APIs | C++ fetch throws and step decides conversion/rethrow | Rust fetch APIs return `MachineCpuInstructionFetchError`; classifier is separate | Equivalent below step | Existing fetch tests and step action tests | Fetch APIs remain non-mutating and do not enter exceptions. |
| Unsupported instructions | C++ reports `kUnsupported` after identify/execute and rollback | Represented unsupported action/application exists separately; this classifier remains pure | Classifier only | Source inspection | This classifier itself does not trigger rollback or Count behavior. |
| Count cadence | C++ advances Count only after committed instructions or local ERET return, not for fetch-fault exceptions or rethrows | Represented Count cadence exists in applicators; this classifier remains pure | Classifier only | Source inspection | Normal Count cadence remains outside fetch-fault classification. |
| PC / next PC cadence | C++ normal step commits or restores PC/next PC around execute; fetch faults happen before speculative mutation | No Rust PC/next PC cadence in classifier | Not in scope | Source inspection | The classifier does not advance, restore, or stage control flow. |
| Memory map / bus / devices | C++ step uses local Machine fetch and execution helpers | No Rust map, bus, or device routing | Not in scope | Source inspection | The classifier only consumes existing fetch error values. |

### Seam 054 Audit Changes

- Added `MachineStepFetchFaultAction` and
  `classify_step_fetch_fault_action` in `machine.rs`.
- Classified source-clear step fetch-fault action for already-earned fetch
  errors: `Unaligned`, `DirectTargetMiss`, and `PifResetUnavailable` become
  `EnterAddressError(plan)`; `NonDirectUnsupported`, lower direct-RDRAM source
  errors, and lower SP DMEM source errors become `Rethrow(fetch_error)`.
- Added tests for convertible action classification, non-converting/rethrow
  preservation, faulting address and width preservation, display text, and no
  Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge
  mutation.
- Did not add `MachineStepResult`, `MachineStepError`, `Machine::step`,
  `Cpu::step`, placeholder step APIs, fetch APIs that enter exceptions, normal
  PC/next PC cadence, Count cadence, unsupported-instruction result behavior,
  instruction execution, generic exception machinery, memory map, bus, device
  routing, SDL/window runtime, host shell, or C++ source changes.

## Machine Unsupported-Instruction Step Outcome Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Step-visible unsupported result owner | `src/core/machine.hpp` `CpuInstructionStepResult::kUnsupported`; `src/core/machine_cpu.cpp` `step_cpu_instruction` | No full Rust step result type | Not yet earned | Source inspection | Full result shape remains coupled to execution, rollback, interrupts, exceptions, Count cadence, and PC cadence. |
| Unsupported execution result owner | `src/core/machine.hpp` `CpuInstructionExecutionResult::kUnsupported`; `machine_cpu.cpp` `execute_cpu_instruction` | No generic Rust execute result type; narrow SPECIAL execution helpers exist separately | Not yet earned | Source inspection | Rust does not implement generic execute or a placeholder execute result. |
| Unknown primary unsupported readiness | `identify_cpu_instruction` default returns `kUnknownPrimary`; `execute_cpu_instruction` default returns `kUnsupported`; step returns `kUnsupported` after PC/next PC restore | `MachineStepUnsupportedInstructionCategory::UnknownPrimary`; `classify_step_unsupported_instruction` | Equivalent for pure unknown identity classification | `step_unsupported_instruction_classifies_source_clear_unknown_identities` | The Rust value preserves decoded fields, raw word, and identity; it performs no rollback. |
| Unknown SPECIAL unsupported readiness | SPECIAL/funct default returns `kSpecialUnknown`; execute default returns `kUnsupported`; step returns `kUnsupported` after restore | `MachineStepUnsupportedInstructionCategory::SpecialUnknown` | Equivalent for pure unknown identity classification | Unsupported-instruction test | This is a raw identity outcome, not reserved-instruction exception behavior. |
| Unknown REGIMM unsupported readiness | REGIMM/rt default returns `kRegimmUnknown`; execute default returns `kUnsupported`; step returns `kUnsupported` after restore | `MachineStepUnsupportedInstructionCategory::RegimmUnknown` | Equivalent for pure unknown identity classification | Unsupported-instruction test | This is a raw identity outcome, not branch execution behavior. |
| Raw word preservation | C++ decoded instruction carries `raw` into execute | `MachineStepUnsupportedInstruction::raw`; `fields` | Equivalent | Unsupported-instruction tests | Rust preserves the already-decoded raw word; it does not fetch or endian-convert. |
| Decoded field preservation | C++ `DecodedCpuInstructionWord` fields are available to execute | `MachineStepUnsupportedInstruction::fields` | Equivalent for preserved proof data | Unsupported-instruction tests | Rust does not interpret operands. |
| Identity preservation | C++ unsupported unknown path begins from `CpuInstructionIdentity` | `MachineStepUnsupportedInstruction::identity` | Equivalent | Unsupported-instruction tests | Identity remains the already-sealed pure classifier output. |
| NOP / SLL behavior | C++ identifies raw zero as `kSpecialSll` and executes it, not unknown | `classify_step_unsupported_instruction` returns `None` for `SpecialSll` | Equivalent boundary | `step_unsupported_instruction_does_not_classify_implemented_or_contextual_identities` | NOP is not represented as unsupported. |
| Known implemented identity behavior | C++ implemented identities execute, may stop, branch, mutate, or fault depending on instruction | `classify_step_unsupported_instruction` returns `None` for examples such as `Addiu` | Equivalent boundary | Unsupported-instruction test | Rust does not execute or claim implemented instruction behavior. |
| Coarse COP0 unimplemented identities | C++ `kCop0` has no execute case and falls through the final `kUnsupported` default without side effects | `MachineStepUnsupportedInstructionCategory::Cop0Unimplemented` | Equivalent pure classification | `step_unsupported_instruction_classifies_source_clear_known_unimplemented_identities` | This includes source-visible non-MFC0/MTC0/ERET COP0 encodings such as TLB-style forms, but does not implement COP0/TLB behavior. |
| COP1/COP2/COP3 unimplemented identities | C++ `kCop1`, `kCop2`, and `kCop3` have no execute cases and fall through the final `kUnsupported` default without side effects | `MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented` | Equivalent pure classification | Known-unimplemented test | This names unsupported readiness only; no FPU/RSP/coprocessor execution is added. |
| Coprocessor memory unimplemented identities | C++ `kLwc1`, `kLwc2`, `kLdc1`, `kLdc2`, `kSwc1`, `kSwc2`, `kSdc1`, and `kSdc2` have no execute cases and fall through the final `kUnsupported` default without side effects | `MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented` | Equivalent pure classification | Known-unimplemented test | This is not CPU load/store semantics, memory mapping, or device routing. |
| CACHE unimplemented identity | C++ `kCache` has no execute case and falls through the final `kUnsupported` default without side effects | `MachineStepUnsupportedInstructionCategory::CacheUnimplemented` | Equivalent pure classification | Known-unimplemented test | No cache state, coherency, or cache operation behavior is represented. |
| Invalid COP0 register forms | C++ `Cop0Mfc0` supports decoded `rd` 8/9/11/12/13/14, `Cop0Mtc0` supports decoded `rd` 9/11/12/13/14, and other decoded `rd` values return `kUnsupported` before GPR/COP0 mutation | `MachineStepUnsupportedInstructionCategory::Cop0RegisterUnsupported` | Equivalent pure decoded-field classification | `step_unsupported_instruction_classifies_invalid_cop0_register_forms` | This is field validation only. Valid COP0 forms and COP0 read/write semantics remain execution-owned. |
| ERET unsupported context | C++ step checks `kCop0Eret` before speculative PC movement and returns `kUnsupported` when `local_eret_can_return` fails | Rust returns `None` for `Cop0Eret` | Source-coupled | Unsupported-instruction test; source inspection | ERET unsupported behavior depends on COP0/control-flow state, not identity alone. |
| Rollback behavior | C++ restores `cpu_pc_` and `cpu_next_pc_` before returning `kUnsupported` after execute result | `cpu/scalars.rs` crate-private `CpuControlFlowSnapshot`, `Cpu::capture_control_flow`, `Cpu::restore_control_flow` | Equivalent for pc/next_pc restore primitive only | Control-flow snapshot/restore tests; source inspection | Rust owns the source-clear control-flow primitive, but no step trigger, execute result handling, or generic rollback exists. |
| PC / next PC cadence | C++ prepares `cpu_next_pc_ = sequential_instruction_address(current_next_pc)` before execute, then restores on unsupported | No Rust PC/next PC cadence in classifier | Not in scope | Source inspection | The classifier is pure and cannot commit or roll back control flow. |
| Count cadence | C++ does not call `advance_cop0_count_after_committed_instruction` on unsupported return | Represented unsupported application restores without Count; this classifier remains pure | Classifier only | Source inspection | Count cadence remains outside unsupported classification. |
| Context-coupled unsupported identities | C++ `kCop0Eret` unsupported result depends on `local_eret_can_return`; valid `Cop0Mfc0`/`Cop0Mtc0` execute or mutate based on decoded register | Rust returns `None` for ERET and supported COP0 register forms | Equivalent blocked boundary | Implemented/contextual negative test | Runtime COP0 validation and ERET behavior remain absent. |
| Stopped/executed identities | C++ `SYSCALL`/`BREAK` return `kStopped`, `SYNC` returns `kExecuted`, and LL/SC have storage/reservation behavior | `classify_step_unsupported_instruction` returns `None`; `classify_step_stopped_instruction` owns the SYSCALL/BREAK stop-readiness subset; `classify_step_no_effect_executed_instruction` owns the SYNC no-effect executed-readiness subset | Equivalent ownership split | Implemented/contextual negative test; stopped-readiness tests; no-effect executed-readiness tests | Stop readiness and no-effect executed readiness are separate from unsupported readiness. LL/SC, load/store, reservation, side-effect execution, and cadence behavior remain absent. |
| Mutation behavior | C++ represented unknown and source-clear known-unimplemented paths return unsupported without instruction side effects | Rust classifier mutates no Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge state | Equivalent for represented subset | `step_unsupported_instruction_classification_performs_no_machine_mutation` | No rollback, cadence action, or exception entry occurs. |
| Exception behavior | C++ unsupported result is not an illegal/reserved instruction exception in this local step policy | Rust classifier enters no exception | Equivalent for represented subset | Unsupported-instruction tests; source inspection | Reserved-instruction exception behavior remains absent. |
| Generic step result machinery | C++ has a full step result enum | No Rust generic step result enum | Not yet earned | Source inspection | Adding one now would imply fake step readiness. |

### Seam 055 Audit Changes

- Added `MachineStepUnsupportedInstructionCategory`,
  `MachineStepUnsupportedInstruction`, and
  `classify_step_unsupported_instruction` in `machine.rs`.
- Classified only source-clear unknown identity categories:
  `UnknownPrimary`, `SpecialUnknown`, and `RegimmUnknown`.
- Preserved the raw instruction word, decoded fields, and
  `CpuInstructionIdentity` in the unsupported-readiness value.
- Added tests proving unknown identity classification, NOP/known/contextual
  identities are not classified, known-but-unimplemented `Cop1` remains absent,
  invalid/contextual COP0 forms remain absent, display text is stable, and no
  Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge
  mutation occurs.
- Did not add `Machine::step`, `Cpu::step`, `execute_cpu_instruction`, a
  placeholder execute API, a generic `MachineStepResult`, rollback behavior,
  normal PC/next PC cadence, Count cadence, instruction side effects, instruction
  writeback, exceptions, memory map, bus, device routing, SDL/window runtime,
  host shell, or C++ source changes.

### Seam 059 Audit Changes

- Extended `MachineStepUnsupportedInstructionCategory` and
  `classify_step_unsupported_instruction` for source-clear known identities that
  C++ returns as `kUnsupported` without side effects.
- Represented coarse `Cop0`, `Cop1`, `Cop2`, `Cop3`, `Cache`, `Lwc1`, `Lwc2`,
  `Ldc1`, `Ldc2`, `Swc1`, `Swc2`, `Sdc1`, and `Sdc2` identities.
- Represented invalid COP0 MFC0/MTC0 decoded `rd` forms using the same supported
  register set visible in C++ execute: MFC0 supports 8/9/11/12/13/14, while
  MTC0 supports 9/11/12/13/14.
- Kept ERET unsupported context, valid COP0 MFC0/MTC0 semantics, stopped
  SYSCALL/BREAK readiness, executed SYNC behavior, LL/SC, implemented
  arithmetic/branch/load/store identities, rollback trigger, cadence commit,
  Count mutation, and full step result machinery out of the unsupported
  classifier.
- Added tests for every newly represented known identity category, invalid COP0
  forms, unchanged negative boundaries, and no Machine/Cpu/COP0/PC/next
  PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge mutation.
- Added no `Machine::step`, `Cpu::step`, `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, rollback trigger, normal
  PC/next PC commit, Count mutation, instruction side effects, reserved
  instruction exception behavior, memory map, bus, device routing, SDL/window
  runtime, host shell, or C++ source changes.

## Machine Stopped-Instruction Step Outcome Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Step-visible stopped result owner | `src/core/machine.hpp` `CpuInstructionStepResult::kStopped`; `src/core/machine_cpu.cpp` `step_cpu_instruction` | No full Rust step result type | Not yet earned | Source inspection | Full step result shape remains coupled to execute, cadence, Count, interrupts, exceptions, unsupported rollback, and host-facing callers. |
| Stopped execution result owner | `src/core/machine.hpp` `CpuInstructionExecutionResult::kStopped`; `machine_cpu.cpp` `execute_cpu_instruction` | No generic Rust execute result type; narrow SPECIAL execution helpers exist separately | Not yet earned | Source inspection | Rust does not implement generic execute or a placeholder execute result. |
| SYSCALL stop readiness | C++ `execute_cpu_instruction` case `kSpecialSyscall` returns `CpuInstructionExecutionResult::kStopped` directly | `MachineStepStoppedInstructionCategory::Syscall`; `classify_step_stopped_instruction` | Equivalent pure classification | `step_stopped_instruction_classifies_source_clear_stopped_identities` | The classifier preserves raw fields and identity. It does not execute SYSCALL or enter a syscall exception. |
| BREAK stop readiness | C++ `execute_cpu_instruction` case `kSpecialBreak` returns `CpuInstructionExecutionResult::kStopped` directly | `MachineStepStoppedInstructionCategory::Break`; `classify_step_stopped_instruction` | Equivalent pure classification | Stopped-readiness test | The classifier preserves raw fields and identity. It does not execute BREAK or enter a breakpoint exception. |
| SYNC boundary | C++ `execute_cpu_instruction` case `kSpecialSync` returns `CpuInstructionExecutionResult::kExecuted` | `classify_step_stopped_instruction` returns `None` for `SpecialSync`; `classify_step_no_effect_executed_instruction` owns the SYNC executed-readiness subset | Equivalent ownership split | Stopped negative test; no-effect executed-readiness test | SYNC is not a stopped instruction. |
| Unknown and unsupported identities | C++ unknown/source-clear known-unimplemented paths return `kUnsupported`, not `kStopped` | Stopped classifier returns `None`; unsupported classifier owns those identities | Equivalent ownership split | Negative stopped-readiness test; unsupported-readiness tests | Unknown and known-unimplemented unsupported readiness remains separate from stop readiness. |
| NOP / implemented identity boundary | C++ raw zero is `SLL`/NOP-like executed behavior; implemented identities execute or may fault/branch/stop based on their own cases | Stopped classifier returns `None` for `SpecialSll` and `Addiu` examples | Equivalent negative boundary | Negative stopped-readiness test | Rust does not infer stop readiness from implemented identities. |
| Direct stopped path side effects | C++ `kSpecialSyscall` and `kSpecialBreak` return `kStopped` before instruction-side-effect writes in their cases | Rust classifier mutates no Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge state | Equivalent pure readiness | `step_stopped_instruction_classification_performs_no_machine_mutation` | No rollback, cadence action, Count mutation, exception entry, host runtime stop, or instruction execution occurs. |
| Step propagation | C++ after execute commits `cpu_pc_ = current_next_pc`, advances Count, then returns `CpuInstructionStepResult::kStopped` when execution result is `kStopped` | `MachineStepCadenceSource::StoppedInstruction` already maps to `CommitStaged` + `Advance`; stopped classifier does not mutate | Equivalent as separate pure plans only | Cadence plan tests; source inspection | Cadence commit and Count mutation remain separate and non-mutating in Rust. |
| Exception behavior | C++ local policy treats `SYSCALL`/`BREAK` stop as `kStopped`, not as syscall/break COP0 exception entry in the current source | Rust classifier enters no exception | Equivalent for represented stop readiness | Stopped-readiness no-mutation test; source inspection | Syscall/break exception behavior remains absent. |
| Host/runtime stop policy | C++ core `kStopped` is a local step result; host callers may observe it, but no host/runtime policy participates in `execute_cpu_instruction` classification | No Rust host stop/runtime behavior | Equivalent boundary | Source inspection | The Rust value is machine-core readiness only. |
| Generic step result machinery | C++ has a full step result enum | No Rust generic step result enum | Not earned | Source inspection | Adding one now would imply fake step readiness. |

### Seam 060 Audit Changes

- Added `MachineStepStoppedInstructionCategory`,
  `MachineStepStoppedInstruction`, and `classify_step_stopped_instruction` in
  `machine.rs`.
- Classified only source-clear stopped identities: `SpecialSyscall` as
  `Syscall` and `SpecialBreak` as `Break`.
- Preserved the raw instruction word, decoded fields, and
  `CpuInstructionIdentity` in the stopped-readiness value.
- Kept `SpecialSync` out of stopped readiness because C++ returns `kExecuted`;
  seam 061 owns the no-effect executed-readiness boundary for SYNC.
- Added tests proving stopped classification, negative boundaries against
  executed/unsupported/implemented identities, stable display text, and no
  Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge
  mutation.
- Added no `Machine::step`, `Cpu::step`, `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, normal PC/next PC commit,
  Count mutation, syscall exception behavior, breakpoint exception behavior,
  host stop/runtime behavior, memory map, bus, device routing, SDL/window
  runtime, host shell, or C++ source changes.

## Machine No-Effect Executed-Instruction Step Outcome Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Step-visible executed result owner | `src/core/machine_cpu.cpp` `step_cpu_instruction` returns `CpuInstructionStepResult::kStepped` for `kExecuted` after normal commit and Count advance | No full Rust step result type | Not yet earned | Source inspection | Full step result shape remains coupled to execute, cadence, Count, interrupts, exceptions, unsupported rollback, and stop handling. |
| Executed execution result owner | `src/core/machine.hpp` `CpuInstructionExecutionResult::kExecuted`; `machine_cpu.cpp` `execute_cpu_instruction` | No generic Rust execute result type; narrow SPECIAL execution helpers exist separately | Not yet earned for generic execute results | Source inspection | Rust does not implement generic execute or a placeholder execute result. The SPECIAL execution helpers return narrow readiness values, not `CpuInstructionExecutionResult`. |
| SYNC no-effect executed readiness | C++ `execute_cpu_instruction` case `kSpecialSync` returns `CpuInstructionExecutionResult::kExecuted` directly | `MachineStepNoEffectExecutedInstructionCategory::Sync`; `classify_step_no_effect_executed_instruction` | Equivalent pure classification | `step_no_effect_executed_instruction_classifies_source_clear_sync_identity` | The classifier preserves raw fields and identity. It does not execute SYNC or model any memory-ordering side effect. |
| SYSCALL/BREAK boundary | C++ `kSpecialSyscall` and `kSpecialBreak` return `kStopped`, not `kExecuted` | No-effect executed classifier returns `None`; stopped classifier owns those identities | Equivalent ownership split | No-effect executed negative test; stopped-readiness tests | Stop readiness remains separate. |
| Unknown and unsupported identities | C++ unknown/source-clear known-unimplemented paths return `kUnsupported`, not `kExecuted` | No-effect executed classifier returns `None`; unsupported classifier owns those identities | Equivalent ownership split | No-effect executed negative test; unsupported-readiness tests | Unknown and known-unimplemented unsupported readiness remains separate from executed readiness. |
| NOP / SLL boundary | C++ identifies raw zero as `kSpecialSll`; the `kSpecialSll` execute case reads a GPR word, calls `write_cpu_gpr_word_sign_extended_result(instruction.rd, value)`, then returns `kExecuted` | No-effect executed classifier returns `None` for `SpecialSll` | Equivalent blocked boundary | No-effect executed negative test; source inspection | Even if register zero discards writes, this path is normal shift/writeback execution and is not a no-effect direct `kExecuted` identity. |
| Other implemented identities | C++ implemented identities may read/write GPR/COP0/memory, branch, fault, or return other execution results before `kExecuted` | No-effect executed classifier returns `None` for examples such as `Addiu` | Equivalent boundary | No-effect executed negative test | Rust does not execute or claim side-effect instruction behavior. |
| Direct no-effect path side effects | C++ `kSpecialSync` returns `kExecuted` directly with no state mutation before the return | Rust classifier mutates no Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge state | Equivalent pure readiness | `step_no_effect_executed_instruction_classification_performs_no_machine_mutation` | No rollback, cadence action, Count mutation, exception entry, host runtime behavior, or instruction execution occurs. |
| Step propagation | C++ after execute commits `cpu_pc_ = current_next_pc`, advances Count, and returns `kStepped` for `kExecuted` | `MachineStepCadenceSource::CommittedInstruction` already maps to `CommitStaged` + `Advance`; no-effect executed classifier does not mutate | Equivalent as separate pure plans only | Cadence plan tests; source inspection | Cadence commit and Count mutation remain separate and non-mutating in Rust. |
| Exception behavior | C++ direct `SYNC` path enters no exception before returning `kExecuted` | Rust classifier enters no exception | Equivalent for represented no-effect readiness | No-effect executed no-mutation test; source inspection | Generic exception behavior remains absent. |
| Generic step result machinery | C++ has a full step result enum | No Rust generic step result enum | Not earned | Source inspection | Adding one now would imply fake step readiness. |

### Seam 061 Audit Changes

- Added `MachineStepNoEffectExecutedInstructionCategory`,
  `MachineStepNoEffectExecutedInstruction`, and
  `classify_step_no_effect_executed_instruction` in `machine.rs`.
- Classified only the source-clear no-effect executed identity:
  `SpecialSync` as `Sync`.
- Preserved the raw instruction word, decoded fields, and
  `CpuInstructionIdentity` in the no-effect executed-readiness value.
- Kept `SpecialSll`/NOP out of no-effect executed readiness because C++
  reaches `kExecuted` through the normal SLL read/writeback path, not a direct
  no-effect return. Seam 064 owns that path as narrow SPECIAL shift execution.
- Added tests proving SYNC classification, negative boundaries against
  stopped/unsupported/writeback/implemented identities, stable display text,
  and no Machine/Cpu/COP0/PC/next PC/Count/GPR/RDRAM/SP DMEM/reservation/
  Cartridge mutation.
- Added no `Machine::step`, `Cpu::step`, `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, normal PC/next PC commit,
  Count mutation, branch/link/delay-slot behavior, arithmetic/logical/shift
  execution, load/store behavior, memory map, bus, device routing, SDL/window
  runtime, host shell, or C++ source changes.

## CPU SPECIAL Shift GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` SPECIAL shift cases | `cpu/instruction.rs` `Cpu::execute_special_shift_instruction` | Equivalent for the represented narrow family only | SPECIAL shift execution tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction` or step API. |
| Represented identities | `kSpecialSll`, `kSpecialSrl`, `kSpecialSra`, `kSpecialSllv`, `kSpecialSrlv`, `kSpecialSrav` return `CpuInstructionExecutionResult::kExecuted` after GPR writeback | `CpuInstructionIdentity::SpecialSll`, `SpecialSrl`, `SpecialSra`, `SpecialSllv`, `SpecialSrlv`, `SpecialSrav` | Equivalent | Per-identity shift tests | SLL, SRL, SRA, SLLV, SRLV, and SRAV are represented. Seam 065 extends the same helper to the source-clear 64-bit SPECIAL shift family. |
| SLL behavior | C++ computes `read_cpu_gpr_word(rt) << sa` and sign-extends the word result to `rd` | Rust computes the same `u32` left shift and sign-extends before `set_gpr(rd, ...)` | Equivalent | `special_sll_writes_sign_extended_word_result` | Raw zero/NOP is still `SpecialSll`; its destination is register zero, so the sealed zero-register rule ignores the write. |
| SRL behavior | C++ computes logical `read_cpu_gpr_word(rt) >> sa` and sign-extends the word result | Rust computes `u32 >> sa` and sign-extends before writeback | Equivalent | SRL shift test | Logical right shift is represented only for the 32-bit word-result SPECIAL form. |
| SRA behavior | C++ uses `arithmetic_shift_right_u32(read_cpu_gpr_word(rt), sa)` | Rust shifts `(value as i32) >> sa` and returns the resulting `u32` bits before sign extension | Equivalent | SRA shift test | Arithmetic sign-fill is represented for the 32-bit word value. |
| Variable shift amount | C++ `variable_shift_amount_u32` returns `read_cpu_gpr_word(rs) & 0x1f` | Rust masks the source `rs` word with `0x1f` | Equivalent | Variable shift tests | SLLV, SRLV, and SRAV use only the low five bits of `rs`. |
| Read-before-write aliases | C++ computes source words before calling `write_cpu_gpr_word_sign_extended_result(rd, value)` | Rust reads `rt` and, for variable shifts, `rs` before `set_gpr(rd, ...)` | Equivalent | Alias tests | Aliased `rs`/`rt`/`rd` preserve source values before destination writeback. |
| Zero-register write behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing | Rust uses sealed `Cpu::set_gpr`, which ignores writes to register zero | Equivalent | NOP and zero-register tests | This is the existing GPR storage rule, now exercised by the narrow shift helper. |
| Result width and sign extension | C++ writes through `write_cpu_gpr_word_sign_extended_result` | Rust sign-extends the computed `u32` word to the 64-bit CPU register value | Equivalent | Sign-extension shift tests | This is not a general sign-extension instruction seam. |
| Rejected identities | C++ reaches these cases only after identity dispatch | Rust helper rejects non-shift identities with `CpuSpecialShiftExecutionError::UnsupportedIdentity` | Rust API safety | Rejection test | The helper requires the caller to provide a matching shift identity and decoded fields; it does not fetch/decode/identify internally. |
| Mutation boundary | SPECIAL shift cases write only the destination GPR and return `kExecuted` | Rust helper mutates only the destination GPR through sealed GPR write semantics | Equivalent for represented subset | CPU and Machine preservation tests | No PC, next PC, Count, timer-pending, COP0, RDRAM, SP DMEM, reservation, Cartridge, fetch, decode, identify, exception entry, cadence commit, branch behavior, memory map, bus, or step behavior is added. |
| Step/cadence boundary | C++ step later commits PC and advances Count after `kExecuted` | Rust helper does not call commit or Count primitives | Equivalent separation | Cadence/commit/Count tests remain separate | The returned executed-readiness value is not a step result and does not wire outcome cadence. |

### Seam 064 Audit Changes

- Added crate-private `CpuSpecialShiftExecutedInstruction`,
  `CpuSpecialShiftExecutionError`, and
  `Cpu::execute_special_shift_instruction` in `cpu/instruction.rs`.
- Mirrored the source-clear 32-bit SPECIAL shift execution family:
  `SLL`, `SRL`, `SRA`, `SLLV`, `SRLV`, and `SRAV`.
- Preserved C++ read-before-write behavior by reading `rt` and, for variable
  shifts, `rs` before destination writeback.
- Preserved C++ zero-register behavior by writing through the sealed
  `Cpu::set_gpr` rule; raw zero/NOP executes as `SpecialSll` and leaves GPRs
  unchanged because the destination is register zero.
- Preserved C++ word-result sign extension and variable shift low-five-bit
  masking. Arithmetic right shift is represented for SRA/SRAV only.
- Added CPU tests for fixed shifts, variable shifts, aliasing, zero-register
  writes, raw-zero NOP behavior, sign extension, identity rejection, and a
  Machine preservation test proving no PC/next PC/Count/COP0/RDRAM/SP DMEM/
  reservation/Cartridge/fetch/decode/identify/exception/cadence/step effects.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, PC/next PC commit call,
  Count advancement call, branch/link/delay-slot behavior, load/store behavior,
  arithmetic overflow behavior, memory map, bus, device routing, SDL/window
  runtime, host shell, or C++ source changes.

## CPU SPECIAL 64-Bit Shift GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` SPECIAL 64-bit shift cases | `cpu/instruction.rs` `Cpu::execute_special_shift_instruction` | Equivalent for the represented narrow family only | 64-bit SPECIAL shift execution tests; source inspection | Rust extends the existing crate-private CPU-owned helper. It does not add generic `execute_cpu_instruction`, `Machine::step`, or `Cpu::step`. |
| Fixed 64-bit shift identities | C++ cases `kSpecialDsll`, `kSpecialDsrl`, `kSpecialDsra`, `kSpecialDsll32`, `kSpecialDsrl32`, and `kSpecialDsra32` return `CpuInstructionExecutionResult::kExecuted` after GPR writeback | `CpuInstructionIdentity::SpecialDsll`, `SpecialDsrl`, `SpecialDsra`, `SpecialDsll32`, `SpecialDsrl32`, and `SpecialDsra32` | Equivalent | Fixed 64-bit shift tests | DSLL, DSRL, DSRA, DSLL32, DSRL32, and DSRA32 are represented. |
| Variable 64-bit shift identities | C++ cases `kSpecialDsllv`, `kSpecialDsrlv`, and `kSpecialDsrav` return `CpuInstructionExecutionResult::kExecuted` after GPR writeback | `CpuInstructionIdentity::SpecialDsllv`, `SpecialDsrlv`, and `SpecialDsrav` | Equivalent | Variable 64-bit shift tests | DSLLV, DSRLV, and DSRAV are represented. |
| Fixed shift amount | C++ DSLL/DSRL/DSRA use `instruction.sa` directly | Rust uses the decoded `sa` field directly | Equivalent | Fixed shift amount boundary tests | The represented amount range is the five-bit decoded field. |
| `*32` shift amount | C++ DSLL32/DSRL32/DSRA32 compute `sa = instruction.sa + 32` | Rust computes `instruction.sa() + 32` | Equivalent | `*32` boundary tests | Shift amounts 32 through 63 are represented. |
| Variable shift amount | C++ `variable_shift_amount_cpu_value` masks the full source value with `0x3f` | Rust masks the full `rs` value with `0x3f` | Equivalent | Variable mask tests | DSLLV, DSRLV, and DSRAV use only the low six bits of `rs`. |
| Full-width logical shifts | C++ DSLL/DSRL/DSLL32/DSRL32/DSLLV/DSRLV operate on `CpuRegisterValue` | Rust reads and shifts full `u64` GPR values | Equivalent | Logical shift tests | No 32-bit truncation or sign extension is applied to represented 64-bit logical results. |
| Full-width arithmetic right shifts | C++ `arithmetic_shift_right_cpu_value` sign-fills from bit 63 | Rust shifts `(value as i64) >> sa` and writes the resulting `u64` bits | Equivalent | DSRA/DSRA32/DSRAV tests | DSRA, DSRA32, and DSRAV use signed 64-bit arithmetic right shift behavior. |
| Read-before-write aliases | C++ reads `rt` and, for variable shifts, `rs` before `write_cpu_gpr_value(rd, value)` | Rust reads `rt` and, for variable shifts, `rs` before `set_gpr(rd, ...)` | Equivalent | Fixed and variable alias tests | Aliased `rs`/`rt`/`rd` preserve source values before destination writeback. |
| Zero-register write behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing | Rust uses sealed `Cpu::set_gpr`, which ignores writes to register zero | Equivalent | Zero-register tests | This is the existing GPR storage rule exercised by the 64-bit shift helper. |
| Rejected identities | C++ reaches these cases only after identity dispatch | Rust helper rejects non-shift identities with `CpuSpecialShiftExecutionError::UnsupportedIdentity`; 32-bit shifts remain valid for this same helper | Rust API safety | Rejection test | The helper requires the caller to provide matching decoded fields and identity. It does not fetch/decode/identify internally. |
| Mutation boundary | SPECIAL 64-bit shift cases write only the destination GPR and return `kExecuted` | Rust helper mutates only the destination GPR through sealed GPR write semantics | Equivalent | CPU and Machine preservation tests | No PC, next PC, Count, timer-pending, COP0, RDRAM, SP DMEM, reservation, Cartridge, fetch, decode, identify, exception entry, cadence commit, branch behavior, memory map, bus, or step behavior is added. |
| Step/cadence boundary | C++ step later commits PC and advances Count after `kExecuted` | Rust helper does not call commit or Count primitives | Equivalent separation | Cadence/commit/Count tests remain separate | The returned executed-readiness value is not a step result and does not wire outcome cadence. |

### Seam 065 Audit Changes

- Extended the existing crate-private `Cpu::execute_special_shift_instruction`
  helper in `cpu/instruction.rs`; no Machine-owned or public generic execute API
  was added.
- Mirrored the source-clear 64-bit SPECIAL shift execution family:
  `DSLL`, `DSRL`, `DSRA`, `DSLL32`, `DSRL32`, `DSRA32`, `DSLLV`, `DSRLV`, and
  `DSRAV`.
- Preserved C++ read-before-write behavior by reading `rt` and, for variable
  shifts, `rs` before destination writeback.
- Preserved C++ zero-register behavior by writing through the sealed
  `Cpu::set_gpr` rule.
- Preserved C++ fixed shift amounts, `sa + 32` shift amounts, variable
  low-six-bit masking, full-width logical shifts, and signed 64-bit arithmetic
  right shifts.
- Added CPU tests for fixed shifts, `*32` shifts, variable shifts, boundary
  amounts, aliasing, zero-register writes, and full-width arithmetic sign-fill.
- Added a Machine preservation test proving no PC/next PC/Count/COP0/RDRAM/
  SP DMEM/reservation/Cartridge/fetch/decode/identify/exception/cadence/step
  effects.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, PC/next PC commit call,
  Count advancement call, branch/link/delay-slot behavior, load/store behavior,
  arithmetic overflow behavior, memory map, bus, device routing, SDL/window
  runtime, host shell, or C++ source changes.

## CPU SPECIAL Bitwise Logical GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` SPECIAL logical cases | `cpu/instruction.rs` `Cpu::execute_special_bitwise_logical_instruction` | Equivalent for the represented narrow family only | SPECIAL bitwise logical execution tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, or `Cpu::step`. |
| Represented logical identities | C++ cases `kSpecialAnd`, `kSpecialOr`, `kSpecialXor`, and `kSpecialNor` return `CpuInstructionExecutionResult::kExecuted` after GPR writeback | `CpuInstructionIdentity::SpecialAnd`, `SpecialOr`, `SpecialXor`, and `SpecialNor` | Equivalent | Per-identity logical tests | AND, OR, XOR, and NOR are represented. No immediate logical identities are included in this seam. |
| AND behavior | C++ computes `read_cpu_gpr_value(rs) & read_cpu_gpr_value(rt)` and writes `rd` | Rust computes the same full-width `u64` bitwise AND before `set_gpr(rd, ...)` | Equivalent | AND logical test | This is register-register SPECIAL AND only, not ANDI. |
| OR behavior | C++ computes `read_cpu_gpr_value(rs) \| read_cpu_gpr_value(rt)` and writes `rd` | Rust computes the same full-width `u64` bitwise OR before `set_gpr(rd, ...)` | Equivalent | OR logical test | This is register-register SPECIAL OR only, not ORI. |
| XOR behavior | C++ computes `read_cpu_gpr_value(rs) ^ read_cpu_gpr_value(rt)` and writes `rd` | Rust computes the same full-width `u64` bitwise XOR before `set_gpr(rd, ...)` | Equivalent | XOR logical test | This is register-register SPECIAL XOR only, not XORI. |
| NOR behavior | C++ computes `~(read_cpu_gpr_value(rs) \| read_cpu_gpr_value(rt))` and writes `rd` | Rust computes `!(rs \| rt)` over full `u64` values | Equivalent | NOR logical test | The result width is the full CPU register value. |
| Full-width operands and results | C++ uses `CpuRegisterValue` reads and writes | Rust uses full `u64` GPR reads and writes | Equivalent | High-bit pattern tests | No 32-bit truncation, sign extension, or zero extension is applied by this helper. |
| Read-before-write aliases | C++ computes source values before calling `write_cpu_gpr_value(rd, value)` | Rust reads `rs` and `rt` before `set_gpr(rd, ...)` | Equivalent | Alias tests | Aliased `rs`/`rt`/`rd` preserve source values before destination writeback. |
| Zero-register source behavior | C++ `read_cpu_gpr_value(0)` returns zero | Rust sealed `Cpu::gpr(0)` returns zero through `read_gpr_value` | Equivalent | Zero-source test | Register zero can be used as either source without mutation. |
| Zero-register write behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing | Rust uses sealed `Cpu::set_gpr`, which ignores writes to register zero | Equivalent | Zero-destination test | This is the existing GPR storage rule exercised by the logical helper. |
| Rejected identities | C++ reaches these cases only after identity dispatch | Rust helper rejects non-logical identities with `CpuSpecialBitwiseLogicalExecutionError::UnsupportedIdentity` | Rust API safety | Rejection test | The helper requires the caller to provide matching decoded fields and identity. It does not fetch/decode/identify internally. |
| Mutation boundary | SPECIAL logical cases write only the destination GPR and return `kExecuted` | Rust helper mutates only the destination GPR through sealed GPR write semantics | Equivalent | CPU and Machine preservation tests | No PC, next PC, Count, timer-pending, COP0, RDRAM, SP DMEM, reservation, Cartridge, fetch, decode, identify, exception entry, cadence commit, branch behavior, memory map, bus, or step behavior is added. |
| Immediate logical separation | C++ has separate primary-opcode execution cases for ANDI/ORI/XORI and LUI | ANDI/ORI/XORI are represented by the separate immediate bitwise logical helper; LUI remains absent | Equivalent separation | Source inspection; immediate bitwise logical tests | SPECIAL register-register logical behavior remains independent from immediate logical behavior. |
| Step/cadence boundary | C++ step later commits PC and advances Count after `kExecuted` | Rust helper does not call commit or Count primitives | Equivalent separation | Cadence/commit/Count tests remain separate | The returned executed-readiness value is not a step result and does not wire outcome cadence. |

### Seam 066 Audit Changes

- Added crate-private `CpuSpecialBitwiseLogicalExecutedInstruction`,
  `CpuSpecialBitwiseLogicalExecutionError`, and
  `Cpu::execute_special_bitwise_logical_instruction` in `cpu/instruction.rs`.
- Mirrored the source-clear SPECIAL register-register logical execution family:
  `AND`, `OR`, `XOR`, and `NOR`.
- Preserved C++ read-before-write behavior by reading `rs` and `rt` before
  destination writeback.
- Preserved C++ zero-register source and destination behavior through the
  sealed GPR read/write rules.
- Preserved C++ full-width `CpuRegisterValue` bitwise behavior, including NOR
  over the full 64-bit result width.
- Added CPU tests for all four identities, high-bit/full-width patterns,
  aliasing, register-zero source/destination behavior, and identity rejection.
- Added a Machine preservation test proving no PC/next PC/Count/COP0/RDRAM/
  SP DMEM/reservation/Cartridge/fetch/decode/identify/exception/cadence/step
  effects.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, PC/next PC commit call,
  Count advancement call, branch/link/delay-slot behavior, immediate logical
  instruction behavior, load/store behavior, arithmetic overflow behavior,
  memory map, bus, device routing, SDL/window runtime, host shell, or C++ source
  changes.

## CPU SPECIAL HI/LO Transfer Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` SPECIAL HI/LO transfer cases | `cpu/instruction.rs` `Cpu::execute_special_hi_lo_transfer_instruction` | Equivalent for the represented narrow family only | SPECIAL HI/LO transfer execution tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, or `Cpu::step`. |
| Represented HI/LO transfer identities | C++ cases `kSpecialMfhi`, `kSpecialMthi`, `kSpecialMflo`, and `kSpecialMtlo` return `CpuInstructionExecutionResult::kExecuted` after scalar/GPR transfer | `CpuInstructionIdentity::SpecialMfhi`, `SpecialMthi`, `SpecialMflo`, and `SpecialMtlo` | Equivalent | Per-identity transfer tests | MFHI, MTHI, MFLO, and MTLO are represented. MULT/DIV and 64-bit multiply/divide identities remain separate. |
| MFHI behavior | C++ writes `rd` from `cpu_hi()` | Rust reads `Cpu::hi()` and writes `rd` through `set_gpr` | Equivalent | MFHI transfer test | Existing zero-register write semantics apply when `rd` is register zero. |
| MFLO behavior | C++ writes `rd` from `cpu_lo()` | Rust reads `Cpu::lo()` and writes `rd` through `set_gpr` | Equivalent | MFLO transfer test | Existing zero-register write semantics apply when `rd` is register zero. |
| MTHI behavior | C++ writes HI from `read_cpu_gpr_value(rs)` | Rust reads full-width `rs` through `read_gpr_value` and writes HI through `stage_hi` | Equivalent | MTHI transfer test | The source GPR is read before HI mutation and remains unchanged. |
| MTLO behavior | C++ writes LO from `read_cpu_gpr_value(rs)` | Rust reads full-width `rs` through `read_gpr_value` and writes LO through `stage_lo` | Equivalent | MTLO transfer test | The source GPR is read before LO mutation and remains unchanged. |
| Full-width values | C++ uses `CpuRegisterValue` for HI, LO, and GPR transfers | Rust uses `u64` HI, LO, and GPR values | Equivalent | High-bit/full-width transfer tests | No 32-bit truncation, sign extension, or zero extension is applied by this helper. |
| Zero-register write behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing | Rust uses sealed `Cpu::set_gpr`, which ignores writes to register zero | Equivalent | MFHI/MFLO zero-destination tests | This applies only to MFHI/MFLO GPR writeback. MTHI/MTLO read register zero as zero if selected as `rs`. |
| HI/LO isolation | C++ MFHI/MFLO do not write HI/LO; MTHI writes only HI; MTLO writes only LO | Rust mirrors the same scalar boundary | Equivalent | Transfer and Machine preservation tests | HI writes do not change LO, LO writes do not change HI, and MFHI/MFLO do not change either scalar. |
| Rejected identities | C++ reaches these cases only after identity dispatch | Rust helper rejects non-transfer identities with `CpuSpecialHiLoTransferExecutionError::UnsupportedIdentity` | Rust API safety | Rejection test | The helper requires the caller to provide matching decoded fields and identity. It does not fetch/decode/identify internally. |
| Multiply/divide separation | C++ MULT/MULTU/DIV/DIVU/DMULT/DMULTU/DDIV/DDIVU are adjacent but distinct cases that compute HI/LO results | No Rust multiply/divide helper | Intentionally absent | Source inspection | HI/LO transfer does not imply HI/LO arithmetic execution. |
| Mutation boundary | SPECIAL HI/LO transfer cases mutate only `rd`, HI, or LO according to identity and return `kExecuted` | Rust helper mutates only the selected GPR or scalar | Equivalent | CPU and Machine preservation tests | No PC, next PC, Count, timer-pending, COP0, RDRAM, SP DMEM, reservation, Cartridge, fetch, decode, identify, exception entry, cadence commit, branch behavior, memory map, bus, or step behavior is added. |
| Step/cadence boundary | C++ step later commits PC and advances Count after `kExecuted` | Rust helper does not call commit or Count primitives | Equivalent separation | Cadence/commit/Count tests remain separate | The returned executed-readiness value is not a step result and does not wire outcome cadence. |

### Seam 067 Audit Changes

- Added crate-private `CpuSpecialHiLoTransferExecutedInstruction`,
  `CpuSpecialHiLoTransferExecutionError`, and
  `Cpu::execute_special_hi_lo_transfer_instruction` in `cpu/instruction.rs`.
- Mirrored the source-clear SPECIAL HI/LO transfer family: `MFHI`, `MTHI`,
  `MFLO`, and `MTLO`.
- Preserved C++ full-width `CpuRegisterValue` movement between HI/LO and GPR
  state.
- Preserved C++ zero-register write behavior for MFHI/MFLO through the sealed
  GPR write rule.
- Preserved C++ scalar isolation: MFHI/MFLO do not mutate HI/LO, MTHI mutates
  HI only, and MTLO mutates LO only.
- Added CPU tests for all four identities, full-width values, zero-register
  writeback, scalar isolation, GPR preservation, and identity rejection.
- Added a Machine preservation test proving no PC/next PC/Count/COP0/RDRAM/
  SP DMEM/reservation/Cartridge/fetch/decode/identify/exception/cadence/step
  effects.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, multiply/divide behavior,
  PC/next PC commit call, Count advancement call, branch/link/delay-slot
  behavior, load/store behavior, arithmetic overflow behavior, memory map, bus,
  device routing, SDL/window runtime, host shell, or C++ source changes.

## CPU SPECIAL Non-Trapping Integer GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` SPECIAL non-trapping integer cases | `cpu/instruction.rs` `Cpu::execute_special_non_trapping_integer_instruction` | Equivalent for the represented narrow family only | SPECIAL non-trapping integer execution tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, or `Cpu::step`. |
| Represented identities | C++ cases `kSpecialAddu`, `kSpecialSubu`, `kSpecialDaddu`, `kSpecialDsubu`, `kSpecialSlt`, and `kSpecialSltu` return `CpuInstructionExecutionResult::kExecuted` after GPR writeback | `CpuInstructionIdentity::SpecialAddu`, `SpecialSubu`, `SpecialDaddu`, `SpecialDsubu`, `SpecialSlt`, and `SpecialSltu` | Equivalent | Per-identity integer tests | ADDU, SUBU, DADDU, DSUBU, SLT, and SLTU are represented. ADD, SUB, DADD, and DSUB remain separate because they own signed overflow exception behavior. |
| ADDU behavior | C++ reads `rs` and `rt` as low 32-bit words, adds them with unsigned word arithmetic, and writes the sign-extended word result | Rust reads both full GPR values before writeback, computes `(rs as u32).wrapping_add(rt as u32)`, sign-extends the word result, and writes `rd` | Equivalent | ADDU wrapping/sign-extension tests | No overflow exception is checked or entered. |
| SUBU behavior | C++ reads `rs` and `rt` as low 32-bit words, subtracts them with unsigned word arithmetic, and writes the sign-extended word result | Rust reads both full GPR values before writeback, computes `(rs as u32).wrapping_sub(rt as u32)`, sign-extends the word result, and writes `rd` | Equivalent | SUBU wrapping/sign-extension tests | No overflow exception is checked or entered. |
| DADDU behavior | C++ adds full-width `CpuRegisterValue` operands and writes the full-width result | Rust uses `u64::wrapping_add` over full GPR values | Equivalent | DADDU wrapping tests | Full 64-bit wrapping is preserved. |
| DSUBU behavior | C++ subtracts full-width `CpuRegisterValue` operands and writes the full-width result | Rust uses `u64::wrapping_sub` over full GPR values | Equivalent | DSUBU wrapping tests | Full 64-bit wrapping is preserved. |
| SLT behavior | C++ compares full-width `CpuRegisterValue` operands through signed `std::int64_t` interpretation and writes `1` or `0` | Rust compares `(rs as i64) < (rt as i64)` and writes `1` or `0` | Equivalent | SLT signed high-bit tests | The helper adds comparison/writeback only, not branch or exception behavior. |
| SLTU behavior | C++ compares full-width `CpuRegisterValue` operands as unsigned values and writes `1` or `0` | Rust compares `rs < rt` as `u64` and writes `1` or `0` | Equivalent | SLTU unsigned high-bit tests | The helper adds comparison/writeback only, not branch or exception behavior. |
| Read-before-write aliases | C++ helper calls read both source values before destination writeback in each case | Rust stores source values in locals before `set_gpr` | Equivalent | Alias tests with `rs == rd` and `rt == rd` | Source/destination register aliases preserve source values exactly. |
| Zero-register source and destination behavior | C++ register zero reads as zero and `write_cpu_gpr_value(0, value)` returns without storing | Rust uses sealed `Cpu::gpr`/`set_gpr` behavior | Equivalent | Zero-source and zero-destination tests | Writes to register zero are ignored and reads from register zero source zero. |
| Rejected identities | C++ reaches these cases only after identity dispatch; trapping arithmetic cases are distinct | Rust helper rejects non-represented identities with `CpuSpecialNonTrappingIntegerExecutionError::UnsupportedIdentity` | Rust API safety | Rejection test with `SpecialAdd` | The helper requires the caller to provide matching decoded fields and identity. It does not fetch/decode/identify internally. |
| Overflow exception separation | C++ `kSpecialAdd`, `kSpecialSub`, `kSpecialDadd`, and `kSpecialDsub` perform signed overflow checks and call overflow exception entry on failure | Seam 068 excluded trapping arithmetic; seam 069 later adds the narrow trapping arithmetic helper and overflow entry | Preserved historical boundary; current owner is seam 069 | Source inspection | Non-trapping ADDU/SUBU/DADDU/DSUBU remain separate from trapping ADD/SUB/DADD/DSUB and overflow exception entry. |
| Mutation boundary | C++ represented cases write only the destination GPR and return `kExecuted` | Rust helper mutates only the destination GPR through sealed GPR write semantics | Equivalent | CPU and Machine preservation tests | No PC, next PC, Count, timer-pending, COP0, HI, LO, RDRAM, SP DMEM, reservation, Cartridge, fetch, decode, identify, exception entry, cadence commit, branch behavior, memory map, bus, or step behavior is added. |
| Step/cadence boundary | C++ step later commits PC and advances Count after `kExecuted` | Rust helper does not call commit or Count primitives | Equivalent separation | Cadence/commit/Count tests remain separate | The returned executed-readiness value is not a step result and does not wire outcome cadence. |

### Seam 068 Audit Changes

- Added crate-private `CpuSpecialNonTrappingIntegerExecutedInstruction`,
  `CpuSpecialNonTrappingIntegerExecutionError`, and
  `Cpu::execute_special_non_trapping_integer_instruction` in
  `cpu/instruction.rs`.
- Mirrored the source-clear SPECIAL non-trapping integer family: `ADDU`,
  `SUBU`, `DADDU`, `DSUBU`, `SLT`, and `SLTU`.
- Preserved C++ read-before-write behavior for `rs`/`rt`/`rd` aliases and the
  sealed zero-register read/write rule.
- Preserved C++ ADDU/SUBU low-word wrapping and sign-extended word result
  behavior, DADDU/DSUBU full-width wrapping behavior, and SLT/SLTU signed and
  unsigned full-width comparison behavior.
- Kept `ADD`, `SUB`, `DADD`, `DSUB`, and arithmetic overflow exception behavior
  intentionally absent.
- Added CPU tests for word wrapping/sign extension, doubleword wrapping,
  signed/unsigned comparison high-bit cases, source/destination aliasing,
  register-zero source/destination behavior, and trapping-identity rejection.
- Added a Machine preservation test proving no PC/next PC/Count/COP0/HI/LO/
  RDRAM/SP DMEM/reservation/Cartridge/fetch/decode/identify/exception/cadence/
  step effects.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, trapping arithmetic,
  overflow exception behavior, PC/next PC commit call, Count advancement call,
  branch/link/delay-slot behavior, load/store behavior, memory map, bus, device
  routing, SDL/window runtime, host shell, or C++ source changes.

## CPU SPECIAL Trapping Integer Overflow Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` SPECIAL trapping integer cases | `cpu/instruction.rs` `Cpu::execute_special_trapping_integer_instruction` | Equivalent for the represented narrow family only | SPECIAL trapping integer tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, or `Cpu::step`. |
| Represented identities | C++ cases `kSpecialAdd`, `kSpecialSub`, `kSpecialDadd`, and `kSpecialDsub` either write `rd` and return `kExecuted` or throw `kSignedArithmeticOverflow` before writeback | `CpuInstructionIdentity::SpecialAdd`, `SpecialSub`, `SpecialDadd`, and `SpecialDsub` | Equivalent | Per-identity trapping integer tests | ADD, SUB, DADD, and DSUB are represented. ADDU, SUBU, DADDU, and DSUBU remain already-sealed non-trapping identities. |
| ADD behavior | C++ reads low 32-bit words, interprets them as signed `int32_t`, computes in `int64_t`, rejects values outside `int32_t`, and sign-extends the successful word result | Rust reads full source GPRs before writeback, uses low-word signed interpretation, detects the same signed 32-bit range overflow, and sign-extends successful results | Equivalent | ADD non-overflow and overflow tests | Overflow returns a narrow overflow outcome before any destination writeback. |
| SUB behavior | C++ reads low 32-bit words, interprets them as signed `int32_t`, computes subtraction in `int64_t`, rejects values outside `int32_t`, and sign-extends the successful word result | Rust mirrors the same low-word signed subtraction, range check, and sign-extended successful result | Equivalent | SUB non-overflow and overflow tests | Overflow returns a narrow overflow outcome before any destination writeback. |
| DADD behavior | C++ uses `checked_signed_cpu_add`, computes full-width wrapping bits, applies signed overflow detection with the sign-bit rule, writes the full-width result on success, and throws before writeback on overflow | Rust uses full-width wrapping addition, the same signed overflow bit rule, full-width successful writeback, and overflow outcome before writeback | Equivalent | DADD non-overflow and overflow tests | No 32-bit truncation or sign extension is applied to successful DADD results. |
| DSUB behavior | C++ uses `checked_signed_cpu_sub`, computes full-width wrapping bits, applies signed overflow detection with the sign-bit rule, writes the full-width result on success, and throws before writeback on overflow | Rust uses full-width wrapping subtraction, the same signed overflow bit rule, full-width successful writeback, and overflow outcome before writeback | Equivalent | DSUB non-overflow and overflow tests | No 32-bit truncation or sign extension is applied to successful DSUB results. |
| Overflow-before-writeback | C++ calls `fail_signed_arithmetic_overflow` before `write_cpu_gpr_*` in each trapping arithmetic case | Rust returns `CpuSpecialTrappingIntegerExecutionOutcome::Overflow` before `set_gpr` | Equivalent | Overflow no-writeback tests | Destination GPR remains unchanged, including when `rd` is register zero. |
| Read-before-write aliases | C++ reads both source values before the destination writeback in represented cases | Rust stores source values in locals before possible `set_gpr` | Equivalent | Alias tests with `rs == rd` and `rt == rd` | Source/destination aliases preserve source values exactly. |
| Zero-register behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing; overflow happens before writeback | Rust uses sealed `Cpu::set_gpr`, and overflow returns before any write | Equivalent | Register-zero tests | Non-overflow writes to register zero are ignored; overflow leaves register zero unchanged. |
| Overflow outcome shape | C++ throws `MachineFaultKind::kSignedArithmeticOverflow` with operation text and no address payload | Rust returns `CpuSpecialTrappingIntegerOverflow` with identity, `rd`, `rs_value`, and `rt_value` | Equivalent readiness, Rust API shape differs | Overflow outcome tests | Rust uses a value because there is no generic MachineFault or execute dispatch in the Rust sidecar. |
| Overflow exception entry source owner | C++ `step_cpu_instruction` catches `kSignedArithmeticOverflow`, restores `pc`/`next_pc`, checks ordinary or delay-slot local synchronous entry, then calls `enter_local_signed_overflow_exception` | `cpu/cop0.rs` `Cpu::enter_arithmetic_overflow_exception` | Equivalent narrow entry primitive only | Arithmetic-overflow entry tests; source inspection | The Rust helper does not call this entry method. Future step wiring must still consume the overflow outcome and restored control-flow context. |
| Overflow exception code | C++ sets `cop0_exception_code_ = kCop0ExceptionCodeSignedOverflow`, code 12 | Rust sets exception code 12 | Equivalent | Entry tests | No reserved-instruction, syscall, break, or generic exception code handling is added. |
| EPC and branch-delay flag | C++ sets EPC to `faulting_pc` for ordinary entry or `faulting_pc - 4` with branch-delay flag for delay-slot entry | Rust uses current `pc` for ordinary entry or `pc - 4` with branch-delay flag when `next_pc` indicates a delay-slot context | Equivalent for the narrow primitive | Ordinary and delay-slot entry tests | Unsupported entry contexts return a narrow entry error without mutation. |
| Status.EXL and vector | C++ sets EXL and vectors `cpu_pc_`/`cpu_next_pc_` to `0x80000180`/`0x80000184` | Rust sets Status.EXL and the same local exception vector pair | Equivalent | Entry tests | BEV, interrupts, ERET, and generic vector dispatch remain absent. |
| BadVAddr and Count behavior | C++ signed overflow entry does not write BadVAddr and does not advance Count on exception paths | Rust overflow entry leaves BadVAddr and Count unchanged | Equivalent | Entry no-mutation tests | Count cadence remains committed-step-owned and unwired. |
| Step/cadence boundary | C++ only reaches overflow entry from `step_cpu_instruction` after execute throws and control-flow rollback occurs | Rust helper and entry are separate; no step wiring exists | Equivalent separation | Machine preservation and entry tests | No commit, Count advance, fetch/decode/identify loop, generic execute result, or step API is added. |
| Mutation boundary | C++ successful represented cases write only `rd`; overflow entry mutates only narrow COP0/control-flow exception state | Rust success writes only `rd`; overflow outcome mutates nothing; overflow entry mutates only code/EPC/BD/EXL/PC/next PC | Equivalent for represented primitives | CPU, COP0, and Machine preservation tests | No RDRAM, SP DMEM, reservation, Cartridge, memory map, bus, device routing, branch behavior, or load/store behavior is added. |

### Seam 069 Audit Changes

- Added crate-private `CpuSpecialTrappingIntegerExecutedInstruction`,
  `CpuSpecialTrappingIntegerOverflow`,
  `CpuSpecialTrappingIntegerExecutionOutcome`,
  `CpuSpecialTrappingIntegerExecutionError`, and
  `Cpu::execute_special_trapping_integer_instruction` in `cpu/instruction.rs`.
- Mirrored the source-clear SPECIAL trapping integer family: `ADD`, `SUB`,
  `DADD`, and `DSUB`.
- Preserved C++ ADD/SUB low-word signed arithmetic, signed 32-bit overflow
  detection, and sign-extended successful word result behavior.
- Preserved C++ DADD/DSUB full-width signed overflow detection and full-width
  successful writeback behavior.
- Preserved overflow-before-writeback: overflow returns a narrow outcome and
  leaves `rd` unchanged.
- Added crate-private `Cpu::enter_arithmetic_overflow_exception` and
  `CpuArithmeticOverflowExceptionEntryError` in `cpu/cop0.rs`.
- Mirrored narrow overflow entry facts: Cause code 12, EPC from faulting PC or
  branch-delay PC, branch-delay flag, Status.EXL, local exception vector, no
  BadVAddr mutation, and no Count advancement.
- Added CPU tests for all four trapping identities, overflow detection,
  successful result width/sign-extension, destination preservation on overflow,
  aliases, register-zero behavior, and identity rejection.
- Added COP0 tests for ordinary overflow entry, delay-slot overflow entry,
  EXL-blocked entry, unsupported delay-slot context, BadVAddr preservation, and
  Count preservation.
- Added Machine preservation tests proving successful trapping arithmetic
  mutates only the destination GPR and overflow outcomes mutate no Machine,
  Cpu, COP0, PC/next PC, Count, HI/LO, RDRAM, SP DMEM, reservation, or
  Cartridge state.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, generic exception
  machinery, overflow-to-entry wiring, PC/next PC commit call, Count advancement
  call, branch/link/delay-slot behavior, load/store behavior, memory map, bus,
  device routing, SDL/window runtime, host shell, or C++ source changes.

## CPU Immediate Trapping Integer Overflow Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` immediate trapping integer cases | `cpu/instruction.rs` `Cpu::execute_immediate_trapping_integer_instruction` | Equivalent for the represented narrow family only | Immediate trapping integer tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, `Cpu::step`, generic immediate helper, or public decode field. |
| Represented identities | C++ cases `kAddi` and `kDaddi` either write `rt` and return `kExecuted` or throw `kSignedArithmeticOverflow` before writeback | `CpuInstructionIdentity::Addi` and `Daddi` | Equivalent | Per-identity immediate trapping tests | ADDI and DADDI are represented. ADDIU and DADDIU remain separate non-trapping immediate identities. |
| ADDI immediate interpretation | C++ uses `DecodedCpuInstructionWord::immediate_i16`, produced by `i16_from_u16_bits(immediate_u16)`, and widens it to `int64_t` for the addition | Rust keeps decode as raw `immediate_u16` and interprets it locally with private `i16_from_u16_bits` inside the ADDI/DADDI helper | Equivalent for ADDI only | Positive and negative immediate tests; source inspection | Signed immediate interpretation is not exported as generic decode truth. |
| ADDI result behavior | C++ reads `rs` as a low 32-bit word, interprets it as signed `int32_t`, adds the signed 16-bit immediate in `int64_t`, rejects values outside `int32_t`, and sign-extends the successful word result | Rust reads full `rs` before writeback, uses low-word signed interpretation, adds the local signed immediate, detects the same signed 32-bit range overflow, and sign-extends successful results | Equivalent | ADDI non-overflow, sign-extension, and overflow tests | High source GPR bits do not affect ADDI except through the low word. |
| DADDI immediate interpretation | C++ uses `sign_extend_u16_to_cpu_value(immediate_u16)` before `checked_signed_cpu_add` | Rust uses private `sign_extend_u16_to_cpu_value` inside the immediate trapping helper | Equivalent for DADDI only | DADDI positive/negative immediate tests; source inspection | This does not add a generic sign-extension API. |
| DADDI result behavior | C++ uses `checked_signed_cpu_add`, computes full-width wrapping bits, applies signed overflow detection with the sign-bit rule, writes the full-width result on success, and throws before writeback on overflow | Rust uses full-width wrapping addition with the sign-extended immediate bits, the same signed overflow bit rule, full-width successful writeback, and overflow outcome before writeback | Equivalent | DADDI non-overflow and overflow tests | No 32-bit truncation or sign extension is applied to successful DADDI results. |
| Overflow-before-writeback | C++ calls `fail_signed_arithmetic_overflow` before `write_cpu_gpr_*` in both represented cases | Rust returns `CpuImmediateTrappingIntegerExecutionOutcome::Overflow` before `set_gpr` | Equivalent | Overflow no-writeback tests | Destination `rt` remains unchanged, including when `rt` is register zero. |
| Read-before-write aliases | C++ reads `rs` before destination `rt` writeback | Rust stores `rs` and the interpreted immediate in locals before possible `set_gpr` | Equivalent | Alias tests with `rs == rt` | Source/destination aliases preserve the original source value exactly. |
| Zero-register behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing; overflow happens before writeback | Rust uses sealed `Cpu::set_gpr`, and overflow returns before any write | Equivalent | Register-zero tests | Non-overflow writes to register zero are ignored; overflow leaves register zero unchanged. |
| Overflow outcome shape | C++ throws `MachineFaultKind::kSignedArithmeticOverflow` with operation text and no address payload | Rust returns `CpuImmediateTrappingIntegerOverflow` with identity, `rt`, `rs_value`, raw `immediate_u16`, and sign-extended `immediate_value` | Equivalent readiness, Rust API shape differs | Overflow outcome tests | Rust uses a value because there is no generic MachineFault or execute dispatch in the Rust sidecar. |
| Overflow exception entry reuse | C++ `step_cpu_instruction` catches the same `kSignedArithmeticOverflow` used by ADD/SUB/DADD/DSUB and calls the same local signed-overflow entry path when allowed | Existing `cpu/cop0.rs` `Cpu::enter_arithmetic_overflow_exception` remains the only Rust overflow-entry primitive | Equivalent separation | Arithmetic-overflow entry tests remain unchanged; source inspection | The immediate helper does not call this entry method. Future step wiring must still consume the overflow outcome and restored control-flow context. |
| ADDIU/DADDIU separation | C++ has separate non-trapping `kAddiu` and `kDaddiu` cases that use signed immediate bits without overflow trapping | Separate Rust `Cpu::execute_immediate_non_trapping_integer_instruction` helper | Equivalent separation after seam 071 | Source inspection; identity rejection tests | Non-trapping immediate arithmetic is represented by its own helper and is not inferred from ADDI/DADDI overflow semantics. |
| Generic immediate boundary | C++ decode stores `immediate_i16`, but earlier Rust decode intentionally exposes only raw `immediate_u16` | Rust immediate interpretation is private to this helper | Boundary preserved | Decode tests and immediate execution tests | No `immediate_i16` decode field, generic sign extension helper, or generic immediate semantics are added. |
| Step/cadence boundary | C++ only reaches overflow entry from `step_cpu_instruction` after execute throws and control-flow rollback occurs; non-overflow later commits PC and Count through step | Rust helper and existing entry are separate; no step wiring exists | Equivalent separation | Machine preservation and entry tests | No commit, Count advance, fetch/decode/identify loop, generic execute result, or step API is added. |
| Mutation boundary | C++ successful represented cases write only `rt`; overflow entry mutates only narrow COP0/control-flow exception state when step selects it | Rust success writes only `rt`; overflow outcome mutates nothing; existing overflow entry is unchanged and unwired | Equivalent for represented primitives | CPU and Machine preservation tests | No RDRAM, SP DMEM, reservation, Cartridge, memory map, bus, device routing, branch behavior, or load/store behavior is added. |

### Seam 070 Audit Changes

- Added crate-private `CpuImmediateTrappingIntegerExecutedInstruction`,
  `CpuImmediateTrappingIntegerOverflow`,
  `CpuImmediateTrappingIntegerExecutionOutcome`,
  `CpuImmediateTrappingIntegerExecutionError`, and
  `Cpu::execute_immediate_trapping_integer_instruction` in
  `cpu/instruction.rs`.
- Mirrored the source-clear immediate trapping integer family: `ADDI` and
  `DADDI`.
- Preserved C++ ADDI immediate interpretation from raw `immediate_u16` to signed
  `i16`, ADDI low-word signed arithmetic, signed 32-bit overflow detection, and
  sign-extended successful word result behavior.
- Preserved C++ DADDI sign-extended immediate bits, full-width signed overflow
  detection, and full-width successful writeback behavior.
- Preserved overflow-before-writeback: overflow returns a narrow outcome and
  leaves `rt` unchanged.
- Reused the already-sealed narrow arithmetic-overflow entry only as a separate
  unwired primitive; no new exception entry or generic exception dispatch was
  added.
- Added CPU tests for both trapping immediate identities, positive and negative
  immediates, overflow detection, successful result width/sign-extension,
  destination preservation on overflow, aliases, register-zero behavior, and
  identity rejection.
- Added Machine preservation tests proving successful immediate trapping
  arithmetic mutates only the destination GPR and overflow outcomes mutate no
  Machine, Cpu, COP0, PC/next PC, Count, HI/LO, RDRAM, SP DMEM, reservation, or
  Cartridge state.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, generic immediate
  semantics, generic exception machinery, overflow-to-entry wiring, PC/next PC
  commit call, Count advancement call,
  branch/link/delay-slot behavior, load/store behavior, memory map, bus, device
  routing, SDL/window runtime, host shell, or C++ source changes.

## CPU Immediate Non-Trapping Integer GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` immediate non-trapping integer cases | `cpu/instruction.rs` `Cpu::execute_immediate_non_trapping_integer_instruction` | Equivalent for the represented narrow family only | Immediate non-trapping integer tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, `Cpu::step`, generic immediate helper, or public decode field. |
| Represented identities | C++ cases `kAddiu` and `kDaddiu` write `rt` and return `kExecuted` without overflow checks | `CpuInstructionIdentity::Addiu` and `Daddiu` | Equivalent | Per-identity immediate non-trapping tests | ADDIU and DADDIU are represented. ADDI and DADDI remain in the separate trapping helper. |
| ADDIU immediate interpretation | C++ uses `sign_extend_u16_to_u32(instruction.immediate_u16)` as the low-word operand | Rust keeps decode as raw `immediate_u16` and interprets it locally with private `sign_extend_u16_to_u32` inside the ADDIU/DADDIU helper | Equivalent for ADDIU only | Positive and negative immediate tests; source inspection | Signed immediate interpretation is not exported as generic decode truth. |
| ADDIU result behavior | C++ reads `rs` as a low 32-bit word, adds the sign-extended 16-bit word operand with `uint32_t` wrapping, then uses `write_cpu_gpr_word_sign_extended_result` | Rust reads full `rs` before writeback, uses low-word wrapping arithmetic with the local sign-extended immediate word, and sign-extends the successful word result | Equivalent | ADDIU positive/negative, word-wrap, and sign-extension tests | High source GPR bits do not affect ADDIU except through the low word. |
| DADDIU immediate interpretation | C++ uses `sign_extend_u16_to_cpu_value(immediate_u16)` as a full CPU-value operand | Rust uses private `sign_extend_u16_to_cpu_value` inside the immediate non-trapping helper | Equivalent for DADDIU only | DADDIU positive/negative immediate tests; source inspection | This does not add a generic sign-extension API. |
| DADDIU result behavior | C++ reads full-width `rs`, adds the sign-extended immediate value with unsigned CPU-register wrapping, writes the full-width result through `write_cpu_gpr_value`, and returns `kExecuted` | Rust reads full-width `rs`, uses `wrapping_add` with the sign-extended immediate bits, writes the full-width result through sealed GPR semantics, and returns the executed-instruction value | Equivalent | DADDIU full-width and wrap tests | No 32-bit truncation or sign extension is applied to successful DADDIU results. |
| Wrapping/no-overflow behavior | C++ ADDIU/DADDIU do not call `fail_signed_arithmetic_overflow`, `checked_signed_cpu_add`, or any exception path | Rust helper has no overflow outcome and does not call `Cpu::enter_arithmetic_overflow_exception` | Equivalent | ADDIU/DADDIU wrap tests; source inspection | Signed overflow shapes still write the wrapped result. Overflow exception behavior remains exclusive to trapping arithmetic helpers and separate entry primitives. |
| Read-before-write aliases | C++ reads `rs` before destination `rt` writeback | Rust stores `rs` in a local before `set_gpr` | Equivalent | Alias tests with `rs == rt` | Source/destination aliases preserve the original source value exactly. |
| Zero-register behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing | Rust uses sealed `Cpu::set_gpr` | Equivalent | Register-zero tests | Writes to register zero are ignored. |
| Rejected identities | C++ has separate implemented cases for ADDI/DADDI, SLTI/SLTIU, ANDI/ORI/XORI, and LUI | Rust helper returns `UnsupportedIdentity` for non-ADDIU/DADDIU identities | Equivalent boundary | Identity rejection test | This helper does not represent trapping immediate arithmetic, immediate comparison, immediate logical instructions, or upper-immediate behavior. |
| Generic immediate boundary | C++ decode stores `immediate_i16`, but earlier Rust decode intentionally exposes only raw `immediate_u16` | Rust immediate interpretation is private to this helper | Boundary preserved | Decode tests and immediate execution tests | No `immediate_i16` decode field, generic sign extension helper, generic zero extension helper, or generic immediate semantics are added. |
| Step/cadence boundary | C++ only commits PC/next PC and advances Count after `step_cpu_instruction` receives `kExecuted` | Rust helper only writes `rt`; no step wiring exists | Equivalent separation | Machine preservation test | No commit, Count advance, fetch/decode/identify loop, generic execute result, or step API is added. |
| Mutation boundary | C++ successful represented cases write only `rt` | Rust success writes only `rt` | Equivalent for represented primitive | CPU and Machine preservation tests | No COP0, PC/next PC, Count, HI/LO, RDRAM, SP DMEM, reservation, Cartridge, memory map, bus, device routing, branch behavior, load/store behavior, or exception behavior is added. |

### Seam 071 Audit Changes

- Added crate-private
  `CpuImmediateNonTrappingIntegerExecutedInstruction`,
  `CpuImmediateNonTrappingIntegerExecutionError`, and
  `Cpu::execute_immediate_non_trapping_integer_instruction` in
  `cpu/instruction.rs`.
- Mirrored the source-clear immediate non-trapping integer family: `ADDIU` and
  `DADDIU`.
- Preserved C++ ADDIU immediate interpretation from raw `immediate_u16` to a
  sign-extended 32-bit operand, ADDIU low-word wrapping arithmetic, and
  sign-extended successful word result behavior.
- Preserved C++ DADDIU sign-extended immediate bits, full-width wrapping
  arithmetic, and full-width successful writeback behavior.
- Preserved wrapping/no-overflow behavior: signed overflow shapes still write
  the wrapped result and do not enter the arithmetic-overflow exception path.
- Added CPU tests for both non-trapping immediate identities, positive and
  negative immediates, wrapping behavior, result width/sign-extension, aliases,
  register-zero behavior, and identity rejection.
- Added a Machine preservation test proving successful immediate non-trapping
  arithmetic mutates only the destination GPR and leaves Machine, Cpu control
  flow, COP0, Count, HI/LO, RDRAM, SP DMEM, reservation, and Cartridge state
  otherwise unchanged.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, generic immediate
  semantics, generic sign/zero extension public API, ADDI/DADDI changes,
  SLTI/SLTIU, ANDI/ORI/XORI, LUI, overflow exception behavior,
  PC/next PC commit call, Count advancement call, branch/link/delay-slot
  behavior, load/store behavior, memory map, bus, device routing,
  SDL/window runtime, host shell, or C++ source changes.

## CPU Immediate Comparison GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` immediate comparison cases | `cpu/instruction.rs` `Cpu::execute_immediate_comparison_instruction` | Equivalent for the represented narrow family only | Immediate comparison tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, `Cpu::step`, generic immediate helper, or public decode field. |
| Represented identities | C++ cases `kSlti` and `kSltiu` write `rt` as 1 or 0 and return `kExecuted` | `CpuInstructionIdentity::Slti` and `Sltiu` | Equivalent | Per-identity immediate comparison tests | SLTI and SLTIU are represented. Arithmetic and logical immediates remain separate. |
| SLTI immediate interpretation | C++ uses `sign_extend_u16_to_cpu_value(instruction.immediate_u16)` as the full-width immediate operand | Rust keeps decode as raw `immediate_u16` and interprets it locally with private `sign_extend_u16_to_cpu_value` inside the SLTI/SLTIU helper | Equivalent for SLTI | Positive and negative immediate tests; source inspection | Signed immediate interpretation is not exported as generic decode truth. |
| SLTI comparison behavior | C++ compares `read_cpu_gpr_value(rs)` and the sign-extended immediate with `signed_cpu_value_less_than` | Rust reads full `rs` before writeback and uses `signed_cpu_value_less_than(rs, immediate_value)` | Equivalent | Signed less/equal/greater and high-bit tests | Comparison is full-width signed CPU-value comparison, not low-word comparison. |
| SLTIU immediate interpretation | C++ also uses `sign_extend_u16_to_cpu_value(instruction.immediate_u16)` for unsigned comparison | Rust uses the same private sign-extended full-width immediate value for SLTIU | Equivalent for SLTIU | SLTIU sign-extension test; source inspection | This intentionally mirrors C++ sign-extension for SLTIU and does not use zero-extension. |
| SLTIU comparison behavior | C++ compares `read_cpu_gpr_value(rs)` and the sign-extended immediate with `unsigned_cpu_value_less_than` | Rust reads full `rs` before writeback and uses unsigned `rs < immediate_value` | Equivalent | Unsigned less/equal/greater and high-bit tests | Comparison is full-width unsigned CPU-value comparison. |
| 1/0 writeback | C++ converts the comparison boolean with `cpu_value_from_bool` and writes through `write_cpu_gpr_value(rt, value)` | Rust converts with private `cpu_value_from_bool` and writes through sealed `Cpu::set_gpr` | Equivalent | Per-outcome tests | True writes 1, false writes 0. |
| Read-before-write aliases | C++ reads `rs` before destination `rt` writeback | Rust stores `rs` and interpreted immediate in locals before `set_gpr` | Equivalent | Alias tests with `rs == rt` | Source/destination aliases preserve the original source value exactly. |
| Zero-register behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing | Rust uses sealed `Cpu::set_gpr` | Equivalent | Register-zero tests | Writes to register zero are ignored. |
| Rejected identities | C++ has separate implemented cases for ADDI/DADDI, ADDIU/DADDIU, ANDI/ORI/XORI, and LUI | Rust helper returns `UnsupportedIdentity` for non-SLTI/SLTIU identities | Equivalent boundary | Identity rejection test | This helper does not represent arithmetic immediates, logical immediates, or upper-immediate behavior. |
| Generic immediate boundary | C++ decode stores `immediate_i16`, but earlier Rust decode intentionally exposes only raw `immediate_u16` | Rust immediate interpretation is private to this helper | Boundary preserved | Decode tests and immediate execution tests | No `immediate_i16` decode field, generic sign extension helper, generic zero extension helper, or generic immediate semantics are added. |
| Step/cadence boundary | C++ only commits PC/next PC and advances Count after `step_cpu_instruction` receives `kExecuted` | Rust helper only writes `rt`; no step wiring exists | Equivalent separation | Machine preservation test | No commit, Count advance, fetch/decode/identify loop, generic execute result, or step API is added. |
| Mutation boundary | C++ successful represented cases write only `rt` | Rust success writes only `rt` | Equivalent for represented primitive | CPU and Machine preservation tests | No COP0, PC/next PC, Count, HI/LO, RDRAM, SP DMEM, reservation, Cartridge, memory map, bus, device routing, branch behavior, load/store behavior, or exception behavior is added. |

### Seam 072 Audit Changes

- Added crate-private `CpuImmediateComparisonExecutedInstruction`,
  `CpuImmediateComparisonExecutionError`, and
  `Cpu::execute_immediate_comparison_instruction` in `cpu/instruction.rs`.
- Mirrored the source-clear immediate comparison family: `SLTI` and `SLTIU`.
- Preserved C++ SLTI immediate interpretation from raw `immediate_u16` to a
  sign-extended full-width CPU value and full-width signed less-than comparison.
- Preserved C++ SLTIU immediate interpretation from raw `immediate_u16` to the
  same sign-extended full-width CPU value and full-width unsigned less-than
  comparison.
- Preserved 1/0 writeback through sealed GPR semantics.
- Added CPU tests for both comparison identities, signed and unsigned
  less/equal/greater cases, negative immediates, high-bit sources,
  sign-extended SLTIU immediate behavior, aliases, register-zero behavior, and
  identity rejection.
- Added a Machine preservation test proving successful immediate comparison
  mutates only the destination GPR and leaves Machine, Cpu control flow, COP0,
  Count, HI/LO, RDRAM, SP DMEM, reservation, and Cartridge state otherwise
  unchanged.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, generic immediate
  semantics, generic sign/zero extension public API, ADDI/DADDI changes,
  ADDIU/DADDIU changes, ANDI/ORI/XORI, LUI, overflow exception behavior,
  PC/next PC commit call, Count advancement call, branch/link/delay-slot
  behavior, load/store behavior, memory map, bus, device routing,
  SDL/window runtime, host shell, or C++ source changes.

## CPU Immediate Bitwise Logical GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` immediate bitwise logical cases | `cpu/instruction.rs` `Cpu::execute_immediate_bitwise_logical_instruction` | Equivalent for the represented narrow family only | Immediate bitwise logical tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, `Cpu::step`, generic immediate helper, or public decode field. |
| Represented identities | C++ cases `kAndi`, `kOri`, and `kXori` write `rt` and return `kExecuted` | `CpuInstructionIdentity::Andi`, `Ori`, and `Xori` | Equivalent | Per-identity immediate bitwise logical tests | ANDI, ORI, and XORI are represented. LUI and other immediates remain separate. |
| Immediate interpretation | C++ casts raw `instruction.immediate_u16` directly to `CpuRegisterValue` | Rust keeps decode as raw `immediate_u16` and interprets it locally with `u64::from(instruction.immediate_u16())` inside this helper | Equivalent for ANDI/ORI/XORI only | High-bit, `0xffff`, and zero-immediate tests; source inspection | This is instruction-family-owned zero-extension, not generic zero-extension semantics or a new decode field. |
| ANDI result behavior | C++ computes `read_cpu_gpr_value(rs) & static_cast<CpuRegisterValue>(immediate_u16)` | Rust reads full `rs` before writeback and computes `rs & immediate_value` | Equivalent | ANDI result and immediate tests | The source value and result are full-width CPU register values. |
| ORI result behavior | C++ computes `read_cpu_gpr_value(rs) \| static_cast<CpuRegisterValue>(immediate_u16)` | Rust reads full `rs` before writeback and computes `rs \| immediate_value` | Equivalent | ORI result and immediate tests | High immediate bit `0x8000` becomes `0x0000_0000_0000_8000`, not a sign-extended value. |
| XORI result behavior | C++ computes `read_cpu_gpr_value(rs) ^ static_cast<CpuRegisterValue>(immediate_u16)` | Rust reads full `rs` before writeback and computes `rs ^ immediate_value` | Equivalent | XORI result and immediate tests | Full-width high source bits are preserved except where the zero-extended low immediate toggles them. |
| GPR writeback | C++ writes through `write_cpu_gpr_value(rt, value)` | Rust writes through sealed `Cpu::set_gpr` | Equivalent | Per-identity and preservation tests | The full 64-bit result is written to `rt`; register zero writes remain ignored. |
| Read-before-write aliases | C++ reads `rs` before destination `rt` writeback | Rust stores `rs` and interpreted immediate in locals before `set_gpr` | Equivalent | Alias test with `rs == rt` | Source/destination aliases preserve the original source value exactly. |
| Zero-register behavior | C++ `write_cpu_gpr_value(0, value)` returns without storing; reads of register zero return zero | Rust uses sealed `Cpu::set_gpr` and `Cpu::gpr` | Equivalent | Register-zero source and destination tests | Writes to register zero are ignored and register zero reads as zero. |
| Rejected identities | C++ has separate implemented cases for LUI, SLTI/SLTIU, ADDI/DADDI, and ADDIU/DADDIU | Rust helper returns `UnsupportedIdentity` for non-ANDI/ORI/XORI identities | Equivalent boundary | Identity rejection test | This helper does not represent upper-immediate, comparison, trapping arithmetic, or non-trapping arithmetic immediates. |
| Generic immediate boundary | C++ decode stores `immediate_i16`, but earlier Rust decode intentionally exposes only raw `immediate_u16` | Rust immediate interpretation is private to this helper | Boundary preserved | Decode tests and immediate execution tests | No `immediate_i16`, generic zero-extension decode field, generic sign/zero extension public API, or generic immediate semantics are added. |
| Step/cadence boundary | C++ only commits PC/next PC and advances Count after `step_cpu_instruction` receives `kExecuted` | Rust helper only writes `rt`; no step wiring exists | Equivalent separation | Machine preservation test | No commit, Count advance, fetch/decode/identify loop, generic execute result, or step API is added. |
| Mutation boundary | C++ successful represented cases write only `rt` | Rust success writes only `rt` | Equivalent for represented primitive | CPU and Machine preservation tests | No COP0, PC/next PC, Count, HI/LO, RDRAM, SP DMEM, reservation, Cartridge, memory map, bus, device routing, branch behavior, load/store behavior, or exception behavior is added. |

### Seam 073 Audit Changes

- Added crate-private `CpuImmediateBitwiseLogicalExecutedInstruction`,
  `CpuImmediateBitwiseLogicalExecutionError`, and
  `Cpu::execute_immediate_bitwise_logical_instruction` in `cpu/instruction.rs`.
- Mirrored the source-clear immediate bitwise logical family: `ANDI`, `ORI`,
  and `XORI`.
- Preserved C++ immediate interpretation from raw `immediate_u16` to a
  zero-extended full-width CPU value inside this instruction family only.
- Preserved full-width GPR source/result behavior and `rt` writeback through
  sealed GPR semantics.
- Added CPU tests for all three identities, high-bit `0x8000`, `0xffff`, and
  `0x0000` immediate behavior, aliases, register-zero source/destination
  behavior, and identity rejection.
- Added a Machine preservation test proving successful immediate bitwise
  logical execution mutates only the destination GPR and leaves Machine, Cpu
  control flow, COP0, Count, HI/LO, RDRAM, SP DMEM, reservation, and Cartridge
  state otherwise unchanged.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, generic immediate
  semantics, generic zero-extension public API, LUI, SLTI/SLTIU changes,
  ADDI/DADDI changes, ADDIU/DADDIU changes, PC/next PC commit call, Count
  advancement call, branch/link/delay-slot behavior, load/store behavior,
  memory map, bus, device routing, SDL/window runtime, host shell, or C++
  source changes.

## CPU Upper-Immediate LUI GPR Writeback Execution Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` `kLui` case | `cpu/instruction.rs` `Cpu::execute_upper_immediate_instruction` | Equivalent for LUI only | Upper-immediate LUI tests; source inspection | Rust adds a crate-private CPU-owned helper, not a generic `execute_cpu_instruction`, `Machine::step`, `Cpu::step`, generic immediate helper, generic upper-immediate helper, or public decode field. |
| Represented identity | C++ case `kLui` writes `rt` and returns `kExecuted` | `CpuInstructionIdentity::Lui` | Equivalent | LUI identity tests | LUI is represented. Arithmetic, comparison, and logical immediates remain separate. |
| Immediate interpretation | C++ computes `static_cast<std::uint32_t>(instruction.immediate_u16) << 16` | Rust keeps decode as raw `immediate_u16` and computes `u32::from(instruction.immediate_u16()) << 16` inside this helper | Equivalent for LUI only | `0x0000`, `0x0001`, `0x7fff`, `0x8000`, and `0xffff` tests | This is LUI-owned upper-immediate interpretation, not generic immediate or generic upper-immediate semantics. |
| Result extension | C++ writes through `write_cpu_gpr_word_sign_extended_result`, which calls `sign_extend_u32_to_cpu_value` | Rust calls private `sign_extend_u32_to_cpu_value` on the shifted 32-bit word result before `set_gpr` | Equivalent | High-bit sign behavior tests | `0x8000` writes `0xffff_ffff_8000_0000`; `0xffff` writes `0xffff_ffff_ffff_0000`. |
| GPR writeback | C++ writes through `write_cpu_gpr_word_sign_extended_result(rt, value)` and then `write_cpu_gpr_value` | Rust writes through sealed `Cpu::set_gpr` | Equivalent | Per-immediate and register-zero tests | Writes to register zero are ignored through sealed GPR semantics. |
| rs field behavior | C++ identifies opcode `0x0f` as `kLui` and the execution case does not read or validate `rs` | Rust helper does not read or validate `instruction.rs()` | Equivalent | Nonzero-`rs` test | `rs` is ignored, not rejected. |
| Rejected identities | C++ has separate implemented cases for ANDI/ORI/XORI, SLTI/SLTIU, ADDI/DADDI, and ADDIU/DADDIU | Rust helper returns `UnsupportedIdentity` for non-LUI identities | Equivalent boundary | Identity rejection test | This helper does not represent immediate logical, comparison, trapping arithmetic, or non-trapping arithmetic families. |
| Generic immediate boundary | C++ decode stores `immediate_i16`, but earlier Rust decode intentionally exposes only raw `immediate_u16` | Rust immediate interpretation is private to this helper | Boundary preserved | Decode tests and immediate execution tests | No `immediate_i16`, generic zero-extension decode field, generic upper-immediate decode field, or generic sign/zero/upper-immediate public API is added. |
| Step/cadence boundary | C++ only commits PC/next PC and advances Count after `step_cpu_instruction` receives `kExecuted` | Rust helper only writes `rt`; no step wiring exists | Equivalent separation | Machine preservation test | No commit, Count advance, fetch/decode/identify loop, generic execute result, or step API is added. |
| Mutation boundary | C++ successful represented case writes only `rt` | Rust success writes only `rt` | Equivalent for represented primitive | CPU and Machine preservation tests | No COP0, PC/next PC, Count, HI/LO, RDRAM, SP DMEM, reservation, Cartridge, memory map, bus, device routing, branch behavior, load/store behavior, or exception behavior is added. |

### Seam 074 Audit Changes

- Added crate-private `CpuUpperImmediateExecutedInstruction`,
  `CpuUpperImmediateExecutionError`, and
  `Cpu::execute_upper_immediate_instruction` in `cpu/instruction.rs`.
- Mirrored the source-clear upper-immediate family: `LUI`.
- Preserved C++ immediate interpretation from raw `immediate_u16` to a shifted
  32-bit word result: `u32::from(immediate_u16) << 16`.
- Preserved C++ writeback through sign-extension from the shifted 32-bit word
  to the full CPU value.
- Preserved C++ `rs` behavior: opcode `0x0f` identifies as LUI regardless of
  `rs`, and execution ignores `rs`.
- Added CPU tests for `0x0000`, `0x0001`, `0x7fff`, `0x8000`, and `0xffff`
  immediate values, high-bit sign behavior, register-zero write behavior,
  ignored `rs`, and identity rejection.
- Added a Machine preservation test proving successful LUI execution mutates
  only the destination GPR and leaves Machine, Cpu control flow, COP0, Count,
  HI/LO, RDRAM, SP DMEM, reservation, and Cartridge state otherwise unchanged.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, generic immediate
  semantics, generic upper-immediate public API, ANDI/ORI/XORI changes,
  SLTI/SLTIU changes, ADDI/DADDI changes, ADDIU/DADDIU changes, PC/next PC
  commit call, Count advancement call, branch/link/delay-slot behavior,
  load/store behavior, memory map, bus, device routing, SDL/window runtime,
  host shell, or C++ source changes.

## CPU-Local Executed-Helper Selection Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Selection source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` switch over `CpuInstructionIdentity` | `cpu/instruction.rs` `select_cpu_local_executed_helper` | Equivalent for identity-to-family selection only | Selector tests; source inspection | The selector itself does not invoke helpers. Rust does not implement generic `execute_cpu_instruction`, `Machine::step`, `Cpu::step`, or a generic step result. |
| Selection input | C++ execute switch receives already identified `CpuInstructionIdentity` and decoded fields | Rust selector consumes only `CpuInstructionIdentity` | Equivalent for this readiness layer | Selector tests | Decoded fields remain required only by the sealed helper bodies at execution time. The selector performs no fetch, decode, identify, or execute. |
| No-effect executed family | C++ `kSpecialSync` returns `kExecuted` directly | `CpuLocalExecutedHelperFamily::NoEffectSync` | Equivalent | SYNC selector test | SYNC remains no-effect readiness; no Count/commit/step behavior is added. |
| SPECIAL shift family | C++ `kSpecialSll`, `kSpecialSrl`, `kSpecialSra`, `kSpecialSllv`, `kSpecialSrlv`, `kSpecialSrav`, `kSpecialDsll`, `kSpecialDsrl`, `kSpecialDsra`, `kSpecialDsll32`, `kSpecialDsrl32`, `kSpecialDsra32`, `kSpecialDsllv`, `kSpecialDsrlv`, and `kSpecialDsrav` execute through CPU-local GPR read/compute/writeback cases | `CpuLocalExecutedHelperFamily::SpecialShift` | Equivalent for represented identities | Shift selector tests | Selection names the existing sealed shift helper family and does not call it. |
| SPECIAL bitwise logical family | C++ `kSpecialAnd`, `kSpecialOr`, `kSpecialXor`, and `kSpecialNor` execute through CPU-local GPR logical writeback cases | `CpuLocalExecutedHelperFamily::SpecialBitwiseLogical` | Equivalent for represented identities | Logical selector tests | Immediate logical instructions remain a separate represented family. |
| SPECIAL HI/LO transfer family | C++ `kSpecialMfhi`, `kSpecialMthi`, `kSpecialMflo`, and `kSpecialMtlo` execute through CPU-local HI/LO/GPR transfer cases | `CpuLocalExecutedHelperFamily::SpecialHiLoTransfer` | Equivalent for represented identities | HI/LO selector tests | Multiply/divide remains excluded. |
| SPECIAL non-trapping integer family | C++ `kSpecialAddu`, `kSpecialSubu`, `kSpecialDaddu`, `kSpecialDsubu`, `kSpecialSlt`, and `kSpecialSltu` execute through CPU-local non-trapping GPR writeback cases | `CpuLocalExecutedHelperFamily::SpecialNonTrappingInteger` | Equivalent for represented identities | Non-trapping selector tests | Trapping ADD/SUB/DADD/DSUB remain separate. |
| SPECIAL trapping integer family | C++ `kSpecialAdd`, `kSpecialSub`, `kSpecialDadd`, and `kSpecialDsub` either write back on success or throw signed-overflow before writeback | `CpuLocalExecutedHelperFamily::SpecialTrappingInteger` | Equivalent for represented readiness family | Trapping selector tests | Selection names the already sealed trapping helper family; it does not enter overflow exception entry. |
| Immediate trapping integer family | C++ `kAddi` and `kDaddi` either write back on success or throw signed-overflow before writeback | `CpuLocalExecutedHelperFamily::ImmediateTrappingInteger` | Equivalent for represented readiness family | Immediate trapping selector tests | Immediate signed interpretation remains inside the sealed helper, not the selector. |
| Immediate non-trapping integer family | C++ `kAddiu` and `kDaddiu` execute through CPU-local immediate non-trapping writeback cases | `CpuLocalExecutedHelperFamily::ImmediateNonTrappingInteger` | Equivalent for represented identities | Immediate non-trapping selector tests | ADDI/DADDI remain separate trapping identities. |
| Immediate comparison family | C++ `kSlti` and `kSltiu` execute through CPU-local 1/0 writeback cases | `CpuLocalExecutedHelperFamily::ImmediateComparison` | Equivalent for represented identities | Immediate comparison selector tests | Immediate interpretation remains helper-owned. |
| Immediate bitwise logical family | C++ `kAndi`, `kOri`, and `kXori` execute through CPU-local zero-extended immediate logical writeback cases | `CpuLocalExecutedHelperFamily::ImmediateBitwiseLogical` | Equivalent for represented identities | Immediate logical selector tests | Generic zero-extension semantics remain absent. |
| Upper-immediate family | C++ `kLui` executes through CPU-local shifted-word sign-extended writeback | `CpuLocalExecutedHelperFamily::UpperImmediateLui` | Equivalent for represented identity | LUI selector test | Generic upper-immediate semantics remain absent. |
| Stopped and unsupported separation | C++ `kSpecialSyscall` and `kSpecialBreak` return `kStopped`; unknown/unimplemented/default paths return `kUnsupported` | Selector returns `None` for stopped, unknown, COP0 unsupported, coprocessor, CACHE, and all other unselected identities | Boundary preserved | Exclusion tests | Stopped and unsupported readiness remain owned by their existing Machine classifiers. |
| Branch/load/store/COP0/ERET/LL/SC exclusion | C++ branch/jump cases mutate control flow and/or link registers; load/store/LL/SC touch memory/reservation; COP0 mutates/reads COP0 or depends on register/context; ERET depends on COP0 context | Selector returns `None` for these identities | Boundary preserved | Exclusion tests | These remain absent because selecting them would imply branch, memory, COP0, ERET, reservation, or generic execute readiness. |
| Mutation boundary | C++ selection is embedded in execute, but the source grouping is identity-visible before helper side effects | Rust selector has no Machine/Cpu state input and no mutation path | Equivalent pure readiness | CPU and Machine no-mutation tests | Selector mutates no GPR, HI/LO, COP0, PC/next PC, Count, RDRAM, SP DMEM, reservation, Cartridge, or power state. |

### Seam 075 Audit Changes

- Added crate-private `CpuLocalExecutedHelperFamily`,
  `CpuLocalExecutedHelperSelection`, and
  `select_cpu_local_executed_helper` in `cpu/instruction.rs`.
- Added a crate-private re-export from `cpu.rs` for future CPU-owned use and
  module tests only; no public Rust API was added.
- Represented helper families:
  - `NoEffectSync`: `SpecialSync`
  - `SpecialShift`: `SpecialSll`, `SpecialSrl`, `SpecialSra`, `SpecialSllv`,
    `SpecialSrlv`, `SpecialSrav`, `SpecialDsll`, `SpecialDsrl`,
    `SpecialDsra`, `SpecialDsll32`, `SpecialDsrl32`, `SpecialDsra32`,
    `SpecialDsllv`, `SpecialDsrlv`, `SpecialDsrav`
  - `SpecialBitwiseLogical`: `SpecialAnd`, `SpecialOr`, `SpecialXor`,
    `SpecialNor`
  - `SpecialHiLoTransfer`: `SpecialMfhi`, `SpecialMthi`, `SpecialMflo`,
    `SpecialMtlo`
  - `SpecialNonTrappingInteger`: `SpecialAddu`, `SpecialSubu`,
    `SpecialDaddu`, `SpecialDsubu`, `SpecialSlt`, `SpecialSltu`
  - `SpecialTrappingInteger`: `SpecialAdd`, `SpecialSub`, `SpecialDadd`,
    `SpecialDsub`
  - `ImmediateTrappingInteger`: `Addi`, `Daddi`
  - `ImmediateNonTrappingInteger`: `Addiu`, `Daddiu`
  - `ImmediateComparison`: `Slti`, `Sltiu`
  - `ImmediateBitwiseLogical`: `Andi`, `Ori`, `Xori`
  - `UpperImmediateLui`: `Lui`
- Excluded stopped identities: `SpecialSyscall` and `SpecialBreak`.
- Excluded unsupported/known-unimplemented identities: unknown primary,
  SPECIAL unknown, REGIMM unknown, COP0 unimplemented/invalid-context
  identities, coprocessor identities, coprocessor memory identities, and CACHE.
- Excluded branch/load/store/COP0/ERET/LL/SC identities, because those require
  branch/control-flow behavior, memory access, reservation behavior, COP0
  execution, ERET context, or step cadence that is not owned by this seam.
- Excluded SPECIAL trap and multiply/divide identities because their side
  effects/outcomes are not the already sealed helper families represented by
  this selector.
- Added selector tests for every represented family and a broad exclusion set.
- Added CPU and Machine no-mutation tests proving selection mutates no CPU,
  COP0, PC/next PC, Count, GPR, HI/LO, RDRAM, SP DMEM, CpuRdramReservation,
  Cartridge, or power state.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic step result shape, execution-helper calls,
  arithmetic-overflow exception entry calls, PC/next PC commit call, Count
  advancement call, fetch/decode/identify loop, branch/link/delay-slot
  behavior, load/store behavior, COP0 execution, ERET execution, memory map,
  bus, device routing, SDL/window runtime, host shell, or C++ source changes.

## Machine Unsupported-Instruction Control-Flow Rollback Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Rollback source snapshot | `src/core/machine_cpu.cpp` `step_cpu_instruction` captures `current_pc = cpu_pc_` and `current_next_pc = cpu_next_pc_` before fetch/decode/execute | `cpu/scalars.rs` `Cpu::capture_control_flow` returns `CpuControlFlowSnapshot` | Equivalent for represented `pc`/`next_pc` only | `control_flow_snapshot_captures_current_pc_and_next_pc` | The Rust snapshot records no Count, COP0, GPR, memory, cartridge, or reservation state. |
| Speculative step staging context | C++ sets `cpu_next_pc_ = sequential_instruction_address(current_next_pc)` before `execute_cpu_instruction` | `Cpu::stage_next_sequential_pc_for_step` | Equivalent for pre-execute next-PC staging only | Step next-PC staging tests; source inspection | Outcome commit/restore choice, branch-likely annul, Count cadence, and full step remain step-owned. |
| Unsupported rollback trigger | C++ runs rollback when `execute_cpu_instruction` returns `CpuInstructionExecutionResult::kUnsupported` | Represented unsupported application exists separately; broad execute-time unsupported remains absent | Narrow represented subset earned | Source inspection | Rust connects represented unsupported production to rollback through `Machine::step`; broader execute-time unsupported behavior remains absent. |
| PC restore | C++ assigns `cpu_pc_ = current_pc` on unsupported result | `Cpu::restore_control_flow` writes snapshot `pc` | Equivalent primitive | `control_flow_restore_restores_only_pc_and_next_pc`; machine no-mutation test | This is restore to a supplied snapshot, not normal PC advancement. |
| next PC restore | C++ assigns `cpu_next_pc_ = current_next_pc` on unsupported result | `Cpu::restore_control_flow` writes snapshot `next_pc` | Equivalent primitive | Control-flow restore tests | This is restore to a supplied snapshot, not next-PC cadence. |
| Count behavior | C++ does not call `advance_cop0_count_after_committed_instruction` before returning `kUnsupported` | Snapshot/restore does not include or mutate COP0 Count | Equivalent for no Count rollback/mutation in the primitive | Control-flow restore tests | Normal Count cadence remains absent and step-owned. |
| COP0 behavior | The unsupported result rollback block restores only `cpu_pc_` and `cpu_next_pc_`; it does not restore COP0 fields | Snapshot/restore does not include or mutate COP0 | Equivalent primitive | Machine no-mutation test with non-default COP0 state | COP0 exception entry and Count cadence remain separate seams. |
| GPR/HI/LO behavior | The unsupported rollback block restores only `cpu_pc_` and `cpu_next_pc_` | Snapshot/restore does not include or mutate GPR, HI, or LO | Equivalent primitive | Scalar and machine no-mutation tests | No instruction writeback or rollback of register effects is added. |
| RDRAM/SP DMEM behavior | The unsupported rollback block restores only `cpu_pc_` and `cpu_next_pc_` | Snapshot/restore cannot access Machine-owned RDRAM or SP DMEM | Equivalent primitive | Machine no-mutation test | No memory rollback, SP device behavior, or savestate is added. |
| Reservation/Cartridge behavior | The unsupported rollback block restores only `cpu_pc_` and `cpu_next_pc_` | Snapshot/restore cannot access Machine-owned reservation or Cartridge | Equivalent primitive | Machine no-mutation test | No reservation invalidation, cartridge mutation, or generic Machine rollback is added. |
| Fault/catch rollback relationship | C++ execution-time fault catch blocks also restore `cpu_pc_`/`cpu_next_pc_` before conversion or rethrow | Same primitive may support future source-backed step fault rollback, but no step wiring exists | Readiness only | Source inspection | This pass seals the primitive, not fault handling or generic exception machinery. |
| ERET unsupported context | C++ can return `kUnsupported` for unsupported ERET context before speculative PC movement | No Rust rollback trigger or ERET context behavior | Blocked | Source inspection | That path depends on COP0/ERET step context and is not represented by identity-only unsupported readiness. |
| Branch-delay/exception dependency | The `kUnsupported` rollback block itself restores only the saved pair; branch-delay and exception state are not inputs to that assignment | Snapshot/restore takes no branch-delay or exception context | Equivalent for primitive only | Source inspection; tests | Future step seams must still audit branch-delay and exception contexts before using the primitive. |
| API visibility | C++ rollback is private to `Machine::step_cpu_instruction` | `CpuControlFlowSnapshot`, `capture_control_flow`, and `restore_control_flow` are crate-private | Equivalent ownership boundary | Rust API inspection | No public savestate, rollback, or step API is exposed. |
| Mutation boundary | C++ rollback mutates only public control-flow state after unsupported execute result | Rust restore mutates only `Cpu` `pc` and `next_pc` from an explicit snapshot | Equivalent primitive | Control-flow restore tests | No fetch, decode, identify, execute, exception entry, memory map, bus, or device behavior is invoked. |

### Seam 056 Audit Changes

- Added crate-private `CpuControlFlowSnapshot`,
  `Cpu::capture_control_flow`, and `Cpu::restore_control_flow` in
  `cpu/scalars.rs`.
- Captured and restored exactly `pc` and `next_pc`, matching the source-clear
  C++ unsupported rollback assignment.
- Added CPU scalar tests for snapshot capture and restore, plus a Machine test
  proving restore preserves non-control-flow CPU state, COP0 Count/status/EPC/
  BadVAddr/Cause/branch-delay facts, RDRAM, SP DMEM, CpuRdramReservation, and
  Cartridge facts.
- Did not add `Machine::step`, `Cpu::step`, `execute_cpu_instruction`, a
  placeholder execute API, a generic `MachineStepResult`, generic rollback,
  savestate machinery, outcome-dependent PC/next PC commit cadence, Count
  cadence, instruction side effects, instruction writeback, exceptions, memory
  map, bus, device routing, SDL/window runtime, host shell, or C++ source
  changes. Seam 063 later seals only the narrow committed-control-flow helper,
  still without outcome wiring or step.

## Machine Step PC/next PC and Count Cadence Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Interrupt ordering | `src/core/machine_cpu.cpp` `step_cpu_instruction` calls `try_enter_local_interrupt()` before `current_pc`/`current_next_pc` capture, fetch, decode, identify, execute, or Count advancement | No Rust interrupt handling | Blocked | Source inspection | A local interrupt returns `kInterrupted` before normal step cadence. Rust still has no interrupts. |
| Pre-step control-flow capture | `step_cpu_instruction` captures `current_pc = cpu_pc_` and `current_next_pc = cpu_next_pc_` after the interrupt check and before fetch | `Cpu::capture_control_flow` | Equivalent for `pc`/`next_pc` value capture only | Control-flow snapshot tests | The snapshot records no Count, COP0, GPR, memory, reservation, or cartridge state. |
| Fetch-fault ordering | C++ fetch/decode/identify happen before normal speculative next-PC staging; selected fetch faults can enter local AdEL and return `kException` before Count advancement | Rust fetch APIs return errors; represented `Machine::step` connects selected fetch-fault action to entry through the producer/applicator | Equivalent for represented selected fetch faults | Fetch, selection, entry, and step tests | Fetch APIs remain non-mutating; step composition owns selected fetch-fault action production/application. |
| ERET ordering | C++ handles `kEret` before normal speculative next-PC staging; unsupported ERET returns `kUnsupported`, successful ERET returns from local entry and advances Count | No Rust ERET behavior | Blocked | Source inspection | ERET cadence remains coupled to COP0/exception-return step context. |
| Pre-execute next-PC staging | C++ non-ERET path assigns `cpu_next_pc_ = sequential_instruction_address(current_next_pc)` immediately before `execute_cpu_instruction` | `Cpu::stage_next_sequential_pc_for_step` | Equivalent for the isolated pre-execute next-PC staging primitive | Step next-PC staging tests | Rust advances only `next_pc` by one sequential instruction. It does not fetch, decode, identify, execute, commit, or step. |
| Pre-execute PC behavior | C++ does not assign `cpu_pc_` during the pre-execute next-PC staging assignment | `Cpu::stage_next_sequential_pc_for_step` leaves `pc` unchanged | Equivalent for the primitive | Scalar staging test; Machine no-mutation test | Normal commit later may assign `cpu_pc_ = current_next_pc`, but that is outcome-dependent and not earned. |
| Sequential arithmetic | `sequential_instruction_address(CpuAddress address)` returns `address + 4u` as `std::uint32_t` | `sequential_instruction_address` uses `u32::wrapping_add(4)` | Equivalent | Wrapping staging tests | Wrapping at the 32-bit boundary is source-clear. |
| Count during pre-execute staging | C++ does not call `advance_cop0_count_after_committed_instruction()` in the staging assignment | `Cpu::stage_next_sequential_pc_for_step` does not touch COP0 Count | Equivalent for the primitive | Scalar and Machine no-mutation tests | Count cadence remains outcome-owned by step. |
| Normal committed outcome | C++ assigns `cpu_pc_ = current_next_pc`, keeps the already-staged or execution-mutated `cpu_next_pc_`, advances Count, and returns `kStepped` | `Cpu::commit_staged_step_control_flow` plus separate `Cpu::advance_count_for_committed_step` exist; no step wiring exists | Equivalent primitives only | Commit and Count tests; source inspection | Commit, Count advance, and cadence plan remain separate. The step trigger and execute result handling are absent. |
| Stopped outcome | C++ uses the same committed PC/Count path, then returns `kStopped` | `Cpu::commit_staged_step_control_flow`; stopped readiness exists separately | Equivalent primitive plus pure readiness only | Commit, stopped readiness, and cadence plan tests | Stopped execution result handling remains execute/step-owned. |
| Unsupported outcome | C++ restores `cpu_pc_ = current_pc` and `cpu_next_pc_ = current_next_pc`, then returns `kUnsupported` without Count advancement | `Cpu::restore_control_flow` can restore the pair; represented `Machine::step` uses it for represented unsupported identities | Equivalent for represented unsupported subset | Rollback and step tests; source inspection | The full generic step result remains absent. |
| Execution-time exception/fault outcome | C++ catch blocks restore `cpu_pc_`/`cpu_next_pc_` before converting selected faults to local exceptions or rethrowing; Count is not advanced on those paths | Restore primitive and selected address-error entry seams exist separately | Readiness only | Source inspection | Rust does not connect execute faults to step outcomes. |
| Branch-likely not-taken annul | C++ sets `cpu_pc_` to the skipped delay-slot PC, sets `cpu_next_pc_` to the next sequential address, advances Count, and returns `kStepped` | No Rust branch-likely annul behavior | Blocked | Source inspection | Branch-likely annul remains branch/execute step behavior, not a generic cadence primitive. |
| Branch/delay-slot target cadence | C++ execute paths may mutate `cpu_next_pc_` while `cpu_pc_` still holds the current instruction address | No Rust branch or delay-slot execution behavior | Blocked | Source inspection | Target formation and delay-slot state remain future instruction/control-flow seams. |
| Count helper | `advance_cop0_count_after_committed_instruction()` increments `cop0_count_` and sets `cop0_timer_interrupt_pending_` when Count equals Compare | `cpu/cop0.rs` `Cpu::advance_count_for_committed_step` | Equivalent primitive; not wired to step | Count advancement tests; source inspection | Count mutation is now sealed as a narrow COP0 primitive, but step outcome wiring remains absent. |
| Count on fetch-fault AdEL | Selected fetch-fault local exceptions return `kException` before committed-instruction Count advancement | Existing narrow fetch-fault entry does not mutate Count | Equivalent for narrow entry only | Entry tests; source inspection | No normal Count cadence is added. |
| Count on `kUnsupported` | C++ returns `kUnsupported` before Count advancement | Unsupported readiness and rollback primitives do not mutate Count | Equivalent for readiness primitives | Unsupported and rollback tests | No Count rollback is needed because no Count increment has occurred. |
| Count on `kStopped` | C++ advances Count before returning `kStopped` | Count primitive exists but stopped outcome wiring is absent | Blocked | Source inspection; Count tests | Requires execute result and committed-step outcome ownership. |
| Count on `kInterrupted` | C++ returns `kInterrupted` before capture/fetch and before Count advancement | No Rust interrupt behavior | Blocked | Source inspection | Interrupt processing remains absent. |
| Count on `kException` | C++ local synchronous exception returns do not call committed-instruction Count advancement | Narrow exception entries do not tick Count | Equivalent for existing narrow entries only | Address-error tests; source inspection | Full exception result handling remains absent. |
| Count on successful ERET | C++ successful ERET return advances Count before returning `kStepped` | Count primitive exists but ERET behavior and wiring are absent | Blocked | Source inspection; Count tests | ERET is step/COP0-return behavior and remains absent. |
| Step result shape | C++ `CpuInstructionStepResult` distinguishes `kStepped`, `kStopped`, `kUnsupported`, `kInterrupted`, and `kException` | No Rust generic step result type | Not earned | Source inspection | A fake step/result shape was avoided. |

### Seam 057 Audit Changes

- Added crate-private `Cpu::stage_next_sequential_pc_for_step` in
  `cpu/scalars.rs`.
- Mirrored the C++ pre-execute staging assignment
  `cpu_next_pc_ = sequential_instruction_address(current_next_pc)` as
  `next_pc = next_pc.wrapping_add(4)`, while leaving `pc` unchanged.
- Added CPU scalar tests for next-PC-only staging and 32-bit wrapping
  arithmetic, plus a Machine preservation test proving no Count, COP0, GPR,
  RDRAM, SP DMEM, reservation, or Cartridge mutation.
- Added no Count cadence primitive in seam 057 because the C++ Count helper
  increments Count and may set timer-pending state only on source-clear
  committed step outcomes or successful ERET, not as a standalone pre-execute
  primitive. Seam 062 later seals that helper as a separate COP0 primitive
  without wiring it to step.
- Added no `Machine::step`, `Cpu::step`, `execute_cpu_instruction`,
  placeholder step API, generic step result shape, normal commit/annul cadence
  at that time, branch/delay-slot behavior, interrupt handling, ERET behavior,
  generic exception machinery, memory map, bus, device routing, SDL/window
  runtime, host shell, or C++ source changes. Seam 063 later seals only the
  normal non-annul commit primitive without branch/annul behavior or step.

## Machine Committed-Step Outcome and Count Cadence Plan Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Cadence plan owner | `src/core/machine_cpu.cpp` `step_cpu_instruction` outcome paths | `machine.rs` `MachineStepCadencePlan`; `classify_machine_step_cadence` | Equivalent pure plan only | Cadence plan tests; source inspection | The plan mutates no state and is not a step result type. |
| Control-flow action names | C++ commits, restores, preserves local exception vector state, returns before cadence, or uses execute-owned ERET/branch-likely control flow | `MachineStepControlFlowAction::{CommitStaged,RestoreSnapshot,PreserveExceptionVector,ReturnBeforeCadence,BlockedByEretReturn,BlockedByBranchLikelyAnnul}` | Equivalent plan vocabulary | Cadence plan tests | Blocked variants intentionally name unearned ERET and branch-likely control-flow behavior without implementing it. |
| Count action names | C++ either calls or does not call `advance_cop0_count_after_committed_instruction()` | `MachineStepCountAction::{Advance,DoNotAdvance}` | Equivalent plan vocabulary | Cadence plan tests | This is not Count mutation. |
| Normal committed instruction | C++ after non-branch-likely execution sets `cpu_pc_ = current_next_pc`, leaves `cpu_next_pc_` as staged/execution-mutated, calls `advance_cop0_count_after_committed_instruction()`, and returns `kStepped` | `MachineStepCadenceSource::CommittedInstruction` -> `CommitStaged` + `Advance`; `Cpu::commit_staged_step_control_flow` and `Cpu::advance_count_for_committed_step` exist separately | Equivalent pure plan plus separate primitives | `machine_step_cadence_plan_maps_source_clear_outcomes`; commit and Count tests | Rust does not wire commit or Count advancement into step. |
| `kStopped` outcome | C++ uses the same committed PC/Count path, then returns `kStopped` | `StoppedInstruction` -> `CommitStaged` + `Advance`; commit and Count primitives exist separately | Equivalent pure plan plus separate primitives | Cadence plan, stopped readiness, commit, and Count tests | Stop/trap execution behavior remains absent. |
| `kUnsupported` outcome | C++ restores `cpu_pc_ = current_pc` and `cpu_next_pc_ = current_next_pc`, then returns `kUnsupported` without Count advancement | `UnsupportedInstruction` -> `RestoreSnapshot` + `DoNotAdvance` | Equivalent plan over already-earned rollback primitive | Cadence plan test; rollback tests | The plan does not trigger rollback and does not add execute. |
| `kInterrupted` outcome | C++ `try_enter_local_interrupt()` returns before capture/fetch/normal cadence and before Count advancement | `InterruptedBeforeFetch` -> `ReturnBeforeCadence` + `DoNotAdvance` | Equivalent pure plan only | Cadence plan test; source inspection | Interrupt handling remains absent. |
| Local exception outcome | C++ local exception paths preserve the exception-vector `pc`/`next_pc` installed by entry and do not advance Count | `EnteredException` -> `PreserveExceptionVector` + `DoNotAdvance` | Equivalent pure plan only | Cadence plan test; source inspection | Generic exception result handling remains absent. |
| Selected fetch-fault AdEL outcome | C++ selected fetch faults enter local AdEL before speculative mutation and return `kException` without Count advancement | `FetchAddressErrorException` -> `PreserveExceptionVector` + `DoNotAdvance` | Equivalent pure plan only | Cadence plan test; fetch-fault entry tests | Fetch APIs remain non-mutating. |
| Successful ERET | C++ handles ERET before normal speculative staging, calls `return_from_local_interrupt_entry()`, then advances Count and returns `kStepped` | `SuccessfulEret` -> `BlockedByEretReturn` + `Advance`; Count primitive exists separately | Equivalent Count plan; control-flow blocked | ERET/branch blocked cadence test; Count tests; source inspection | Rust still has no ERET behavior and does not wire Count advancement to ERET. |
| Branch-likely annul | C++ not-taken branch-likely sets `cpu_pc_` to the skipped delay-slot PC, sets `cpu_next_pc_` sequentially after it, advances Count, and returns `kStepped` | `BranchLikelyAnnul` -> `BlockedByBranchLikelyAnnul` + `Advance` | Equivalent Count plan; control-flow blocked | ERET/branch blocked cadence test; source inspection | Branch-likely annul remains execute-owned branch behavior. |
| Count helper mutation | `advance_cop0_count_after_committed_instruction()` increments Count with `++cop0_count_` and sets `cop0_timer_interrupt_pending_ = true` when Count equals Compare | `cpu/cop0.rs` `Cpu::advance_count_for_committed_step` | Equivalent primitive; not wired to outcomes | Count tests; source inspection | Count/Compare/timer-pending mutation is source-clear and sealed as a COP0 helper. Step cadence still decides when to call it later. |
| Count wrapping | C++ `std::uint32_t` pre-increment wraps by unsigned arithmetic | `Cpu::advance_count_for_committed_step` uses `wrapping_add(1)` | Equivalent | Count wrapping test | The primitive performs arithmetic only; no step or interrupt processing is added. |
| Count restored by outcomes | C++ does not restore Count in observed cadence paths; it either has not advanced or advances once on committed paths | No Rust Count restore behavior | Equivalent absence for plan | Source inspection | No Count rollback primitive is added. |
| Commit primitive | C++ commit assigns `cpu_pc_ = current_next_pc` after execute result selection and leaves the staged/execution-mutated `cpu_next_pc_` intact | `cpu/scalars.rs` `Cpu::commit_staged_step_control_flow` | Equivalent primitive; not wired to step | Control-flow commit tests; source inspection | The helper takes an explicit snapshot, sets `pc = snapshot.next_pc`, and preserves already-staged `next_pc`. Execute outcome selection remains absent. |
| Generic step result shape | C++ has `CpuInstructionStepResult`, but its full meaning depends on fetch, execute, interrupts, exceptions, ERET, stop, unsupported, Count, and PC cadence | No Rust `MachineStepResult` or `MachineStepError` | Not earned | Source inspection | A fake generic result shape was avoided. |
| Mutation boundary | C++ step mutates state on committed, exception, interrupt, ERET, and branch-likely paths | `classify_machine_step_cadence` returns a value only | Equivalent for pure planning only | `machine_step_cadence_plan_performs_no_machine_mutation` | No Machine, Cpu, COP0, PC/next PC, Count, GPR, RDRAM, SP DMEM, reservation, Cartridge, fetch, decode, identify, execute, or exception mutation. |

### Seam 058 Audit Changes

- Added `MachineStepCadenceSource`, `MachineStepControlFlowAction`,
  `MachineStepCountAction`, `MachineStepCadencePlan`, and
  `classify_machine_step_cadence` in `machine.rs`.
- Exported only the pure cadence plan/value layer through `lib.rs`.
- Represented source-clear cadence cases: committed instruction, stopped
  instruction, unsupported instruction, interrupted before fetch, entered
  exception, selected fetch-fault AdEL exception, successful ERET, and
  branch-likely annul.
- Kept successful ERET and branch-likely annul control flow blocked by explicit
  action variants while preserving their source-clear Count-advance plan.
- Added cadence plan tests for action mapping and no Machine/Cpu/COP0/PC/
  next PC/Count/GPR/RDRAM/SP DMEM/reservation/Cartridge mutation.
- Added no `Machine::step`, `Cpu::step`, `execute_cpu_instruction`, placeholder
  step API, generic step result shape, pc/next PC commit primitive at that
  time, Count outcome wiring, branch/delay-slot behavior, interrupt handling,
  ERET behavior, generic exception machinery, memory map, bus, device routing,
  SDL/window runtime, host shell, or C++ source changes. Seam 063 later seals
  only the narrow commit primitive without wiring it to step.

## COP0 Count / Compare / Timer-Pending Cadence Primitive Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Count advancement owner | `src/core/machine_cpu.cpp` `Machine::advance_cop0_count_after_committed_instruction` | `cpu/cop0.rs` `Cpu::advance_count_for_committed_step` | Equivalent crate-private primitive | Count advancement tests; source inspection | Rust owns only the helper needed by future committed step outcomes. It is not called by fetch, decode, identity, unsupported/stopped/executed readiness, cadence planning, or a step loop. |
| Count arithmetic | C++ uses `++cop0_count_` on `std::uint32_t` | Rust uses `wrapping_add(1)` on `u32` | Equivalent | `count_advance_for_committed_step_increments_and_latches_timer_after_increment`; wrapping test | Count increments by one and wraps from `0xffff_ffff` to `0x0000_0000`. |
| Compare equality timing | C++ compares `cop0_count_ == cop0_compare_` after increment | Rust compares `count == compare` after the wrapping increment | Equivalent | Timer latch tests; source inspection | Pre-increment equality alone does not latch timer-pending in this primitive. |
| Timer-pending latch | C++ sets `cop0_timer_interrupt_pending_ = true` when post-increment Count equals Compare | Rust sets `timer_interrupt_pending = true` on the same post-increment equality | Equivalent | Timer latch tests | This is a latch, not interrupt delivery. |
| Timer-pending non-match behavior | C++ does not clear `cop0_timer_interrupt_pending_` in the Count helper when Count does not equal Compare | Rust leaves timer-pending unchanged on non-match | Equivalent | `count_advance_for_committed_step_preserves_timer_pending_when_not_matching_compare` | Existing pending remains pending; unset remains unset when there is no match. |
| Compare write clearing | C++ `write_cop0_compare` assigns Compare and clears timer-pending | No Rust Compare write behavior | Intentionally absent | Source inspection | Compare writes are instruction/COP0-write execution behavior and remain a future seam. |
| Interrupt processing | C++ Count helper only marks pending; interrupt delivery is elsewhere through local interrupt checks and Cause reads | Rust Count helper only mutates Count and timer-pending | Equivalent absence | Mutation tests; source inspection | No interrupt is processed, no exception is entered, and no Cause read/result is synthesized. |
| Committed outcomes planned to call Count later | C++ calls the helper for normal `kExecuted`/`kStepped`, `kStopped`, successful ERET, and branch-likely not-taken annul | `MachineStepCountAction::Advance` names those future calls; no wiring exists | Equivalent readiness split | Cadence plan tests; source inspection | Outcome classification and cadence wiring remain separate from the primitive. |
| Outcomes that do not advance Count | C++ does not call the helper for `kUnsupported`, selected fetch-fault/local exceptions, execution-time local exceptions, or `kInterrupted` before fetch | `MachineStepCountAction::DoNotAdvance` names those cases; no wiring exists | Equivalent readiness split | Cadence plan tests; source inspection | The Count primitive is not called by non-committed readiness paths. |
| State mutation boundary | C++ helper mutates only Count and timer-pending | Rust helper mutates only Count and timer-pending | Equivalent | CPU and Machine preservation tests | It mutates no pc/next PC, GPR, HI/LO, BadVAddr, EPC, exception code, branch-delay flag, status, software interrupt pending, RDRAM, SP DMEM, reservation, or Cartridge. |
| Step boundary | C++ invokes the helper from `step_cpu_instruction`; represented Rust `Machine::step` reaches it only through sealed applicators | No `Cpu::step` or generic timer machinery | Boundary preserved | API/source inspection; tests | Count advancement readiness alone is not execute, pc/next PC commit, or generic timer machinery. |

### Seam 062 Audit Changes

- Added crate-private `Cpu::advance_count_for_committed_step` and a private
  COP0 helper in `cpu/cop0.rs`.
- Mirrored the C++ helper exactly: wrapping Count increment, post-increment
  Compare equality, timer-pending latch on equality, and no clearing on
  non-match.
- Added CPU tests for increment, wrapping, timer latch timing, pending
  preservation, and no mutation of pc/next PC, GPR, HI/LO, BadVAddr, EPC,
  exception code, branch-delay flag, status, software interrupt pending, or
  Compare.
- Added a Machine test proving the primitive does not mutate RDRAM, SP DMEM,
  CpuRdramReservation, Cartridge, power state, or non-Count CPU state.
- Added no `Machine::step`, `Cpu::step`, `execute_cpu_instruction`, placeholder
  step API, pc/next PC commit wiring, Count outcome wiring, interrupt
  processing, exception entry, Compare writes, generic timer machinery, memory
  map, bus, device routing, SDL/window runtime, host shell, or C++ source
  changes.

## CPU Committed-Step Control-Flow Commit Primitive Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Current PC capture | `src/core/machine_cpu.cpp` `step_cpu_instruction` stores `current_pc = cpu_pc_` after the local interrupt check and before fetch | `Cpu::capture_control_flow` stores snapshot `pc` | Equivalent existing primitive | Snapshot tests; source inspection | The new commit primitive consumes the same snapshot type; it does not capture by itself. |
| Current next-PC capture | `step_cpu_instruction` stores `current_next_pc = cpu_next_pc_` at the same point | `CpuControlFlowSnapshot::next_pc` | Equivalent existing primitive | Snapshot tests; source inspection | This value is the future committed `pc` for normal non-annul outcomes. |
| Speculative next-PC staging | C++ non-ERET path assigns `cpu_next_pc_ = sequential_instruction_address(current_next_pc)` before execute | `Cpu::stage_next_sequential_pc_for_step` | Equivalent existing primitive | Staging tests; source inspection | Branch or jump execution may later mutate `next_pc`; the commit helper preserves whatever value is already staged. |
| Normal commit rule | C++ after non-branch-likely `kExecuted` assigns `cpu_pc_ = current_next_pc` and leaves `cpu_next_pc_` intact | `Cpu::commit_staged_step_control_flow` sets `pc = snapshot.next_pc` and does not write `next_pc` | Equivalent primitive | Commit tests; source inspection | This is only the control-flow write. Count advancement and result return remain separate. |
| `kStopped` commit rule | C++ uses the same `cpu_pc_ = current_next_pc` committed path before returning `CpuInstructionStepResult::kStopped` | Same `Cpu::commit_staged_step_control_flow` primitive | Equivalent primitive | Commit, stopped-readiness, and cadence-plan tests; source inspection | SYSCALL/BREAK stopped readiness remains pure and does not call this helper. |
| `next_pc` handling | C++ leaves the already-staged or execution-mutated `cpu_next_pc_` value alone on the normal commit line | Rust helper preserves `next_pc` exactly | Equivalent | `commit_staged_step_control_flow_preserves_already_staged_next_pc` | Preserving staged `next_pc` is not branch target formation or branch behavior. |
| Count separation | C++ calls `advance_cop0_count_after_committed_instruction()` after `cpu_pc_ = current_next_pc` | Rust helper does not call `Cpu::advance_count_for_committed_step` | Equivalent separation | Commit no-mutation tests; Count tests | Count advancement is sealed as a separate primitive and remains unwired to step. |
| Unsupported restore separation | C++ `kUnsupported` restores `cpu_pc_ = current_pc` and `cpu_next_pc_ = current_next_pc` instead of committing | `Cpu::restore_control_flow` remains the separate rollback primitive | Equivalent split | Restore tests; source inspection | Commit and restore are distinct helpers over the same snapshot value. |
| Exception-vector preservation | C++ selected fetch-fault AdEL and other local exceptions preserve the exception-vector state installed by entry and do not reach normal commit | Represented exception paths in `Machine::step` delegate to sealed applicators and do not call normal commit | Equivalent for represented exception paths | Entry, cadence-plan, and step tests; source inspection | Exception entry remains separate from normal committed control flow. |
| ERET separation | C++ successful ERET runs before normal staging/commit and uses `return_from_local_interrupt_entry()` instead | No Rust ERET behavior | Blocked separately | Source inspection | The commit helper does not encode ERET return behavior. |
| Branch-likely annul separation | C++ not-taken branch-likely uses a distinct annul path that sets `pc` to the skipped delay-slot PC and restages `next_pc` | No Rust branch-likely annul behavior | Blocked separately | Source inspection | The commit helper covers only the normal non-annul committed path. |
| Mutation boundary | C++ normal commit line mutates public control-flow `pc`; next-PC was already staged earlier | Rust helper mutates only `pc` and preserves `next_pc` | Equivalent primitive | CPU and Machine preservation tests | It mutates no Count/timer-pending/COP0/GPR/HI/LO/RDRAM/SP DMEM/reservation/Cartridge and performs no fetch/decode/identify/execute. |
| Step boundary | C++ calls commit from `step_cpu_instruction`; represented Rust `Machine::step` reaches commit only through sealed applicators | No `Cpu::step` or generic step result machinery | Boundary preserved | API/source inspection; tests | The primitive itself is not CPU step, execute, Count cadence, branch behavior, interrupt/exception/ERET handling, or generic step result machinery. |

### Seam 063 Audit Changes

- Added crate-private `Cpu::commit_staged_step_control_flow` in
  `cpu/scalars.rs`.
- Mirrored the source-clear C++ normal committed-control-flow assignment:
  `pc = snapshot.next_pc`, while preserving whatever `next_pc` value was
  already staged by pre-execute sequencing or future execute-owned
  control-flow behavior.
- Added CPU scalar tests for the normal commit rule, preserved staged `next_pc`,
  and wrapped staged `next_pc` values.
- Added a Machine preservation test proving the primitive does not mutate
  Count, timer-pending, COP0 status/software interrupt/EPC/BadVAddr/exception
  code/branch-delay state, HI/LO, GPRs, RDRAM, SP DMEM, CpuRdramReservation,
  Cartridge, or power state.
- Added no `Machine::step`, `Cpu::step`, `execute_cpu_instruction`,
  placeholder step API, Count call from commit, branch/link/delay-slot
  behavior, branch-likely annul behavior, ERET behavior, interrupt processing,
  exception processing, generic step result machinery, memory map, bus, device
  routing, SDL/window runtime, host shell, or C++ source changes.

## CPU-Local Executed-Helper Invocation Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Invocation source owner | `src/core/machine_cpu.cpp` `Machine::execute_cpu_instruction` switch cases for already sealed CPU-local identities | `cpu/instruction.rs` `Cpu::invoke_cpu_local_executed_helper` | Equivalent for invoking selected sealed helper families only | Invocation tests; source inspection | Rust takes `CpuInstructionFields` plus `CpuLocalExecutedHelperSelection`; it performs no fetch, decode, identify, or selector call internally. |
| Invokable families | C++ direct cases for `kSpecialSync`, SPECIAL shift, SPECIAL bitwise logical, SPECIAL HI/LO transfer, SPECIAL non-trapping integer, SPECIAL trapping integer, ADDI/DADDI, ADDIU/DADDIU, SLTI/SLTIU, ANDI/ORI/XORI, and LUI | `CpuLocalExecutedHelperFamily` variants are dispatched to the already sealed Rust helpers | Equivalent for represented families | Representative invocation tests | No represented selected family remains excluded. |
| Executed outcome | C++ represented successful cases return `CpuInstructionExecutionResult::kExecuted` | `CpuLocalExecutedHelperInvocationOutcome::Executed` | Equivalent local outcome | Invocation tests | The outcome is local to helper invocation and is not a generic step result. |
| Arithmetic overflow outcome | C++ ADD/SUB/DADD/DSUB/ADDI/DADDI overflow throws `kSignedArithmeticOverflow` inside execute; `step_cpu_instruction` catches it and may enter COP0 exception state | `CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow` wrapping the existing SPECIAL or immediate overflow value | Equivalent pre-entry readiness | Overflow invocation tests | Rust returns overflow before writeback and does not call `Cpu::enter_arithmetic_overflow_exception`; exception entry remains a separate unwired primitive. |
| Stopped and unsupported separation | C++ SYSCALL/BREAK return `kStopped`; unknown/unimplemented/default paths return `kUnsupported` | Invoker requires a `CpuLocalExecutedHelperSelection`; selector returns `None` for stopped and unsupported identities | Boundary preserved | Exclusion tests | Stopped and unsupported readiness remain Machine-owned classifiers, not invoker outcomes. |
| Branch/load/store/COP0/ERET/LL/SC exclusion | C++ branch/jump mutate control flow; load/store/LL/SC touch memory/reservation; COP0/ERET depend on COP0 context; traps and multiply/divide remain separate families | No selector value exists for these identities, so the invoker has no source-clear entry for them | Boundary preserved | Exclusion tests | These remain absent to avoid branch, memory, COP0, ERET, LL/SC, trap, multiply/divide, bus, or device overclaiming. |
| Mutation boundary | C++ represented helper cases mutate only their CPU-local target or throw overflow before writeback; step owns cadence, Count, rollback, and exception entry | Rust invoker calls exactly one CPU-local helper and returns local outcome | Equivalent bounded mutation | CPU and Machine preservation tests | Invoker does not mutate PC/next PC/Count/COP0 unrelated state/RDRAM/SP DMEM/CpuRdramReservation/Cartridge, does not call Machine, and does not create step or generic execute machinery. |

### Seam 076 Audit Changes

- Added crate-private `CpuLocalExecutedHelperInvocationOutcome`,
  `CpuLocalExecutedHelperExecutedInstruction`,
  `CpuLocalExecutedHelperArithmeticOverflow`, and
  `CpuLocalExecutedHelperInvocationError` in `cpu/instruction.rs`.
- Added crate-private `Cpu::invoke_cpu_local_executed_helper`, taking already
  decoded fields plus a `CpuLocalExecutedHelperSelection`.
- The invoker dispatches only selected, already sealed families:
  no-effect `SpecialSync`, SPECIAL shift, SPECIAL bitwise logical, SPECIAL
  HI/LO transfer, SPECIAL non-trapping integer, SPECIAL trapping integer,
  immediate trapping integer, immediate non-trapping integer, immediate
  comparison, immediate bitwise logical, and LUI.
- SPECIAL and immediate trapping overflow returns an arithmetic-overflow
  invocation outcome before writeback. The narrow arithmetic-overflow exception
  entry remains separate and unwired.
- Stopped identities, unsupported identities, branch/load/store/COP0/ERET/LL/SC
  identities, trap identities, and multiply/divide identities remain outside the
  selector and cannot enter this invocation path through source-clear API use.
- Added CPU invocation tests and a Machine preservation test proving invocation
  does not fetch, decode, identify, call Machine, commit cadence, advance Count,
  enter exceptions, mutate unrelated COP0 state, mutate PC/next PC except none,
  mutate RDRAM, mutate SP DMEM, invalidate CpuRdramReservation, mutate
  Cartridge, add `Machine::step`, add `Cpu::step`, or add generic execute.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic `MachineStepResult`, arithmetic-overflow
  exception entry wiring, branch/link/delay-slot behavior, load/store behavior,
  COP0 execution, ERET execution, memory map, bus, device routing,
  SDL/window runtime, host shell, or C++ source changes.

## CPU-Local Invocation Outcome Step-Action Planning Readiness Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Execution-result to step-action source | `src/core/machine_cpu.cpp` `Machine::step_cpu_instruction` after `execute_cpu_instruction` returns or throws | `machine.rs` `classify_cpu_local_invocation_step_action` | Equivalent for pure planning only | Step-action plan tests; source inspection | C++ step owns the mapping from execution-local outcomes to public step behavior. Rust names the future action without mutating state. |
| Successful local execution action | C++ non-branch-likely `kExecuted` reaches `cpu_pc_ = current_next_pc`, `advance_cop0_count_after_committed_instruction()`, and `CpuInstructionStepResult::kStepped` | `MachineCpuLocalInvocationStepActionPlan::CommitControlFlowAndAdvanceCount` with `classify_machine_step_cadence(CommittedInstruction)` | Equivalent plan only | Success mapping tests | Rust references the already sealed committed cadence plan rather than duplicating cadence rules. It does not call `Cpu::commit_staged_step_control_flow` or `Cpu::advance_count_for_committed_step`. |
| Arithmetic overflow action | C++ signed-overflow from ADD/SUB/DADD/DSUB/ADDI/DADDI throws `MachineFaultKind::kSignedArithmeticOverflow`; `step_cpu_instruction` restores `current_pc/current_next_pc` and may call `enter_local_signed_overflow_exception`, returning `kException` without Count advancement | `MachineCpuLocalInvocationStepActionPlan::EnterArithmeticOverflowException` preserving `CpuLocalExecutedHelperArithmeticOverflow` | Equivalent pre-entry planning only | Overflow mapping tests | Rust preserves SPECIAL versus immediate overflow payloads. It does not call `Cpu::enter_arithmetic_overflow_exception`, does not commit control flow, and does not advance Count. |
| Invocation error action | C++ has no public step result for Rust-only selector/invoker invariant failures | `MachineCpuLocalInvocationStepActionPlan::RejectInvocationError` | Rust-only invariant rejection | Rejection mapping test | Rust represents invocation errors only as rejected/internal planning data. They are not C++ step outcomes and are not mapped to `kUnsupported`, `kException`, or `MachineStepResult`. |
| Stopped and unsupported separation | C++ `kStopped` and `kUnsupported` are distinct `execute_cpu_instruction` results handled by separate `step_cpu_instruction` branches | Stopped and unsupported Rust readiness remain `MachineStepStoppedInstruction` and `MachineStepUnsupportedInstruction` classifiers, outside this plan | Boundary preserved | Exclusion tests | SYSCALL/BREAK and unknown/source-clear unsupported identities do not enter the CPU-local invocation path. |
| Branch/load/store/COP0/ERET/LL/SC exclusion | C++ branch-likely annul, control transfer, memory faults, COP0, ERET, and LL/SC have separate step/cadence/memory/COP0 semantics | No selector or plan entry is provided for these identities | Boundary preserved | Exclusion tests | These remain absent to avoid overclaiming branch, memory, COP0, ERET, reservation, bus, or device behavior. |
| Mutation boundary | C++ step performs the actual control-flow, Count, rollback, and exception-entry mutations | Rust classifier consumes only an invocation `Result` and returns a plan value | Equivalent pure boundary | Machine no-mutation test | The plan performs no Machine/Cpu/GPR/HI/LO/COP0/PC/next PC/Count/RDRAM/SP DMEM/reservation mutation, no fetch, no decode, no identify, no execution helper call, and no exception entry. |

### Seam 077 Audit Changes

- Added crate-private `MachineCpuLocalInvocationStepAction`,
  `MachineCpuLocalInvocationStepActionPlan`, and
  `classify_cpu_local_invocation_step_action` in `machine.rs`.
- Successful `CpuLocalExecutedHelperInvocationOutcome::Executed` maps to
  `CommitControlFlowAndAdvanceCount` and carries the existing
  `MachineStepCadencePlan` for `CommittedInstruction`.
- `CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow` maps to
  `EnterArithmeticOverflowException` and preserves the original SPECIAL or
  immediate overflow payload for a future entry seam.
- `CpuLocalExecutedHelperInvocationError` maps only to
  `RejectInvocationError`, documenting Rust-side invariant failure rather than
  pretending it is a C++ step result.
- Added tests proving successful SYNC, GPR writeback, HI/LO transfer,
  immediate, and LUI invocation outcomes map to committed cadence planning;
  SPECIAL and immediate overflow outcomes map to overflow-entry planning;
  stopped/unsupported/branch/load/store/COP0/ERET/LL/SC identities remain
  excluded; and planning mutates no Machine, Cpu, GPR, HI/LO, COP0, PC/next PC,
  Count, RDRAM, SP DMEM, CpuRdramReservation, Cartridge, or power state.
- Added no `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  placeholder execute API, generic `MachineStepResult`, execution helper calls
  as new behavior, cadence primitive calls, Count advancement calls,
  arithmetic-overflow exception entry calls, branch/link/delay-slot behavior,
  load/store behavior, COP0 execution, ERET execution, memory map, bus, device
  routing, SDL/window runtime, host shell, or C++ source changes.

## Committed CPU-Local Success Cadence Composition Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Successful local action accepted | C++ `step_cpu_instruction` reaches the normal committed path only after `execute_cpu_instruction` returns `kExecuted` without a branch-likely annul | `Machine::apply_cpu_local_committed_success_cadence` accepts only `MachineCpuLocalInvocationStepActionPlan::CommitControlFlowAndAdvanceCount` | Equivalent narrow precondition | Success and rejection tests | Rust does not fetch, decode, identify, select, or invoke helpers. The caller must already have a successful CPU-local action plan. |
| Control-flow commit ordering | C++ stages `cpu_next_pc_ += 4` before execute, then normal commit sets `cpu_pc_ = current_next_pc` | `apply_cpu_local_committed_success_cadence` calls `Cpu::commit_staged_step_control_flow(snapshot)` first | Equivalent for already staged non-branch local success | Control-flow commit test | The Rust helper uses the existing `CpuControlFlowSnapshot`; `pc` becomes the snapshot `next_pc` and the already staged `next_pc` remains intact. Branch, jump, link, delay-slot, and branch-likely annul behavior remain absent. |
| Count advancement ordering | C++ calls `advance_cop0_count_after_committed_instruction()` after the normal `cpu_pc_` commit | `apply_cpu_local_committed_success_cadence` calls `Cpu::advance_count_for_committed_step()` after control-flow commit | Equivalent ordering | Count-once and timer-latch tests | Count advances exactly once through the sealed COP0 helper, preserving wrapping and Compare/timer-pending latch semantics. This does not process interrupts. |
| Overflow and rejection excluded | C++ signed overflow enters the exception path and does not commit or advance Count; Rust-only invocation errors are not C++ step outcomes | Non-success variants return `MachineCpuLocalCommittedSuccessCadenceError::NonSuccessAction` | Boundary preserved | Non-success rejection test | Arithmetic-overflow exception entry remains unwired here, and rejection data remains internal readiness data. |
| Mutation boundary | C++ normal success cadence mutates only committed control flow and Count/timer-pending after execution has already happened | Rust applicator mutates only through the two sealed CPU primitives | Equivalent narrow mutation | Preservation tests | The applicator performs no execution helper calls, no fetch/decode/identify, no GPR/HI/LO writeback, no unrelated COP0 mutation, no RDRAM/SP DMEM/reservation mutation, no arithmetic-overflow entry, no `Machine::step`, and no generic `MachineStepResult`. |

### Seam 078 Audit Changes

- Added crate-private `Machine::apply_cpu_local_committed_success_cadence`,
  `MachineCpuLocalCommittedSuccessCadence`, and
  `MachineCpuLocalCommittedSuccessCadenceError` in `machine.rs`.
- The applicator accepts only the existing successful CPU-local action plan,
  then composes the already sealed committed control-flow commit followed by
  the already sealed Count advancement helper.
- The ordering mirrors C++ `step_cpu_instruction`: after a source-clear local
  `kExecuted` success, control flow is committed first and COP0 Count advances
  once afterward. Existing Compare/timer-pending latch behavior remains owned by
  the Count helper.
- Overflow and invocation-rejection actions are rejected by this applicator
  without mutation. Arithmetic-overflow exception entry remains separate and
  unwired.
- Added focused tests that construct action plans directly, so the seam proves
  no fetch, decode, identify, helper selection, helper invocation, instruction
  execution, exception entry, branch/load/store/COP0/ERET/LL/SC behavior,
  memory-map/bus/device routing, `Machine::step`, `Cpu::step`, generic
  `execute_cpu_instruction`, or generic `MachineStepResult`.
- Added no C++ source changes.

## CPU-Local Arithmetic-Overflow Exception Application Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Overflow action accepted | C++ `step_cpu_instruction` converts `MachineFaultKind::kSignedArithmeticOverflow` from execute into local signed-overflow exception entry only on the overflow path | `Machine::apply_cpu_local_arithmetic_overflow_exception` accepts only `MachineCpuLocalInvocationStepActionPlan::EnterArithmeticOverflowException` | Equivalent narrow precondition | Overflow application and rejection tests | Rust does not fetch, decode, identify, select, or invoke helpers. The caller must already have a CPU-local arithmetic-overflow action plan. |
| Overflow context restore | C++ restores `cpu_pc_ = current_pc` and `cpu_next_pc_ = current_next_pc` before exception entry or rethrow | `apply_cpu_local_arithmetic_overflow_exception` calls `Cpu::restore_control_flow(snapshot)` before entry | Equivalent ordering | Ordinary and delay-slot application tests | This prevents normal committed cadence from leaking into overflow entry. It is not generic rollback and restores only the already sealed control-flow snapshot. |
| Arithmetic-overflow exception entry | C++ `enter_local_signed_overflow_exception` sets Cause code 12, EPC/branch-delay according to ordinary versus narrow delay-slot context, EXL, and the local exception vector without BadVAddr or Count advancement | `Cpu::enter_arithmetic_overflow_exception` reused by the Machine applicator | Equivalent via sealed helper | Exception state tests | Rust does not duplicate exception semantics. BadVAddr remains unchanged, Count is not advanced, and PC/next PC move to the existing local exception vector through the CPU helper. |
| Success/rejection excluded | C++ successful execution commits normal cadence; Rust-only invocation errors are not C++ exception outcomes | Non-overflow variants return `MachineCpuLocalArithmeticOverflowExceptionError::NonOverflowAction` | Boundary preserved | Non-overflow rejection test | Successful CPU-local action cadence remains owned by seam 078. Invocation rejection remains internal readiness data and does not enter exceptions. |
| Mutation boundary | C++ overflow path does not commit normal pc/next_pc movement and does not call Count advancement | Rust applicator mutates only by restoring the sealed control-flow snapshot and calling the sealed arithmetic-overflow entry helper | Equivalent narrow mutation | Preservation tests | The applicator performs no execution helper calls, no fetch/decode/identify, no GPR/HI/LO writeback, no RDRAM/SP DMEM/reservation mutation, no Count advancement, no `Machine::step`, no generic exception machinery, and no generic `MachineStepResult`. |

### Seam 079 Audit Changes

- Added crate-private `Machine::apply_cpu_local_arithmetic_overflow_exception`,
  `MachineCpuLocalArithmeticOverflowException`, and
  `MachineCpuLocalArithmeticOverflowExceptionError` in `machine.rs`.
- The applicator accepts only the existing arithmetic-overflow action plan,
  restores the provided `CpuControlFlowSnapshot`, and then calls the already
  sealed `Cpu::enter_arithmetic_overflow_exception` helper.
- The ordering mirrors C++ `step_cpu_instruction`: overflow restores the
  captured faulting context before local signed-overflow exception entry.
- Ordinary and delay-slot tests prove the existing Cause code 12, EPC,
  branch-delay, EXL, local vector, BadVAddr preservation, and no-Count-advance
  semantics are preserved by composition rather than reimplemented.
- Successful and invocation-rejection actions are rejected by this applicator
  without mutation. Successful committed cadence remains owned by seam 078.
- Added focused tests that construct action plans directly, so the seam proves
  no fetch, decode, identify, helper selection, helper invocation, instruction
  execution, normal cadence commit, Count advancement, branch/load/store/COP0/
  ERET/LL/SC behavior, memory-map/bus/device routing, `Machine::step`,
  `Cpu::step`, generic `execute_cpu_instruction`, generic exception machinery,
  or generic `MachineStepResult`.
- Added no C++ source changes.

## CPU-Local Step-Action Application Composition Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Successful action delegated | C++ `step_cpu_instruction` commits normal cadence after a source-clear local `kExecuted` result | `Machine::apply_cpu_local_step_action` delegates `CommitControlFlowAndAdvanceCount` to `Machine::apply_cpu_local_committed_success_cadence` | Equivalent composition | Success composition test | Rust does not duplicate cadence logic. It uses the already sealed success applicator, so `pc` becomes the snapshot `next_pc`, staged `next_pc` remains intact, and Count advances once through the sealed Count helper. |
| Timer latch preserved | C++ Count advancement latches timer-pending only when post-increment Count equals Compare | Success delegation reaches `Cpu::advance_count_for_committed_step` only through the seam 078 applicator | Equivalent by delegation | Success timer-latch test | The composition seam adds no Count logic of its own and does not process interrupts. |
| Overflow action delegated | C++ signed arithmetic overflow restores current control-flow context and enters local signed-overflow exception without Count advancement | `Machine::apply_cpu_local_step_action` delegates `EnterArithmeticOverflowException` to `Machine::apply_cpu_local_arithmetic_overflow_exception` | Equivalent composition | Overflow composition test | Rust uses the already sealed overflow applicator, which restores the `CpuControlFlowSnapshot` and calls the sealed arithmetic-overflow entry helper. |
| Overflow exception semantics preserved | C++ overflow entry sets Cause code 12, EPC/branch-delay, EXL, and vectoring without BadVAddr or Count mutation | Delegated path reaches `Cpu::enter_arithmetic_overflow_exception` only through the seam 079 applicator | Equivalent by delegation | Overflow exception-state test | The composition seam does not commit normal staged cadence and does not advance Count on overflow. |
| Rejection excluded | C++ has no CPU-local success/overflow application for Rust-side invocation invariant failures | `MachineCpuLocalStepActionApplicationError::RejectedInvocation` | Rust-only rejection | Rejection no-mutation test | Invocation rejection is returned without delegating to either applicator and without restoring snapshots, committing cadence, entering exceptions, or mutating CPU/Machine state. |
| Mutation boundary | C++ success and overflow paths are distinct step-owned branches after execute | Rust composition chooses only between already sealed CPU-local applicators | Equivalent narrow composition | Preservation tests and source inspection | The composition seam performs no fetch, decode, identify, selection, helper invocation, instruction execution, branch/load/store/COP0/ERET/LL/SC behavior, generic action dispatch, `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`, generic `MachineStepResult`, memory map, bus, or device routing. |

### Seam 080 Audit Changes

- Added crate-private `Machine::apply_cpu_local_step_action`,
  `MachineCpuLocalStepActionApplication`, and
  `MachineCpuLocalStepActionApplicationError` in `machine.rs`.
- The applicator accepts only the existing
  `MachineCpuLocalInvocationStepActionPlan` from seam 077.
- Successful action plans are delegated to the seam 078 committed-success
  cadence applicator. This preserves existing control-flow commit ordering,
  Count-once behavior, and Compare/timer-pending latch semantics.
- Arithmetic-overflow action plans are delegated to the seam 079
  arithmetic-overflow exception applicator. This preserves existing
  control-flow snapshot restoration, Cause code 12, EPC/branch-delay,
  EXL/vectoring, BadVAddr preservation, and no-Count-advance semantics.
- Invocation rejection plans return
  `MachineCpuLocalStepActionApplicationError::RejectedInvocation` without
  mutation.
- Added focused tests that construct action plans directly, so the seam proves
  no fetch, decode, identify, helper selection, helper invocation, instruction
  execution, branch/load/store/COP0/ERET/LL/SC behavior, memory-map/bus/device
  routing, `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  generic action dispatcher, or generic `MachineStepResult`.
- Added no C++ source changes.

## Non-CPU-Local Step-Frontier Application Composition Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| No-effect executed action | C++ `execute_cpu_instruction` returns `kExecuted` for `kSpecialSync`; `step_cpu_instruction` then commits `cpu_pc_ = current_next_pc` and advances Count | `MachineNonCpuLocalStepFrontierAction::NoEffectExecuted`; `Machine::apply_non_cpu_local_step_frontier_action` | Equivalent narrow mutation for already-classified SYNC readiness | No-effect frontier application test | Rust uses the existing committed control-flow helper and Count helper. It does not execute SYNC, fetch/decode/identify, call CPU-local helpers, or create a step result. |
| No-effect timer latch | C++ committed Count advancement latches timer pending when post-increment Count equals Compare | `Cpu::advance_count_for_committed_step` reached from the no-effect frontier applicator | Equivalent by delegation | Timer-latch assertion in no-effect frontier test | The applicator adds no new timer logic and does not process interrupts. |
| Stopped action | C++ `execute_cpu_instruction` returns `kStopped` for `kSpecialSyscall` and `kSpecialBreak`; `step_cpu_instruction` uses the same committed PC/Count cadence before returning `kStopped` | `MachineNonCpuLocalStepFrontierAction::Stopped`; `Machine::apply_non_cpu_local_step_frontier_action` | Equivalent narrow mutation for already-classified stopped readiness | Stopped frontier application test | Rust commits staged control flow and advances Count once using the stopped cadence plan. It does not enter SYSCALL/BREAK exception state or model host stop/runtime behavior. |
| Unsupported action | C++ restores `cpu_pc_ = current_pc` and `cpu_next_pc_ = current_next_pc` for `kUnsupported`, then returns without Count advancement | `MachineNonCpuLocalStepFrontierAction::Unsupported`; `Cpu::restore_control_flow` | Equivalent narrow mutation for already-classified unsupported readiness | Unsupported frontier application test | Rust restores only the provided control-flow snapshot, preserves Count, and does not invent generic unsupported-instruction machinery. |
| Selected fetch-fault action | C++ selected fetch faults enter local AdEL before speculative next-PC staging and return `kException` without Count advancement | `MachineNonCpuLocalStepFrontierAction::FetchFault(MachineStepFetchFaultAction::EnterAddressError)`; `Machine::enter_instruction_fetch_address_error_exception` | Equivalent narrow mutation for already-classified selected fetch faults | Fetch-fault frontier application test | Rust delegates to the already sealed instruction-fetch AdEL entry helper. It does not commit normal staged cadence, advance Count, or restore a staged snapshot because C++ handles this path before speculative staging. |
| Fetch-fault rejection | C++ non-converting fetch faults are rethrown; selected fetch faults can still be rejected by local entry guards | `MachineNonCpuLocalStepFrontierApplicationError::{FetchFaultRethrow,FetchAddressErrorEntry}` | Equivalent rejection boundary | Fetch-fault rejection no-mutation test | Rust returns the rejection without mutation and does not convert it to unsupported, exception, or a generic step result. |
| CPU-local separation | C++ local execute success/overflow paths are handled after execute; this frontier covers separate no-effect/stopped/unsupported/fetch-fault readiness categories | This seam never calls `Machine::apply_cpu_local_step_action`, CPU-local selector, or CPU-local invoker | Boundary preserved | Source inspection; existing CPU-local tests still pass | CPU-local action application remains owned by seam 080. Branch/load/store/COP0/ERET/LL/SC remain absent. |
| Mutation boundary | C++ `step_cpu_instruction` contains broader fetch/decode/identify/execute/cadence/result logic | Rust applicator consumes only already-classified frontier actions and a `CpuControlFlowSnapshot` | Equivalent narrow composition only | Preservation tests and source inspection | This is not `Machine::step`, `Cpu::step`, `execute_cpu_instruction`, generic `MachineStepResult`, a generic all-category dispatcher, branch/delay-slot behavior, memory map, bus, or device routing. |

### Seam 081 Audit Changes

- Added crate-private `MachineNonCpuLocalStepFrontierAction`,
  `MachineNonCpuLocalStepFrontierApplication`,
  `MachineNonCpuLocalStepFrontierApplicationError`, and
  `Machine::apply_non_cpu_local_step_frontier_action` in `machine.rs`.
- No-effect executed readiness currently covers SYNC only. Its application uses
  the already sealed committed control-flow and Count helpers, preserving
  Compare/timer-pending latch semantics.
- Stopped readiness currently covers SYSCALL and BREAK. Its application uses
  the already sealed stopped cadence plan, commits staged control flow, and
  advances Count once without entering SYSCALL/BREAK exception state.
- Unsupported readiness restores the provided `CpuControlFlowSnapshot` and does
  not advance Count.
- Selected fetch-fault readiness delegates only to the sealed instruction-fetch
  AdEL entry helper and preserves the already sealed Cause code, EPC/BadVAddr,
  EXL/vectoring, and no-Count semantics.
- Non-converting fetch faults and fetch-entry guard rejections return narrow
  errors without mutation.
- The seam does not call CPU-local action application, CPU-local selection, or
  CPU-local invocation. It performs no fetch, decode, identify, instruction
  execution, branch/load/store/COP0/ERET/LL/SC behavior, memory-map/bus/device
  routing, `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`,
  generic all-category dispatcher, or generic `MachineStepResult`.
- Added no C++ source changes.

## Classified Step-Action Application Composition Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| CPU-local success action | C++ `step_cpu_instruction` reaches normal committed cadence after a local `kExecuted` result | `MachineClassifiedStepAction::CpuLocal`; `Machine::apply_classified_step_action` delegates to `Machine::apply_cpu_local_step_action` | Equivalent by delegation | Classified CPU-local success test | The delegated seam 080 path reaches the seam 078 committed-success applicator, so staged control flow commits and Count advances exactly once with existing Compare/timer-pending latch semantics. |
| CPU-local overflow action | C++ `step_cpu_instruction` converts signed arithmetic overflow from execute into local signed-overflow exception entry without normal commit or Count advancement | `MachineClassifiedStepAction::CpuLocal`; delegated `MachineCpuLocalStepActionApplication::ArithmeticOverflowException` | Equivalent by delegation | Classified CPU-local overflow test | The delegated seam 080 path reaches the seam 079 overflow applicator, preserving Cause code 12, EPC/branch-delay, EXL/vectoring, BadVAddr preservation, and no-Count semantics. |
| No-effect frontier action | C++ `kSpecialSync` returns `kExecuted`; step then commits normal cadence and advances Count | `MachineClassifiedStepAction::NonCpuLocalFrontier`; delegated `MachineNonCpuLocalStepFrontierApplication::NoEffectExecuted` | Equivalent by delegation | Classified no-effect test | Rust does not execute SYNC here; it only delegates an already-classified frontier action to seam 081. |
| Stopped frontier action | C++ `kSpecialSyscall` and `kSpecialBreak` return `kStopped`; step uses the committed cadence before returning stopped | `MachineClassifiedStepAction::NonCpuLocalFrontier`; delegated `MachineNonCpuLocalStepFrontierApplication::Stopped` | Equivalent by delegation | Classified stopped test | Rust commits stopped cadence and advances Count through seam 081, without entering SYSCALL/BREAK exception state. |
| Unsupported frontier action | C++ `kUnsupported` restores `current_pc/current_next_pc` and returns without Count advancement | `MachineClassifiedStepAction::NonCpuLocalFrontier`; delegated `MachineNonCpuLocalStepFrontierApplication::Unsupported` | Equivalent by delegation | Classified unsupported test | Rust restores the provided `CpuControlFlowSnapshot` through seam 081 and does not invent generic unsupported-instruction machinery. |
| Fetch-fault frontier action | C++ selected instruction-fetch faults enter local AdEL before normal speculative staging and return `kException` without Count advancement | `MachineClassifiedStepAction::NonCpuLocalFrontier`; delegated `MachineNonCpuLocalStepFrontierApplication::FetchAddressErrorException` | Equivalent by delegation | Classified fetch-fault test | Rust enters only the sealed instruction-fetch AdEL path through seam 081. Normal committed cadence and Count advancement remain absent. |
| Delegated rejections | C++ non-converting fetch faults are rethrown; Rust CPU-local invocation errors are invariant rejections, not C++ step successes | `MachineClassifiedStepActionApplicationError::{CpuLocal,NonCpuLocalFrontier}` | Equivalent rejection boundary | Classified rejection no-mutation test | Rejections are returned from the delegated applicators without mutation and are not converted to a generic step result. Non-applicable future categories are not represented by this seam. |
| Mutation boundary | C++ `step_cpu_instruction` owns broader fetch/decode/identify/execute/category selection before applying these outcomes | Rust composition consumes only already-classified sealed-category actions plus a `CpuControlFlowSnapshot` | Equivalent narrow composition only | Preservation tests and source inspection | The seam does not fetch, decode, identify, classify instruction identities, select CPU-local helpers, invoke CPU-local helpers, call CPU-local execution directly, add future placeholders, implement branch/load/store/COP0/ERET/LL/SC behavior, add `Machine::step`, add `Cpu::step`, add generic execute, add a generic all-category dispatcher, add generic `MachineStepResult`, memory map, bus, or device routing. |

### Seam 082 Audit Changes

- Added crate-private `MachineClassifiedStepAction`,
  `MachineClassifiedStepActionApplication`,
  `MachineClassifiedStepActionApplicationError`, and
  `Machine::apply_classified_step_action` in `machine.rs`.
- CPU-local classified actions delegate only to the seam 080
  `Machine::apply_cpu_local_step_action` applicator.
- Non-CPU-local frontier classified actions delegate only to the seam 081
  `Machine::apply_non_cpu_local_step_frontier_action` applicator.
- CPU-local success still reaches the seam 078 committed-success cadence path:
  committed control-flow application followed by exactly one Count advance with
  the existing Compare/timer-pending latch behavior.
- CPU-local arithmetic overflow still reaches the seam 079 overflow exception
  path: snapshot restoration followed by the sealed arithmetic-overflow entry,
  preserving Cause code 12, EPC/branch-delay, EXL/vectoring, no BadVAddr write,
  and no Count advancement.
- No-effect, stopped, unsupported, selected fetch-fault, and fetch-rejection
  frontier actions remain owned by seam 081.
- Invocation errors and fetch rejections remain delegated rejection results
  without mutation. Future/non-applicable categories are not represented.
- The seam performs no fetch, decode, identify, instruction classification,
  CPU-local helper selection, CPU-local helper invocation, instruction
  execution, branch/load/store/COP0/ERET/LL/SC behavior, interrupt processing,
  memory-map/bus/device routing, `Machine::step`, `Cpu::step`, generic
  `execute_cpu_instruction`, generic all-future-category dispatcher, or generic
  `MachineStepResult`.
- Added no C++ source changes.

## Current-PC Classified Step-Action Production Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Snapshot before staging | `src/core/machine_cpu.cpp` `step_cpu_instruction` captures `current_pc` and `current_next_pc` before normal speculative execution staging | `Machine::produce_current_pc_classified_step_action` captures `CpuControlFlowSnapshot` before calling `Cpu::stage_next_sequential_pc_for_step` | Equivalent for represented control-flow state | Current-PC production snapshot test | Rust preserves the captured snapshot in the produced action wrapper so seam 082 can later apply the action. |
| Current-PC fetch source | C++ fetches through `fetch_cpu_instruction_word()` from `cpu_pc()` before decode/identify | `Machine::fetch_current_cpu_instruction_word` is called after snapshot capture and staging, using the still-current `pc` as fetch address | Equivalent for represented current-PC fetch | Current-PC production tests; existing fetch tests | Staging changes only `next_pc`, so the fetch address remains the captured/current `pc`. |
| Sequential staging | C++ stages `cpu_next_pc_ = sequential_instruction_address(current_next_pc)` after fetch/decode/identity and before execution for non-ERET identities | Rust stages exactly once through `Cpu::stage_next_sequential_pc_for_step` before producing the action | Equivalent staged-state primitive | Current-PC production staging tests | The seam does not commit `pc`, does not advance Count, and does not apply the produced action. |
| Fetch-fault action production | C++ selected instruction-fetch faults enter local AdEL before normal cadence; non-converting faults are rethrown | Rust classifies fetch failures with `classify_step_fetch_fault_action`, restores the snapshot, returns a classified fetch-fault action for selected AdEL conversion, and returns a fetch rethrow error for non-converting faults | Equivalent production boundary | Selected fetch-fault and fetch-rejection tests | The snapshot restore before returning selected fetch-fault action keeps the already sealed seam 081 AdEL applicator in its source-clear pre-staging context. No decode, identify, helper selection, helper invocation, exception entry, cadence commit, or Count advance occurs on fetch failure. |
| Non-CPU-local frontier identities | C++ returns or handles SYNC as `kExecuted`, SYSCALL/BREAK as `kStopped`, and unsupported identities as `kUnsupported` outside CPU-local writeback helper execution | Rust maps SYNC to `NoEffectExecuted`, SYSCALL/BREAK to `Stopped`, and unsupported identities to `Unsupported` classified frontier actions | Equivalent for currently sealed frontier categories | No-effect, stopped, and unsupported production tests | Rust does not invoke CPU-local helpers for these identities and does not enter SYSCALL/BREAK exception state. |
| CPU-local success production | C++ `execute_cpu_instruction` mutates CPU-local state for represented local helpers and returns `kExecuted`; step applies cadence later | Rust selects with `select_cpu_local_executed_helper`, invokes with `Cpu::invoke_cpu_local_executed_helper`, and maps success through `classify_cpu_local_invocation_step_action` | Equivalent for currently sealed CPU-local helper families | CPU-local success production test; selector/invocation tests | Successful CPU-local helper writeback is the only instruction-side mutation besides `next_pc` staging. The produced classified action is not applied in this seam. |
| CPU-local overflow production | C++ signed overflow from ADD/SUB/DADD/DSUB/ADDI/DADDI is produced before writeback, then the step catch path restores control flow and enters overflow exception | Rust receives `ArithmeticOverflow` from the sealed CPU-local invocation helper and maps it to the CPU-local overflow action plan without entering exception state | Equivalent production boundary | CPU-local overflow production test; overflow helper tests | Rust performs no overflow writeback, no exception entry, no normal cadence commit, and no Count advancement. The existing seam 082 applicator can later apply the produced action. |
| Rejection boundary | C++ branch/load/store/COP0/ERET/LL/SC and other future categories are not represented by this narrow Rust producer | `MachineCurrentPcClassifiedStepActionError::{UnrepresentedInstruction,CpuLocalInvocation,FetchFaultRethrow}` | Equivalent absence boundary for unsealed categories | Unrepresented identity and fetch-rejection tests | Rejections restore captured `pc`/`next_pc` when production had staged `next_pc` and return without Count advancement or exception entry. Invocation errors are Rust-side invariants and are not presented as C++ step outcomes. |
| Mutation boundary | C++ full step also owns final cadence, exceptions, interrupts, branch/load/store/COP0/ERET/LL/SC, and result reporting | Rust producer only returns a `MachineClassifiedStepAction` plus snapshot | Equivalent narrow production only | Focused current-PC tests; source inspection | The seam does not call `Machine::apply_classified_step_action`, `Cpu::commit_staged_step_control_flow`, `Cpu::advance_count_for_committed_step`, arithmetic-overflow entry, instruction-fetch AdEL entry, `Machine::step`, `Cpu::step`, generic execute, generic `MachineStepResult`, memory map, bus, or device routing. |

### Seam 083 Audit Changes

- Added crate-private `MachineCurrentPcClassifiedStepAction`,
  `MachineCurrentPcClassifiedStepActionError`, and
  `Machine::produce_current_pc_classified_step_action` in `machine.rs`.
- The producer captures a `CpuControlFlowSnapshot`, stages sequential
  `next_pc` exactly once, fetches through the current-PC fetch wrapper,
  decodes the fetched instruction word, and identifies only after successful
  fetch.
- Fetch faults do not decode, identify, select, or invoke helpers. Selected
  fetch faults restore the captured control flow and return the existing
  classified fetch-fault action for seam 082; non-converting fetch faults
  restore and return a source-clear fetch rethrow error.
- SYNC maps to the existing no-effect frontier action. SYSCALL and BREAK map
  to stopped frontier actions. Unsupported identities map to unsupported
  frontier actions. None of those paths invokes CPU-local helpers.
- CPU-local represented identities use the already sealed selector, invocation
  helper, and outcome-to-action planner. Successful invocation may perform the
  sealed helper writeback but does not commit cadence or advance Count.
  Arithmetic overflow performs no writeback and is returned as an applicable
  classified overflow action without entering exception state.
- Unrepresented identities and invocation rejections restore captured
  `pc`/`next_pc` and return narrow errors without Count advancement or
  exception entry.
- The seam produces classified actions but does not apply them. It adds no
  `Machine::step`, `Cpu::step`, generic `execute_cpu_instruction`, generic
  all-future-category dispatcher, generic `MachineStepResult`, branch/jump/
  link/delay-slot/branch-likely behavior, load/store behavior, COP0 execution,
  ERET, LL/SC, interrupt processing, memory map, bus, or device routing.
- `fn64_machine_probe` remains construction/reset/no-window only.
- Added no C++ source changes.

## Machine Step Composition Seal

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Step composition owner | `src/core/machine_cpu.cpp` `Machine::step_cpu_instruction` composes fetch/decode/identify/execution outcome production with final cadence or exception application | `Machine::step` calls `Machine::produce_current_pc_classified_step_action` then `Machine::apply_classified_step_action` | Equivalent for represented categories only | Machine step composition tests; source inspection | `Machine::step` is intentionally a thin composition layer. It does not duplicate fetch, decode, identify, CPU-local selection, CPU-local invocation, or classified action application logic. |
| Public represented outcome | C++ returns `CpuInstructionStepResult` for broad categories | `MachineRepresentedStepOutcome` | Narrow Rust API shape for represented categories only | Machine step outcome tests | Outcomes cover CPU-local committed instruction, arithmetic-overflow exception, SYNC no-effect commit, SYSCALL/BREAK stopped commit, unsupported rollback, and selected instruction-fetch AdEL exception. This is not a generic all-future `MachineStepResult`. |
| Public represented rejection | C++ can rethrow non-converting faults and broader unrepresented paths remain outside Rust | `MachineRepresentedStepError` | Narrow Rust rejection shape | Fetch rejection and unrepresented-category tests | Rejections cover non-converting fetch errors, CPU-local invocation invariant rejections, unrepresented identities, and guarded exception-entry rejection. They restore source-clear staged state through the already sealed producer/applicators rather than inventing generic step errors. |
| CPU-local success | C++ local `kExecuted` commits `pc = current_next_pc` and advances Count once | Producer performs sealed helper writeback, applicator commits through seam 078 | Equivalent for represented CPU-local helper success | CPU-local success step test | Count advances exactly once and preserves Compare/timer-pending latch semantics. |
| CPU-local arithmetic overflow | C++ signed-overflow path restores current control flow and enters local overflow exception without writeback or Count advancement | Producer returns overflow action, applicator delegates through seam 079 | Equivalent for represented overflow paths | CPU-local overflow step test | Rust preserves Cause code 12, EPC/branch-delay, EXL/vectoring, BadVAddr preservation, and no normal cadence commit. |
| SYNC no-effect | C++ SYNC returns `kExecuted` with no instruction-local mutation, then commits normal cadence | Producer returns no-effect frontier action, applicator delegates through seam 081 | Equivalent for SYNC only | SYNC step test | No CPU-local helper is invoked for SYNC. |
| SYSCALL/BREAK stopped | C++ SYSCALL/BREAK return `kStopped` and still use committed PC/Count cadence | Producer returns stopped frontier action, applicator delegates through seam 081 | Equivalent for stopped readiness only | SYSCALL/BREAK step tests | Rust does not enter SYSCALL or BREAK exception state and does not model host stop/runtime policy. |
| Unsupported instruction | C++ `kUnsupported` restores captured `pc`/`next_pc` and does not advance Count | Producer returns unsupported frontier action, applicator delegates through seam 081 | Equivalent for represented unsupported subset | Unsupported step test | Rust does not invent generic unsupported-instruction machinery. |
| Selected instruction-fetch fault | C++ selected fetch faults enter local instruction-fetch AdEL and do not advance Count | Producer returns selected fetch-fault action without decode/identify/helper invocation, applicator delegates through seam 081 | Equivalent for selected fetch faults | Fetch-fault step test | Non-converting fetch faults return rejection with restored control flow and no Count advancement. |
| Strict absence | C++ full step also owns interrupts, branch/load/store/COP0/ERET/LL/SC, device routing, and full probe behavior | No Rust owner in this seam | Intentionally absent | Source inspection; gates | `Machine::step` adds no `Cpu::step`, generic `execute_cpu_instruction`, generic all-future step result, branch/jump/link/delay-slot/branch-likely behavior, load/store behavior, COP0 execution, ERET, LL/SC, interrupts, bus, memory map, device/MMIO routing, cartridge execution mapping, PIF/BIOS behavior, Rust step probe, or probe coverage claim. |

### Seam 084 Audit Changes

- Added public represented-category `Machine::step` in `machine.rs`.
- Added public `MachineRepresentedStepOutcome` and
  `MachineRepresentedStepError` plus narrow rejection payloads for CPU-local
  invocation and arithmetic-overflow entry guard failures.
- `Machine::step` composes only the seam 083 current-PC classified action
  producer and the seam 082 classified action applicator, then converts the
  internal application result into a represented public outcome.
- CPU-local helper writeback remains owned by the producer/invoker path, normal
  committed cadence remains owned by the success applicator, overflow exception
  entry remains owned by the overflow applicator, and frontier actions remain
  owned by the non-CPU-local applicator.
- The focused `machine_step` tests cover CPU-local success, CPU-local arithmetic
  overflow, SYNC, SYSCALL, BREAK, unsupported instruction rollback, selected
  fetch-fault AdEL entry, and non-converting fetch rejection.
- `fn64_machine_probe` remains construction/reset/no-window only and no Rust
  step probe was added.
- Added no `Cpu::step`, generic `execute_cpu_instruction`, generic
  `MachineStepResult`, future-category placeholders, branch/load/store/COP0/
  ERET/LL/SC behavior, interrupts, memory map, bus, device routing, cartridge
  execution mapping, PIF/BIOS behavior, host runtime, or C++ source changes.

### Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Represented Machine step no-window inspection probe audit | `Machine::step` now exists for represented categories, but `fn64_machine_probe` intentionally remains construction/reset only and no Rust step probe claims have been made | Seam 084 represented-category `Machine::step` | Medium | Recommended |
| Full step outcome compatibility audit | Fetch, decode, identity, selected readiness outcomes, rollback, commit, Count, and several narrow execute helpers now compose for represented categories, but branch/load/store/COP0/ERET/LL/SC, interrupts, devices, and full probe behavior remain absent | Represented Machine step plus many future instruction/device seams | High | Later |
| Instruction execute readiness audit | Decode/identity exist and the narrow SPECIAL shift/logical/HI-LO transfer/non-trapping integer/trapping integer and immediate trapping/non-trapping integer/comparison/bitwise-logical/LUI families are sealed, but broad execution side effects remain absent | Step result/error shape, selected instruction families, writeback readiness | High | Needs future pass |
| CPU step | Still needs broader execute, branch/load/store/COP0/ERET/LL/SC, interrupts, and device behavior before any CPU-owned step can be honest | Multiple future seams | High | Blocked |
| Memory map / bus audit | Current fetch still uses local represented target checks only | Step/fetch/data access readiness | High | Not recommended before rollback/cadence seams |

## CPU GPR Access/Mutation Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ GPR storage ownership | `src/core/machine.hpp` `Machine::cpu_gprs_`; `kCpuGprCount` | `rust/crates/fn64-core/src/cpu.rs` `Cpu::gprs`; `CPU_GPR_COUNT` | Equivalent state semantics, different ownership shape | Source inspection; CPU construction tests | C++ stores GPRs directly in `Machine`; Rust groups the already-earned CPU state in sidecar `Cpu`. |
| Rust GPR storage ownership | C++ `Machine` fields listed above | `cpu.rs` private `gprs: [u64; CPU_GPR_COUNT]` | Equivalent state semantics, different ownership shape | `new_cpu_zeroes_integer_register_state`; `new_cpu_exposes_cpp_gpr_count_boundary` | Rust does not claim type-layout equivalence with C++. |
| C++ GPR read behavior | `src/core/machine_cpu.cpp` `Machine::inspect_cpu_gpr` -> `read_cpu_gpr_value` | `cpu/registers.rs` `Cpu::gpr` | Equivalent for valid indices | `new_cpu_zeroes_integer_register_state`; source inspection | C++ returns full 64-bit `CpuRegisterValue`; Rust returns `Some(u64)` for valid indices. |
| Rust GPR read behavior | C++ `read_cpu_gpr_value` checks bounds and special-cases index 0 | `cpu/registers.rs` `Cpu::gpr` checks bounds and special-cases index 0 | Equivalent for valid indices; Rust-only API safety for invalid indices | `new_cpu_exposes_cpp_gpr_count_boundary`; source inspection | Invalid Rust reads return `None`; C++ throws `std::out_of_range`. |
| C++ GPR write behavior | `src/core/machine_cpu.cpp` `Machine::stage_cpu_gpr` -> `write_cpu_gpr_value` | `cpu/registers.rs` `Cpu::set_gpr` | Equivalent state semantics, different ownership shape | `gpr_write_updates_only_the_selected_nonzero_register`; C++ `run_machine_construction_isolation_demo` | This is storage mutation only. It is not instruction writeback, execution, or reset. |
| Rust GPR write behavior | C++ public staging helper and private full-value write helper | `cpu/registers.rs` `Cpu::set_gpr` | Equivalent for valid indices | Rust GPR mutation tests; source inspection | Rust keeps mutation on `Cpu`; no `Machine::cpu_mut`, step, decode, or instruction APIs were added. |
| Register zero write behavior | `src/core/machine_cpu.cpp` `write_cpu_gpr_value` returns without storing when `index == 0`; `read_cpu_gpr_value(0)` returns zero | `cpu/registers.rs` `Cpu::set_gpr` returns `Ok(())` without storing for index 0; `Cpu::gpr(0)` returns `Some(0)` | Equivalent | `gpr_zero_write_is_ignored_without_changing_other_state`; C++ zero-register proof paths | Rust mirrors the state rule only; branch/link aliasing through GPR zero remains instruction-level future work. |
| Valid register index behavior | C++ accepts indices `0..cpu_gprs_.size()`; index 0 is special | Rust accepts indices `0..CPU_GPR_COUNT`; index 0 is special | Equivalent | `new_cpu_exposes_cpp_gpr_count_boundary`; GPR mutation tests | Count is 32 in both implementations. |
| Invalid register index behavior | `fail_cpu_gpr_index` throws `std::out_of_range` with `CPU GPR index out of range: {index}` | `cpu/registers.rs` `Cpu::gpr` returns `None`; `Cpu::set_gpr` returns `CpuRegisterIndexError` with matching display text | Rust-only API safety, no emulator truth | `gpr_write_invalid_index_is_explicit_rust_api_safety`; C++ public guard demo | Rust uses explicit `Option`/`Result` instead of exceptions. The display text mirrors C++ for auditability. |
| GPR mutation independent of instruction execution | Public C++ `stage_cpu_gpr` mutates storage directly; proof setup uses it before inspection/steps | `cpu/registers.rs` `Cpu::set_gpr` mutates only `Cpu::gprs` | Equivalent state semantics, different ownership shape | `run_machine_construction_isolation_demo`; `gpr_write_updates_only_the_selected_nonzero_register` | C++ also uses GPR helpers inside execution paths, but this seam claims only the public staging/storage rule. |
| GPR mutation changes PC | `write_cpu_gpr_value` does not assign `cpu_pc_` | `cpu/registers.rs` `Cpu::set_gpr` does not assign `pc` | Equivalent | `gpr_write_updates_only_the_selected_nonzero_register`; source inspection | Instruction step PC movement is not in scope. |
| GPR mutation changes next PC | `write_cpu_gpr_value` does not assign `cpu_next_pc_` | `cpu/registers.rs` `Cpu::set_gpr` does not assign `next_pc` | Equivalent | `gpr_write_updates_only_the_selected_nonzero_register`; source inspection | Branch and delay-slot scheduling remain outside this seam. |
| GPR mutation changes HI/LO | `write_cpu_gpr_value` does not assign `cpu_hi_` or `cpu_lo_` | `cpu/registers.rs` `Cpu::set_gpr` does not assign `hi` or `lo` | Equivalent | `gpr_write_updates_only_the_selected_nonzero_register`; source inspection | GPR mutation does not touch HI/LO. HI/LO scalar staging is covered by the CPU scalar state seam. |
| GPR mutation touches COP0 | `write_cpu_gpr_value` does not assign local COP0 fields | `cpu/registers.rs` `Cpu::set_gpr` does not assign COP0 fields | Equivalent | `gpr_write_updates_only_the_selected_nonzero_register`; source inspection | COP0 writes, exceptions, ERET, and Count step wiring remain absent. |
| Read-before-write instruction semantics | C++ execution/proof files have instruction-level operand and alias demos | No Rust instruction execution or operand timing API | Not in scope | C++ gates only | This pass does not claim instruction operand ordering. |
| Branch/link alias behavior | C++ branch/jump proof files exercise link register and GPR zero instruction behavior | No Rust branch, link, or instruction writeback API | Not in scope | C++ gates only | Future instruction-level seam; not part of plain GPR storage mutation. |
| Delay-slot behavior | C++ control-flow proof files exercise delay-slot scheduling | No Rust branch or delay-slot behavior | Not in scope | C++ gates only | Delay-slot behavior remains intentionally absent from represented `Machine::step`. |
| Rust-only safety behavior | C++ exceptions for invalid GPR indices | `cpu/registers.rs` `CpuRegisterIndexError`; `Option` invalid read boundary | Rust-only API safety, no emulator truth | Rust tests; source inspection | This keeps Rust APIs safe without claiming C++ exception/type parity. |

## CPU Scalar State Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ PC storage ownership | `src/core/machine.hpp` `Machine::cpu_pc_`; `CpuAddress` | `cpu.rs` private `pc`; `cpu/scalars.rs` `Cpu::pc` | Equivalent state semantics, different ownership shape | Source inspection; scalar staging tests | C++ stores PC directly in `Machine`; Rust keeps it in sidecar `Cpu` owned by `Machine`. |
| Rust PC storage ownership | C++ Machine field above | `cpu.rs` private `pc: u32` | Equivalent state semantics, different ownership shape | `new_cpu_starts_at_cpp_non_boot_pc_pair`; scalar staging tests | Width mirrors C++ `CpuAddress = std::uint32_t`. |
| C++ PC read behavior | `src/core/machine_cpu.cpp` `Machine::cpu_pc()` returns `cpu_pc_` | `cpu/scalars.rs` `Cpu::pc()` returns `pc` | Equivalent state semantics, different ownership shape | CPU construction and scalar staging tests; source inspection | Rust reads from `Cpu` instead of Machine forwarding. |
| Rust PC read behavior | C++ `Machine::cpu_pc()` | `cpu/scalars.rs` `Cpu::pc()` | Equivalent state semantics, different ownership shape | `new_cpu_starts_at_cpp_non_boot_pc_pair`; scalar staging tests | Read-only observation; not fetch or step. |
| C++ `stage_cpu_pc` behavior | `src/core/machine.hpp` `stage_cpu_pc`; `src/core/machine_cpu.cpp` `stage_cpu_pc` -> `write_cpu_pc` | `cpu/scalars.rs` `Cpu::stage_pc` | Equivalent state semantics, different ownership shape | `stage_pc_sets_pc_and_sequential_next_pc_without_touching_other_cpu_state`; C++ source inspection | C++ PC staging sets `cpu_pc_ = value` and then stages the sequential next PC. |
| Rust `stage_pc` behavior | C++ `write_cpu_pc` | `cpu/scalars.rs` `Cpu::stage_pc` | Equivalent state semantics, different ownership shape | `stage_pc_sets_pc_and_sequential_next_pc_without_touching_other_cpu_state`; scalar staging tests | This is stored scalar staging only. It is not PC advancement, branch, jump, reset, fetch, or step behavior. |
| `stage_pc` next-PC side effect | `src/core/machine_cpu.cpp` `write_cpu_pc` assigns `cpu_next_pc_ = sequential_instruction_address(value)` | `cpu/scalars.rs` `Cpu::stage_pc` assigns `next_pc = sequential_instruction_address(value)` | Equivalent state semantics, different ownership shape | `stage_pc_sets_pc_and_sequential_next_pc_without_touching_other_cpu_state`; source inspection | The side effect is intentional because C++ source truth does it. Rust keeps the `stage_` name and does not call this a simple setter. |
| `stage_pc` wrapping behavior | C++ `CpuAddress = std::uint32_t`; `sequential_instruction_address(address) { return address + 4u; }` | Rust `pc: u32`; private `sequential_instruction_address(address).wrapping_add(4)` | Equivalent | `stage_pc_uses_cpp_u32_wrapping_for_next_pc`; C++ source inspection | Rust proves the boundary cases `0xffff_fffc..=0xffff_ffff` wrap to `0..=3`, matching unsigned 32-bit C++ addition. |
| C++ next PC storage ownership | `src/core/machine.hpp` `Machine::cpu_next_pc_`; `CpuAddress` | `cpu.rs` private `next_pc`; `cpu/scalars.rs` `Cpu::next_pc` | Equivalent state semantics, different ownership shape | Source inspection; scalar staging tests | C++ stores next PC directly in `Machine`; Rust stores it in sidecar `Cpu`. |
| Rust next PC storage ownership | C++ Machine field above | `cpu.rs` private `next_pc: u32` | Equivalent state semantics, different ownership shape | `new_cpu_starts_at_cpp_non_boot_pc_pair`; scalar staging tests | Width mirrors C++ `CpuAddress = std::uint32_t`. |
| C++ next PC read behavior | `src/core/machine_cpu.cpp` `Machine::cpu_next_pc()` returns `cpu_next_pc_` | `cpu/scalars.rs` `Cpu::next_pc()` returns `next_pc` | Equivalent state semantics, different ownership shape | CPU construction and scalar staging tests; source inspection | Read-only observation; not delay-slot behavior. |
| Rust next PC read behavior | C++ `Machine::cpu_next_pc()` | `cpu/scalars.rs` `Cpu::next_pc()` | Equivalent state semantics, different ownership shape | `new_cpu_starts_at_cpp_non_boot_pc_pair`; scalar staging tests | Rust keeps the accessor on `Cpu`, not `Machine`. |
| C++ `stage_cpu_next_pc` behavior | `src/core/machine.hpp` `stage_cpu_next_pc`; `src/core/machine_cpu.cpp` `stage_cpu_next_pc` -> `write_cpu_next_pc` | `cpu/scalars.rs` `Cpu::stage_next_pc` | Equivalent state semantics, different ownership shape | `stage_next_pc_updates_only_next_pc_without_validation`; C++ source inspection | C++ next-PC staging assigns `cpu_next_pc_ = value` and does not assign `cpu_pc_`. |
| Rust `stage_next_pc` behavior | C++ `write_cpu_next_pc` | `cpu/scalars.rs` `Cpu::stage_next_pc` | Equivalent state semantics, different ownership shape | `stage_next_pc_updates_only_next_pc_without_validation`; scalar staging tests | This is scalar staging only. It does not imply delay-slot scheduling or control flow. |
| C++ HI storage ownership | `src/core/machine.hpp` `Machine::cpu_hi_`; `CpuRegisterValue` | `cpu.rs` private `hi`; `cpu/scalars.rs` `Cpu::hi` | Equivalent state semantics, different ownership shape | Source inspection; scalar staging tests | C++ stores HI directly in `Machine`; Rust stores it in sidecar `Cpu`. |
| Rust HI storage ownership | C++ Machine field above | `cpu.rs` private `hi: u64` | Equivalent state semantics, different ownership shape | CPU construction and scalar staging tests | Width mirrors C++ `CpuRegisterValue = std::uint64_t`. |
| C++ `stage_cpu_hi` behavior | `inspect_cpu_hi` -> `cpu_hi`; `stage_cpu_hi` -> `write_cpu_hi` | `cpu/scalars.rs` `Cpu::hi`; `Cpu::stage_hi` | Equivalent state semantics, different ownership shape | `stage_hi_updates_only_hi`; C++ source inspection and proof staging | C++ HI staging assigns only `cpu_hi_`. |
| Rust `stage_hi` behavior | C++ `inspect_cpu_hi` / `stage_cpu_hi` | `cpu/scalars.rs` `Cpu::hi`; `Cpu::stage_hi` | Equivalent state semantics, different ownership shape | `stage_hi_updates_only_hi`; scalar staging tests | HI staging does not alter LO, GPRs, PC/next PC, or COP0 construction fields. It does not add MULT/DIV/MTHI instruction behavior. |
| C++ LO storage ownership | `src/core/machine.hpp` `Machine::cpu_lo_`; `CpuRegisterValue` | `cpu.rs` private `lo`; `cpu/scalars.rs` `Cpu::lo` | Equivalent state semantics, different ownership shape | Source inspection; scalar staging tests | C++ stores LO directly in `Machine`; Rust stores it in sidecar `Cpu`. |
| Rust LO storage ownership | C++ Machine field above | `cpu.rs` private `lo: u64` | Equivalent state semantics, different ownership shape | CPU construction and scalar staging tests | Width mirrors C++ `CpuRegisterValue = std::uint64_t`. |
| C++ `stage_cpu_lo` behavior | `inspect_cpu_lo` -> `cpu_lo`; `stage_cpu_lo` -> `write_cpu_lo` | `cpu/scalars.rs` `Cpu::lo`; `Cpu::stage_lo` | Equivalent state semantics, different ownership shape | `stage_lo_updates_only_lo`; C++ source inspection and proof staging | C++ LO staging assigns only `cpu_lo_`. |
| Rust `stage_lo` behavior | C++ `inspect_cpu_lo` / `stage_cpu_lo` | `cpu/scalars.rs` `Cpu::lo`; `Cpu::stage_lo` | Equivalent state semantics, different ownership shape | `stage_lo_updates_only_lo`; scalar staging tests | LO staging does not alter HI, GPRs, PC/next PC, or COP0 construction fields. It does not add MULT/DIV/MTLO instruction behavior. |
| PC / next PC alignment or validity rules | `stage_cpu_pc` and `stage_cpu_next_pc` assign values without validation; execution/fetch paths check alignment separately | `cpu/scalars.rs` `Cpu::stage_pc` and `Cpu::stage_next_pc` are infallible and do not validate | Equivalent for scalar staging | `stage_next_pc_updates_only_next_pc_without_validation`; source inspection | Alignment/target validity belongs to fetch/control-transfer/execution seams and is not claimed here. |
| Scalar mutation independent of instruction execution | Public C++ `stage_cpu_pc`, `stage_cpu_next_pc`, `stage_cpu_hi`, and `stage_cpu_lo` mutate storage before proof steps | `cpu/scalars.rs` `Cpu::stage_pc`, `stage_next_pc`, `stage_hi`, `stage_lo` mutate only stored fields | Equivalent state semantics, different ownership shape | C++ `run_machine_construction_isolation_demo`; scalar staging tests | C++ also mutates these fields during execution, but this seam claims only public source-level staging/storage behavior. |
| Scalar mutation changes GPRs | C++ scalar write helpers do not assign `cpu_gprs_` | Rust scalar staging methods do not assign `gprs` | Equivalent | Scalar staging tests | Tests preserve nonzero GPR 8 and zero register state across scalar staging. |
| Scalar mutation touches COP0 | C++ scalar write helpers do not assign local COP0 fields | Rust scalar staging methods do not assign COP0 construction fields | Equivalent | Scalar staging tests; source inspection | COP0 mutation remains absent in Rust. |
| Scalar mutation touches RDRAM or cartridge | C++ scalar write helpers do not assign `rdram_` or `cartridge_` | Rust scalar staging lives on `Cpu`; no Machine forwarding or cartridge/RDRAM access added | Equivalent state semantics, different ownership shape | Source inspection; Rust API inspection | Rust cannot touch Machine-owned cartridge/RDRAM through scalar staging because no mutable Machine scalar forwarding API exists. |
| Rust Machine-level scalar forwarding | C++ public staging methods live on `Machine` because C++ owns CPU fields directly | Rust Machine exposes only `Machine::cpu() -> &Cpu`; no `Machine::stage_cpu_*` or `cpu_mut` exists | C++ exists, Rust intentionally absent | Source inspection; Rust API inspection | Rust keeps scalar staging on the sidecar `Cpu` owner. Machine-level forwarding is not needed for current Rust parity and would widen the public surface. |
| Branch/link behavior | C++ branch/jump proof files use PC/next PC during execution | No Rust branch, link, jump, or control-flow API | Not in scope | C++ gates only | Future instruction-level seam. PC and next PC are just stored scalar state here. |
| Delay-slot behavior | C++ execution/proof files use `next_pc` for delay-slot cadence | No Rust delay-slot behavior | Not in scope | C++ gates only | `stage_next_pc` and represented `Machine::step` do not claim delay-slot semantics. |
| Instruction writeback | C++ execution writes PC/next PC/HI/LO in instruction paths | Rust has raw instruction-word field decode and identity classification only; no writeback, fetch, execute, or step API | Not in scope | C++ gates only | Scalar staging is independent storage mutation, not instruction behavior. |
| Rust-only safety behavior | C++ scalar staging has no exception/result boundary | No Rust scalar error type; scalar staging methods are infallible | Not applicable | Source inspection; Rust tests | No Rust-only scalar validation was added. |
| Naming/layout changes for scalar seam | C++ names are Machine-level `stage_cpu_*` | Rust uses `cpu/scalars.rs` with `Cpu::stage_*` methods | Rust-only repo hygiene, no emulator truth | Source inspection; layout audit | Split scalar access/staging into the earned `cpu::scalars` owner. No broad helper module was added. |

## COP0 Construction/Access Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ COP0 storage ownership | `src/core/machine.hpp` private `cop0_count_`, `cop0_compare_`, `cop0_timer_interrupt_pending_`, `cop0_status_`, `cop0_software_interrupt_pending_`, `cop0_epc_`, `cop0_bad_vaddr_`, `cop0_exception_code_`, `cop0_exception_branch_delay_` | `cpu/cop0.rs` private `Cop0` fields owned by `Cpu`; `cpu.rs` private `cop0: Cop0` | Equivalent construction state, different ownership shape | Source inspection; `new_cpu_zeroes_cpp_cop0_construction_state` | C++ stores COP0 state directly in `Machine`; Rust groups the already-earned construction subset in a private COP0 owner under `Cpu`. |
| Rust COP0 storage ownership | C++ Machine-owned fields listed above | `cpu/cop0.rs` private `Cop0`; `cpu.rs` private `cop0: Cop0` | Equivalent construction state, different ownership shape | Rust tests; source inspection | `Cop0` is not exported from `lib.rs`; public observation remains through `Cpu::cop0_*` accessors. |
| C++ standalone COP0 type absence | No standalone COP0 class; COP0 fields are Machine members | Private Rust `Cop0` type | Rust-only repo hygiene, no emulator truth | Source inspection | Rust type exists only as sidecar semantic ownership, not type-layout parity. |
| Rust standalone `Cop0` type | C++ has no standalone COP0 type | `cpu/cop0.rs` `pub(super) struct Cop0` | Rust-only repo hygiene, no emulator truth | Source inspection | Private to the `cpu` module; not a public product API. |
| Rust `Cop0` public export | No standalone public C++ COP0 owner | No `Cop0` export from `lib.rs`; `cpu.rs` declares `mod cop0` privately | Equivalent for public surface restraint | Source inspection | Public Rust COP0 observation stays through narrow `Cpu::cop0_*` construction accessors. |
| Count construction/default value | `machine.cpp` `cop0_count_ = 0`; field type `std::uint32_t` | `cpu/cop0.rs` `count: u32`, `Cop0::new`, `Cpu::cop0_count`, `Cpu::advance_count_for_committed_step` | Equivalent construction/access state and narrow helper, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; Count tests; source inspection | Count construction/access and the committed-step helper are claimed; MTC0 Count writes and step wiring remain absent. |
| Compare construction/default value | `machine.cpp` `cop0_compare_ = 0`; field type `std::uint32_t` | `cpu/cop0.rs` `compare: u32`, `Cop0::new`, `Cpu::cop0_compare` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | No Compare write or timer-pending clear behavior is claimed. |
| Timer pending construction/default value | `machine.cpp` `cop0_timer_interrupt_pending_ = false` | `cpu/cop0.rs` `timer_interrupt_pending: bool`, `Cpu::cop0_timer_interrupt_pending` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Timer pending is stored only as construction false state in Rust. |
| Status construction/default value | `machine.cpp` `cop0_status_ = 0`; `read_cop0_status` masks stored status | `cpu/cop0.rs` `status: u32`, `Cpu::cop0_status` | Equivalent for construction zero only | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Rust `cop0_status()` is direct construction-field inspection, not the full C++ masked `read_cop0_status` behavior. |
| Software pending construction/default value | `machine.cpp` `cop0_software_interrupt_pending_ = 0` | `cpu/cop0.rs` `software_interrupt_pending: u32`, `Cpu::cop0_software_interrupt_pending` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | This is one input to C++ derived Cause, but Rust does not expose derived Cause. |
| EPC construction/default value | `machine.cpp` `cop0_epc_ = 0`; field type `CpuAddress` | `cpu/cop0.rs` `epc: u32`, `Cpu::cop0_epc` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Step-probe reset later observes EPC through MFC0 after exception entry; Rust claims construction zero only. |
| BadVAddr construction/default value | `machine.cpp` `cop0_bad_vaddr_ = 0`; field type `CpuAddress` | `cpu/cop0.rs` `bad_vaddr: u32`, `Cpu::cop0_bad_vaddr` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Exception address writes are not modeled. |
| Exception code construction/default value | `machine.cpp` `cop0_exception_code_ = 0`; field type `std::uint8_t` | `cpu/cop0.rs` `exception_code: u8`, `Cpu::cop0_exception_code` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Exception code contributes to C++ derived Cause only after unearned exception behavior. |
| Branch-delay flag construction/default value | `machine.cpp` `cop0_exception_branch_delay_ = false` | `cpu/cop0.rs` `exception_branch_delay: bool`, `Cpu::cop0_exception_branch_delay` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; source inspection | Delay-slot exception behavior is not modeled. |
| COP0 fields/registers represented in C++ | `machine.hpp` fields plus register indices 8 BadVAddr, 9 Count, 11 Compare, 12 Status, 13 Cause, 14 EPC | Rust private fields: count, compare, timer pending, status, software pending, epc, bad_vaddr, exception code, branch delay | Equivalent for construction fields; not equivalent as indexed register API | Source inspection; Rust construction test | Rust intentionally does not add a generic COP0 register file or indexed register API. |
| COP0 fields/registers represented in Rust | C++ fields listed above | `cpu/cop0.rs` private fields and `Cpu::cop0_*` inspectors | Equivalent for construction fields only | Rust tests | Rust exposes internal constituent fields already earned by construction; it does not expose derived `Cause` register composition. |
| C++ direct COP0 inspection behavior | Private `read_cop0_bad_vaddr`, `read_cop0_count`, `read_cop0_compare`, `read_cop0_epc`; `read_cop0_status`; derived `read_cop0_cause`; proof reads through MFC0/step | `cpu/cop0.rs` read-only `Cpu::cop0_*` construction inspectors | Equivalent only for direct stored construction facts | C++ proof source; Rust construction/access tests | Rust does not claim the MFC0 path, masked Status beyond default zero, or derived Cause behavior. |
| Rust direct COP0 inspection behavior | C++ stored fields listed above | `Cpu::cop0_count`, `cop0_compare`, `cop0_timer_interrupt_pending`, `cop0_status`, `cop0_software_interrupt_pending`, `cop0_epc`, `cop0_bad_vaddr`, `cop0_exception_code`, `cop0_exception_branch_delay` | Equivalent construction/access state, different ownership shape | `new_cpu_zeroes_cpp_cop0_construction_state`; `cop0_construction_access_does_not_change_earned_cpu_state`; source inspection | Accessors take `&self` and do not mutate earned CPU state. |
| C++ `read_cop0_status` behavior | `machine_cpu.cpp` `read_cop0_status` returns `cop0_status_ & kCop0SupportedStatusBits` | No Rust `read_cop0_status`; `Cpu::cop0_status` exposes stored construction field only | Not yet earned beyond default zero | C++ `run_cop0_status_observation_demo`; Rust construction test | Default zero is equivalent. Nonzero masking requires COP0 mutation through MTC0 and is intentionally absent. |
| Rust `read_cop0_status` equivalent | C++ masked Status read helper | Intentionally absent | C++ exists, Rust intentionally absent | Source inspection | Adding a C++-named derived accessor now would overclaim nonzero masking behavior that Rust cannot test without mutation. |
| C++ `read_cop0_cause` behavior | `machine_cpu.cpp` derives Cause from exception code, software pending bits, MI pending/mask IP2, timer pending IP7, and branch-delay bit | No Rust `read_cop0_cause` or `cop0_cause` accessor | Not yet earned beyond default zero inputs | C++ cause, MI, software, and timer proof demos | Default all-zero construction fields imply zero Cause, but Rust does not expose or claim derived Cause composition. |
| Rust `read_cop0_cause` equivalent | C++ derived Cause read helper | Intentionally absent | C++ exists, Rust intentionally absent | Source inspection | Nonzero Cause depends on unearned COP0 mutation, MI device shadows, timer pending, exceptions, and delay-slot state. |
| Derived/masked read behavior testability | C++ Status/Cause nonzero cases require MTC0, MMIO interrupt state, timer/count cadence, exception entry, or ERET | No Rust mutation or derived read injection hooks | Not yet earned | Proof source inspection; no hidden test hooks added | Only default zero is testable without inventing behavior, so Rust keeps derived reads absent. |
| C++ COP0 staging/mutation behavior | Private `write_cop0_count`, `write_cop0_compare`, `write_cop0_status`, `write_cop0_cause`, `write_cop0_epc`; reached by MTC0 execution and exception/interrupt helpers | `Cpu::enter_data_address_error_exception` only for sealed data address errors; no indexed COP0 write API | C++ exists, Rust intentionally narrow | Source inspection; C++ proof helpers `write_cop0_register_through_cpu`; seam 039 tests | MTC0, timer, reset, interrupt, ERET, and generic exception mutation remain absent; seam 039 earns only the local data address-error entry mutation. |
| Rust COP0 staging/mutation behavior | C++ private mutation helpers listed above | No `Cpu::stage_cop0_*`, no `Cop0` mutation API | Not yet earned | Rust API inspection | No mutation was added in this pass. |
| Valid COP0 field/index behavior | C++ supports MFC0 reads for BadVAddr, Count, Compare, Status, Cause, EPC; MTC0 writes for Count, Compare, Status, Cause, EPC | No Rust indexed COP0 API | C++ exists, Rust intentionally absent | `machine_cpu.cpp` COP0 switch source | Register-index behavior belongs to MFC0/MTC0 instruction seams. |
| Invalid COP0 field/index behavior | Unsupported COP0 indices/forms return `kUnsupported` through execution; proof checks no-ghost behavior | No Rust indexed COP0 API or error type | C++ exists, Rust intentionally absent | `execute_cpu_instruction`; `run_cop0_unsupported_no_ghost_demo` source inspection | No Rust-only invalid-index safety was added because no Rust indexed API exists. |
| COP0 access independent of instruction execution | C++ private read helpers are pure reads, but public observations route through MFC0/step | Rust `Cpu::cop0_*` inspectors are pure `&self` reads | Equivalent for construction inspectors only | Rust access test; C++ source inspection | Rust accessors are not MFC0 semantics and do not imply execution readiness. |
| COP0 access changes GPRs | C++ MFC0 writes a GPR target during instruction execution | Rust `Cpu::cop0_*` accessors do not touch GPRs | Equivalent for Rust direct access only; MFC0 not in scope | `cop0_construction_access_does_not_change_earned_cpu_state`; C++ MFC0 source | The C++ public proof read path changes GPRs because it executes MFC0. Rust direct access is construction inspection, not instruction transfer. |
| COP0 access changes PC / next PC | C++ MFC0 read path advances PC/next PC through step cadence | Rust `Cpu::cop0_*` accessors do not touch PC/next PC | Equivalent for Rust direct access only; step not in scope | Rust access test; C++ step/proof helper inspection | PC cadence belongs to step/execution, not this access seam. |
| COP0 access changes HI / LO | C++ COP0 read helpers do not assign HI/LO; MFC0 path does not model HI/LO mutation | Rust `Cpu::cop0_*` accessors do not touch HI/LO | Equivalent for direct access | Rust access test; source inspection | No arithmetic or HI/LO instruction behavior is claimed. |
| COP0 access touches RDRAM or cartridge | C++ proof reads stage MFC0 instructions in RDRAM; private read helpers do not touch RDRAM or cartridge | Rust `Cpu::cop0_*` accessors have no Machine/RDRAM/cartridge access | Equivalent for direct access; C++ MFC0 staging not in scope | Rust API inspection; proof helper source | Rust Machine exposes only `cpu() -> &Cpu`, so COP0 access cannot mutate Machine-owned Cartridge or RDRAM facts. |
| COP0 mutation independent of instruction execution | C++ has private helpers, but public mutation is through MTC0 instruction execution or exception/interrupt/ERET paths | No Rust mutation | Not yet earned | Source inspection | Unlike GPR/scalar public `stage_cpu_*`, there is no public `stage_cop0_*` source seam. |
| COP0 mutation changes GPRs | C++ MTC0 consumes GPR source values through instruction execution; MFC0 writes GPR targets | No Rust mutation | Not in scope | `machine_cpu.cpp` MFC0/MTC0 switch | This is instruction read/writeback behavior and remains absent. |
| COP0 mutation changes PC / next PC | Exception/interrupt entry and ERET mutate COP0 and PC/next PC together | No Rust mutation | Not in scope | `try_enter_local_interrupt`, exception entry, `return_from_local_interrupt_entry` source inspection | Rust must not model this before reset/exception/step control flow is earned. |
| COP0 mutation changes HI / LO | C++ COP0 helpers do not assign HI/LO directly | No Rust mutation | Not yet earned | Source inspection | This pass adds no mutation test because no mutation API exists. |
| COP0 mutation touches RDRAM or cartridge | C++ COP0 helpers do not assign `rdram_` or `cartridge_`, but public mutation path requires staged instructions in RDRAM | No Rust mutation; no RDRAM/cartridge touch | Not yet earned | C++ proof helper inspection; Rust API inspection | Public COP0 mutation remains entangled with instruction staging and step. |
| COP0 reset behavior | `reset_to_non_boot_power_on_state` clears local COP0 fields | `Machine::reset` replaces represented CPU/COP0 state with `Cpu::new` / `Cop0::new` | Equivalent for represented COP0 reset fields | Seam 042 reset audit; reset tests; source inspection | Rust reset clears Count, Compare, Status, software pending, EPC, BadVAddr, exception code, timer pending, and branch-delay flag. Broader COP0 instruction/timer/interrupt reset behavior remains absent. |
| Exception/interrupt behavior | C++ local exception/interrupt helpers write EPC, Cause fields, Status EXL, PC, next PC | No Rust exception/interrupt APIs | Not in scope | `try_enter_local_interrupt`; exception-entry source inspection | No exception or interrupt readiness is claimed. |
| TLB/MMU behavior | C++ comments/proof keep TLB/MMU unearned for current local COP0 seam | No Rust TLB/MMU API | Not in scope | Source inspection | No TLB or MMU behavior was added. |
| Timer/count behavior | C++ `advance_cop0_count_after_committed_instruction`, Compare write clearing pending, and timer pending are step/timer behavior | Rust now owns only crate-private Count advancement plus timer-pending latch; Compare write remains absent | Narrow Count primitive earned; broader timer/COP0 writes blocked | `advance_cop0_count_after_committed_instruction`; Count tests; timer proof source | Rust owns construction zero values and the committed-step Count helper only. Interrupt delivery, Compare writes, and MTC0 remain absent. |
| MFC0/MTC0 instruction behavior | C++ `execute_cpu_instruction` implements narrow MFC0/MTC0 forms | No Rust instruction representation or COP0 transfer API | Not in scope | C++ proof demos; no Rust execution APIs | Rust read-only accessors are not MFC0/MTC0 semantics. |
| ERET behavior | C++ `local_eret_can_return` and `return_from_local_interrupt_entry` | No Rust ERET API | Not in scope | Source inspection | ERET mutates Status and PC/next PC; it is execution/control-flow behavior. |
| Rust-only API safety | C++ unsupported COP0 indices report through step result, not a safe indexed API | No Rust COP0 error type | Not applicable | Rust API inspection | No Rust-only COP0 invalid-index safety was added. |
| Naming/layout changes for this seam | C++ keeps COP0 fields in `Machine` monolith | `cpu/cop0.rs` private owner; this ledger | Rust-only repo hygiene, no emulator truth | Source inspection; `cargo test` | Seam 014 kept the existing semantic owner. No README/module rename or broad helper module was added. |

## Seam 014 Audit Changes

- Re-audited C++ COP0 construction fields, private read helpers, MFC0/MTC0
  dispatch, step probe reads, and proof demos.
- Kept Rust `Cop0` private under `cpu/cop0.rs`; `lib.rs` does not export it.
- Did not add Rust `read_cop0_status` or `read_cop0_cause` accessors. The C++
  default-zero result is construction-backed, but nonzero masking/derived Cause
  behavior requires unearned COP0 mutation, MI state, timer state, exception
  state, branch-delay state, or instruction stepping.
- Added a construction/access test proving Rust COP0 inspection does not mutate
  earned CPU GPR/scalar state.
- No C++ source files were changed.

## COP0 Derived-Read/Mutation Readiness Audit Table

| Behavior/path | C++ owner file/function/field | Rust owner file/function/field | Reads | Writes | Pure read? | Inputs earned? | Blockers | Recommended status | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `read_cop0_status` | `src/core/machine_cpu.cpp` `read_cop0_status`; `machine.hpp` `cop0_status_`, `kCop0SupportedStatusBits` | No Rust `read_cop0_status`; `cpu/cop0.rs` direct `Cpu::cop0_status` | Stored Status plus supported-bit mask constant | None | Yes | Default zero only; stored field exists, nonzero writes absent | COP0 mutation and MTC0/instruction path for nonzero tests | Not yet earned | Source is simple, but adding a C++-named derived read now would overclaim nonzero masking that cannot be exercised without mutation. |
| `read_cop0_cause` | `src/core/machine_cpu.cpp` `read_cop0_cause`; COP0 Cause constants; MI pending/mask fields | No Rust `read_cop0_cause` or `cop0_cause` | exception code, software pending, MI pending/mask, timer pending, branch-delay flag | None | Yes | Only default zero inputs are earned | Blocked by exception/interrupt seam; Blocked by timer seam; Blocked by instruction seam; MI/device state absent | Not yet earned | Cause is a derived view, not direct storage. Nonzero cases require multiple unearned owners. |
| Count direct storage/access | `machine.hpp` `cop0_count_`; `machine_cpu.cpp` `read_cop0_count` | `cpu/cop0.rs` `count`; `Cpu::cop0_count`; `Cpu::advance_count_for_committed_step` | Count | Count through the narrow committed-step helper | Yes for reads | Yes for construction/access and Count helper inputs | MTC0 Count writes and step wiring absent | Narrow Count helper earned | Count is represented as construction/access state and can now advance through the sealed committed-step helper. |
| Compare direct storage/access | `machine.hpp` `cop0_compare_`; `machine_cpu.cpp` `read_cop0_compare` | `cpu/cop0.rs` `compare`; `Cpu::cop0_compare` | Compare | None | Yes | Yes for construction/access | Compare writes and pending-clear behavior unearned | Documentation only | Already represented as construction/access state only. |
| Timer pending direct storage/access | `machine.hpp` `cop0_timer_interrupt_pending_`; `read_cop0_cause` input | `cpu/cop0.rs` `timer_interrupt_pending`; `Cpu::cop0_timer_interrupt_pending`; `Cpu::advance_count_for_committed_step` | timer pending flag | Latches true through Count/Compare equality | Yes for Rust direct accessor | Yes for construction false and Count helper latch | Compare write clear and interrupt delivery absent | Narrow Count helper latch earned | Rust stores false construction state and now latches pending on Count equality; it does not clear pending or deliver interrupts. |
| Status direct storage/access | `machine.hpp` `cop0_status_`; `read_cop0_status`; interrupt gate reads Status | `cpu/cop0.rs` `status`; `Cpu::cop0_status` | stored Status | None for direct access | Yes | Yes for construction zero only | COP0 write and interrupt gate behavior absent | Not yet earned | Direct construction access is earned; nonzero Status semantics are not. |
| Software pending direct storage/access | `machine.hpp` `cop0_software_interrupt_pending_`; `read_cop0_cause` input | `cpu/cop0.rs` `software_interrupt_pending`; `Cpu::cop0_software_interrupt_pending` | software pending bits | None for direct access | Yes | Yes for construction zero only | `write_cop0_cause` and software interrupt behavior absent | Not yet earned | Stored field exists in Rust, but Cause composition from nonzero bits is unearned. |
| EPC direct storage/access | `machine.hpp` `cop0_epc_`; `read_cop0_epc` | `cpu/cop0.rs` `epc`; `Cpu::cop0_epc` | EPC | None for direct access | Yes | Yes for construction zero only | Exception/interrupt entry, ERET, and MTC0 EPC write absent | Not yet earned | Construction access is earned; EPC behavior is not. |
| BadVAddr direct storage/access | `machine.hpp` `cop0_bad_vaddr_`; `read_cop0_bad_vaddr` | `cpu/cop0.rs` `bad_vaddr`; `Cpu::cop0_bad_vaddr`; `Cpu::enter_data_address_error_exception` | BadVAddr | None for direct access | Yes | Yes for construction zero and seam 039 data address-error entry | Generic exception entry absent | Narrow data address-error entry earned | Nonzero BadVAddr mutation is earned only for sealed `CpuDataAddressError`. |
| Exception code direct storage/access | `machine.hpp` `cop0_exception_code_`; `read_cop0_cause` input | `cpu/cop0.rs` `exception_code`; `Cpu::cop0_exception_code`; `Cpu::enter_data_address_error_exception` | exception code | None for direct access | Yes | Yes for construction zero and seam 039 data address-error entry | Generic exception/interrupt entry absent | Narrow data address-error entry earned | Nonzero local exception-code mutation is earned only for sealed data address errors. |
| Branch-delay flag direct storage/access | `machine.hpp` `cop0_exception_branch_delay_`; `read_cop0_cause` input | `cpu/cop0.rs` `exception_branch_delay`; `Cpu::cop0_exception_branch_delay`; `Cpu::enter_data_address_error_exception` | branch-delay flag | None for direct access | Yes | Yes for construction false and seam 039 data address-error entry | Delay-slot instruction behavior absent | Narrow data address-error entry earned | Rust mirrors C++ entry guard and branch-delay flag mutation, but does not execute delay slots. |
| `write_cop0_count` | `machine_cpu.cpp` `write_cop0_count` | No Rust mutation API | Input value | Count | No | No, because writes absent | MTC0/instruction path; Count ticking interactions | Not yet earned | Private helper is source-clear, but current C++ public mutation reaches it through MTC0 execution. |
| `write_cop0_compare` | `machine_cpu.cpp` `write_cop0_compare` | No Rust mutation API | Input value | Compare and timer pending clear | No | No | MTC0/instruction path; timer pending clear behavior | Blocked by timer seam | This helper couples direct storage assignment with timer pending ownership. |
| `write_cop0_status` | `machine_cpu.cpp` `write_cop0_status`; `kCop0SupportedStatusBits` | No Rust mutation API | Input value and mask constant | stored Status masked to supported bits | No | No | MTC0/instruction path; interrupt gate behavior | Not yet earned | Masking is source-backed but needs an earned mutation seam before derived reads can prove nonzero cases. |
| `write_cop0_cause` | `machine_cpu.cpp` `write_cop0_cause`; software pending mask | No Rust mutation API | Input value and software pending mask | software pending bits | No | No | MTC0/instruction path; software interrupt/Cause composition | Not yet earned | C++ write only owns IP0/IP1 software bits, not full Cause storage. |
| `write_cop0_epc` | `machine_cpu.cpp` `write_cop0_epc` | No Rust mutation API | Input value | EPC | No | No | MTC0/instruction path; ERET and exception entry relationships | Not yet earned | EPC writes are source-clear but not public staging truth. |
| Indexed COP0 read path | `execute_cpu_instruction` `kCop0Mfc0` switch over BadVAddr, Count, Compare, Status, Cause, EPC | No Rust indexed COP0 API | COP0 register index, selected read helper | GPR target through writeback | No | No | Blocked by instruction seam | Blocked by instruction seam | MFC0 is instruction transfer/writeback behavior, not direct access. |
| Indexed COP0 write path | `execute_cpu_instruction` `kCop0Mtc0` switch over Count, Compare, Status, Cause, EPC | No Rust indexed COP0 API | COP0 register index, GPR source word | selected COP0 field/helper side effects | No | No | Blocked by instruction seam; timer/status/cause side effects | Blocked by instruction seam | MTC0 is the current public C++ mutation path. |
| Invalid indexed COP0 path | MFC0/MTC0 default cases return `kUnsupported`; proof `run_cop0_unsupported_no_ghost_demo` | No Rust indexed COP0 API or error | unsupported instruction/index | None on unsupported path | No, because it is execution result behavior | No | Blocked by instruction seam | Blocked by instruction seam | No Rust-only invalid-index safety should be added until an indexed API exists. |
| Count advance/timer latch | `advance_cop0_count_after_committed_instruction` | `Cpu::advance_count_for_committed_step` | Count, Compare | Count increment; timer pending true when equal | No | Yes for represented Count/Compare/timer-pending storage | Step outcome wiring, Compare writes, and interrupt delivery remain absent | Narrow primitive earned | Called by C++ step after committed instructions and ERET; Rust owns the helper but does not call it from any step path. |
| Interrupt pending/gate derived reads | `local_cop0_interrupt_pending_lines`, `local_interrupt_pending`, `local_interrupt_enabled` | No Rust interrupt gate API | derived Cause, Status, pending masks | None | Yes | No | Blocked by exception/interrupt seam; timer and MI state absent | Blocked by exception/interrupt seam | These helpers consume derived Cause and Status to decide interrupt entry. |
| Local interrupt mutation path | `try_enter_local_interrupt` | No Rust interrupt API | Status, pending lines, PC/next PC, address translation | EPC, exception code, branch-delay flag, Status EXL, PC, next PC | No | No | Blocked by exception/interrupt seam; memory/address translation absent | Blocked by exception/interrupt seam | Mutates COP0 and control-flow state together. |
| Local exception mutation paths | `enter_local_signed_overflow_exception`, `enter_local_address_error_exception` | `Cpu::enter_data_address_error_exception`, crate-private `Cpu::enter_instruction_fetch_address_error_exception`, and crate-private `Cpu::enter_arithmetic_overflow_exception` for sealed narrow sources only | fault PC, BadVAddr when relevant, exception code, branch-delay input | EPC, BadVAddr when relevant, exception code, branch-delay flag, Status EXL, PC, next PC | Partial | Yes for sealed data address-error, selected instruction-fetch address-error, and signed-overflow entry primitives | Generic exception, interrupt, and broad instruction/fault behavior absent | Narrow local entry primitives earned for sealed sources only | Broader exception machinery remains absent; trapping arithmetic helpers still do not wire overflow outcomes to entry. |
| ERET mutation path | `local_eret_can_return`; `return_from_local_interrupt_entry` | No Rust ERET API | Status EXL, EPC, PC/next PC | Status EXL clear, PC, next PC | No | No | Blocked by instruction seam; Blocked by exception/interrupt seam | Blocked by instruction seam | ERET is instruction/control-flow behavior. |
| Reset COP0 mutation path | `Machine::reset_to_non_boot_power_on_state` | `Machine::reset` via `Cpu::new` / `Cop0::new` | none beyond represented Machine reset ownership | all represented COP0 fields zero/false | No RDRAM/device access | Yes for represented COP0 reset fields | Generic exception, interrupt, ERET, MTC0, and Compare-write behavior absent | Represented Machine reset earned | Rust mirrors construction defaults and now restores them through Machine-owned reset without adding broad COP0 mutation APIs. |
| Rust private COP0 ownership | C++ has no standalone COP0 type; fields are Machine-owned | `cpu/cop0.rs` private `Cop0`; `cpu.rs` private `cop0: Cop0` | construction/access fields | none | Yes for accessors | Yes for construction/access only | None for current surface | Rust-only repo hygiene, no emulator truth | Private owner remains valid as a sidecar structure. |
| Rust public COP0 surface through `Cpu` | C++ private helpers plus MFC0 proof reads | `Cpu::cop0_*` construction/accessors only | direct stored construction fields | none | Yes | Yes for construction/access only | Derived reads, mutation, indexed APIs absent by design | Documentation only | Surface is intentionally narrow and source-backed. |
| Why no Rust behavior was added in seam 015 | C++ derived reads and writes are source-backed but coupled to unearned owners | No Rust code change | N/A | N/A | N/A | N/A | Derived reads need nonzero inputs; mutation needs instruction/timer/exception/reset context | Documentation only | This pass only maps readiness. It does not claim derived-read parity or mutation readiness. |

## Seam 015 Audit Changes

- Added the COP0 derived-read/mutation readiness audit map above.
- Left Rust `cpu/cop0.rs`, `cpu.rs`, `machine.rs`, `lib.rs`, and README behavior
  unchanged.
- Found no speculative Rust COP0 mutation, indexed COP0 API, or derived
  Status/Cause accessor to remove.
- Recommended moving the next behavior seam away from COP0 unless the next pass
  explicitly earns mutation/input state. RDRAM storage access is the cleaner next
  candidate because current C++ has direct storage inspection/staging surfaces,
  while COP0 non-default derived reads are coupled to instruction, timer,
  exception/interrupt, MI/device, or reset ownership.
- No C++ source files were changed.

## RDRAM Storage Access Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ RDRAM storage ownership | `src/core/machine.hpp` `Machine::rdram_` | `rust/crates/fn64-core/src/rdram.rs` `Rdram::bytes`; `machine.rs` `Machine::rdram` | Equivalent storage semantics, different ownership shape | `default_rdram_has_cpp_construction_size`; `default_rdram_storage_is_zero_filled`; source inspection | C++ stores bytes directly on `Machine`; Rust uses an owned `Rdram` sidecar field under `Machine`. |
| Rust RDRAM storage ownership | C++ Machine-owned `std::array<std::uint8_t, kRdramSizeBytes>` | `Rdram` private `Vec<u8>` | Equivalent for current owned-storage facts | RDRAM and Machine tests | Rust remains a sidecar owner; no C++ depends on it. |
| RDRAM default size | `machine.hpp` `kRdramSizeBytes = 4 * 1024 * 1024`; `Machine::rdram_size_bytes` | `rdram.rs` `RDRAM_SIZE_BYTES`; `Rdram::size_bytes` | Equivalent | `default_rdram_has_cpp_construction_size` | Both represent 4 MiB. |
| RDRAM zero-filled construction/reset | `machine.cpp` `reset_to_non_boot_power_on_state` calls `rdram_.fill(0)` during construction/reset | `Rdram::default` allocates zero-filled bytes; `Machine::reset` replaces represented RDRAM with that default | Equivalent for construction and represented reset zero-fill | `default_rdram_storage_is_zero_filled`; reset tests; C++ `require_non_boot_reset_power_on_state` | Rust reset clears represented RDRAM bytes without adding range access, DMA, memory-map, or device reset behavior. |
| C++ raw read behavior | `machine.cpp` private `Machine::read_rdram_u8`, `read_rdram_u16_be`, `read_rdram_u32_be`, and `read_rdram_u64_be` | `rdram.rs` `Rdram::read_u8`, `read_u16_be`, `read_u32_be`, and `read_u64_be` | Equivalent storage semantics, different ownership shape | RDRAM byte-read tests; raw read-width tests; source inspection | C++ helpers are private Machine methods; Rust exposes pure storage inspection on `Rdram` because reads do not mutate reservation or other Machine state. |
| Rust raw read behavior | C++ private read helpers; proof/CPU/DMA callers consume them through C++ owners | `Rdram::read_u8/u16_be/u32_be/u64_be` return `Result<_, RdramAccessError>` | Equivalent for valid storage offsets; Rust-only API safety for invalid offsets | RDRAM byte-read and read-width tests | Rust uses `offset` in the API because this is storage, not a CPU address. Display text mirrors C++ `fail_rdram_access`. |
| C++ raw byte write behavior | `machine.cpp` private `Machine::write_rdram_u8` writes a byte after `invalidate_cpu_rdram_reservation_for_write(address, 1)` | `machine.rs` `Machine::write_rdram_u8` | Equivalent behavior, different ownership shape | Raw byte-write tests; source inspection | C++ byte writes are not pure storage writes because they can clear local LL/SC reservation state. Rust mirrors the reservation-aware byte write seam. |
| Rust raw byte write behavior | C++ `write_rdram_u8` plus reservation invalidation | `Machine::write_rdram_u8` plus private `Rdram::write_u8_at_checked_offset` | Equivalent behavior, different ownership shape | Raw byte-write tests | Rust does not expose a public storage-only byte write that could bypass reservation invalidation. |
| C++ range read behavior | DMA loops use `read_rdram_u8` over ranges after SP/SI/AI preflight | No Rust range read API | C++ exists, Rust intentionally absent | Source inspection | C++ range reads are owned by DMA/device paths, not a standalone public raw range seam. |
| Rust range read behavior | C++ DMA/device range consumers | Intentionally absent | Not yet earned | Source inspection | Range APIs are deferred until a source-backed raw range seam is earned. |
| C++ range write behavior | `stage_cartridge_bytes_to_rdram`, SP write DMA, SI PIF-to-DRAM DMA, CPU stores loop/write through RDRAM helpers | No Rust range write API | C++ exists, Rust intentionally absent | Source inspection | Range writes use `write_rdram_u8`, so they can invalidate reservations and are tied to staging/DMA/load-store owners. |
| Rust range write behavior | C++ range write paths above | Intentionally absent | Not yet earned | Source inspection | No range write was added. |
| Invalid offset behavior | `machine.cpp` `fail_rdram_access` throws `std::out_of_range` with address and width | `RdramAccessError` with `offset`, `width`, and matching display text | Rust-only API safety, no emulator truth | `byte_read_out_of_range_is_explicit_rust_api_safety`; source inspection | Rust returns `Result` instead of throwing. The text mirrors C++ for auditability. |
| Invalid read-width range behavior | `read_rdram_u16_be/u32_be/u64_be` width guards reject partial windows at the end of RDRAM | `Rdram::read_u16_be/u32_be/u64_be` reject partial windows with `RdramAccessError` | Equivalent storage boundary; Rust-only API safety for error carrier | Raw read-width invalid-offset tests | This is fixed-width raw storage reading only; arbitrary range access remains absent. |
| First-byte access | `read_rdram_u8(0)` accepts offset 0 | `Rdram::read_u8(0)` | Equivalent | `byte_read_returns_default_storage_bytes_by_offset_without_mutation`; Machine read-preservation test | Offset 0 succeeds. |
| Last-byte access | `read_rdram_u8(kRdramSizeBytes - 1)` accepts the last byte | `Rdram::read_u8(RDRAM_SIZE_BYTES - 1)` | Equivalent | `byte_read_returns_default_storage_bytes_by_offset_without_mutation`; Machine read-preservation test | Last byte succeeds for byte-width reads. |
| Exact-end read-width behavior | C++ width guards allow fixed-width reads ending exactly at `rdram_.size()` | Rust accepts `RDRAM_SIZE_BYTES - 2`, `- 4`, and `- 8` for u16/u32/u64 reads | Equivalent | Raw read-width last-valid tests | Arbitrary range access is still absent. |
| Endianness at raw storage level | Byte read/write helpers have no endian conversion; u16/u32/u64 helpers compose/store big-endian words | Rust byte read/write has no byte-level conversion; Rust u16_be, u32_be, and u64_be reads and writes use high-to-low byte order | Equivalent for byte access, raw read widths, and raw write widths | RDRAM byte tests; raw read-width tests; raw u16_be/u32_be/u64_be write tests; source inspection | Multi-byte raw reads/writes are storage-order helpers only, not CPU load/store behavior. |
| Storage offset vs emulated address | C++ raw helpers take `RdramOffset`; CPU paths translate CpuAddress separately | Rust `Rdram::read_*` and `Machine::write_rdram_*` use `usize` storage offsets | Equivalent for valid storage offsets; API shape differs | Source inspection; Rust API inspection | This is not CPU address translation or memory-map behavior. |
| Whether writes affect CPU/RDRAM reservation state | All C++ RDRAM write helpers call `invalidate_cpu_rdram_reservation_for_write` | Rust raw byte/u16_be/u32_be/u64_be writes call private `CpuRdramReservation::invalidate_for_rdram_write` | Equivalent for earned write widths; range absent | Raw write and reservation invalidation tests | Private invalidation is earned and used by raw byte, u16_be, u32_be, and u64_be writes. |
| Whether writes affect CPU state | C++ raw write helpers can affect reservation state; CPU stores also write GPR results for SC/SCD paths | Rust raw byte/u16_be/u32_be/u64_be writes mutate RDRAM plus reservation only | Equivalent for represented raw helper state; CPU load/store absent | Raw write preservation tests; source inspection | Rust does not model load/store, SC/SCD, or instruction result writeback. |
| Whether access changes Cartridge facts | C++ `read_rdram_u8` does not touch `cartridge_` | Rust `Rdram::read_u8` cannot access Cartridge | Equivalent for read-only access | `rdram_byte_read_does_not_change_machine_owned_facts` | Machine retains cartridge facts after RDRAM byte reads. |
| Whether access changes COP0 state | C++ `read_rdram_u8` does not touch COP0 fields | Rust `Rdram::read_u8` cannot access CPU/COP0 | Equivalent for read-only access | `rdram_byte_read_does_not_change_machine_owned_facts` | Rust test checks COP0 construction/access fields remain unchanged. |
| Whether reset behavior is in scope | `reset_to_non_boot_power_on_state` clears RDRAM and many other owners | `Machine::reset` clears represented RDRAM plus CPU/reservation/powered_on state | Equivalent for represented Machine-owned reset state; C++ exists beyond Rust scope | Reset audit; reset tests; source inspection | SP/PIF/device reset remains intentionally absent because those owners do not exist in Rust. |
| Whether CPU load/store behavior is in scope | C++ CPU memory paths call RDRAM helpers after address resolution | No Rust load/store API | Not in scope | Source inspection | CPU load/store behavior remains unearned. |
| Whether memory-map behavior is in scope | `require_cpu_data_target` and translation helpers resolve CPU data targets | No Rust memory-map API | Not in scope | Source inspection | This seam uses storage offsets only. |
| Whether DMA behavior is in scope | SP/SI/PI/AI DMA paths consume RDRAM helpers | No Rust DMA API | Not in scope | Source inspection | DMA range behavior remains intentionally absent. |
| Whether bus behavior is in scope | C++ has local target resolver, not a Rust bus | No Rust bus API | Not in scope | Source inspection | No bus abstraction was added. |
| Machine-level forwarding | C++ public `Machine::inspect_rdram_u32_be` / `stage_rdram_u32_be`; private raw read/write helpers on Machine | Rust exposes `Machine::write_rdram_u8`, `Machine::write_rdram_u16_be`, `Machine::write_rdram_u32_be`, and `Machine::write_rdram_u64_be`; existing `Machine::rdram() -> &Rdram` exposes pure storage reads | Equivalent behavior for earned raw storage seams; different ownership shape for reads | Rust API inspection; raw read/write tests | Rust exposes pure reads through the RDRAM owner and reservation-aware raw writes through Machine. It does not add `read_rdram_*` forwarding, range access, CPU load/store, DMA, reset, or execution forwarding. |
| Rust-only API safety | C++ throws on invalid RDRAM access | `RdramAccessError`; `Result` return | Rust-only API safety, no emulator truth | Error test | Error type is domain-specific and explicit; it does not add emulator behavior beyond the read seam. |
| Naming/layout changes | C++ monolith keeps RDRAM on `Machine`; Rust keeps owner in `rdram.rs` | `rdram.rs`, `lib.rs`, `README.md`, `PARITY.md` | Rust-only repo hygiene, no emulator truth | Source inspection | No broad `memory`, `bus`, `util`, or helper module was added. |

## Seam 016 Audit Changes

- Audited C++ `Machine::rdram_`, private `read_rdram_u8`, private
  `write_rdram_u8/u16/u32/u64`, public `inspect_rdram_u32_be`,
  `stage_rdram_u32_be`, cartridge-to-RDRAM staging, CPU memory paths, DMA paths,
  reset, and CPU/RDRAM reservation helpers.
- Added `Rdram::read_u8(offset)` and `RdramAccessError` for the source-backed,
  side-effect-free raw byte read subset only.
- Exported `RdramAccessError` from `lib.rs` because the public Rust RDRAM read
  API returns it.
- Did not add RDRAM writes because C++ writes invalidate overlapping local
  CPU/RDRAM reservations. Later seams added construction/default, private
  staging, and private invalidation reservation state, but write APIs remain
  absent.
- Did not add range reads/writes, Machine forwarding,
  memory mapping, CPU load/store behavior, DMA, reset, bus, reservation
  invalidation behavior, instruction execution, renderer, SDL, host shell, or
  C++ integration.
- Updated README scope text to say RDRAM has read-only raw byte storage access
  and still has no writes.
- No C++ source files were changed.

## RDRAM Raw Byte Read Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ RDRAM storage ownership | `src/core/machine.hpp` `Machine::rdram_` | `rust/crates/fn64-core/src/rdram.rs` `Rdram::bytes`; `machine.rs` `Machine::rdram` | Equivalent storage semantics, different ownership shape | Source inspection; RDRAM construction tests | C++ owns bytes directly on `Machine`; Rust owns bytes in a `Rdram` sidecar field owned by `Machine`. |
| Rust RDRAM storage ownership | C++ Machine-owned `rdram_` | `Rdram` private `bytes: Vec<u8>` | Equivalent for earned storage facts | RDRAM construction and byte-read tests | No standalone C++ RDRAM type exists. |
| RDRAM default size | `machine.hpp` `kRdramSizeBytes`; `Machine::rdram_size_bytes` | `RDRAM_SIZE_BYTES`; `Rdram::size_bytes` | Equivalent | `default_rdram_has_cpp_construction_size` | Both are 4 MiB. |
| RDRAM zero-filled construction/reset | `machine.cpp` `rdram_.fill(0)` during construction/reset helper | `Rdram::default`; `Machine::reset` | Equivalent for construction and represented reset zero-fill | `default_rdram_storage_is_zero_filled`; reset tests; source inspection | Rust reset clears represented RDRAM bytes without implying range access, DMA, memory map, bus, or device reset. |
| C++ raw byte read behavior | `machine.cpp` `Machine::read_rdram_u8(RdramOffset)` | `Rdram::read_u8(offset)` | Equivalent source semantics for storage-offset byte read; API differs | Source inspection; `byte_read_returns_default_storage_bytes_by_offset_without_mutation` | C++ returns `rdram_[address]` after a range check. Rust uses `Result` and `offset` naming. |
| Rust raw byte read behavior | C++ private `read_rdram_u8` | `Rdram::read_u8` | Equivalent for valid default-zero reads; nonzero content not directly tested | Rust RDRAM tests | Rust has no earned write or staging API, so tests do not inject nonzero RDRAM bytes. |
| First-byte read | `read_rdram_u8(0)` | `Rdram::read_u8(0)` | Equivalent | `byte_read_returns_default_storage_bytes_by_offset_without_mutation`; Machine read-preservation test | Offset 0 succeeds. |
| Last-byte read | `read_rdram_u8(kRdramSizeBytes - 1)` | `Rdram::read_u8(RDRAM_SIZE_BYTES - 1)` | Equivalent | `byte_read_returns_default_storage_bytes_by_offset_without_mutation`; Machine read-preservation test | The last byte succeeds for byte-width reads. |
| Exact-length out-of-range read | `read_rdram_u8(kRdramSizeBytes)` throws via `fail_rdram_access` | `Rdram::read_u8(RDRAM_SIZE_BYTES)` returns `Err(RdramAccessError)` | Rust-only API safety, no emulator truth | `byte_read_out_of_range_is_explicit_rust_api_safety`; source inspection | C++ throws; Rust returns explicit `Result`. |
| Past-end out-of-range read | `read_rdram_u8(address >= kRdramSizeBytes)` throws via `fail_rdram_access` | `Rdram::read_u8(RDRAM_SIZE_BYTES + 1)` returns `Err(RdramAccessError)` | Rust-only API safety, no emulator truth | `byte_read_out_of_range_is_explicit_rust_api_safety`; source inspection | Error carries offset and width. |
| Invalid offset behavior | `fail_rdram_access` throws `std::out_of_range` with address and width | `RdramAccessError` with `offset`, `width`, and display text | Rust-only API safety, no emulator truth | Error test | Display text mirrors C++ text, but Rust does not throw. |
| Invalid offset classification | C++ exception behavior | Rust `Result` error | Rust-only API safety, no emulator truth | Source inspection; error test | The storage bounds are source-backed; the Rust error carrier is API shape. |
| Storage offset vs emulated address | Raw helper takes `RdramOffset`; CPU paths translate addresses elsewhere | `read_u8(offset)` | Equivalent for storage-offset seam | Source inspection | The Rust API deliberately avoids `address`, `load`, `bus`, or `cpu` names. |
| Byte-read endianness | `read_rdram_u8` returns one byte with no conversion | `read_u8` returns one byte with no conversion | Equivalent | Source inspection; Rust tests | Endianness is absent at the byte level. |
| Read mutates RDRAM | C++ `read_rdram_u8` is `const` and does not write `rdram_` | Rust `read_u8(&self)` does not mutate `bytes` | Equivalent | `byte_read_returns_default_storage_bytes_by_offset_without_mutation` | Rust test verifies reads leave zero-filled storage unchanged. |
| Read affects CPU/RDRAM reservation state | C++ reads do not call reservation helpers; writes do | Rust has private construction/default, staging, and invalidation reservation state, and `Rdram::read_u8` has no access to mutate it | Equivalent for read-only seam | Source inspection; Rust invalidation tests | RDRAM reads remain side-effect free; private invalidation is not reachable from read-only storage access. |
| Read affects CPU state | C++ raw read helper does not assign CPU fields | Rust `Rdram::read_u8` has no CPU access | Equivalent for read-only seam | `rdram_byte_read_does_not_change_machine_owned_facts`; source inspection | CPU GPRs, zero register, PC, next PC, HI, LO, and COP0 fields remain unchanged. |
| Read changes Cartridge facts | C++ raw read helper does not touch `cartridge_` | Rust `Rdram::read_u8` has no Cartridge access | Equivalent | `rdram_byte_read_does_not_change_machine_owned_facts`; source inspection | Cartridge metadata and bytes are preserved. |
| Read changes COP0 state | C++ raw read helper does not assign COP0 fields | Rust `Rdram::read_u8` has no CPU/COP0 access | Equivalent | `rdram_byte_read_does_not_change_machine_owned_facts`; source inspection | COP0 construction/access fields remain unchanged. |
| Nonzero storage reads | C++ source would return the stored nonzero byte if storage contains one | No Rust write/staging API to create nonzero RDRAM bytes | Not yet earned beyond source-level read semantics | Source inspection only | Removed the seam 016 private-byte injection test so Rust tests do not overclaim nonzero content parity. |
| C++ raw byte write behavior | `write_rdram_u8` validates, invalidates overlapping reservation, then writes | No Rust write API | C++ exists, Rust intentionally absent | Source inspection | Private invalidation is earned, but storage mutation and write-call behavior remain absent. |
| Rust write absence | C++ write helpers and public staging exist | No `Rdram::write_u8`, no `stage_byte`, no mutable Machine RDRAM API | C++ exists, Rust intentionally absent | Source inspection; Rust API inspection | No write readiness is claimed. |
| Range read absence | DMA paths loop over `read_rdram_u8`; no Rust range read | No Rust range API | Not yet earned | Source inspection | Range behavior is owned by DMA/device paths, not this read-only seam. |
| Range write absence | Staging/DMA paths loop over write helpers | No Rust range write API | Not yet earned | Source inspection | Range writes would inherit reservation side effects. |
| Reset behavior | C++ reset clears RDRAM and many other owners | `Machine::reset` clears represented RDRAM, CPU, reservation, and powered_on state | Equivalent for represented reset state; C++ exists beyond Rust scope | Reset audit; reset tests; source inspection | This read-parity table still does not claim SP/PIF/device reset parity or execution readiness. |
| CPU load/store behavior | C++ CPU memory paths translate and call RDRAM helpers | No Rust CPU load/store API | Not in scope | Source inspection | `read_u8` is not a CPU load. |
| Memory-map behavior | C++ target resolver maps CPU addresses to RDRAM/device targets | No Rust memory-map API | Not in scope | Source inspection | No address translation was added. |
| DMA behavior | SP/SI/AI/PI paths consume RDRAM helpers | No Rust DMA API | Not in scope | Source inspection | No DMA readiness is claimed. |
| Bus behavior | C++ has local target resolver, not Rust bus | No Rust bus API | Not in scope | Source inspection | No bus abstraction was added. |
| Machine-level forwarding | C++ public word inspect/stage lives on `Machine` | No new Rust Machine forwarding; existing `Machine::rdram() -> &Rdram` only | C++ exists, Rust intentionally absent for forwarding | Rust API inspection | Rust keeps RDRAM read on the `Rdram` owner and Machine remains construction/ownership surface. |
| Rust-only API safety | C++ throws on invalid raw byte read | `RdramAccessError` and `Result` | Rust-only API safety, no emulator truth | Error test | Domain-specific error; no catchall error type. |
| Naming/layout changes | C++ monolith stores RDRAM on `Machine` | `rdram.rs`, no new module | Rust-only repo hygiene, no emulator truth | Source inspection; rustfmt/clippy gates | Kept RDRAM code in `rdram.rs`; no `memory`, `bus`, `map`, `util`, or helper module was added. Formatting and lint-only cleanup did not change machine behavior. |

## Seam 017 Audit Changes

- Re-audited C++ raw byte reads, invalid read behavior, offset naming,
  CPU-memory separation, DMA consumers, reservation helpers, and proof/step-probe
  RDRAM observations.
- Kept `Rdram::read_u8(offset)` and `RdramAccessError`; no API rename was needed.
- Removed the seam 016 test-only private RDRAM byte injection. Rust tests now
  prove default-zero first/last byte reads and source-level read bounds without
  creating nonzero storage through an unearned write or staging path.
- Strengthened invalid-offset coverage for both exact-length and past-end reads.
- Applied rustfmt and clippy-only cleanup where current tooling made those gates
  active; this did not add emulator behavior or widen public machine APIs.
- Confirmed Machine has no RDRAM forwarding beyond existing read-only
  `Machine::rdram() -> &Rdram`.
- Did not add writes, ranges, reset, CPU load/store, memory map, bus, DMA,
  reservation invalidation behavior, cartridge-to-RDRAM staging, fetch, step,
  execution, renderer, SDL, host shell, or C++ integration.
- No C++ source files were changed.

## RDRAM Write / Reservation Readiness Audit

| Behavior/path | C++ owner file/function/field | Rust owner file/function/field | Reads | Writes | Side effects | Inputs earned? | Blockers | Recommended status | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| C++ RDRAM storage ownership | `src/core/machine.hpp` `Machine::rdram_`; `src/core/machine.cpp` reset/write helpers | `rust/crates/fn64-core/src/rdram.rs` `Rdram::bytes`; `machine.rs` private `rdram: Rdram` | C++ reads through raw helpers and higher owners | C++ writes through raw helpers and staging/DMA/store paths | Writes can clear CPU/RDRAM reservation | Partial: Rust owns storage, byte reads, reservation construction/default state, private reservation staging, and private reservation invalidation | RDRAM write APIs absent | Ready for RDRAM write decision after invalidation seal | Storage ownership is earned; write semantics are not storage-only in current C++. |
| Rust RDRAM storage ownership | C++ `Machine::rdram_` | `Rdram` owned by `Machine` | `Rdram::read_u8` only | No Rust write or range API | None for read-only seam | Yes for construction/read-only storage | Write side effects unowned | Documentation only | Rust remains a sidecar storage owner, not a memory-map or write owner. |
| C++ raw byte read path | `machine.cpp` `Machine::read_rdram_u8(RdramOffset)` | `Rdram::read_u8(offset)` | One storage byte after range check | None | None | Yes | None for read seam | Documentation only | Sealed in seam 017; included here as the read/write boundary baseline. |
| Rust raw byte read path | C++ private `read_rdram_u8` | `Rdram::read_u8` | One storage byte by offset | None | None | Yes | None for read seam | Documentation only | Rust API safety uses `Result`; no mutable access is exposed. |
| C++ raw byte write path | `machine.cpp` `Machine::write_rdram_u8` | No Rust equivalent write API; `machine/rdram_reservation.rs` owns construction/default, private staging, and private invalidation reservation state only | Reservation fields if valid; target byte | One storage byte | Invalidates overlapping reservation before storing | Partial: reservation fields, staging, and invalidation exist; write behavior absent | Mutable RDRAM write API unowned | Ready for RDRAM write decision after invalidation seal | This is not a pure storage write because it calls `invalidate_cpu_rdram_reservation_for_write(address, 1)`. |
| Rust raw byte write absence | C++ `write_rdram_u8` | No `Rdram::write_u8`, no `stage_byte`, no mutable Machine RDRAM API | Not applicable | Not applicable | Not applicable | Invalidation dependency now privately owned; storage mutation absent | Write API intentionally absent | Ready for RDRAM write decision after invalidation seal | Adding storage mutation still requires a distinct write seam. |
| C++ raw range write path | `stage_cartridge_bytes_to_rdram`; SP write DMA; SI PIF-to-DRAM DMA; CPU store loops | No Rust equivalent | Cartridge/SP/PIF/CPU source bytes depending on caller | Multiple RDRAM bytes through raw write helpers | Each byte write can invalidate an overlapping reservation | No | DMA, staging, load/store, reservation | Blocked by DMA seam | No standalone storage-only range writer was found; range writes are owned by staging, DMA, or execution paths. |
| Rust range write absence | C++ range writers loop through reservation-aware byte writes | No Rust range write API | Not applicable | Not applicable | Not applicable | No | Reservation invalidation and caller-specific owners absent | Not yet earned | Range write remains absent with byte write. |
| C++ invalid write offset behavior | `write_rdram_u8/u16/u32/u64`; `fail_rdram_access` | No Rust write error behavior | Bounds and width | None when invalid | Throws before reservation invalidation and before storage mutation | No for write behavior | No write API; exception-vs-Result boundary unearned | Not yet earned | Proof demos show failed public staging throws before invalidating an existing reservation. |
| Rust invalid write behavior absence | C++ throws for invalid writes | No Rust write error type beyond read-only `RdramAccessError` use | Not applicable | Not applicable | Not applicable | No | Write API intentionally absent | Not yet earned | Existing `RdramAccessError` is only used by read-only byte access. |
| C++ reservation state fields | `machine.hpp` `CpuRdramReservation { valid, rdram_offset, width }`; `Machine::cpu_rdram_reservation_` | `machine/rdram_reservation.rs` `CpuRdramReservation { valid, rdram_offset, width }`; `machine.rs` private `cpu_rdram_reservation` | Valid flag, offset, width | C++ writes fields through reset/clear/set/invalidation; Rust writes construction/default, private staged, and private invalidated values | Drives C++ LL/SC match and write invalidation | Yes for construction/default, staging, and invalidation fields | Reset API, LL/SC, writes remain absent | Documentation only | C++ documents this as local single-Machine LL/SC reservation state for Machine-owned RDRAM only. |
| Rust reservation state ownership | C++ `Machine::cpu_rdram_reservation_` | `machine.rs` private `cpu_rdram_reservation`; `machine/rdram_reservation.rs` private module | Construction/default and private staging/invalidation inspection in tests | Construction initializes invalid/zero/zero; staging assigns valid/offset/width; invalidation may clear all fields | No side effects beyond reservation fields | Yes for construction/default, private staging, and private invalidation | LL/SC, writes, reset, DMA write behavior absent | Documentation only | No public reservation API exists. |
| Reservation construction/default state | `Machine::reset_to_non_boot_power_on_state`; `clear_cpu_rdram_reservation()` assigns `{}` | `CpuRdramReservation::new`; `Default` | Existing reservation field | Initializes reservation to invalid/offset 0/width 0 | Construction path in C++ clears reservation because construction calls reset | Yes for construction-cleared state | Reset mutation point remains absent | Documentation only | Rust mirrors the resulting construction state without exposing reset. |
| Reservation reset behavior | `reset_to_non_boot_power_on_state` calls `clear_cpu_rdram_reservation()` | No Rust reset API; construction state matches cleared value | Existing reservation field | Clears reservation to `{}` | Also clears RDRAM, SP/PIF/device shadows, CPU fields | Construction value yes; reset behavior no | Full reset remains unearned | Blocked by reset/state-ownership seam | Seam 019 does not add reset or clear methods. |
| Reservation invalidation helper | `Machine::invalidate_cpu_rdram_reservation_for_write` | `CpuRdramReservation::invalidate_for_rdram_write` | Reservation valid/offset/width and write offset/width | Clears reservation on overlap | No storage write itself; caller writes after invalidation | State inputs and private invalidation yes | RDRAM write caller absent | Ready for RDRAM write decision after invalidation seal | Helper returns when reservation invalid or write width is zero. |
| Write/reservation overlap rule | `invalidate_cpu_rdram_reservation_for_write` half-open interval check | `CpuRdramReservation::invalidate_for_rdram_write` | Write `[offset, offset + width)` and reservation `[offset, offset + width)` | Clears reservation if intervals overlap | Non-overlap preserves reservation | Yes for private invalidation | RDRAM write caller absent | Ready for RDRAM write decision after invalidation seal | Source rule is `write_begin < reservation_end && reservation_begin < write_end`. |
| LL/SC relationship | `execute_cpu_instruction` LL/LLD set reservation; SC/SCD match and clear it | No Rust LL/SC or reservation behavior | CPU GPRs, translated RDRAM target, reservation | GPR result, reservation, maybe RDRAM | Tied to instruction execution and load/store | No | Execution and load/store absent | Blocked by LL/SC instruction seam | LL/LLD set widths 4/8; SC/SCD clear reservation before optional write. |
| CPU load/store relationship | `write_cpu_memory_u8/u16/u32/u64`; store instruction cases | No Rust CPU load/store API | CPU address translation and source registers | RDRAM or device/SP targets | RDRAM writes invalidate reservation; SC/SCD also write GPR result | No | CPU load/store and execution absent | Blocked by CPU load/store seam | CPU paths use emulated addresses, not raw storage offsets. |
| DMA relationship | SP write DMA, SI PIF-to-DRAM DMA, PI cart-to-RDRAM DMA | No Rust DMA API | SP/PIF/cartridge sources and RDRAM destination | RDRAM bytes through write helpers | Overlapping reservations invalidated after caller preflight | No | DMA owners absent | Blocked by DMA seam | C++ comments explicitly route DMA RDRAM writes through existing helpers. |
| Memory-map relationship | `require_cpu_data_target`; `require_cpu_rdram_address`; CPU memory write helpers | No Rust memory-map API | CPU addresses and target resolver | Dispatches to RDRAM/device/SP writes | Target-dependent; RDRAM writes invalidate reservation | No | Address translation and target resolver absent | Blocked by memory-map seam | Raw RDRAM helpers take `RdramOffset`; CPU paths translate elsewhere. |
| Reset relationship | `reset_to_non_boot_power_on_state` clears RDRAM and reservation | No Rust reset API | Machine-owned state | RDRAM, SP/PIF/device shadows, CPU/COP0/reservation fields | Full-machine state creation point | No | Full reset state ownership absent | Blocked by reset/state-ownership seam | Reset coupling is evidence against adding write/reservation behavior prematurely. |
| Endianness at raw write seam | `write_rdram_u16_be/u32_be/u64_be`; byte write has no conversion | No Rust write API | Numeric value for multi-byte helpers | Big-endian byte sequence for multi-byte helpers | Also invalidates reservation by helper width | No | Multi-byte write and reservation unearned | Not yet earned | Byte write has no endian conversion; multi-byte helpers are big-endian storage writes plus reservation invalidation. |
| Storage-only write helper exists | Searched raw helpers, public staging, CPU, DMA, proof paths | No Rust storage-only write | Not applicable | Not applicable | Not applicable | No | C++ helper does not exist as storage-only seam | Documentation only | No C++ storage-only RDRAM write helper was found; even public staging reaches reservation-aware writes. |
| Write can be tested without nonzero test-only injection | C++ tests use public staging and execution/proof paths | Rust has no write or storage-injection API; private reservation staging/invalidation exists only for reservation state | Existing read-only defaults | Not applicable | Not applicable | No for write behavior | Write API absent | Documentation only | Rust deliberately removed test-only private byte injection in seam 017. |
| Why no Rust write behavior was added | C++ writes mutate storage and reservation state together | Rust private invalidation exists, but no RDRAM write API exists | Not applicable | Not applicable | Avoids premature storage mutation parity | Yes for audit, state ownership, staging, and private invalidation only | Writes unearned | Documentation only | Seam 024 earns private invalidation without adding storage mutation. |
| Recommended next seam | C++ invalidation helper has source-clear behavior and Rust mirrors it privately | Rust private invalidation exists; write APIs remain absent | Private invalidation facts only | No write behavior yet | Enables future write decision after sealing invalidation | Invalidation implemented; write behavior absent | RDRAM writes and LL/SC must stay out of scope | Ready for reservation invalidation parity seal | Recommended next pass: `rust_parallel_core_seam_025_cpu_rdram_reservation_invalidation_parity_seal`, because the new helper should be sealed before RDRAM writes. |

## Seam 018 Audit Changes

- Re-audited C++ `Machine::rdram_`, raw RDRAM write helpers, public
  `stage_rdram_u32_be`, cartridge-to-RDRAM staging, CPU store paths, SP/SI/PI DMA
  RDRAM write paths, reservation fields, reset reservation clearing, LL/SC paths,
  and proof demos for reservation invalidation.
- Found that C++ raw RDRAM writes are reservation-aware. `write_rdram_u8`,
  `write_rdram_u16_be`, `write_rdram_u32_be`, and `write_rdram_u64_be` validate
  bounds, call `invalidate_cpu_rdram_reservation_for_write`, then mutate storage.
- Found no storage-only C++ RDRAM write helper to mirror honestly in Rust.
- Kept Rust RDRAM writes, range writes, reservation invalidation, LL/SC, CPU
  load/store, memory-map, DMA, reset, bus, step, execution, and C++ integration
  intentionally absent.
- Added no Rust emulator behavior and no new tests in this pass.
- No C++ source files were changed.

## CpuRdramReservation State Ownership Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `CpuRdramReservation` type or field | `src/core/machine.hpp` `struct CpuRdramReservation`; private `Machine::cpu_rdram_reservation_` | `machine/rdram_reservation.rs` `CpuRdramReservation`; `machine.rs` private `cpu_rdram_reservation` | Equivalent construction state, different ownership shape | `reservation_construction_matches_cpp_cleared_state`; source inspection | Rust mirrors the exact fields as a private semantic owner, not a public API or type-layout claim. |
| Rust reservation owner type | C++ Machine-owned nested/adjacent state in `machine.hpp` | `machine/rdram_reservation.rs` `pub(crate) struct CpuRdramReservation` | Rust-only repo hygiene, no emulator truth | Source inspection; `cargo test` | The module name is specific to the earned machine truth and avoids a generic reservation framework. |
| C++ reservation ownership location | `src/core/machine.hpp` private `cpu_rdram_reservation_` inside `Machine` | `machine.rs` private `cpu_rdram_reservation` field | Equivalent construction state, different ownership shape | Machine reservation construction test | C++ owns it on `Machine`; Rust also keeps ownership on `Machine`, with a private owner module for layout clarity. |
| Rust reservation ownership location | C++ Machine-owned reservation field | `machine.rs` `Machine::from_cartridge` initializes `cpu_rdram_reservation: CpuRdramReservation::new()` | Equivalent for construction ownership | `machine_from_cartridge_owns_default_cpu_rdram_reservation_state` | Reservation is not placed under `Cpu` or `Rdram` because current C++ names it as Machine-local state for Machine-owned RDRAM. |
| Construction/default valid state | C++ field default `bool valid = false`; `clear_cpu_rdram_reservation()` assigns `{}` | `CpuRdramReservation::new` sets `valid: false`; private test accessors | Equivalent | `reservation_construction_matches_cpp_cleared_state`; Machine construction test | Rust exposes no public reserve/invalidate/clear method. |
| Construction/default offset field | C++ field default `RdramOffset rdram_offset = 0`; `RdramOffset` is `std::uint32_t` | `rdram_offset: u32` default `0` | Equivalent | Reservation construction tests; source inspection | This is a storage offset field, not an emulated CPU address API. |
| Construction/default size/range field | C++ field default `std::size_t width = 0` | `width: usize` default `0` | Equivalent construction state, platform-width type analogue | Reservation construction tests; source inspection | Width is construction-observable as zero. Rust does not add overlap or range behavior. |
| Other reservation construction fields | No other C++ reservation fields found | No other Rust reservation fields | Equivalent | Source inspection | Rust did not add generation, mask, address, owner, or range fields. |
| Whether construction calls reset | `Machine::Machine(Cartridge)` calls `reset_to_non_boot_power_on_state()`; reset calls `clear_cpu_rdram_reservation()` | `Machine::from_cartridge` directly constructs `CpuRdramReservation::new()` | Equivalent construction-cleared value only; reset not earned | Source inspection; Rust tests | Rust mirrors the resulting cleared construction state without adding a reset API. |
| Reservation reset behavior | `reset_to_non_boot_power_on_state()` calls `clear_cpu_rdram_reservation()` | No Rust reset or clear API | C++ exists, Rust intentionally absent | Reset audit; source inspection | Full reset touches many unearned owners; seam 019 only owns the construction-cleared state. |
| Reservation staging helper | `Machine::set_cpu_rdram_reservation(RdramOffset, std::size_t)` | `CpuRdramReservation::stage(u32, usize)` | Equivalent behavior, different ownership shape | `staging_matches_cpp_set_cpu_rdram_reservation_assignments`; source inspection | Seam 022 adds private staging only; no public Machine forwarding is exposed. |
| Reservation invalidation helper | `Machine::invalidate_cpu_rdram_reservation_for_write` | No Rust equivalent | C++ exists, Rust intentionally absent | Source inspection | Invalidation remains absent; staging is added only to make future invalidation tests honest. |
| Write/reservation overlap rule | `invalidate_cpu_rdram_reservation_for_write` compares half-open write and reservation intervals | No Rust equivalent | C++ exists, Rust intentionally absent | Source inspection | The source rule is documented for the next seam but not implemented. |
| RDRAM write relationship | `write_rdram_u8/u16_be/u32_be/u64_be` all invalidate before storing | No Rust RDRAM write API | C++ exists, Rust intentionally absent | Source inspection | Reservation state is now owned so future write work can avoid fake storage-only parity, but writes remain absent. |
| LL/SC relationship | `execute_cpu_instruction` LL/LLD set reservation; SC/SCD match/clear and may write RDRAM | No Rust LL/SC behavior | Not in scope | Source inspection | State ownership is separable; instruction behavior is not earned. |
| CPU load/store relationship | CPU memory helpers translate CPU addresses and dispatch to RDRAM/device targets | No Rust CPU load/store API | Not in scope | Source inspection | Reservation state ownership does not add load/store readiness. |
| DMA relationship | SP/SI/PI DMA RDRAM writes route through reservation-aware write helpers | No Rust DMA API | Not in scope | Source inspection | DMA invalidation remains future behavior. |
| Memory-map relationship | CPU target resolution is separate from raw RDRAM offset helpers | No Rust memory-map API | Not in scope | Source inspection | Reservation state uses RDRAM offsets only; no address translation was added. |
| Reservation state can be owned before invalidation | C++ state has explicit default fields independent of helper bodies | Private Rust construction owner only | Equivalent construction state, different ownership shape | Reservation construction tests | This is the earned seam: storage/default ownership without behavior. |
| Reservation invalidation scope | C++ helper exists and is used by writes | No Rust invalidation method | Not yet earned | Source inspection | No `invalidate`, `clear`, `overlaps`, `reserve`, or `commit` method was added. |
| RDRAM writes scope | C++ write helpers exist and are reservation-aware | No Rust writes | Not in scope | Source inspection; RDRAM read tests still pass | `Rdram::read_u8` remains read-only. |
| Nonzero RDRAM storage mutation scope | C++ writes and staging can create nonzero bytes | No Rust write/staging/injection API | Not yet earned | Rust API inspection | No test-only nonzero injection was reintroduced. |
| Machine-level reservation access | C++ reservation helpers are private Machine functions | No public Rust Machine reservation accessor; private field inspected only by child-module tests | C++ exists, Rust intentionally absent for public access | Rust API inspection | Rust does not expose reservation state as product API. |
| Cpu-level reservation access | C++ does not store reservation in a standalone CPU object | No Rust `Cpu` reservation field or accessor | C++ exists, Rust intentionally absent | Rust API inspection | Keeping reservation off `Cpu` avoids pretending CPU owns the state in current C++. |
| Rust-only API safety | C++ has no public reservation API boundary here | No Rust public reservation API or error type | Not applicable | Rust API inspection | No Rust-only safety behavior was needed. |
| Naming/layout changes | C++ monolith keeps reservation state in Machine declarations | Added `machine/rdram_reservation.rs`; updated README/PARITY layout lists | Rust-only repo hygiene, no emulator truth | Source inspection; rustfmt/clippy gates | The file names the exact earned owner; no `memory`, `bus`, `util`, or helper bucket was added. |
| Recommended next seam | C++ invalidation helper has source-clear overlap behavior and Rust now has source-backed private staging setup | Rust reservation construction/staging owner exists; invalidation absent | Not yet earned for invalidation | Source inspection | Recommended next pass after seam 023: `rust_parallel_core_seam_024_cpu_rdram_reservation_invalidation_implementation_decision`, because staged valid reservations can now be tested without fake hooks. |

## Seam 019 Audit Changes

- Audited C++ `CpuRdramReservation`, the Machine-owned reservation field,
  construction/reset clearing, raw RDRAM write invalidation, LL/SC uses, CPU
  load/store paths, DMA paths, memory-map coupling, and proof observations.
- Added private Rust `machine::rdram_reservation::CpuRdramReservation` with only
  the source-backed construction/default fields: invalid valid flag, offset `0`,
  and width `0`.
- Added a private `Machine` reservation field initialized at construction and
  construction/default-state tests that also prove existing Cartridge, RDRAM
  read, CPU, GPR, scalar, and COP0 facts remain unchanged.
- Did not add reservation invalidation, RDRAM writes, range writes, LL/SC,
  CPU load/store, memory-map, DMA, reset, step, execution, renderer, SDL, host
  shell, or C++ integration.
- No C++ source files were changed.

## CpuRdramReservation State Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `CpuRdramReservation` type or field | `src/core/machine.hpp` private `struct CpuRdramReservation`; private field `cpu_rdram_reservation_` | `machine/rdram_reservation.rs` private module `CpuRdramReservation`; `machine.rs` private field `cpu_rdram_reservation` | Equivalent construction state, different ownership shape | `reservation_construction_matches_cpp_cleared_state`; source inspection | C++ type is private inside the Machine class declaration; Rust type is crate-private inside the Machine module tree. |
| Rust `CpuRdramReservation` type | C++ private reservation struct listed above | `machine/rdram_reservation.rs` `pub(crate) struct CpuRdramReservation` with private fields and private `stage` method | Equivalent construction/staging state, different ownership shape | Rust reservation construction/staging tests; source inspection | The type owns construction/default and private staging state. It is not a public machine API. |
| C++ reservation ownership location | `src/core/machine.hpp` `Machine::cpu_rdram_reservation_` | `machine.rs` private `cpu_rdram_reservation` field | Equivalent construction state, different ownership shape | Machine reservation construction test | C++ and Rust both place ownership on Machine. |
| Rust reservation ownership location | C++ Machine-owned reservation field | `machine.rs` `Machine::from_cartridge` initializes `CpuRdramReservation::new()` | Equivalent construction state, different ownership shape | Machine reservation construction test | This is not a Cpu-owned or Rdram-owned sidecar field. |
| C++ standalone reservation type presence | C++ has a private nested/adjacent Machine-local struct, not a public standalone reservation type | Rust has a private owner module under `machine` | Equivalent construction state, different ownership shape | Source inspection | Rust does not claim public type-layout parity. |
| Rust private reservation type | C++ private Machine-local struct | `machine/rdram_reservation.rs` is not exported by `lib.rs`; fields and accessors are private/test-only | Equivalent for privacy boundary | `lib.rs` inspection; Rust API inspection | No public reservation accessor or error type exists. |
| Rust reservation publicly exported | No C++ public reservation type or accessor | No `pub use` in `lib.rs`; no public Machine reservation accessor | C++ exists, Rust intentionally absent for public access | `lib.rs` inspection | Keeping it private avoids claiming a product-visible reservation API. |
| Construction/default valid value | `valid = false`; `clear_cpu_rdram_reservation()` assigns `{}` | `CpuRdramReservation::new` sets `valid: false` | Equivalent | `reservation_construction_matches_cpp_cleared_state`; `machine_from_cartridge_owns_default_cpu_rdram_reservation_state` | `false` is the exact C++ default, not a placeholder. |
| Construction/default `rdram_offset` value | `RdramOffset rdram_offset = 0`; `{}` reset/clear returns to zero | `rdram_offset: u32` set to `0` | Equivalent | Reservation tests; source inspection | Offset zero is construction state even though invalid reservations do not match. |
| Construction/default `width` value | `std::size_t width = 0`; `{}` reset/clear returns to zero | `width: usize` set to `0` | Equivalent construction state, platform-width type analogue | Reservation tests; source inspection | Width zero is the exact C++ default and is also the invalidation helper's zero-width early-return guard input. Rust adds no helper behavior. |
| Meaning/unit of `rdram_offset` | `RdramOffset` is `std::uint32_t`; raw RDRAM helpers and LL/SC use storage offsets after CPU address translation | Rust field `rdram_offset: u32` | Equivalent construction state, different ownership shape | Source inspection | This is a Machine-owned RDRAM storage offset, not a CPU address or memory-map API. |
| Meaning/unit of `width` | C++ helper arguments and LL/SC use byte widths: 1/2/4/8 for writes, 4 for LL/SC, 8 for LLD/SCD | Rust field `width: usize`; `CpuRdramReservation::stage` input | Equivalent construction/staging state only | Source inspection; staging tests | Rust proves default `0` and source-backed staged byte widths. Invalidation, matching, LL/SC, and write behavior remain absent. |
| Offset/width when `valid=false` | `cpu_rdram_reservation_matches` requires `valid`; invalidation returns immediately when `!valid` | Rust tests still assert offset `0` and width `0` at construction | Equivalent construction state; behavior absent | Source inspection; Rust tests | C++ ignores offset/width for matching/invalidation when invalid, but their construction values are still explicit source truth. |
| Whether construction calls reset | C++ `Machine::Machine` calls `reset_to_non_boot_power_on_state`; reset calls `clear_cpu_rdram_reservation()` | Rust `Machine::from_cartridge` directly constructs `CpuRdramReservation::new()` | Equivalent construction-cleared value only; reset not earned | Source inspection; Rust tests | Rust mirrors the resulting state, not the reset API. |
| Reservation reset behavior | `reset_to_non_boot_power_on_state` calls `clear_cpu_rdram_reservation()` and also clears many other owners | No Rust reset or clear method | C++ exists, Rust intentionally absent | Reset audit; source inspection | Reset remains unearned because full reset touches unearned SP/PIF/device, RDRAM mutation, CPU, COP0, and reservation state. |
| Reservation invalidation helper | `invalidate_cpu_rdram_reservation_for_write` reads valid/offset/width and clears on overlap | No Rust invalidation helper | C++ exists, Rust intentionally absent | Source inspection | No `invalidate`, `clear`, `reserve`, `commit`, or overlap helper exists in Rust. |
| Write/reservation overlap rule | C++ half-open intervals: `write_begin < reservation_end && reservation_begin < write_end` | No Rust overlap behavior | C++ exists, Rust intentionally absent | Source inspection | The rule is documented as a future behavior seam only. |
| RDRAM write relationship | C++ raw write helpers bounds-check, invalidate overlapping reservations, then mutate storage | No Rust RDRAM write API | Not in scope | Source inspection; Rust API inspection | Rust read-only `Rdram::read_u8` remains unchanged. |
| LL/SC relationship | LL/LLD set reservation; SC/SCD match/clear reservation and may write RDRAM/result GPR | No Rust LL/SC behavior | Not in scope | Source inspection; proof path inspection | State ownership does not imply instruction readiness. |
| CPU load/store relationship | CPU memory helpers translate CPU addresses and dispatch writes to RDRAM/device/SP owners | No Rust CPU load/store API | Not in scope | Source inspection | Reservation state ownership is not a load/store seam. |
| DMA relationship | SP/SI/PI DMA RDRAM writes use reservation-aware helpers or preserve reservations depending on direction/preflight | No Rust DMA API | Not in scope | Source inspection; proof path inspection | DMA remains unearned. |
| Memory-map relationship | C++ CPU paths translate and target-resolve before producing RDRAM offsets | No Rust memory-map API | Not in scope | Source inspection | `rdram_offset` remains raw storage truth only. |
| Reservation state can be owned before invalidation | C++ field defaults are explicit and source-separable from helper behavior | Private Rust state owner only | Equivalent construction state, different ownership shape | Reservation tests; source inspection | This is the sealed seam. Behavior remains absent. |
| Reservation invalidation in scope | C++ helper exists | No Rust helper | Not in scope | Source inspection | This pass intentionally does not add invalidation readiness as implemented behavior. |
| RDRAM writes in scope | C++ writes exist and are reservation-aware | No Rust writes | Not in scope | Source inspection | No storage mutation or nonzero fixture injection was added. |
| Nonzero RDRAM storage mutation in scope | C++ write/stage/DMA/load-store paths can mutate RDRAM bytes | No Rust write/stage/injection API | Not in scope | Rust API inspection | Rust tests still avoid nonzero RDRAM content requiring unearned mutation. |
| Machine-level reservation access | C++ helpers are private Machine methods | No public Rust Machine reservation accessor; tests inspect private field inside module | C++ exists, Rust intentionally absent for public access | Rust API inspection | Machine ownership is private state only. |
| Cpu-level reservation access | C++ has no standalone CPU owner for reservation | No Rust `Cpu` reservation field or accessors | C++ exists, Rust intentionally absent | Rust API inspection | Keeping reservation off `Cpu` preserves C++ Machine-local truth. |
| Rust-only API safety | C++ reservation state has no public failure boundary | No Rust reservation error or public API | Not applicable | Rust API inspection | No Rust-only safety behavior was added. |
| Naming/layout changes for this seam | C++ monolith keeps reservation in `Machine` | Kept `machine/rdram_reservation.rs`; updated this parity seal table | Rust-only repo hygiene, no emulator truth | Source inspection | No file/module split or rename was needed in seam 020. |
| Recommended next seam | C++ invalidation helper is source-clear and Rust now has source-backed private staged-reservation setup | Rust construction/default and private staging are sealed; invalidation absent | Not yet earned for invalidation | Source inspection | Recommended next pass after seam 023: `rust_parallel_core_seam_024_cpu_rdram_reservation_invalidation_implementation_decision`, because invalidation can now be tested without fake hooks while still keeping RDRAM writes and LL/SC out of scope. |

## Seam 020 Audit Changes

- Re-audited current C++ `CpuRdramReservation` declaration, Machine ownership,
  construction/reset clearing, invalidation helper, RDRAM write relationship,
  LL/SC relationship, DMA relationship, memory-map relationship, and proof
  observations.
- Re-audited Rust `machine/rdram_reservation.rs`, private Machine ownership,
  `lib.rs` exports, README scope, and parity claims.
- Confirmed Rust reservation construction/default state exactly mirrors C++:
  `valid=false`, `rdram_offset=0`, and `width=0`.
- Confirmed `rdram_offset` is a Machine-owned raw RDRAM storage offset, and
  `width` is a byte count. With `valid=false`, C++ matching/invalidation ignore
  offset/width behaviorally, but their construction values remain explicit
  source truth and are tested.
- Left Rust code unchanged. No reservation invalidation, overlap, reserve,
  clear, commit, RDRAM write, LL/SC, load/store, memory-map, DMA, reset, step,
  execution, renderer, SDL, host shell, or C++ integration behavior was added.
- No C++ source files were changed.

## CpuRdramReservation Invalidation Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ invalidation helper | `src/core/machine.cpp` `Machine::invalidate_cpu_rdram_reservation_for_write(RdramOffset address, std::size_t width)` | No Rust invalidation helper | C++ exists, Rust intentionally absent | Source inspection | The helper is source-clear; seam 021 left it absent because valid reservation setup was not yet earned, and seam 022 earns setup without adding invalidation. |
| Rust invalidation helper | C++ helper listed above | No `invalidate`, `overlaps`, `reserve`, `clear`, or `commit` method in `machine/rdram_reservation.rs` | Documentation only | Rust API inspection; `cargo test` | Leaving behavior absent avoids adding untestable state mutation or test-only hooks. |
| Invalidation input offset/address | C++ input type is `RdramOffset`, a raw RDRAM storage offset; write callers pass `target.rdram_offset` or raw RDRAM helper offsets | No Rust invalidation input | C++ exists, Rust intentionally absent | Source inspection | This is not a CPU address, memory-map address, bus address, or cartridge address seam. |
| Invalidation input width | C++ input type is `std::size_t width` | No Rust invalidation input | C++ exists, Rust intentionally absent | Source inspection | Write helpers call with widths 1, 2, 4, and 8; the helper also explicitly handles `width == 0`. |
| Width unit | C++ helper treats width as a byte count in `write_end = write_begin + width` | Rust reservation field `width: usize`; `CpuRdramReservation::stage` stores it directly | Equivalent construction/staging state only | Source inspection; construction/staging tests | Rust has earned field setup, but not invalidation math. |
| Reservation `valid=false` behavior | `invalidate_cpu_rdram_reservation_for_write` returns immediately when `!valid` | Rust construction/default state is `valid=false`; no invalidation behavior | C++ exists, Rust intentionally absent for behavior | Source inspection; reservation construction tests | With invalid state, C++ ignores offset/width behaviorally. |
| Overlapping valid reservation behavior | C++ computes half-open intervals and calls `clear_cpu_rdram_reservation()` when they overlap | No Rust invalidation equivalent; Rust can now stage valid reservation state | C++ exists, Rust intentionally absent for invalidation | Source inspection; C++ proof observations | C++ overlap behavior is source-clear; Rust invalidation remains absent until the next seam. |
| Non-overlapping valid reservation behavior | C++ leaves the reservation unchanged when half-open intervals do not overlap | No Rust invalidation equivalent; Rust can now stage valid reservation state | C++ exists, Rust intentionally absent for invalidation | Source inspection; C++ proof observations | Non-overlap preservation is not implemented or tested in Rust. |
| Boundary overlap rule | C++ condition is `write_begin < reservation_end && reservation_begin < write_end` | No Rust equivalent | C++ exists, Rust intentionally absent | Source inspection | Adjacent half-open intervals do not overlap; shared bytes overlap. |
| Invalidation changes `valid` | On overlap, C++ calls `clear_cpu_rdram_reservation()`, assigning `{}` and setting `valid=false` | Rust construction state already has `valid=false`; no mutation method | C++ exists, Rust intentionally absent | Source inspection | Rust proves only the cleared construction value. |
| Invalidation changes `rdram_offset` | On overlap, C++ clear resets `rdram_offset` to `0` | Rust construction state has `rdram_offset=0`; no mutation method | C++ exists, Rust intentionally absent | Source inspection | C++ overlapping invalidation clears all fields, not only `valid`. |
| Invalidation changes `width` | On overlap, C++ clear resets `width` to `0` | Rust construction state has `width=0`; no mutation method | C++ exists, Rust intentionally absent | Source inspection | C++ overlapping invalidation clears all fields through aggregate assignment. |
| Invalidation reads RDRAM | C++ invalidation helper reads only reservation fields and helper inputs | No Rust helper | Documentation only | Source inspection | The helper itself does not inspect `rdram_`. |
| Invalidation writes RDRAM | C++ invalidation helper does not write `rdram_`; write callers mutate storage after invalidation | No Rust helper or write API | Documentation only | Source inspection | Invalidation is not RDRAM writing, and seam 021 adds no write API. |
| Invalidation changes CPU fields | C++ helper mutates only `cpu_rdram_reservation_`; SC/SCD instruction paths separately write GPR result state | No Rust helper | Documentation only | Source inspection | No GPR, PC, next PC, HI, LO, or COP0 mutation is part of the helper. |
| Invalidation changes COP0 fields | C++ helper does not reference COP0 fields | No Rust helper | Documentation only | Source inspection | COP0 construction/access state remains unrelated to this helper. |
| Invalidation called by RDRAM writes | `write_rdram_u8/u16_be/u32_be/u64_be` call invalidation after bounds checks and before storage writes | No Rust RDRAM write API | C++ exists, Rust intentionally absent | Source inspection | This coupling is why Rust still cannot add storage-only writes. |
| Invalidation used by LL/SC | LL/LLD set reservations; SC/SCD match and clear reservations; RDRAM writes can invalidate reservations between them | No Rust LL/SC behavior | Not in scope | Source inspection; proof observations | The helper supports LL/SC-visible behavior but does not itself execute LL/SC. |
| Tied to CPU load/store | CPU stores can reach RDRAM write helpers after address translation; LL/SC paths use reservation helpers | No Rust CPU load/store API | Not in scope | Source inspection | CPU load/store behavior remains unearned. |
| Tied to DMA | SP/SI/PI paths that write RDRAM route through write helpers, so they can trigger invalidation after preflight | No Rust DMA API | Not in scope | Source inspection; proof observations | DMA remains a future owner and is not added here. |
| Tied to memory map | CPU paths translate emulated addresses to `RdramOffset` before write helpers are called | No Rust memory-map API | Not in scope | Source inspection | Raw invalidation inputs are storage offsets; translation remains out of scope. |
| Tied to reset | Reset clears reservation through `clear_cpu_rdram_reservation()`, not through invalidation | No Rust reset API | Not in scope | Source inspection | Reset remains intentionally absent. |
| Valid reservation can be staged without LL/SC | C++ has private `set_cpu_rdram_reservation`, while current C++ production setup is reached by LL/LLD execution paths | `CpuRdramReservation::stage` mirrors the private helper | Equivalent behavior, different ownership shape | Seam 022 staging tests; source inspection | Rust staging is private setup truth only; no LL/SC behavior is added. |
| Invalidation can be tested without test-only hooks | C++ proofs create valid reservations through LL/LLD instruction paths before public staging or write observations | Rust can now stage a valid reservation through source-backed private setup | Ready for reservation invalidation decision | Rust API inspection; source inspection | Test-only constructors or hidden mutation hooks remain disallowed. |
| RDRAM writes in scope | C++ writes call invalidation and then mutate storage | No Rust RDRAM write API | Not in scope | Source inspection | Seam 021 does not add byte, range, endian, DMA, or load/store writes. |
| LL/SC in scope | C++ LL/SC paths set/match/clear reservations and write GPR/RDRAM state | No Rust LL/SC API | Not in scope | Source inspection | No instruction behavior was added. |
| Reservation staging/set behavior in scope | C++ private `set_cpu_rdram_reservation` exists | `CpuRdramReservation::stage` | Equivalent behavior, different ownership shape | Seam 022 staging tests | Staging is now earned as private setup state. |
| Rust-only API safety | C++ invalidation helper is private and noexcept; no public invalidation failure boundary exists | No Rust public invalidation API or error type | Not applicable | Rust API inspection | No Rust-only safety behavior was added. |
| Naming/layout changes for this seam | C++ monolith keeps invalidation on private Machine helper | Rust kept `machine/rdram_reservation.rs`; updated this ledger only | Rust-only repo hygiene, no emulator truth | Source inspection | No module split, rename, public export, or source behavior change was made. |
| Recommended next seam | C++ invalidation helper has source-clear behavior and Rust now has source-backed private staged-reservation setup | Rust construction/default and private staging are earned; invalidation absent | Not yet earned for invalidation | Source inspection; staging tests | Recommend `rust_parallel_core_seam_024_cpu_rdram_reservation_invalidation_implementation_decision`, because invalidation can now be tested without fake hooks. |

## Seam 021 Audit Changes

- Re-audited C++ `CpuRdramReservation` fields, private Machine ownership,
  `clear_cpu_rdram_reservation`, `set_cpu_rdram_reservation`,
  `cpu_rdram_reservation_matches`, and
  `invalidate_cpu_rdram_reservation_for_write`.
- Re-audited C++ RDRAM write callers, LL/LLD reservation setup, SC/SCD
  reservation match/clear paths, SP/SI/PI DMA write paths, reset clearing, and
  proof observations for reservation invalidation.
- Confirmed the C++ invalidation rule is source-clear: invalid or zero-width
  writes are no-ops; otherwise C++ compares half-open write and reservation
  byte ranges and clears the entire reservation when they overlap.
- Confirmed overlapping invalidation clears `valid`, `rdram_offset`, and
  `width` through `clear_cpu_rdram_reservation()`. Non-overlap preserves all
  reservation fields.
- Confirmed invalidation inputs are raw RDRAM storage offsets and byte widths,
  not CPU addresses or memory-map API values.
- Left Rust behavior unchanged in seam 021. At that point Rust could not honestly
  test invalidation without first earning a source-backed way to stage a valid
  reservation, and test-only reservation setup hooks were disallowed.
- No C++ source files were changed.

## CpuRdramReservation Staging Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ reservation staging helper | `src/core/machine.hpp` private `set_cpu_rdram_reservation(RdramOffset, std::size_t)`; `src/core/machine.cpp` definition | `machine/rdram_reservation.rs` `CpuRdramReservation::stage(u32, usize)` | Equivalent behavior, different ownership shape | `staging_matches_cpp_set_cpu_rdram_reservation_assignments`; source inspection | C++ names the helper `set`; Rust uses `stage` to match the sidecar staging vocabulary and avoid broad setter language in the public API. |
| Rust reservation staging helper | C++ helper listed above | `pub(super) fn stage(&mut self, rdram_offset: u32, width: usize)` | Equivalent behavior, different ownership shape | Rust reservation staging test | The method is private to the `machine` module tree and is not exported from `lib.rs`. |
| Staging owner | C++ helper is a private `Machine` method mutating `Machine::cpu_rdram_reservation_` | Rust keeps the method on private `CpuRdramReservation`, owned by private `Machine::cpu_rdram_reservation` | Equivalent behavior, different ownership shape | Source inspection; Machine preservation test | Rust does not add public Machine-level reservation forwarding. |
| Staging input offset/address | C++ input is `RdramOffset address` (`std::uint32_t`) | Rust input is `rdram_offset: u32` | Equivalent | Staging test; source inspection | This is raw RDRAM storage offset truth, not CPU address or memory-map truth. |
| Staging input width | C++ input is `std::size_t width` | Rust input is `width: usize` | Equivalent platform-width analogue | Staging test; source inspection | Width is stored directly. |
| Width unit | C++ LL/LLD callers pass 4 or 8 byte widths; helper stores `width` as `std::size_t` | Rust stores width as `usize` | Equivalent | Staging test; source inspection | This pass does not add LL/LLD behavior; it only mirrors the helper's byte-count field. |
| Staging sets `valid` | C++ assigns `cpu_rdram_reservation_.valid = true` | Rust assigns `self.valid = true` | Equivalent | `staging_matches_cpp_set_cpu_rdram_reservation_assignments` | Staging always makes the reservation valid. |
| Staging sets `rdram_offset` | C++ assigns `cpu_rdram_reservation_.rdram_offset = address` | Rust assigns `self.rdram_offset = rdram_offset` | Equivalent | Staging test | Value is stored exactly. |
| Staging sets `width` | C++ assigns `cpu_rdram_reservation_.width = width` | Rust assigns `self.width = width` | Equivalent | Staging test | Value is stored exactly. |
| Staging validates offset | C++ helper performs no bounds check or offset validation | Rust helper performs no bounds check or offset validation | Equivalent | Source inspection; staging test | Validation belongs to callers such as LL/LLD target resolution, not this helper. |
| Staging validates width | C++ helper performs no width validation | Rust helper performs no width validation | Equivalent | Source inspection; staging test | The helper stores whatever width it is given. |
| Staging permits `width=0` | C++ helper has no zero-width guard and would store zero while setting valid true | Rust test stages width zero and observes valid true, stored offset, stored width zero | Equivalent | Staging test; source inspection | This is helper truth only; C++ LL/LLD callers pass nonzero widths. |
| Staging changes non-reservation state | C++ helper assigns only the three reservation fields | Rust helper can access only the private reservation fields | Equivalent | `machine_private_reservation_staging_preserves_earned_machine_facts`; source inspection | Staging does not change Cartridge, RDRAM bytes/read facts, CPU GPRs, PC, next PC, HI, LO, or COP0 construction/access fields. |
| Source-separable from LL/SC | C++ staging is a separate private helper, though current C++ callers are LL/LLD execution paths | Rust implements only the helper's field assignments | Equivalent helper semantics; LL/SC not earned | Source inspection; Rust tests | This is reservation setup state, not LL/SC instruction behavior. |
| Source-separable from RDRAM writes | C++ staging helper does not read/write `rdram_`; RDRAM writes use invalidation helper, not staging | Rust helper does not access `Rdram` | Equivalent | Source inspection; Machine preservation test | No RDRAM write API is added. |
| Source-separable from CPU load/store | C++ helper has already translated storage-offset inputs when LL/LLD calls it | Rust helper takes raw offset directly | Equivalent helper semantics; load/store not earned | Source inspection | No CPU address translation, load, or store API is added. |
| Source-separable from memory map | C++ helper takes `RdramOffset`, not `CpuAddress`; target resolution happens before helper calls | Rust helper takes `u32` raw offset | Equivalent helper semantics; memory map not earned | Source inspection | No memory-map behavior is added. |
| Source-separable from DMA | C++ DMA paths do not call the staging helper; DMA write paths use RDRAM write helpers and invalidation | Rust helper is reservation-only | Not in scope | Source inspection | No DMA behavior is added. |
| Source-separable from reset | C++ reset clears reservation through `clear_cpu_rdram_reservation`, not staging | No Rust reset API | Not in scope | Source inspection | Staging does not imply reset readiness. |
| Source-separable from execution | Helper is factored out, but C++ production callers are execution paths; Rust adds only the factored field assignments | Equivalent helper semantics; execution not earned | Source inspection; Rust tests | No fetch, decode, execute, step, instruction, or writeback behavior is added. |
| Invalidation in scope | C++ invalidation helper exists separately | No Rust invalidation helper | Not in scope | Source inspection | Staging exists so a future invalidation seam can be tested honestly. |
| RDRAM writes in scope | C++ write helpers exist and are reservation-aware | No Rust write API | Not in scope | Source inspection; Rust API inspection | No storage mutation or nonzero RDRAM fixture injection is added. |
| LL/SC in scope | C++ LL/LLD call staging; SC/SCD match/clear and may write RDRAM/GPRs | No Rust LL/SC API | Not in scope | Source inspection | Staging does not claim LL/SC readiness. |
| Staging can support future invalidation seam | C++ invalidation requires a valid reservation to exercise overlap behavior | Rust can now stage a valid reservation without a test-only hook | Ready for reservation invalidation decision | Rust staging tests | Future invalidation can be tested through source-backed staging while still avoiding LL/SC and RDRAM writes. |
| Rust-only API safety | C++ helper is private and `noexcept`; no failure boundary exists | Rust helper is private and infallible | Not applicable | Rust API inspection | No Rust error type or public safety boundary was added. |
| Naming/layout changes for this seam | C++ keeps reservation helpers on private Machine | Rust kept `machine/rdram_reservation.rs`; no new module | Rust-only repo hygiene, no emulator truth | Source inspection | Method name `stage` is owner-local and avoids `helper`, `util`, `memory_state`, or broad bucket naming. |
| Recommended next seam | C++ invalidation helper has source-clear behavior and now has an honest Rust valid-reservation setup dependency | Rust private staging is earned; invalidation remains absent | Ready for reservation invalidation decision | Source inspection; Rust staging tests | Recommend `rust_parallel_core_seam_024_cpu_rdram_reservation_invalidation_implementation_decision`, because staged valid reservations can now exercise invalidation without fake hooks. |

## Seam 022 Audit Changes

- Re-audited C++ `CpuRdramReservation`, Machine ownership,
  `set_cpu_rdram_reservation`, LL/LLD callers, SC/SCD relationship, RDRAM
  write separation, DMA relationship, reset relationship, and proof
  observations.
- Added private Rust `CpuRdramReservation::stage` in
  `machine/rdram_reservation.rs` to mirror C++ `set_cpu_rdram_reservation`.
- Added tests proving staging sets `valid=true`, stores raw RDRAM offset exactly,
  stores byte width exactly, allows width zero because C++ has no guard, and
  preserves already-earned Machine facts.
- Did not add reservation invalidation, overlap, RDRAM writes, LL/SC,
  load/store, memory-map, DMA, reset, step, execution, renderer, SDL, host
  shell, or C++ integration behavior.
- No C++ source files were changed.

## CpuRdramReservation Staging Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ staging helper | `src/core/machine.hpp` private `set_cpu_rdram_reservation(RdramOffset, std::size_t)`; `src/core/machine.cpp` `Machine::set_cpu_rdram_reservation` | `machine/rdram_reservation.rs` `CpuRdramReservation::stage(u32, usize)` | Equivalent behavior, different ownership shape | `staging_matches_cpp_set_cpu_rdram_reservation_assignments`; source inspection | This seal covers only the private setup helper, not LL/SC instruction behavior. |
| Rust staging helper | C++ helper listed above | `machine/rdram_reservation.rs` `pub(super) fn stage(&mut self, rdram_offset: u32, width: usize)` | Equivalent behavior, different ownership shape | Rust staging tests; API inspection | The method remains private to the `machine` module tree and is not re-exported from `lib.rs`. |
| C++ staging owner | C++ private `Machine` helper mutates `Machine::cpu_rdram_reservation_` | Rust private Machine field owns `CpuRdramReservation` | Equivalent behavior, different ownership shape | Source inspection | C++ keeps the helper on the monolithic `Machine`; Rust keeps behavior on the semantic reservation owner. |
| Rust staging owner | C++ Machine-owned reservation field | `machine.rs` private `cpu_rdram_reservation`; `machine/rdram_reservation.rs` private owner module | Equivalent behavior, different ownership shape | Rust API inspection; Machine preservation test | No public Machine-level forwarding was added because the sidecar owner can seal the same private state truth. |
| Staging input offset/address | C++ input type is `RdramOffset address` (`std::uint32_t`) | Rust input is `rdram_offset: u32` | Equivalent | Source inspection; staging tests | This is raw RDRAM storage offset truth, not CPU address, bus address, memory-map address, or cartridge address truth. |
| Staging input width | C++ input type is `std::size_t width` | Rust input is `width: usize` | Equivalent platform-width analogue | Source inspection; staging tests | The value is stored directly. |
| Width unit | C++ callers pass byte widths and the reservation field is named `width` for byte spans | Rust `width: usize` mirrors the stored byte count | Equivalent | Source inspection; staging tests | This does not add overlap, range, load/store, or write behavior. |
| Staging sets `valid` | C++ assigns `cpu_rdram_reservation_.valid = true` | Rust assigns `self.valid = true` | Equivalent | `staging_matches_cpp_set_cpu_rdram_reservation_assignments`; `repeated_staging_overwrites_previous_reservation_fields` | Staging always leaves the reservation valid. |
| Staging sets `rdram_offset` | C++ assigns `cpu_rdram_reservation_.rdram_offset = address` | Rust assigns `self.rdram_offset = rdram_offset` | Equivalent | Staging tests | Value is stored exactly. |
| Staging sets `width` | C++ assigns `cpu_rdram_reservation_.width = width` | Rust assigns `self.width = width` | Equivalent | Staging tests | Value is stored exactly. |
| Staging validates offset | C++ helper has no offset bounds check or validation branch | Rust helper has no offset bounds check or validation branch | Equivalent | Source inspection; Rust API inspection | Offset validation belongs to callers such as LL/LLD target resolution, which remain unearned. |
| Staging validates width | C++ helper has no width validation branch | Rust helper has no width validation branch | Equivalent | Source inspection; Rust API inspection | Width is accepted as provided. |
| Staging permits `width = 0` | C++ helper has no zero-width guard and stores zero while setting `valid=true` | Rust test stages width zero and observes `valid=true`, stored offset, and stored width zero | Equivalent | `staging_matches_cpp_set_cpu_rdram_reservation_assignments`; source inspection | This is helper parity only. C++ LL/LLD production callers pass nonzero widths. |
| Repeated staging overwrites prior reservation | C++ helper assigns all three fields on every call | Rust repeated-staging test calls `stage` twice and observes only the second offset/width | Equivalent | `repeated_staging_overwrites_previous_reservation_fields`; source inspection | There is no merge, preserve, or overlap behavior in the staging helper. |
| Staging changes non-reservation state | C++ helper assigns only `cpu_rdram_reservation_.valid`, `.rdram_offset`, and `.width` | Rust helper can assign only private reservation fields | Equivalent | `machine_private_reservation_staging_preserves_earned_machine_facts`; source inspection | Cartridge, RDRAM bytes/read facts, CPU GPRs, register zero, PC, next PC, HI, LO, and COP0 construction/access facts remain unchanged. |
| Source-separable from LL/SC | C++ helper is factored out, though current production callers are LL/LLD execution paths | Rust mirrors only helper field assignment | Equivalent helper semantics; LL/SC not earned | Source inspection; Rust staging tests | No LL, LLD, SC, or SCD instruction behavior is added. |
| Source-separable from RDRAM writes | C++ staging helper does not read or write `rdram_`; C++ writes use invalidation separately | Rust helper does not access `Rdram` | Equivalent | Source inspection; Machine preservation test | No RDRAM write API or nonzero storage mutation is added. |
| Source-separable from CPU load/store | C++ helper receives an already translated `RdramOffset` | Rust helper receives a raw storage offset | Equivalent helper semantics; load/store not earned | Source inspection | No CPU address translation, load, store, sign extension, or writeback behavior is added. |
| Source-separable from memory map | C++ helper takes `RdramOffset`, not `CpuAddress` | Rust helper takes raw offset `u32` | Equivalent helper semantics; memory map not earned | Source inspection | No bus or memory-map API is added. |
| Source-separable from DMA | C++ DMA paths do not call `set_cpu_rdram_reservation`; DMA writes route through RDRAM write helpers and invalidation | Rust helper is reservation-only | Not in scope | Source inspection | No DMA behavior is added. |
| Source-separable from reset | C++ reset calls `clear_cpu_rdram_reservation`, not staging | No Rust reset API | Not in scope | Source inspection | Staging parity does not imply reset readiness. |
| Source-separable from execution | C++ helper body is field assignment only; current callers live in execution paths | Rust helper adds only field assignment | Equivalent helper semantics; execution not earned | Source inspection; Rust tests | No fetch, decode, execute, step, instruction, branch, jump, link, or delay-slot behavior is added. |
| Invalidation in scope | C++ invalidation helper remains separate | No Rust invalidation helper | Not in scope | Source inspection; Rust API inspection | This pass seals setup only. |
| RDRAM writes in scope | C++ write helpers are reservation-aware and call invalidation | No Rust write API | Not in scope | Source inspection; Rust API inspection | Writes remain intentionally absent. |
| LL/SC in scope | C++ LL/LLD call staging; SC/SCD match/clear and may write GPR/RDRAM state | No Rust LL/SC API | Not in scope | Source inspection | Staging is not LL/SC readiness. |
| Future invalidation support | C++ invalidation needs a valid reservation to exercise overlap behavior | Rust can stage valid reservation state without test-only hooks | Ready for reservation invalidation decision | Rust staging tests | Future invalidation can now be tested honestly while keeping RDRAM writes and LL/SC out of scope. |
| Rust-only API safety | C++ helper is private and `noexcept`; no failure boundary exists | Rust helper is private and infallible | Not applicable | Rust API inspection | No Rust error type or public safety boundary was added. |
| Naming/layout changes for this seam | C++ monolith keeps helper on private Machine | Rust kept `machine/rdram_reservation.rs`; no new module | Rust-only repo hygiene, no emulator truth | Source inspection; layout audit | No rename or split was needed. The method name `stage` matches the sidecar staging vocabulary and avoids broad helper/util naming. |
| Recommended next seam | C++ invalidation helper has source-clear overlap behavior and Rust staging parity is sealed | Rust private staging is sealed; invalidation remains absent | Ready for reservation invalidation decision | Source inspection; Rust tests | Recommend `rust_parallel_core_seam_024_cpu_rdram_reservation_invalidation_implementation_decision`, because valid-reservation setup is now source-backed and testable without fake hooks. |

## Seam 023 Audit Changes

- Re-audited C++ `CpuRdramReservation`, Machine ownership,
  `set_cpu_rdram_reservation`, invalidation separation, RDRAM write separation,
  LL/LLD callers, reset relationship, DMA relationship, and proof/test
  observations.
- Re-audited Rust `machine/rdram_reservation.rs`, private Machine ownership,
  `lib.rs` exports, reservation staging tests, README, and this ledger.
- Added an explicit repeated-staging test proving a second call overwrites the
  prior staged offset and width while leaving `valid=true`, matching the C++
  helper's unconditional field assignments.
- Sealed staging as private setup truth only. No reservation invalidation,
  overlap, RDRAM writes, LL/SC, load/store, memory-map, DMA, reset, step,
  execution, renderer, SDL, host shell, or C++ integration behavior was added.
- No C++ source files were changed.

## CpuRdramReservation Invalidation Implementation Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ invalidation helper | `src/core/machine.cpp` `Machine::invalidate_cpu_rdram_reservation_for_write(RdramOffset, std::size_t)`; declaration in `machine.hpp` | `machine/rdram_reservation.rs` `CpuRdramReservation::invalidate_for_rdram_write(u32, usize)` | Equivalent behavior, different ownership shape | Invalidation tests; source inspection | Rust keeps the behavior on the private reservation owner instead of exposing a Machine forwarding method. |
| Rust invalidation helper | C++ helper listed above | `pub(super) fn invalidate_for_rdram_write(&mut self, rdram_offset: u32, width: usize)` | Equivalent behavior, different ownership shape | Rust invalidation tests; API inspection | The method is private to the `machine` module tree and is not exported from `lib.rs`. |
| C++ clear helper | `src/core/machine.cpp` `Machine::clear_cpu_rdram_reservation()` assigns `cpu_rdram_reservation_ = {}` | `machine/rdram_reservation.rs` private `clear()` assigns `*self = Self::new()` | Equivalent | `invalidation_clears_overlapping_valid_reservation`; source inspection | Clear resets all reservation fields, not only `valid`. |
| Rust clear behavior | C++ helper listed above | Private `CpuRdramReservation::clear` only | Equivalent behavior, different ownership shape | Overlap and contained-range tests | No public clear/reset/reserve/commit API is added. |
| Invalidation input offset/address | C++ input type is `RdramOffset`, a raw storage offset | Rust input is `rdram_offset: u32` | Equivalent | Source inspection; boundary tests | This is not a CPU address, bus address, cartridge address, or memory-map API. |
| Invalidation input width | C++ input is `std::size_t width` | Rust input is `width: usize` | Equivalent platform-width analogue | Source inspection; invalidation tests | Width is passed directly to overlap math. |
| Width unit | C++ computes byte ranges from offset plus width | Rust computes byte ranges from offset plus width | Equivalent | Boundary and contained-range tests | Width is a byte count. |
| `valid=false` behavior | C++ returns immediately when `!cpu_rdram_reservation_.valid` | Rust returns immediately when `!self.valid` | Equivalent for construction-cleared invalid state | `invalidation_noops_for_invalid_reservation_and_zero_width_write` | No source-backed Rust path creates invalid nonzero fields, so that artificial state is not tested. |
| `width=0` behavior | C++ returns immediately when write width is zero | Rust returns immediately when write width is zero | Equivalent | `invalidation_noops_for_invalid_reservation_and_zero_width_write` | Staged reservation fields are preserved. |
| Overlap rule | C++ uses `write_begin < reservation_end && reservation_begin < write_end` | Rust uses the same half-open interval condition | Equivalent | Boundary, contained-range, exact-range tests | Ranges are `[offset, offset + width)`. |
| Boundary non-overlap rule | C++ half-open intervals do not overlap when `write_end == reservation_begin` or `write_begin == reservation_end` | Rust preserves the staged reservation for both adjacent cases | Equivalent | `invalidation_uses_cpp_half_open_boundary_rules` | Adjacent ranges do not clear reservation. |
| Boundary overlap rule | C++ clears when the write shares at least one byte with the reservation | Rust clears for one-byte beginning and one-byte ending overlap | Equivalent | `invalidation_uses_cpp_half_open_boundary_rules` | Shared bytes clear the reservation. |
| Exact same range behavior | C++ overlap condition is true for identical nonzero ranges | Rust clears identical staged/write ranges | Equivalent | `invalidation_clears_contained_and_exact_overlapping_ranges` | Exact match is not special-cased. |
| Write-contained-in-reservation behavior | C++ overlap condition is true when write is inside reservation | Rust clears when write range is inside reservation range | Equivalent | `invalidation_clears_contained_and_exact_overlapping_ranges` | This proves byte-width interpretation without adding writes. |
| Reservation-contained-in-write behavior | C++ overlap condition is true when reservation is inside write | Rust clears when write range covers reservation range | Equivalent | `invalidation_clears_contained_and_exact_overlapping_ranges` | This is still only private invalidation, not a write API. |
| Non-overlap behavior | C++ leaves all reservation fields unchanged when ranges do not overlap | Rust preserves `valid`, `rdram_offset`, and `width` when ranges do not overlap | Equivalent | `invalidation_preserves_valid_non_overlapping_reservation`; boundary test | Non-overlap does not partially mutate fields. |
| Invalidation changes `valid` | C++ clear sets `valid=false` on overlap | Rust clear sets `valid=false` on overlap | Equivalent | Overlap tests | Overlap clears the valid flag. |
| Invalidation changes `rdram_offset` | C++ clear resets `rdram_offset=0` on overlap | Rust clear resets `rdram_offset=0` on overlap | Equivalent | Overlap tests | Offset is cleared, not preserved. |
| Invalidation changes `width` | C++ clear resets `width=0` on overlap | Rust clear resets `width=0` on overlap | Equivalent | Overlap tests | Width is cleared, not preserved. |
| Invalidation reads RDRAM | C++ invalidation helper does not read `rdram_` | Rust invalidation helper cannot access `Rdram` | Equivalent | Source/API inspection | RDRAM byte contents are not inspected. |
| Invalidation writes RDRAM | C++ invalidation helper does not write `rdram_`; write callers do that later | Rust helper has no RDRAM access and no write API exists | Equivalent for helper behavior | Machine preservation test; API inspection | RDRAM writes remain absent. |
| Invalidation changes CPU fields | C++ helper mutates only `cpu_rdram_reservation_` | Rust helper mutates only private reservation fields | Equivalent for represented state | `machine_private_reservation_invalidation_preserves_earned_machine_facts` | GPRs, zero register, PC, next PC, HI, and LO are unchanged. |
| Invalidation changes COP0 fields | C++ helper does not reference COP0 fields | Rust helper does not reference `Cpu` or `Cop0` | Equivalent for represented state | Machine preservation test | COP0 construction/access facts remain unchanged. |
| Invalidation is called by RDRAM writes in C++ | `write_rdram_u8/u16_be/u32_be/u64_be` call invalidation after bounds checks and before storage mutation | Rust RDRAM writes remain absent | C++ exists, Rust intentionally absent for write callers | Source inspection | This pass adds the private helper only, not its future write caller. |
| Rust RDRAM writes remain absent | C++ has reservation-aware write helpers | No `Rdram::write_*`, no mutable Machine RDRAM API, no range write API | Not in scope | Rust API inspection | No nonzero RDRAM storage mutation or test fixture injection was added. |
| LL/SC in scope | C++ LL/LLD set reservations and SC/SCD match/clear/write through execution paths | No Rust LL/SC API | Not in scope | Source inspection; proof observations | Private invalidation does not execute instructions. |
| CPU load/store in scope | C++ CPU stores can reach RDRAM write helpers after address translation | No Rust CPU load/store API | Not in scope | Source inspection | No sign extension, address translation, or writeback behavior is added. |
| Memory-map in scope | C++ target resolution produces `RdramOffset` before raw helpers | No Rust memory-map or bus API | Not in scope | Source inspection | Invalidation inputs are already raw storage offsets. |
| DMA in scope | C++ SP/SI/PI write DMA paths route to reservation-aware RDRAM writes | No Rust DMA API | Not in scope | Source inspection; proof observations | DMA write behavior remains absent. |
| Reset in scope | C++ reset clears reservation through clear helper and also resets many other owners | No Rust reset API | Not in scope | Source inspection | Construction-cleared value is owned; reset behavior is not. |
| Staging used as earned setup seam | C++ private `set_cpu_rdram_reservation` makes a valid reservation | Rust private `stage` is used by invalidation tests | Equivalent behavior, different ownership shape | Staging and invalidation tests | Tests use source-backed staging, not fake hooks. |
| Overflow/range arithmetic notes | C++ casts offset and width to `std::uint64_t` and adds without explicit validation | Rust uses `u64::from(offset)` and `wrapping_add(width as u64)` | Equivalent for unsigned arithmetic shape; tests use source-clear non-overflowing ranges | Source inspection; Rust implementation inspection | No checked validation or error path was invented. |
| Rust-only API safety | C++ invalidation helper is private and `noexcept` | Rust helper is private and infallible | Not applicable | API inspection | No public error type or safety boundary was added. |
| Naming/layout changes for this seam | C++ monolith keeps helper on private Machine | Rust kept `machine/rdram_reservation.rs`; no new module or public export | Rust-only repo hygiene, no emulator truth | Layout/API inspection | Names describe `CpuRdramReservation`; no helper/util/common bucket was added. |
| Recommended next seam | C++ invalidation behavior is now mirrored privately; RDRAM writes still call invalidation before storage mutation | Rust private invalidation is implemented but not yet sealed | Ready for reservation invalidation parity seal | Rust tests and source inspection | Recommend `rust_parallel_core_seam_025_cpu_rdram_reservation_invalidation_parity_seal` before any RDRAM write decision. |

## Seam 024 Audit Changes

- Re-audited current C++ `CpuRdramReservation` fields, `clear_cpu_rdram_reservation`,
  `set_cpu_rdram_reservation`, `invalidate_cpu_rdram_reservation_for_write`,
  write callers, LL/LLD setup, SC/SCD match/clear paths, reset clearing, and
  proof observations.
- Added private Rust `CpuRdramReservation::invalidate_for_rdram_write` plus a
  private local clear path in `machine/rdram_reservation.rs`.
- Mirrored the C++ rule exactly for the earned helper subset: invalid reservation
  and zero-width write are no-ops; half-open range overlap clears `valid`,
  `rdram_offset`, and `width`; non-overlap preserves all reservation fields.
- Added tests for no-op, non-overlap, overlap, adjacent-boundary, one-byte
  boundary, contained-range, exact-range, and Machine-owned state preservation.
- Did not add RDRAM writes, range writes, nonzero RDRAM mutation, LL/SC,
  CPU load/store, memory-map, DMA, reset, step, execution, renderer, SDL, host
  shell, or C++ integration behavior.
- No C++ source files were changed.

## CpuRdramReservation Invalidation Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ invalidation helper | `src/core/machine.hpp` declaration and `src/core/machine.cpp` `Machine::invalidate_cpu_rdram_reservation_for_write(RdramOffset, std::size_t)` | `machine/rdram_reservation.rs` `CpuRdramReservation::invalidate_for_rdram_write(u32, usize)` | Equivalent behavior, different ownership shape | Source inspection; Rust invalidation tests | C++ keeps the helper on private `Machine`; Rust keeps it on the private reservation owner. |
| Rust invalidation helper | C++ helper listed above | `pub(super) fn invalidate_for_rdram_write(&mut self, rdram_offset: u32, width: usize)` | Equivalent behavior, different ownership shape | Rust API inspection; Rust invalidation tests | The method is private to the `machine` module tree and is not exported from `lib.rs`. |
| C++ clear helper | `src/core/machine.cpp` `Machine::clear_cpu_rdram_reservation()` assigns `cpu_rdram_reservation_ = {}` | `machine/rdram_reservation.rs` private `clear()` assigns `*self = Self::new()` | Equivalent | `invalidation_clears_overlapping_valid_reservation`; source inspection | Both clear paths restore `valid=false`, `rdram_offset=0`, and `width=0`. |
| Rust clear behavior | C++ helper listed above | Private `CpuRdramReservation::clear` only | Equivalent behavior, different ownership shape | Overlap, contained-range, exact-range, and latest-staging tests | No public clear/reset/reserve/commit API is added. |
| Invalidation input offset/address | C++ input type is `RdramOffset = std::uint32_t` | Rust input is `rdram_offset: u32` | Equivalent | Source inspection; boundary tests | This is a raw RDRAM storage offset, not a CPU address, bus address, cartridge address, or memory-map API. |
| Invalidation input width | C++ input type is `std::size_t width` | Rust input is `width: usize` | Equivalent platform-width analogue | Source inspection; invalidation tests | Width is used directly in range arithmetic. |
| Width unit | C++ computes byte ranges from offset plus width | Rust computes byte ranges from offset plus width | Equivalent | Boundary and contained-range tests | Width is a byte count. |
| `valid=false` behavior | C++ returns immediately when `!cpu_rdram_reservation_.valid` | Rust returns immediately when `!self.valid` | Equivalent for source-backed invalid states | `invalidation_noops_for_invalid_reservation_and_zero_width_write`; `invalidation_uses_latest_staged_reservation_and_cleared_state_stays_invalid` | Cleared invalid state remains invalid/zero on later invalidation. |
| `width=0` behavior | C++ returns immediately when write width is zero | Rust returns immediately when write width is zero | Equivalent | `invalidation_noops_for_invalid_reservation_and_zero_width_write` | Staged reservation fields are preserved. |
| Half-open overlap rule | C++ uses `write_begin < reservation_end && reservation_begin < write_end` | Rust uses the same half-open interval condition | Equivalent | Boundary, contained-range, exact-range, and latest-staging tests | Ranges are `[offset, offset + width)`. |
| Boundary non-overlap: `write_end == reservation_begin` | C++ half-open intervals do not overlap | Rust preserves the staged reservation | Equivalent | `invalidation_uses_cpp_half_open_boundary_rules` | Adjacent lower write range does not clear reservation. |
| Boundary non-overlap: `write_begin == reservation_end` | C++ half-open intervals do not overlap | Rust preserves the staged reservation | Equivalent | `invalidation_uses_cpp_half_open_boundary_rules` | Adjacent upper write range does not clear reservation. |
| Boundary one-byte overlap at beginning | C++ clears when the write shares the first reservation byte | Rust clears for a one-byte lower-boundary overlap | Equivalent | `invalidation_uses_cpp_half_open_boundary_rules` | Shared bytes clear the reservation. |
| Boundary one-byte overlap at end | C++ clears when the write shares the last reservation byte | Rust clears for a one-byte upper-boundary overlap | Equivalent | `invalidation_uses_cpp_half_open_boundary_rules` | Shared bytes clear the reservation. |
| Exact same range behavior | C++ overlap condition is true for identical nonzero ranges | Rust clears identical staged/write ranges | Equivalent | `invalidation_clears_contained_and_exact_overlapping_ranges` | Exact match is not special-cased. |
| Write-contained-in-reservation behavior | C++ overlap condition is true when the write range is inside the reservation range | Rust clears when write range is inside reservation range | Equivalent | `invalidation_clears_contained_and_exact_overlapping_ranges` | This proves byte-width interpretation without adding writes. |
| Reservation-contained-in-write behavior | C++ overlap condition is true when the reservation range is inside the write range | Rust clears when write range covers reservation range | Equivalent | `invalidation_clears_contained_and_exact_overlapping_ranges` | This is private invalidation only, not a write API. |
| Non-overlap behavior | C++ leaves all reservation fields unchanged when ranges do not overlap | Rust preserves `valid`, `rdram_offset`, and `width` when ranges do not overlap | Equivalent | `invalidation_preserves_valid_non_overlapping_reservation`; boundary test; latest-staging test | Non-overlap does not partially mutate fields. |
| Invalidation changes `valid` | C++ clear sets `valid=false` on overlap | Rust clear sets `valid=false` on overlap | Equivalent | Overlap and contained-range tests | Overlap clears the valid flag. |
| Invalidation changes `rdram_offset` | C++ clear resets `rdram_offset=0` on overlap | Rust clear resets `rdram_offset=0` on overlap | Equivalent | Overlap and contained-range tests | Offset is cleared, not preserved. |
| Invalidation changes `width` | C++ clear resets `width=0` on overlap | Rust clear resets `width=0` on overlap | Equivalent | Overlap and contained-range tests | Width is cleared, not preserved. |
| Invalidation reads RDRAM | C++ invalidation helper does not read `rdram_` | Rust invalidation helper cannot access `Rdram` | Equivalent | Source/API inspection | RDRAM byte contents are not inspected. |
| Invalidation writes RDRAM | C++ invalidation helper does not write `rdram_`; write callers mutate storage after invalidation | Rust helper has no RDRAM access and no write API exists | Equivalent for helper behavior | Machine preservation test; API inspection | RDRAM writes remain absent. |
| Invalidation changes CPU fields | C++ helper mutates only `cpu_rdram_reservation_` | Rust helper mutates only private reservation fields | Equivalent for represented state | `machine_private_reservation_invalidation_preserves_earned_machine_facts` | GPRs, zero register, PC, next PC, HI, and LO are unchanged. |
| Invalidation changes COP0 fields | C++ helper does not reference COP0 fields | Rust helper does not reference `Cpu` or `Cop0` | Equivalent for represented state | Machine preservation test | COP0 construction/access facts remain unchanged. |
| Invalidation changes Cartridge facts | C++ helper does not reference `cartridge_` | Rust helper does not reference `Cartridge` | Equivalent for represented state | Machine preservation test | Cartridge bytes and metadata remain unchanged. |
| Invalidation is called by RDRAM writes in C++ | `write_rdram_u8/u16_be/u32_be/u64_be` call invalidation after bounds checks and before storage mutation | Rust RDRAM writes remain absent | C++ exists, Rust intentionally absent for write callers | Source inspection | The private helper is sealed; its future write caller is not added. |
| Rust RDRAM writes remain absent | C++ has reservation-aware write helpers | No `Rdram::write_*`, no mutable Machine RDRAM API, no range write API | Not in scope | Rust API inspection | No nonzero RDRAM storage mutation or test fixture injection exists. |
| LL/SC is in scope | C++ LL/LLD set reservations and SC/SCD match/clear/write through execution paths | No Rust LL/SC API | Not in scope | Source inspection; proof observations | Private invalidation does not execute instructions. |
| CPU load/store is in scope | C++ CPU stores can reach RDRAM write helpers after address translation | No Rust CPU load/store API | Not in scope | Source inspection | No sign extension, address translation, or writeback behavior is added. |
| Memory-map is in scope | C++ target resolution produces `RdramOffset` before raw helpers | No Rust memory-map or bus API | Not in scope | Source inspection | Invalidation inputs are already raw storage offsets. |
| DMA is in scope | C++ SP/SI/PI write DMA paths route to reservation-aware RDRAM writes | No Rust DMA API | Not in scope | Source inspection; proof observations | DMA write behavior remains absent. |
| Reset is in scope | C++ reset clears reservation through clear helper and resets many other owners | No Rust reset API | Not in scope | Source inspection | Construction-cleared value is owned; reset behavior is not. |
| Staging is used as earned setup seam | C++ private `set_cpu_rdram_reservation` makes a valid reservation | Rust private `stage` is used by invalidation tests | Equivalent behavior, different ownership shape | Staging and invalidation tests | Tests use source-backed staging, not fake hooks. |
| Overflow/range arithmetic notes | C++ casts offset and width to `std::uint64_t` and adds without explicit validation | Rust uses `u64::from(offset)` and `wrapping_add(width as u64)` | Equivalent for tested non-overflowing ranges; overflow behavior not separately claimed | Source inspection; Rust implementation inspection | No checked validation or error path was invented. Tests use source-clear non-overflowing ranges. |
| Rust-only API safety | C++ invalidation helper is private and `noexcept` | Rust helper is private and infallible | Not applicable | API inspection | No public error type or safety boundary was added. |
| Naming/layout changes made for this seam | C++ monolith keeps helper on private Machine | Rust kept `machine/rdram_reservation.rs`; no new module or public export | Rust-only repo hygiene, no emulator truth | Layout/API inspection | Names describe `CpuRdramReservation`; no helper/util/common bucket was added. |
| Recommended next seam | C++ reservation-aware RDRAM writes call the now-sealed invalidation rule before mutating storage | Rust private invalidation parity is sealed; write APIs remain absent | Ready for RDRAM write decision after seal | Rust tests and source inspection | Recommend `rust_parallel_core_seam_026_rdram_write_access_decision`, because the invalidation side effect is now owned privately but storage mutation still needs its own source-backed decision. |

## Seam 025 Audit Changes

- Re-audited current C++ `CpuRdramReservation` fields, `clear_cpu_rdram_reservation`,
  `set_cpu_rdram_reservation`, `invalidate_cpu_rdram_reservation_for_write`,
  RDRAM write callers, LL/LLD setup, SC/SCD match/clear paths, reset clearing,
  and proof observations.
- Re-audited Rust `machine/rdram_reservation.rs`, private Machine ownership,
  invalidation tests, Machine preservation coverage, `lib.rs` exports, README,
  and this ledger for overclaims.
- Added one seal-level Rust test proving repeated staging followed by
  invalidation uses the latest staged reservation, and that a cleared invalid
  reservation remains invalid/zero on a later invalidation attempt.
- Sealed private invalidation parity only. No RDRAM write, range write, nonzero
  RDRAM mutation, LL/SC, CPU load/store, memory-map, DMA, reset, step,
  execution, renderer, SDL, host shell, or C++ integration behavior was added.
- No C++ source files were changed.

## RDRAM Raw Byte Write Decision Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `write_rdram_u8` owner | `src/core/machine.hpp` private `Machine::write_rdram_u8(RdramOffset, std::uint8_t)`; `src/core/machine.cpp` definition | `machine.rs` public `Machine::write_rdram_u8(offset, value)` | Equivalent behavior, different ownership shape | Raw byte-write tests; source inspection | C++ helper is private and reached by C++ owners. Rust exposes a narrow Machine-level sidecar seam because both RDRAM and reservation state are Machine-owned. |
| Rust raw byte write owner | C++ private helper listed above | `Machine::write_rdram_u8(&mut self, offset: usize, value: u8) -> Result<(), RdramAccessError>` | Equivalent behavior, different ownership shape | Raw byte-write tests; API inspection | This is raw storage-offset access, not CPU address, load/store, bus, or memory-map behavior. |
| C++ RDRAM storage mutation owner | `src/core/machine.cpp` `rdram_[address] = value` | `rdram.rs` private `Rdram::write_u8_at_checked_offset` | Equivalent behavior, different ownership shape | First/last byte and neighbor-preservation tests | Rust keeps storage mutation local to `Rdram` and calls it only after Machine-level validation/invalidation ordering. |
| Rust RDRAM storage mutation owner | C++ `rdram_` byte assignment | `Rdram` private `bytes: Vec<u8>` plus crate-private checked-offset mutation | Equivalent storage semantics, different ownership shape | Raw byte-write tests | No public `Rdram::write_u8` API exists. |
| Offset/address input | C++ input type is `RdramOffset = std::uint32_t` | Rust input is `offset: usize` and valid offsets are raw RDRAM storage offsets | Equivalent for valid storage offsets; Rust API shape differs | First/last byte tests; source inspection | Rust accepts `usize` to match existing `Rdram::read_u8`; valid offsets are below 4 MiB and fit the C++ `RdramOffset` domain before invalidation. |
| Invalid offset behavior | C++ throws `std::out_of_range` through `fail_rdram_access(address, 1)` | Rust returns `Err(RdramAccessError { offset, width: 1 })` | Rust-only API safety, no emulator truth | Invalid-write tests | Display text mirrors C++ `RDRAM access out of range: address=... width=1`. |
| Bounds-check ordering | C++ checks `address >= rdram_.size()` before invalidation or storage mutation | Rust calls `Rdram::require_u8_offset` before invalidation or storage mutation | Equivalent behavior, different API shape | Invalid-write reservation/storage preservation test | Invalid writes do not touch reservation state or RDRAM bytes. |
| Reservation invalidation ordering | C++ calls `invalidate_cpu_rdram_reservation_for_write(address, 1)` after bounds check and before storage mutation | Rust calls `CpuRdramReservation::invalidate_for_rdram_write(offset as u32, 1)` after bounds check and before storage mutation | Equivalent behavior, different ownership shape | Reservation invalidation write tests | The cast happens only after valid-offset check; valid RDRAM offsets fit `u32`. |
| Storage mutation ordering | C++ assigns `rdram_[address] = value` after invalidation | Rust calls private `Rdram::write_u8_at_checked_offset` after invalidation | Equivalent behavior, different ownership shape | Raw byte-write tests | Mutation happens after reservation invalidation, matching C++. |
| Reservation invalidation width | C++ byte write passes width `1` | Rust byte write passes width `1` | Equivalent | Same-offset invalidation test; source inspection | This uses the already-sealed invalidation rule. |
| First-byte write | C++ accepts offset `0` | Rust `Machine::write_rdram_u8(0, value)` succeeds | Equivalent | `raw_rdram_byte_write_updates_first_and_last_storage_offsets` | `read_u8(0)` observes the written value. |
| Last-byte write | C++ accepts `kRdramSizeBytes - 1` | Rust accepts `RDRAM_SIZE_BYTES - 1` | Equivalent | `raw_rdram_byte_write_updates_first_and_last_storage_offsets` | Last byte succeeds for byte-width writes. |
| Exact-length invalid write | C++ rejects offset `kRdramSizeBytes` | Rust rejects `RDRAM_SIZE_BYTES` | Rust-only API safety for error carrier; storage boundary equivalent | `raw_rdram_byte_write_rejects_invalid_offsets_before_mutation` | Error width is `1`; reservation and bytes are unchanged. |
| Past-end invalid write | C++ rejects offsets beyond RDRAM size | Rust rejects `RDRAM_SIZE_BYTES + 1` | Rust-only API safety for error carrier; storage boundary equivalent | `raw_rdram_byte_write_rejects_invalid_offsets_before_mutation` | Error text mirrors C++. |
| One-byte mutation behavior | C++ assigns exactly one `rdram_` byte | Rust writes exactly one `Rdram::bytes` element | Equivalent | First/last and preservation tests | No range or multi-byte mutation is added. |
| Neighboring byte preservation | C++ byte assignment does not write adjacent bytes | Rust tests adjacent bytes remain unchanged | Equivalent | First/last and unrelated-state preservation tests | Confirms byte write has no endian spreading. |
| Nonzero storage creation through earned write behavior | C++ byte write can create nonzero RDRAM storage | Rust byte write creates nonzero storage through `Machine::write_rdram_u8` | Equivalent | First/last and preservation tests | This replaces the old absence of nonzero RDRAM mutation without adding fixture injection. |
| `read_u8` observing written byte | C++ raw reads return stored bytes after writes | Rust `Rdram::read_u8` observes bytes written through `Machine::write_rdram_u8` | Equivalent for raw byte storage seam | Raw byte-write tests | Observation remains storage-offset based. |
| Byte-level endian behavior | C++ byte write has no endian conversion | Rust byte write stores the single byte value unchanged | Equivalent | Neighbor-preservation tests; source inspection | u16/u32/u64 big-endian helpers remain absent. |
| Reservation overlap invalidation behavior | C++ byte write invalidates overlapping reservations with width `1` | Rust byte write clears overlapping private reservation state through the sealed invalidation helper | Equivalent behavior, different ownership shape | `raw_rdram_byte_write_invalidates_only_overlapping_reservation` | Same-offset byte write clears `valid`, `rdram_offset`, and `width`. |
| Reservation non-overlap preservation behavior | C++ non-overlapping byte writes preserve reservation fields | Rust adjacent before/after byte writes preserve staged reservation fields | Equivalent | `raw_rdram_byte_write_invalidates_only_overlapping_reservation` | Adjacent byte writes use the half-open rule from seam 025. |
| Invalid write reservation preservation behavior | C++ bounds-checks before invalidation | Rust returns error before invalidation | Equivalent behavior, Rust API safety for error | `raw_rdram_byte_write_rejects_invalid_offsets_before_mutation` | Invalid writes leave a staged reservation intact. |
| Write mutates CPU fields | C++ raw byte helper does not assign CPU GPR/scalar fields | Rust write method does not mutate `Cpu` | Equivalent for represented state | `raw_rdram_byte_write_preserves_unrelated_machine_facts` | GPRs, register zero, PC, next PC, HI, and LO remain unchanged. |
| Write mutates COP0 fields | C++ raw byte helper does not reference COP0 | Rust write method does not mutate `Cpu`/`Cop0` | Equivalent for represented state | Preservation test | COP0 construction/access fields remain unchanged. |
| Write mutates Cartridge facts | C++ raw byte helper does not reference `cartridge_` | Rust write method does not mutate `Cartridge` | Equivalent for represented state | Preservation test | Cartridge bytes and metadata remain unchanged. |
| Machine-level write exists and why | C++ helper is a private Machine method mutating both `rdram_` and `cpu_rdram_reservation_` | Rust exposes `Machine::write_rdram_u8` | Equivalent behavior, different ownership shape | Source inspection; API inspection | Machine owns both `Rdram` and `CpuRdramReservation`, so Machine is the honest seam owner. |
| RDRAM-level private mutation exists and why | C++ writes storage in `Machine::write_rdram_u8` after invalidation | Rust has private `Rdram::write_u8_at_checked_offset` | Equivalent storage semantics, different ownership shape | Source/API inspection | This prevents a public storage-only write that would skip reservation invalidation. |
| u16/u32/u64 writes at seam 026 | C++ has `write_rdram_u16_be/u32_be/u64_be` | Later seams mirror these as raw write-width APIs | Documentation only | Source inspection | This row records the seam 026 byte-write boundary. Later seams added and sealed u16_be, u32_be, and u64_be raw writes. |
| Range writes | C++ staging/DMA/copy paths loop through write helpers | No Rust range write API | C++ exists, Rust intentionally absent | Source inspection | Range writes remain tied to staging/DMA/load-store owners. |
| CPU load/store | C++ CPU memory write paths translate CPU addresses and call raw helpers | No Rust CPU load/store API | Not in scope | Source inspection | `write_rdram_u8` is not a CPU store. |
| LL/SC | C++ LL/LLD and SC/SCD instruction paths interact with reservation state | No Rust LL/SC API | Not in scope | Source inspection | Byte write can invalidate reservation state, but it does not implement LL/SC instructions. |
| Memory-map | C++ CPU target resolution produces `RdramOffset` before raw helpers | No Rust memory-map API | Not in scope | Source inspection | Raw byte write takes a storage offset, not a mapped CPU address. |
| Bus | No Rust bus owner exists; C++ does not expose this helper as a bus abstraction | No Rust bus API | Not in scope | Source/API inspection | No bus abstraction is introduced. |
| DMA | C++ SP/SI/PI DMA can call write helpers through their own owners | No Rust DMA API | Not in scope | Source inspection | DMA remains intentionally absent. |
| Reset | C++ reset clears many owners including RDRAM and reservation | No Rust reset API | Not in scope | Source inspection | Byte write does not imply reset readiness. |
| Rust-only API safety | C++ throws for invalid offset | Rust returns `RdramAccessError` | Rust-only API safety, no emulator truth | Invalid-write tests | Existing error type is reused; no broad error type is added. |
| Naming/layout changes made for this seam | C++ names the raw helper `write_rdram_u8` | Rust adds `Machine::write_rdram_u8`; private `Rdram` mutation stays in `rdram.rs` | Rust-only repo hygiene, no emulator truth | Layout/API inspection | No new module, broad bucket, host concept, or Rust-branded name was added. |
| Recommended next seam | C++ byte write is now mirrored; multi-byte writes remain unearned | Rust byte write is implemented but not yet sealed | Ready for RDRAM write parity seal | Rust tests and source inspection | Recommend `rust_parallel_core_seam_027_rdram_raw_byte_write_parity_seal`, because the new byte-write seam should be sealed before u16/u32/u64 or range writes. |

## Seam 026 Audit Changes

- Re-audited C++ `RdramOffset`, RDRAM storage, `fail_rdram_access`,
  `write_rdram_u8`, multi-byte RDRAM write helpers, DMA callers, CPU load/store
  callers, reset clearing, and LL/SC reservation callers.
- Added `Machine::write_rdram_u8(offset, value)` as a raw RDRAM storage-offset
  byte write seam.
- Added private storage-local RDRAM methods for checked-offset validation and
  checked byte mutation; no public `Rdram::write_u8` was added.
- Mirrored the C++ order exactly for the earned byte-write subset:
  bounds-check first, reservation invalidation with width `1` second, then one
  byte of storage mutation.
- Reused `RdramAccessError` for invalid write offsets as Rust-only API safety.
- Added tests for first/last byte writes, invalid exact-end and past-end writes,
  one-byte mutation, neighbor preservation, nonzero storage creation, `read_u8`
  observation, reservation overlap invalidation, non-overlap preservation,
  invalid-write reservation preservation, and unrelated Machine fact preservation.
- Did not add u16/u32/u64 writes, range writes, CPU load/store, LL/SC,
  memory-map, bus, DMA, reset, step, execution, renderer, SDL, host shell, or
  C++ integration behavior.
- No C++ source files were changed.

## RDRAM Raw Byte Write Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `write_rdram_u8` owner | `src/core/machine.hpp` private `Machine::write_rdram_u8(RdramOffset, std::uint8_t)`; `src/core/machine.cpp` definition | `machine.rs` `Machine::write_rdram_u8(offset, value)` | Equivalent behavior, different ownership shape | Raw byte-write tests; source inspection | C++ helper is private and reached by C++ owners. Rust keeps the sidecar seam on `Machine` because the operation mutates both RDRAM and reservation state. |
| Rust `write_rdram_u8` owner | C++ private helper listed above | `Machine::write_rdram_u8(&mut self, offset: usize, value: u8) -> Result<(), RdramAccessError>` | Equivalent behavior, different ownership shape | Raw byte-write tests; API inspection | The Rust API is raw storage-offset access, not CPU address, load/store, bus, or memory-map behavior. |
| C++ RDRAM storage mutation owner | `src/core/machine.cpp` `rdram_[address] = value` | `rdram.rs` private `Rdram::write_u8_at_checked_offset` | Equivalent behavior, different ownership shape | First/last byte and neighbor-preservation tests | C++ assigns one byte directly in the Machine helper. Rust keeps the byte assignment storage-local but private. |
| Rust private RDRAM storage mutation owner | C++ `rdram_` byte assignment inside `Machine::write_rdram_u8` | `Rdram` private `bytes: Vec<u8>` plus crate-private checked-offset mutation | Equivalent storage semantics, different ownership shape | Raw byte-write tests; API inspection | No public `Rdram::write_u8` or storage-only write path exists. |
| Machine-level write ownership | C++ helper is a private `Machine` method mutating `rdram_` and `cpu_rdram_reservation_` | Rust exposes `Machine::write_rdram_u8` | Equivalent behavior, different ownership shape | Source/API inspection | Machine owns both `Rdram` and `CpuRdramReservation`, so Machine is the honest public sidecar owner for this write seam. |
| Private `Rdram` helper ownership | C++ does not expose a standalone storage-only RDRAM owner | `Rdram::require_u8_offset`; `Rdram::write_u8_at_checked_offset` are crate-private | Equivalent storage semantics, different ownership shape | API inspection | The helper split preserves storage ownership without bypassing reservation invalidation. |
| Offset/address input | C++ input type is `RdramOffset = std::uint32_t` | Rust input is `offset: usize`; valid offsets are raw RDRAM storage offsets | Equivalent for valid storage offsets; Rust API shape differs | First/last byte tests; source inspection | Valid Rust offsets are below 4 MiB and fit the C++ `RdramOffset` domain before invalidation. This is not a CPU address. |
| Invalid offset behavior | C++ throws `std::out_of_range` through `fail_rdram_access(address, 1)` | Rust returns `Err(RdramAccessError { offset, width: 1 })` | Rust-only API safety, no emulator truth | Invalid-write tests | The Rust error carrier is API safety; display text mirrors C++ `RDRAM access out of range: address=... width=1`. |
| Bounds-check ordering | C++ checks `address >= rdram_.size()` before invalidation or storage mutation | Rust calls `Rdram::require_u8_offset` before invalidation or storage mutation | Equivalent behavior, different API shape | Invalid-write reservation/storage preservation test | Invalid writes do not touch reservation state or RDRAM bytes. |
| Reservation invalidation ordering | C++ calls `invalidate_cpu_rdram_reservation_for_write(address, 1)` after bounds check and before storage mutation | Rust calls `CpuRdramReservation::invalidate_for_rdram_write(offset as u32, 1)` after bounds check and before storage mutation | Equivalent behavior, different ownership shape | Reservation invalidation write tests | The cast is after a valid-offset check; valid offsets fit `u32`. |
| Storage mutation ordering | C++ assigns `rdram_[address] = value` after invalidation | Rust calls private `Rdram::write_u8_at_checked_offset` after invalidation | Equivalent behavior, different ownership shape | Raw byte-write tests | Mutation happens after reservation invalidation, matching C++. |
| Reservation invalidation width | C++ byte write passes width `1` | Rust byte write passes width `1` | Equivalent | Same-offset invalidation test; source inspection | This uses the sealed half-open invalidation rule. |
| First-byte write | C++ accepts offset `0` | Rust `Machine::write_rdram_u8(0, value)` succeeds | Equivalent | `raw_rdram_byte_write_updates_first_and_last_storage_offsets` | `read_u8(0)` observes the written value. |
| Last-byte write | C++ accepts `kRdramSizeBytes - 1` | Rust accepts `RDRAM_SIZE_BYTES - 1` | Equivalent | `raw_rdram_byte_write_updates_first_and_last_storage_offsets` | Last byte succeeds for byte-width writes. |
| Exact-length invalid write | C++ rejects offset `kRdramSizeBytes` | Rust rejects `RDRAM_SIZE_BYTES` | Rust-only API safety for error carrier; storage boundary equivalent | `raw_rdram_byte_write_rejects_invalid_offsets_before_mutation` | Error width is `1`; reservation and bytes are unchanged. |
| Past-end invalid write | C++ rejects offsets beyond RDRAM size | Rust rejects `RDRAM_SIZE_BYTES + 1` | Rust-only API safety for error carrier; storage boundary equivalent | `raw_rdram_byte_write_rejects_invalid_offsets_before_mutation` | Error text mirrors C++. |
| Invalid write RDRAM preservation | C++ throws before writing `rdram_` | Rust returns `Err` before private byte mutation | Equivalent behavior, Rust API safety for error | Invalid-write test | The test checks constructed storage remains unchanged at representative valid offsets after invalid exact-end and past-end writes. |
| Invalid write reservation preservation | C++ throws before invalidation | Rust returns `Err` before invalidation | Equivalent behavior, Rust API safety for error | Invalid-write test | A staged reservation remains valid with the same offset and width after invalid writes. |
| One-byte mutation behavior | C++ assigns exactly one `rdram_` byte | Rust writes exactly one `Rdram::bytes` element | Equivalent | First/last and preservation tests | No range or multi-byte mutation is added. |
| Neighboring byte preservation | C++ byte assignment does not write adjacent bytes | Rust tests adjacent bytes remain unchanged | Equivalent | First/last and unrelated-state preservation tests | Confirms byte write has no endian spreading or implicit multi-byte behavior. |
| Nonzero storage creation through earned write behavior | C++ byte write can create nonzero RDRAM storage | Rust byte write creates nonzero storage through `Machine::write_rdram_u8` | Equivalent | First/last and preservation tests | This is earned through the write seam, not a test-only fixture. |
| `read_u8` observing written byte | C++ raw reads return stored bytes after writes | Rust `Rdram::read_u8` observes bytes written through `Machine::write_rdram_u8` | Equivalent for raw byte storage seam | Raw byte-write tests | Observation remains storage-offset based. |
| Byte-level endian behavior | C++ byte write has no endian conversion | Rust byte write stores the single byte value unchanged | Equivalent | Neighbor-preservation tests; source inspection | u16/u32/u64 big-endian helpers remain intentionally absent. |
| Reservation overlap invalidation behavior | C++ byte write invalidates overlapping reservations with width `1` | Rust byte write clears overlapping private reservation state through the sealed helper | Equivalent behavior, different ownership shape | `raw_rdram_byte_write_invalidates_only_overlapping_reservation`; preservation test | Same-offset and contained-byte writes clear `valid`, `rdram_offset`, and `width`. |
| Reservation non-overlap preservation behavior | C++ non-overlapping byte writes preserve reservation fields | Rust adjacent-before and adjacent-after byte writes preserve staged reservation fields | Equivalent | `raw_rdram_byte_write_invalidates_only_overlapping_reservation` | Adjacent byte writes use the sealed half-open rule. |
| Repeated staging then write behavior | C++ `set_cpu_rdram_reservation` overwrites prior fields; writes invalidate against the latest fields | Rust repeated private staging overwrites prior fields before `Machine::write_rdram_u8` invalidation | Equivalent | `raw_rdram_byte_write_uses_latest_staged_reservation` | A write overlapping the old reservation is non-overlap after restaging; a write overlapping the latest reservation invalidates. |
| Write mutates CPU fields | C++ raw byte helper does not assign CPU GPR/scalar fields | Rust write method does not mutate `Cpu` | Equivalent for represented state | `raw_rdram_byte_write_preserves_unrelated_machine_facts` | GPRs, register zero, PC, next PC, HI, and LO remain unchanged. |
| Write mutates COP0 fields | C++ raw byte helper does not reference COP0 | Rust write method does not mutate `Cpu`/`Cop0` | Equivalent for represented state | Preservation test | COP0 construction/access fields remain unchanged. |
| Write mutates Cartridge facts | C++ raw byte helper does not reference `cartridge_` | Rust write method does not mutate `Cartridge` | Equivalent for represented state | Preservation test | Cartridge bytes and metadata remain unchanged. |
| u16/u32/u64 writes at seam 027 | C++ has `write_rdram_u16_be/u32_be/u64_be` | Later seams mirror these as raw write-width APIs | Documentation only | Source inspection | This row records the seam 027 byte-write seal boundary. Later seams added and sealed u16_be, u32_be, and u64_be raw writes. |
| Range writes | C++ staging/DMA/copy paths loop through write helpers | No Rust range write API | C++ exists, Rust intentionally absent | Source inspection | Range writes remain tied to staging/DMA/load-store owners. |
| CPU load/store | C++ CPU memory write paths translate CPU addresses and call raw helpers | No Rust CPU load/store API | Not in scope | Source inspection | `write_rdram_u8` is not a CPU store. |
| LL/SC | C++ LL/LLD and SC/SCD instruction paths interact with reservation state | No Rust LL/SC API | Not in scope | Source inspection | Byte write can invalidate reservation state, but it does not implement LL/SC instructions. |
| Memory-map | C++ CPU target resolution produces `RdramOffset` before raw helpers | No Rust memory-map API | Not in scope | Source inspection | Raw byte write takes a storage offset, not a mapped CPU address. |
| Bus | No Rust bus owner exists; C++ does not expose this helper as a bus abstraction | No Rust bus API | Not in scope | Source/API inspection | No bus abstraction is introduced. |
| DMA | C++ SP/SI/PI DMA can call write helpers through their own owners | No Rust DMA API | Not in scope | Source inspection | DMA remains intentionally absent. |
| Reset | C++ reset clears many owners including RDRAM and reservation | No Rust reset API | Not in scope | Source inspection | Byte write does not imply reset readiness. |
| Rust-only API safety | C++ throws for invalid offset | Rust returns `RdramAccessError` | Rust-only API safety, no emulator truth | Invalid-write tests | Existing RDRAM access error remains appropriate for raw read and raw write access; no broad catchall error was added. |
| Naming/layout changes made for this seam | C++ names the raw helper `write_rdram_u8` | Rust keeps `Machine::write_rdram_u8`; private `Rdram` mutation stays in `rdram.rs` | Rust-only repo hygiene, no emulator truth | Layout/API inspection | No new module, broad bucket, host concept, mapped-memory name, or Rust-branded name was added. |
| Recommended next seam | C++ byte write parity is sealed; multi-byte writes remain unearned | Rust byte write is sealed; no u16/u32/u64 or range writes exist | Ready for u16_be write decision after seal | Rust tests and source inspection | Recommend `rust_parallel_core_seam_028_rdram_u16_be_write_decision`, because C++ has a distinct big-endian two-byte helper with width `2` invalidation and byte-order behavior. |

## Seam 027 Audit Changes

- Re-audited C++ `RdramOffset`, RDRAM storage, `fail_rdram_access`,
  `write_rdram_u8`, multi-byte RDRAM write helpers, DMA callers, CPU load/store
  callers, reset clearing, and LL/SC reservation callers.
- Re-audited Rust `Machine::write_rdram_u8`, private RDRAM checked-offset and
  byte-mutation helpers, private reservation invalidation, and byte-write tests.
- Added one seal-level test proving repeated reservation staging followed by
  `Machine::write_rdram_u8` uses the latest staged reservation.
- Confirmed `Machine::write_rdram_u8` remains a raw storage-offset byte write:
  bounds-check first, reservation invalidation with width `1` second, then one
  byte of storage mutation.
- Confirmed no public `Rdram::write_u8`, storage-only public write path,
  u16/u32/u64 write, range write, CPU load/store, LL/SC, memory-map, bus, DMA,
	  reset, step, execution, renderer, SDL, host shell, or C++ integration behavior
	  was added.
- No C++ source files were changed.

## RDRAM Raw u16_be Write Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `write_rdram_u16_be` owner | `src/core/machine.hpp` private `Machine::write_rdram_u16_be(RdramOffset, std::uint16_t)`; `src/core/machine.cpp` definition | `machine.rs` `Machine::write_rdram_u16_be(offset, value)` | Equivalent behavior, different ownership shape | Raw u16_be write tests; source inspection | C++ helper is private and reached by C++ owners. Rust keeps the sidecar seam on `Machine` because the operation mutates both RDRAM and reservation state. |
| Rust `write_rdram_u16_be` owner | C++ private helper listed above | `Machine::write_rdram_u16_be(&mut self, offset: usize, value: u16) -> Result<(), RdramAccessError>` | Equivalent behavior, different ownership shape | Raw u16_be write tests; API inspection | The Rust API is raw storage-offset access, not CPU address, load/store, bus, or memory-map behavior. |
| C++ RDRAM storage mutation owner | `src/core/machine.cpp` assigns `rdram_[address]` and `rdram_[address + 1]` | `rdram.rs` private `Rdram::write_u16_be_at_checked_offset` | Equivalent behavior, different ownership shape | First/last-valid and neighbor-preservation tests | C++ assigns two bytes directly in the Machine helper. Rust keeps the byte-pair assignment storage-local but private. |
| Rust private RDRAM storage mutation owner | C++ `rdram_` byte assignments inside `Machine::write_rdram_u16_be` | `Rdram` private `bytes: Vec<u8>` plus crate-private checked-offset mutation | Equivalent storage semantics, different ownership shape | Raw u16_be write tests; API inspection | No public `Rdram::write_u16_be` or storage-only write path exists. |
| Machine-level write ownership | C++ helper is a private `Machine` method mutating `rdram_` and `cpu_rdram_reservation_` | Rust exposes `Machine::write_rdram_u16_be` | Equivalent behavior, different ownership shape | Source/API inspection | Machine owns both `Rdram` and `CpuRdramReservation`, so Machine is the honest public sidecar owner for this write seam. |
| Private `Rdram` helper ownership | C++ does not expose a standalone storage-only RDRAM owner | `Rdram::require_u16_be_offset`; `Rdram::write_u16_be_at_checked_offset` are crate-private | Equivalent storage semantics, different ownership shape | API inspection | The helper split preserves storage ownership without bypassing reservation invalidation. |
| Offset/address input | C++ input type is `RdramOffset = std::uint32_t` | Rust input is `offset: usize`; valid offsets are raw RDRAM storage offsets | Equivalent for valid storage offsets; Rust API shape differs | First/last-valid tests; source inspection | Valid Rust offsets are below 4 MiB and fit the C++ `RdramOffset` domain before invalidation. This is not a CPU address. |
| Invalid offset/range behavior | C++ rejects `address > rdram_.size() - 2` through `fail_rdram_access(address, 2)` | Rust returns `Err(RdramAccessError { offset, width: 2 })` when `offset > len - 2` | Rust-only API safety, no emulator truth | Invalid u16_be tests | Rust returns `Result` instead of throwing. Display text mirrors C++ `RDRAM access out of range: address=... width=2`. |
| Bounds-check ordering | C++ checks `address > rdram_.size() - 2` before invalidation or storage mutation | Rust calls `Rdram::require_u16_be_offset` before invalidation or storage mutation | Equivalent behavior, different API shape | Invalid-write reservation/storage preservation test | Invalid u16_be writes do not touch reservation state or RDRAM bytes. |
| Reservation invalidation ordering | C++ calls `invalidate_cpu_rdram_reservation_for_write(address, 2)` after bounds check and before storage mutation | Rust calls `CpuRdramReservation::invalidate_for_rdram_write(offset as u32, 2)` after bounds check and before storage mutation | Equivalent behavior, different ownership shape | Reservation invalidation write tests | The cast is after a valid-offset check; valid offsets fit `u32`. |
| Storage mutation ordering | C++ writes two `rdram_` bytes after invalidation | Rust calls private `Rdram::write_u16_be_at_checked_offset` after invalidation | Equivalent behavior, different ownership shape | Raw u16_be write tests | Mutation happens after reservation invalidation, matching C++. |
| Reservation invalidation width | C++ u16_be write passes width `2` | Rust u16_be write passes width `2` | Equivalent | Overlap invalidation tests; source inspection | This uses the sealed half-open invalidation rule. |
| First valid offset write | C++ accepts offset `0` | Rust `Machine::write_rdram_u16_be(0, value)` succeeds | Equivalent | `raw_rdram_u16_be_write_updates_first_and_last_valid_storage_offsets` | `read_u8(0)` and `read_u8(1)` observe the high and low bytes. |
| Last valid offset write | C++ accepts `kRdramSizeBytes - 2` | Rust accepts `RDRAM_SIZE_BYTES - 2` | Equivalent | `raw_rdram_u16_be_write_updates_first_and_last_valid_storage_offsets` | The last valid u16_be write ends exactly at RDRAM length. |
| Odd-offset write behavior | C++ raw helper has no alignment branch and accepts any offset that satisfies `address <= rdram_.size() - 2` | Rust accepts odd storage offsets that satisfy the same two-byte range check | Equivalent | `raw_rdram_u16_be_write_accepts_odd_storage_offset_without_alignment_check`; source inspection | This is raw storage-offset behavior only. CPU `SH` alignment faults are checked before CPU load/store reaches the raw helper. |
| Exact-length invalid write | C++ rejects offset `kRdramSizeBytes` | Rust rejects `RDRAM_SIZE_BYTES` | Rust-only API safety for error carrier; storage boundary equivalent | `raw_rdram_u16_be_write_rejects_invalid_offsets_before_mutation` | Error width is `2`; reservation and bytes are unchanged. |
| Last-byte invalid write | C++ rejects `kRdramSizeBytes - 1` because two bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 1` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u16_be test | This proves the full two-byte span is checked. |
| Past-end invalid write | C++ rejects offsets beyond RDRAM size | Rust rejects `RDRAM_SIZE_BYTES + 1` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u16_be test | Error text mirrors C++. |
| Invalid write RDRAM preservation | C++ throws before writing `rdram_` | Rust returns `Err` before private u16_be mutation | Equivalent behavior, Rust API safety for error | Invalid u16_be test | The test checks previously written valid bytes and representative zero bytes remain unchanged. |
| Invalid write reservation preservation | C++ throws before invalidation | Rust returns `Err` before invalidation | Equivalent behavior, Rust API safety for error | Invalid u16_be test | A staged reservation remains valid with the same offset and width after invalid writes. |
| Two-byte mutation behavior | C++ assigns exactly two `rdram_` bytes | Rust writes exactly two `Rdram::bytes` elements | Equivalent | First/last-valid and preservation tests | No range, u32, or u64 mutation is added. |
| Neighboring byte preservation | C++ u16_be assignment does not write adjacent bytes | Rust tests adjacent bytes remain unchanged | Equivalent | First/last-valid and unrelated-state preservation tests | Confirms the helper writes only the two-byte span. |
| Big-endian byte order | C++ stores `(value >> 8) & 0xff`, then `value & 0xff` | Rust stores the high byte first and low byte second | Equivalent | `raw_rdram_u16_be_write_updates_first_and_last_valid_storage_offsets`; preservation test | This is raw storage byte order, not CPU load/store behavior. |
| `read_u8` observing written bytes | C++ raw reads return stored bytes after writes | Rust `Rdram::read_u8` observes bytes written through `Machine::write_rdram_u16_be` | Equivalent for raw byte storage seam | Raw u16_be write tests | Observation remains storage-offset based. |
| Alignment validation presence or absence | C++ raw helper has no alignment check; it only range-checks width `2` | Rust raw helper has no alignment check; it only range-checks width `2` | Equivalent | Source inspection; odd-offset and unaligned-overlap tests | CPU `SH` alignment checks exist elsewhere and are not part of this raw storage seam. |
| Reservation overlap invalidation behavior | C++ u16_be write invalidates overlapping reservations with width `2` | Rust u16_be write clears overlapping private reservation state through the sealed helper | Equivalent behavior, different ownership shape | `raw_rdram_u16_be_write_invalidates_only_overlapping_reservation`; preservation test | One-byte overlap at reservation start clears `valid`, `rdram_offset`, and `width`. |
| Reservation non-overlap preservation behavior | C++ non-overlapping u16_be writes preserve reservation fields | Rust adjacent-before and adjacent-after u16_be writes preserve staged reservation fields | Equivalent | `raw_rdram_u16_be_write_invalidates_only_overlapping_reservation` | Adjacent ranges use the sealed half-open rule. |
| Repeated staging then write behavior | C++ `set_cpu_rdram_reservation` overwrites prior fields; writes invalidate against latest fields | Rust repeated private staging overwrites prior fields before `Machine::write_rdram_u16_be` invalidation | Equivalent | `raw_rdram_u16_be_write_uses_latest_staged_reservation` | A write overlapping the old reservation is non-overlap after restaging; a write overlapping the latest reservation invalidates. |
| Write mutates CPU fields | C++ raw u16_be helper does not assign CPU GPR/scalar fields | Rust write method does not mutate `Cpu` | Equivalent for represented state | `raw_rdram_u16_be_write_preserves_unrelated_machine_facts` | GPRs, register zero, PC, next PC, HI, and LO remain unchanged. |
| Write mutates COP0 fields | C++ raw u16_be helper does not reference COP0 | Rust write method does not mutate `Cpu`/`Cop0` | Equivalent for represented state | Preservation test | COP0 construction/access fields remain unchanged. |
| Write mutates Cartridge facts | C++ raw u16_be helper does not reference `cartridge_` | Rust write method does not mutate `Cartridge` | Equivalent for represented state | Preservation test | Cartridge bytes and metadata remain unchanged. |
| u32/u64 writes at seam 029 | C++ has `write_rdram_u32_be/u64_be` | Later seams mirror these as raw write-width APIs | Documentation only | Source inspection | This row records the seam 029 u16_be seal boundary. Later seams added and sealed u32_be and u64_be raw writes. |
| Range writes | C++ staging/DMA/copy paths loop through write helpers | No Rust range write API | C++ exists, Rust intentionally absent | Source inspection | Range writes remain tied to staging/DMA/load-store owners. |
| CPU load/store | C++ CPU halfword stores translate CPU addresses and call raw helpers | No Rust CPU load/store API | Not in scope | Source inspection | `write_rdram_u16_be` is not a CPU store-halfword. |
| LL/SC | C++ LL/LLD and SC/SCD instruction paths interact with reservation state | No Rust LL/SC API | Not in scope | Source inspection | u16_be write can invalidate reservation state, but it does not implement LL/SC instructions. |
| Memory-map | C++ CPU target resolution produces `RdramOffset` before raw helpers | No Rust memory-map API | Not in scope | Source inspection | Raw u16_be write takes a storage offset, not a mapped CPU address. |
| Bus | No Rust bus owner exists; C++ does not expose this helper as a bus abstraction | No Rust bus API | Not in scope | Source/API inspection | No bus abstraction is introduced. |
| DMA | C++ DMA can reach RDRAM writes through separate owners | No Rust DMA API | Not in scope | Source inspection | DMA remains intentionally absent. |
| Reset | C++ reset clears many owners including RDRAM and reservation | No Rust reset API | Not in scope | Source inspection | u16_be write does not imply reset readiness. |
| Rust-only API safety | C++ throws for invalid raw u16_be offset/range | Rust returns `RdramAccessError` | Rust-only API safety, no emulator truth | Invalid u16_be tests | Existing RDRAM access error remains appropriate for raw read and raw write access; no broad catchall error was added. |
| Naming/layout changes made for this seam | C++ names the raw helper `write_rdram_u16_be` | Rust adds `Machine::write_rdram_u16_be`; private `Rdram` mutation stays in `rdram.rs` | Rust-only repo hygiene, no emulator truth | Layout/API inspection | No new module, broad bucket, host concept, mapped-memory name, or Rust-branded name was added. |
| Recommended next seam | C++ u16_be write parity is sealed; u32/u64 writes remain unearned | Rust u16_be write is sealed; no u32/u64 or range writes exist | Ready for u32_be write decision after seal | Rust tests and source inspection | Recommend `rust_parallel_core_seam_030_rdram_u32_be_write_decision`, because C++ has a distinct big-endian four-byte helper with width `4` invalidation and byte-order behavior. |

## Seam 028 Audit Changes

- Re-audited C++ `RdramOffset`, RDRAM storage, `fail_rdram_access`,
  `write_rdram_u8`, `write_rdram_u16_be`, larger RDRAM write helpers, CPU
  load/store callers, DMA callers, reset clearing, and LL/SC reservation callers.
- Added `Machine::write_rdram_u16_be(offset, value)` as a raw RDRAM
  storage-offset u16_be write seam.
- Added private storage-local RDRAM methods for checked u16_be offset validation
  and checked big-endian two-byte mutation; no public `Rdram::write_u16_be` was
  added.
- Mirrored the C++ order exactly for the earned u16_be write subset:
  bounds-check the full two-byte span first, reservation invalidation with width
  `2` second, then two bytes of big-endian storage mutation.
- Reused `RdramAccessError` for invalid u16_be write offsets/ranges as Rust-only
  API safety.
- Added tests for first and last valid u16_be writes, invalid last-byte,
  exact-end, and past-end writes, two-byte mutation, neighbor preservation,
  big-endian byte order, `read_u8` observation, reservation overlap
  invalidation, non-overlap preservation, latest staged reservation behavior,
  invalid-write reservation/RDRAM preservation, and unrelated Machine fact
  preservation.
- Did not add u32/u64 writes, range writes, CPU load/store, LL/SC, memory-map,
  bus, DMA, reset, step, execution, renderer, SDL, host shell, or C++
  integration behavior.
- No C++ source files were changed.

## Seam 029 Audit Changes

- Re-audited C++ `Machine::write_rdram_u16_be`, `RdramOffset`, RDRAM storage,
  `fail_rdram_access`, reservation invalidation ordering, CPU `SH` alignment
  checks, CPU memory-write dispatch, DMA callers, reset clearing, and LL/SC
  reservation callers.
- Confirmed Rust `Machine::write_rdram_u16_be` exactly mirrors the raw C++
  helper for the earned seam: full two-byte bounds-check first, reservation
  invalidation with width `2` second, then two bytes of big-endian storage
  mutation.
- Strengthened tests for odd raw storage offsets. Odd offsets are accepted by
  the raw helper when the two-byte span is in range because C++ has no raw-helper
  alignment validation; CPU halfword alignment remains separate load/store
  behavior and is not implemented in Rust.
- Confirmed invalid last-byte, exact-end, and past-end offsets preserve both
  RDRAM bytes and reservation state because bounds checking happens before
  invalidation and mutation.
- Confirmed the private `Rdram` checked-offset and mutation helpers do not expose
  a public storage-only RDRAM write path that could bypass Machine-owned
  reservation invalidation.
- Did not add, remove, or change emulator behavior beyond parity-seal test/docs.
  No u32/u64 writes, range writes, CPU load/store, LL/SC, memory-map, bus, DMA,
  reset, step, execution, renderer, SDL, host shell, or C++ integration behavior
  was added.
- No C++ source files were changed.

## RDRAM Raw u32_be Write Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `write_rdram_u32_be` owner | `src/core/machine.hpp` private `Machine::write_rdram_u32_be(RdramOffset, std::uint32_t)`; `src/core/machine.cpp` definition | `machine.rs` `Machine::write_rdram_u32_be(offset, value)` | Equivalent behavior, different ownership shape | Raw u32_be write tests; source inspection | C++ helper is private and reached by C++ owners. Rust exposes a narrow Machine-level sidecar seam because the operation mutates both RDRAM and reservation state. |
| Rust `write_rdram_u32_be` owner | C++ private helper listed above | `Machine::write_rdram_u32_be(&mut self, offset: usize, value: u32) -> Result<(), RdramAccessError>` | Equivalent behavior, different ownership shape | Raw u32_be write tests; API inspection | This is raw storage-offset access, not CPU address, load/store, bus, or memory-map behavior. |
| C++ RDRAM storage mutation owner | `src/core/machine.cpp` assigns `rdram_[address]` through `rdram_[address + 3]` | `rdram.rs` private `Rdram::write_u32_be_at_checked_offset` | Equivalent behavior, different ownership shape | First/last-valid and neighbor-preservation tests | C++ assigns four bytes directly in the Machine helper. Rust keeps the four-byte assignment storage-local but private. |
| Rust private RDRAM storage mutation owner | C++ `rdram_` byte assignments inside `Machine::write_rdram_u32_be` | `Rdram` private `bytes: Vec<u8>` plus crate-private checked-offset mutation | Equivalent storage semantics, different ownership shape | Raw u32_be write tests; API inspection | No public `Rdram::write_u32_be` or storage-only write path exists. |
| Machine-level write ownership | C++ helper is a private `Machine` method mutating `rdram_` and `cpu_rdram_reservation_` | Rust exposes `Machine::write_rdram_u32_be` | Equivalent behavior, different ownership shape | Source/API inspection | Machine owns both `Rdram` and `CpuRdramReservation`, so Machine is the honest public sidecar owner for this write seam. |
| Private `Rdram` helper ownership | C++ does not expose a standalone storage-only RDRAM owner | `Rdram::require_u32_be_offset`; `Rdram::write_u32_be_at_checked_offset` are crate-private | Equivalent storage semantics, different ownership shape | API inspection | The helper split preserves storage ownership without bypassing reservation invalidation. |
| Private checked-offset helper ownership | C++ range check is local to `Machine::write_rdram_u32_be` before invalidation | `Rdram::require_u32_be_offset` is crate-private and storage-local | Equivalent behavior, different ownership shape | Source/API inspection; invalid-offset tests | The helper only checks the raw RDRAM storage span for width `4`; it is not a memory-map, bus, or CPU-address helper. |
| Offset/address input | C++ input type is `RdramOffset = std::uint32_t` | Rust input is `offset: usize`; valid offsets are raw RDRAM storage offsets | Equivalent for valid storage offsets; Rust API shape differs | First/last-valid tests; source inspection | Valid Rust offsets are below 4 MiB and fit the C++ `RdramOffset` domain before invalidation. This is not a CPU address. |
| Invalid offset/range behavior | C++ rejects `address > rdram_.size() - 4` through `fail_rdram_access(address, 4)` | Rust returns `Err(RdramAccessError { offset, width: 4 })` when `offset > len - 4` | Rust-only API safety, no emulator truth | Invalid u32_be tests | Rust returns `Result` instead of throwing. Display text mirrors C++ `RDRAM access out of range: address=... width=4`. |
| Bounds-check ordering | C++ checks `address > rdram_.size() - 4` before invalidation or storage mutation | Rust calls `Rdram::require_u32_be_offset` before invalidation or storage mutation | Equivalent behavior, different API shape | Invalid-write reservation/storage preservation test | Invalid u32_be writes do not touch reservation state or RDRAM bytes. |
| Reservation invalidation ordering | C++ calls `invalidate_cpu_rdram_reservation_for_write(address, 4)` after bounds check and before storage mutation | Rust calls `CpuRdramReservation::invalidate_for_rdram_write(offset as u32, 4)` after bounds check and before storage mutation | Equivalent behavior, different ownership shape | Reservation invalidation write tests | The cast is after a valid-offset check; valid offsets fit `u32`. |
| Storage mutation ordering | C++ writes four `rdram_` bytes after invalidation | Rust calls private `Rdram::write_u32_be_at_checked_offset` after invalidation | Equivalent behavior, different ownership shape | Raw u32_be write tests | Mutation happens after reservation invalidation, matching C++. |
| Reservation invalidation width | C++ u32_be write passes width `4` | Rust u32_be write passes width `4` | Equivalent | Overlap invalidation tests; source inspection | This uses the sealed half-open invalidation rule. |
| First valid offset write | C++ accepts offset `0` | Rust `Machine::write_rdram_u32_be(0, value)` succeeds | Equivalent | `raw_rdram_u32_be_write_updates_first_and_last_valid_storage_offsets` | `read_u8(0..3)` observes the four written bytes. |
| Last valid offset write | C++ accepts `kRdramSizeBytes - 4` | Rust accepts `RDRAM_SIZE_BYTES - 4` | Equivalent | `raw_rdram_u32_be_write_updates_first_and_last_valid_storage_offsets` | The last valid u32_be write ends exactly at RDRAM length. |
| Odd/unaligned-offset write behavior | C++ raw helper has no alignment branch and accepts any offset that satisfies `address <= rdram_.size() - 4` | Rust accepts unaligned storage offsets that satisfy the same four-byte range check | Equivalent | `raw_rdram_u32_be_write_accepts_unaligned_storage_offset_without_alignment_check`; source inspection | This is raw storage-offset behavior only. CPU `SW` alignment faults are checked before CPU load/store reaches the raw helper. |
| Exact-length invalid write | C++ rejects offset `kRdramSizeBytes` | Rust rejects `RDRAM_SIZE_BYTES` | Rust-only API safety for error carrier; storage boundary equivalent | `raw_rdram_u32_be_write_rejects_invalid_offsets_before_mutation` | Error width is `4`; reservation and bytes are unchanged. |
| Last-byte invalid write | C++ rejects `kRdramSizeBytes - 1` because four bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 1` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u32_be test | This proves the full four-byte span is checked. |
| Second-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 2` because four bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 2` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u32_be test | Reservation and RDRAM are preserved before invalidation or mutation. |
| Third-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 3` because four bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 3` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u32_be test | This is the first invalid offset after the last valid four-byte span. |
| Past-end invalid write | C++ rejects offsets beyond RDRAM size | Rust rejects `RDRAM_SIZE_BYTES + 1` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u32_be test | Error width is `4`. |
| Invalid write RDRAM preservation | C++ throws before writing `rdram_` | Rust returns `Err` before private u32_be mutation | Equivalent behavior, Rust API safety for error | Invalid u32_be test | The test checks previously written valid bytes and representative zero bytes remain unchanged. |
| Invalid write reservation preservation | C++ throws before invalidation | Rust returns `Err` before invalidation | Equivalent behavior, Rust API safety for error | Invalid u32_be test | A staged reservation remains valid with the same offset and width after invalid writes. |
| Four-byte mutation behavior | C++ assigns exactly four `rdram_` bytes | Rust writes exactly four `Rdram::bytes` elements | Equivalent | First/last-valid and preservation tests | No range or u64 mutation is added. |
| Neighboring byte preservation | C++ u32_be assignment does not write adjacent bytes | Rust tests adjacent bytes remain unchanged | Equivalent | First/last-valid and unrelated-state preservation tests | Confirms the helper writes only the four-byte span. |
| Big-endian byte order | C++ stores `(value >> 24)`, `(value >> 16)`, `(value >> 8)`, then `value` low byte | Rust stores the same high-to-low byte order | Equivalent | u32_be byte-order tests | This is raw storage byte order, not CPU load/store behavior. |
| `read_u8` observing written bytes | C++ raw reads return stored bytes after writes | Rust `Rdram::read_u8` observes bytes written through `Machine::write_rdram_u32_be` | Equivalent for raw byte storage seam | Raw u32_be write tests | Observation remains storage-offset based. |
| Alignment validation presence or absence | C++ raw helper has no alignment check; it only range-checks width `4` | Rust raw helper has no alignment check; it only range-checks width `4` | Equivalent | Source inspection; unaligned-offset test | CPU `SW` alignment checks exist elsewhere and are not part of this raw storage seam. |
| Reservation overlap invalidation behavior | C++ u32_be write invalidates overlapping reservations with width `4` | Rust u32_be write clears overlapping private reservation state through the sealed helper | Equivalent behavior, different ownership shape | `raw_rdram_u32_be_write_invalidates_only_overlapping_reservation`; preservation test | Exact same range and one-byte overlap at reservation start clear `valid`, `rdram_offset`, and `width`. |
| Reservation non-overlap preservation behavior | C++ non-overlapping u32_be writes preserve reservation fields | Rust adjacent-before and adjacent-after u32_be writes preserve staged reservation fields | Equivalent | `raw_rdram_u32_be_write_invalidates_only_overlapping_reservation` | Adjacent ranges use the sealed half-open rule. |
| Repeated staging then write behavior | C++ `set_cpu_rdram_reservation` overwrites prior fields; writes invalidate against latest fields | Rust repeated private staging overwrites prior fields before `Machine::write_rdram_u32_be` invalidation | Equivalent | `raw_rdram_u32_be_write_uses_latest_staged_reservation` | A write overlapping the old reservation is non-overlap after restaging; a write overlapping the latest reservation invalidates. |
| Write mutates CPU fields | C++ raw u32_be helper does not assign CPU GPR/scalar fields | Rust write method does not mutate `Cpu` | Equivalent for represented state | `raw_rdram_u32_be_write_preserves_unrelated_machine_facts` | GPRs, register zero, PC, next PC, HI, and LO remain unchanged. |
| Write mutates COP0 fields | C++ raw u32_be helper does not reference COP0 | Rust write method does not mutate `Cpu`/`Cop0` | Equivalent for represented state | Preservation test | COP0 construction/access fields remain unchanged. |
| Write mutates Cartridge facts | C++ raw u32_be helper does not reference `cartridge_` | Rust write method does not mutate `Cartridge` | Equivalent for represented state | Preservation test | Cartridge bytes and metadata remain unchanged. |
| u64 write at seam 031 | C++ has `write_rdram_u64_be` | Seam 032 mirrors this as `Machine::write_rdram_u64_be` | Documentation only | Source inspection | This row records the seam 031 u32_be seal boundary. Seam 032 adds and seals the u64_be raw write. |
| Range writes | C++ staging/DMA/copy paths loop through write helpers | No Rust range write API | C++ exists, Rust intentionally absent | Source inspection | Range writes remain tied to staging/DMA/load-store owners. |
| CPU load/store | C++ CPU word stores translate CPU addresses and call raw helpers | No Rust CPU load/store API | Not in scope | Source inspection | `write_rdram_u32_be` is not a CPU store-word. |
| LL/SC | C++ LL/LLD and SC/SCD instruction paths interact with reservation state | No Rust LL/SC API | Not in scope | Source inspection | u32_be write can invalidate reservation state, but it does not implement LL/SC instructions. |
| Memory-map | C++ CPU target resolution produces `RdramOffset` before raw helpers | No Rust memory-map API | Not in scope | Source inspection | Raw u32_be write takes a storage offset, not a mapped CPU address. |
| Bus | No Rust bus owner exists; C++ does not expose this helper as a bus abstraction | No Rust bus API | Not in scope | Source/API inspection | No bus abstraction is introduced. |
| DMA | C++ DMA can reach RDRAM writes through separate owners | No Rust DMA API | Not in scope | Source inspection | DMA remains intentionally absent. |
| Reset | C++ reset clears many owners including RDRAM and reservation | No Rust reset API | Not in scope | Source inspection | u32_be write does not imply reset readiness. |
| Rust-only API safety | C++ throws for invalid raw u32_be offset/range | Rust returns `RdramAccessError` | Rust-only API safety, no emulator truth | Invalid u32_be tests | Existing RDRAM access error remains appropriate for raw read and raw write access; no broad catchall error was added. |
| Naming/layout changes made for this seam | C++ names the raw helper `write_rdram_u32_be` | Rust adds `Machine::write_rdram_u32_be`; private `Rdram` mutation stays in `rdram.rs` | Rust-only repo hygiene, no emulator truth | Layout/API inspection | No new module, broad bucket, host concept, mapped-memory name, or Rust-branded name was added. |
| Recommended next seam | C++ u32_be write is now mirrored and sealed; u64 writes remain unearned | Rust u32_be write is implemented and parity-sealed | Ready for u64_be write decision after seal | Rust tests and source inspection | Recommend `rust_parallel_core_seam_032_rdram_u64_be_write_decision`, because C++ has a distinct width-8 big-endian raw helper that must be audited before any u64 sidecar behavior. |

## Seam 030 Audit Changes

- Re-audited C++ `Machine::write_rdram_u32_be`, `RdramOffset`, RDRAM storage,
  `fail_rdram_access`, `stage_rdram_u32_be`, `write_rdram_u8`,
  `write_rdram_u16_be`, `write_rdram_u64_be`, CPU word-store and LL/SC callers,
  DMA callers, reset clearing, and proof/step-probe uses of staged u32 words.
- Added `Machine::write_rdram_u32_be(offset, value)` as a raw RDRAM
  storage-offset u32_be write seam.
- Added private storage-local RDRAM methods for checked u32_be offset validation
  and checked big-endian four-byte mutation; no public `Rdram::write_u32_be` was
  added.
- Mirrored the C++ order exactly for the earned u32_be write subset:
  bounds-check the full four-byte span first, reservation invalidation with
  width `4` second, then four bytes of big-endian storage mutation.
- Reused `RdramAccessError` for invalid u32_be write offsets/ranges as Rust-only
  API safety.
- Added tests for first and last valid u32_be writes, invalid third-to-last,
  second-to-last, last-byte, exact-end, and past-end writes, four-byte mutation,
  neighbor preservation, big-endian byte order, `read_u8` observation, unaligned
  raw storage offsets, reservation overlap invalidation, non-overlap
  preservation, latest staged reservation behavior, invalid-write
  reservation/RDRAM preservation, and unrelated Machine fact preservation.
- Did not add u64 writes, range writes, CPU load/store, LL/SC, memory-map, bus,
  DMA, reset, step, execution, renderer, SDL, host shell, or C++ integration
  behavior.
- No C++ source files were changed.

## Seam 031 Audit Changes

- Re-audited C++ `Machine::write_rdram_u32_be`, `RdramOffset`, RDRAM storage,
  `fail_rdram_access`, `write_rdram_u8`, `write_rdram_u16_be`,
  `write_rdram_u64_be`, CPU word-store and LL/SC callers, DMA callers, reset
  clearing, and proof/step-probe uses of staged u32 words.
- Confirmed Rust `Machine::write_rdram_u32_be` mirrors the raw C++ helper for
  the earned seam: full four-byte bounds-check first, reservation invalidation
  with width `4` second, then four bytes of big-endian storage mutation.
- Confirmed invalid third-to-last, second-to-last, last-byte, exact-end, and
  past-end offsets preserve both RDRAM bytes and reservation state because
  bounds checking happens before invalidation and mutation.
- Strengthened reservation-overlap coverage for the exact same raw u32_be range
  starting at the staged reservation offset, then restaged to keep the
  one-byte-overlap boundary proof.
- Confirmed the private `Rdram` checked-offset and mutation helpers remain
  storage-local and do not expose a public storage-only RDRAM write path that
  could bypass Machine-owned reservation invalidation.
- Did not add, remove, or change emulator behavior beyond parity-seal test/docs.
  No u64 writes, range writes, CPU load/store, LL/SC, memory-map, bus, DMA,
	  reset, step, execution, renderer, SDL, host shell, or C++ integration behavior
	  was added.
- No C++ source files were changed.

## RDRAM Raw u64_be Write Decision/Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `write_rdram_u64_be` owner | `src/core/machine.hpp` private `Machine::write_rdram_u64_be(RdramOffset, CpuRegisterValue)`; `src/core/machine.cpp` definition | `machine.rs` `Machine::write_rdram_u64_be(offset, value)` | Equivalent behavior, different ownership shape | Raw u64_be write tests; source inspection | C++ helper is private and reached by C++ owners. Rust exposes a narrow Machine-level sidecar seam because the operation mutates both RDRAM and reservation state. |
| Rust `write_rdram_u64_be` owner | C++ private helper listed above | `Machine::write_rdram_u64_be(&mut self, offset: usize, value: u64) -> Result<(), RdramAccessError>` | Equivalent behavior, different ownership shape | Raw u64_be write tests; API inspection | This is raw storage-offset access, not CPU address, load/store, bus, or memory-map behavior. |
| C++ RDRAM storage mutation owner | `src/core/machine.cpp` assigns `rdram_[address]` through `rdram_[address + 7]` | `rdram.rs` private `Rdram::write_u64_be_at_checked_offset` | Equivalent behavior, different ownership shape | First/last-valid and neighbor-preservation tests | C++ assigns eight bytes directly in the Machine helper. Rust keeps the eight-byte assignment storage-local but private. |
| Rust private RDRAM storage mutation owner | C++ `rdram_` byte assignments inside `Machine::write_rdram_u64_be` | `Rdram` private `bytes: Vec<u8>` plus crate-private checked-offset mutation | Equivalent storage semantics, different ownership shape | Raw u64_be write tests; API inspection | No public `Rdram::write_u64_be` or storage-only write path exists. |
| Machine-level write ownership | C++ helper is a private `Machine` method mutating `rdram_` and `cpu_rdram_reservation_` | Rust exposes `Machine::write_rdram_u64_be` | Equivalent behavior, different ownership shape | Source/API inspection | Machine owns both `Rdram` and `CpuRdramReservation`, so Machine is the honest public sidecar owner for this write seam. |
| Private `Rdram` helper ownership | C++ does not expose a standalone storage-only RDRAM owner | `Rdram::require_u64_be_offset`; `Rdram::write_u64_be_at_checked_offset` are crate-private | Equivalent storage semantics, different ownership shape | API inspection | The helper split preserves storage ownership without bypassing reservation invalidation. |
| Private checked-offset helper ownership | C++ range check is local to `Machine::write_rdram_u64_be` before invalidation | `Rdram::require_u64_be_offset` is crate-private and storage-local | Equivalent behavior, different ownership shape | Source/API inspection; invalid-offset tests | The helper only checks the raw RDRAM storage span for width `8`; it is not a memory-map, bus, or CPU-address helper. |
| Offset/address input | C++ input type is `RdramOffset = std::uint32_t` | Rust input is `offset: usize`; valid offsets are raw RDRAM storage offsets | Equivalent for valid storage offsets; Rust API shape differs | First/last-valid tests; source inspection | Valid Rust offsets are below 4 MiB and fit the C++ `RdramOffset` domain before invalidation. This is not a CPU address. |
| Invalid offset/range behavior | C++ rejects `address > rdram_.size() - 8` through `fail_rdram_access(address, 8)` | Rust returns `Err(RdramAccessError { offset, width: 8 })` when `offset > len - 8` | Rust-only API safety, no emulator truth | Invalid u64_be tests | Rust returns `Result` instead of throwing. Display text mirrors C++ `RDRAM access out of range: address=... width=8`. |
| Bounds-check ordering | C++ checks `address > rdram_.size() - 8` before invalidation or storage mutation | Rust calls `Rdram::require_u64_be_offset` before invalidation or storage mutation | Equivalent behavior, different API shape | Invalid-write reservation/storage preservation test | Invalid u64_be writes do not touch reservation state or RDRAM bytes. |
| Reservation invalidation ordering | C++ calls `invalidate_cpu_rdram_reservation_for_write(address, 8)` after bounds check and before storage mutation | Rust calls `CpuRdramReservation::invalidate_for_rdram_write(offset as u32, 8)` after bounds check and before storage mutation | Equivalent behavior, different ownership shape | Reservation invalidation write tests | The cast is after a valid-offset check; valid offsets fit `u32`. |
| Storage mutation ordering | C++ writes eight `rdram_` bytes after invalidation | Rust calls private `Rdram::write_u64_be_at_checked_offset` after invalidation | Equivalent behavior, different ownership shape | Raw u64_be write tests | Mutation happens after reservation invalidation, matching C++. |
| Reservation invalidation width | C++ u64_be write passes width `8` | Rust u64_be write passes width `8` | Equivalent | Overlap invalidation tests; source inspection | This uses the sealed half-open invalidation rule. |
| First valid offset write | C++ accepts offset `0` | Rust `Machine::write_rdram_u64_be(0, value)` succeeds | Equivalent | `raw_rdram_u64_be_write_updates_first_and_last_valid_storage_offsets` | `read_u8(0..7)` observes the eight written bytes. |
| Last valid offset write | C++ accepts `kRdramSizeBytes - 8` | Rust accepts `RDRAM_SIZE_BYTES - 8` | Equivalent | `raw_rdram_u64_be_write_updates_first_and_last_valid_storage_offsets` | The last valid u64_be write ends exactly at RDRAM length. |
| Odd/unaligned-offset write behavior | C++ raw helper has no alignment branch and accepts any offset that satisfies `address <= rdram_.size() - 8` | Rust accepts unaligned storage offsets that satisfy the same eight-byte range check | Equivalent | `raw_rdram_u64_be_write_accepts_unaligned_storage_offset_without_alignment_check`; source inspection | This is raw storage-offset behavior only. CPU `SD`/`SCD` alignment checks are checked before CPU load/store reaches the raw helper. |
| Exact-length invalid write | C++ rejects offset `kRdramSizeBytes` | Rust rejects `RDRAM_SIZE_BYTES` | Rust-only API safety for error carrier; storage boundary equivalent | `raw_rdram_u64_be_write_rejects_invalid_offsets_before_mutation` | Error width is `8`; reservation and bytes are unchanged. |
| Last-byte invalid write | C++ rejects `kRdramSizeBytes - 1` because eight bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 1` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | This proves the full eight-byte span is checked. |
| Second-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 2` because eight bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 2` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | Reservation and RDRAM are preserved before invalidation or mutation. |
| Third-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 3` because eight bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 3` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | Reservation and RDRAM are preserved before invalidation or mutation. |
| Fourth-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 4` because eight bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 4` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | Reservation and RDRAM are preserved before invalidation or mutation. |
| Fifth-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 5` because eight bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 5` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | Reservation and RDRAM are preserved before invalidation or mutation. |
| Sixth-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 6` because eight bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 6` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | Reservation and RDRAM are preserved before invalidation or mutation. |
| Seventh-to-last-byte invalid write | C++ rejects `kRdramSizeBytes - 7` because eight bytes would run past storage | Rust rejects `RDRAM_SIZE_BYTES - 7` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | This is the first invalid offset after the last valid eight-byte span. |
| Past-end invalid write | C++ rejects offsets beyond RDRAM size | Rust rejects `RDRAM_SIZE_BYTES + 1` | Rust-only API safety for error carrier; storage boundary equivalent | Invalid u64_be test | Error width is `8`. |
| Invalid write RDRAM preservation | C++ throws before writing `rdram_` | Rust returns `Err` before private u64_be mutation | Equivalent behavior, Rust API safety for error | Invalid u64_be test | The test checks previously written valid bytes and representative zero bytes remain unchanged. |
| Invalid write reservation preservation | C++ throws before invalidation | Rust returns `Err` before invalidation | Equivalent behavior, Rust API safety for error | Invalid u64_be test | A staged reservation remains valid with the same offset and width after invalid writes. |
| Eight-byte mutation behavior | C++ assigns exactly eight `rdram_` bytes | Rust writes exactly eight `Rdram::bytes` elements | Equivalent | First/last-valid and preservation tests | No range mutation is added. |
| Neighboring byte preservation | C++ u64_be assignment does not write adjacent bytes | Rust tests adjacent bytes remain unchanged | Equivalent | First/last-valid, unaligned, and unrelated-state preservation tests | Confirms the helper writes only the eight-byte span. |
| Big-endian byte order | C++ stores `(value >> 56)`, `(value >> 48)`, `(value >> 40)`, `(value >> 32)`, `(value >> 24)`, `(value >> 16)`, `(value >> 8)`, then `value` low byte | Rust stores the same high-to-low byte order | Equivalent | u64_be byte-order tests | This is raw storage byte order, not CPU load/store behavior. |
| `read_u8` observing written bytes | C++ raw reads return stored bytes after writes | Rust `Rdram::read_u8` observes bytes written through `Machine::write_rdram_u64_be` | Equivalent for raw byte storage seam | Raw u64_be write tests | Observation remains storage-offset based. |
| Alignment validation presence or absence | C++ raw helper has no alignment check; it only range-checks width `8` | Rust raw helper has no alignment check; it only range-checks width `8` | Equivalent | Source inspection; unaligned-offset test | CPU `SD`/`SCD` alignment checks exist elsewhere and are not part of this raw storage seam. |
| Reservation overlap invalidation behavior | C++ u64_be write invalidates overlapping reservations with width `8` | Rust u64_be write clears overlapping private reservation state through the sealed helper | Equivalent behavior, different ownership shape | `raw_rdram_u64_be_write_invalidates_only_overlapping_reservation`; preservation test | Exact same range and one-byte overlap at reservation start clear `valid`, `rdram_offset`, and `width`. |
| Reservation non-overlap preservation behavior | C++ non-overlapping u64_be writes preserve reservation fields | Rust adjacent-before and adjacent-after u64_be writes preserve staged reservation fields | Equivalent | `raw_rdram_u64_be_write_invalidates_only_overlapping_reservation` | Adjacent ranges use the sealed half-open rule. |
| Repeated staging then write behavior | C++ `set_cpu_rdram_reservation` overwrites prior fields; writes invalidate against latest fields | Rust repeated private staging overwrites prior fields before `Machine::write_rdram_u64_be` invalidation | Equivalent | `raw_rdram_u64_be_write_uses_latest_staged_reservation` | A write overlapping the old reservation is non-overlap after restaging; a write overlapping the latest reservation invalidates. |
| Write mutates CPU fields | C++ raw u64_be helper does not assign CPU GPR/scalar fields | Rust write method does not mutate `Cpu` | Equivalent for represented state | `raw_rdram_u64_be_write_preserves_unrelated_machine_facts` | GPRs, register zero, PC, next PC, HI, and LO remain unchanged. |
| Write mutates COP0 fields | C++ raw u64_be helper does not reference COP0 | Rust write method does not mutate `Cpu`/`Cop0` | Equivalent for represented state | Preservation test | COP0 construction/access fields remain unchanged. |
| Write mutates Cartridge facts | C++ raw u64_be helper does not reference `cartridge_` | Rust write method does not mutate `Cartridge` | Equivalent for represented state | Preservation test | Cartridge bytes and metadata remain unchanged. |
| Range writes | C++ staging/DMA/copy paths loop through write helpers | No Rust range write API | C++ exists, Rust intentionally absent | Source inspection | Range writes remain tied to staging/DMA/load-store owners. |
| CPU load/store | C++ CPU doubleword stores translate CPU addresses and call raw helpers | No Rust CPU load/store API | Not in scope | Source inspection | `write_rdram_u64_be` is not a CPU store-doubleword. |
| LL/SC | C++ LL/LLD and SC/SCD instruction paths interact with reservation state | No Rust LL/SC API | Not in scope | Source inspection | u64_be write can invalidate reservation state, but it does not implement LL/SC instructions. |
| Memory-map | C++ CPU target resolution produces `RdramOffset` before raw helpers | No Rust memory-map API | Not in scope | Source inspection | Raw u64_be write takes a storage offset, not a mapped CPU address. |
| Bus | No Rust bus owner exists; C++ does not expose this helper as a bus abstraction | No Rust bus API | Not in scope | Source/API inspection | No bus abstraction is introduced. |
| DMA | C++ DMA can reach RDRAM writes through separate owners | No Rust DMA API | Not in scope | Source inspection | DMA remains intentionally absent. |
| Reset | C++ reset clears many owners including RDRAM and reservation | No Rust reset API | Not in scope | Source inspection | u64_be write does not imply reset readiness. |
| Rust-only API safety | C++ throws for invalid raw u64_be offset/range | Rust returns `RdramAccessError` | Rust-only API safety, no emulator truth | Invalid u64_be tests | Existing RDRAM access error remains appropriate for raw read and raw write access; no broad catchall error was added. |
| Naming/layout changes made for this seam | C++ names the raw helper `write_rdram_u64_be` | Rust adds `Machine::write_rdram_u64_be`; private `Rdram` mutation stays in `rdram.rs` | Rust-only repo hygiene, no emulator truth | Layout/API inspection | No new module, broad bucket, host concept, mapped-memory name, or Rust-branded name was added. |
| Recommended next seam | C++ raw write-width family is mirrored for u8/u16_be/u32_be/u64_be; seam 033 mirrors raw read widths | Rust raw read/write storage access is sealed; CPU load/store remains absent | Ready for load/store readiness audit | Rust tests and source inspection | Recommend `rust_parallel_core_seam_034_load_store_readiness_audit`, because raw storage access is sealed and CPU load/store should be audited before any implementation. |

## RDRAM Raw Write-Family Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| u8 raw write status | `src/core/machine.cpp` `Machine::write_rdram_u8` | `machine.rs` `Machine::write_rdram_u8`; `rdram.rs` private `write_u8_at_checked_offset` | Raw write-family sealed | Raw byte-write tests; seam 027 seal | Bounds-checks one byte, invalidates with width `1`, then stores one byte. |
| u16_be raw write status | `src/core/machine.cpp` `Machine::write_rdram_u16_be` | `machine.rs` `Machine::write_rdram_u16_be`; `rdram.rs` private `write_u16_be_at_checked_offset` | Raw write-family sealed | Raw u16_be write tests; seam 029 seal | Bounds-checks two bytes, invalidates with width `2`, then stores high byte followed by low byte. |
| u32_be raw write status | `src/core/machine.cpp` `Machine::write_rdram_u32_be` | `machine.rs` `Machine::write_rdram_u32_be`; `rdram.rs` private `write_u32_be_at_checked_offset` | Raw write-family sealed | Raw u32_be write tests; seam 031 seal | Bounds-checks four bytes, invalidates with width `4`, then stores bytes from most significant to least significant. |
| u64_be raw write status | `src/core/machine.cpp` `Machine::write_rdram_u64_be` | `machine.rs` `Machine::write_rdram_u64_be`; `rdram.rs` private `write_u64_be_at_checked_offset` | Raw write-family sealed | Raw u64_be write tests; seam 032 seal | Bounds-checks eight bytes, invalidates with width `8`, then stores bytes from most significant to least significant. |
| Machine-level ownership reason | C++ raw write helpers are `Machine` methods mutating both `rdram_` and `cpu_rdram_reservation_` | Rust raw write methods live on `Machine` | Equivalent behavior, different ownership shape | API inspection; write tests | Machine owns both affected Rust fields, so public sidecar writes cannot bypass reservation invalidation. |
| Private `Rdram` mutation-helper reason | C++ storage mutation is private inside `Machine`; no standalone public storage-only write owner exists | Rust storage mutation helpers are crate-private in `rdram.rs` | Equivalent behavior, different ownership shape | API inspection | The split keeps storage-local byte assignments in `Rdram` while Machine retains reservation-aware write ownership. |
| Reservation invalidation width per helper | C++ passes widths `1`, `2`, `4`, and `8` | Rust passes widths `1`, `2`, `4`, and `8` | Equivalent | Raw write reservation tests | Each helper invalidates once with its full write width. |
| Big-endian behavior for multi-byte helpers | C++ u16/u32/u64 helpers store high-to-low bytes | Rust u16/u32/u64 helpers store high-to-low bytes | Equivalent | Byte-order tests | Byte write has no endian conversion; multi-byte helpers are explicitly big-endian storage writes. |
| Bounds-check-before-invalidation rule | C++ raw helpers check the full write span before invalidation | Rust `Rdram::require_*_offset` helpers run before invalidation | Equivalent behavior, Rust API safety for errors | Invalid-write tests | Invalid writes preserve reservation state. |
| Invalid-write preservation rule | C++ throws before invalidation and before storage mutation | Rust returns `Err(RdramAccessError)` before invalidation and before storage mutation | Equivalent behavior, Rust API safety for errors | Invalid-write tests | Invalid writes preserve RDRAM bytes and staged reservation fields. |
| No public storage-only write bypass | C++ raw writes are private `Machine` helpers; public paths are owner-specific | Rust exposes no public `Rdram::write_*` | Equivalent ownership guard, different API shape | API inspection | All public Rust raw writes go through `Machine` and reservation invalidation. |
| CPU load/store still out of scope | C++ CPU memory paths translate addresses and call raw helpers | No Rust CPU load/store API | Not in scope | Source inspection | Raw writes take storage offsets and do not implement CPU store instructions. |
| Memory-map/bus still out of scope | C++ target resolution is separate from raw helper storage offsets | No Rust memory-map or bus API | Not in scope | Source inspection | No address translation or bus abstraction was added. |
| DMA still out of scope | C++ DMA paths can call raw byte writes after device-specific checks | No Rust DMA API | Not in scope | Source inspection | Range/device transfer behavior remains intentionally absent. |
| LL/SC still out of scope | C++ LL/LLD and SC/SCD instruction paths use reservation state plus raw helpers | No Rust LL/SC API | Not in scope | Source inspection | Reservation invalidation exists, but LL/SC instructions do not. |
| Reset/execution still out of scope | C++ reset and step/execution own broader Machine state | Represented reset, raw field decode, and identity classification exist; no fetch, execute, or step API | Not in scope | Source inspection | Raw write-family sealing does not imply execution readiness. |
| Recommended next seam | C++ raw read/write storage helpers are mirrored for the fixed widths | Rust raw read/write storage access is sealed; CPU load/store remains absent | Ready for load/store readiness audit | Source inspection; Rust tests | Recommend `rust_parallel_core_seam_034_load_store_readiness_audit` before any CPU load/store implementation. |

## Seam 032 Audit Changes

- Re-audited C++ `Machine::write_rdram_u64_be`, `RdramOffset`, RDRAM storage,
  `fail_rdram_access`, the sealed raw u8/u16_be/u32_be helpers, CPU doubleword
  store and LL/SC callers, DMA callers, reset clearing, and proof/step-probe
  uses of staged u64 words.
- Added `Machine::write_rdram_u64_be(offset, value)` as a raw RDRAM
  storage-offset u64_be write seam.
- Added private storage-local RDRAM methods for checked u64_be offset validation
  and checked big-endian eight-byte mutation; no public `Rdram::write_u64_be`
  was added.
- Mirrored the C++ order exactly for the earned u64_be write subset:
  bounds-check the full eight-byte span first, reservation invalidation with
  width `8` second, then eight bytes of big-endian storage mutation.
- Reused `RdramAccessError` for invalid u64_be write offsets/ranges as Rust-only
  API safety.
- Added tests for first and last valid u64_be writes, invalid seventh-to-last
  through last-byte, exact-end, and past-end writes, eight-byte mutation,
  neighbor preservation, big-endian byte order, `read_u8` observation, unaligned
  raw storage offsets, reservation overlap invalidation, non-overlap
  preservation, latest staged reservation behavior, invalid-write
  reservation/RDRAM preservation, and unrelated Machine fact preservation.
- Added the raw write-family seal table for u8, u16_be, u32_be, and u64_be.
- Did not add range writes, CPU load/store, LL/SC, memory-map, bus, DMA, reset,
  step, execution, renderer, SDL, host shell, or C++ integration behavior.
- No C++ source files were changed.

## RDRAM Raw Read-Width Family Decision/Parity Seal Table

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| C++ `read_rdram_u8` owner | `src/core/machine.hpp` private `Machine::read_rdram_u8(RdramOffset) const`; `src/core/machine.cpp` definition | `rdram.rs` `Rdram::read_u8(offset)` | Equivalent storage semantics, different ownership shape | Existing RDRAM byte-read tests; source inspection | C++ helper is a private Machine helper; Rust keeps the pure storage read on the RDRAM owner. |
| Rust `read_u8` owner | C++ private helper listed above | `Rdram::read_u8(&self, offset: usize) -> Result<u8, RdramAccessError>` | Equivalent storage semantics, different ownership shape | Existing RDRAM byte-read tests | This is raw storage-offset inspection, not CPU address access. |
| C++ `read_rdram_u16_be` owner | `src/core/machine.hpp` private `Machine::read_rdram_u16_be(RdramOffset) const`; `src/core/machine.cpp` definition | `rdram.rs` `Rdram::read_u16_be(offset)` | Equivalent storage semantics, different ownership shape | `raw_rdram_u16_be_read_observes_big_endian_storage_offsets`; source inspection | C++ composes two bytes after a two-byte bounds check. |
| Rust `read_u16_be` owner | C++ private helper listed above | `Rdram::read_u16_be(&self, offset: usize) -> Result<u16, RdramAccessError>` | Equivalent storage semantics, different ownership shape | Raw read-width tests | Rust uses the same high-byte, low-byte composition and does not mutate reservation state. |
| C++ `read_rdram_u32_be` owner | `src/core/machine.hpp` private `Machine::read_rdram_u32_be(RdramOffset) const`; `src/core/machine.cpp` definition | `rdram.rs` `Rdram::read_u32_be(offset)` | Equivalent storage semantics, different ownership shape | `raw_rdram_u32_be_read_observes_big_endian_storage_offsets`; source inspection | C++ composes four bytes after a four-byte bounds check. |
| Rust `read_u32_be` owner | C++ private helper listed above | `Rdram::read_u32_be(&self, offset: usize) -> Result<u32, RdramAccessError>` | Equivalent storage semantics, different ownership shape | Raw read-width tests | Rust uses the same most-significant to least-significant byte composition. |
| C++ `read_rdram_u64_be` owner | `src/core/machine.hpp` private `Machine::read_rdram_u64_be(RdramOffset) const`; `src/core/machine.cpp` definition | `rdram.rs` `Rdram::read_u64_be(offset)` | Equivalent storage semantics, different ownership shape | `raw_rdram_u64_be_read_observes_big_endian_storage_offsets`; source inspection | C++ composes eight bytes after an eight-byte bounds check. |
| Rust `read_u64_be` owner | C++ private helper listed above | `Rdram::read_u64_be(&self, offset: usize) -> Result<u64, RdramAccessError>` | Equivalent storage semantics, different ownership shape | Raw read-width tests | Rust uses the same most-significant to least-significant byte composition. |
| Rust owner decision: Rdram vs Machine | C++ keeps raw helpers private on `Machine` because C++ RDRAM storage is a Machine field | Rust read methods live on `Rdram`; `Machine::rdram()` provides read-only access | Equivalent storage semantics, different ownership shape | Source/API inspection | Reads are side-effect-free storage inspection, so Rust does not need a Machine-level cross-owner method. Reservation-aware writes remain Machine-owned. |
| Offset/address input | C++ helper input type is `RdramOffset = std::uint32_t` | Rust read methods take `offset: usize` | Equivalent for valid raw storage offsets; API shape differs | Source/API inspection; boundary tests | Inputs are raw RDRAM storage offsets, not CPU addresses. |
| u16 invalid offset/range behavior | C++ rejects `address > rdram_.size() - 2` through `fail_rdram_access(address, 2)` | Rust returns `Err(RdramAccessError { offset, width: 2 })` for `offset > len - 2` | Rust-only API safety, storage boundary equivalent | `raw_rdram_u16_be_read_rejects_invalid_offsets_without_mutation` | Exact end, last byte, and past-end reads fail without mutation. |
| u32 invalid offset/range behavior | C++ rejects `address > rdram_.size() - 4` through `fail_rdram_access(address, 4)` | Rust returns `Err(RdramAccessError { offset, width: 4 })` for `offset > len - 4` | Rust-only API safety, storage boundary equivalent | `raw_rdram_u32_be_read_rejects_invalid_offsets_without_mutation` | Third-to-last, second-to-last, last byte, exact end, and past-end reads fail without mutation. |
| u64 invalid offset/range behavior | C++ rejects `address > rdram_.size() - 8` through `fail_rdram_access(address, 8)` | Rust returns `Err(RdramAccessError { offset, width: 8 })` for `offset > len - 8` | Rust-only API safety, storage boundary equivalent | `raw_rdram_u64_be_read_rejects_invalid_offsets_without_mutation` | Seventh-to-last through last byte, exact end, and past-end reads fail without mutation. |
| Bounds-check behavior | C++ checks each fixed-width span before indexing bytes | Rust `require_u16_be_offset`, `require_u32_be_offset`, and `require_u64_be_offset` run before byte composition | Equivalent behavior, different ownership shape | Raw read-width invalid-offset tests | The helper is private and storage-local; it is not a memory-map or bus helper. |
| Big-endian composition for u16/u32/u64 | C++ shifts high bytes first and ORs lower bytes through the last byte | Rust shifts the same bytes in the same order | Equivalent | Raw read-width big-endian tests | Read composition is explicit and storage-order based. |
| Alignment validation presence or absence | C++ raw helpers have no alignment branch | Rust raw helpers have no alignment branch | Equivalent | Source inspection; unaligned read tests via offset `3` | Odd/unaligned offsets are allowed when the full byte window fits in RDRAM. CPU alignment behavior remains elsewhere and out of scope. |
| Odd/unaligned raw offset behavior | C++ accepts unaligned raw offsets that pass the fixed-width range check | Rust accepts unaligned raw offsets that pass the same storage range check | Equivalent | Raw read-width tests at offset `3` | This proves raw helper behavior only, not CPU load behavior. |
| Reads mutate RDRAM | C++ read helpers are `const` and only read `rdram_` bytes | Rust read methods take `&self` and only read `Rdram::bytes` | Equivalent | `raw_rdram_read_widths_preserve_unrelated_machine_facts`; source inspection | Reads are side-effect-free storage inspection. |
| Reads mutate reservation | C++ read helpers do not reference `cpu_rdram_reservation_` | Rust read methods cannot access `CpuRdramReservation` | Equivalent | Raw read-width invalid-offset and preservation tests | Staged reservation fields remain unchanged after valid and invalid reads. |
| Reads mutate CPU fields | C++ raw read helpers do not assign CPU GPR/scalar fields | Rust `Rdram` read methods cannot access `Cpu` | Equivalent for represented state | `raw_rdram_read_widths_preserve_unrelated_machine_facts` | GPRs, register zero, PC, next PC, HI, and LO remain unchanged. |
| Reads mutate COP0 fields | C++ raw read helpers do not reference COP0 fields | Rust `Rdram` read methods cannot access `Cpu`/`Cop0` | Equivalent for represented state | Preservation test | COP0 construction/access fields remain unchanged. |
| Reads mutate Cartridge facts | C++ raw read helpers do not reference `cartridge_` | Rust `Rdram` read methods cannot access `Cartridge` | Equivalent for represented state | Preservation test | Cartridge bytes and metadata remain unchanged. |
| CPU load behavior | C++ CPU load paths translate CPU addresses and then call raw helpers | No Rust CPU load API | Not in scope | Source inspection | Raw read-width helpers are not CPU loads. |
| Sign extension | C++ sign/zero extension belongs to CPU instruction helpers, not raw RDRAM reads | No Rust sign-extension API | Not in scope | Source inspection | Raw reads return unsigned storage values only. |
| Memory-map | C++ CPU target resolution is separate from raw helpers | No Rust memory-map API | Not in scope | Source inspection | Raw read inputs are storage offsets. |
| Bus | No Rust bus owner exists; C++ raw helpers are not a bus abstraction | No Rust bus API | Not in scope | Source/API inspection | No bus abstraction was introduced. |
| DMA | C++ DMA paths can call raw byte helpers through separate owners | No Rust DMA API | Not in scope | Source inspection | DMA remains intentionally absent. |
| LL/SC | C++ LL/LLD paths call raw reads as part of instruction behavior and then stage reservations | No Rust LL/SC API | Not in scope | Source inspection | Raw reads do not stage or clear reservations. |
| Reset | C++ reset clears RDRAM and many other owners | No Rust reset API | Not in scope | Source inspection | Read-width helpers do not imply reset readiness. |
| Rust-only API safety | C++ throws `std::out_of_range` via `fail_rdram_access` | Rust returns `RdramAccessError` with matching offset/width display text | Rust-only API safety, no emulator truth | RDRAM access error tests | Existing domain error remains appropriate for raw reads and writes; no catchall error was added. |
| Naming/layout changes made for this seam | C++ names helpers `read_rdram_u16_be`, `read_rdram_u32_be`, and `read_rdram_u64_be` | Rust adds `Rdram::read_u16_be`, `Rdram::read_u32_be`, and `Rdram::read_u64_be` in `rdram.rs` | Rust-only repo hygiene, no emulator truth | Source/API inspection | No broad `memory`, `bus`, `util`, or storage-utils module was added. |
| Recommended next seam | C++ load/store paths remain broader than raw storage helpers | Rust raw storage reads and writes are sealed; CPU load/store remains absent | Ready for load/store readiness audit | Source inspection; Rust tests | Recommend `rust_parallel_core_seam_034_load_store_readiness_audit`, because raw storage access is now sealed and the next decision should audit whether CPU load/store can be scoped without memory-map or bus overclaiming. |

## Raw RDRAM Read/Write Storage-Family Status Summary

| Behavior | C++ owner file/function/field | Rust owner file/function/field | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Raw read u8 status | `Machine::read_rdram_u8` | `Rdram::read_u8` | Raw storage access family sealed | RDRAM byte-read tests | Pure raw storage read; no side effects. |
| Raw read u16_be status | `Machine::read_rdram_u16_be` | `Rdram::read_u16_be` | Raw read-width family sealed | Raw read-width tests | Two-byte bounds check, big-endian composition. |
| Raw read u32_be status | `Machine::read_rdram_u32_be` | `Rdram::read_u32_be` | Raw read-width family sealed | Raw read-width tests | Four-byte bounds check, big-endian composition. |
| Raw read u64_be status | `Machine::read_rdram_u64_be` | `Rdram::read_u64_be` | Raw read-width family sealed | Raw read-width tests | Eight-byte bounds check, big-endian composition. |
| Raw write u8 status | `Machine::write_rdram_u8` | `Machine::write_rdram_u8`; private `Rdram::write_u8_at_checked_offset` | Raw storage access family sealed | Raw byte-write tests; seam 027 seal | Bounds-checks one byte, invalidates with width `1`, then mutates one byte. |
| Raw write u16_be status | `Machine::write_rdram_u16_be` | `Machine::write_rdram_u16_be`; private `Rdram::write_u16_be_at_checked_offset` | Raw storage access family sealed | Raw u16_be write tests; seam 029 seal | Bounds-checks two bytes, invalidates with width `2`, then mutates two big-endian bytes. |
| Raw write u32_be status | `Machine::write_rdram_u32_be` | `Machine::write_rdram_u32_be`; private `Rdram::write_u32_be_at_checked_offset` | Raw storage access family sealed | Raw u32_be write tests; seam 031 seal | Bounds-checks four bytes, invalidates with width `4`, then mutates four big-endian bytes. |
| Raw write u64_be status | `Machine::write_rdram_u64_be` | `Machine::write_rdram_u64_be`; private `Rdram::write_u64_be_at_checked_offset` | Raw storage access family sealed | Raw u64_be write tests; seam 032 seal | Bounds-checks eight bytes, invalidates with width `8`, then mutates eight big-endian bytes. |
| Why reads may be Rdram-owned | C++ reads are private Machine methods only because storage is a Machine field | Rust pure reads live on `Rdram` | Equivalent storage semantics, different ownership shape | Source/API inspection | Reads do not touch reservation, CPU, COP0, Cartridge, DMA, reset, or execution state. |
| Why writes are Machine-owned | C++ writes mutate `rdram_` and invalidate `cpu_rdram_reservation_` | Rust writes live on `Machine` and use private `Rdram` mutation helpers | Equivalent behavior, different ownership shape | Raw write tests | Public Rust writes cannot bypass Machine-owned reservation invalidation. |
| No CPU load/store | C++ CPU load/store paths are separate translated-address instruction paths | No Rust CPU load/store API | Not in scope | Source inspection | Raw storage access does not imply load/store readiness. |
| No memory map/bus | C++ target resolution happens before raw helpers | No Rust memory-map or bus API | Not in scope | Source inspection | Raw helpers use storage offsets only. |
| No DMA | C++ DMA owners can use raw helpers in separate paths | No Rust DMA API | Not in scope | Source inspection | No range transfer behavior was added. |
| No LL/SC | C++ LL/SC instruction behavior wraps raw helpers plus reservation semantics | No Rust LL/SC API | Not in scope | Source inspection | Reservation invalidation exists for writes; LL/SC instructions do not. |
| No broad execution | C++ reset and step/execution own broader Machine state | Represented reset, raw field decode, and identity classification exist; no fetch, execute, or step API | Not in scope | Source inspection | Raw storage-family sealing does not imply execution readiness. |
| Recommended next seam | C++ has CPU load/store callers that compose raw helpers with target resolution and instruction semantics | Rust raw storage access family is sealed, but CPU load/store remains absent | Ready for load/store readiness audit | Source inspection; Rust tests | Recommend a read-only load/store readiness audit before any implementation. |

## CPU Direct RDRAM Value Access Decision/Seal

C++ CPU-addressed read helpers in `machine_cpu.cpp` resolve a
`CpuAddress` to a `CpuDataTarget`. C++ CPU-addressed write helpers use the
same resolver. The direct RDRAM target arms are
source-clear: they use the existing CPU-address-to-RDRAM classification,
then call the corresponding raw RDRAM read/write helper. The broader
`CpuDataTarget` resolver, SP/MMIO/device reads, instruction load cases,
alignment checks, sign/zero extension, GPR writeback, LL/LLD reservation
staging, Machine faults, COP0 exception entry, memory map, bus, DMA, reset,
and execution remain out of scope. Store instruction source-register selection,
partial-store lane behavior, SC/SCD success writeback, and alignment faults
also remain out of scope. Rust mirrors only the direct RDRAM value access
subset on `Machine`.

### Direct Value Access Owner Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Equivalent? | Notes |
| --- | --- | --- | --- | --- |
| CpuAddress | `src/core/machine.hpp` `using CpuAddress = std::uint32_t`; read helpers take `CpuAddress` | `cpu/address.rs` `CpuAddress(u32)` | Equivalent behavior, different ownership shape | Rust uses the seam 035 newtype for CPU-visible direct-address input. |
| RdramOffset | `src/core/machine.hpp` `using RdramOffset = std::uint32_t`; `CpuDataTarget::rdram_offset` | `cpu/address.rs` `RdramOffset(u32)` from `classify_direct_rdram_address` | Equivalent behavior, different ownership shape | The offset remains a byte offset into raw RDRAM storage. Existing raw APIs still use `usize` offsets. |
| Direct RDRAM classification | `machine.cpp` `translate_direct_cpu_physical_address`, `translate_cpu_rdram_address`, `translate_cpu_physical_rdram_address`; `machine_cpu.cpp` `require_cpu_data_target` RDRAM target branch | `cpu/address.rs` `classify_direct_rdram_address` | Equivalent for direct RDRAM subset | Rust intentionally does not port SP/MMIO/device target classification. |
| u8 direct RDRAM read value | `machine_cpu.cpp` `read_cpu_memory_u8` `CpuDataTargetKind::kRdram` branch calls `read_rdram_u8` | `machine.rs` `Machine::read_direct_rdram_u8` | Equivalent for direct RDRAM subset | Reads a byte from the translated direct RDRAM offset. This is not `LB`/`LBU`. |
| u16_be direct RDRAM read value | `read_cpu_memory_u16_be` RDRAM branch calls `read_rdram_u16_be` | `Machine::read_direct_rdram_u16_be` | Equivalent for direct RDRAM subset | Returns the big-endian halfword value. This is not `LH`/`LHU` and does not check alignment. |
| u32_be direct RDRAM read value | `read_cpu_memory_u32_be` RDRAM branch calls `read_rdram_u32_be` | `Machine::read_direct_rdram_u32_be` | Equivalent for direct RDRAM subset | Returns the big-endian word value. MMIO word reads remain intentionally absent. |
| u64_be direct RDRAM read value | `read_cpu_memory_u64_be` RDRAM branch calls `read_rdram_u64_be` | `Machine::read_direct_rdram_u64_be` | Equivalent for direct RDRAM subset | Returns the big-endian doubleword value. This is not `LD`/`LLD`. |
| u8 direct RDRAM write value | `machine_cpu.cpp` `write_cpu_memory_u8` `CpuDataTargetKind::kRdram` branch calls `write_rdram_u8` | `machine.rs` `Machine::write_direct_rdram_u8` | Equivalent for direct RDRAM subset | Classifies first, then uses the sealed raw byte write. This is not `SB`. |
| u16_be direct RDRAM write value | `write_cpu_memory_u16_be` RDRAM branch calls `write_rdram_u16_be` | `Machine::write_direct_rdram_u16_be` | Equivalent for direct RDRAM subset | Writes the big-endian halfword through raw storage. This is not `SH` and does not check alignment. |
| u32_be direct RDRAM write value | `write_cpu_memory_u32_be` RDRAM branch calls `write_rdram_u32_be` | `Machine::write_direct_rdram_u32_be` | Equivalent for direct RDRAM subset | MMIO word write branches remain intentionally absent. |
| u64_be direct RDRAM write value | `write_cpu_memory_u64_be` RDRAM branch calls `write_rdram_u64_be` | `Machine::write_direct_rdram_u64_be` | Equivalent for direct RDRAM subset | This is not `SD` or `SCD`. |
| Unsupported/non-direct result | `require_cpu_data_target` may throw `MachineFault` for rejected direct data address; raw direct read subset is absent as a public C++ API | `DirectRdramAccessError` with `CpuAddress` and width | Rust-only API safety, no emulator truth | Rust returns `Result` instead of modeling `MachineFault` or COP0 exception entry. |
| Machine ownership | C++ direct RDRAM value access is private `Machine` CPU memory helper behavior using Machine-owned RDRAM and reservation-aware writes | `machine.rs` public `Machine::read_direct_rdram_*` and `Machine::write_direct_rdram_*` | Equivalent behavior, different ownership shape | Rust owns this seam on `Machine` because it combines CPU-address classification with Machine-owned RDRAM and write-side reservation invalidation. |
| Rdram raw read ownership | C++ raw RDRAM reads are private `Machine` helpers | `rdram.rs` `Rdram::read_u8/u16_be/u32_be/u64_be` | Equivalent storage semantics, different ownership shape | Direct helpers reuse sealed pure raw reads and do not expose a public storage write bypass. |
| Raw write ownership | C++ raw RDRAM writes are private `Machine` helpers and invalidate reservation before mutation | `machine.rs` `Machine::write_rdram_u8/u16_be/u32_be/u64_be`; `rdram.rs` private mutation helpers | Equivalent behavior, different ownership shape | Direct writes reuse sealed raw writes so the order remains classification, raw bounds check, reservation invalidation, storage mutation. |

### Direct Value Access Behavior

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| KSEG0 u8 read | `read_cpu_memory_u8` RDRAM branch after direct target resolution | `Machine::read_direct_rdram_u8` | Equivalent for direct RDRAM subset | `direct_rdram_read_values_support_kseg0_and_kseg1_for_all_widths` | KSEG0 base plus offset maps to raw RDRAM offset and returns one byte. |
| KSEG1 u8 read | Same | Same | Equivalent for direct RDRAM subset | Same | KSEG1 alias maps through the same `0x1fffffff` mask. |
| KSEG0 u16_be read | `read_cpu_memory_u16_be` RDRAM branch | `Machine::read_direct_rdram_u16_be` | Equivalent for direct RDRAM subset | Same | Big-endian value comes from raw `Rdram::read_u16_be`. |
| KSEG1 u16_be read | Same | Same | Equivalent for direct RDRAM subset | Same | No sign extension or GPR writeback is performed. |
| KSEG0 u32_be read | `read_cpu_memory_u32_be` RDRAM branch | `Machine::read_direct_rdram_u32_be` | Equivalent for direct RDRAM subset | Same | MMIO word read branches in C++ are intentionally absent in Rust. |
| KSEG1 u32_be read | Same | Same | Equivalent for direct RDRAM subset | Same | Returns the raw big-endian word value only. |
| KSEG0 u64_be read | `read_cpu_memory_u64_be` RDRAM branch | `Machine::read_direct_rdram_u64_be` | Equivalent for direct RDRAM subset | Same | No `LD` or `LLD` instruction behavior is added. |
| KSEG1 u64_be read | Same | Same | Equivalent for direct RDRAM subset | Same | Returns the raw big-endian doubleword value only. |
| KSEG0/KSEG1 u8 writes | `write_cpu_memory_u8` RDRAM branch after direct target resolution | `Machine::write_direct_rdram_u8` | Equivalent for direct RDRAM subset | `direct_rdram_write_values_support_kseg0_and_kseg1_for_all_widths` | Writes exactly one byte through the raw write helper. |
| KSEG0/KSEG1 u16_be writes | `write_cpu_memory_u16_be` RDRAM branch | `Machine::write_direct_rdram_u16_be` | Equivalent for direct RDRAM subset | Same | Writes high byte then low byte through the raw write helper. |
| KSEG0/KSEG1 u32_be writes | `write_cpu_memory_u32_be` RDRAM branch | `Machine::write_direct_rdram_u32_be` | Equivalent for direct RDRAM subset | Same | Writes four big-endian bytes through the raw write helper. |
| KSEG0/KSEG1 u64_be writes | `write_cpu_memory_u64_be` RDRAM branch | `Machine::write_direct_rdram_u64_be` | Equivalent for direct RDRAM subset | Same | Writes eight big-endian bytes through the raw write helper. |
| Big-endian composition | C++ direct RDRAM branch delegates to `read_rdram_u16_be/u32_be/u64_be` | Rust delegates to `Rdram::read_u16_be/u32_be/u64_be` | Equivalent | Direct read tests and raw read-width tests | Multi-byte reads are high-byte-first composition. |
| Big-endian storage mutation | C++ direct RDRAM branch delegates to `write_rdram_u16_be/u32_be/u64_be` | Rust delegates to `Machine::write_rdram_u16_be/u32_be/u64_be` | Equivalent | Direct write-value tests and raw write tests | Multi-byte writes store high byte first. |
| Last valid direct address per width | C++ target resolution accepts `offset <= kRdramSizeBytes - width` | Rust classifier accepts the same full-width span before raw read | Equivalent | `direct_rdram_read_values_accept_last_valid_address_per_width` | Last valid offsets are `size - 1`, `size - 2`, `size - 4`, and `size - 8`. |
| Last valid direct write address per width | C++ write target resolution uses the same span check before raw write | Rust classifier accepts the same full-width span before raw write | Equivalent | `direct_rdram_write_values_accept_last_valid_address_per_width` | Last valid direct writes succeed for widths 1, 2, 4, and 8. |
| Invalid direct span per width | `translate_cpu_physical_rdram_address` rejects spans past 4 MiB; `require_cpu_data_target` would fault | Rust returns `DirectRdramAccessError` before raw read/write | Equivalent classification fact; Rust-only API safety | Direct read/write invalid-span tests | Exact-end, near-end partial spans, and past-end direct aliases are rejected without state mutation. |
| Non-direct unsupported behavior | `translate_direct_cpu_physical_address` rejects non-KSEG0/KSEG1 top bits | Rust returns `DirectRdramAccessError` through `CpuAddressTarget::Unsupported` | Equivalent classification fact; Rust-only API safety | Direct read/write non-direct tests | Low addresses, reset-vector address, and non-direct top-bit examples are unsupported for this seam. |
| Alignment behavior | C++ read helpers do not check alignment; instruction cases check alignment before calling helpers | Rust direct read helpers do not check alignment | Equivalent for helper subset | `direct_rdram_read_values_accept_unaligned_addresses_without_alignment_check` | Alignment exception behavior remains out of scope. |
| Write alignment behavior | C++ write helpers do not check alignment; instruction store cases check alignment before calling helpers | Rust direct write helpers do not check alignment beyond span classification | Equivalent for helper subset | Source inspection; direct write tests use direct offsets only | Alignment exception behavior remains out of scope. |
| Side-effect-free behavior | C++ direct RDRAM read branch only reads target/raw RDRAM bytes | Rust methods take `&self`, classify, and read RDRAM | Equivalent for direct RDRAM subset | `direct_rdram_read_values_preserve_unrelated_machine_facts` | No RDRAM, reservation, CPU, COP0, or Cartridge facts change. |
| Direct write mutation order | C++ RDRAM write branch calls raw write after target resolution; raw write bounds-checks, invalidates reservation, then mutates storage | Rust direct write classifies first, then calls sealed raw write with the same invalidation/mutation order | Equivalent for direct RDRAM subset | Direct write reservation and invalid-span tests | Classification failure happens before reservation invalidation or storage mutation. |
| Whether RDRAM mutates | C++ read helpers do not mutate `rdram_` | Rust direct read helpers call immutable raw reads | Equivalent | Preservation test | RDRAM bytes seeded by raw writes remain unchanged after reads. |
| Whether direct writes mutate RDRAM | C++ write helpers mutate RDRAM only through raw write helpers | Rust direct writes mutate RDRAM only through sealed raw writes | Equivalent | Direct write tests | Exactly the selected width mutates in big-endian order. |
| Whether reservation mutates | C++ normal read helpers do not touch reservation; LL/LLD instruction paths are separate | Rust direct read helpers do not access `CpuRdramReservation` | Equivalent for helper subset | Preservation test | LL/LLD reservation staging remains out of scope. |
| Whether direct writes mutate reservation | C++ direct RDRAM write branch uses raw writes that invalidate overlapping reservation | Rust direct writes use raw writes that invalidate overlapping reservation | Equivalent for direct RDRAM subset | `direct_rdram_write_values_invalidate_only_overlapping_reservation` | Non-overlapping reservation survives; overlapping reservation clears. |
| Whether CPU state mutates | C++ read helpers return values; instruction load cases separately write GPRs | Rust direct read helpers return values only | Equivalent for helper subset | Preservation test | PC, next PC, HI, LO, GPRs, and register zero are preserved. |
| Whether direct writes mutate CPU state | C++ write helpers do not write GPRs; store instruction cases choose source values separately; SC/SCD write success to GPR separately | Rust direct write helpers accept values and do not mutate CPU | Equivalent for helper subset | `direct_rdram_write_values_preserve_unrelated_machine_facts` | Store instruction semantics remain out of scope. |
| Whether COP0 mutates | C++ helper itself does not enter COP0; `step_cpu_instruction` fault handling is separate | Lower Rust direct value helpers have no exception/COP0 mutation path | Equivalent for successful direct reads; exceptions not in scope for this lower helper | Preservation test; source inspection | Rejected lower direct value helpers return Rust errors. Seam 040 adds a separate preflight API for alignment address-error entry. |
| Whether Cartridge mutates | C++ read helper does not reference `cartridge_` for RDRAM target | Rust direct read helpers do not access Cartridge | Equivalent | Preservation test | Cartridge facts remain unchanged. |
| CPU load instruction out of scope | `LB/LBU/LH/LHU/LW/LWU/LD` cases compute effective address, call helpers, extend values, and write GPRs | No Rust load instruction API | Not in scope | Source inspection | Direct read values are not load instructions. |
| CPU store instruction out of scope | `SB/SH/SW/SD` and partial/SC store cases compute effective address, select source GPR bytes, check alignment, and may write GPR result for SC/SCD | No Rust store instruction API | Not in scope | Source inspection | Direct write values are not store instructions. |
| Sign extension out of scope | C++ sign/zero extension lives in instruction cases | No Rust sign-extension behavior added | Not in scope | Source inspection | u8/u16/u32 direct helpers return unsigned raw values. |
| GPR writeback out of scope | C++ instruction cases call `write_cpu_gpr_*`; read helpers do not | Rust direct read helpers return values and do not mutate CPU | Not in scope | Preservation test | No load writeback readiness is claimed. |
| Exceptions out of scope | C++ `MachineFault` and local COP0 exception entry are owned elsewhere | Rust returns `DirectRdramAccessError`; no COP0 mutation | Not in scope | Source inspection | Error shape is Rust-only API safety, not emulator fault delivery. |
| Memory-map/bus out of scope | C++ `require_cpu_data_target` includes SP/MMIO/device targets but is not a full bus | Rust mirrors only direct RDRAM classification | Not in scope | Source inspection | No target resolver or bus abstraction was added. |
| DMA out of scope | C++ DMA/device paths are separate Machine paths | No Rust DMA API | Not in scope | Source inspection | Direct reads do not copy or stage ranges. |
| LL/SC out of scope | C++ `LL/LLD` read RDRAM then stage reservation in instruction cases | No Rust LL/SC read behavior | Not in scope | Source inspection | Direct u32/u64 reads do not set reservations. |
| Execution out of scope | C++ instruction execution lives in `step_cpu_instruction` and `execute_cpu_instruction` | Rust has raw field decode and identity classification only; no fetch, execute, or step API | Not in scope | Source inspection | Direct read helpers can be tested without executing instructions. |

### Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Direct RDRAM value-access parity seal | Implemented and sealed in this pass | Raw read/write families and direct address classification | Low | Direct RDRAM value access family sealed |
| Direct RDRAM write-value parity seal | Implemented and sealed in this pass | Raw write family and direct address classification | Medium: reservation invalidation order had to remain raw-helper-owned | Direct RDRAM value access family sealed |
| Alignment/exception readiness audit | Completed by seams 037 through 040 for pure alignment, AdEL/AdES selection, narrow entry, and direct RDRAM CPU data preflight | Direct read/write value seams stayed pure; preflight lives in separate Machine methods | Medium | Completed |
| Sign/zero extension value seam | C++ sign/zero extension is source-clear inside instruction cases | Direct read values exist, but GPR writeback/instruction ownership remains absent | Medium-high | Needs future pass after load-instruction boundary is audited |
| CPU load instruction readiness audit | Full load instructions compose effective address, read helper, sign/zero extension, GPR writeback, and exceptions | Direct read values are sealed | High | Needs future pass |
| Device/MMIO target classification audit | C++ target resolver includes SP/MMIO/MI/AI/PI/SI targets and device reads | Device state and DMA/interrupt ownership absent | High | Needs future pass |
| Documentation-only continuation | Direct value access is sealed and next risk is instruction/device/exception coupling | Current seam complete | Low | Not recommended unless implementation is deferred |

### Seam 036 Audit Changes

- Audited C++ `read_cpu_memory_u8`, `read_cpu_memory_u16_be`,
  `read_cpu_memory_u32_be`, `read_cpu_memory_u64_be`, `write_cpu_memory_u8`,
  `write_cpu_memory_u16_be`, `write_cpu_memory_u32_be`,
  `write_cpu_memory_u64_be`, `require_cpu_data_target`, direct KSEG0/KSEG1
  translation, raw RDRAM read/write helpers, load/store instruction cases,
  alignment checks, LL/LLD and SC/SCD reservation coupling, exception/COP0
  paths, and device/MMIO/DMA coupling.
- Added `Machine::read_direct_rdram_u8`, `Machine::read_direct_rdram_u16_be`,
  `Machine::read_direct_rdram_u32_be`, and `Machine::read_direct_rdram_u64_be`.
- Added `Machine::write_direct_rdram_u8`, `Machine::write_direct_rdram_u16_be`,
  `Machine::write_direct_rdram_u32_be`, and `Machine::write_direct_rdram_u64_be`.
- Added `DirectRdramAccessError` for unsupported/non-direct/out-of-range direct
  CPU-addressed RDRAM value access attempts. This is Rust-only API safety; it does not
  model C++ `MachineFault` throwing or local COP0 exception entry.
- Kept the helpers Machine-owned because they combine CPU-visible direct address
  classification with Machine-owned RDRAM, and write values must preserve
  Machine-owned reservation invalidation. Pure raw storage reads remain owned by
  `Rdram`.
- Added tests for KSEG0/KSEG1 direct reads and writes across all widths,
  first/last valid direct spans, invalid direct spans, non-direct unsupported
  addresses, unaligned direct read addresses, big-endian composition,
  write-side reservation invalidation/non-invalidation, and unrelated Machine
  fact preservation.
- Added no CPU load instruction behavior, CPU store behavior, sign extension,
  GPR writeback, alignment exception behavior, exception/COP0 mutation, memory
  map, bus, device/MMIO behavior, DMA, LL/SC behavior, reset, instruction
  execution, host shell, SDL, renderer, or C++ integration behavior.
- No C++ source files were changed.

## Machine Direct RDRAM CPU Data Rejection To Address-Error Entry Seal

C++ source truth maps two data-fault sources through the same local step
conversion: `MachineFaultKind::kUnalignedCpuMemoryAccess` and
`MachineFaultKind::kCpuRdramAddressRejected`. When either carries
`MachineFaultAccessIntent::kDataRead`, `step_cpu_instruction` enters local AdEL
when the narrow entry guard allows it. When either carries
`MachineFaultAccessIntent::kDataWrite`, it enters local AdES. The direct CPU
data helpers call `require_cpu_data_target` with read/write intent; direct or
non-direct target rejection throws `kCpuRdramAddressRejected` before any RDRAM
read/write. C++ also has SP/MMIO/device target branches in that resolver; Rust
does not mirror those branches in this direct RDRAM-only seam.

Rust refines only the Machine-owned direct RDRAM CPU data APIs from seam 040.
Aligned target rejection from the lower-level direct RDRAM value helpers is now
reinterpreted by the CPU-data wrapper as data address-error entry. The
lower-level `Machine::read_direct_rdram_*` and `Machine::write_direct_rdram_*`
methods remain direct value-access APIs and still return
`DirectRdramAccessError` without COP0/PC mutation.

### Rejection Owner Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Equivalent? | Notes |
| --- | --- | --- | --- | --- |
| Rejected data target fault kind | `src/core/machine_cpu.cpp` `fail_cpu_data_address_rejected`; `src/core/machine.hpp` `MachineFaultKind::kCpuRdramAddressRejected` | `machine.rs` lower direct value helpers return `DirectRdramAccessError`; CPU-data wrappers consume it | Equivalent behavior, different ownership shape | Rust does not clone C++ `MachineFault`; it uses direct errors below the CPU-data wrapper. |
| Read/write intent | `MachineFaultAccessIntent::{kDataRead,kDataWrite}` | `CpuDataAccessKind::{Read,Write}` | Equivalent | The same access kind selects AdEL or AdES. |
| Read rejection exception selection | `step_cpu_instruction` maps data read rejection to `kCop0ExceptionCodeAddressErrorLoad = 4` | `select_cpu_data_address_error_for_access(Read, address, width)` | Equivalent | The rejected CPU address is preserved as BadVAddr input. |
| Write rejection exception selection | `step_cpu_instruction` maps data write rejection to `kCop0ExceptionCodeAddressErrorStore = 5` | `select_cpu_data_address_error_for_access(Write, address, width)` | Equivalent | The rejected CPU address is preserved as BadVAddr input. |
| Address-error entry | `enter_local_address_error_exception` when the local guard allows entry | `Cpu::enter_data_address_error_exception` through `Machine::enter_direct_rdram_cpu_data_rejection_address_error` | Equivalent for the sealed narrow entry path | If entry is blocked, Rust returns `AddressErrorEntryBlocked` and does not mutate RDRAM/reservation. |
| Lower-level direct value APIs | `require_cpu_rdram_address`/raw direct RDRAM helpers can reject direct RDRAM aliases before raw access | `Machine::read_direct_rdram_*`, `Machine::write_direct_rdram_*` | Equivalent for direct value-access API safety | These APIs remain direct-access APIs. They do not enter AdEL/AdES. |
| Device/MMIO target resolver | `require_cpu_data_target` can route SP memory, SP/MMIO, MI, AI, PI, and SI targets | No Rust target resolver | Not in scope | Rust does not add a memory map, bus, device routing, MMIO, or DMA. |

### Rejection Behavior

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Alignment still runs before target classification | Instruction cases call unaligned fault helpers before memory helper calls | `preflight_direct_rdram_cpu_data_access` runs before direct value helpers | Equivalent | Unaligned read/write tests | Unaligned target-rejected addresses take the alignment path first. |
| Aligned KSEG0/KSEG1 direct RDRAM success still does not enter exceptions | `require_cpu_data_target` returns RDRAM target, then raw helper succeeds | `read_direct_rdram_cpu_data_*` / `write_direct_rdram_cpu_data_*` return `Ok` | Equivalent for direct RDRAM subset | Success tests | COP0/PC remain unchanged on success. |
| Aligned read target rejection selects AdEL | `kCpuRdramAddressRejected` with `kDataRead` maps to AdEL/code 4 | CPU-data wrapper converts `DirectRdramAccessError` into `AddressErrorEntered(AddressErrorLoad)` | Equivalent for direct RDRAM CPU-data subset | `direct_rdram_cpu_data_aligned_target_rejection_enters_adel_or_ades`; boundary tests | Includes non-direct and exact-end direct-RDRAM subset examples. |
| Aligned write target rejection selects AdES | `kCpuRdramAddressRejected` with `kDataWrite` maps to AdES/code 5 | CPU-data wrapper converts `DirectRdramAccessError` into `AddressErrorEntered(AddressErrorStore)` | Equivalent for direct RDRAM CPU-data subset | Same tests | No RDRAM write and no reservation invalidation occur. |
| Rejected read BadVAddr | `fault.cpu_address()` is passed to address-error entry | `CpuDataAddressError::bad_vaddr()` is the rejected `CpuAddress` | Equivalent | Same tests | BadVAddr equals the rejected CPU address. |
| Rejected write BadVAddr | Same | Same | Equivalent | Same tests | BadVAddr equals the rejected CPU address. |
| Rejected read RDRAM preservation | Rejection occurs before raw read/write | Rust enters address-error path before any RDRAM access | Equivalent | Same tests | RDRAM bytes are unchanged. |
| Rejected write RDRAM preservation | Rejection occurs before raw write | Rust enters address-error path before any direct/raw write | Equivalent | Same tests | Seeded RDRAM bytes are unchanged. |
| Rejected write reservation preservation | C++ rejection occurs before raw write invalidation | Rust does not call direct/raw write after target rejection | Equivalent | Same tests | Existing reservation remains valid. |
| Byte target rejection proves rejection path only | C++ byte access has no alignment rejection | Rust `CpuDataWidth::Byte` has no alignment rejection; rejected byte read enters AdEL | Equivalent | Boundary tests | Byte failures are target rejection, not alignment faults. |
| Lower direct read rejection remains direct error | Lower helper rejection is below CPU-data interpretation | `read_direct_rdram_*` returns `DirectRdramAccessError` and does not mutate COP0/PC | Equivalent for lower helper API | `lower_level_direct_rdram_value_apis_keep_direct_rejection_errors` | This preserves seam 036 direct value-access semantics. |
| Lower direct write rejection remains direct error | Same | `write_direct_rdram_*` returns `DirectRdramAccessError` and does not mutate COP0/PC/RDRAM/reservation | Equivalent for lower helper API | Same test | The reinterpretation exists only in CPU-data APIs. |
| Device/MMIO-like direct addresses | C++ may route some physical targets to SP/MMIO/MI/AI/PI/SI or unsupported device handlers | Rust direct RDRAM CPU-data APIs do not route devices | Not in scope | Source inspection | This seam is not a CPU data target resolver, memory map, or bus. |
| CPU load/store instruction behavior | C++ instruction cases add effective-address calculation, extension, GPR writeback, and PC commit/rollback | No Rust instruction API | Not in scope | Source inspection | Rejection entry does not make these methods load/store instructions. |

### Rejection Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Direct RDRAM CPU-data rejection to AdEL/AdES | Implemented and sealed here | Direct CPU data preflight, AdEL/AdES selection, narrow entry | Medium | Completed |
| Direct RDRAM CPU-data parity seal follow-up | Useful only if this seam needs separate audit after implementation | Current seam | Low | Not required unless requested |
| CPU load sign/zero extension value seam | Direct read values and error preflight now exist, but load result shaping is still absent | Current seam | Medium | Recommended |
| CPU load/store instruction readiness audit | Full instruction behavior still needs effective-address calculation, GPR source/destination, sign/zero extension, writeback, and step commit/rollback | Current seam plus extension seam | High | Needs future pass |
| Device/MMIO target classification audit | C++ resolver has non-RDRAM targets that this seam intentionally avoids | Device ownership and DMA/interrupt audits | High | Needs future pass |

### Seam 041 Audit Changes

- Added `select_cpu_data_address_error_for_access` in `cpu/address.rs` so
  source-clear non-alignment data address errors can preserve address, width,
  and access kind without pretending they came from `CpuDataAlignmentError`.
- Updated `Machine::read_direct_rdram_cpu_data_*` and
  `Machine::write_direct_rdram_cpu_data_*` so aligned direct target rejection
  enters the narrow data address-error path.
- Preserved lower-level direct value-access APIs:
  `Machine::read_direct_rdram_*` and `Machine::write_direct_rdram_*` still
  return `DirectRdramAccessError` and do not mutate COP0/PC.
- Added tests for aligned target rejection to AdEL/AdES, rejected BadVAddr,
  rejected RDRAM/reservation preservation, byte target-rejection behavior, and
  lower-level direct API preservation.
- Added no CPU load/store instruction behavior, GPR writeback, sign/zero
  extension, instruction decode/execution, generic exceptions, memory map, bus,
  devices/MMIO, DMA, LL/SC instruction behavior, host shell, SDL, renderer, or
  C++ integration behavior.
- No C++ source files were changed.

## Machine Direct RDRAM CPU Data Access Preflight / Value Access Seal

C++ source truth is source-clear for a narrow composition under instruction
semantics. Aligned scalar load/store instruction cases first check natural
alignment in `execute_cpu_instruction`, then call the private
`read_cpu_memory_*` or `write_cpu_memory_*` helpers. Those helpers resolve a
`CpuDataTarget`; for the direct RDRAM target they delegate to the existing raw
RDRAM read/write helpers. The write helpers then preserve the sealed raw write
ordering: target/bounds classification first, reservation invalidation second,
and storage mutation third. Seam 041 refines the aligned target-rejection path
for these CPU-data wrappers so direct target rejection now enters the narrow
AdEL/AdES address-error path.

Rust mirrors only that direct RDRAM CPU data value-access subset on `Machine`.
It does not compute effective addresses from GPRs, select source/destination
registers, sign-extend or zero-extend load results, perform instruction
writeback, execute instructions, dispatch SP/MMIO/device targets, or implement a
bus or memory map.

### Direct CPU Data Access Owner Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Equivalent? | Notes |
| --- | --- | --- | --- | --- |
| Machine ownership | `Machine::read_cpu_memory_*`, `Machine::write_cpu_memory_*`, `step_cpu_instruction` fault conversion | `machine.rs` `Machine::read_direct_rdram_cpu_data_*`, `Machine::write_direct_rdram_cpu_data_*` | Equivalent behavior, different ownership shape | Rust owner is `Machine` because alignment faults may mutate CPU/COP0/control-flow and aligned writes may mutate RDRAM/reservation. |
| CPU address input | Instruction cases compute `CpuAddress effective_address`; helpers take `CpuAddress` | Methods take `CpuAddress` | Equivalent | No GPR/effective-address calculation is added. |
| Width input | Instruction cases choose byte/halfword/word/doubleword widths | Method names map to `CpuDataWidth::{Byte,Halfword,Word,Doubleword}` | Equivalent for scalar widths | Partial lane load/store and LL/SC instruction widths remain separate. |
| Access kind | `MachineFaultAccessIntent::{kDataRead,kDataWrite}` | `CpuDataAccessKind::{Read,Write}` | Equivalent | Used only for alignment-fault AdEL/AdES selection. |
| Alignment preflight | Instruction cases check `(effective_address & 1/3/7) != 0` before helper calls | `check_cpu_data_alignment` before direct classification | Equivalent | Byte has no alignment rejection. |
| Address-error selection | `step_cpu_instruction` maps read to AdEL/code 4 and write to AdES/code 5 | `select_cpu_data_address_error`; `select_cpu_data_address_error_for_access` | Equivalent | Selection preserves CPU address, width, and access kind for alignment faults and direct target rejections. |
| Address-error entry | `enter_local_address_error_exception` from the step fault handler when local guard allows | `Cpu::enter_data_address_error_exception` called by Machine preflight or target-rejection path | Equivalent for sealed data address errors | If the entry guard rejects, Rust returns `AddressErrorEntryBlocked` and mutates nothing. |
| Direct RDRAM classification | `require_cpu_data_target` direct RDRAM branch after direct translation | `classify_direct_rdram_address` through existing direct value methods | Equivalent for direct RDRAM subset | SP/MMIO/device targets are intentionally absent. |
| Direct RDRAM read value | C++ RDRAM branch calls raw `read_rdram_*` | `read_direct_rdram_cpu_data_*` calls sealed direct read value helpers | Equivalent for direct RDRAM subset | Returns raw unsigned values; sign/zero extension is absent. |
| Direct RDRAM write value | C++ RDRAM branch calls raw `write_rdram_*` | `write_direct_rdram_cpu_data_*` calls sealed direct write value helpers | Equivalent for direct RDRAM subset | Write-side reservation invalidation remains owned by raw write helpers. |
| Error shape | C++ throws `MachineFault` or unsupported errors | `MachineDirectRdramCpuDataAccessError`; lower direct APIs use `DirectRdramAccessError` | Rust-only API safety, no emulator truth | CPU-data target rejection enters address-error variants. Lower-level direct APIs still report direct errors. |

### Direct CPU Data Access Behavior

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Aligned KSEG0 u8 read | `read_cpu_memory_u8` RDRAM branch | `read_direct_rdram_cpu_data_u8` | Equivalent | `direct_rdram_cpu_data_reads_support_kseg0_kseg1_and_big_endian_values` | No alignment rejection for byte access. |
| Aligned KSEG1 u8 read | Same | Same | Equivalent | Same | KSEG1 alias maps through sealed direct classification. |
| Aligned KSEG0/KSEG1 u16/u32/u64 reads | `read_cpu_memory_u16_be/u32_be/u64_be` RDRAM branch after instruction alignment checks | `read_direct_rdram_cpu_data_u16_be/u32_be/u64_be` | Equivalent for direct RDRAM subset | Same | Big-endian value composition comes from sealed raw reads. |
| Aligned KSEG0/KSEG1 u8/u16/u32/u64 writes | `write_cpu_memory_u8/u16_be/u32_be/u64_be` RDRAM branch after instruction alignment checks | `write_direct_rdram_cpu_data_u8/u16_be/u32_be/u64_be` | Equivalent for direct RDRAM subset | `direct_rdram_cpu_data_writes_support_kseg0_kseg1_and_big_endian_values` | Big-endian storage mutation comes from sealed raw writes. |
| Alignment happens before classification | Instruction cases reject unaligned multi-byte accesses before helper call | Machine preflight calls `check_cpu_data_alignment` before direct value helpers | Equivalent | Unaligned read/write tests | Alignment fault performs no RDRAM access and no reservation invalidation. |
| Unaligned read selects AdEL | `fail_unaligned_*_memory_access(..., kDataRead)` then step maps read to code 4 | `MachineDirectRdramCpuDataAccessError::AddressErrorEntered` with `AddressErrorLoad` | Equivalent | `direct_rdram_cpu_data_unaligned_reads_enter_adel_without_storage_or_gpr_mutation` | BadVAddr is the faulting `CpuAddress`; EPC/EXL/PC vectoring follows seam 039. |
| Unaligned write selects AdES | `fail_unaligned_*_memory_access(..., kDataWrite)` then step maps write to code 5 | `AddressErrorEntered` with `AddressErrorStore` | Equivalent | `direct_rdram_cpu_data_unaligned_writes_enter_ades_without_storage_or_reservation_mutation` | No RDRAM byte changes and no reservation invalidation happen on the alignment fault. |
| Byte access never alignment-faults | C++ byte load/store cases have no natural-alignment rejection | `CpuDataWidth::Byte` has mask `0` | Equivalent | `direct_rdram_cpu_data_byte_access_never_enters_alignment_exception` | Byte direct CPU data access can use unaligned addresses without COP0 mutation. |
| Unsupported/non-direct aligned address | `require_cpu_data_target` rejects with `kCpuRdramAddressRejected` and data intent when no supported target exists; step maps data read/write rejection to AdEL/AdES | Rust CPU-data wrappers convert direct target rejection to address-error entry | Equivalent for direct RDRAM CPU-data subset | `direct_rdram_cpu_data_aligned_target_rejection_enters_adel_or_ades` | This still does not model SP/MMIO/device target routing. |
| End-of-RDRAM direct bounds | C++ target translation checks the full requested span; rejected data spans can enter AdEL/AdES through step conversion | Rust uses sealed direct classifier after alignment passes; CPU-data rejection enters address-error entry | Equivalent for direct RDRAM CPU-data subset | `direct_rdram_cpu_data_boundary_acceptance_uses_direct_width_rules` | Last valid aligned widths pass; exact-end aligned spans enter AdEL/AdES in CPU-data APIs. Lower direct APIs still return direct errors. |
| Reservation invalidation on aligned writes | C++ direct write delegates to raw RDRAM write, which invalidates overlapping reservation | Rust direct CPU data write delegates to sealed direct/raw write | Equivalent | `direct_rdram_cpu_data_writes_preserve_raw_reservation_invalidation_order` | Non-overlapping reservation survives; overlapping reservation clears. |
| RDRAM preservation on alignment fault | C++ unaligned fault occurs before helper call | Rust enters address-error path before direct value helper | Equivalent | Unaligned write/read tests | Seeded RDRAM bytes remain unchanged. |
| GPR preservation | C++ helper composition does not include instruction writeback/source selection | Rust methods do not access GPRs | Equivalent for this seam | Unaligned and success tests | CPU load/store instruction behavior remains absent. |
| PC/next PC/COP0 mutation on success | Successful helper calls do not enter exceptions | Successful Rust direct CPU data access does not touch COP0/control flow | Equivalent | Success tests | PC/next PC/COP0 mutation happens only on alignment address-error entry. |
| CPU load/store instruction behavior | C++ instruction cases wrap these helpers with effective-address, register, extension, and writeback behavior | No Rust instruction API | Not in scope | Source inspection | This seam is value access with preflight, not load/store instructions. |
| Memory map/bus/device/DMA | C++ target resolver has SP/MMIO/device branches | No Rust target resolver, bus, or devices | Not in scope | Source inspection | Only direct KSEG0/KSEG1 RDRAM is represented. |
| LL/SC instruction behavior | C++ LL/LLD and SC/SCD own reservation staging/result writeback | No Rust LL/SC instruction behavior | Not in scope | Source inspection | Write reservation invalidation is raw write behavior only. |

### Direct CPU Data Access Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Direct RDRAM CPU data preflight parity seal | Implemented and sealed in seam 040 | Direct value access, alignment, AdEL/AdES selection, narrow entry mutation | Medium | Completed |
| Aligned direct RDRAM data target rejection to address-error entry audit | Implemented and sealed in seam 041 | Current direct CPU data access seam | Medium | Completed |
| Sign/zero extension value seam | Raw/direct read values exist but instruction load result shaping is still absent | Current seam plus load-readiness audit | Medium-high | Needs future pass |
| CPU load/store instruction readiness audit | Full instructions need effective address, register source/destination, extension, writeback, and exception policy | Current seam plus target rejection audit | High | Needs future pass |
| Device/MMIO target classification audit | C++ target resolver includes SP/MMIO/MI/AI/PI/SI branches | Device state ownership | High | Needs future pass |

### Seam 040 Audit Changes

- Added `Machine::read_direct_rdram_cpu_data_u8`,
  `Machine::read_direct_rdram_cpu_data_u16_be`,
  `Machine::read_direct_rdram_cpu_data_u32_be`, and
  `Machine::read_direct_rdram_cpu_data_u64_be`.
- Added `Machine::write_direct_rdram_cpu_data_u8`,
  `Machine::write_direct_rdram_cpu_data_u16_be`,
  `Machine::write_direct_rdram_cpu_data_u32_be`, and
  `Machine::write_direct_rdram_cpu_data_u64_be`.
- Added `MachineDirectRdramCpuDataAccessError` to distinguish entered
  address-error entry and blocked address-error entry for the CPU-data wrapper.
  Seam 041 refines aligned direct RDRAM rejection so it also enters this narrow
  address-error path. The error shape remains Rust-only API safety and not a
  C++ `MachineFault` clone.
- Preserved successful write order: alignment preflight, direct classification,
  raw write bounds check, reservation invalidation, storage mutation.
- Added tests for KSEG0/KSEG1 aligned direct CPU data reads/writes across all
  widths, big-endian values, boundary behavior, AdEL/AdES entry on unaligned
  multi-byte access, byte no-alignment behavior, later-refined direct rejection
  behavior, RDRAM preservation on alignment faults, and reservation
  invalidation preservation on aligned writes.
- Added no CPU load/store instruction behavior, GPR writeback, sign/zero
  extension, instruction decode/execution, generic exceptions, memory map, bus,
  devices/MMIO, DMA, LL/SC instruction behavior, host shell, SDL, renderer, or
  C++ integration behavior.
- No C++ source files were changed.

## CPU Address-Error Exception / COP0 Mutation Readiness Seal

C++ source truth supports a narrow local data address-error exception entry once
the sealed `CpuDataAddressError` exists and the CPU has explicit `pc`/`next_pc`
state. `step_cpu_instruction` only enters the local address-error exception when
Status.EXL is clear and the current PC cadence is either ordinary
`next_pc == pc + 4` or the currently earned narrow delay-slot cadence
`next_pc != pc + 4`, aligned `pc`, and `pc >= 4`. The entry itself writes
BadVAddr from the fault address, writes the selected AdEL/AdES exception code,
writes EPC from the faulting PC or `pc - 4` for the narrow delay-slot case, sets
the branch-delay flag, sets Status.EXL, and vectors PC/next PC to the current
local exception vector. Rust mirrors only this data address-error entry path on
`Cpu`. It does not implement CPU load/store instructions, instruction
execution, generic exception handling, interrupts, ERET, memory-map behavior,
bus behavior, devices/MMIO, DMA, TLB/MMU, or LL/SC instruction behavior.

### Address-Error Entry Owner Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Equivalent? | Notes |
| --- | --- | --- | --- | --- |
| Entry input | `MachineFault` with `kDataRead`/`kDataWrite` converted to AdEL/AdES selection | `CpuDataAddressError` | Equivalent for sealed data-address-error input | Rust does not accept generic exception inputs. |
| Entry guard | `local_synchronous_exception_entry_allowed`; `local_delay_slot_synchronous_exception_entry_allowed` | `Cpu::enter_data_address_error_exception` internal guard | Equivalent | Ordinary entry requires `next_pc == pc + 4` and EXL clear. Narrow delay-slot entry requires `next_pc != pc + 4`, EXL clear, aligned `pc`, and `pc >= 4`. |
| Entry blocked result | C++ rethrows local `MachineFault` when the guard does not allow local entry | `CpuAddressErrorExceptionEntryError` with current `pc`, `next_pc`, and `status` | Equivalent behavior, different ownership shape | Rust returns `Result` and performs no mutation on blocked entry. |
| BadVAddr mutation | `enter_local_address_error_exception` `cop0_bad_vaddr_ = bad_vaddr` | `Cpu::enter_data_address_error_exception` writes `cop0.bad_vaddr` from `CpuDataAddressError::bad_vaddr()` | Equivalent | Source value is the original faulting CPU address. |
| Cause exception code mutation | C++ writes `cop0_exception_code_ = exception_code` after AdEL/AdES selection | Rust writes `cop0.exception_code = address_error.cause_exception_code()` | Equivalent | Rust stores the current local exception-code field, not a full architectural Cause register. |
| Branch-delay flag mutation | C++ writes `cop0_exception_branch_delay_ = branch_delay` | Rust writes `cop0.exception_branch_delay` from the same guard-derived branch-delay fact | Equivalent | This is narrow entry metadata only, not branch execution behavior. |
| EPC mutation | C++ writes `faulting_pc` or `faulting_pc - 4` when `branch_delay` is true | Rust writes current `pc` or `pc.wrapping_sub(4)` for the allowed delay-slot cadence | Equivalent | Delay-slot entry requires `pc >= 4`, matching C++ guard. |
| Status.EXL mutation | C++ `cop0_status_ |= kCop0StatusExl` | Rust `cop0.status |= 0x00000002` | Equivalent | EXL is set; broad Status behavior remains absent. |
| PC/next PC vectoring | C++ uses `kLocalInterruptVectorPc = 0x80000180`, `kLocalInterruptVectorNextPc = 0x80000184` | Rust private `LOCAL_EXCEPTION_VECTOR_PC = 0x80000180`, `LOCAL_EXCEPTION_VECTOR_NEXT_PC = 0x80000184` | Equivalent behavior, different name | Rust names the local exception vector by current use and does not claim interrupt readiness. |
| Machine/RDRAM/reservation mutation | C++ entry helper mutates only local CPU/COP0/control-flow fields | Rust method is on `Cpu` only | Equivalent for no RDRAM involvement | No Machine, RDRAM, or reservation state is touched. |

### Address-Error Entry Behavior

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| AdEL/read entry writes BadVAddr | `step_cpu_instruction` AdEL arm passes `fault.cpu_address()` as `bad_vaddr` | `enter_data_address_error_exception` writes `cop0.bad_vaddr` | Equivalent | `data_address_error_load_enters_local_exception_without_gpr_mutation` | Fault address is preserved exactly. |
| AdES/write entry writes BadVAddr | Same entry helper with AdES code | Same | Equivalent | `data_address_error_store_enters_local_exception_with_store_code` | Fault address is preserved exactly. |
| AdEL code | `kCop0ExceptionCodeAddressErrorLoad = 4` | `CpuDataAddressError::cause_exception_code() == 4` | Equivalent | Same | Rust writes `cop0_exception_code()` value `4`. |
| AdES code | `kCop0ExceptionCodeAddressErrorStore = 5` | `CpuDataAddressError::cause_exception_code() == 5` | Equivalent | Same | Rust writes `cop0_exception_code()` value `5`. |
| Status.EXL set | `cop0_status_ |= kCop0StatusExl` | `cop0.status |= 0x00000002` | Equivalent | Entry tests | Other Status bits are preserved. |
| Non-branch EPC | `cop0_epc_ = faulting_pc` when `branch_delay == false` | `cop0.epc = pc` for ordinary cadence | Equivalent | Read/write entry tests | Ordinary cadence requires `next_pc == pc + 4`. |
| Narrow delay-slot EPC and flag | `cop0_epc_ = faulting_pc - 4`; branch-delay flag true | Rust uses `pc - 4` and branch-delay flag true for allowed delay-slot cadence | Equivalent | `data_address_error_delay_slot_entry_sets_branch_delay_epc` | This does not add branch instruction execution. |
| Vector PC/next PC | C++ sets `cpu_pc_ = 0x80000180`, `cpu_next_pc_ = 0x80000184` | Rust sets `pc`/`next_pc` to the same local vector pair | Equivalent | Entry tests | No BEV-dependent vector selection exists in current C++ source. |
| EXL already set | Guard rejects local entry when `cop0_status_ & EXL != 0`; fault remains unconverted | Rust returns `CpuAddressErrorExceptionEntryError` and preserves state | Equivalent behavior, different error shape | `data_address_error_entry_blocks_when_exl_is_already_set` | Rust returns `Result` instead of throwing. |
| Unsupported delay-slot context | Guard rejects when non-sequential context has unaligned `pc` or `pc < 4` | Rust returns error and preserves state | Equivalent behavior, different error shape | `data_address_error_entry_blocks_unsupported_delay_slot_context_without_mutation` | This keeps broad delay-slot fidelity out of scope. |
| GPR preservation | C++ entry helper does not write GPRs | Rust method does not write GPRs | Equivalent | Entry tests | No load/store writeback was added. |
| RDRAM/reservation preservation | C++ entry helper does not touch RDRAM/reservation | Rust method is CPU-only | Equivalent | Source inspection; ownership shape | No direct RDRAM value access is called. |
| Generic exception handling | C++ has other local exception paths | Rust adds only narrow data address-error, selected instruction-fetch address-error, and signed-overflow entry primitives | Not in scope | Source inspection | Interrupt, ERET, TLB/MMU, broad exception dispatch, and wiring from execute outcomes to exception entry remain absent. |

### Address-Error Entry Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Data address-error entry parity seal | Implemented and sealed in this pass | Pure data alignment and AdEL/AdES selection | Low | CPU data address-error entry sealed |
| Direct RDRAM value access with alignment and address-error entry preflight | The pure pieces now compose, but API shape must not become CPU load/store instruction behavior | Direct value access, alignment, selection, entry mutation | Medium | Recommended |
| CPU load/store instruction readiness audit | Full instructions still need effective-address calculation, target access, sign/zero extension, GPR writeback, and instruction PC commit/rollback | Current seam plus value access | High | Needs future pass |
| Fetch/control-transfer address-error audit | C++ also has instruction fetch and control-transfer address-error entries | Current data entry seam | Medium | Needs future pass |
| Generic exception framework | Current C++ has multiple local exception sources, but generic handling would overclaim | More exception classes and instruction boundaries | High | Blocked |

### Seam 039 Audit Changes

- Audited C++ local address-error entry guards, BadVAddr mutation, exception
  code mutation, branch-delay flag mutation, EPC rule, Status.EXL mutation,
  vector PC/next PC, EXL-blocked behavior, and unsupported delay-slot context.
- Added `Cpu::enter_data_address_error_exception` in `cpu/cop0.rs`.
- Added `CpuAddressErrorExceptionEntryError` in `cpu/address.rs`.
- Re-exported the entry error through `cpu.rs` and `lib.rs`.
- Added synthetic Rust tests for AdEL/AdES entry, BadVAddr, exception code,
  Status.EXL, non-branch EPC, delay-slot EPC/branch-delay flag, vector PC/next
  PC, EXL blocked entry, unsupported delay-slot blocked entry, GPR preservation,
  and no Machine/RDRAM involvement.
- Added no CPU load/store instruction behavior, sign extension, zero extension,
  GPR writeback, fetch/decode/execute/step, branch instruction behavior,
  interrupt behavior, ERET, TLB/MMU, memory map, bus, device/MMIO behavior,
  DMA, LL/SC instruction behavior, graphics, SDL/window/runtime, host shell, or
  C++ integration behavior.
- No C++ source files were changed.

## CPU Data Address-Error Exception Selection Readiness/Seal

C++ keeps a source-clear selection fact inside `step_cpu_instruction`: a
`MachineFault` carrying `MachineFaultAccessIntent::kDataRead` selects local
Address Error Load (`AdEL`, exception code `4`), while
`MachineFaultAccessIntent::kDataWrite` selects local Address Error Store
(`AdES`, exception code `5`). That selection consumes the faulting CPU address
as the `bad_vaddr` argument to `enter_local_address_error_exception`. Rust's
selection value from `CpuDataAlignmentError` remains pure; seam 039 consumes
that value in the narrow CPU-owned entry mutation path. The selection function
itself still does not enter exceptions, mutate COP0, mutate PC/next PC, execute
instructions, or touch Machine state.

### Address-Error Selection Owner Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Equivalent? | Notes |
| --- | --- | --- | --- | --- |
| Alignment fault input | `MachineFaultKind::kUnalignedCpuMemoryAccess`; `MachineFault` address, access size, and access intent | `cpu/address.rs` `CpuDataAlignmentError` | Equivalent for pure data-alignment input | Rust input is produced by the sealed pure alignment contract. |
| Data read/write intent | `MachineFaultAccessIntent::{kDataRead,kDataWrite}` | `CpuDataAccessKind::{Read,Write}` | Equivalent | Read/write distinction is preserved because C++ uses it to choose AdEL versus AdES. |
| Address Error Load selection | `step_cpu_instruction` maps `kDataRead` to `kCop0ExceptionCodeAddressErrorLoad = 4` | `CpuAddressErrorKind::AddressErrorLoad`; `cause_exception_code() == 4`; `short_name() == "AdEL"` | Equivalent for pure selection | No Cause register mutation is performed. |
| Address Error Store selection | `step_cpu_instruction` maps `kDataWrite` to `kCop0ExceptionCodeAddressErrorStore = 5` | `CpuAddressErrorKind::AddressErrorStore`; `cause_exception_code() == 5`; `short_name() == "AdES"` | Equivalent for pure selection | No Cause register mutation is performed. |
| Faulting address as BadVAddr source | `fault.cpu_address()` is passed as `bad_vaddr` to `enter_local_address_error_exception` | `CpuDataAddressError::address()` and `bad_vaddr()` return the original `CpuAddress`; seam 039 entry writes it to COP0 | Equivalent source value | The pure selection value preserves the future BadVAddr input; `Cpu::enter_data_address_error_exception` owns mutation. |
| Width/access-kind diagnostic preservation | `MachineFault::access_size()` and `MachineFault::access_intent()` | `CpuDataAddressError::{width,access_kind}` | Equivalent for proof/diagnostic truth | Rust keeps width and access kind with the selection result. |
| Exception entry mutation | `enter_local_address_error_exception` mutates `cop0_bad_vaddr_`, exception code, EPC, branch-delay flag, EXL, PC, and next PC | `Cpu::enter_data_address_error_exception` in seam 039 | Equivalent for sealed data address-error entry | Selection remains pure; entry mutation is sealed separately below. |

### Address-Error Selection Behavior

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Read alignment fault selects AdEL | `step_cpu_instruction` `fault.access_intent() == kDataRead` arm | `select_cpu_data_address_error` maps `Read` to `AddressErrorLoad` | Equivalent | `read_alignment_fault_selects_address_error_load` | The selected code is `4`. |
| Write alignment fault selects AdES | `step_cpu_instruction` `fault.access_intent() == kDataWrite` arm | `select_cpu_data_address_error` maps `Write` to `AddressErrorStore` | Equivalent | `write_alignment_fault_selects_address_error_store` | The selected code is `5`. |
| Halfword fault payload preservation | `fail_unaligned_halfword_memory_access` carries address, width `2`, and access intent | `CpuDataAddressError` from halfword alignment error | Equivalent | `halfword_address_error_preserves_fault_payload` | Address is also exposed as future BadVAddr input. |
| Word fault payload preservation | `fail_unaligned_word_memory_access` carries address, width `4`, and access intent | `CpuDataAddressError` from word alignment error | Equivalent | `word_address_error_preserves_every_rejected_low_bit_payload` | Low bits `01`, `10`, and `11` are covered. |
| Doubleword fault payload preservation | `fail_unaligned_doubleword_memory_access` carries address, width `8`, and access intent | `CpuDataAddressError` from doubleword alignment error | Equivalent | `doubleword_address_error_preserves_every_rejected_low_bit_payload` | Low bits `001` through `111` are covered. |
| Byte alignment path | C++ byte loads/stores have no natural-alignment fault helper call | `check_cpu_data_alignment(_, _, Byte)` returns `Ok(())` | Equivalent | `byte_alignment_has_no_address_error_selection_source` | There is no `CpuDataAlignmentError` to convert for byte access. |
| Machine mutation | Selection happens before `enter_local_address_error_exception`; mutation belongs to the entry helper | Conversion consumes a value and touches no Machine | Equivalent for pure selection | `address_error_selection_does_not_mutate_cpu_or_machine_state` | RDRAM, cartridge, CPU construction fields, and COP0 inspectors remain unchanged. |
| CPU/COP0 mutation | `enter_local_address_error_exception` owns mutation, not the selection predicate itself | `select_cpu_data_address_error` remains pure; `Cpu::enter_data_address_error_exception` mutates in seam 039 | Equivalent ownership split | Same test plus seam 039 entry tests | Selection performs no mutation; the narrow entry method mutates BadVAddr, local exception code, EPC, branch-delay flag, Status.EXL, PC, and next PC. |
| CPU load/store instruction behavior | C++ instruction paths compute effective address, call helpers, and may write GPRs | No Rust load/store instruction API | Not in scope | Source inspection | Selection is not load/store readiness as implemented behavior. |

### Address-Error Selection Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Address-error exception/COP0 mutation readiness audit | Completed by seam 039 for sealed data address errors | COP0 mutation ownership decision | Medium-high | Completed |
| Direct RDRAM value access with alignment preflight | Could combine direct value access, alignment, and pure selection, but still needs exception/fault return policy | Current seam plus exception policy | Medium | Needs future pass |
| CPU load/store instruction readiness audit | Full instructions need effective address, target access, sign/zero extension, GPR writeback, and exception entry | Exception/COP0 mutation readiness | High | Blocked |
| Documentation-only continuation | Pure selection is sealed and the next missing piece is mutation policy | Current seam complete | Low | Not recommended unless implementation is deferred |

### Seam 038 Audit Changes

- Audited C++ data alignment fault payloads, `MachineFaultAccessIntent`,
  Address Error Load/Store exception-code constants, and
  `step_cpu_instruction` fault handling.
- Added `CpuAddressErrorKind`, `CpuDataAddressError`, and
  `select_cpu_data_address_error` in `cpu/address.rs`.
- Re-exported the selection contract through `cpu.rs` and `lib.rs`.
- Added synthetic Rust tests for AdEL/AdES selection, faulting address as
  future BadVAddr source, width/access-kind preservation, byte no-fault
  behavior, and no Machine/CPU/COP0 mutation.
- Added no exception entry, COP0 mutation, BadVAddr/Cause/EPC mutation,
  PC/next PC mutation, CPU load/store instruction behavior, sign/zero
  extension, GPR writeback, memory map, bus, device/MMIO behavior, DMA,
  LL/SC instruction behavior, fetch/decode/execute/step, SDL/window/runtime,
  host shell, or C++ integration behavior.
- No C++ source files were changed.

## CPU Data Alignment / Exception Readiness Decision/Seal

C++ alignment truth is source-clear as a pure low-address-bit predicate, but
exception entry is not pure. Natural alignment checks live in instruction cases
before CPU memory helper calls. The C++ failure helpers carry data read/write
intent in `MachineFaultAccessIntent`, and `step_cpu_instruction` later converts
that intent into local Address Error Load or Address Error Store COP0 exception
entry when the control-flow state allows it. Rust mirrors only the low-bit
contract and the read/write intent metadata required for future exception
classification. It does not mutate COP0, enter exceptions, perform GPR
writeback, sign/zero extend, resolve memory targets, execute instructions, or
touch Machine state.

### Alignment Owner Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Equivalent? | Notes |
| --- | --- | --- | --- | --- |
| CPU data address | `src/core/machine.hpp` `CpuAddress = std::uint32_t`; instruction effective addresses | `cpu/address.rs` `CpuAddress(u32)` | Equivalent behavior, different ownership shape | Alignment operates on the CPU-visible address value, not raw RDRAM offset. |
| Data width | Instruction cases choose byte/halfword/word/doubleword operations; C++ failure helpers carry width 2/4/8 | `cpu/address.rs` `CpuDataWidth::{Byte, Halfword, Word, Doubleword}` | Equivalent for alignment widths | Byte width has no rejection; multi-byte widths map to 2/4/8-byte natural alignment. |
| Data read/write distinction | `MachineFaultAccessIntent::{kDataRead,kDataWrite}` | `CpuDataAccessKind::{Read,Write}` | Equivalent for exception classification | C++ maps read intent to AdEL and write intent to AdES in the step fault handler. The pure alignment check records kind; seam 039 may consume the selected address error for narrow entry. |
| Pure alignment predicate | Instruction checks `(effective_address & 1/3/7) != 0` before helper calls | `check_cpu_data_alignment(access_kind, address, width)` | Equivalent | Rust checks only low address bits and returns `Ok(())` or `CpuDataAlignmentError`. |
| Alignment error payload | C++ `MachineFault` stores operation, address, width, and access intent | `CpuDataAlignmentError` stores `CpuDataAccessKind`, `CpuAddress`, and `CpuDataWidth` | Equivalent for pure contract; Rust-only API safety | Rust omits operation strings and exception delivery because those belong to instruction/exception seams. |
| COP0 exception entry | `step_cpu_instruction` calls `enter_local_address_error_exception` with AdEL/AdES code | `Cpu::enter_data_address_error_exception` in seam 039 | Equivalent for sealed data address-error entry | Alignment remains a pure predicate; entry mutation is sealed separately and still excludes instruction execution. |

### Alignment Behavior

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| Byte access alignment | `LB/LBU/SB` have no natural-alignment rejection; partial lane paths preflight byte targets | `CpuDataWidth::Byte` mask `0` | Equivalent | `byte_data_alignment_accepts_representative_addresses` | All low address bits are accepted for byte access. |
| Halfword read alignment | `LH/LHU` reject `(effective_address & 0x1) != 0` with `kDataRead` | `check_cpu_data_alignment(Read, _, Halfword)` | Equivalent | `halfword_data_alignment_uses_low_bit_only` | Odd addresses reject; even addresses pass. |
| Halfword write alignment | `SH` rejects `(effective_address & 0x1) != 0` with `kDataWrite` | `check_cpu_data_alignment(Write, _, Halfword)` | Equivalent | Same | Read/write distinction is preserved in the Rust error. |
| Word read alignment | `LW/LL/LWU` reject `(effective_address & 0x3) != 0` with `kDataRead` | `check_cpu_data_alignment(Read, _, Word)` | Equivalent | `word_data_alignment_rejects_low_bits_one_two_and_three` | Low bits `01`, `10`, and `11` reject. |
| Word write alignment | `SW/SC` reject `(effective_address & 0x3) != 0` with `kDataWrite` | `check_cpu_data_alignment(Write, _, Word)` | Equivalent | Alignment tests plus source inspection | Store instruction source-value selection and SC result writeback are absent. |
| Doubleword read alignment | `LD/LLD` reject `(effective_address & 0x7) != 0` with `kDataRead` | `check_cpu_data_alignment(Read, _, Doubleword)` | Equivalent | `doubleword_data_alignment_rejects_low_bits_one_through_seven` | Low bits `001` through `111` reject. |
| Doubleword write alignment | `SD/SCD` reject `(effective_address & 0x7) != 0` with `kDataWrite` | `check_cpu_data_alignment(Write, _, Doubleword)` | Equivalent | Same | SCD reservation/GPR result behavior remains absent. |
| High address bits | C++ masks only low bits in the alignment check | Rust uses only `address.value() & width.alignment_mask()` | Equivalent | `data_alignment_ignores_high_cpu_address_bits` | KSEG bits, direct/non-direct status, and target mapping are not part of this pure contract. |
| Error preservation | C++ `MachineFault` carries address, width, and access intent | `CpuDataAlignmentError::{address,width,access_kind}` | Equivalent for pure contract | Alignment tests | Rust returns an error value instead of throwing or entering COP0. |
| Address Error Load / Store mapping | `step_cpu_instruction` maps read to `kCop0ExceptionCodeAddressErrorLoad` and write to `kCop0ExceptionCodeAddressErrorStore` | `select_cpu_data_address_error`; `Cpu::enter_data_address_error_exception` | Equivalent | Source inspection; selection and entry tests | The Rust access kind chooses AdEL versus AdES before the narrow entry method writes the selected code. |
| BadVAddr/Cause/EPC mutation | `enter_local_address_error_exception` mutates COP0/control-flow state | `Cpu::enter_data_address_error_exception` in seam 039 | Equivalent for sealed data address-error entry | Source inspection; entry tests | Mutation is narrow to sealed data address errors and does not add generic exceptions or instruction execution. |
| CPU load/store instructions | C++ instruction cases compute effective address, check alignment, call helpers, extend/select values, and write GPRs | No Rust load/store instruction API | Not in scope | Source inspection | This seam is a value/address contract only. |
| Memory map/bus/devices | C++ target resolution is separate from natural-alignment checks | No Rust memory map, bus, device, or DMA API | Not in scope | Source inspection | Alignment may be checked before future target resolution, but this pass adds no target resolver. |

### Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| CPU data alignment parity seal | Implemented and sealed in this pass | C++ instruction low-bit checks and read/write fault intent | Low | CPU data alignment contract sealed |
| Address-error exception/COP0 readiness audit | C++ already maps unaligned data read/write to AdEL/AdES and mutates BadVAddr/Cause/EPC/PC state | COP0 mutation ownership decision | Medium-high | Recommended |
| Direct RDRAM load-value with alignment preflight | Could compose direct value access plus pure alignment, but still needs exception/fault policy | Direct value access and alignment contract | Medium | Needs future pass after exception policy |
| CPU load/store instruction readiness audit | Full instructions need effective address, alignment, target access, sign/zero extension, GPR writeback, and exceptions | Exception/COP0 readiness | High | Needs future pass |
| LL/SC readiness audit | LL/LLD/SC/SCD combine alignment, target resolution, reservation staging/matching/clear, and GPR result writeback | Exception/COP0 and instruction boundaries | High | Blocked |
| Documentation-only continuation | Alignment contract is now sealed and next missing piece is exception mutation | Current seam complete | Low | Not recommended unless implementation is deferred |

### Seam 037 Audit Changes

- Audited C++ natural-alignment checks for `LH/LHU/LW/LL/LWU/LD/LLD` and
  `SH/SW/SC/SD/SCD`, byte access behavior, partial lane preflight behavior,
  `MachineFaultAccessIntent`, `MachineFaultKind::kUnalignedCpuMemoryAccess`,
  `step_cpu_instruction` fault handling, and COP0 address-error constants.
- Added `CpuDataWidth`, `CpuDataAccessKind`, `CpuDataAlignmentError`, and
  `check_cpu_data_alignment` in `cpu/address.rs`.
- Re-exported the alignment contract through `cpu.rs` and `lib.rs`.
- Added synthetic Rust tests for byte, halfword, word, doubleword, high-bit
  independence, error payload preservation, and read/write access-kind
  preservation.
- Added no CPU load/store instruction behavior, GPR writeback, sign extension,
  zero extension, exception entry, COP0 mutation, BadVAddr/Cause/EPC mutation,
  branch-delay exception handling, memory map, bus, device/MMIO behavior, DMA,
  LL/SC instruction behavior, fetch/decode/execute/step, SDL/window/runtime,
  host shell, or C++ integration behavior.
- No C++ source files were changed.

## CPU Direct RDRAM Address Classification Decision/Seal

The direct CPU-address-to-RDRAM subset is source-clear and separable from the
broader C++ data target resolver. C++ translates direct KSEG0/KSEG1-style
`CpuAddress` values by masking to a 29-bit physical address, then accepts only
full-width spans inside 4 MiB Machine-owned RDRAM. Rust mirrors only that direct
RDRAM subset in `cpu::address`; it does not implement CPU load/store,
`CpuDataTarget`, SP/MMIO/device targets, exceptions, DMA, LL/SC instruction
semantics, instruction execution, or a memory map.

### Address Type and Owner Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Equivalent? | Notes |
| --- | --- | --- | --- | --- |
| CpuAddress type | `src/core/machine.hpp` `using CpuAddress = std::uint32_t` | `rust/crates/fn64-core/src/cpu/address.rs` `CpuAddress(u32)` | Equivalent behavior, different ownership shape | Rust adds an explicit newtype for the earned sidecar seam without rewriting existing raw storage APIs. |
| RdramOffset type | `src/core/machine.hpp` `using RdramOffset = std::uint32_t` | `cpu/address.rs` `RdramOffset(u32)` for classification results; raw RDRAM APIs still use `usize` offsets | Equivalent behavior, different ownership shape | The newtype is only the CPU-address classification result. Prior raw storage APIs are not churned. |
| CpuDataTarget or equivalent target classification | `src/core/machine.hpp` private `CpuDataTargetKind` / `CpuDataTarget`; `machine_cpu.cpp` `require_cpu_data_target` | `cpu/address.rs` `CpuAddressTarget` with `DirectRdram` or `Unsupported` only | Equivalent for direct RDRAM subset; C++ exists, Rust intentionally absent for broader targets | SP DMEM/IMEM, SP/MMIO, MI, AI, PI, and SI target resolution is not ported. |
| Direct RDRAM target | `translate_cpu_rdram_address` and `translate_cpu_physical_rdram_address` | `classify_direct_rdram_address` -> `CpuAddressTarget::DirectRdram(RdramOffset)` | Equivalent | Rust mirrors only the direct alias plus RDRAM span check. |
| Unsupported/non-direct target | `translate_direct_cpu_physical_address` / `translate_cpu_physical_rdram_address` return `false`; `require_*` callers may throw | `CpuAddressTarget::Unsupported` | Equivalent classification fact; Rust-only API safety, no emulator truth for fault delivery | Rust does not model `MachineFault` or COP0 exception entry. |
| KSEG0 direct range | `translate_direct_cpu_physical_address` accepts `(address & 0xe0000000) == 0x80000000` | `translate_direct_cpu_physical_address` in `cpu/address.rs` | Equivalent | RDRAM success also requires the masked physical span to fit the requested width. |
| KSEG1 direct range | `translate_direct_cpu_physical_address` accepts `(address & 0xe0000000) == 0xa0000000` | `translate_direct_cpu_physical_address` in `cpu/address.rs` | Equivalent | KSEG1 uses the same 29-bit physical-address mask and RDRAM span check. |
| RDRAM size/bounds | `Machine::kRdramSizeBytes = 4 * 1024 * 1024`; `translate_cpu_physical_rdram_address` rejects `width == 0`, `width > size`, and `offset > size - width` | `RDRAM_SIZE_BYTES`; `translate_cpu_physical_rdram_address` in `cpu/address.rs` | Equivalent | Bounds are full-span checks before producing an offset. |
| Translation from CpuAddress to RdramOffset | `physical = cpu_address & 0x1fffffff`; `out_rdram_address = static_cast<RdramOffset>(physical_address)` | `RdramOffset::new(cpu_address.value() & 0x1fff_ffff)` after bounds check | Equivalent | This is mask-based direct-alias translation, not a TLB/MMU model. |
| Side-effect-free classification | Static private C++ translation functions return `bool` and write an out parameter only | Pure Rust function returns enum value and mutates no Machine state | Equivalent | Test `direct_rdram_address_classification_preserves_machine_state` covers no mutation of RDRAM, reservation, CPU/COP0, or cartridge facts. |

### Direct RDRAM Classification Behavior

| Behavior | C++ owner file/function | Rust owner file/function | Equivalent? | Proof/gate | Notes |
| --- | --- | --- | --- | --- | --- |
| KSEG0 base maps to offset 0 | `machine.cpp` `translate_direct_cpu_physical_address`; `translate_cpu_physical_rdram_address` | `cpu/address.rs` `classify_direct_rdram_address` | Equivalent | `kseg0_and_kseg1_bases_map_to_rdram_offset_zero` | `0x80000000` maps to physical/RDRAM offset `0`. |
| KSEG1 base maps to offset 0 | Same | Same | Equivalent | `kseg0_and_kseg1_bases_map_to_rdram_offset_zero` | `0xa0000000` maps to physical/RDRAM offset `0`. |
| KSEG0 last valid RDRAM address | `translate_cpu_physical_rdram_address` `offset <= size - width` | Same | Equivalent | `kseg0_and_kseg1_last_byte_map_to_last_rdram_offset` | For width 1, `0x803fffff` maps to offset `0x003fffff`. |
| KSEG1 last valid RDRAM address | Same | Same | Equivalent | `kseg0_and_kseg1_last_byte_map_to_last_rdram_offset` | For width 1, `0xa03fffff` maps to offset `0x003fffff`. |
| KSEG0 address beyond RDRAM | `translate_cpu_physical_rdram_address` returns `false` | `CpuAddressTarget::Unsupported` | Equivalent classification fact | `direct_rdram_classification_rejects_out_of_range_spans` | `0x80400000` and partial-width near-end spans are rejected. |
| KSEG1 address beyond RDRAM | Same | Same | Equivalent classification fact | `direct_rdram_classification_rejects_out_of_range_spans` | `0xa0400000` and partial-width near-end spans are rejected. |
| Non-KSEG0/KSEG1 direct address behavior | `translate_direct_cpu_physical_address` returns `false` | `CpuAddressTarget::Unsupported` | Equivalent classification fact | `non_direct_and_non_rdram_direct_addresses_are_unsupported` | Low, `0x60000000`, `0xc0000000`, and `0xe0000000` examples reject before RDRAM bounds. |
| Offset calculation | `cpu_address & 0x1fffffff` | `cpu_address.value() & 0x1fff_ffff` | Equivalent | `direct_rdram_offset_calculation_masks_segment_bits` | Both KSEG0 and KSEG1 aliases produce the same raw RDRAM offset. |
| Raw RDRAM offset unit | `RdramOffset` is byte offset into RDRAM storage | `RdramOffset` is byte offset into RDRAM storage | Equivalent | Source inspection; Rust tests | This is not a CPU virtual address. |
| Whether classification checks alignment | C++ translation functions do not check alignment | Rust classifier does not check alignment | Equivalent | `classification_accepts_unaligned_cpu_addresses_without_alignment_check` | Alignment remains instruction/load/store exception behavior, not classification behavior. |
| Whether classification enters exceptions | Translation functions return `false`; `require_*` wrappers and `step_cpu_instruction` own faults/exceptions | Rust returns `Unsupported`; no exception/COP0 mutation | Equivalent for translation only; exceptions not in scope | Source inspection; Rust tests | Rust does not mirror `MachineFault` throwing or COP0 exception entry. |
| Whether classification touches devices | Direct RDRAM translation does not inspect SP/MMIO/device state; `require_cpu_data_target` is separate | Rust has no device target resolver | Equivalent for direct RDRAM subset | Source inspection | Device/MMIO target classification remains intentionally absent. |
| Whether classification mutates state | Static translation functions are side-effect-free except out parameter | Pure Rust return value only | Equivalent | `direct_rdram_address_classification_preserves_machine_state` | No RDRAM, reservation, CPU/COP0, cartridge, device, DMA, reset, or execution mutation. |
| Whether load/store is in scope | C++ load/store callers consume the classification later | No Rust load/store API | Not in scope | Source inspection | Classification is not CPU load/store readiness as implemented behavior. |
| Whether memory-map/bus is in scope | C++ comments separate current target resolver from a full bus/map | No Rust memory-map or bus API | Not in scope | Source inspection | The Rust enum is direct-RDRAM-only and not a bus target table. |
| Whether DMA is in scope | DMA paths are separate Machine/device behavior | No Rust DMA API | Not in scope | Source inspection | Direct classification does not stage or copy bytes. |
| Whether instruction execution is in scope | Instruction execution lives in `step_cpu_instruction` and private execute helpers | Rust has raw field decode and identity classification only; no fetch, execute, or step API | Not in scope | Source inspection | The classifier can be tested without instruction execution. |

### Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| Direct RDRAM address classification parity seal | Implemented and sealed in this pass | Source-clear C++ direct translation and Rust raw storage constants | Low | Direct address classification sealed |
| Direct RDRAM value access family | Direct RDRAM classification plus raw RDRAM reads/writes now compose into CPU-addressed direct value helpers | Implemented by seam 036 without full `require_cpu_data_target`, exceptions, sign extension, or writeback | Medium | Direct RDRAM value access family sealed |
| CPU load/store instruction readiness decision | Direct RDRAM value helpers now exist, but instruction cases still own effective-address calculation, alignment, sign/zero extension, source/target GPRs, and writeback | Alignment/exception ownership and instruction-boundary audit | High | Needs future pass |
| Alignment/exception readiness audit | C++ instruction paths enforce alignment and may enter local COP0 address-error state | COP0 mutation/exception ownership remains absent | Medium | Recommended |
| Device/MMIO target classification audit | C++ `require_cpu_data_target` includes SP/MMIO/MI/AI/PI/SI targets | Device state ownership and DMA/interrupt policy are absent | High | Needs future pass |
| Memory-map/bus readiness audit | C++ has a local target resolver but not a generalized bus | Address classification is now narrower than a map; device decisions remain absent | High | Documentation only |
| Documentation-only continuation | Direct classification is sealed but load/store remains blocked | Current seam complete | Low | Not recommended unless next implementation seam is intentionally deferred |

### Seam 035 Audit Changes

- Audited C++ `CpuAddress`, `CpuPhysicalAddress`, `RdramOffset`,
  `CpuDataTarget`, direct KSEG0/KSEG1 translation, direct RDRAM span bounds,
  CPU data target resolution, load/store callers, exception/COP0 coupling, and
  SP/MMIO/device/DMA coupling.
- Added `rust/crates/fn64-core/src/cpu/address.rs` with `CpuAddress`,
  `RdramOffset`, `CpuAddressTarget`, and `classify_direct_rdram_address`.
- Mirrored only the source-backed direct RDRAM subset: KSEG0/KSEG1 top-bit
  acceptance, `0x1fffffff` offset masking, full-width RDRAM bounds checks,
  zero/oversized-width rejection, unsupported/non-direct classification, and
  absence of alignment checks or state mutation.
- Added tests for KSEG0/KSEG1 base and last-byte mappings, requested-width
  boundaries, out-of-range spans, non-direct unsupported addresses, mask-based
  offset calculation, unaligned raw CPU addresses, zero/oversized widths, and
  Machine-state preservation.
- Added no CPU load/store, sign extension, alignment exception behavior,
  exception/COP0 mutation, `CpuDataTarget` SP/MMIO/device target resolver,
  memory-map, bus, DMA, LL/SC instruction behavior, reset, instruction
  execution, host shell, SDL, renderer, or C++ integration behavior.
- No C++ source files were changed.

## CPU Load/Store Readiness Audit

Raw RDRAM storage access and direct RDRAM address classification are not
CPU-visible load/store behavior. Current C++ CPU data paths use `CpuAddress`,
translate only direct KSEG0/KSEG1-style aliases to `CpuPhysicalAddress`,
classify a small set of local data targets, and then reach raw
RDRAM/SP/device helpers. The Rust sidecar now has the raw storage prerequisites
and direct RDRAM classification listed below, but no CPU load/store,
`CpuDataTarget`, memory-map, exception, LL/SC instruction, or execution surface.

### Address / Ownership Map

| Concept | C++ owner file/function/type | Rust owner file/function/type | Current status | Notes |
| --- | --- | --- | --- | --- |
| CPU-visible address type | `src/core/machine.hpp` `CpuAddress = std::uint32_t` | `cpu/address.rs` `CpuAddress(u32)` | Equivalent for direct classification | CPU-visible helpers and instruction effective addresses use `CpuAddress`, not `RdramOffset`. Rust does not yet use this type for load/store. |
| Raw RDRAM offset type | `src/core/machine.hpp` `RdramOffset = std::uint32_t`; raw `read_rdram_*` / `write_rdram_*` helpers | `cpu/address.rs` `RdramOffset(u32)` for classification results; raw storage APIs still use `usize` offsets | Equivalent storage semantics, different ownership shape | Raw storage offsets are already earned; the classification newtype is not a CPU address. |
| Direct CPU physical translation | `src/core/machine.cpp` `translate_direct_cpu_physical_address` | `cpu/address.rs` private translation inside `classify_direct_rdram_address` | Equivalent for direct RDRAM subset | Accepts only CPU addresses whose top bits match `0x80000000` or `0xa0000000`, then masks with `0x1fffffff`. |
| RDRAM CPU-address to offset translation | `translate_cpu_rdram_address`; `translate_cpu_physical_rdram_address` | `cpu/address.rs` `classify_direct_rdram_address` | Equivalent for direct RDRAM subset | Source-clear direct aliases and full-width RDRAM span checks are now mirrored. |
| CPU data target resolution | `src/core/machine_cpu.cpp` `require_cpu_data_target`; `CpuDataTargetKind` | No Rust target resolver | Source-coupled | C++ dispatch recognizes RDRAM, SP DMEM, SP IMEM, SP MMIO, MI, AI, PI, and SI. This is broader than raw storage. |
| RDRAM region detection | `translate_cpu_physical_rdram_address` | `cpu/address.rs` private span check inside `classify_direct_rdram_address` | Equivalent for direct RDRAM subset | The source rule is a pure physical span check against 4 MiB RDRAM. |
| Unmapped address handling | `fail_cpu_data_address_rejected`; `fail_cpu_direct_rdram_address` | `CpuAddressTarget::Unsupported`; no Rust CPU fault model | Rust-only API safety, no emulator truth for fault delivery | C++ wrappers throw `MachineFault(kCpuRdramAddressRejected)` with data read/write intent, and `step_cpu_instruction` may convert it to local COP0 AdEL/AdES. Rust classification only returns unsupported. |
| Out-of-range RDRAM handling | `translate_cpu_physical_rdram_address` returns false; raw helpers throw `std::out_of_range` when called directly | `RdramAccessError` for raw storage access only | Rust-only API safety, no emulator truth | Rust storage errors do not model C++ CPU data faults or local exception entry. |
| Alignment/misalignment handling | `fail_unaligned_halfword_memory_access`; `fail_unaligned_word_memory_access`; `fail_unaligned_doubleword_memory_access`; instruction cases | No Rust alignment model | Blocked by alignment/exception audit | Alignment is instruction-path behavior before CPU memory helpers; raw storage helpers intentionally accept unaligned offsets. |
| Exception/COP0 coupling | `enter_local_address_error_exception`; `step_cpu_instruction` fault catch | `Cpu::enter_data_address_error_exception` for sealed data address errors only | Narrow data address-error entry earned | Local AdEL/AdES entry mutation is now mirrored only for the sealed value path; generic exception and instruction fault handling remain absent. |
| Memory-map/bus coupling | `CpuDataTargetKind`; `require_cpu_data_target` comments say this is not a full bus/map | No Rust memory map or bus | Blocked by memory-map seam | C++ has a local target resolver, not a generalized bus. Rust has none. |
| Device/DMA coupling | SP/PI/SI/AI register writes and DMA paths in `machine_cpu.cpp` | No Rust device or DMA state | Blocked by device/DMA seam | CPU word writes can trigger local SP/PI/SI/AI side effects; this prevents honest load/store implementation from being RDRAM-only. |

### Load Readiness Map

| Load/path | C++ owner file/function | Address input | Width | Signed? | Endian behavior | Alignment behavior | State read | State written | Rust prerequisites earned? | Blockers | Recommended status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CPU byte read helper | `machine_cpu.cpp` `read_cpu_memory_u8` | `CpuAddress` | 1 | None | Byte read | Target resolver only | RDRAM or SP byte memory; other targets unsupported | None | Raw RDRAM `read_u8` and direct RDRAM classification | `CpuDataTarget`, SP/device targets, exception model | Not yet earned |
| CPU halfword read helper | `read_cpu_memory_u16_be` | `CpuAddress` | 2 | None | Big-endian helper | Target resolver only; instruction checks alignment | RDRAM or SP byte memory; other targets unsupported | None | Raw RDRAM `read_u16_be` and direct RDRAM classification | `CpuDataTarget`, SP/device targets, exception model | Not yet earned |
| CPU word read helper | `read_cpu_memory_u32_be` | `CpuAddress` | 4 | None | Big-endian helper or MMIO register read | Target resolver only; instruction checks alignment | RDRAM, SP DMEM/IMEM, SP/MI/AI/PI/SI MMIO | Device reads may observe local pending/status fields | Raw RDRAM `read_u32_be` and direct RDRAM classification | Device/MMIO state, `CpuDataTarget`, and exception model | Blocked by memory-map seam |
| CPU doubleword read helper | `read_cpu_memory_u64_be` | `CpuAddress` | 8 | None | Big-endian helper | Target resolver only; instruction checks alignment | RDRAM or SP byte memory; other targets unsupported | None | Raw RDRAM `read_u64_be` and direct RDRAM classification | `CpuDataTarget`, SP targets, exception model | Not yet earned |
| `LB` | `execute_cpu_instruction` `kLb` | Effective `CpuAddress` from GPR low word plus sign-extended immediate | 1 | Sign-extends byte to 32 bits, then sign-extends GPR word | Byte | No alignment check | CPU GPR source, CPU memory helper | Target GPR | Raw byte read and GPR storage only | Instruction execution, address translation, sign extension, writeback, exception model | Blocked by instruction execution seam |
| `LBU` | `kLbu` | Effective `CpuAddress` | 1 | Zero-extends byte to GPR word | Byte | No alignment check | CPU GPR source, CPU memory helper | Target GPR | Raw byte read and GPR storage only | Instruction execution, address translation, zero extension, writeback | Blocked by instruction execution seam |
| `LH` | `kLh` | Effective `CpuAddress` | 2 | Sign-extends halfword to 32 bits, then sign-extends GPR word | Big-endian | Requires `address & 1 == 0` | CPU GPR source, CPU memory helper | Target GPR or local AdEL exception | Raw u16_be read and GPR storage only | Alignment fault and exception/COP0 behavior | Blocked by alignment/exception audit |
| `LHU` | `kLhu` | Effective `CpuAddress` | 2 | Zero-extends halfword to GPR word | Big-endian | Requires `address & 1 == 0` | CPU GPR source, CPU memory helper | Target GPR or local AdEL exception | Raw u16_be read and GPR storage only | Alignment fault, zero extension, exception/COP0 behavior | Blocked by alignment/exception audit |
| `LWL` | `kLwl` | Effective `CpuAddress` | 1-4 byte lanes | Merged low word is sign-extended after byte 0 replacement | Big-endian lane helpers | No natural word alignment; preflights effective byte | CPU GPR target value, CPU memory bytes | Target GPR | Raw byte read and GPR storage only | Partial-load lane rules, preflight/no-ghost fault behavior, instruction writeback | Blocked by instruction execution seam |
| `LW` | `kLw` | Effective `CpuAddress` | 4 | Sign-extends word to GPR value | Big-endian | Requires `address & 3 == 0` | CPU GPR source, CPU memory helper | Target GPR or local AdEL exception | Raw u32_be read and GPR storage only | Alignment fault and exception/COP0 behavior | Blocked by alignment/exception audit |
| `LL` | `kLl` | Effective `CpuAddress` | 4 | Sign-extends word to GPR value | Big-endian raw RDRAM read | Requires `address & 3 == 0`; RDRAM target only | CPU GPR source, RDRAM storage | Target GPR and CPU/RDRAM reservation width 4 | Raw u32_be read and private reservation staging exist separately | LL instruction semantics, target resolver, reservation coupling, exception model | Blocked by LL/SC seam |
| `LWU` | `kLwu` | Effective `CpuAddress` | 4 | Zero-extends word to GPR value | Big-endian | Requires `address & 3 == 0` | CPU GPR source, CPU memory helper | Target GPR or local AdEL exception | Raw u32_be read and GPR storage only | Alignment fault, zero extension, writeback | Blocked by instruction execution seam |
| `LWR` | `kLwr` | Effective `CpuAddress` plus aligned base | 1-4 byte lanes | Full offset 3 row sign-extends; partial rows preserve high 32 bits | Big-endian lane helpers | No natural word alignment; preflights effective byte | Previous target GPR and CPU memory bytes | Target GPR | Raw byte read and GPR storage only | Partial-load lane rules, no-ghost fault behavior, writeback | Blocked by instruction execution seam |
| `LDL` | `kLdl` | Effective `CpuAddress` | 1-8 byte lanes | Full 64-bit merge, no sign conversion | Big-endian lane helpers | No natural doubleword alignment; preflights effective byte | Previous target GPR and CPU memory bytes | Target GPR | Raw byte read and GPR storage only | Partial-doubleword lane rules and writeback | Blocked by instruction execution seam |
| `LDR` | `kLdr` | Effective `CpuAddress` plus aligned base | 1-8 byte lanes | Full 64-bit merge, no sign conversion | Big-endian lane helpers | No natural doubleword alignment; preflights effective byte | Previous target GPR and CPU memory bytes | Target GPR | Raw byte read and GPR storage only | Partial-doubleword lane rules and writeback | Blocked by instruction execution seam |
| `LD` | `kLd` | Effective `CpuAddress` | 8 | Full 64-bit value | Big-endian | Requires `address & 7 == 0` | CPU GPR source, CPU memory helper | Target GPR or local AdEL exception | Raw u64_be read and GPR storage only | Alignment fault and exception/COP0 behavior | Blocked by alignment/exception audit |
| `LLD` | `kLld` | Effective `CpuAddress` | 8 | Full 64-bit value | Big-endian raw RDRAM read | Requires `address & 7 == 0`; RDRAM target only | CPU GPR source, RDRAM storage | Target GPR and CPU/RDRAM reservation width 8 | Raw u64_be read and private reservation staging exist separately | LLD instruction semantics, target resolver, reservation coupling | Blocked by LL/SC seam |
| Coprocessor memory load identities | `identify_cpu_instruction` `kLwc1`, `kLwc2`, `kLdc1`, `kLdc2`; default execute path | Instruction word identity only | 4 or 8 by mnemonic, not executed | Not modeled | Not modeled | Not modeled | None | Unsupported step result, no memory read | None | COP1/COP2 unsupported by current C++ execution | Documentation only |
| Raw RDRAM read helpers | `src/core/machine.cpp` `read_rdram_u8/u16_be/u32_be/u64_be` | `RdramOffset` | 1/2/4/8 | None | Byte or big-endian composition | No alignment validation | RDRAM storage | None | Yes | Not CPU-visible; no address translation | Equivalent storage semantics, different ownership shape |

### Store Readiness Map

| Store/path | C++ owner file/function | Address input | Width | Endian behavior | Alignment behavior | State read | State written | Reservation effect | Rust prerequisites earned? | Blockers | Recommended status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| CPU byte write helper | `machine_cpu.cpp` `write_cpu_memory_u8` | `CpuAddress` | 1 | Byte write | Target resolver only | Source value plus target state | RDRAM or SP byte memory; other byte targets unsupported | RDRAM target calls raw write and invalidates overlapping reservation | Raw RDRAM byte write and direct RDRAM classification exist | `CpuDataTarget`, SP/device targets, exception model | Not yet earned |
| CPU halfword write helper | `write_cpu_memory_u16_be` | `CpuAddress` | 2 | Big-endian helper | Target resolver only; instruction checks alignment | Source value plus target state | RDRAM or SP byte memory; other halfword targets unsupported | RDRAM target invalidates width 2 | Raw RDRAM u16_be write and direct RDRAM classification exist | `CpuDataTarget`, SP/device targets, exception model | Not yet earned |
| CPU word write helper | `write_cpu_memory_u32_be` | `CpuAddress` | 4 | Big-endian helper or MMIO register write | Target resolver only; instruction checks alignment | Source value plus device state | RDRAM, SP DMEM/IMEM, SP/MI/AI/PI/SI MMIO | RDRAM target invalidates width 4; device writes may trigger DMA/interrupt state | Raw RDRAM u32_be write and direct RDRAM classification exist | Device/MMIO/DMA state, `CpuDataTarget`, exception model | Blocked by device/DMA seam |
| CPU doubleword write helper | `write_cpu_memory_u64_be` | `CpuAddress` | 8 | Big-endian helper | Target resolver only; instruction checks alignment | Source value plus target state | RDRAM or SP byte memory; other doubleword targets unsupported | RDRAM target invalidates width 8 | Raw RDRAM u64_be write and direct RDRAM classification exist | `CpuDataTarget`, SP targets, exception model | Not yet earned |
| `SB` | `execute_cpu_instruction` `kSb` | Effective `CpuAddress` | 1 | Low byte of GPR word | No alignment check | CPU GPR source, target resolver | CPU memory helper target | RDRAM target invalidates width 1 | Raw byte write and GPR storage only | Instruction execution and CPU address path | Blocked by instruction execution seam |
| `SH` | `kSh` | Effective `CpuAddress` | 2 | Low halfword stored big-endian | Requires `address & 1 == 0` | CPU GPR source, target resolver | CPU memory helper target or local AdES exception | RDRAM target invalidates width 2 | Raw u16_be write and GPR storage only | Alignment fault and exception/COP0 behavior | Blocked by alignment/exception audit |
| `SWL` | `kSwl` | Effective `CpuAddress` | 1-4 byte lanes | Big-endian lane bytes from low GPR word | No natural word alignment; preflights every byte before mutation | CPU GPR source and target resolver | CPU memory bytes | Each RDRAM byte write invalidates through raw helper after full preflight | Raw byte write exists | Partial-store lane rules, no-ghost preflight, instruction execution | Blocked by instruction execution seam |
| `SW` | `kSw` | Effective `CpuAddress` | 4 | Big-endian low GPR word | Requires `address & 3 == 0` | CPU GPR source and target resolver | CPU memory helper target | RDRAM target invalidates width 4 | Raw u32_be write exists | Alignment fault, exception/COP0, target resolver | Blocked by alignment/exception audit |
| `SC` | `kSc` | Effective `CpuAddress` | 4 | Big-endian low GPR word on success | Requires `address & 3 == 0`; RDRAM target only | CPU GPR source, reservation state | Clears reservation; maybe RDRAM; writes `rt` success result | Checks reservation width 4, clears reservation, successful write invalidates through raw helper | Raw u32_be write and private reservation pieces exist separately | LL/SC instruction semantics and GPR result writeback | Blocked by LL/SC seam |
| `SWR` | `kSwr` | Effective `CpuAddress` plus aligned base | 1-4 byte lanes | Big-endian lane bytes from low GPR word | No natural word alignment; preflights every byte before mutation | CPU GPR source and target resolver | CPU memory bytes | Each RDRAM byte write invalidates through raw helper after full preflight | Raw byte write exists | Partial-store lane rules and no-ghost fault behavior | Blocked by instruction execution seam |
| `SDL` | `kSdl` | Effective `CpuAddress` | 1-8 byte lanes | Big-endian lane bytes from full GPR value | No natural doubleword alignment; preflights every byte before mutation | CPU GPR source and target resolver | CPU memory bytes | Each RDRAM byte write invalidates through raw helper after full preflight | Raw byte write exists | Partial-doubleword store lane rules | Blocked by instruction execution seam |
| `SDR` | `kSdr` | Effective `CpuAddress` plus aligned base | 1-8 byte lanes | Big-endian lane bytes from full GPR value | No natural doubleword alignment; preflights every byte before mutation | CPU GPR source and target resolver | CPU memory bytes | Each RDRAM byte write invalidates through raw helper after full preflight | Raw byte write exists | Partial-doubleword store lane rules | Blocked by instruction execution seam |
| `SD` | `kSd` | Effective `CpuAddress` | 8 | Big-endian full GPR value | Requires `address & 7 == 0` | CPU GPR source and target resolver | CPU memory helper target | RDRAM target invalidates width 8 | Raw u64_be write exists | Alignment fault and exception/COP0 behavior | Blocked by alignment/exception audit |
| `SCD` | `kScd` | Effective `CpuAddress` | 8 | Big-endian full GPR value on success | Requires `address & 7 == 0`; RDRAM target only | CPU GPR source, reservation state | Clears reservation; maybe RDRAM; writes `rt` success result | Checks reservation width 8, clears reservation, successful write invalidates through raw helper | Raw u64_be write and private reservation pieces exist separately | LL/SC instruction semantics and GPR result writeback | Blocked by LL/SC seam |
| Coprocessor memory store identities | `identify_cpu_instruction` `kSwc1`, `kSwc2`, `kSdc1`, `kSdc2`; default execute path | Instruction word identity only | 4 or 8 by mnemonic, not executed | Not modeled | Not modeled | None | Unsupported step result, no memory write | None | None | COP1/COP2 unsupported by current C++ execution | Documentation only |
| Raw RDRAM write helpers | `src/core/machine.cpp` `write_rdram_u8/u16_be/u32_be/u64_be` | `RdramOffset` | 1/2/4/8 | Byte or big-endian storage write | No alignment validation | RDRAM storage and reservation state | RDRAM storage plus possible reservation clear | Invalidates once with width 1/2/4/8 after bounds check | Yes | Not CPU-visible; no address translation | Equivalent behavior, different ownership shape |
| SP/PI/SI/AI DMA side paths | `perform_sp_write_dma`; `perform_si_pif_to_dram_dma`; `perform_ai_rdram_dma`; `stage_cartridge_bytes_to_rdram` | Device/register-local values or raw offsets | Byte loops or byte spans | Byte copies/reads | Preflight span checks | RDRAM, SP/PIF/cartridge/device shadows | RDRAM/SP/PIF/device/MI interrupt state depending path | RDRAM-writing DMA paths use raw helpers and invalidate reservations | Raw byte write/read exist | DMA/device ownership absent in Rust | Blocked by device/DMA seam |

### Recommended Next Seam

| Candidate seam | Why | Prerequisites | Risk | Recommended? |
| --- | --- | --- | --- | --- |
| CPU direct RDRAM address classification | Direct RDRAM subset is now mirrored in Rust without full target resolution | Raw storage access family sealed | Low after seam 035 | Direct address classification sealed |
| Direct RDRAM value access family | Direct RDRAM classification and raw access are now composed for direct RDRAM read/write values only | No full target resolver, memory map, exception entry, sign extension, or GPR writeback | Medium | Direct RDRAM value access family sealed |
| Load sign/zero extension | C++ source is clear for `LB/LBU/LH/LHU/LW/LWU`, but it is embedded in instruction execution and GPR writeback | Direct RDRAM value helpers, GPR writeback policy, exception boundary | High if attempted before address/exceptions | Needs future pass |
| Store CPU-address path | Stores dispatch across RDRAM, SP memory, and MMIO, and word writes can trigger DMA/device effects | Address translation, target resolver, exception behavior, device decisions | High | Blocked |
| Alignment/exception audit | C++ alignment faults and address rejection are converted to local COP0 exception entry in `step_cpu_instruction` | COP0 mutation/exception readiness audit | Medium | Recommended |
| LL/SC audit | `LL/LLD/SC/SCD` use RDRAM-only target checks, reservation staging/matching/clear, and GPR result writeback | Address translation, instruction execution/readiness, reservation seal | High | Blocked |
| Memory-map audit | C++ `require_cpu_data_target` already spans RDRAM/SP/MMIO, but comments say it is not a full bus/map | Address classification; device ownership audit | High | Needs future pass |
| Documentation-only continuation | This pass already maps the current blockers without adding behavior | Current audit | Low | Documentation only |

### Seam 034 Audit Changes

- Audited C++ CPU-visible memory helpers, direct-address translation, target
  classification, alignment faults, local address-error exception entry, LL/SC
  reservation coupling, SP/PI/SI/AI device/DMA coupling, proof observations, and
  current Rust raw storage code.
- Added no Rust emulator behavior, no new Rust source methods, and no tests.
- Confirmed during seam 034 that Rust had no speculative CPU load/store,
  address translation, memory-map, bus, DMA, reset, execution, exception,
  interrupt, or LL/SC API; seam 035 later added only the direct RDRAM
  classification subset.
- Recommended the next narrow seam as CPU direct-address classification /
  RDRAM CPU-address-to-offset translation, because CPU load/store cannot be
  honest until CPU-visible addresses are mapped to raw storage offsets without
  implying a full bus or device memory map.

## Seam 033 Audit Changes

- Re-audited C++ `Machine::read_rdram_u16_be`, `read_rdram_u32_be`,
  `read_rdram_u64_be`, `read_rdram_u8`, `RdramOffset`, RDRAM storage,
  `fail_rdram_access`, CPU load/LL/LLD callers, DMA callers, reset clearing,
  and proof/step-probe observations.
- Added pure raw RDRAM storage read-width methods on `Rdram`:
  `read_u16_be`, `read_u32_be`, and `read_u64_be`.
- Kept reads RDRAM-owned because they inspect storage only and do not mutate
  reservation, RDRAM bytes, CPU, COP0, Cartridge, DMA, reset, or execution
  state. Reservation-aware writes remain Machine-owned.
- Mirrored the C++ read behavior for the earned subset: fixed-width bounds
  checks happen before byte composition; valid reads compose bytes in
  big-endian order; invalid reads return `RdramAccessError` as Rust-only API
  safety.
- Added tests for first/last valid reads, exact-end and near-end invalid reads,
  past-end invalid reads, unaligned raw storage offsets, big-endian
  composition, invalid-read reservation/RDRAM preservation, and unrelated
  Machine fact preservation.
- Added the raw read-width family seal table and read/write storage-family
  status summary.
- Did not add CPU load/store, sign extension, memory-map, bus, DMA, LL/SC,
  reset, step, execution, renderer, SDL, host shell, or C++ integration
  behavior.
- No C++ source files were changed.

## Rust Layout/Repo Hygiene Seal Table

| Area | Current owner/file | Hygiene status | Action taken | Notes |
| --- | --- | --- | --- | --- |
| Rust sidecar status | `rust/README.md`; `rust/PARITY.md`; Cargo workspace under `rust/` | Clean after change | Added README sidecar statement; updated this ledger | Rust remains a sidecar candidate only. No C++ build integration was added. |
| Module naming | `cartridge`, `cartridge::byte_order`, `cartridge::metadata`, `machine`, `machine::rdram_reservation`, `cpu`, `cpu::address`, `cpu::cop0`, `cpu::registers`, `cpu::scalars`, `rdram` | Clean after change | Split earned byte-order, metadata, CPU address, register, scalar-state, private COP0 construction, and private CPU/RDRAM reservation construction owners into named modules | Names map to owned machine truth, not broad utility buckets. |
| `lib.rs` public exports | `rust/crates/fn64-core/src/lib.rs` | Clean after change | Re-exported `DirectRdramAccessError` with `Machine` | Exports remain limited to earned sidecar cartridge, Machine, CPU, RDRAM, direct-RDRAM value access API safety, metadata, and constants. No prelude was added. |
| Unsafe policy | `rust/crates/fn64-core/src/lib.rs`; `rust/crates/fn64-core/Cargo.toml` | Clean | Preserved `#![forbid(unsafe_code)]` and Cargo `unsafe_code = "forbid"` lint | No unsafe code is present in `fn64-core`. |
| Broad utility module absence | `rust/crates/fn64-core/src/` | Clean | Inspected names; added README layout law | No `util`, `common`, `core2`, `misc`, `helpers`, `engine`, `graphics_api`, `platform`, or `emulator` module exists. |
| File-size/semantic ownership audit | `cartridge.rs`, `cpu.rs`, `machine.rs`, split child modules | Clean after change | Moved byte order to `cartridge/byte_order.rs`, metadata to `cartridge/metadata.rs`, GPR access/mutation to `cpu/registers.rs`, PC/next PC/HI/LO scalar access/staging to `cpu/scalars.rs`, COP0 construction/access fields to `cpu/cop0.rs`, and CPU/RDRAM reservation construction/staging/invalidation state to `machine/rdram_reservation.rs` | `cartridge.rs` remains over 250 lines because it includes cartridge aggregate tests; it now has one aggregate owner and should stay whole until entry/read behavior grows separate pressure. |
| Cargo workspace shape | `rust/Cargo.toml`; `rust/crates/fn64-core/Cargo.toml` | Clean | Inspected; no Cargo metadata expansion | Workspace has one `fn64-core` crate, resolver 2, `publish = false`, no dependencies. |
| Cargo.lock policy | `rust/Cargo.lock`; `rust/README.md` | Clean after change | Documented lockfile policy in README | Lockfile is retained for reproducible local sidecar verification even with no third-party dependencies. |
| rustfmt availability | Toolchain invoked by `cargo fmt --check` | Clean after change | Attempted and recorded | Current verification toolchain provides `cargo fmt`; formatting is part of the active gate. |
| clippy availability | Toolchain invoked by `cargo clippy --all-targets -- -D warnings` | Clean after change | Attempted and recorded | Current verification toolchain provides `cargo clippy`; linting is part of the active gate. |
| `rust/target` exclusion | `rust/.gitignore` | Clean after change | Added `/target/`; final verification removes `rust/target` | Build artifacts must stay out of final status and upload bundles. |
| Commercial ROM / BIOS / copyrighted fixture absence | Rust tests under owner modules | Clean | Inspected tests | Tests continue to use synthetic byte arrays only. No BIOS/PIF blobs or commercial ROM fixtures are present. |
| C++ monolith not copied into Rust | Rust module tree | Clean after change | Added ownership-named submodules where current owners were distinct | Rust does not mirror the C++ `Machine` monolith file structure. |
| Future split triggers | `rust/README.md`; this ledger | Needs future pass | Documented layout law and split trigger | Future reset, step, COP0 mutation, instruction, memory, DMA, or host seams should create owner-named modules only when earned. |

## Open-Source Naming / Identity Seal Table

| Area | Current name/text | Classification | Action taken | Notes |
| --- | --- | --- | --- | --- |
| `rust/` directory name | Forward implementation workspace path | Tooling/workspace name | Kept | Directory name is workflow context, not product identity or machine semantics. |
| `fn64-core` crate name | `rust/crates/fn64-core/Cargo.toml` package `fn64-core` | Product/domain name | Kept | Names the fn64 machine core, not the implementation language, host workbench, renderer, or platform. |
| `lib.rs` exports | `Cartridge`, `Machine`, `Cpu`, `Rdram`, ROM metadata/source-layout types, constants, and earned error types | Product/domain name | Kept | Export surface remains limited to earned machine-core truth and Rust API safety errors. No prelude or broad convenience namespace exists. |
| Module names | `cartridge`, `cartridge::byte_order`, `cartridge::metadata`, `machine`, `machine::rdram_reservation`, `cpu`, `cpu::address`, `cpu::cop0`, `cpu::registers`, `cpu::scalars`, `rdram` | Product/domain name | Kept after change | Modules describe owned machine truth. `address`, `cop0`, and `rdram_reservation` are semantic owners; no language/platform/engine name was found. |
| Public type names | `Cartridge`, `Machine`, `Cpu`, `Rdram`, `CpuAddress`, `RdramOffset`, `CpuAddressTarget`, `DirectRdramAccessError`, `RomSourceLayout`, `RomMetadata`, `CpuRegisterIndexError`, `RdramAccessError` | Product/domain name | Added `DirectRdramAccessError` for the direct RDRAM value access API | Type names describe domain state or explicit API safety. Private `Cop0` and `CpuRdramReservation` are not exported. No `RustMachine`, `RustCpu`, engine, backend, frontend, or platform type exists. |
| README identity text | `rust/README.md` title and opening scope | Clean after change | Rewrote | README now leads with fn64 as a small headless machine core and says Rust is implementation material, not product identity. |
| PARITY identity text | This ledger title and opening scope | Clean after retirement | Rewrote | Ledger names Rust as the sole current implementation and qualifies all C++ rows as historical Git anchors. |
| Rust/tooling mentions | `rust/` path, Cargo files, and cargo command logs | Tooling/workflow name | Kept | Mentions are allowed only as tooling or parity context. They do not brand the product as a Rust emulator. |
| Fedora/Linux mentions | No Fedora or Linux product names in Rust source/docs | Clean | No change | Search found no Fedora/Linux identity drift. |
| SDL/window/graphics mentions | Negative-scope entries in README and PARITY; C++ host owner references in parity tables | Negative-scope mention | Kept | Mentions state absent/not-in-scope host plumbing and do not name the core. |
| frontend/backend/platform/engine mentions | README/PARITY negative-scope layout law mentions broad names; no modules/types use them | Negative-scope mention | Kept | Mentions are warnings against identity drift, not product names. |
| helper/misc/common/util bucket names | README/PARITY negative-scope layout law mentions broad buckets; no modules/types use them | Negative-scope mention | Kept | Search hits are the rule text documenting absence. No vague bucket module exists. |
| Host shell vocabulary | README and PARITY non-goal/identity text | Product/domain name | Kept | `host shell` is the preferred vocabulary for thin hosts when earned. |
| Open-source/legal fixture language | `rust/README.md` fixture policy; this ledger fixture rows | Clean after change | Added README fixture policy | README now explicitly rejects commercial ROMs, BIOS/PIF blobs, copyrighted fixtures, and circumvention material. |

## Seam 004 Audit Changes

- Audited and kept `Machine::powered_on` as equivalent for construction flag
  value only.
- Added `rust/crates/fn64-core/src/rdram.rs` as the minimal Rust-sidecar RDRAM
  construction owner.
- Added Machine ownership of `Rdram` in `rust/crates/fn64-core/src/machine.rs`.
- Exported `Rdram` and `RDRAM_SIZE_BYTES` from `rust/crates/fn64-core/src/lib.rs`.
- Did not add Rust CPU state, RDRAM read/write behavior, address translation,
  memory maps, bus abstractions, reset APIs, step/fetch/decode/execute,
  staging, DMA, renderer, SDL, host shell, or C++ integration.

## Seam 005 Audit Changes

- Added `rust/crates/fn64-core/src/cpu.rs` as the minimal Rust-sidecar CPU
  construction/default-state owner.
- Added Machine ownership of `Cpu` in `rust/crates/fn64-core/src/machine.rs`.
- Exported `Cpu`, `CPU_GPR_COUNT`, `NON_BOOT_RESET_VECTOR_PC`, and
  `NON_BOOT_RESET_VECTOR_NEXT_PC` from `rust/crates/fn64-core/src/lib.rs`.
- Mirrored only construction/default-state facts: non-boot PC/next PC, zero
  HI/LO, zero general registers, and zero/false COP0 construction fields.
- Did not add reset, step, fetch, decode, execute, instruction identity,
  register writes, CPU/RDRAM reservation, memory-map behavior, bus abstractions,
  DMA, renderer, SDL, host shell, or C++ integration.

## Seam 006 Audit Changes

- Sealed Rust `Cpu` as a valid sidecar ownership refinement for C++ Machine-owned
  CPU construction fields.
- Added the CPU construction parity seal table to make construction-state
  equivalence explicit without claiming type-layout equivalence.
- Strengthened Rust CPU tests with the explicit 32-GPR construction boundary.
- Re-audited CPU/RDRAM reservation construction state and kept it intentionally
  absent because the C++ state is tied to LL/SC, load/store, RDRAM writes, DMA
  invalidation, and execution behavior.
- Did not add reset, step, fetch, decode, execute, instruction identity,
  register writes, zero-register write enforcement, RDRAM behavior, memory-map
  behavior, bus abstractions, DMA, renderer, SDL, host shell, or C++ integration.

## Seam 007 Audit Changes

- Audited C++ reset ownership and callers. C++ has a public
  `Machine::reset_to_non_boot_power_on_state()` operation, and construction calls
  it.
- Decided not to add Rust reset. The C++ reset mutates already represented
  CPU/RDRAM/powered-on state, but also clears unearned SP DMEM/IMEM, PIF RAM,
  SP/MI/PI/AI/SI local shadows, and CPU/RDRAM reservation state.
- Documented reset caller lineage from construction, the C++ bootstrap proof
  path, and the no-window step probe.
- No Rust tests were added because there is no Rust reset API in this pass.
- Did not add reset, step, fetch, decode, execute, instruction identity,
  register writes, zero-register write enforcement, RDRAM read/write behavior,
  memory-map behavior, bus abstractions, DMA, renderer, SDL, host shell, or C++
  integration.

## Seam 008 Audit Changes

- Added `Cpu::set_gpr` as a narrow GPR storage mutation seam that mirrors C++
  `Machine::stage_cpu_gpr` / `write_cpu_gpr_value` state semantics.
- Made `Cpu::gpr(0)` and `Cpu::set_gpr(0, value)` explicitly preserve the
  zero register as zero.
- Added `CpuRegisterIndexError` for invalid Rust writes. This is Rust API
  safety; C++ throws `std::out_of_range` for invalid public stage/inspect GPR
  indices.
- Strengthened Rust tests for nonzero GPR mutation, zero-register write-ignore,
  invalid write indices, and proof that GPR mutation does not alter PC, next PC,
  HI/LO, or COP0 construction fields.
- Did not add `Machine::cpu_mut`, reset, step, fetch, decode, execute,
  instruction identity, instruction writeback, HI/LO mutation, COP0 mutation,
  RDRAM read/write behavior, memory-map behavior, bus abstractions, DMA,
  renderer, SDL, host shell, or C++ integration.

## Seam 009 Audit Changes

- Added `rust/README.md` to document sidecar status, owner-named layout law,
  current scope, Cargo.lock policy, and Rust verification gates.
- Added `rust/.gitignore` with `/target/` so Rust build artifacts do not enter
  status or upload bundles.
- Split cartridge byte-order logic into
  `rust/crates/fn64-core/src/cartridge/byte_order.rs`.
- Split cartridge metadata parsing into
  `rust/crates/fn64-core/src/cartridge/metadata.rs`.
- Split earned CPU GPR access/mutation into
  `rust/crates/fn64-core/src/cpu/registers.rs`.
- Kept `cartridge.rs`, `cpu.rs`, `machine.rs`, and `rdram.rs` as owner modules.
  `cartridge.rs` remains over the line-count smoke threshold because it owns the
  cartridge aggregate, range reads, entry inspection, and close tests.
- Did not add, remove, or change emulator behavior. No reset, step, fetch,
  decode, execute, instruction writeback, PC/next PC mutation, HI/LO mutation,
  COP0 mutation, RDRAM read/write behavior, memory-map behavior, bus
  abstractions, DMA, renderer, SDL, host shell, or C++ integration was added.

## Seam 010 Audit Changes

- Rewrote `rust/README.md` title and opening identity text so fn64 is presented
  as a small headless machine core, not as a Rust-branded emulator.
- Added README identity and fixture-policy sections covering implementation
  workspace status, machine-truth public naming, thin host-shell vocabulary, and the
  absence of commercial ROMs, BIOS/PIF blobs, copyrighted fixtures, or
  circumvention material.
- Kept this ledger under fn64 Rust parity/audit wording and updated the pass
  identifier.
- Added the open-source naming / identity seal table to classify the `rust/`
  path, `fn64-core` crate name, public exports, module names, type names, and
  search hits.
- Kept all Rust source module/type/API names unchanged because inspection found
  no public `RustMachine`, `RustCore`, Fedora/Linux, SDL/window/graphics,
  frontend/backend/platform/engine, or vague bucket product naming.
- Did not add, remove, or change emulator behavior. No reset, step, fetch,
  decode, execute, instruction writeback, PC/next PC mutation, HI/LO mutation,
  COP0 mutation, RDRAM read/write behavior, memory-map behavior, bus
  abstractions, DMA, renderer, SDL, host shell, or C++ integration was added.

## Seam 011 Audit Changes

- Audited C++ PC, next PC, HI, and LO scalar storage, public inspection, and
  public staging in `src/core/machine.hpp` and `src/core/machine_cpu.cpp`.
- Added `Cpu::stage_pc`, `Cpu::stage_next_pc`, `Cpu::stage_hi`, and
  `Cpu::stage_lo` in `rust/crates/fn64-core/src/cpu/scalars.rs`.
- Mirrored C++ `stage_cpu_pc` exactly for scalar state: staging PC also stages
  next PC to `pc + 4` using unsigned 32-bit wrapping addition.
- Kept scalar mutation on Rust `Cpu` as the sidecar ownership refinement. No
  `Machine` forwarding, mutable Machine CPU accessor, reset, step, fetch,
  decode, execute, branch, jump, delay-slot, instruction writeback, COP0
  mutation, memory behavior, DMA, renderer, SDL, host shell, or C++ integration
  was added.
- Split PC, next PC, HI, and LO access/staging into
  `rust/crates/fn64-core/src/cpu/scalars.rs` because scalar state is now an
  earned CPU sub-owner distinct from construction/COP0 state and the existing
  `cpu::registers` GPR owner.
- Strengthened Rust tests for PC staging, next-PC staging, HI staging, LO
  staging, zero-register preservation, GPR preservation, and COP0 construction
  field preservation.

## Seam 012 Audit Changes

- Re-audited C++ scalar declarations, storage fields, public staging methods,
  private write helpers, and proof/step-probe callers for PC, next PC, HI, and
  LO.
- Sealed `Cpu::stage_pc` as exact state parity with C++ `stage_cpu_pc` /
  `write_cpu_pc`: it stages PC and also stages next PC to `pc + 4`.
- Added an explicit Rust test for unsigned 32-bit wrapping at the `stage_pc`
  next-PC boundary: `0xffff_fffc..=0xffff_ffff` wraps to `0..=3`.
- Confirmed `Cpu::stage_next_pc`, `Cpu::stage_hi`, and `Cpu::stage_lo` remain
  single-field scalar staging operations.
- Confirmed Rust Machine still exposes only read-only `Machine::cpu()` and does
  not add Machine-level scalar forwarding or `cpu_mut`.
- No Rust APIs were renamed. The `stage_` names remain intentional because they
  match the C++ staging seam and avoid presenting `stage_pc` as a simple setter.
- Did not add, remove, or change emulator behavior. No reset, step, fetch,
  decode, execute, instruction writeback, PC advancement helper, branch, jump,
  link, delay-slot, COP0 mutation, RDRAM read/write behavior, memory-map
  behavior, bus abstractions, DMA, renderer, SDL, host shell, or C++
  integration was added.

## Seam 013 Audit Changes

- Audited C++ COP0 storage fields, construction/reset initialization, private
  read/write helpers, MFC0/MTC0 execution switches, local exception/interrupt
  entry, ERET return, Count ticking, and no-window proof/step-probe observations
  for the then-current construction/access seam.
- Split the already-earned Rust COP0 construction fields from `cpu.rs` into the
  private semantic owner `rust/crates/fn64-core/src/cpu/cop0.rs`.
- Kept public Rust COP0 inspection on `Cpu::cop0_*` accessors and did not export
  `Cop0` from `lib.rs`.
- Decided not to add COP0 mutation in seam 013. Current C++ mutation is
  source-visible but
  private and reached through MTC0 instruction execution, exception/interrupt
  entry, ERET, or Count ticking rather than through a public stage helper.
  Seam 062 later seals only the narrow Count helper.
- Moved the COP0 construction test to the new owner module with the same test
  count. Existing GPR and scalar staging tests still exercise COP0 construction
  preservation through the unchanged `Cpu::cop0_*` accessors.
- Did not add reset, step, fetch, decode, execute, instruction writeback, MFC0/
  MTC0 behavior, ERET behavior, exception/interrupt behavior, TLB/MMU behavior,
  timer/count progression at that time, broad COP0 mutation, RDRAM read/write
  behavior, memory-map behavior, bus abstractions, DMA, renderer, SDL, host
  shell, or C++ integration.

## Current Rust-Only Surface

- `Result`/error enums are Rust API shape only. They do not claim new emulator
  truth.
- `impl std::error::Error` and `fmt::Display` exist to make those explicit
  errors usable and testable.
- `Cpu::gpr` returns `Option<u64>` for Rust-side inspection, and
  `Cpu::set_gpr` returns `Result<(), CpuRegisterIndexError>` for Rust-side GPR
  mutation. Invalid-index return shapes are Rust API safety; they do not mirror
  C++ exception behavior or add emulator behavior.
- `Cpu::stage_pc`, `Cpu::stage_next_pc`, `Cpu::stage_hi`, and `Cpu::stage_lo`
  are Rust-side scalar staging methods that mirror C++ storage semantics through
  the sidecar `Cpu` ownership shape. They do not add reset, step, control-flow,
  instruction writeback, or validation behavior.
- `cpu/cop0.rs` is a private Rust ownership split for already-earned COP0
  construction fields. It does not add a public `Cop0` export, COP0 mutation,
  MFC0/MTC0 instruction behavior, exceptions, interrupts, ERET, or timer/count
  progression.
- `cpu/instruction.rs` owns the raw CPU instruction-word field decode value:
  `CpuInstructionWord`, `CpuInstructionFields`, and
  `decode_cpu_instruction_word`. It decodes an already-formed `u32` into raw
  opcode, register, shift amount, function, immediate, and target fields. It
  also owns `CpuInstructionIdentity` and `identify_cpu_instruction`, a pure
  classification from those decoded fields into C++ source-clear instruction
  identity names and unknown boundaries. It does not fetch bytes, perform endian
  conversion, execute instructions, mutate CPU/Machine/RDRAM/COP0 state,
  sign/zero extend immediates, form branch offsets, form jump addresses,
  interpret operands, or step.
- `Machine::classify_cpu_instruction_fetch_target` owns the narrow Machine
  instruction-fetch target classification seam. It checks 4-byte instruction
  alignment first, names direct RDRAM, SP DMEM, unavailable PIF reset fetch,
  non-direct unsupported fetches, and direct-target misses, and reads no memory.
  It does not fetch from SP DMEM/PIF, enter exceptions, read current PC, mutate
  state, decode, identify, execute, step, or create a memory map/bus.
- `Machine::fetch_direct_rdram_cpu_instruction_word` owns the narrow direct
  RDRAM instruction-word fetch seam. It checks 4-byte instruction alignment,
  resolves only direct KSEG0/KSEG1 RDRAM through the sealed classifier, reads one
  big-endian u32 through sealed direct RDRAM value access, and returns
  `CpuInstructionWord`. It does not fetch from SP DMEM, PIF/reset vectors, or
  devices, read current PC, mutate state, decode, identify, execute, step, enter
  exceptions, or create a memory map/bus.
- `machine/rdram_reservation.rs` is a private Rust ownership split for
  already-earned CPU/RDRAM reservation construction/default state, private
  staging/setup state, and private invalidation behavior. It does not add public
  reservation access, LL/SC, DMA, or memory-map behavior.
- `Rdram::read_u8`, `Rdram::read_u16_be`, `Rdram::read_u32_be`, and
  `Rdram::read_u64_be` are read-only raw storage-offset accessors. Their invalid
  offset/range `Result` errors are Rust API safety mirroring C++ out-of-range
  text.
- `Machine::write_rdram_u8` is a raw storage-offset byte write seam. It
  bounds-checks first, invalidates overlapping private reservation state with
  byte width `1`, and then mutates exactly one RDRAM byte.
- `Machine::write_rdram_u16_be` is a raw storage-offset u16_be write seam. It
  bounds-checks the full two-byte span first, invalidates overlapping private
  reservation state with byte width `2`, and then mutates exactly two RDRAM bytes
  in big-endian order.
- `Machine::write_rdram_u32_be` is a raw storage-offset u32_be write seam. It
  bounds-checks the full four-byte span first, invalidates overlapping private
  reservation state with byte width `4`, and then mutates exactly four RDRAM
  bytes in big-endian order.
- `Machine::write_rdram_u64_be` is a raw storage-offset u64_be write seam. It
  bounds-checks the full eight-byte span first, invalidates overlapping private
  reservation state with byte width `8`, and then mutates exactly eight RDRAM
  bytes in big-endian order.
- `Machine::reset` is a Machine-owned reset seam for the represented non-boot
  power-on state. It resets CPU scalar/GPR/COP0 state, RDRAM bytes, private
  CPU/RDRAM reservation state, and the local powered-on flag while preserving
  the owned Cartridge.
- `fn64-inspection` is a Rust no-window probe crate outside `fn64-core`. It owns
  process output, exit status, and probe sequencing only; it does not own
  Machine truth, path policy, SDL/window behavior, step, or execution.
- `Rdram` has crate-private checked-offset validation and byte/u16_be/u32_be/u64_be
  mutation helpers used by `Machine::write_rdram_u8`,
  `Machine::write_rdram_u16_be`, `Machine::write_rdram_u32_be`, and
  `Machine::write_rdram_u64_be`; it does not expose a public storage-only write
  API.
- `lib.rs` re-exports the sidecar cartridge, Machine, RDRAM construction/raw
  read-width, raw byte/u16_be/u32_be/u64_be-write, raw instruction-word decode,
  instruction identity classification, direct RDRAM instruction-fetch error
  shape, and CPU construction/staging APIs for the Rust workspace only.

## Still Not Product Parity

This sidecar is not product-parity unless all current C++ gates and
seam-equivalence checks pass. Even when they pass, this crate only proves the
cartridge/ROM byte-normalization seam, the narrow Machine construction
ownership subset, RDRAM construction size/zero-fill ownership, raw RDRAM byte
storage read/write access, raw RDRAM u16_be/u32_be/u64_be read access, raw RDRAM
u16_be/u32_be/u64_be write access, and the CPU
construction/default-state fields plus the narrow GPR, scalar staging, COP0
construction/access, and CPU/RDRAM reservation construction/default-state plus
private staging/setup/invalidation seams above, represented Machine reset, and
  the no-window construction/reset probe, plus pure raw CPU instruction-word field
  decode and identity classification from already-decoded raw fields, CPU
  instruction-fetch target classification, direct KSEG0/KSEG1 RDRAM CPU
  instruction-word fetch, read-only SP DMEM CPU instruction-word fetch, and
  explicit-address and current-PC instruction fetch over represented targets,
  pure instruction-fetch fault address-error selection, and narrow
  instruction-fetch address-error entry mutation. It does not prove step-owned
  full instruction fetch, SP IMEM fetch, PIF/reset-vector fetch bytes, fetch
  APIs that enter exceptions, broad/generic CPU execution, COP0 mutation outside the sealed
  reset/data-address-error/instruction-fetch-address-error seams, MFC0/MTC0
  instruction execution behavior, generic exception/interrupt behavior, ERET
  behavior, timer/count behavior beyond the narrow Count helper, CPU reset
  behavior beyond the represented non-boot reset seam, Machine-level broad CPU
  staging beyond the named
  GPR/scalar storage seams, RDRAM range access, CPU load/store, full address
  translation, Machine execution, cartridge/RDRAM staging, boot, PIF/CIC,
  device reset, memory map, bus, DMA, renderer, SDL, host runtime, broad host
  shell, ROM path policy, or game compatibility behavior.
