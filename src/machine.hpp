#pragma once

#include <array>
#include <cstddef>
#include <cstdint>

#include "cartridge.hpp"

namespace fn64 {

class Machine {
public:
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

  enum class CpuInstructionStepResult {
    kStepped,
    kStopped,
    kUnsupported,
  };

  explicit Machine(Cartridge cartridge);

  bool powered_on() const;
  const Cartridge& cartridge() const;

  std::uint8_t read_rdram_u8(std::uint32_t address) const;
  std::uint16_t read_rdram_u16_be(std::uint32_t address) const;
  std::uint32_t read_rdram_u32_be(std::uint32_t address) const;

  void write_rdram_u8(std::uint32_t address, std::uint8_t value);
  void write_rdram_u16_be(std::uint32_t address, std::uint16_t value);
  void write_rdram_u32_be(std::uint32_t address, std::uint32_t value);

  std::uint32_t cpu_pc() const;
  std::uint32_t cpu_next_pc() const;
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
  static const char* cpu_instruction_identity_name(CpuInstructionIdentity identity);

  CpuInstructionExecutionResult execute_cpu_instruction(
      CpuInstructionIdentity identity,
      const DecodedCpuInstructionWord& instruction);

  CpuInstructionStepResult step_cpu_instruction();

private:
  static constexpr std::size_t kRdramSizeBytes = 4 * 1024 * 1024;
  static constexpr std::size_t kCpuGprCount = 32;

  Cartridge cartridge_;
  bool powered_on_ = true;
  std::array<std::uint8_t, kRdramSizeBytes> rdram_{};

  std::uint32_t cpu_pc_ = 0;
  std::uint32_t cpu_next_pc_ = 4;
  std::uint32_t cpu_hi_ = 0;
  std::uint32_t cpu_lo_ = 0;
  std::array<std::uint32_t, kCpuGprCount> cpu_gprs_{};
};

}  // namespace fn64