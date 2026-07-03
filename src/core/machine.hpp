#pragma once

#include <array>
#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>

#include "cartridge.hpp"

namespace fn64 {

using CpuRegisterValue = std::uint64_t;
using CpuAddress = std::uint32_t;
using CpuPhysicalAddress = std::uint32_t;
using CpuInstructionWord = std::uint32_t;
using RdramOffset = std::uint32_t;
using CartridgeOffset = std::uint32_t;
using PiCartAddress = std::uint32_t;

enum class MachineFaultKind {
  // Current local address-rejection fault for direct-RDRAM fetch/data gates and
  // data-dispatch misses. This is not COP0 address-error delivery.
  kCpuRdramAddressRejected,
  kUnsupportedCpuDataAccess,
  kUnalignedInstructionFetch,
  kUnalignedCpuMemoryAccess,
  kUnalignedControlTransferTarget,
  kSignedArithmeticOverflow,
};

class MachineFault : public std::runtime_error {
public:
  MachineFault(
      MachineFaultKind kind,
      std::string operation,
      CpuAddress cpu_address,
      std::size_t access_size,
      std::string message);

  MachineFaultKind kind() const noexcept;
  const std::string& operation() const noexcept;
  CpuAddress cpu_address() const noexcept;
  std::size_t access_size() const noexcept;

private:
  MachineFaultKind kind_;
  std::string operation_;
  CpuAddress cpu_address_;
  std::size_t access_size_;
};

class Machine {
public:
  // Current CPU scope: fn64 owns 64-bit integer register storage and a tiny
  // explicitly supported 64-bit D instruction cluster; most executed instructions
  // still model a local 32-bit word subset. CPU addresses, CPU physical
  // addresses produced by direct aliases, instruction words, physical RDRAM
  // offsets, PI cart-domain addresses, and normalized cartridge byte offsets
  // are deliberately separate 32-bit domains. CPU addresses include the direct
  // KSEG0/KSEG1 RDRAM alias form. This is not the full N64 VR4300 64-bit
  // execution model.

  // Public CPU execution result for fn64's current local step policy.
  // kStopped is a local stop condition, not N64 COP0 exception delivery.
  // kUnsupported is a non-compatibility result for unknown or unsupported
  // instructions; proof-backed unsupported paths roll back visible step state.
  // kInterrupted is a local interrupt-entry result; no instruction was fetched
  // or executed from the interrupted PC.
  // Local Machine step faults throw MachineFault; unrelated public API
  // precondition failures may still use standard C++ exceptions.
  enum class CpuInstructionStepResult {
    // A local instruction step completed and committed current pc/next_pc movement.
    kStepped,
    // A local stop instruction/condition completed and committed step PC movement.
    kStopped,
    // An unknown or unsupported instruction was reported without committing a step.
    kUnsupported,
    // A local interrupt entry completed without executing the interrupted instruction.
    kInterrupted,
  };

  explicit Machine(Cartridge cartridge);

  static constexpr CpuAddress kBlankInitialCpuPc = 0x00000000u;
  static constexpr CpuAddress kBlankInitialCpuNextPc = 0x00000004u;

  // Construction owns the Cartridge and creates fn64's current local blank
  // powered-on state: zeroed RDRAM, zeroed CPU registers, and the blank
  // pc/next_pc values above. This is not N64 reset, PIF/BIOS boot, or
  // cartridge execution. powered_on() is an informational local construction
  // state today; there is no public power-off transition, and kStopped does
  // not power the Machine off.
  // Public stage_* APIs are explicit synthetic mutation points for proof and
  // no-window hosts. stage_cartridge_bytes_to_rdram copies normalized
  // Cartridge bytes into physical RDRAM offsets; it does not map or execute
  // cartridge memory.
  bool powered_on() const;
  const Cartridge& cartridge() const;
  std::size_t rdram_size_bytes() const noexcept;

  std::uint32_t inspect_rdram_u32_be(RdramOffset address) const;

