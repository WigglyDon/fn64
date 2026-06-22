#include "bootstrap_common.hpp"

#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

void run_fetch_failure_no_ghost_case(
    Machine& machine,
    const char* label,
    std::uint32_t failing_pc,
    std::uint32_t preserved_next_pc,
    const char* expected_error_text) {
  constexpr std::size_t kFirstGprIndex = 4;
  constexpr std::size_t kSecondGprIndex = 5;
  constexpr std::size_t kLinkGprIndex = 31;
  constexpr std::uint32_t kFirstGprValue = 0x10203040u;
  constexpr std::uint32_t kSecondGprValue = 0xaabbccddu;
  constexpr std::uint32_t kLinkGprValue = 0x13579bdfu;
  constexpr std::uint32_t kHiValue = 0x2468ace0u;
  constexpr std::uint32_t kLoValue = 0x0badc0deu;
  constexpr std::uint32_t kRdramSentinelAddress = 0x00000900u;
  constexpr std::uint32_t kRdramSentinelValue = 0xfeed1234u;

  machine.write_cpu_pc(failing_pc);
  machine.write_cpu_next_pc(preserved_next_pc);
  machine.write_cpu_gpr(kFirstGprIndex, kFirstGprValue);
  machine.write_cpu_gpr(kSecondGprIndex, kSecondGprValue);
  machine.write_cpu_gpr(kLinkGprIndex, kLinkGprValue);
  machine.write_cpu_hi(kHiValue);
  machine.write_cpu_lo(kLoValue);
  machine.write_rdram_u32_be(kRdramSentinelAddress, kRdramSentinelValue);

  std::cout << "fetch failure row: " << label << '\n';
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kFirstGprIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kSecondGprIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkGprIndex));
  print_hex32("  hi", machine.cpu_hi());
  print_hex32("  lo", machine.cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000900]", kRdramSentinelAddress);

  bool threw = false;
  std::string error_text;
  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::exception& error) {
    threw = true;
    error_text = error.what();
    std::cout << "  " << label << " threw: " << error_text << '\n';
  }

  if (!threw) {
    throw std::runtime_error(std::string(label) + " expected step_cpu_instruction to throw");
  }

  if (error_text.find(expected_error_text) == std::string::npos) {
    throw std::runtime_error(std::string(label) + " threw unexpected text");
  }

  std::cout << "after failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kFirstGprIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kSecondGprIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkGprIndex));
  print_hex32("  hi", machine.cpu_hi());
  print_hex32("  lo", machine.cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000900]", kRdramSentinelAddress);

  if (machine.cpu_pc() != failing_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error(std::string(label) + " changed pc/next_pc on fetch failure");
  }

  if (machine.read_cpu_gpr(kFirstGprIndex) != kFirstGprValue ||
      machine.read_cpu_gpr(kSecondGprIndex) != kSecondGprValue ||
      machine.read_cpu_gpr(kLinkGprIndex) != kLinkGprValue) {
    throw std::runtime_error(std::string(label) + " changed GPR state on fetch failure");
  }

  if (machine.cpu_hi() != kHiValue || machine.cpu_lo() != kLoValue) {
    throw std::runtime_error(std::string(label) + " changed HI/LO on fetch failure");
  }

  if (machine.read_rdram_u32_be(kRdramSentinelAddress) != kRdramSentinelValue) {
    throw std::runtime_error(std::string(label) + " changed RDRAM on fetch failure");
  }
}

void run_fetch_failure_no_ghost_demo(Machine& machine) {
  std::cout
      << "fn64 bootstrap fetch failure no-ghost demo: failed instruction fetch leaves machine state unchanged\n";

  run_fetch_failure_no_ghost_case(
      machine,
      "misaligned_pc_fetch",
      0x00000902u,
      0x00000980u,
      "Unaligned CPU instruction fetch at PC");

  run_fetch_failure_no_ghost_case(
      machine,
      "out_of_window_pc_fetch",
      0x00400000u,
      0x00000990u,
      "CPU instruction fetch");
}

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
  constexpr std::uint32_t kHiValue = 0x24681357u;
  constexpr std::uint32_t kLoValue = 0x89abcdefu;
  constexpr std::uint32_t kRdramSentinelAddress = 0x00000940u;
  constexpr std::uint32_t kRdramSentinelValue = 0x5a17c0deu;

  const std::uint32_t kFollowingAddress = unsupported_address + 4u;
  const std::uint32_t kFollowingInstruction = encode_ori(
      static_cast<std::uint8_t>(kFollowingMarkerIndex), 0, following_marker);

  machine.write_cpu_pc(unsupported_address);
  machine.write_cpu_gpr(kPreservedRegisterIndex, kPreservedRegisterValue);
  machine.write_cpu_gpr(kFollowingMarkerIndex, 0);
  machine.write_cpu_hi(kHiValue);
  machine.write_cpu_lo(kLoValue);

  machine.write_rdram_u32_be(unsupported_address, unsupported_instruction);
  machine.write_rdram_u32_be(kFollowingAddress, kFollowingInstruction);
  machine.write_rdram_u32_be(kRdramSentinelAddress, kRdramSentinelValue);

  std::cout << "fn64 bootstrap unsupported demo: " << label << '\n';
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[23]", machine.read_cpu_gpr(kPreservedRegisterIndex));
  print_hex64("  gpr[24]", machine.read_cpu_gpr(kFollowingMarkerIndex));
  print_hex32("  hi", machine.cpu_hi());
  print_hex32("  lo", machine.cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000940]", kRdramSentinelAddress);

  const std::uint32_t raw = unsupported_instruction;
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
  print_hex32("  hi", machine.cpu_hi());
  print_hex32("  lo", machine.cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000940]", kRdramSentinelAddress);

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

  if (machine.cpu_hi() != kHiValue || machine.cpu_lo() != kLoValue) {
    throw std::runtime_error(
        std::string("unsupported demo changed HI/LO state: ") + label);
  }

  if (machine.read_rdram_u32_be(kRdramSentinelAddress) != kRdramSentinelValue) {
    throw std::runtime_error(
        std::string("unsupported demo changed RDRAM sentinel: ") + label);
  }

  if (machine.read_rdram_u32_be(unsupported_address) != unsupported_instruction) {
    throw std::runtime_error(
        std::string("unsupported demo did not preserve the staged unsupported instruction: ") + label);
  }
}

}  // namespace

void run_unsupported_instruction_demos(Machine& machine) {
  run_fetch_failure_no_ghost_demo(machine);

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
