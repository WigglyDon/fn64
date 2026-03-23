#include "bootstrap_common.hpp"

#include <iomanip>
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
    Machine::CpuInstructionIdentity expected_identity,
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

  machine.write_cpu_pc(kJumpAddress);
  machine.write_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.write_cpu_gpr(kTargetMarkerIndex, 0);
  machine.write_cpu_gpr(kLinkIndex, 0);

  machine.write_rdram_u32_be(kJumpAddress, jump_instruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jump demo: " << label << '\n';
  std::cout << "before jump step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const std::uint32_t jump_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord jump_decoded =
      Machine::decode_cpu_instruction_word(jump_raw);
  const Machine::CpuInstructionIdentity jump_identity =
      Machine::identify_cpu_instruction(jump_decoded);

  print_hex32("  jump_raw", jump_raw);
  std::cout << "  jump_identity = "
            << Machine::cpu_instruction_identity_name(jump_identity) << '\n';

  if (jump_identity != expected_identity) {
    throw std::runtime_error(
        std::string("jump demo identified the wrong instruction: ") + label);
  }

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

  machine.write_cpu_pc(kJrAddress);
  machine.write_cpu_gpr(kTargetRegisterIndex, kTargetAddress);
  machine.write_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.write_cpu_gpr(kTargetMarkerIndex, 0);
  machine.write_cpu_gpr(kLinkIndex, 0);

  machine.write_rdram_u32_be(kJrAddress, kJrInstruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jump demo: special_jr explicit register target\n";
  std::cout << "before jump step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kTargetMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const std::uint32_t jr_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord jr_decoded =
      Machine::decode_cpu_instruction_word(jr_raw);
  const Machine::CpuInstructionIdentity jr_identity =
      Machine::identify_cpu_instruction(jr_decoded);

  print_hex32("  jr_raw", jr_raw);
  std::cout << "  jr_identity = "
            << Machine::cpu_instruction_identity_name(jr_identity) << '\n';

  if (jr_identity != Machine::CpuInstructionIdentity::kSpecialJr) {
    throw std::runtime_error("jr demo did not identify JR explicitly");
  }

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

void run_ordinary_branch_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t branch_instruction,
    Machine::CpuInstructionIdentity expected_identity,
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

  const std::uint32_t branch_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord branch_decoded =
      Machine::decode_cpu_instruction_word(branch_raw);
  const Machine::CpuInstructionIdentity branch_identity =
      Machine::identify_cpu_instruction(branch_decoded);

  print_hex32("  branch_raw", branch_raw);
  std::cout << "  branch_identity = "
            << Machine::cpu_instruction_identity_name(branch_identity) << '\n';

  if (branch_identity != expected_identity) {
    throw std::runtime_error(
        std::string("ordinary branch demo identified the wrong instruction: ") + label);
  }

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

void run_ordinary_jump_demos(Machine& machine) {
  run_jump_demo(
      machine,
      "j explicit target scheduling with normal delay slot",
      0x00000500u,
      encode_j(0x00000510u),
      Machine::CpuInstructionIdentity::kJ,
      false,
      0x7301u,
      0x7302u);

  run_jump_demo(
      machine,
      "jal explicit target scheduling with link",
      0x00000520u,
      encode_jal(0x00000530u),
      Machine::CpuInstructionIdentity::kJal,
      true,
      0x7311u,
      0x7312u);

  run_jr_demo(machine);
}

void run_ordinary_branch_demos(Machine& machine) {
  constexpr std::uint16_t kTargetImmediate = 0x0003u;

  run_ordinary_branch_demo(
      machine,
      "beq taken equality compare",
      0x00000560u,
      encode_beq(4, 5, kTargetImmediate),
      Machine::CpuInstructionIdentity::kBeq,
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
      Machine::CpuInstructionIdentity::kBne,
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
      Machine::CpuInstructionIdentity::kBlez,
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
      Machine::CpuInstructionIdentity::kBgtz,
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
      Machine::CpuInstructionIdentity::kRegimmBltz,
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
      Machine::CpuInstructionIdentity::kRegimmBgez,
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
      Machine::CpuInstructionIdentity::kRegimmBltzal,
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
      Machine::CpuInstructionIdentity::kRegimmBgezal,
      0xffffffffu,
      0u,
      false,
      true,
      0x73a1u,
      0x73a2u,
      0x73a3u);
}

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

    if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != static_cast<std::uint32_t>(delay_slot_marker)) {
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

    if (machine.read_cpu_gpr(kTargetMarkerIndex) != static_cast<std::uint32_t>(target_marker)) {
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

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != static_cast<std::uint32_t>(fallthrough_marker)) {
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

    if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != static_cast<std::uint32_t>(delay_slot_marker)) {
      throw std::runtime_error(
          std::string("branch-likely link taken demo did not execute delay slot: ") + label);
    }

    if (machine.read_cpu_gpr(kTargetMarkerIndex) != static_cast<std::uint32_t>(target_marker)) {
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

  if (machine.read_cpu_gpr(kFallthroughMarkerIndex) != static_cast<std::uint32_t>(fallthrough_marker)) {
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

void run_branch_likely_demos(Machine& machine) {
  constexpr std::uint16_t kTargetImmediate = 0x0003u;

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
}

void run_jalr_encoded_rd_demo(Machine& machine) {
  constexpr std::uint32_t kLoadTargetAddress = 0x00000000u;
  constexpr std::uint32_t kJalrAddress = 0x00000004u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000008u;
  constexpr std::uint32_t kLinkReturnAddress = 0x0000000cu;
  constexpr std::uint32_t kTargetAddress = 0x00000010u;
  constexpr std::uint32_t kSentinelAddress = 0x00000014u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(4, 0, 0x0010);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(7, 4);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(5, 0, 0x5555);
  constexpr std::uint32_t kTargetInstruction = encode_ori(6, 0, 0x6666);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kLoadTargetAddress);
  machine.write_cpu_gpr(4, 0);
  machine.write_cpu_gpr(5, 0);
  machine.write_cpu_gpr(6, 0);
  machine.write_cpu_gpr(7, 0);
  machine.write_cpu_gpr(31, 0);

  machine.write_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.write_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kLinkReturnAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 1: encoded rd link register\n";
  std::cout << "  ori_target_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kLoadTargetInstruction
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  delay_slot_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kDelaySlotInstruction
            << std::dec << std::setfill(' ') << '\n';

  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(4));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(7));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_load_target");
  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(4));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_issue_jalr");
  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[7]", machine.read_cpu_gpr(7));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error("jalr demo 1 did not move into the delay slot");
  }

  if (machine.cpu_next_pc() != kTargetAddress) {
    throw std::runtime_error("jalr demo 1 did not schedule the register target");
  }

  if (machine.read_cpu_gpr(7) != kLinkReturnAddress) {
    throw std::runtime_error("jalr demo 1 wrote the wrong link address");
  }

  if (machine.read_cpu_gpr(31) != 0) {
    throw std::runtime_error("jalr demo 1 unexpectedly touched gpr[31]");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_delay_slot");
  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.read_cpu_gpr(5));

  if (machine.cpu_pc() != kTargetAddress) {
    throw std::runtime_error("jalr demo 1 delay slot did not hand off to the target");
  }

  if (machine.read_cpu_gpr(5) != 0x00005555u) {
    throw std::runtime_error("jalr demo 1 delay slot did not execute");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_target");
  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(6));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error("jalr demo 1 ended at the wrong sentinel");
  }

  if (machine.read_cpu_gpr(6) != 0x00006666u) {
    throw std::runtime_error("jalr demo 1 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo1_break_stop");
}

