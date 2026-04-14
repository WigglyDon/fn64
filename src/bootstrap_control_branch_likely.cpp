#include "bootstrap_common.hpp"

#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

void run_branch_likely_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t branch_instruction,
    Machine::CpuInstructionIdentity expected_identity,
    std::uint32_t rs_value,
    std::uint32_t rt_value,
    bool expect_taken,
    std::uint16_t delay_slot_marker,
    std::uint16_t fallthrough_marker,
    std::uint16_t target_marker) {
  constexpr std::size_t kRsIndex = 4;
  constexpr std::size_t kRtIndex = 5;
  constexpr std::size_t kDelaySlotMarkerIndex = 6;
  constexpr std::size_t kFallthroughMarkerIndex = 7;
  constexpr std::size_t kTargetMarkerIndex = 8;

  const std::uint32_t kBranchAddress = base_address;
  const std::uint32_t kDelaySlotAddress = base_address + 4u;
  const std::uint32_t kFallthroughAddress = base_address + 8u;
  const std::uint32_t kNotTakenSentinelAddress = base_address + 12u;
  const std::uint32_t kTargetAddress = base_address + 16u;
  const std::uint32_t kTakenSentinelAddress = base_address + 20u;

  const std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, delay_slot_marker);
  const std::uint32_t kFallthroughInstruction = encode_ori(
      static_cast<std::uint8_t>(kFallthroughMarkerIndex), 0, fallthrough_marker);
  const std::uint32_t kTargetInstruction = encode_ori(
      static_cast<std::uint8_t>(kTargetMarkerIndex), 0, target_marker);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kBranchAddress);
  machine.write_cpu_gpr(kRsIndex, rs_value);
  machine.write_cpu_gpr(kRtIndex, rt_value);
  machine.write_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.write_cpu_gpr(kFallthroughMarkerIndex, 0);
  machine.write_cpu_gpr(kTargetMarkerIndex, 0);

  machine.write_rdram_u32_be(kBranchAddress, branch_instruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kFallthroughAddress, kFallthroughInstruction);
  machine.write_rdram_u32_be(kNotTakenSentinelAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kTakenSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap branch-likely demo: " << label << '\n';
  std::cout << "before branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kRsIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kRtIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  const std::uint32_t branch_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord branch_decoded =
      Machine::decode_cpu_instruction_word(branch_raw);
  const Machine::CpuInstructionIdentity branch_identity =
      Machine::identify_cpu_instruction(branch_decoded);

  std::cout << "  branch_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << branch_raw
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  branch_identity = "
            << Machine::cpu_instruction_identity_name(branch_identity) << '\n';

  if (branch_identity != expected_identity) {
    throw std::runtime_error(
        std::string("branch-likely demo identified the wrong instruction: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_branch");

  std::cout << "after branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (expect_taken) {
    if (machine.cpu_pc() != kDelaySlotAddress) {
      throw std::runtime_error(
          std::string("branch-likely taken demo did not move into the delay slot: ") + label);
    }

    if (machine.cpu_next_pc() != kTargetAddress) {
      throw std::runtime_error(
          std::string("branch-likely taken demo did not schedule the target: ") + label);
    }

    if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
        machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
        machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("branch-likely taken demo changed marker registers too early: ") + label);
    }

    require_stepped(machine.step_cpu_instruction(), std::string(label) + "_delay_slot");

    std::cout << "after delay-slot step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
    print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
    print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

    if (machine.cpu_pc() != kTargetAddress) {
      throw std::runtime_error(
          std::string("branch-likely taken demo did not hand off to target after delay slot: ") + label);
    }

    if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) !=
        static_cast<std::uint32_t>(delay_slot_marker)) {
      throw std::runtime_error(
          std::string("branch-likely taken demo did not execute the delay slot: ") + label);
    }

    if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("branch-likely taken demo unexpectedly executed fallthrough path: ") + label);
    }

    require_stepped(machine.step_cpu_instruction(), std::string(label) + "_target");

    std::cout << "after target step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
    print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
    print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

    if (machine.cpu_pc() != kTakenSentinelAddress) {
      throw std::runtime_error(
          std::string("branch-likely taken demo did not advance to taken sentinel: ") + label);
    }

    if (machine.read_cpu_gpr(kTargetMarkerIndex) !=
        static_cast<std::uint32_t>(target_marker)) {
      throw std::runtime_error(
          std::string("branch-likely taken demo did not execute target instruction: ") + label);
    }

    require_stopped(machine.step_cpu_instruction(), std::string(label) + "_taken_break");
    return;
  }

  if (machine.cpu_pc() != kFallthroughAddress) {
    throw std::runtime_error(
        std::string("branch-likely not-taken demo did not skip the delay slot: ") + label);
  }

  if (machine.cpu_next_pc() != kNotTakenSentinelAddress) {
    throw std::runtime_error(
        std::string("branch-likely not-taken demo advanced to wrong next_pc after annul: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("branch-likely not-taken demo executed the annulled delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("branch-likely not-taken demo changed marker registers too early: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_fallthrough");

  std::cout << "after fallthrough step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != kNotTakenSentinelAddress) {
    throw std::runtime_error(
        std::string("branch-likely not-taken demo did not execute fallthrough instruction: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) !=
      static_cast<std::uint32_t>(fallthrough_marker)) {
    throw std::runtime_error(
        std::string("branch-likely not-taken demo did not execute fallthrough path: ") + label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("branch-likely not-taken demo unexpectedly executed target path: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_not_taken_break");
}

void run_branch_likely_link_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t branch_instruction,
    Machine::CpuInstructionIdentity expected_identity,
    std::uint32_t rs_value,
    bool expect_taken,
    std::uint16_t delay_slot_marker,
    std::uint16_t fallthrough_marker,
    std::uint16_t target_marker) {
  constexpr std::size_t kRsIndex = 4;
  constexpr std::size_t kDelaySlotMarkerIndex = 6;
  constexpr std::size_t kFallthroughMarkerIndex = 7;
  constexpr std::size_t kTargetMarkerIndex = 8;
  constexpr std::size_t kLinkIndex = 31;

  const std::uint32_t kBranchAddress = base_address;
  const std::uint32_t kDelaySlotAddress = base_address + 4u;
  const std::uint32_t kFallthroughAddress = base_address + 8u;
  const std::uint32_t kNotTakenSentinelAddress = base_address + 12u;
  const std::uint32_t kTargetAddress = base_address + 16u;
  const std::uint32_t kTakenSentinelAddress = base_address + 20u;
  const std::uint32_t kExpectedLinkValue = base_address + 8u;

  const std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, delay_slot_marker);
  const std::uint32_t kFallthroughInstruction = encode_ori(
      static_cast<std::uint8_t>(kFallthroughMarkerIndex), 0, fallthrough_marker);
  const std::uint32_t kTargetInstruction = encode_ori(
      static_cast<std::uint8_t>(kTargetMarkerIndex), 0, target_marker);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kBranchAddress);
  machine.write_cpu_gpr(kRsIndex, rs_value);
  machine.write_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.write_cpu_gpr(kFallthroughMarkerIndex, 0);
  machine.write_cpu_gpr(kTargetMarkerIndex, 0);
  machine.write_cpu_gpr(kLinkIndex, 0);

  machine.write_rdram_u32_be(kBranchAddress, branch_instruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kFallthroughAddress, kFallthroughInstruction);
  machine.write_rdram_u32_be(kNotTakenSentinelAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kTakenSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap branch-likely link demo: " << label << '\n';
  std::cout << "before branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kRsIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const std::uint32_t branch_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord branch_decoded =
      Machine::decode_cpu_instruction_word(branch_raw);
  const Machine::CpuInstructionIdentity branch_identity =
      Machine::identify_cpu_instruction(branch_decoded);

  std::cout << "  branch_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << branch_raw
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  branch_identity = "
            << Machine::cpu_instruction_identity_name(branch_identity) << '\n';

  if (branch_identity != expected_identity) {
    throw std::runtime_error(
        std::string("branch-likely link demo identified the wrong instruction: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_branch");

  std::cout << "after branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (expect_taken) {
    if (machine.cpu_pc() != kDelaySlotAddress) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo did not move into delay slot: ") + label);
    }

    if (machine.cpu_next_pc() != kTargetAddress) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo did not schedule target: ") + label);
    }

    if (machine.read_cpu_gpr(kLinkIndex) != kExpectedLinkValue) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo wrote wrong link value: ") + label);
    }

    if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
        machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
        machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo changed markers too early: ") + label);
    }

    require_stepped(machine.step_cpu_instruction(), std::string(label) + "_delay_slot");
    require_stepped(machine.step_cpu_instruction(), std::string(label) + "_target");

    std::cout << "after target step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
    print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
    print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
    print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

    if (machine.cpu_pc() != kTakenSentinelAddress) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo did not advance to taken sentinel: ") + label);
    }

    if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) !=
        static_cast<std::uint32_t>(delay_slot_marker)) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo did not execute delay slot: ") + label);
    }

    if (machine.read_cpu_gpr(kTargetMarkerIndex) !=
        static_cast<std::uint32_t>(target_marker)) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo did not execute target: ") + label);
    }

    if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo unexpectedly executed fallthrough path: ") + label);
    }

    require_stopped(machine.step_cpu_instruction(), std::string(label) + "_taken_break");
    return;
  }

  if (machine.cpu_pc() != kFallthroughAddress) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo did not skip delay slot: ") + label);
  }

  if (machine.cpu_next_pc() != kNotTakenSentinelAddress) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo advanced to wrong next_pc after annul: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo executed annulled delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kLinkIndex) != 0) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo unexpectedly wrote link register: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_fallthrough");

  std::cout << "after fallthrough step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kNotTakenSentinelAddress) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo did not execute fallthrough path: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) !=
      static_cast<std::uint32_t>(fallthrough_marker)) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo did not execute fallthrough marker: ") + label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo unexpectedly executed target path: ") + label);
  }

  if (machine.read_cpu_gpr(kLinkIndex) != 0) {
    throw std::runtime_error(
        std::string("branch-likely link not-taken demo link register changed after fallthrough: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_not_taken_break");
}

void run_backward_branch_likely_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t branch_instruction,
    Machine::CpuInstructionIdentity expected_identity,
    std::uint32_t rs_value,
    std::uint32_t rt_value,
    std::int16_t expected_immediate,
    std::uint16_t delay_slot_marker,
    std::uint16_t fallthrough_marker,
    std::uint16_t target_marker) {
  constexpr std::size_t kRsIndex = 4;
  constexpr std::size_t kRtIndex = 5;
  constexpr std::size_t kDelaySlotMarkerIndex = 6;
  constexpr std::size_t kFallthroughMarkerIndex = 7;
  constexpr std::size_t kTargetMarkerIndex = 8;

  const std::uint32_t kTargetAddress = base_address - 8u;
  const std::uint32_t kTargetSentinelAddress = base_address - 4u;
  const std::uint32_t kBranchAddress = base_address;
  const std::uint32_t kDelaySlotAddress = base_address + 4u;
  const std::uint32_t kFallthroughAddress = base_address + 8u;
  const std::uint32_t kFallthroughSentinelAddress = base_address + 12u;

  const std::uint32_t kTargetInstruction = encode_ori(
      static_cast<std::uint8_t>(kTargetMarkerIndex), 0, target_marker);
  const std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, delay_slot_marker);
  const std::uint32_t kFallthroughInstruction = encode_ori(
      static_cast<std::uint8_t>(kFallthroughMarkerIndex), 0, fallthrough_marker);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kBranchAddress);
  machine.write_cpu_gpr(kRsIndex, rs_value);
  machine.write_cpu_gpr(kRtIndex, rt_value);
  machine.write_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.write_cpu_gpr(kFallthroughMarkerIndex, 0);
  machine.write_cpu_gpr(kTargetMarkerIndex, 0);

  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kTargetSentinelAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kBranchAddress, branch_instruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kFallthroughAddress, kFallthroughInstruction);
  machine.write_rdram_u32_be(kFallthroughSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap backward branch-likely demo: " << label << '\n';
  std::cout << "before branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kRsIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kRtIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  const std::uint32_t branch_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord branch_decoded =
      Machine::decode_cpu_instruction_word(branch_raw);
  const Machine::CpuInstructionIdentity branch_identity =
      Machine::identify_cpu_instruction(branch_decoded);

  print_hex32("  branch_raw", branch_raw);
  std::cout << "  branch_identity = "
            << Machine::cpu_instruction_identity_name(branch_identity) << '\n';
  std::cout << "  decoded_immediate_i16 = "
            << branch_decoded.immediate_i16 << '\n';

  if (branch_identity != expected_identity) {
    throw std::runtime_error(
        std::string("backward branch-likely demo identified the wrong instruction: ") + label);
  }

  if (branch_decoded.immediate_i16 != expected_immediate) {
    throw std::runtime_error(
        std::string("backward branch-likely demo decoded the wrong signed immediate: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_branch");

  std::cout << "after branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not move into the delay slot: ") + label);
  }

  if (machine.cpu_next_pc() != kTargetAddress) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not schedule the backward target: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("backward branch-likely demo changed marker registers too early: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_delay_slot");

  std::cout << "after delay-slot step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != kTargetAddress) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not hand off to the backward target: ") + label);
  }

  if (machine.cpu_next_pc() != kTargetSentinelAddress) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not preserve sequential next_pc at the backward target: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) !=
      static_cast<std::uint32_t>(delay_slot_marker)) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not execute the delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("backward branch-likely demo changed path markers too early after the delay slot: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_target");

  std::cout << "after backward target step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != kTargetSentinelAddress) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not advance to the backward sentinel: ") + label);
  }

  if (machine.cpu_next_pc() != kBranchAddress) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not preserve the current pc/next_pc model after the backward target: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("backward branch-likely demo unexpectedly executed fallthrough: ") + label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) !=
      static_cast<std::uint32_t>(target_marker)) {
    throw std::runtime_error(
        std::string("backward branch-likely demo did not execute the backward target instruction: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_backward_break");
}

}  // namespace

