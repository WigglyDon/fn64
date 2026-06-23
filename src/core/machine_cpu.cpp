#include "machine.hpp"

#include <limits>
#include <stdexcept>
#include <string>

namespace fn64 {
namespace {

[[noreturn]] void fail_cpu_rdram_address(
    const char* operation,
    std::uint32_t cpu_address,
    std::size_t width) {
  throw std::out_of_range(
      std::string("RDRAM access out of range through CPU address: operation=") +
      operation +
      " address=" + std::to_string(cpu_address) +
      " width=" + std::to_string(width));
}

[[noreturn]] void fail_cpu_gpr_index(std::size_t index) {
  throw std::out_of_range("CPU GPR index out of range: " + std::to_string(index));
}

[[noreturn]] void fail_unaligned_instruction_fetch(std::uint32_t pc) {
  throw std::runtime_error("Unaligned CPU instruction fetch at PC " + std::to_string(pc));
}

[[noreturn]] void fail_unaligned_halfword_memory_access(
    const char* operation,
    std::uint32_t address) {
  throw std::runtime_error(
      std::string(operation) +
      " requires naturally aligned halfword address: " +
      std::to_string(address));
}

[[noreturn]] void fail_unaligned_word_memory_access(
    const char* operation,
    std::uint32_t address) {
  throw std::runtime_error(
      std::string(operation) +
      " requires naturally aligned word address: " +
      std::to_string(address));
}

[[noreturn]] void fail_unaligned_control_transfer_target(
    const char* operation,
    std::uint32_t address) {
  throw std::runtime_error(
      std::string(operation) +
      " requires naturally aligned control-transfer target: " +
      std::to_string(address));
}

std::uint8_t variable_shift_amount_u32(std::uint32_t value) {
  return static_cast<std::uint8_t>(value & 0x1fu);
}

std::uint32_t arithmetic_shift_right_u32(std::uint32_t value, std::uint8_t sa) {
  if (sa == 0) {
    return value;
  }

  const std::uint32_t shifted = value >> sa;
  if ((value & 0x80000000u) == 0) {
    return shifted;
  }

  const std::uint32_t fill_mask = 0xffffffffu << (32 - sa);
  return shifted | fill_mask;
}

std::int32_t i32_from_u32_bits(std::uint32_t value) {
  if ((value & 0x80000000u) == 0) {
    return static_cast<std::int32_t>(value);
  }

  const std::int64_t signed_value =
      static_cast<std::int64_t>(value) - 0x100000000ll;
  return static_cast<std::int32_t>(signed_value);
}

bool signed_i32_result_out_of_range(std::int64_t value) {
  return value < static_cast<std::int64_t>(std::numeric_limits<std::int32_t>::min()) ||
         value > static_cast<std::int64_t>(std::numeric_limits<std::int32_t>::max());
}

std::uint32_t u32_bits_from_i32_value(std::int64_t value) {
  if (value < 0) {
    return static_cast<std::uint32_t>(0x100000000ll + value);
  }

  return static_cast<std::uint32_t>(value);
}

std::int16_t i16_from_u16_bits(std::uint16_t value) {
  if ((value & 0x8000u) == 0) {
    return static_cast<std::int16_t>(value);
  }

  const std::int32_t signed_value =
      static_cast<std::int32_t>(value) - 0x10000;
  return static_cast<std::int16_t>(signed_value);
}

bool signed_register_value_greater_equal(std::uint32_t lhs, std::uint32_t rhs) {
  return i32_from_u32_bits(lhs) >= i32_from_u32_bits(rhs);
}

bool signed_register_value_less_than(std::uint32_t lhs, std::uint32_t rhs) {
  return i32_from_u32_bits(lhs) < i32_from_u32_bits(rhs);
}

bool signed_register_value_greater_equal_immediate(std::uint32_t lhs, std::int16_t rhs) {
  return i32_from_u32_bits(lhs) >= static_cast<std::int32_t>(rhs);
}

bool signed_register_value_less_than_immediate(std::uint32_t lhs, std::int16_t rhs) {
  return i32_from_u32_bits(lhs) < static_cast<std::int32_t>(rhs);
}

std::uint32_t sign_extend_u8_to_u32(std::uint8_t value) {
  if ((value & 0x80u) == 0) {
    return static_cast<std::uint32_t>(value);
  }

  return 0xffffff00u | static_cast<std::uint32_t>(value);
}

std::uint32_t sign_extend_u16_to_u32(std::uint16_t value) {
  if ((value & 0x8000u) == 0) {
    return static_cast<std::uint32_t>(value);
  }

  return 0xffff0000u | static_cast<std::uint32_t>(value);
}

std::uint32_t low_u32(std::uint64_t value) {
  return static_cast<std::uint32_t>(value & 0xffffffffull);
}

std::uint32_t high_u32(std::uint64_t value) {
  return static_cast<std::uint32_t>(value >> 32);
}

std::uint32_t sequential_instruction_address(std::uint32_t address) {
  return address + 4u;
}

std::uint32_t link_return_address(std::uint32_t current_pc) {
  return sequential_instruction_address(sequential_instruction_address(current_pc));
}

std::uint32_t jump_target_address(std::uint32_t current_pc, std::uint32_t jump_target) {
  const std::uint32_t next_sequential = sequential_instruction_address(current_pc);
  return (next_sequential & 0xf0000000u) | ((jump_target & 0x03ffffffu) << 2);
}

std::uint32_t branch_target_address(std::uint32_t current_pc, std::int16_t immediate) {
  const std::int32_t offset_bytes = static_cast<std::int32_t>(immediate) * 4;
  return sequential_instruction_address(current_pc) +
         static_cast<std::uint32_t>(offset_bytes);
}

void validate_control_transfer_target_alignment(
    const char* operation,
    std::uint32_t address) {
  if ((address & 0x3u) != 0) {
    fail_unaligned_control_transfer_target(operation, address);
  }
}

std::uint8_t u32_byte_be(std::uint32_t value, std::size_t byte_index) {
  const std::uint32_t shift = static_cast<std::uint32_t>((3u - byte_index) * 8u);
  return static_cast<std::uint8_t>((value >> shift) & 0xffu);
}

std::uint32_t replace_u32_byte_be(
    std::uint32_t value,
    std::size_t byte_index,
    std::uint8_t byte_value) {
  const std::uint32_t shift = static_cast<std::uint32_t>((3u - byte_index) * 8u);
  const std::uint32_t clear_mask = ~(0xffu << shift);
  return (value & clear_mask) |
         (static_cast<std::uint32_t>(byte_value) << shift);
}

}  // namespace

std::uint32_t Machine::require_cpu_rdram_address(
    const char* operation,
    std::uint32_t cpu_address,
    std::size_t width) {
  std::uint32_t rdram_address = 0;
  if (!translate_cpu_rdram_address(cpu_address, width, rdram_address)) {
    fail_cpu_rdram_address(operation, cpu_address, width);
  }

  return rdram_address;
}

std::uint8_t Machine::read_cpu_memory_u8(std::uint32_t cpu_address) const {
  return read_rdram_u8(require_cpu_rdram_address("CPU byte read", cpu_address, 1));
}

std::uint16_t Machine::read_cpu_memory_u16_be(std::uint32_t cpu_address) const {
  return read_rdram_u16_be(require_cpu_rdram_address("CPU halfword read", cpu_address, 2));
}

std::uint32_t Machine::read_cpu_memory_u32_be(std::uint32_t cpu_address) const {
  return read_rdram_u32_be(require_cpu_rdram_address("CPU word read", cpu_address, 4));
}

void Machine::write_cpu_memory_u8(std::uint32_t cpu_address, std::uint8_t value) {
  write_rdram_u8(require_cpu_rdram_address("CPU byte write", cpu_address, 1), value);
}

void Machine::write_cpu_memory_u16_be(std::uint32_t cpu_address, std::uint16_t value) {
  write_rdram_u16_be(require_cpu_rdram_address("CPU halfword write", cpu_address, 2), value);
}

void Machine::write_cpu_memory_u32_be(std::uint32_t cpu_address, std::uint32_t value) {
  write_rdram_u32_be(require_cpu_rdram_address("CPU word write", cpu_address, 4), value);
}

std::uint32_t Machine::cpu_pc() const {
  return cpu_pc_;
}

std::uint32_t Machine::cpu_next_pc() const {
  return cpu_next_pc_;
}

std::uint32_t Machine::cpu_hi() const {
  return cpu_hi_;
}

std::uint32_t Machine::cpu_lo() const {
  return cpu_lo_;
}

std::uint32_t Machine::read_cpu_gpr(std::size_t index) const {
  if (index >= cpu_gprs_.size()) {
    fail_cpu_gpr_index(index);
  }

  if (index == 0) {
    return 0;
  }

  return cpu_gprs_[index];
}

void Machine::write_cpu_pc(std::uint32_t value) {
  cpu_pc_ = value;
  cpu_next_pc_ = sequential_instruction_address(value);
}

void Machine::write_cpu_next_pc(std::uint32_t value) {
  cpu_next_pc_ = value;
}

void Machine::write_cpu_hi(std::uint32_t value) {
  cpu_hi_ = value;
}

void Machine::write_cpu_lo(std::uint32_t value) {
  cpu_lo_ = value;
}

void Machine::write_cpu_gpr(std::size_t index, std::uint32_t value) {
  if (index >= cpu_gprs_.size()) {
    fail_cpu_gpr_index(index);
  }

  if (index == 0) {
    return;
  }

  cpu_gprs_[index] = value;
}

std::uint32_t Machine::fetch_cpu_instruction_word() const {
  const std::uint32_t pc = cpu_pc();

  if ((pc & 0x3u) != 0) {
    fail_unaligned_instruction_fetch(pc);
  }

  const std::uint32_t rdram_address =
      require_cpu_rdram_address("CPU instruction fetch", pc, 4);
  return read_rdram_u32_be(rdram_address);
}

Machine::DecodedCpuInstructionWord Machine::decode_cpu_instruction_word(std::uint32_t raw) {
  DecodedCpuInstructionWord instruction;
  instruction.raw = raw;
  instruction.opcode = static_cast<std::uint8_t>((raw >> 26) & 0x3f);
  instruction.rs = static_cast<std::uint8_t>((raw >> 21) & 0x1f);
  instruction.rt = static_cast<std::uint8_t>((raw >> 16) & 0x1f);
  instruction.rd = static_cast<std::uint8_t>((raw >> 11) & 0x1f);
  instruction.sa = static_cast<std::uint8_t>((raw >> 6) & 0x1f);
  instruction.funct = static_cast<std::uint8_t>(raw & 0x3f);
  instruction.immediate_u16 = static_cast<std::uint16_t>(raw & 0xffff);
  instruction.immediate_i16 = i16_from_u16_bits(instruction.immediate_u16);
  instruction.jump_target = raw & 0x03ffffff;
  return instruction;
}

Machine::CpuInstructionIdentity Machine::identify_cpu_instruction(
    const DecodedCpuInstructionWord& instruction) {
  switch (instruction.opcode) {
    case 0x00:
      switch (instruction.funct) {
        case 0x00: return CpuInstructionIdentity::kSpecialSll;
        case 0x02: return CpuInstructionIdentity::kSpecialSrl;
        case 0x03: return CpuInstructionIdentity::kSpecialSra;
        case 0x04: return CpuInstructionIdentity::kSpecialSllv;
        case 0x06: return CpuInstructionIdentity::kSpecialSrlv;
        case 0x07: return CpuInstructionIdentity::kSpecialSrav;
        case 0x08: return CpuInstructionIdentity::kSpecialJr;
        case 0x09: return CpuInstructionIdentity::kSpecialJalr;
        case 0x0c: return CpuInstructionIdentity::kSpecialSyscall;
        case 0x0d: return CpuInstructionIdentity::kSpecialBreak;
        case 0x0f: return CpuInstructionIdentity::kSpecialSync;
        case 0x10: return CpuInstructionIdentity::kSpecialMfhi;
        case 0x11: return CpuInstructionIdentity::kSpecialMthi;
        case 0x12: return CpuInstructionIdentity::kSpecialMflo;
        case 0x13: return CpuInstructionIdentity::kSpecialMtlo;
        case 0x14: return CpuInstructionIdentity::kSpecialDsllv;
        case 0x16: return CpuInstructionIdentity::kSpecialDsrlv;
        case 0x17: return CpuInstructionIdentity::kSpecialDsrav;
        case 0x18: return CpuInstructionIdentity::kSpecialMult;
        case 0x19: return CpuInstructionIdentity::kSpecialMultu;
        case 0x1a: return CpuInstructionIdentity::kSpecialDiv;
        case 0x1b: return CpuInstructionIdentity::kSpecialDivu;
        case 0x1c: return CpuInstructionIdentity::kSpecialDmult;
        case 0x1d: return CpuInstructionIdentity::kSpecialDmultu;
        case 0x1e: return CpuInstructionIdentity::kSpecialDdiv;
        case 0x1f: return CpuInstructionIdentity::kSpecialDdivu;
        case 0x20: return CpuInstructionIdentity::kSpecialAdd;
        case 0x21: return CpuInstructionIdentity::kSpecialAddu;
        case 0x22: return CpuInstructionIdentity::kSpecialSub;
        case 0x23: return CpuInstructionIdentity::kSpecialSubu;
        case 0x24: return CpuInstructionIdentity::kSpecialAnd;
        case 0x25: return CpuInstructionIdentity::kSpecialOr;
        case 0x26: return CpuInstructionIdentity::kSpecialXor;
        case 0x27: return CpuInstructionIdentity::kSpecialNor;
        case 0x2a: return CpuInstructionIdentity::kSpecialSlt;
        case 0x2b: return CpuInstructionIdentity::kSpecialSltu;
        case 0x2c: return CpuInstructionIdentity::kSpecialDadd;
        case 0x2d: return CpuInstructionIdentity::kSpecialDaddu;
        case 0x2e: return CpuInstructionIdentity::kSpecialDsub;
        case 0x2f: return CpuInstructionIdentity::kSpecialDsubu;
        case 0x30: return CpuInstructionIdentity::kSpecialTge;
        case 0x31: return CpuInstructionIdentity::kSpecialTgeu;
        case 0x32: return CpuInstructionIdentity::kSpecialTlt;
        case 0x33: return CpuInstructionIdentity::kSpecialTltu;
        case 0x34: return CpuInstructionIdentity::kSpecialTeq;
        case 0x36: return CpuInstructionIdentity::kSpecialTne;
        case 0x38: return CpuInstructionIdentity::kSpecialDsll;
        case 0x3a: return CpuInstructionIdentity::kSpecialDsrl;
        case 0x3b: return CpuInstructionIdentity::kSpecialDsra;
        case 0x3c: return CpuInstructionIdentity::kSpecialDsll32;
        case 0x3e: return CpuInstructionIdentity::kSpecialDsrl32;
        case 0x3f: return CpuInstructionIdentity::kSpecialDsra32;
        default: return CpuInstructionIdentity::kSpecialUnknown;
      }

    case 0x01:
      switch (instruction.rt) {
        case 0x00: return CpuInstructionIdentity::kRegimmBltz;
        case 0x01: return CpuInstructionIdentity::kRegimmBgez;
        case 0x02: return CpuInstructionIdentity::kRegimmBltzl;
        case 0x03: return CpuInstructionIdentity::kRegimmBgezl;
        case 0x08: return CpuInstructionIdentity::kRegimmTgei;
        case 0x09: return CpuInstructionIdentity::kRegimmTgeiu;
        case 0x0a: return CpuInstructionIdentity::kRegimmTlti;
        case 0x0b: return CpuInstructionIdentity::kRegimmTltiu;
        case 0x0c: return CpuInstructionIdentity::kRegimmTeqi;
        case 0x0e: return CpuInstructionIdentity::kRegimmTnei;
        case 0x10: return CpuInstructionIdentity::kRegimmBltzal;
        case 0x11: return CpuInstructionIdentity::kRegimmBgezal;
        case 0x12: return CpuInstructionIdentity::kRegimmBltzall;
        case 0x13: return CpuInstructionIdentity::kRegimmBgezall;
        default: return CpuInstructionIdentity::kRegimmUnknown;
      }

    case 0x02: return CpuInstructionIdentity::kJ;
    case 0x03: return CpuInstructionIdentity::kJal;
    case 0x04: return CpuInstructionIdentity::kBeq;
    case 0x05: return CpuInstructionIdentity::kBne;
    case 0x06: return CpuInstructionIdentity::kBlez;
    case 0x07: return CpuInstructionIdentity::kBgtz;
    case 0x08: return CpuInstructionIdentity::kAddi;
    case 0x09: return CpuInstructionIdentity::kAddiu;
    case 0x0a: return CpuInstructionIdentity::kSlti;
    case 0x0b: return CpuInstructionIdentity::kSltiu;
    case 0x0c: return CpuInstructionIdentity::kAndi;
    case 0x0d: return CpuInstructionIdentity::kOri;
    case 0x0e: return CpuInstructionIdentity::kXori;
    case 0x0f: return CpuInstructionIdentity::kLui;
    case 0x10: return CpuInstructionIdentity::kCop0;
    case 0x11: return CpuInstructionIdentity::kCop1;
    case 0x12: return CpuInstructionIdentity::kCop2;
    case 0x13: return CpuInstructionIdentity::kCop3;
    case 0x14: return CpuInstructionIdentity::kBeql;
    case 0x15: return CpuInstructionIdentity::kBnel;
    case 0x16: return CpuInstructionIdentity::kBlezl;
    case 0x17: return CpuInstructionIdentity::kBgtzl;
    case 0x18: return CpuInstructionIdentity::kDaddi;
    case 0x19: return CpuInstructionIdentity::kDaddiu;
    case 0x1a: return CpuInstructionIdentity::kLdl;
    case 0x1b: return CpuInstructionIdentity::kLdr;
    case 0x20: return CpuInstructionIdentity::kLb;
    case 0x21: return CpuInstructionIdentity::kLh;
    case 0x22: return CpuInstructionIdentity::kLwl;
    case 0x23: return CpuInstructionIdentity::kLw;
    case 0x24: return CpuInstructionIdentity::kLbu;
    case 0x25: return CpuInstructionIdentity::kLhu;
    case 0x26: return CpuInstructionIdentity::kLwr;
    case 0x27: return CpuInstructionIdentity::kLwu;
    case 0x28: return CpuInstructionIdentity::kSb;
    case 0x29: return CpuInstructionIdentity::kSh;
    case 0x2a: return CpuInstructionIdentity::kSwl;
    case 0x2b: return CpuInstructionIdentity::kSw;
    case 0x2c: return CpuInstructionIdentity::kSdl;
    case 0x2d: return CpuInstructionIdentity::kSdr;
    case 0x2e: return CpuInstructionIdentity::kSwr;
    case 0x2f: return CpuInstructionIdentity::kCache;
    case 0x30: return CpuInstructionIdentity::kLl;
    case 0x31: return CpuInstructionIdentity::kLwc1;
    case 0x32: return CpuInstructionIdentity::kLwc2;
    case 0x34: return CpuInstructionIdentity::kLld;
    case 0x35: return CpuInstructionIdentity::kLdc1;
    case 0x36: return CpuInstructionIdentity::kLdc2;
    case 0x37: return CpuInstructionIdentity::kLd;
    case 0x38: return CpuInstructionIdentity::kSc;
    case 0x39: return CpuInstructionIdentity::kSwc1;
    case 0x3a: return CpuInstructionIdentity::kSwc2;
    case 0x3c: return CpuInstructionIdentity::kScd;
    case 0x3d: return CpuInstructionIdentity::kSdc1;
    case 0x3e: return CpuInstructionIdentity::kSdc2;
    case 0x3f: return CpuInstructionIdentity::kSd;
    default: return CpuInstructionIdentity::kUnknownPrimary;
  }
}

Machine::CpuInstructionExecutionResult Machine::execute_cpu_instruction(
    CpuInstructionIdentity identity,
    const DecodedCpuInstructionWord& instruction) {
  const auto trap_execution_result = [](bool trap_taken) {
    return trap_taken ? CpuInstructionExecutionResult::kStopped
                      : CpuInstructionExecutionResult::kExecuted;
  };

  switch (identity) {
    case CpuInstructionIdentity::kSpecialSll: {
      const std::uint32_t value = read_cpu_gpr(instruction.rt) << instruction.sa;
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSrl: {
      const std::uint32_t value = read_cpu_gpr(instruction.rt) >> instruction.sa;
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSra: {
      const std::uint32_t value =
          arithmetic_shift_right_u32(read_cpu_gpr(instruction.rt), instruction.sa);
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSllv: {
      const std::uint8_t sa =
          variable_shift_amount_u32(read_cpu_gpr(instruction.rs));
      const std::uint32_t value = read_cpu_gpr(instruction.rt) << sa;
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSrlv: {
      const std::uint8_t sa =
          variable_shift_amount_u32(read_cpu_gpr(instruction.rs));
      const std::uint32_t value = read_cpu_gpr(instruction.rt) >> sa;
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSrav: {
      const std::uint8_t sa =
          variable_shift_amount_u32(read_cpu_gpr(instruction.rs));
      const std::uint32_t value =
          arithmetic_shift_right_u32(read_cpu_gpr(instruction.rt), sa);
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialJr: {
      const std::uint32_t target = read_cpu_gpr(instruction.rs);
      validate_control_transfer_target_alignment("JR", target);
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialJalr: {
      const std::uint32_t target = read_cpu_gpr(instruction.rs);
      validate_control_transfer_target_alignment("JALR", target);
      write_cpu_gpr(instruction.rd, link_return_address(cpu_pc()));
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSyscall:
      return CpuInstructionExecutionResult::kStopped;

    case CpuInstructionIdentity::kSpecialBreak:
      return CpuInstructionExecutionResult::kStopped;

    case CpuInstructionIdentity::kSpecialSync:
      return CpuInstructionExecutionResult::kExecuted;

    case CpuInstructionIdentity::kSpecialMfhi: {
      write_cpu_gpr(instruction.rd, cpu_hi());
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMthi: {
      write_cpu_hi(read_cpu_gpr(instruction.rs));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMflo: {
      write_cpu_gpr(instruction.rd, cpu_lo());
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMtlo: {
      write_cpu_lo(read_cpu_gpr(instruction.rs));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMult: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rs)));
      const std::int64_t rhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rt)));
      const std::uint64_t product = static_cast<std::uint64_t>(lhs * rhs);
      write_cpu_lo(low_u32(product));
      write_cpu_hi(high_u32(product));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMultu: {
      const std::uint64_t lhs = static_cast<std::uint64_t>(read_cpu_gpr(instruction.rs));
      const std::uint64_t rhs = static_cast<std::uint64_t>(read_cpu_gpr(instruction.rt));
      const std::uint64_t product = lhs * rhs;
      write_cpu_lo(low_u32(product));
      write_cpu_hi(high_u32(product));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDiv: {
      const std::uint32_t divisor_u32 = read_cpu_gpr(instruction.rt);
      if (divisor_u32 == 0) {
        return CpuInstructionExecutionResult::kExecuted;
      }

      const std::int64_t dividend =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rs)));
      const std::int64_t divisor =
          static_cast<std::int64_t>(i32_from_u32_bits(divisor_u32));
      const std::int64_t quotient = dividend / divisor;
      const std::int64_t remainder = dividend % divisor;
      write_cpu_lo(static_cast<std::uint32_t>(quotient));
      write_cpu_hi(static_cast<std::uint32_t>(remainder));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDivu: {
      const std::uint32_t divisor = read_cpu_gpr(instruction.rt);
      if (divisor == 0) {
        return CpuInstructionExecutionResult::kExecuted;
      }

      const std::uint32_t dividend = read_cpu_gpr(instruction.rs);
      write_cpu_lo(dividend / divisor);
      write_cpu_hi(dividend % divisor);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialTge:
      return trap_execution_result(
          signed_register_value_greater_equal(
              read_cpu_gpr(instruction.rs),
              read_cpu_gpr(instruction.rt)));

    case CpuInstructionIdentity::kSpecialTgeu:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) >= read_cpu_gpr(instruction.rt));

    case CpuInstructionIdentity::kSpecialTlt:
      return trap_execution_result(
          signed_register_value_less_than(
              read_cpu_gpr(instruction.rs),
              read_cpu_gpr(instruction.rt)));

    case CpuInstructionIdentity::kSpecialTltu:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) < read_cpu_gpr(instruction.rt));

    case CpuInstructionIdentity::kSpecialTeq:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) == read_cpu_gpr(instruction.rt));

    case CpuInstructionIdentity::kSpecialTne:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) != read_cpu_gpr(instruction.rt));