  void stage_rdram_u32_be(RdramOffset address, std::uint32_t value);

  void stage_cartridge_bytes_to_rdram(
      CartridgeOffset cartridge_offset,
      RdramOffset rdram_address,
      std::uint32_t byte_count);

  CpuAddress cpu_pc() const;
  CpuAddress cpu_next_pc() const;
  CpuRegisterValue inspect_cpu_hi() const;
  CpuRegisterValue inspect_cpu_lo() const;
  CpuRegisterValue inspect_cpu_gpr(std::size_t index) const;

  void stage_cpu_pc(CpuAddress value);
  void stage_cpu_next_pc(CpuAddress value);
  void stage_cpu_hi(CpuRegisterValue value);
  void stage_cpu_lo(CpuRegisterValue value);
  void stage_cpu_gpr(std::size_t index, CpuRegisterValue value);

  // Public CPU execution creation point. A completed step commits this
  // Machine's current pc/next_pc movement. Thrown MachineFault values are
  // local fn64 step faults, not modeled N64 exception paths; no-ghost rollback
  // is only claimed for paths covered by the proof suite.
  CpuInstructionStepResult step_cpu_instruction();

private:
  struct DecodedCpuInstructionWord {
    CpuInstructionWord raw = 0;
    std::uint8_t opcode = 0;
    std::uint8_t rs = 0;
    std::uint8_t rt = 0;
    std::uint8_t rd = 0;
    std::uint8_t sa = 0;
    std::uint8_t funct = 0;
    std::uint16_t immediate_u16 = 0;
    std::int16_t immediate_i16 = 0;
    std::uint32_t jump_target = 0;
  };

  // Local single-Machine LL/SC reservation state for Machine-owned RDRAM only.
  // This is not cache coherence, memory ordering, SMP, TLB, or COP0 behavior.
  struct CpuRdramReservation {
    bool valid = false;
    RdramOffset rdram_offset = 0;
    std::size_t width = 0;
  };

  // Private CPU data access dispatch seam. It names the current
  // CpuAddress -> CpuPhysicalAddress -> target split without adding a bus or
  // full memory map. RDRAM, local SP DMEM/IMEM byte memories, minimal local SP
  // DMA MMIO, tiny PI DMA MMIO, and local MI pending/mask MMIO are the only CPU
  // data targets today; instruction fetch remains RDRAM-only, and cartridge
  // bytes are not CPU-addressable.
  enum class CpuDataTargetKind {
    kRdram,
    kSpDmem,
    kSpImem,
    kSpMmio,
    kMi,
    kPi,
  };

  struct CpuDataTarget {
    CpuDataTargetKind kind = CpuDataTargetKind::kRdram;
    CpuPhysicalAddress physical_address = 0;
    RdramOffset rdram_offset = 0;
    std::uint32_t sp_memory_offset = 0;
    std::uint32_t sp_register_offset = 0;
    std::uint32_t mi_register_offset = 0;
    std::uint32_t pi_register_offset = 0;
  };

  struct SpDmaLengthCommand {
    std::uint32_t length = 0;
    std::uint32_t count = 0;
    std::uint32_t skip = 0;
    std::uint32_t transfer_length_per_block = 0;
    std::uint32_t block_count = 0;
  };

  // D/MIPS64-style identities are decoded so the step path can either execute
  // the small explicitly supported 64-bit cluster or report the rest as
  // unsupported; recognition here does not imply full VR4300 execution support.
  // COP0 is only narrowly subdecoded for local MFC0/MTC0 Status/Count/Compare/
  // Cause/EPC state, minimal local interrupt entry, and ERET return from that
  // entry. COP1/COP2/COP3, CACHE, and coprocessor memory
  // identities remain coarse unsupported decode boundaries. fn64 does not
  // model cache state/ops/coherence or general COP0 exception delivery from
  // these identities.
  enum class CpuInstructionIdentity {
    kUnknownPrimary,
    kSpecialUnknown,
    kRegimmUnknown,