void run_branch_likely_demos(Machine& machine) {
  constexpr std::uint16_t kTargetImmediate = 0x0003u;
  constexpr std::int16_t kBackwardImmediate = -3;

  run_branch_likely_demo(
      machine,
      "beql taken equality compare",
      0x00000300u,
      encode_beql(4, 5, kTargetImmediate),
      Machine::CpuInstructionIdentity::kBeql,
      0x11223344u,
      0x11223344u,
      true,
      0x7201u,
      0x7202u,
      0x7203u);

  run_branch_likely_demo(
      machine,
      "bnel not taken equality compare",
      0x00000320u,
      encode_bnel(4, 5, kTargetImmediate),
      Machine::CpuInstructionIdentity::kBnel,
      0x55667788u,
      0x55667788u,
      false,
      0x7211u,
      0x7212u,
      0x7213u);

  run_branch_likely_demo(
      machine,
      "blezl taken signed compare",
      0x00000340u,
      encode_blezl(4, kTargetImmediate),
      Machine::CpuInstructionIdentity::kBlezl,
      0xffffffffu,
      0,
      true,
      0x7221u,
      0x7222u,
      0x7223u);

  run_branch_likely_demo(
      machine,
      "bgtzl not taken signed compare",
      0x00000360u,
      encode_bgtzl(4, kTargetImmediate),
      Machine::CpuInstructionIdentity::kBgtzl,
      0,
      0,
      false,
      0x7231u,
      0x7232u,
      0x7233u);

  run_branch_likely_demo(
      machine,
      "regimm_bltzl taken signed compare",
      0x00000380u,
      encode_bltzl(4, kTargetImmediate),
      Machine::CpuInstructionIdentity::kRegimmBltzl,
      0xffffffffu,
      0,
      true,
      0x7241u,
      0x7242u,
      0x7243u);

  run_branch_likely_demo(
      machine,
      "regimm_bgezl not taken signed compare",
      0x000003a0u,
      encode_bgezl(4, kTargetImmediate),
      Machine::CpuInstructionIdentity::kRegimmBgezl,
      0xffffffffu,
      0,
      false,
      0x7251u,
      0x7252u,
      0x7253u);

  run_branch_likely_link_demo(
      machine,
      "regimm_bltzall taken signed compare",
      0x000003c0u,
      encode_bltzall(4, kTargetImmediate),
      Machine::CpuInstructionIdentity::kRegimmBltzall,
      0xffffffffu,
      true,
      0x7261u,
      0x7262u,
      0x7263u);

  run_branch_likely_link_demo(
      machine,
      "regimm_bgezall not taken signed compare",
      0x000003e0u,
      encode_bgezall(4, kTargetImmediate),
      Machine::CpuInstructionIdentity::kRegimmBgezall,
      0xffffffffu,
      false,
      0x7271u,
      0x7272u,
      0x7273u);

  run_backward_branch_likely_demo(
      machine,
      "beql taken backward negative offset",
      0x00000420u,
      encode_beql(4, 5, static_cast<std::uint16_t>(kBackwardImmediate)),
      Machine::CpuInstructionIdentity::kBeql,
      0x2468ace0u,
      0x2468ace0u,
      kBackwardImmediate,
      0x7281u,
      0x7282u,
      0x7283u);
}

}  // namespace fn64::bootstrap_detail