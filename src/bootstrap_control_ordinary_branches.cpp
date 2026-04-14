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

void run_backward_ordinary_branch_demo(
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

  std::cout << "fn64 bootstrap backward ordinary branch demo: " << label << '\n';
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
        std::string("backward ordinary branch demo identified the wrong instruction: ") + label);
  }

  if (branch_decoded.immediate_i16 != expected_immediate) {
    throw std::runtime_error(
        std::string("backward ordinary branch demo decoded the wrong signed immediate: ") + label);
  }

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

  run_backward_ordinary_branch_demo(
      machine,
      "beq taken backward negative offset",
      0x00000660u,
      encode_beq(4, 5, static_cast<std::uint16_t>(kBackwardImmediate)),
      Machine::CpuInstructionIdentity::kBeq,
      0x13579bdfu,
      0x13579bdfu,
      kBackwardImmediate,
      0x73b1u,
      0x73b2u,
      0x73b3u);
}

}  // namespace fn64::bootstrap_detail