    kSpecialSll,
    kSpecialSrl,
    kSpecialSra,
    kSpecialSllv,
    kSpecialSrlv,
    kSpecialSrav,
    kSpecialJr,
    kSpecialJalr,
    kSpecialSyscall,
    kSpecialBreak,
    kSpecialSync,
    kSpecialMfhi,
    kSpecialMthi,
    kSpecialMflo,
    kSpecialMtlo,
    kSpecialDsllv,
    kSpecialDsrlv,
    kSpecialDsrav,
    kSpecialMult,
    kSpecialMultu,
    kSpecialDiv,
    kSpecialDivu,
    kSpecialDmult,
    kSpecialDmultu,
    kSpecialDdiv,
    kSpecialDdivu,
    kSpecialAdd,
    kSpecialAddu,
    kSpecialSub,
    kSpecialSubu,
    kSpecialAnd,
    kSpecialOr,
    kSpecialXor,
    kSpecialNor,
    kSpecialSlt,
    kSpecialSltu,
    kSpecialDadd,
    kSpecialDaddu,
    kSpecialDsub,
    kSpecialDsubu,
    kSpecialTge,
    kSpecialTgeu,
    kSpecialTlt,
    kSpecialTltu,
    kSpecialTeq,
    kSpecialTne,
    kSpecialDsll,
    kSpecialDsrl,
    kSpecialDsra,
    kSpecialDsll32,
    kSpecialDsrl32,
    kSpecialDsra32,

    kRegimmBltz,
    kRegimmBgez,
    kRegimmBltzl,
    kRegimmBgezl,
    kRegimmTgei,
    kRegimmTgeiu,
    kRegimmTlti,
    kRegimmTltiu,
    kRegimmTeqi,
    kRegimmTnei,
    kRegimmBltzal,
    kRegimmBgezal,
    kRegimmBltzall,
    kRegimmBgezall,

    kJ,
    kJal,
    kBeq,
    kBne,
    kBlez,
    kBgtz,
    kAddi,
    kAddiu,
    kSlti,
    kSltiu,
    kAndi,
    kOri,
    kXori,
    kLui,
    kCop0,
    kCop0Mfc0,
    kCop0Mtc0,
    kCop0Eret,
    kCop1,
    kCop2,
    kCop3,
    kBeql,
    kBnel,
    kBlezl,
    kBgtzl,
    kDaddi,
    kDaddiu,
    kLdl,
    kLdr,
    kLb,
    kLh,
    kLwl,
    kLw,
    kLbu,
    kLhu,
    kLwr,
    kLwu,
    kSb,
    kSh,
    kSwl,
    kSw,
    kSdl,
    kSdr,
    kSwr,
    kCache,
    kLl,
    kLwc1,
    kLwc2,
    kLld,
    kLdc1,
    kLdc2,
    kLd,
    kSc,
    kSwc1,
    kSwc2,
    kScd,
    kSdc1,
    kSdc2,
    kSd,
  };

  enum class CpuInstructionExecutionResult {
    kExecuted,
    kStopped,
    kBranchLikelyNotTaken,
    kUnsupported,
  };