void run_jalr_rd31_demo(Machine& machine) {
  constexpr std::uint32_t kLoadTargetAddress = 0x00000040u;
  constexpr std::uint32_t kJalrAddress = 0x00000044u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000048u;
  constexpr std::uint32_t kLinkReturnAddress = 0x0000004cu;
  constexpr std::uint32_t kTargetAddress = 0x00000050u;
  constexpr std::uint32_t kSentinelAddress = 0x00000054u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(4, 0, 0x0050);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(31, 4);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(8, 0, 0x8888);
  constexpr std::uint32_t kTargetInstruction = encode_ori(9, 0, 0x9999);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kLoadTargetAddress);
  machine.write_cpu_gpr(4, 0);
  machine.write_cpu_gpr(8, 0);
  machine.write_cpu_gpr(9, 0);
  machine.write_cpu_gpr(31, 0);

  machine.write_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.write_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kLinkReturnAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 2: rd = 31 normal link case\n";
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';

  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_load_target");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_issue_jalr");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_delay_slot");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_target");

  print_control_flow_state(machine);
  print_hex64("  gpr[8]", machine.read_cpu_gpr(8));
  print_hex64("  gpr[9]", machine.read_cpu_gpr(9));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error("jalr demo 2 ended at the wrong sentinel");
  }

  if (machine.read_cpu_gpr(31) != kLinkReturnAddress) {
    throw std::runtime_error("jalr demo 2 wrote the wrong return address into gpr[31]");
  }

  if (machine.read_cpu_gpr(8) != 0x00008888u) {
    throw std::runtime_error("jalr demo 2 delay slot did not execute");
  }

  if (machine.read_cpu_gpr(9) != 0x00009999u) {
    throw std::runtime_error("jalr demo 2 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo2_break_stop");
}

void run_jalr_rd0_demo(Machine& machine) {
  constexpr std::uint32_t kLoadTargetAddress = 0x00000080u;
  constexpr std::uint32_t kJalrAddress = 0x00000084u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000088u;
  constexpr std::uint32_t kTargetAddress = 0x00000090u;
  constexpr std::uint32_t kSentinelAddress = 0x00000094u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(10, 0, 0x0090);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(0, 10);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(11, 0, 0xabcd);
  constexpr std::uint32_t kTargetInstruction = encode_ori(12, 0, 0xdcba);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kLoadTargetAddress);
  machine.write_cpu_gpr(10, 0);
  machine.write_cpu_gpr(11, 0);
  machine.write_cpu_gpr(12, 0);

  machine.write_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.write_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kJalrAddress + 8u, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 3: rd = 0 discards link through normal gpr[0] behavior\n";
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';

  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_load_target");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_issue_jalr");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_delay_slot");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_target");

  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.read_cpu_gpr(0));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(11));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(12));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error("jalr demo 3 ended at the wrong sentinel");
  }

  if (machine.read_cpu_gpr(0) != 0) {
    throw std::runtime_error("jalr demo 3 unexpectedly changed gpr[0]");
  }

  if (machine.read_cpu_gpr(11) != 0x0000abcdu) {
    throw std::runtime_error("jalr demo 3 delay slot did not execute");
  }

  if (machine.read_cpu_gpr(12) != 0x0000dcbau) {
    throw std::runtime_error("jalr demo 3 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo3_break_stop");
}

}  // namespace

void run_control_demos(Machine& machine) {
  run_ordinary_jump_demos(machine);
  run_ordinary_branch_demos(machine);
  run_branch_likely_demos(machine);
  run_jalr_encoded_rd_demo(machine);
  run_jalr_rd31_demo(machine);
  run_jalr_rd0_demo(machine);
}

}  // namespace fn64::bootstrap_detail