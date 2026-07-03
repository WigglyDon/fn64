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
    MachineFaultKind expected_kind,
    std::size_t expected_access_size) {
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

  machine.stage_cpu_pc(failing_pc);
  machine.stage_cpu_next_pc(preserved_next_pc);
  machine.stage_cpu_gpr(kFirstGprIndex, kFirstGprValue);
  machine.stage_cpu_gpr(kSecondGprIndex, kSecondGprValue);
  machine.stage_cpu_gpr(kLinkGprIndex, kLinkGprValue);
  machine.stage_cpu_hi(kHiValue);
  machine.stage_cpu_lo(kLoValue);
  machine.stage_rdram_u32_be(kRdramSentinelAddress, kRdramSentinelValue);

  std::cout << "fetch failure row: " << label << '\n';
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kFirstGprIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kSecondGprIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(kLinkGprIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000900]", kRdramSentinelAddress);

  bool threw = false;
  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& error) {
    threw = true;
    std::cout << "  " << label << " threw: " << error.what() << '\n';
    if (error.kind() != expected_kind) {
      throw std::runtime_error(std::string(label) + " threw unexpected MachineFault kind");
    }
    if (error.access_size() != expected_access_size) {
      throw std::runtime_error(std::string(label) + " threw unexpected MachineFault access size");
    }
  } catch (const std::exception& error) {
    throw std::runtime_error(
        std::string(label) + " threw unexpected exception type: " + error.what());
  }

  if (!threw) {
    throw std::runtime_error(std::string(label) + " expected step_cpu_instruction to throw");
  }

  std::cout << "after failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kFirstGprIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kSecondGprIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(kLinkGprIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000900]", kRdramSentinelAddress);

  if (machine.cpu_pc() != failing_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error(std::string(label) + " changed pc/next_pc on fetch failure");
  }

  if (machine.inspect_cpu_gpr(kFirstGprIndex) != kFirstGprValue ||
      machine.inspect_cpu_gpr(kSecondGprIndex) != kSecondGprValue ||
      machine.inspect_cpu_gpr(kLinkGprIndex) != kLinkGprValue) {
    throw std::runtime_error(std::string(label) + " changed GPR state on fetch failure");
  }

  if (machine.inspect_cpu_hi() != kHiValue || machine.inspect_cpu_lo() != kLoValue) {
    throw std::runtime_error(std::string(label) + " changed HI/LO on fetch failure");
  }

  if (machine.inspect_rdram_u32_be(kRdramSentinelAddress) != kRdramSentinelValue) {
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
      MachineFaultKind::kUnalignedInstructionFetch,
      4);

  run_fetch_failure_no_ghost_case(
      machine,
      "out_of_window_pc_fetch",
      0x00400000u,
      0x00000990u,
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
}