  static constexpr std::size_t kRdramSizeBytes = 4 * 1024 * 1024;
  static constexpr std::size_t kSpMemorySizeBytes = 4 * 1024;
  static constexpr std::size_t kCpuGprCount = 32;
  static constexpr CpuPhysicalAddress kSpDmemPhysicalBase = 0x04000000u;
  static constexpr CpuPhysicalAddress kSpImemPhysicalBase = 0x04001000u;
  static constexpr CpuPhysicalAddress kSpRegisterPhysicalBase = 0x04040000u;
  static constexpr std::uint32_t kSpRegisterWindowSize = 0x20u;
  static constexpr std::uint32_t kSpMemoryAddressRegisterOffset = 0x00u;
  static constexpr std::uint32_t kSpDramAddressRegisterOffset = 0x04u;
  static constexpr std::uint32_t kSpReadLengthRegisterOffset = 0x08u;
  static constexpr std::uint32_t kSpWriteLengthRegisterOffset = 0x0cu;
  static constexpr std::uint32_t kSpStatusRegisterOffset = 0x10u;
  static constexpr CpuPhysicalAddress kMiPhysicalBase = 0x04300000u;
  static constexpr std::uint32_t kMiRegisterWindowSize = 0x20u;
  static constexpr std::uint32_t kMiInterruptPendingRegisterOffset = 0x08u;
  static constexpr std::uint32_t kMiInterruptMaskRegisterOffset = 0x0cu;
  static constexpr std::uint32_t kMiInterruptPendingSp = 0x00000001u;
  static constexpr std::uint32_t kMiInterruptPendingPi = 0x00000010u;
  static constexpr std::uint32_t kMiSupportedInterruptBits =
      kMiInterruptPendingSp | kMiInterruptPendingPi;
  static constexpr std::uint8_t kCop0CountRegisterIndex = 9;
  static constexpr std::uint8_t kCop0CompareRegisterIndex = 11;
  static constexpr std::uint8_t kCop0StatusRegisterIndex = 12;
  static constexpr std::uint8_t kCop0CauseRegisterIndex = 13;
  static constexpr std::uint8_t kCop0EpcRegisterIndex = 14;
  static constexpr std::uint32_t kCop0StatusIe = 0x00000001u;
  static constexpr std::uint32_t kCop0StatusExl = 0x00000002u;
  static constexpr std::uint32_t kCop0StatusInterruptMask0 = 0x00000100u;
  static constexpr std::uint32_t kCop0StatusInterruptMask1 = 0x00000200u;
  static constexpr std::uint32_t kCop0StatusInterruptMask2 = 0x00000400u;
  static constexpr std::uint32_t kCop0StatusInterruptMask7 = 0x00008000u;
  static constexpr std::uint32_t kCop0StatusInterruptMask = 0x0000ff00u;
  static constexpr std::uint32_t kCop0SupportedStatusBits =
      kCop0StatusIe | kCop0StatusExl | kCop0StatusInterruptMask;
  static constexpr std::uint32_t kCop0CauseInterruptPending0 = 0x00000100u;
  static constexpr std::uint32_t kCop0CauseInterruptPending1 = 0x00000200u;
  static constexpr std::uint32_t kCop0CauseInterruptPending2 = 0x00000400u;
  static constexpr std::uint32_t kCop0CauseInterruptPending7 = 0x00008000u;
  static constexpr std::uint32_t kCop0SoftwareInterruptPendingBits =
      kCop0CauseInterruptPending0 | kCop0CauseInterruptPending1;
  static constexpr std::uint32_t kCop0SupportedInterruptPendingBits =
      kCop0SoftwareInterruptPendingBits | kCop0CauseInterruptPending2 |
      kCop0CauseInterruptPending7;
  static constexpr CpuAddress kLocalInterruptVectorPc = 0x80000180u;
  static constexpr CpuAddress kLocalInterruptVectorNextPc = 0x80000184u;
  static constexpr CpuPhysicalAddress kPiPhysicalBase = 0x04600000u;
  static constexpr std::uint32_t kPiRegisterWindowSize = 0x20u;
  static constexpr std::uint32_t kPiDramAddressRegisterOffset = 0x00u;
  static constexpr std::uint32_t kPiCartAddressRegisterOffset = 0x04u;
  static constexpr std::uint32_t kPiCartToRdramLengthRegisterOffset = 0x0cu;
  static constexpr std::uint32_t kPiStatusRegisterOffset = 0x10u;
  static constexpr PiCartAddress kPiCartRomBase = 0x10000000u;

  void reset_to_blank_rdram_power_on_state();

