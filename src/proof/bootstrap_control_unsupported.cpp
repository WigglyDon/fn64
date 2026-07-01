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

  constexpr std::uint32_t kCop0UnsupportedInstruction = 0x40000000u;
  constexpr std::uint32_t kSpecialUnknownUnsupportedInstruction = 0x00000001u;
  constexpr std::uint32_t kRegimmUnknownUnsupportedInstruction = 0x04040000u;
  constexpr std::uint32_t kUnknownPrimaryUnsupportedInstruction = 0xcc000000u;
  constexpr CpuInstructionWord kDaddUnsupportedInstruction = encode_special(4, 5, 6, 0, 0x2c);
  constexpr CpuInstructionWord kDaddiUnsupportedInstruction = encode_i_type(0x18, 4, 6, 0x0001u);
  constexpr CpuInstructionWord kDsubUnsupportedInstruction = encode_special(4, 5, 6, 0, 0x2e);
  constexpr CpuInstructionWord kLdlUnsupportedInstruction = encode_ldl(6, 4, 0x0000u);
  constexpr CpuInstructionWord kLdrUnsupportedInstruction = encode_ldr(6, 4, 0x0000u);
  constexpr CpuInstructionWord kSdlUnsupportedInstruction = encode_sdl(6, 4, 0x0000u);
  constexpr CpuInstructionWord kSdrUnsupportedInstruction = encode_sdr(6, 4, 0x0000u);

  run_unsupported_identity_demo(
      machine,
      "cop0 path returns unsupported with rollback intact",
      0x000006c0u,
      kCop0UnsupportedInstruction,
      0x75c1u);

  run_unsupported_identity_demo(
      machine,
      "special unknown funct returns unsupported with rollback intact",
      0x000006d0u,
      kSpecialUnknownUnsupportedInstruction,
      0x75d1u);

  run_unsupported_identity_demo(
      machine,
      "regimm unknown rt returns unsupported with rollback intact",
      0x000006e0u,
      kRegimmUnknownUnsupportedInstruction,
      0x75e1u);

  run_unsupported_identity_demo(
      machine,
      "unknown primary opcode returns unsupported with rollback intact",
      0x000006f0u,
      kUnknownPrimaryUnsupportedInstruction,
      0x75f1u);

  run_unsupported_identity_demo(
      machine,
      "DADD remains unsupported with rollback intact",
      0x00000700u,
      kDaddUnsupportedInstruction,
      0x7601u);

  run_unsupported_identity_demo(
      machine,
      "DADDI remains unsupported with rollback intact",
      0x00000710u,
      kDaddiUnsupportedInstruction,
      0x7611u);

  run_unsupported_identity_demo(
      machine,
      "DSUB remains unsupported with rollback intact",
      0x00000720u,
      kDsubUnsupportedInstruction,
      0x7621u);

  run_unsupported_identity_demo(
      machine,
      "LDL remains unsupported with rollback intact",
      0x00000730u,
      kLdlUnsupportedInstruction,
      0x7631u);

  run_unsupported_identity_demo(
      machine,
      "LDR remains unsupported with rollback intact",
      0x00000740u,
      kLdrUnsupportedInstruction,
      0x7641u);

  run_unsupported_identity_demo(
      machine,
      "SDL remains unsupported with rollback intact",
      0x00000750u,
      kSdlUnsupportedInstruction,
      0x7651u);

  run_unsupported_identity_demo(
      machine,
      "SDR remains unsupported with rollback intact",
      0x00000760u,
      kSdrUnsupportedInstruction,
      0x7661u);
}

}  // namespace fn64::bootstrap_detail