    case CpuInstructionIdentity::kRegimmTgei:
      return trap_execution_result(
          signed_register_value_greater_equal_immediate(
              read_cpu_gpr(instruction.rs),
              instruction.immediate_i16));

    case CpuInstructionIdentity::kRegimmTgeiu:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) >=
          sign_extend_u16_to_u32(instruction.immediate_u16));

    case CpuInstructionIdentity::kRegimmTlti:
      return trap_execution_result(
          signed_register_value_less_than_immediate(
              read_cpu_gpr(instruction.rs),
              instruction.immediate_i16));

    case CpuInstructionIdentity::kRegimmTltiu:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) <
          sign_extend_u16_to_u32(instruction.immediate_u16));

    case CpuInstructionIdentity::kRegimmTeqi:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) ==
          sign_extend_u16_to_u32(instruction.immediate_u16));

    case CpuInstructionIdentity::kRegimmTnei:
      return trap_execution_result(
          read_cpu_gpr(instruction.rs) !=
          sign_extend_u16_to_u32(instruction.immediate_u16));

    case CpuInstructionIdentity::kRegimmBltz: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value < 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBgez: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value >= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBltzl: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value < 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kRegimmBgezl: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value >= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kRegimmBltzal: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      write_cpu_gpr(31, link_return_address(cpu_pc()));
      if (value < 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBgezal: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      write_cpu_gpr(31, link_return_address(cpu_pc()));
      if (value >= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBltzall: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value < 0) {
        write_cpu_gpr(31, link_return_address(cpu_pc()));
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kRegimmBgezall: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value >= 0) {
        write_cpu_gpr(31, link_return_address(cpu_pc()));
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kJ: {
      const std::uint32_t target = jump_target_address(cpu_pc(), instruction.jump_target);
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kJal: {
      write_cpu_gpr(31, link_return_address(cpu_pc()));
      const std::uint32_t target = jump_target_address(cpu_pc(), instruction.jump_target);
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBeq: {
      if (read_cpu_gpr(instruction.rs) == read_cpu_gpr(instruction.rt)) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBne: {
      if (read_cpu_gpr(instruction.rs) != read_cpu_gpr(instruction.rt)) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBlez: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value <= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBgtz: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value > 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBeql: {
      if (read_cpu_gpr(instruction.rs) == read_cpu_gpr(instruction.rt)) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kBnel: {
      if (read_cpu_gpr(instruction.rs) != read_cpu_gpr(instruction.rt)) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kBlezl: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value <= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kBgtzl: {
      const std::int32_t value = i32_from_u32_bits(read_cpu_gpr(instruction.rs));
      if (value > 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kSpecialAdd: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rs)));
      const std::int64_t rhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rt)));
      const std::int64_t value = lhs + rhs;

      if (signed_i32_result_out_of_range(value)) {
        throw std::runtime_error("ADD overflow");
      }

      write_cpu_gpr(instruction.rd, u32_bits_from_i32_value(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialAddu: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) + read_cpu_gpr(instruction.rt);
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSub: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rs)));
      const std::int64_t rhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rt)));
      const std::int64_t value = lhs - rhs;

      if (signed_i32_result_out_of_range(value)) {
        throw std::runtime_error("SUB overflow");
      }

      write_cpu_gpr(instruction.rd, u32_bits_from_i32_value(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSubu: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) - read_cpu_gpr(instruction.rt);
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialAnd: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) & read_cpu_gpr(instruction.rt);
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialOr: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) | read_cpu_gpr(instruction.rt);
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialXor: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) ^ read_cpu_gpr(instruction.rt);
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialNor: {
      const std::uint32_t value =
          ~(read_cpu_gpr(instruction.rs) | read_cpu_gpr(instruction.rt));
      write_cpu_gpr(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSlt: {
      const bool value =
          i32_from_u32_bits(read_cpu_gpr(instruction.rs)) <
          i32_from_u32_bits(read_cpu_gpr(instruction.rt));
      write_cpu_gpr(instruction.rd, value ? 1u : 0u);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSltu: {
      const bool value =
          read_cpu_gpr(instruction.rs) < read_cpu_gpr(instruction.rt);
      write_cpu_gpr(instruction.rd, value ? 1u : 0u);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kAddi: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr(instruction.rs)));
      const std::int64_t rhs = static_cast<std::int64_t>(instruction.immediate_i16);
      const std::int64_t value = lhs + rhs;

      if (signed_i32_result_out_of_range(value)) {
        throw std::runtime_error("ADDI overflow");
      }

      write_cpu_gpr(instruction.rt, u32_bits_from_i32_value(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kAddiu: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSlti: {
      const bool value =
          i32_from_u32_bits(read_cpu_gpr(instruction.rs)) <
          static_cast<std::int32_t>(instruction.immediate_i16);
      write_cpu_gpr(instruction.rt, value ? 1u : 0u);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSltiu: {
      const bool value =
          read_cpu_gpr(instruction.rs) <
          sign_extend_u16_to_u32(instruction.immediate_u16);
      write_cpu_gpr(instruction.rt, value ? 1u : 0u);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kAndi: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) &
          static_cast<std::uint32_t>(instruction.immediate_u16);
      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kOri: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) |
          static_cast<std::uint32_t>(instruction.immediate_u16);
      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kXori: {
      const std::uint32_t value =
          read_cpu_gpr(instruction.rs) ^
          static_cast<std::uint32_t>(instruction.immediate_u16);
      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLui: {
      const std::uint32_t value =
          static_cast<std::uint32_t>(instruction.immediate_u16) << 16;
      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLb: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint8_t value = read_cpu_memory_u8(effective_address);
      write_cpu_gpr(instruction.rt, sign_extend_u8_to_u32(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLbu: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint8_t value = read_cpu_memory_u8(effective_address);
      write_cpu_gpr(instruction.rt, static_cast<std::uint32_t>(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLh: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x1u) != 0) {
        fail_unaligned_halfword_memory_access("LH", effective_address);
      }

      const std::uint16_t value = read_cpu_memory_u16_be(effective_address);
      write_cpu_gpr(instruction.rt, sign_extend_u16_to_u32(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLhu: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x1u) != 0) {
        fail_unaligned_halfword_memory_access("LHU", effective_address);
      }

      const std::uint16_t value = read_cpu_memory_u16_be(effective_address);
      write_cpu_gpr(instruction.rt, static_cast<std::uint32_t>(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLwl: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = 4u - byte_offset;

      std::uint32_t value = read_cpu_gpr(instruction.rt);
      for (std::uint32_t i = 0; i < byte_count; ++i) {
        const std::uint8_t memory_byte = read_cpu_memory_u8(effective_address + i);
        value = replace_u32_byte_be(value, static_cast<std::size_t>(i), memory_byte);
      }

      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLw: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x3u) != 0) {
        fail_unaligned_word_memory_access("LW", effective_address);
      }

      const std::uint32_t value = read_cpu_memory_u32_be(effective_address);
      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLwr: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t aligned_address = effective_address & ~0x3u;
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = byte_offset + 1u;
      const std::uint32_t first_register_byte = 4u - byte_count;

      std::uint32_t value = read_cpu_gpr(instruction.rt);
      for (std::uint32_t i = 0; i < byte_count; ++i) {
        const std::uint8_t memory_byte = read_cpu_memory_u8(aligned_address + i);
        value = replace_u32_byte_be(
            value,
            static_cast<std::size_t>(first_register_byte + i),
            memory_byte);
      }

      write_cpu_gpr(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSb: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint8_t value =
          static_cast<std::uint8_t>(read_cpu_gpr(instruction.rt) & 0xffu);
      write_cpu_memory_u8(effective_address, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSh: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x1u) != 0) {
        fail_unaligned_halfword_memory_access("SH", effective_address);
      }

      const std::uint16_t value =
          static_cast<std::uint16_t>(read_cpu_gpr(instruction.rt) & 0xffffu);
      write_cpu_memory_u16_be(effective_address, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSwl: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = 4u - byte_offset;
      const std::uint32_t value = read_cpu_gpr(instruction.rt);

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        static_cast<void>(read_cpu_memory_u8(effective_address + i));
      }

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        write_cpu_memory_u8(
            effective_address + i,
            u32_byte_be(value, static_cast<std::size_t>(i)));
      }

      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSw: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x3u) != 0) {
        fail_unaligned_word_memory_access("SW", effective_address);
      }

      write_cpu_memory_u32_be(effective_address, read_cpu_gpr(instruction.rt));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSwr: {
      const std::uint32_t effective_address =
          read_cpu_gpr(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t aligned_address = effective_address & ~0x3u;
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = byte_offset + 1u;
      const std::uint32_t first_register_byte = 4u - byte_count;
      const std::uint32_t value = read_cpu_gpr(instruction.rt);

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        static_cast<void>(read_cpu_memory_u8(aligned_address + i));
      }

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        write_cpu_memory_u8(
            aligned_address + i,
            u32_byte_be(value, static_cast<std::size_t>(first_register_byte + i)));
      }

      return CpuInstructionExecutionResult::kExecuted;
    }

    default:
      return CpuInstructionExecutionResult::kUnsupported;
  }
}

Machine::CpuInstructionStepResult Machine::step_cpu_instruction() {
  const std::uint32_t current_pc = cpu_pc_;
  const std::uint32_t current_next_pc = cpu_next_pc_;

  const std::uint32_t raw = fetch_cpu_instruction_word();
  const DecodedCpuInstructionWord instruction = decode_cpu_instruction_word(raw);
  const CpuInstructionIdentity identity = identify_cpu_instruction(instruction);

  cpu_next_pc_ = sequential_instruction_address(current_next_pc);

  CpuInstructionExecutionResult execution_result = CpuInstructionExecutionResult::kUnsupported;
  try {
    execution_result = execute_cpu_instruction(identity, instruction);
  } catch (...) {
    cpu_pc_ = current_pc;
    cpu_next_pc_ = current_next_pc;
    throw;
  }

  if (execution_result == CpuInstructionExecutionResult::kUnsupported) {
    cpu_pc_ = current_pc;
    cpu_next_pc_ = current_next_pc;
    return CpuInstructionStepResult::kUnsupported;
  }

  if (execution_result == CpuInstructionExecutionResult::kBranchLikelyNotTaken) {
    const std::uint32_t skipped_delay_slot_pc = cpu_next_pc_;
    cpu_pc_ = skipped_delay_slot_pc;
    cpu_next_pc_ = sequential_instruction_address(skipped_delay_slot_pc);
    return CpuInstructionStepResult::kStepped;
  }

  cpu_pc_ = current_next_pc;

  if (execution_result == CpuInstructionExecutionResult::kStopped) {
    return CpuInstructionStepResult::kStopped;
  }

  return CpuInstructionStepResult::kStepped;
}

}  // namespace fn64