  std::uint8_t read_rdram_u8(RdramOffset address) const;
  std::uint16_t read_rdram_u16_be(RdramOffset address) const;
  std::uint32_t read_rdram_u32_be(RdramOffset address) const;
  CpuRegisterValue read_rdram_u64_be(RdramOffset address) const;

  void write_rdram_u8(RdramOffset address, std::uint8_t value);
  void write_rdram_u16_be(RdramOffset address, std::uint16_t value);
  void write_rdram_u32_be(RdramOffset address, std::uint32_t value);
  void write_rdram_u64_be(RdramOffset address, CpuRegisterValue value);

  void clear_cpu_rdram_reservation() noexcept;
  void set_cpu_rdram_reservation(RdramOffset address, std::size_t width) noexcept;
  bool cpu_rdram_reservation_matches(RdramOffset address, std::size_t width) const noexcept;
  void invalidate_cpu_rdram_reservation_for_write(
      RdramOffset address,
      std::size_t width) noexcept;

  // Current direct-RDRAM CPU address gate. KSEG0/KSEG1-style CpuAddress values
  // first translate to CpuPhysicalAddress; this gate accepts only physical
  // spans belonging to Machine-owned RDRAM and converts them to RdramOffset.
  // Every other CPU range remains a local MachineFault. This is not a bus,
  // full memory map, TLB translation, cartridge ROM mapping, or device/MMIO
  // dispatch; the separate CPU data target resolver owns the local SP
  // DMEM/IMEM byte memories, minimal SP DMA MMIO, and tiny PI MMIO subset that
  // have been earned for data loads/stores.
  static RdramOffset require_cpu_rdram_address(
      const char* operation,
      CpuAddress cpu_address,
      std::size_t width);
  static CpuDataTarget require_cpu_data_target(
      const char* operation,
      CpuAddress cpu_address,
      std::size_t width);

  std::uint8_t read_cpu_memory_u8(CpuAddress cpu_address) const;
  std::uint16_t read_cpu_memory_u16_be(CpuAddress cpu_address) const;
  std::uint32_t read_cpu_memory_u32_be(CpuAddress cpu_address) const;
  CpuRegisterValue read_cpu_memory_u64_be(CpuAddress cpu_address) const;

  void write_cpu_memory_u8(CpuAddress cpu_address, std::uint8_t value);
  void write_cpu_memory_u16_be(CpuAddress cpu_address, std::uint16_t value);
  void write_cpu_memory_u32_be(CpuAddress cpu_address, std::uint32_t value);
  void write_cpu_memory_u64_be(CpuAddress cpu_address, CpuRegisterValue value);

  static bool translate_cpu_rdram_address(
      CpuAddress cpu_address,
      std::size_t width,
      RdramOffset& out_rdram_address) noexcept;
  static bool translate_direct_cpu_physical_address(
      CpuAddress cpu_address,
      CpuPhysicalAddress& out_physical_address) noexcept;
  static bool translate_cpu_physical_rdram_address(
      CpuPhysicalAddress physical_address,
      std::size_t width,
      RdramOffset& out_rdram_address) noexcept;
  static bool translate_cpu_physical_sp_memory_address(
      CpuPhysicalAddress physical_address,
      std::size_t width,
      CpuDataTargetKind& out_kind,
      std::uint32_t& out_sp_offset) noexcept;
  static bool translate_cpu_physical_sp_register_address(
      CpuPhysicalAddress physical_address,
      std::uint32_t& out_register_offset) noexcept;
  static bool translate_cpu_physical_mi_register_address(
      CpuPhysicalAddress physical_address,
      std::uint32_t& out_register_offset) noexcept;
  static bool translate_cpu_physical_pi_register_address(
      CpuPhysicalAddress physical_address,
      std::uint32_t& out_register_offset) noexcept;

