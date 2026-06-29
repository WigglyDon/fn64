#include "bootstrap_common.hpp"

#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

void run_jump_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t jump_instruction,
    bool expect_link,
    std::uint16_t delay_slot_marker,
    std::uint16_t target_marker) {
  constexpr std::size_t kDelaySlotMarkerIndex = 6;
  constexpr std::size_t kTargetMarkerIndex = 7;
  constexpr std::size_t kLinkIndex = 31;

  const std::uint32_t kJumpAddress = base_address;
  const std::uint32_t kDelaySlotAddress = base_address + 4u;
  const std::uint32_t kLinkReturnAddress = base_address + 8u;
  const std::uint32_t kTargetAddress = base_address + 16u;
  const std::uint32_t kSentinelAddress = base_address + 20u;

  const std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, delay_slot_marker);
  const std::uint32_t kTargetInstruction = encode_ori(
      static_cast<std::uint8_t>(kTargetMarkerIndex), 0, target_marker);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(kJumpAddress);
  machine.stage_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.stage_cpu_gpr(kTargetMarkerIndex, 0);
  machine.stage_cpu_gpr(kLinkIndex, 0);

  machine.stage_rdram_u32_be(kJumpAddress, jump_instruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.stage_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jump demo: " << label << '\n';
  std::cout << "before jump step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const std::uint32_t jump_raw = jump_instruction;

  print_hex32("  jump_raw", jump_raw);

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_jump");

  std::cout << "after jump step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error(
        std::string("jump demo did not move into the delay slot: ") + label);
  }

  if (machine.cpu_next_pc() != kTargetAddress) {
    throw std::runtime_error(
        std::string("jump demo did not schedule the target: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("jump demo changed marker registers too early: ") + label);
  }

  const std::uint32_t expected_link = expect_link ? kLinkReturnAddress : 0u;
  if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
    throw std::runtime_error(
        std::string("jump demo wrote the wrong link value: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_delay_slot");

  std::cout << "after delay-slot step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kTargetAddress) {
    throw std::runtime_error(
        std::string("jump demo did not hand off to the target after the delay slot: ") + label);
  }

  if (machine.cpu_next_pc() != kSentinelAddress) {
    throw std::runtime_error(
        std::string("jump demo did not preserve sequential next_pc at the target: ") + label);
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) !=
      static_cast<std::uint32_t>(delay_slot_marker)) {
    throw std::runtime_error(
        std::string("jump demo did not execute the delay slot: ") + label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("jump demo executed the target too early: ") + label);
  }

  if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
    throw std::runtime_error(
        std::string("jump demo changed the link register after the delay slot: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_target");

  std::cout << "after target step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error(
        std::string("jump demo did not advance to the sentinel after the target: ") + label);
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) !=
      static_cast<std::uint32_t>(target_marker)) {
    throw std::runtime_error(
        std::string("jump demo did not execute the target instruction: ") + label);
  }

  if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
    throw std::runtime_error(
        std::string("jump demo changed the link register after the target: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_break");
}

void run_jr_demo(Machine& machine) {
  constexpr std::size_t kTargetRegisterIndex = 4;
  constexpr std::size_t kDelaySlotMarkerIndex = 6;
  constexpr std::size_t kTargetMarkerIndex = 7;
  constexpr std::size_t kLinkIndex = 31;

  constexpr std::uint32_t kJrAddress = 0x00000540u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000544u;
  constexpr std::uint32_t kTargetAddress = 0x00000550u;
  constexpr std::uint32_t kSentinelAddress = 0x00000554u;

  constexpr std::uint32_t kJrInstruction = encode_jr(
      static_cast<std::uint8_t>(kTargetRegisterIndex));
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, 0x7321u);
  constexpr std::uint32_t kTargetInstruction = encode_ori(
      static_cast<std::uint8_t>(kTargetMarkerIndex), 0, 0x7322u);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(kJrAddress);
  machine.stage_cpu_gpr(kTargetRegisterIndex, kTargetAddress);
  machine.stage_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.stage_cpu_gpr(kTargetMarkerIndex, 0);
  machine.stage_cpu_gpr(kLinkIndex, 0);

  machine.stage_rdram_u32_be(kJrAddress, kJrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.stage_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jump demo: special_jr explicit register target\n";
  std::cout << "before jump step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const std::uint32_t jr_raw = kJrInstruction;

  print_hex32("  jr_raw", jr_raw);

  require_stepped(machine.step_cpu_instruction(), "jr_demo_jump");

  std::cout << "after jump step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error("jr demo did not move into the delay slot");
  }

  if (machine.cpu_next_pc() != kTargetAddress) {
    throw std::runtime_error("jr demo did not schedule the register target");
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
      machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error("jr demo changed marker registers too early");
  }

  if (machine.read_cpu_gpr(kLinkIndex) != 0) {
    throw std::runtime_error("jr demo unexpectedly changed the link register");
  }

  require_stepped(machine.step_cpu_instruction(), "jr_demo_delay_slot");

  std::cout << "after delay-slot step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kTargetAddress) {
    throw std::runtime_error("jr demo did not hand off to the target after the delay slot");
  }

  if (machine.cpu_next_pc() != kSentinelAddress) {
    throw std::runtime_error("jr demo did not preserve sequential next_pc at the target");
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0x00007321u) {
    throw std::runtime_error("jr demo did not execute the delay slot");
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error("jr demo executed the target too early");
  }

  require_stepped(machine.step_cpu_instruction(), "jr_demo_target");

  std::cout << "after target step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error("jr demo did not advance to the sentinel after the target");
  }

  if (machine.read_cpu_gpr(kTargetMarkerIndex) != 0x00007322u) {
    throw std::runtime_error("jr demo did not execute the target instruction");
  }

  if (machine.read_cpu_gpr(kLinkIndex) != 0) {
    throw std::runtime_error("jr demo unexpectedly changed the link register");
  }

  require_stopped(machine.step_cpu_instruction(), "jr_demo_break");
}

}  // namespace

void run_ordinary_jump_demos(Machine& machine) {
  run_jump_demo(
      machine,
      "j explicit target scheduling with normal delay slot",
      0x00000500u,
      encode_j(0x00000510u),
      false,
      0x7301u,
      0x7302u);

  run_jump_demo(
      machine,
      "jal explicit target scheduling with link",
      0x00000520u,
      encode_jal(0x00000530u),
      true,
      0x7311u,
      0x7312u);

  run_jr_demo(machine);
}

}  // namespace fn64::bootstrap_detail
