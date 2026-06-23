#include "bootstrap_common.hpp"

#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

void run_ordinary_branch_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t branch_instruction,
    std::uint32_t rs_value,
    std::uint32_t rt_value,
    bool expect_taken,
    bool expect_link,
    std::uint16_t delay_slot_marker,
    std::uint16_t fallthrough_marker,
    std::uint16_t target_marker) {
  constexpr std::size_t kRsIndex = 4;
  constexpr std::size_t kRtIndex = 5;
  constexpr std::size_t kDelaySlotMarkerIndex = 6;
  constexpr std::size_t kFallthroughMarkerIndex = 7;
  constexpr std::size_t kTargetMarkerIndex = 8;
  constexpr std::size_t kLinkIndex = 31;

  const std::uint32_t kBranchAddress = base_address;
  const std::uint32_t kDelaySlotAddress = base_address + 4u;
  const std::uint32_t kLinkReturnAddress = base_address + 8u;
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
  machine.write_cpu_gpr(kLinkIndex, 0);

  machine.write_rdram_u32_be(kBranchAddress, branch_instruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kFallthroughAddress, kFallthroughInstruction);
  machine.write_rdram_u32_be(kNotTakenSentinelAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kTakenSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap ordinary branch demo: " << label << '\n';
  std::cout << "before branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kRsIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kRtIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const std::uint32_t branch_raw = branch_instruction;

  print_hex32("  branch_raw", branch_raw);

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_branch");

  std::cout << "after branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error(
        std::string("ordinary branch demo did not move into the delay slot: ") + label);
  }

  const std::uint32_t expected_post_branch_next_pc =
      expect_taken ? kTargetAddress : kFallthroughAddress;
  if (machine.cpu_next_pc() != expected_post_branch_next_pc) {
    throw std::runtime_error(
        std::string("ordinary branch demo scheduled the wrong next_pc: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("ordinary branch demo changed marker registers too early: ") + label);
  }

  const std::uint32_t expected_link = expect_link ? kLinkReturnAddress : 0u;
  if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
    throw std::runtime_error(
        std::string("ordinary branch demo wrote the wrong link value: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_delay_slot");

  std::cout << "after delay-slot step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const std::uint32_t expected_post_delay_pc =
      expect_taken ? kTargetAddress : kFallthroughAddress;
  const std::uint32_t expected_post_delay_next_pc =
      expect_taken ? kTakenSentinelAddress : kNotTakenSentinelAddress;

  if (machine.cpu_pc() != expected_post_delay_pc) {
    throw std::runtime_error(
        std::string("ordinary branch demo did not hand off to the right path after the delay slot: ") + label);
  }

  if (machine.cpu_next_pc() != expected_post_delay_next_pc) {
    throw std::runtime_error(
        std::string("ordinary branch demo preserved the wrong next_pc after the delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) !=
      static_cast<std::uint32_t>(delay_slot_marker)) {
    throw std::runtime_error(
        std::string("ordinary branch demo did not execute the delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
    throw std::runtime_error(
        std::string("ordinary branch demo changed the link register after the delay slot: ") + label);
  }

  if (expect_taken) {
    if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
        machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("ordinary branch taken demo changed markers too early: ") + label);
    }

    require_stepped(machine.step_cpu_instruction(), std::string(label) + "_target");

    std::cout << "after target step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
    print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
    print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
    print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

    if (machine.cpu_pc() != kTakenSentinelAddress) {
      throw std::runtime_error(
          std::string("ordinary branch taken demo did not advance to the taken sentinel: ") + label);
    }

    if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("ordinary branch taken demo unexpectedly executed fallthrough: ") + label);
    }

    if (machine.read_cpu_gpr(kTargetMarkerIndex) !=
        static_cast<std::uint32_t>(target_marker)) {
      throw std::runtime_error(
          std::string("ordinary branch taken demo did not execute the target instruction: ") + label);
    }

    if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
      throw std::runtime_error(
          std::string("ordinary branch taken demo changed the link register after the target: ") + label);
    }

    require_stopped(machine.step_cpu_instruction(), std::string(label) + "_taken_break");
    return;
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("ordinary branch not-taken demo changed markers too early: ") + label);
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
        std::string("ordinary branch not-taken demo did not advance to the not-taken sentinel: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) !=
      static_cast<std::uint32_t>(fallthrough_marker)) {
    throw std::runtime_error(
        std::string("ordinary branch not-taken demo did not execute fallthrough: ") + label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("ordinary branch not-taken demo unexpectedly executed the target: ") + label);
  }

  if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
    throw std::runtime_error(
        std::string("ordinary branch not-taken demo changed the link register after fallthrough: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_not_taken_break");
}

void run_aliased_register_link_branch_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t branch_instruction,
    std::uint32_t original_aliased_source_value,
    bool expect_taken,
    std::uint16_t delay_slot_marker,
    std::uint16_t fallthrough_marker,
    std::uint16_t target_marker) {
  constexpr std::size_t kAliasedSourceAndLinkIndex = 31;
  constexpr std::size_t kDelaySlotMarkerIndex = 6;
  constexpr std::size_t kFallthroughMarkerIndex = 7;
  constexpr std::size_t kTargetMarkerIndex = 8;

  const std::uint32_t kBranchAddress = base_address;
  const std::uint32_t kDelaySlotAddress = base_address + 4u;
  const std::uint32_t kLinkReturnAddress = base_address + 8u;
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
  machine.write_cpu_gpr(kAliasedSourceAndLinkIndex, original_aliased_source_value);
  machine.write_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.write_cpu_gpr(kFallthroughMarkerIndex, 0);
  machine.write_cpu_gpr(kTargetMarkerIndex, 0);

  machine.write_rdram_u32_be(kBranchAddress, branch_instruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kFallthroughAddress, kFallthroughInstruction);
  machine.write_rdram_u32_be(kNotTakenSentinelAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kTakenSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap aliased register-link branch demo: " << label << '\n';
  std::cout << "before branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kAliasedSourceAndLinkIndex));

  const std::uint32_t branch_raw = branch_instruction;

  print_hex32("  branch_raw", branch_raw);

  if ((original_aliased_source_value & 0x80000000u) == 0) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo expected a negative original rs value: ") +
        label);
  }

  if ((kLinkReturnAddress & 0x80000000u) != 0) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo expected a non-negative link value: ") +
        label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_branch");

  std::cout << "after branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kAliasedSourceAndLinkIndex));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo did not move into the delay slot: ") + label);
  }

  const std::uint32_t expected_post_branch_next_pc =
      expect_taken ? kTargetAddress : kFallthroughAddress;
  if (machine.cpu_next_pc() != expected_post_branch_next_pc) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo scheduled the wrong next_pc: ") + label);
  }

  if (machine.read_cpu_gpr(kAliasedSourceAndLinkIndex) != kLinkReturnAddress) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo wrote the wrong link value: ") + label);
  }

  if (machine.read_cpu_gpr(kAliasedSourceAndLinkIndex) == original_aliased_source_value) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo did not overwrite the aliased register: ") +
        label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo changed marker registers too early: ") +
        label);
  }

  if (expect_taken) {
    if (machine.cpu_next_pc() == machine.read_cpu_gpr(kAliasedSourceAndLinkIndex)) {
      throw std::runtime_error(
          std::string("aliased register-link branch taken demo scheduled the post-link value instead of the original source decision: ") +
          label);
    }
  } else {
    if (machine.cpu_next_pc() != machine.read_cpu_gpr(kAliasedSourceAndLinkIndex)) {
      throw std::runtime_error(
          std::string("aliased register-link branch not-taken demo did not preserve fallthrough/link equivalence: ") +
          label);
    }

    if (kTargetAddress == machine.read_cpu_gpr(kAliasedSourceAndLinkIndex)) {
      throw std::runtime_error(
          std::string("aliased register-link branch not-taken demo chose a target indistinguishable from the link value: ") +
          label);
    }

    if (machine.cpu_next_pc() == kTargetAddress) {
      throw std::runtime_error(
          std::string("aliased register-link branch not-taken demo scheduled the target from the post-link value: ") +
          label);
    }
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_delay_slot");

  std::cout << "after delay-slot step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kAliasedSourceAndLinkIndex));

  const std::uint32_t expected_post_delay_pc =
      expect_taken ? kTargetAddress : kFallthroughAddress;
  const std::uint32_t expected_post_delay_next_pc =
      expect_taken ? kTakenSentinelAddress : kNotTakenSentinelAddress;

  if (machine.cpu_pc() != expected_post_delay_pc) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo did not hand off to the right path after the delay slot: ") +
        label);
  }

  if (machine.cpu_next_pc() != expected_post_delay_next_pc) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo preserved the wrong next_pc after the delay slot: ") +
        label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) !=
      static_cast<std::uint32_t>(delay_slot_marker)) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo did not execute the delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kAliasedSourceAndLinkIndex) != kLinkReturnAddress) {
    throw std::runtime_error(
        std::string("aliased register-link branch demo changed gpr[31] after the delay slot: ") +
        label);
  }

  if (expect_taken) {
    if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
        machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("aliased register-link branch taken demo changed markers too early after the delay slot: ") +
          label);
    }

    require_stepped(machine.step_cpu_instruction(), std::string(label) + "_target");

    std::cout << "after target step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
    print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
    print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
    print_hex64("  gpr[31]", machine.read_cpu_gpr(kAliasedSourceAndLinkIndex));

    if (machine.cpu_pc() != kTakenSentinelAddress) {
      throw std::runtime_error(
          std::string("aliased register-link branch taken demo did not advance to the taken sentinel: ") +
          label);
    }

    if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("aliased register-link branch taken demo unexpectedly executed fallthrough: ") +
          label);
    }

    if (machine.read_cpu_gpr(kTargetMarkerIndex) !=
        static_cast<std::uint32_t>(target_marker)) {
      throw std::runtime_error(
          std::string("aliased register-link branch taken demo did not execute the target instruction: ") +
          label);
    }

    if (machine.read_cpu_gpr(kAliasedSourceAndLinkIndex) != kLinkReturnAddress) {
      throw std::runtime_error(
          std::string("aliased register-link branch taken demo changed gpr[31] after the target: ") +
          label);
    }

    require_stopped(machine.step_cpu_instruction(), std::string(label) + "_taken_break");
    return;
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("aliased register-link branch not-taken demo changed markers too early after the delay slot: ") +
        label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_fallthrough");

  std::cout << "after fallthrough step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kAliasedSourceAndLinkIndex));

  if (machine.cpu_pc() != kNotTakenSentinelAddress) {
    throw std::runtime_error(
        std::string("aliased register-link branch not-taken demo did not advance to the not-taken sentinel: ") +
        label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) !=
      static_cast<std::uint32_t>(fallthrough_marker)) {
    throw std::runtime_error(
        std::string("aliased register-link branch not-taken demo did not execute fallthrough: ") +
        label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("aliased register-link branch not-taken demo unexpectedly executed the target: ") +
        label);
  }

  if (machine.read_cpu_gpr(kAliasedSourceAndLinkIndex) != kLinkReturnAddress) {
    throw std::runtime_error(
        std::string("aliased register-link branch not-taken demo changed gpr[31] after fallthrough: ") +
        label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_not_taken_break");
}

void run_backward_ordinary_branch_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t branch_instruction,
    std::uint32_t rs_value,
    std::uint32_t rt_value,
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

  std::cout << "fn64 bootstrap backward ordinary branch demo: " << label << '\n';
  std::cout << "before branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kRsIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kRtIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  const std::uint32_t branch_raw = branch_instruction;

  print_hex32("  branch_raw", branch_raw);

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_branch");

  std::cout << "after branch step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not move into the delay slot: ") + label);
  }

  if (machine.cpu_next_pc() != kTargetAddress) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not schedule the backward target: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo changed marker registers too early: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_delay_slot");

  std::cout << "after delay-slot step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != kTargetAddress) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not hand off to the backward target: ") + label);
  }

  if (machine.cpu_next_pc() != kTargetSentinelAddress) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not preserve sequential next_pc at the backward target: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) !=
      static_cast<std::uint32_t>(delay_slot_marker)) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not execute the delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo changed path markers too early after the delay slot: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_target");

  std::cout << "after backward target step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kFallthroughMarkerIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != kTargetSentinelAddress) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not advance to the backward sentinel: ") + label);
  }

  if (machine.cpu_next_pc() != kBranchAddress) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not preserve the current pc/next_pc model after the backward target: ") + label);
  }

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo unexpectedly executed fallthrough: ") + label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) !=
      static_cast<std::uint32_t>(target_marker)) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo did not execute the backward target instruction: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_backward_break");
}

}  // namespace

