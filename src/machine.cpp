#include "machine.hpp"

#include <stdexcept>
#include <string>
#include <utility>

namespace fn64 {
namespace {

[[noreturn]] void fail_rdram_access(std::uint32_t address, std::size_t width) {
  throw std::out_of_range(
      "RDRAM access out of range: address=" + std::to_string(address) +
      " width=" + std::to_string(width));
}

[[noreturn]] void fail_cpu_gpr_index(std::size_t index) {
  throw std::out_of_range("CPU GPR index out of range: " + std::to_string(index));
}

[[noreturn]] void fail_unaligned_instruction_fetch(std::uint32_t pc) {
  throw std::runtime_error("Unaligned CPU instruction fetch at PC " + std::to_string(pc));
}

}  // namespace

Machine::Machine(Cartridge cartridge)
    : cartridge_(std::move(cartridge)) {}

bool Machine::powered_on() const {
  return powered_on_;
}

const Cartridge& Machine::cartridge() const {
  return cartridge_;
}

std::uint8_t Machine::read_rdram_u8(std::uint32_t address) const {
  if (address >= rdram_.size()) {
    fail_rdram_access(address, 1);
  }

  return rdram_[address];
}

std::uint16_t Machine::read_rdram_u16_be(std::uint32_t address) const {
  if (address > rdram_.size() - 2) {
    fail_rdram_access(address, 2);
  }

  return static_cast<std::uint16_t>(
      (static_cast<std::uint16_t>(rdram_[address]) << 8) |
      static_cast<std::uint16_t>(rdram_[address + 1]));
}

std::uint32_t Machine::read_rdram_u32_be(std::uint32_t address) const {
  if (address > rdram_.size() - 4) {
    fail_rdram_access(address, 4);
  }

  return (static_cast<std::uint32_t>(rdram_[address]) << 24) |
         (static_cast<std::uint32_t>(rdram_[address + 1]) << 16) |
         (static_cast<std::uint32_t>(rdram_[address + 2]) << 8) |
         static_cast<std::uint32_t>(rdram_[address + 3]);
}

void Machine::write_rdram_u8(std::uint32_t address, std::uint8_t value) {
  if (address >= rdram_.size()) {
    fail_rdram_access(address, 1);
  }

  rdram_[address] = value;
}

void Machine::write_rdram_u16_be(std::uint32_t address, std::uint16_t value) {
  if (address > rdram_.size() - 2) {
    fail_rdram_access(address, 2);
  }

  rdram_[address] = static_cast<std::uint8_t>((value >> 8) & 0xff);
  rdram_[address + 1] = static_cast<std::uint8_t>(value & 0xff);
}

void Machine::write_rdram_u32_be(std::uint32_t address, std::uint32_t value) {
  if (address > rdram_.size() - 4) {
    fail_rdram_access(address, 4);
  }

  rdram_[address] = static_cast<std::uint8_t>((value >> 24) & 0xff);
  rdram_[address + 1] = static_cast<std::uint8_t>((value >> 16) & 0xff);
  rdram_[address + 2] = static_cast<std::uint8_t>((value >> 8) & 0xff);
  rdram_[address + 3] = static_cast<std::uint8_t>(value & 0xff);
}

std::uint32_t Machine::cpu_pc() const {
  return cpu_pc_;
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

  return read_rdram_u32_be(pc);
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
  instruction.immediate_i16 = static_cast<std::int16_t>(raw & 0xffff);
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

const char* Machine::cpu_instruction_identity_name(CpuInstructionIdentity identity) {
  switch (identity) {
    case CpuInstructionIdentity::kUnknownPrimary: return "unknown_primary";
    case CpuInstructionIdentity::kSpecialUnknown: return "special_unknown";
    case CpuInstructionIdentity::kRegimmUnknown: return "regimm_unknown";

    case CpuInstructionIdentity::kSpecialSll: return "special_sll";
    case CpuInstructionIdentity::kSpecialSrl: return "special_srl";
    case CpuInstructionIdentity::kSpecialSra: return "special_sra";
    case CpuInstructionIdentity::kSpecialSllv: return "special_sllv";
    case CpuInstructionIdentity::kSpecialSrlv: return "special_srlv";
    case CpuInstructionIdentity::kSpecialSrav: return "special_srav";
    case CpuInstructionIdentity::kSpecialJr: return "special_jr";
    case CpuInstructionIdentity::kSpecialJalr: return "special_jalr";
    case CpuInstructionIdentity::kSpecialSyscall: return "special_syscall";
    case CpuInstructionIdentity::kSpecialBreak: return "special_break";
    case CpuInstructionIdentity::kSpecialSync: return "special_sync";
    case CpuInstructionIdentity::kSpecialMfhi: return "special_mfhi";
    case CpuInstructionIdentity::kSpecialMthi: return "special_mthi";
    case CpuInstructionIdentity::kSpecialMflo: return "special_mflo";
    case CpuInstructionIdentity::kSpecialMtlo: return "special_mtlo";
    case CpuInstructionIdentity::kSpecialDsllv: return "special_dsllv";
    case CpuInstructionIdentity::kSpecialDsrlv: return "special_dsrlv";
    case CpuInstructionIdentity::kSpecialDsrav: return "special_dsrav";
    case CpuInstructionIdentity::kSpecialMult: return "special_mult";
    case CpuInstructionIdentity::kSpecialMultu: return "special_multu";
    case CpuInstructionIdentity::kSpecialDiv: return "special_div";
    case CpuInstructionIdentity::kSpecialDivu: return "special_divu";
    case CpuInstructionIdentity::kSpecialDmult: return "special_dmult";
    case CpuInstructionIdentity::kSpecialDmultu: return "special_dmultu";
    case CpuInstructionIdentity::kSpecialDdiv: return "special_ddiv";
    case CpuInstructionIdentity::kSpecialDdivu: return "special_ddivu";
    case CpuInstructionIdentity::kSpecialAdd: return "special_add";
    case CpuInstructionIdentity::kSpecialAddu: return "special_addu";
    case CpuInstructionIdentity::kSpecialSub: return "special_sub";
    case CpuInstructionIdentity::kSpecialSubu: return "special_subu";
    case CpuInstructionIdentity::kSpecialAnd: return "special_and";
    case CpuInstructionIdentity::kSpecialOr: return "special_or";
    case CpuInstructionIdentity::kSpecialXor: return "special_xor";
    case CpuInstructionIdentity::kSpecialNor: return "special_nor";
    case CpuInstructionIdentity::kSpecialSlt: return "special_slt";
    case CpuInstructionIdentity::kSpecialSltu: return "special_sltu";
    case CpuInstructionIdentity::kSpecialDadd: return "special_dadd";
    case CpuInstructionIdentity::kSpecialDaddu: return "special_daddu";
    case CpuInstructionIdentity::kSpecialDsub: return "special_dsub";
    case CpuInstructionIdentity::kSpecialDsubu: return "special_dsubu";
    case CpuInstructionIdentity::kSpecialTge: return "special_tge";
    case CpuInstructionIdentity::kSpecialTgeu: return "special_tgeu";
    case CpuInstructionIdentity::kSpecialTlt: return "special_tlt";
    case CpuInstructionIdentity::kSpecialTltu: return "special_tltu";
    case CpuInstructionIdentity::kSpecialTeq: return "special_teq";
    case CpuInstructionIdentity::kSpecialTne: return "special_tne";
    case CpuInstructionIdentity::kSpecialDsll: return "special_dsll";
    case CpuInstructionIdentity::kSpecialDsrl: return "special_dsrl";
    case CpuInstructionIdentity::kSpecialDsra: return "special_dsra";
    case CpuInstructionIdentity::kSpecialDsll32: return "special_dsll32";
    case CpuInstructionIdentity::kSpecialDsrl32: return "special_dsrl32";
    case CpuInstructionIdentity::kSpecialDsra32: return "special_dsra32";

    case CpuInstructionIdentity::kRegimmBltz: return "regimm_bltz";
    case CpuInstructionIdentity::kRegimmBgez: return "regimm_bgez";
    case CpuInstructionIdentity::kRegimmBltzl: return "regimm_bltzl";
    case CpuInstructionIdentity::kRegimmBgezl: return "regimm_bgezl";
    case CpuInstructionIdentity::kRegimmTgei: return "regimm_tgei";
    case CpuInstructionIdentity::kRegimmTgeiu: return "regimm_tgeiu";
    case CpuInstructionIdentity::kRegimmTlti: return "regimm_tlti";
    case CpuInstructionIdentity::kRegimmTltiu: return "regimm_tltiu";
    case CpuInstructionIdentity::kRegimmTeqi: return "regimm_teqi";
    case CpuInstructionIdentity::kRegimmTnei: return "regimm_tnei";
    case CpuInstructionIdentity::kRegimmBltzal: return "regimm_bltzal";
    case CpuInstructionIdentity::kRegimmBgezal: return "regimm_bgezal";
    case CpuInstructionIdentity::kRegimmBltzall: return "regimm_bltzall";
    case CpuInstructionIdentity::kRegimmBgezall: return "regimm_bgezall";

    case CpuInstructionIdentity::kJ: return "j";
    case CpuInstructionIdentity::kJal: return "jal";
    case CpuInstructionIdentity::kBeq: return "beq";
    case CpuInstructionIdentity::kBne: return "bne";
    case CpuInstructionIdentity::kBlez: return "blez";
    case CpuInstructionIdentity::kBgtz: return "bgtz";
    case CpuInstructionIdentity::kAddi: return "addi";
    case CpuInstructionIdentity::kAddiu: return "addiu";
    case CpuInstructionIdentity::kSlti: return "slti";
    case CpuInstructionIdentity::kSltiu: return "sltiu";
    case CpuInstructionIdentity::kAndi: return "andi";
    case CpuInstructionIdentity::kOri: return "ori";
    case CpuInstructionIdentity::kXori: return "xori";
    case CpuInstructionIdentity::kLui: return "lui";
    case CpuInstructionIdentity::kCop0: return "cop0";
    case CpuInstructionIdentity::kCop1: return "cop1";
    case CpuInstructionIdentity::kCop2: return "cop2";
    case CpuInstructionIdentity::kCop3: return "cop3";
    case CpuInstructionIdentity::kBeql: return "beql";
    case CpuInstructionIdentity::kBnel: return "bnel";
    case CpuInstructionIdentity::kBlezl: return "blezl";
    case CpuInstructionIdentity::kBgtzl: return "bgtzl";
    case CpuInstructionIdentity::kDaddi: return "daddi";
    case CpuInstructionIdentity::kDaddiu: return "daddiu";
    case CpuInstructionIdentity::kLdl: return "ldl";
    case CpuInstructionIdentity::kLdr: return "ldr";
    case CpuInstructionIdentity::kLb: return "lb";
    case CpuInstructionIdentity::kLh: return "lh";
    case CpuInstructionIdentity::kLwl: return "lwl";
    case CpuInstructionIdentity::kLw: return "lw";
    case CpuInstructionIdentity::kLbu: return "lbu";
    case CpuInstructionIdentity::kLhu: return "lhu";
    case CpuInstructionIdentity::kLwr: return "lwr";
    case CpuInstructionIdentity::kLwu: return "lwu";
    case CpuInstructionIdentity::kSb: return "sb";
    case CpuInstructionIdentity::kSh: return "sh";
    case CpuInstructionIdentity::kSwl: return "swl";
    case CpuInstructionIdentity::kSw: return "sw";
    case CpuInstructionIdentity::kSdl: return "sdl";
    case CpuInstructionIdentity::kSdr: return "sdr";
    case CpuInstructionIdentity::kSwr: return "swr";
    case CpuInstructionIdentity::kCache: return "cache";
    case CpuInstructionIdentity::kLl: return "ll";
    case CpuInstructionIdentity::kLwc1: return "lwc1";
    case CpuInstructionIdentity::kLwc2: return "lwc2";
    case CpuInstructionIdentity::kLld: return "lld";
    case CpuInstructionIdentity::kLdc1: return "ldc1";
    case CpuInstructionIdentity::kLdc2: return "ldc2";
    case CpuInstructionIdentity::kLd: return "ld";
    case CpuInstructionIdentity::kSc: return "sc";
    case CpuInstructionIdentity::kSwc1: return "swc1";
    case CpuInstructionIdentity::kSwc2: return "swc2";
    case CpuInstructionIdentity::kScd: return "scd";
    case CpuInstructionIdentity::kSdc1: return "sdc1";
    case CpuInstructionIdentity::kSdc2: return "sdc2";
    case CpuInstructionIdentity::kSd: return "sd";
  }

  return "unreachable";
}

Machine::CpuInstructionExecutionResult Machine::execute_cpu_instruction(
    CpuInstructionIdentity identity,
    const DecodedCpuInstructionWord& instruction) {
  switch (identity) {
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

    default:
      return CpuInstructionExecutionResult::kUnsupported;
  }
}

Machine::CpuInstructionStepResult Machine::step_cpu_instruction() {
  const std::uint32_t raw = fetch_cpu_instruction_word();
  const DecodedCpuInstructionWord instruction = decode_cpu_instruction_word(raw);
  const CpuInstructionIdentity identity = identify_cpu_instruction(instruction);
  const CpuInstructionExecutionResult execution_result =
      execute_cpu_instruction(identity, instruction);

  if (execution_result != CpuInstructionExecutionResult::kExecuted) {
    return CpuInstructionStepResult::kUnsupported;
  }

  write_cpu_pc(cpu_pc() + 4);
  return CpuInstructionStepResult::kStepped;
}

}  // namespace fn64