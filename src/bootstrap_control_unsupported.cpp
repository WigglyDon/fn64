#include "bootstrap_common.hpp"

#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

void run_unsupported_identity_demo(
    Machine& machine,
    const char* label,
    std::uint32_t unsupported_address,
    std::uint32_t unsupported_instruction,
    Machine::CpuInstructionIdentity expected_identity,
    std::uint16_t following_marker) {
  constexpr std::size_t kPreservedRegisterIndex = 23;
  constexpr std::size_t kFollowingMarkerIndex = 24;
  constexpr std::uint32_t kPreservedRegisterValue = 0x13572468u;

  const std::uint32_t kFollowingAddress = unsupported_address + 4u;
  const std::uint32_t kFollowingInstruction = encode_ori(
      static_cast<std::uint8_t>(kFollowingMarkerIndex), 0, following_marker);

  machine.write_cpu_pc(unsupported_address);
  machine.write_cpu_gpr(kPreservedRegisterIndex, kPreservedRegisterValue);
  machine.write_cpu_gpr(kFollowingMarkerIndex, 0);

  machine.write_rdram_u32_be(unsupported_address, unsupported_instruction);
  machine.write_rdram_u32_be(kFollowingAddress, kFollowingInstruction);

  std::cout << "fn64 bootstrap unsupported demo: " << label << '\n';
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[23]", machine.read_cpu_gpr(kPreservedRegisterIndex));
  print_hex64("  gpr[24]", machine.read_cpu_gpr(kFollowingMarkerIndex));

  const std::uint32_t raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord decoded =
      Machine::decode_cpu_instruction_word(raw);
  const Machine::CpuInstructionIdentity identity =
      Machine::identify_cpu_instruction(decoded);

  print_hex32("  unsupported_raw", raw);
  std::cout << "  unsupported_identity = "
            << Machine::cpu_instruction_identity_name(identity) << '\n';

  if (identity != expected_identity) {
    throw std::runtime_error(
        std::string("unsupported demo identified the wrong unsupported path: ") + label);
  }

  const std::uint32_t preserved_pc = machine.cpu_pc();
  const std::uint32_t preserved_next_pc = machine.cpu_next_pc();

  const Machine::CpuInstructionStepResult step_result = machine.step_cpu_instruction();
  if (step_result != Machine::CpuInstructionStepResult::kUnsupported) {
    throw std::runtime_error(
        std::string("unsupported demo did not return kUnsupported: ") + label);
  }

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[23]", machine.read_cpu_gpr(kPreservedRegisterIndex));
  print_hex64("  gpr[24]", machine.read_cpu_gpr(kFollowingMarkerIndex));

  if (machine.cpu_pc() != preserved_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error(
        std::string("unsupported demo did not preserve pc/next_pc rollback: ") + label);
  }

  if (machine.read_cpu_gpr(kPreservedRegisterIndex) != kPreservedRegisterValue) {
    throw std::runtime_error(
        std::string("unsupported demo changed preserved register state: ") + label);
  }

  if (machine.read_cpu_gpr(kFollowingMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("unsupported demo leaked follow-on instruction side effects: ") + label);
  }

  if (machine.fetch_cpu_instruction_word() != unsupported_instruction) {
    throw std::runtime_error(
        std::string("unsupported demo did not remain positioned at the same instruction: ") + label);
  }
}

}  // namespace

void run_unsupported_instruction_demos(Machine& machine) {
  constexpr std::uint32_t kCop0UnsupportedInstruction = 0x40000000u;
  constexpr std::uint32_t kSpecialUnknownUnsupportedInstruction = 0x00000001u;
  constexpr std::uint32_t kRegimmUnknownUnsupportedInstruction = 0x04040000u;
  constexpr std::uint32_t kUnknownPrimaryUnsupportedInstruction = 0xcc000000u;

  run_unsupported_identity_demo(
      machine,
      "cop0 path returns unsupported with rollback intact",
      0x000006c0u,
      kCop0UnsupportedInstruction,
      Machine::CpuInstructionIdentity::kCop0,
      0x75c1u);

  run_unsupported_identity_demo(
      machine,
      "special unknown funct returns unsupported with rollback intact",
      0x000006d0u,
      kSpecialUnknownUnsupportedInstruction,
      Machine::CpuInstructionIdentity::kSpecialUnknown,
      0x75d1u);

  run_unsupported_identity_demo(
      machine,
      "regimm unknown rt returns unsupported with rollback intact",
      0x000006e0u,
      kRegimmUnknownUnsupportedInstruction,
      Machine::CpuInstructionIdentity::kRegimmUnknown,
      0x75e1u);

  run_unsupported_identity_demo(
      machine,
      "unknown primary opcode returns unsupported with rollback intact",
      0x000006f0u,
      kUnknownPrimaryUnsupportedInstruction,
      Machine::CpuInstructionIdentity::kUnknownPrimary,
      0x75f1u);
}

}  // namespace fn64::bootstrap_detail