  const std::array<std::uint8_t, kSpMemorySizeBytes>& sp_memory_for_kind(
      CpuDataTargetKind kind) const;
  std::array<std::uint8_t, kSpMemorySizeBytes>& sp_memory_for_kind(
      CpuDataTargetKind kind);
  std::uint8_t read_sp_memory_u8(CpuDataTargetKind kind, std::uint32_t offset) const;
  std::uint16_t read_sp_memory_u16_be(CpuDataTargetKind kind, std::uint32_t offset) const;
  std::uint32_t read_sp_memory_u32_be(CpuDataTargetKind kind, std::uint32_t offset) const;
  CpuRegisterValue read_sp_memory_u64_be(CpuDataTargetKind kind, std::uint32_t offset) const;
  void write_sp_memory_u8(CpuDataTargetKind kind, std::uint32_t offset, std::uint8_t value);
  void write_sp_memory_u16_be(
      CpuDataTargetKind kind,
      std::uint32_t offset,
      std::uint16_t value);
  void write_sp_memory_u32_be(
      CpuDataTargetKind kind,
      std::uint32_t offset,
      std::uint32_t value);
  void write_sp_memory_u64_be(
      CpuDataTargetKind kind,
      std::uint32_t offset,
      CpuRegisterValue value);

  std::uint32_t read_sp_register_u32(
      CpuPhysicalAddress physical_address,
      CpuAddress cpu_address) const;
  void write_sp_register_u32(
      CpuPhysicalAddress physical_address,
      CpuAddress cpu_address,
      std::uint32_t value);
  static SpDmaLengthCommand decode_sp_dma_length_command(
      std::uint32_t length_register_value) noexcept;
  static bool translate_sp_memory_dma_base(
      std::uint32_t sp_memory_address,
      CpuDataTargetKind& out_kind,
      std::uint32_t& out_sp_offset) noexcept;
  static CpuDataTarget require_sp_memory_dma_blocks(
      const char* operation,
      std::uint32_t sp_memory_address,
      const SpDmaLengthCommand& command);
  static RdramOffset require_sp_dma_rdram_blocks(
      const char* operation,
      RdramOffset rdram_address,
      const SpDmaLengthCommand& command);
  void perform_sp_read_dma(std::uint32_t length_register_value);
  void perform_sp_write_dma(std::uint32_t length_register_value);

  std::uint32_t read_mi_register_u32(
      std::uint32_t register_offset,
      CpuAddress cpu_address) const;
  void write_mi_register_u32(
      std::uint32_t register_offset,
      CpuAddress cpu_address,
      std::uint32_t value);
  void latch_mi_interrupt_pending(std::uint32_t pending_bit) noexcept;
  std::uint32_t read_cop0_count() const noexcept;
  std::uint32_t read_cop0_compare() const noexcept;
  std::uint32_t read_cop0_status() const noexcept;
  std::uint32_t read_cop0_cause() const noexcept;
  std::uint32_t read_cop0_epc() const noexcept;
  void write_cop0_count(std::uint32_t value) noexcept;
  void write_cop0_compare(std::uint32_t value) noexcept;
  void write_cop0_status(std::uint32_t value) noexcept;
  void write_cop0_cause(std::uint32_t value) noexcept;
  void write_cop0_epc(std::uint32_t value) noexcept;
  void advance_cop0_count_after_committed_instruction() noexcept;
  std::uint32_t local_cop0_interrupt_pending_lines() const noexcept;
  bool local_interrupt_pending() const noexcept;
  bool local_interrupt_enabled() const noexcept;
  bool current_pc_allows_local_interrupt_entry() const noexcept;
  bool try_enter_local_interrupt() noexcept;
  bool local_eret_can_return() const noexcept;
  void return_from_local_interrupt_entry();

  std::uint32_t read_pi_register_u32(
      CpuPhysicalAddress physical_address,
      CpuAddress cpu_address) const;
  void write_pi_register_u32(
      CpuPhysicalAddress physical_address,
      CpuAddress cpu_address,
      std::uint32_t value);
  CartridgeOffset require_pi_cart_rom_source(
      PiCartAddress pi_cart_address,
      std::uint32_t byte_count) const;
  void perform_pi_cart_to_rdram_dma(std::uint32_t length_register_value);

