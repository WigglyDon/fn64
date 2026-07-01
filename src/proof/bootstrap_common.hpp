#pragma once

#include <cstdint>
#include <string>

#include "machine.hpp"

namespace fn64::bootstrap_detail {

constexpr CpuAddress kSyntheticKseg0RdramBase = 0x80000000u;
constexpr CpuAddress kSyntheticKseg1RdramBase = 0xa0000000u;
constexpr RdramOffset kSyntheticDirectSegmentOffsetMask = 0x1fffffffu;

constexpr CpuAddress cpu_rdram_alias(RdramOffset rdram_offset) {
  return kSyntheticKseg0RdramBase + rdram_offset;
}

constexpr CpuAddress cpu_rdram_uncached_alias(RdramOffset rdram_offset) {
  return kSyntheticKseg1RdramBase + rdram_offset;
}

constexpr RdramOffset rdram_offset_from_cpu_address(CpuAddress cpu_address) {
  return cpu_address & kSyntheticDirectSegmentOffsetMask;
}

constexpr CpuRegisterValue cpu_value_from_sign_extended_u32(std::uint32_t value) {
  if ((value & 0x80000000u) == 0) {
    return static_cast<CpuRegisterValue>(value);
  }

  return static_cast<CpuRegisterValue>(0xffffffff00000000ull) |
         static_cast<CpuRegisterValue>(value);
}

constexpr CpuRegisterValue cpu_value_from_zero_extended_u32(std::uint32_t value) {
  return static_cast<CpuRegisterValue>(value);
}

constexpr CpuInstructionWord encode_i_type(
    std::uint8_t opcode,
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return (static_cast<std::uint32_t>(opcode) << 26) |
         (static_cast<std::uint32_t>(rs) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(immediate);
}

constexpr CpuInstructionWord encode_special(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint8_t rd,
    std::uint8_t sa,
    std::uint8_t funct) {
  return (static_cast<std::uint32_t>(0x00) << 26) |
         (static_cast<std::uint32_t>(rs) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         (static_cast<std::uint32_t>(rd) << 11) |
         (static_cast<std::uint32_t>(sa) << 6) |
         static_cast<std::uint32_t>(funct);
}

constexpr CpuInstructionWord encode_j_type(
    std::uint8_t opcode,
    CpuAddress target_address) {
  return (static_cast<std::uint32_t>(opcode) << 26) |
         ((target_address >> 2) & 0x03ffffffu);
}

constexpr CpuInstructionWord encode_regimm(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x01, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_j(CpuAddress target_address) {
  return encode_j_type(0x02, target_address);
}

constexpr CpuInstructionWord encode_jal(CpuAddress target_address) {
  return encode_j_type(0x03, target_address);
}

constexpr CpuInstructionWord encode_beq(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x04, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_bne(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x05, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_blez(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x06, rs, 0, immediate);
}

constexpr CpuInstructionWord encode_bgtz(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x07, rs, 0, immediate);
}

constexpr CpuInstructionWord encode_addi(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x08, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_addiu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x09, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_daddi(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x18, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_daddiu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x19, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_slti(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0a, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_sltiu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0b, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_andi(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0c, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_ori(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0d, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_xori(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0e, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lui(std::uint8_t rt, std::uint16_t immediate) {
  return encode_i_type(0x0f, 0, rt, immediate);
}

constexpr CpuInstructionWord encode_lb(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x20, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lh(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x21, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lwl(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x22, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lw(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x23, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lbu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x24, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lhu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x25, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lwr(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x26, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_lwu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x27, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_ldl(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x1a, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_ldr(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x1b, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_ld(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x37, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_sb(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x28, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_sh(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x29, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_swl(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2a, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_sw(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2b, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_swr(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2e, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_sdl(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2c, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_sdr(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2d, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_sd(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x3f, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_beql(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x14, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_bnel(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x15, rs, rt, immediate);
}

constexpr CpuInstructionWord encode_blezl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x16, rs, 0, immediate);
}

constexpr CpuInstructionWord encode_bgtzl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x17, rs, 0, immediate);
}

constexpr CpuInstructionWord encode_bltz(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x00, immediate);
}

constexpr CpuInstructionWord encode_bgez(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x01, immediate);
}

constexpr CpuInstructionWord encode_bltzl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x02, immediate);
}

constexpr CpuInstructionWord encode_bgezl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x03, immediate);
}

constexpr CpuInstructionWord encode_bltzal(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x10, immediate);
}

constexpr CpuInstructionWord encode_bgezal(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x11, immediate);
}

constexpr CpuInstructionWord encode_bltzall(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x12, immediate);
}

constexpr CpuInstructionWord encode_bgezall(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x13, immediate);
}

constexpr CpuInstructionWord encode_jr(std::uint8_t rs) {
  return encode_special(rs, 0, 0, 0, 0x08);
}

constexpr CpuInstructionWord encode_jalr(std::uint8_t rd, std::uint8_t rs) {
  return encode_special(rs, 0, rd, 0, 0x09);
}

constexpr CpuInstructionWord encode_sltu(
    std::uint8_t rd,
    std::uint8_t rs,
    std::uint8_t rt) {
  return encode_special(rs, rt, rd, 0, 0x2b);
}

constexpr CpuInstructionWord encode_dadd(
    std::uint8_t rd,
    std::uint8_t rs,
    std::uint8_t rt) {
  return encode_special(rs, rt, rd, 0, 0x2c);
}

constexpr CpuInstructionWord encode_daddu(
    std::uint8_t rd,
    std::uint8_t rs,
    std::uint8_t rt) {
  return encode_special(rs, rt, rd, 0, 0x2d);
}

constexpr CpuInstructionWord encode_dsub(
    std::uint8_t rd,
    std::uint8_t rs,
    std::uint8_t rt) {
  return encode_special(rs, rt, rd, 0, 0x2e);
}

constexpr CpuInstructionWord encode_dsubu(
    std::uint8_t rd,
    std::uint8_t rs,
    std::uint8_t rt) {
  return encode_special(rs, rt, rd, 0, 0x2f);
}

constexpr CpuInstructionWord encode_dsll(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t sa) {
  return encode_special(0, rt, rd, sa, 0x38);
}

constexpr CpuInstructionWord encode_dsrl(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t sa) {
  return encode_special(0, rt, rd, sa, 0x3a);
}

constexpr CpuInstructionWord encode_dsra(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t sa) {
  return encode_special(0, rt, rd, sa, 0x3b);
}

constexpr CpuInstructionWord encode_dsll32(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t sa) {
  return encode_special(0, rt, rd, sa, 0x3c);
}

constexpr CpuInstructionWord encode_dsrl32(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t sa) {
  return encode_special(0, rt, rd, sa, 0x3e);
}

constexpr CpuInstructionWord encode_dsra32(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t sa) {
  return encode_special(0, rt, rd, sa, 0x3f);
}

constexpr CpuInstructionWord encode_dsllv(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t rs) {
  return encode_special(rs, rt, rd, 0, 0x14);
}

constexpr CpuInstructionWord encode_dsrlv(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t rs) {
  return encode_special(rs, rt, rd, 0, 0x16);
}

constexpr CpuInstructionWord encode_dsrav(
    std::uint8_t rd,
    std::uint8_t rt,
    std::uint8_t rs) {
  return encode_special(rs, rt, rd, 0, 0x17);
}

constexpr CpuInstructionWord encode_special_register_trap(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint8_t funct) {
  return encode_special(rs, rt, 0, 0, funct);
}

constexpr CpuInstructionWord encode_regimm_immediate_trap(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_regimm(rs, rt, immediate);
}

constexpr CpuInstructionWord encode_syscall() {
  return encode_special(0, 0, 0, 0, 0x0c);
}

constexpr CpuInstructionWord encode_break() {
  return encode_special(0, 0, 0, 0, 0x0d);
}

constexpr CpuInstructionWord encode_sync() {
  return encode_special(0, 0, 0, 0, 0x0f);
}

void print_hex64(const char* label, std::uint64_t value);
void print_hex32(const char* label, std::uint32_t value);
void print_control_flow_state(const Machine& machine);
void print_rdram_word(const Machine& machine, const char* label, RdramOffset address);

void require_stepped(Machine::CpuInstructionStepResult result, const std::string& label);
void require_stopped(Machine::CpuInstructionStepResult result, const std::string& label);

}  // namespace fn64::bootstrap_detail
