#pragma once

#include <cstdint>
#include <string>

#include "machine.hpp"

namespace fn64::bootstrap_detail {

constexpr std::uint32_t encode_i_type(
    std::uint8_t opcode,
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return (static_cast<std::uint32_t>(opcode) << 26) |
         (static_cast<std::uint32_t>(rs) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(immediate);
}

constexpr std::uint32_t encode_special(
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

constexpr std::uint32_t encode_j_type(
    std::uint8_t opcode,
    std::uint32_t target_address) {
  return (static_cast<std::uint32_t>(opcode) << 26) |
         ((target_address >> 2) & 0x03ffffffu);
}

constexpr std::uint32_t encode_regimm(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x01, rs, rt, immediate);
}

constexpr std::uint32_t encode_j(std::uint32_t target_address) {
  return encode_j_type(0x02, target_address);
}

constexpr std::uint32_t encode_jal(std::uint32_t target_address) {
  return encode_j_type(0x03, target_address);
}

constexpr std::uint32_t encode_beq(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x04, rs, rt, immediate);
}

constexpr std::uint32_t encode_bne(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x05, rs, rt, immediate);
}

constexpr std::uint32_t encode_blez(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x06, rs, 0, immediate);
}

constexpr std::uint32_t encode_bgtz(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x07, rs, 0, immediate);
}

constexpr std::uint32_t encode_addi(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x08, rs, rt, immediate);
}

constexpr std::uint32_t encode_addiu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x09, rs, rt, immediate);
}

constexpr std::uint32_t encode_slti(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0a, rs, rt, immediate);
}

constexpr std::uint32_t encode_sltiu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0b, rs, rt, immediate);
}

constexpr std::uint32_t encode_andi(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0c, rs, rt, immediate);
}

constexpr std::uint32_t encode_ori(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0d, rs, rt, immediate);
}

constexpr std::uint32_t encode_xori(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0e, rs, rt, immediate);
}

constexpr std::uint32_t encode_lui(std::uint8_t rt, std::uint16_t immediate) {
  return encode_i_type(0x0f, 0, rt, immediate);
}

constexpr std::uint32_t encode_lb(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x20, rs, rt, immediate);
}

constexpr std::uint32_t encode_lh(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x21, rs, rt, immediate);
}

constexpr std::uint32_t encode_lwl(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x22, rs, rt, immediate);
}

constexpr std::uint32_t encode_lw(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x23, rs, rt, immediate);
}

constexpr std::uint32_t encode_lbu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x24, rs, rt, immediate);
}

constexpr std::uint32_t encode_lhu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x25, rs, rt, immediate);
}

constexpr std::uint32_t encode_lwr(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x26, rs, rt, immediate);
}

constexpr std::uint32_t encode_sb(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x28, rs, rt, immediate);
}

constexpr std::uint32_t encode_sh(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x29, rs, rt, immediate);
}

constexpr std::uint32_t encode_swl(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2a, rs, rt, immediate);
}

constexpr std::uint32_t encode_sw(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2b, rs, rt, immediate);
}

constexpr std::uint32_t encode_swr(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2e, rs, rt, immediate);
}

constexpr std::uint32_t encode_beql(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x14, rs, rt, immediate);
}

constexpr std::uint32_t encode_bnel(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_i_type(0x15, rs, rt, immediate);
}

constexpr std::uint32_t encode_blezl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x16, rs, 0, immediate);
}

constexpr std::uint32_t encode_bgtzl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_i_type(0x17, rs, 0, immediate);
}

constexpr std::uint32_t encode_bltz(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x00, immediate);
}

constexpr std::uint32_t encode_bgez(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x01, immediate);
}

constexpr std::uint32_t encode_bltzl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x02, immediate);
}

constexpr std::uint32_t encode_bgezl(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x03, immediate);
}

constexpr std::uint32_t encode_bltzal(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x10, immediate);
}

constexpr std::uint32_t encode_bgezal(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x11, immediate);
}

constexpr std::uint32_t encode_bltzall(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x12, immediate);
}

constexpr std::uint32_t encode_bgezall(std::uint8_t rs, std::uint16_t immediate) {
  return encode_regimm(rs, 0x13, immediate);
}

constexpr std::uint32_t encode_jr(std::uint8_t rs) {
  return encode_special(rs, 0, 0, 0, 0x08);
}

constexpr std::uint32_t encode_jalr(std::uint8_t rd, std::uint8_t rs) {
  return encode_special(rs, 0, rd, 0, 0x09);
}

constexpr std::uint32_t encode_sltu(
    std::uint8_t rd,
    std::uint8_t rs,
    std::uint8_t rt) {
  return encode_special(rs, rt, rd, 0, 0x2b);
}

constexpr std::uint32_t encode_special_register_trap(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint8_t funct) {
  return encode_special(rs, rt, 0, 0, funct);
}

constexpr std::uint32_t encode_regimm_immediate_trap(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return encode_regimm(rs, rt, immediate);
}

constexpr std::uint32_t encode_syscall() {
  return encode_special(0, 0, 0, 0, 0x0c);
}

constexpr std::uint32_t encode_break() {
  return encode_special(0, 0, 0, 0, 0x0d);
}

constexpr std::uint32_t encode_sync() {
  return encode_special(0, 0, 0, 0, 0x0f);
}

void print_hex64(const char* label, std::uint64_t value);
void print_hex32(const char* label, std::uint32_t value);
void print_control_flow_state(const Machine& machine);
void print_rdram_word(const Machine& machine, const char* label, std::uint32_t address);

void require_stepped(Machine::CpuInstructionStepResult result, const std::string& label);
void require_stopped(Machine::CpuInstructionStepResult result, const std::string& label);

}  // namespace fn64::bootstrap_detail