void run_unsupported_identity_demo(
    Machine& machine,
    const char* label,
    std::uint32_t unsupported_address,
    std::uint32_t unsupported_instruction,
    std::uint16_t following_marker) {
  constexpr std::size_t kPreservedRegisterIndex = 23;
  constexpr std::size_t kFollowingMarkerIndex = 24;
  constexpr CpuRegisterValue kPreservedRegisterValue = 0x1122334413572468ull;
  constexpr CpuRegisterValue kHiValue = 0x5566778824681357ull;
  constexpr CpuRegisterValue kLoValue = 0x99aabbcc89abcdefull;
  constexpr std::uint32_t kRdramSentinelAddress = 0x00000940u;
  constexpr std::uint32_t kRdramSentinelValue = 0x5a17c0deu;

  const std::uint32_t kFollowingAddress = unsupported_address + 4u;
  const std::uint32_t kFollowingInstruction = encode_ori(
      static_cast<std::uint8_t>(kFollowingMarkerIndex), 0, following_marker);

  machine.stage_cpu_pc(cpu_rdram_alias(unsupported_address));
  machine.stage_cpu_gpr(kPreservedRegisterIndex, kPreservedRegisterValue);
  machine.stage_cpu_gpr(kFollowingMarkerIndex, 0);
  machine.stage_cpu_hi(kHiValue);
  machine.stage_cpu_lo(kLoValue);

  machine.stage_rdram_u32_be(unsupported_address, unsupported_instruction);
  machine.stage_rdram_u32_be(kFollowingAddress, kFollowingInstruction);
  machine.stage_rdram_u32_be(kRdramSentinelAddress, kRdramSentinelValue);

  std::cout << "fn64 bootstrap unsupported demo: " << label << '\n';
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[23]", machine.inspect_cpu_gpr(kPreservedRegisterIndex));
  print_hex64("  gpr[24]", machine.inspect_cpu_gpr(kFollowingMarkerIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000940]", kRdramSentinelAddress);

  const std::uint32_t raw = unsupported_instruction;

  print_hex32("  unsupported_raw", raw);

  const std::uint32_t preserved_pc = machine.cpu_pc();
  const std::uint32_t preserved_next_pc = machine.cpu_next_pc();

  const Machine::CpuInstructionStepResult step_result = machine.step_cpu_instruction();
  if (step_result != Machine::CpuInstructionStepResult::kUnsupported) {
    throw std::runtime_error(
        std::string("unsupported demo did not return kUnsupported: ") + label);
  }

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[23]", machine.inspect_cpu_gpr(kPreservedRegisterIndex));
  print_hex64("  gpr[24]", machine.inspect_cpu_gpr(kFollowingMarkerIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_rdram_word(machine, "  rdram[0x00000940]", kRdramSentinelAddress);

  if (machine.cpu_pc() != preserved_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error(
        std::string("unsupported demo did not preserve pc/next_pc rollback: ") + label);
  }

  if (machine.inspect_cpu_gpr(kPreservedRegisterIndex) != kPreservedRegisterValue) {
    throw std::runtime_error(
        std::string("unsupported demo changed preserved register state: ") + label);
  }

  if (machine.inspect_cpu_gpr(kFollowingMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("unsupported demo leaked follow-on instruction side effects: ") + label);
  }

  if (machine.inspect_cpu_hi() != kHiValue || machine.inspect_cpu_lo() != kLoValue) {
    throw std::runtime_error(
        std::string("unsupported demo changed HI/LO state: ") + label);
  }

  if (machine.inspect_rdram_u32_be(kRdramSentinelAddress) != kRdramSentinelValue) {
    throw std::runtime_error(
        std::string("unsupported demo changed RDRAM sentinel: ") + label);
  }

  if (machine.inspect_rdram_u32_be(unsupported_address) != unsupported_instruction) {
    throw std::runtime_error(
        std::string("unsupported demo did not preserve the staged unsupported instruction: ") + label);
  }
}

}  // namespace

void run_unsupported_instruction_demos(Machine& machine) {
  run_fetch_failure_no_ghost_demo(machine);

  constexpr CpuInstructionWord kCop0Mfc0UnsupportedRegisterInstruction =
      encode_i_type(0x10, 0x00, 0, 0x0000u);
  constexpr CpuInstructionWord kCop0Dmfc0UnsupportedInstruction =
      encode_i_type(0x10, 0x01, 0, static_cast<std::uint16_t>(12u << 11));
  constexpr CpuInstructionWord kCop0Mtc0UnsupportedRegisterInstruction =
      encode_i_type(0x10, 0x04, 23, static_cast<std::uint16_t>(13u << 11));
  constexpr CpuInstructionWord kCop0Dmtc0UnsupportedInstruction =
      encode_i_type(0x10, 0x05, 23, static_cast<std::uint16_t>(12u << 11));
  constexpr CpuInstructionWord kCop0TlbpUnsupportedInstruction = 0x42000008u;
  constexpr CpuInstructionWord kCop0EretUnsupportedInstruction = 0x42000018u;
  constexpr CpuInstructionWord kCop1UnsupportedInstruction = encode_i_type(0x11, 0, 0, 0x0000u);
  constexpr CpuInstructionWord kCop2UnsupportedInstruction = encode_i_type(0x12, 0, 0, 0x0000u);
  constexpr CpuInstructionWord kCop3UnsupportedInstruction = encode_i_type(0x13, 0, 0, 0x0000u);
  constexpr std::uint32_t kSpecialUnknownUnsupportedInstruction = 0x00000001u;
  constexpr std::uint32_t kRegimmUnknownUnsupportedInstruction = 0x04040000u;
  constexpr std::uint32_t kUnknownPrimaryUnsupportedInstruction = 0xcc000000u;
  constexpr CpuInstructionWord kCacheUnsupportedInstruction = encode_i_type(0x2f, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kLwc1UnsupportedInstruction = encode_i_type(0x31, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kLwc2UnsupportedInstruction = encode_i_type(0x32, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kLdc1UnsupportedInstruction = encode_i_type(0x35, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kLdc2UnsupportedInstruction = encode_i_type(0x36, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kSwc1UnsupportedInstruction = encode_i_type(0x39, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kSwc2UnsupportedInstruction = encode_i_type(0x3a, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kSdc1UnsupportedInstruction = encode_i_type(0x3d, 6, 4, 0x0000u);
  constexpr CpuInstructionWord kSdc2UnsupportedInstruction = encode_i_type(0x3e, 6, 4, 0x0000u);

  run_unsupported_identity_demo(
      machine,
      "COP0 MFC0 unsupported register returns unsupported with rollback intact",
      0x000006c0u,
      kCop0Mfc0UnsupportedRegisterInstruction,
      0x75c1u);

  run_unsupported_identity_demo(
      machine,
      "COP0 DMFC0 remains unsupported with rollback intact",
      0x000006c4u,
      kCop0Dmfc0UnsupportedInstruction,
      0x75c5u);

  run_unsupported_identity_demo(
      machine,
      "COP0 MTC0 unsupported register remains unsupported with rollback intact",
      0x000006c8u,
      kCop0Mtc0UnsupportedRegisterInstruction,
      0x75c9u);

  run_unsupported_identity_demo(
      machine,
      "COP0 DMTC0 remains unsupported with rollback intact",
      0x000006ccu,
      kCop0Dmtc0UnsupportedInstruction,
      0x75cdu);

  run_unsupported_identity_demo(
      machine,
      "COP0 TLB-shaped operation remains unsupported with rollback intact",
      0x000006d0u,
      kCop0TlbpUnsupportedInstruction,
      0x75d1u);

  run_unsupported_identity_demo(
      machine,
      "COP0 ERET remains unsupported with rollback intact",
      0x000006d4u,
      kCop0EretUnsupportedInstruction,
      0x75d5u);

  run_unsupported_identity_demo(
      machine,
      "cop1 path returns unsupported with rollback intact",
      0x000006d8u,
      kCop1UnsupportedInstruction,
      0x75d9u);

  run_unsupported_identity_demo(
      machine,
      "cop2 path returns unsupported with rollback intact",
      0x000006dcu,
      kCop2UnsupportedInstruction,
      0x75ddu);

  run_unsupported_identity_demo(
      machine,
      "cop3 path returns unsupported with rollback intact",
      0x000006e0u,
      kCop3UnsupportedInstruction,
      0x75e1u);

  run_unsupported_identity_demo(
      machine,
      "special unknown funct returns unsupported with rollback intact",
      0x000006e4u,
      kSpecialUnknownUnsupportedInstruction,
      0x75e5u);

  run_unsupported_identity_demo(
      machine,
      "regimm unknown rt returns unsupported with rollback intact",
      0x000006e8u,
      kRegimmUnknownUnsupportedInstruction,
      0x75e9u);

  run_unsupported_identity_demo(
      machine,
      "unknown primary opcode returns unsupported with rollback intact",
      0x000006f0u,
      kUnknownPrimaryUnsupportedInstruction,
      0x75f1u);

  run_unsupported_identity_demo(
      machine,
      "CACHE remains unsupported with rollback intact",
      0x00000730u,
      kCacheUnsupportedInstruction,
      0x7631u);

  run_unsupported_identity_demo(
      machine,
      "LWC1 remains unsupported with rollback intact",
      0x00000748u,
      kLwc1UnsupportedInstruction,
      0x7649u);

  run_unsupported_identity_demo(
      machine,
      "LWC2 remains unsupported with rollback intact",
      0x0000074cu,
      kLwc2UnsupportedInstruction,
      0x764du);

  run_unsupported_identity_demo(
      machine,
      "LDC1 remains unsupported with rollback intact",
      0x00000760u,
      kLdc1UnsupportedInstruction,
      0x7661u);

  run_unsupported_identity_demo(
      machine,
      "LDC2 remains unsupported with rollback intact",
      0x00000768u,
      kLdc2UnsupportedInstruction,
      0x7669u);

  run_unsupported_identity_demo(
      machine,
      "SWC1 remains unsupported with rollback intact",
      0x00000778u,
      kSwc1UnsupportedInstruction,
      0x7679u);

  run_unsupported_identity_demo(
      machine,
      "SWC2 remains unsupported with rollback intact",
      0x0000077cu,
      kSwc2UnsupportedInstruction,
      0x767du);

  run_unsupported_identity_demo(
      machine,
      "SDC1 remains unsupported with rollback intact",
      0x00000790u,
      kSdc1UnsupportedInstruction,
      0x7691u);

  run_unsupported_identity_demo(
      machine,
      "SDC2 remains unsupported with rollback intact",
      0x00000798u,
      kSdc2UnsupportedInstruction,
      0x7699u);
}

}  // namespace fn64::bootstrap_detail
