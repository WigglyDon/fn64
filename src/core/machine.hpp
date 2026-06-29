#pragma once

#include <array>
#include <cstddef>
#include <cstdint>

#include "cartridge.hpp"

namespace fn64 {

class Machine {
public:
  // Public CPU execution result for fn64's current local step policy.
  // kStopped is a local stop condition, not N64 COP0 exception delivery.
  // kUnsupported is a non-compatibility result for unknown or unsupported
  // instructions; proof-backed unsupported paths roll back visible step state.
  // Local core/precondition/fault failures throw standard C++ exceptions today.
  enum class CpuInstructionStepResult {
    // A local instruction step completed and committed current pc/next_pc movement.
    kStepped,
    // A local stop instruction/condition completed and committed step PC movement.
    kStopped,
    // An unknown or unsupported instruction was reported without committing a step.
    kUnsupported,
  };

  explicit Machine(Cartridge cartridge);

  static constexpr std::uint32_t kBlankInitialCpuPc = 0x00000000u;
  static constexpr std::uint32_t kBlankInitialCpuNextPc = 0x00000004u;

  bool powered_on() const;
  const Cartridge& cartridge() const;
  std::size_t rdram_size_bytes() const noexcept;

  std::uint32_t inspect_rdram_u32_be(std::uint32_t address) const;

  void stage_rdram_u32_be(std::uint32_t address, std::uint32_t value);

  void stage_cartridge_bytes_to_rdram(
      std::uint32_t cartridge_offset,
      std::uint32_t rdram_address,
      std::uint32_t byte_count);

  std::uint32_t cpu_pc() const;
  std::uint32_t cpu_next_pc() const;
  std::uint32_t inspect_cpu_hi() const;
  std::uint32_t inspect_cpu_lo() const;
  std::uint32_t inspect_cpu_gpr(std::size_t index) const;

  void stage_cpu_pc(std::uint32_t value);
  void stage_cpu_next_pc(std::uint32_t value);
  void stage_cpu_hi(std::uint32_t value);
  void stage_cpu_lo(std::uint32_t value);
  void stage_cpu_gpr(std::size_t index, std::uint32_t value);

  // Public CPU execution creation point. A completed step commits this
  // Machine's current pc/next_pc movement. Thrown faults are local fn64
  // failures, not modeled N64 exception paths; no-ghost rollback is only
  // claimed for paths covered by the proof suite.
  CpuInstructionStepResult step_cpu_instruction();

private:
  struct DecodedCpuInstructionWord {
    std::uint32_t raw = 0;
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
  static constexpr std::size_t kCpuGprCount = 32;

  void reset_to_blank_rdram_power_on_state();

  std::uint8_t read_rdram_u8(std::uint32_t address) const;
  std::uint16_t read_rdram_u16_be(std::uint32_t address) const;
  std::uint32_t read_rdram_u32_be(std::uint32_t address) const;

  void write_rdram_u8(std::uint32_t address, std::uint8_t value);
  void write_rdram_u16_be(std::uint32_t address, std::uint16_t value);
  void write_rdram_u32_be(std::uint32_t address, std::uint32_t value);

  static std::uint32_t require_cpu_rdram_address(
      const char* operation,
      std::uint32_t cpu_address,
      std::size_t width);

  std::uint8_t read_cpu_memory_u8(std::uint32_t cpu_address) const;
  std::uint16_t read_cpu_memory_u16_be(std::uint32_t cpu_address) const;
  std::uint32_t read_cpu_memory_u32_be(std::uint32_t cpu_address) const;

  void write_cpu_memory_u8(std::uint32_t cpu_address, std::uint8_t value);
  void write_cpu_memory_u16_be(std::uint32_t cpu_address, std::uint16_t value);
  void write_cpu_memory_u32_be(std::uint32_t cpu_address, std::uint32_t value);

  static bool translate_cpu_rdram_address(
      std::uint32_t cpu_address,
      std::size_t width,
      std::uint32_t& out_rdram_address) noexcept;

  std::uint32_t cpu_hi() const;
  std::uint32_t cpu_lo() const;
  std::uint32_t read_cpu_gpr(std::size_t index) const;

  void write_cpu_pc(std::uint32_t value);
  void write_cpu_next_pc(std::uint32_t value);
  void write_cpu_hi(std::uint32_t value);
  void write_cpu_lo(std::uint32_t value);
  void write_cpu_gpr(std::size_t index, std::uint32_t value);

  std::uint32_t fetch_cpu_instruction_word() const;

  static DecodedCpuInstructionWord decode_cpu_instruction_word(std::uint32_t raw);
  static CpuInstructionIdentity identify_cpu_instruction(
      const DecodedCpuInstructionWord& instruction);

  CpuInstructionExecutionResult execute_cpu_instruction(
      CpuInstructionIdentity identity,
      const DecodedCpuInstructionWord& instruction);

  Cartridge cartridge_;
  bool powered_on_ = false;
  std::array<std::uint8_t, kRdramSizeBytes> rdram_{};

  std::uint32_t cpu_pc_ = kBlankInitialCpuPc;
  std::uint32_t cpu_next_pc_ = kBlankInitialCpuNextPc;
  std::uint32_t cpu_hi_ = 0;
  std::uint32_t cpu_lo_ = 0;
  std::array<std::uint32_t, kCpuGprCount> cpu_gprs_{};
};

}  // namespace fn64