void run_ordinary_branch_demos(Machine& machine) {
  constexpr std::uint16_t kTargetImmediate = 0x0003u;
  constexpr std::int16_t kBackwardImmediate = -3;

  run_ordinary_branch_demo(
      machine,
      "beq taken equality compare",
      0x00000560u,
      encode_beq(4, 5, kTargetImmediate),
      0x11223344u,
      0x11223344u,
      true,
      false,
      0x7331u,
      0x7332u,
      0x7333u);

  run_ordinary_branch_demo(
      machine,
      "bne not taken equality compare",
      0x00000580u,
      encode_bne(4, 5, kTargetImmediate),
      0x55667788u,
      0x55667788u,
      false,
      false,
      0x7341u,
      0x7342u,
      0x7343u);

  run_ordinary_branch_demo(
      machine,
      "blez taken signed compare",
      0x000005a0u,
      encode_blez(4, kTargetImmediate),
      0u,
      0u,
      true,
      false,
      0x7351u,
      0x7352u,
      0x7353u);

  run_ordinary_branch_demo(
      machine,
      "bgtz not taken signed compare",
      0x000005c0u,
      encode_bgtz(4, kTargetImmediate),
      0u,
      0u,
      false,
      false,
      0x7361u,
      0x7362u,
      0x7363u);

  run_ordinary_branch_demo(
      machine,
      "regimm_bltz taken signed compare",
      0x000005e0u,
      encode_bltz(4, kTargetImmediate),
      0xffffffffu,
      0u,
      true,
      false,
      0x7371u,
      0x7372u,
      0x7373u);

  run_ordinary_branch_demo(
      machine,
      "regimm_bgez not taken signed compare",
      0x00000600u,
      encode_bgez(4, kTargetImmediate),
      0xffffffffu,
      0u,
      false,
      false,
      0x7381u,
      0x7382u,
      0x7383u);

  run_ordinary_branch_demo(
      machine,
      "regimm_bltzal taken signed compare with link",
      0x00000620u,
      encode_bltzal(4, kTargetImmediate),
      0xffffffffu,
      0u,
      true,
      true,
      0x7391u,
      0x7392u,
      0x7393u);

  run_ordinary_branch_demo(
      machine,
      "regimm_bgezal not taken signed compare with unconditional link",
      0x00000640u,
      encode_bgezal(4, kTargetImmediate),
      0xffffffffu,
      0u,
      false,
      true,
      0x73a1u,
      0x73a2u,
      0x73a3u);

  run_aliased_register_link_branch_demo(
      machine,
      "regimm_bltzal taken signed compare with rs == 31 reads original source before link",
      0x00000780u,
      encode_bltzal(31, kTargetImmediate),
      0xffffffffu,
      true,
      0x73c1u,
      0x73c2u,
      0x73c3u);

  run_aliased_register_link_branch_demo(
      machine,
      "regimm_bgezal not taken signed compare with rs == 31 still links after reading original source",
      0x000007a0u,
      encode_bgezal(31, kTargetImmediate),
      0xffffffffu,
      false,
      0x73d1u,
      0x73d2u,
      0x73d3u);

  run_backward_ordinary_branch_demo(
      machine,
      "beq taken backward negative offset",
      0x00000660u,
      encode_beq(4, 5, static_cast<std::uint16_t>(kBackwardImmediate)),
      0x13579bdfu,
      0x13579bdfu,
      0x73b1u,
      0x73b2u,
      0x73b3u);
}

}  // namespace fn64::bootstrap_detail