  CpuRegisterValue cpu_hi() const;
  CpuRegisterValue cpu_lo() const;
  // Full-value helpers touch the GPR storage/staging surface. Word helpers are
  // the current local 32-bit instruction operand seam. Result helpers name the
  // current MIPS64-shaped writeback policy: arithmetic/shift/LUI/signed loads
  // sign-extend, unsigned loads zero-extend, comparisons write full 0/1
  // values, word MULT/DIV results sign-extend each HI/LO word, D MULT/DIV
  // results write full 64-bit HI/LO halves, and partial-word/doubleword loads
  // have named local 64-bit storage policies.
  CpuRegisterValue read_cpu_gpr_value(std::size_t index) const;
  std::uint32_t read_cpu_gpr_word(std::size_t index) const;

  void write_cpu_pc(CpuAddress value);
  void write_cpu_next_pc(CpuAddress value);
  void write_cpu_hi(CpuRegisterValue value);
  void write_cpu_lo(CpuRegisterValue value);
  void write_cpu_hi_word_sign_extended_result(std::uint32_t value);
  void write_cpu_lo_word_sign_extended_result(std::uint32_t value);
  void write_cpu_gpr_value(std::size_t index, CpuRegisterValue value);
  void write_cpu_gpr_word_sign_extended_result(std::size_t index, std::uint32_t value);
  void write_cpu_gpr_word_zero_extended_result(std::size_t index, std::uint32_t value);
  void write_cpu_gpr_partial_word_sign_extended_result(
      std::size_t index,
      std::uint32_t value);
  void write_cpu_gpr_partial_word_preserve_high_result(
      std::size_t index,
      std::uint32_t value,
      CpuRegisterValue previous_value);

  CpuInstructionWord fetch_cpu_instruction_word() const;

  static DecodedCpuInstructionWord decode_cpu_instruction_word(CpuInstructionWord raw);
  static CpuInstructionIdentity identify_cpu_instruction(
      const DecodedCpuInstructionWord& instruction);

  CpuInstructionExecutionResult execute_cpu_instruction(
      CpuInstructionIdentity identity,
      const DecodedCpuInstructionWord& instruction);

  Cartridge cartridge_;
  bool powered_on_ = false;
  std::array<std::uint8_t, kRdramSizeBytes> rdram_{};
  std::array<std::uint8_t, kSpMemorySizeBytes> sp_dmem_{};
  std::array<std::uint8_t, kSpMemorySizeBytes> sp_imem_{};
  CpuRdramReservation cpu_rdram_reservation_{};
  std::uint32_t sp_mem_address_ = 0;
  RdramOffset sp_dram_address_ = 0;
  std::uint32_t sp_rd_len_ = 0;
  std::uint32_t sp_wr_len_ = 0;
  std::uint32_t sp_status_ = 0;
  std::uint32_t mi_interrupt_pending_ = 0;
  std::uint32_t mi_interrupt_mask_ = 0;
  std::uint32_t cop0_count_ = 0;
  std::uint32_t cop0_compare_ = 0;
  bool cop0_timer_interrupt_pending_ = false;
  std::uint32_t cop0_status_ = 0;
  std::uint32_t cop0_software_interrupt_pending_ = 0;
  CpuAddress cop0_epc_ = 0;
  RdramOffset pi_dram_address_ = 0;
  PiCartAddress pi_cart_address_ = 0;
  std::uint32_t pi_cart_to_rdram_length_ = 0;
  std::uint32_t pi_status_ = 0;

  CpuAddress cpu_pc_ = kBlankInitialCpuPc;
  CpuAddress cpu_next_pc_ = kBlankInitialCpuNextPc;
  CpuRegisterValue cpu_hi_ = 0;
  CpuRegisterValue cpu_lo_ = 0;
  std::array<CpuRegisterValue, kCpuGprCount> cpu_gprs_{};
};

}  // namespace fn64
