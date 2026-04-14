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

void run_high_region_jump_target_execute_demo(
    Machine& machine,
    const char* label,
    std::uint32_t current_pc,
    std::uint32_t jump_instruction,
    Machine::CpuInstructionIdentity expected_identity,
    std::uint32_t expected_target,
    bool expect_link) {
  constexpr std::size_t kLinkIndex = 31;

  machine.write_cpu_pc(current_pc);
  machine.write_cpu_gpr(kLinkIndex, 0);

  std::cout << "fn64 bootstrap upper-region jump target demo: " << label << '\n';
  std::cout << "before execute:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const Machine::DecodedCpuInstructionWord jump_decoded =
      Machine::decode_cpu_instruction_word(jump_instruction);
  const Machine::CpuInstructionIdentity jump_identity =
      Machine::identify_cpu_instruction(jump_decoded);
  const std::uint32_t pc_plus_4_upper_nibble = machine.cpu_next_pc() & 0xf0000000u;
  const std::uint32_t low_28_from_field = jump_decoded.jump_target << 2;

  print_hex32("  jump_raw", jump_instruction);
  std::cout << "  jump_identity = "
            << Machine::cpu_instruction_identity_name(jump_identity) << '\n';
  print_hex32("  pc_plus_4_upper_nibble", pc_plus_4_upper_nibble);
  print_hex32("  low_28_from_field", low_28_from_field);
  print_hex32("  expected_target", expected_target);

  if (jump_identity != expected_identity) {
    throw std::runtime_error(
        std::string("upper-region jump demo identified the wrong instruction: ") + label);
  }

  const Machine::CpuInstructionExecutionResult execution_result =
      machine.execute_cpu_instruction(jump_identity, jump_decoded);
  if (execution_result != Machine::CpuInstructionExecutionResult::kExecuted) {
    throw std::runtime_error(
        std::string("upper-region jump demo did not execute cleanly: ") + label);
  }

  std::cout << "after execute:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != current_pc) {
    throw std::runtime_error(
        std::string("upper-region jump demo unexpectedly advanced pc during execute: ") + label);
  }

  if (machine.cpu_next_pc() != expected_target) {
    throw std::runtime_error(
        std::string("upper-region jump demo formed the wrong composed target: ") + label);
  }

  const std::uint32_t expected_link = expect_link ? (current_pc + 8u) : 0u;
  if (machine.read_cpu_gpr(kLinkIndex) != expected_link) {
    throw std::runtime_error(
        std::string("upper-region jump demo wrote the wrong link value: ") + label);
  }
}

void run_jal_link_carry_edge_demo(Machine& machine) {
  constexpr std::uint32_t kCurrentPc = 0x0ffffffcu;
  constexpr std::uint32_t kExpectedPcPlus4 = 0x10000000u;
  constexpr std::uint32_t kExpectedLinkValue = 0x10000004u;
  constexpr std::uint32_t kExpectedTarget = 0x10005678u;
  constexpr std::uint32_t kJalInstruction = encode_jal(0x00005678u);

  machine.write_cpu_pc(kCurrentPc);
  machine.write_cpu_gpr(31, 0);

  std::cout << "fn64 bootstrap carry-edge link demo: jal link and target composition across upper-region carry\n";
  std::cout << "before execute:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  const Machine::DecodedCpuInstructionWord decoded =
      Machine::decode_cpu_instruction_word(kJalInstruction);
  const Machine::CpuInstructionIdentity identity =
      Machine::identify_cpu_instruction(decoded);

  print_hex32("  jal_raw", kJalInstruction);
  std::cout << "  jal_identity = "
            << Machine::cpu_instruction_identity_name(identity) << '\n';
  print_hex32("  expected_pc_plus_4", kExpectedPcPlus4);
  print_hex32("  expected_link_value", kExpectedLinkValue);
  print_hex32("  expected_target", kExpectedTarget);

  if (identity != Machine::CpuInstructionIdentity::kJal) {
    throw std::runtime_error("jal carry-edge demo did not identify JAL explicitly");
  }

  if (machine.cpu_next_pc() != kExpectedPcPlus4) {
    throw std::runtime_error("jal carry-edge demo did not start with the expected pc + 4 value");
  }

  const Machine::CpuInstructionExecutionResult execution_result =
      machine.execute_cpu_instruction(identity, decoded);
  if (execution_result != Machine::CpuInstructionExecutionResult::kExecuted) {
    throw std::runtime_error("jal carry-edge demo did not execute cleanly");
  }

  std::cout << "after execute:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  if (machine.cpu_pc() != kCurrentPc) {
    throw std::runtime_error("jal carry-edge demo unexpectedly advanced pc during execute");
  }

  if (machine.cpu_next_pc() != kExpectedTarget) {
    throw std::runtime_error("jal carry-edge demo formed the wrong target");
  }

  if (machine.read_cpu_gpr(31) != kExpectedLinkValue) {
    throw std::runtime_error("jal carry-edge demo wrote the wrong link value");
  }
}

void run_jalr_link_carry_edge_demo(Machine& machine) {
  constexpr std::size_t kTargetRegisterIndex = 21;
  constexpr std::size_t kDelaySlotMarkerIndex = 22;
  constexpr std::size_t kLinkIndex = 31;

  constexpr std::uint32_t kCurrentPc = 0x0ffffffcu;
  constexpr std::uint32_t kExpectedPcPlus4 = 0x10000000u;
  constexpr std::uint32_t kExpectedLinkValue = 0x10000004u;
  constexpr std::uint32_t kTargetAddress = 0x00000740u;
  constexpr std::uint32_t kJalrInstruction = encode_jalr(
      static_cast<std::uint8_t>(kLinkIndex),
      static_cast<std::uint8_t>(kTargetRegisterIndex));

  machine.write_cpu_pc(kCurrentPc);
  machine.write_cpu_gpr(kTargetRegisterIndex, kTargetAddress);
  machine.write_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.write_cpu_gpr(kLinkIndex, 0);

  std::cout << "fn64 bootstrap carry-edge link demo: jalr link and target scheduling across upper-region carry\n";
  std::cout << "before execute:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[21]", machine.read_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[22]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  const Machine::DecodedCpuInstructionWord decoded =
      Machine::decode_cpu_instruction_word(kJalrInstruction);
  const Machine::CpuInstructionIdentity identity =
      Machine::identify_cpu_instruction(decoded);

  print_hex32("  jalr_raw", kJalrInstruction);
  std::cout << "  jalr_identity = "
            << Machine::cpu_instruction_identity_name(identity) << '\n';
  std::cout << "  decoded_rs = " << static_cast<unsigned>(decoded.rs) << '\n';
  std::cout << "  decoded_rd = " << static_cast<unsigned>(decoded.rd) << '\n';
  print_hex32("  expected_pc_plus_4", kExpectedPcPlus4);
  print_hex32("  expected_link_value", kExpectedLinkValue);
  print_hex32("  expected_target", kTargetAddress);

  if (identity != Machine::CpuInstructionIdentity::kSpecialJalr) {
    throw std::runtime_error("jalr carry-edge demo did not identify JALR explicitly");
  }

  if (decoded.rs != kTargetRegisterIndex || decoded.rd != kLinkIndex) {
    throw std::runtime_error("jalr carry-edge demo decoded the wrong rd/rs pair");
  }

  if (machine.cpu_next_pc() != kExpectedPcPlus4) {
    throw std::runtime_error("jalr carry-edge demo did not start with the expected pc + 4 value");
  }

  const Machine::CpuInstructionExecutionResult execution_result =
      machine.execute_cpu_instruction(identity, decoded);
  if (execution_result != Machine::CpuInstructionExecutionResult::kExecuted) {
    throw std::runtime_error("jalr carry-edge demo did not execute cleanly");
  }

  std::cout << "after execute:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[21]", machine.read_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[22]", machine.read_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != kCurrentPc) {
    throw std::runtime_error("jalr carry-edge demo unexpectedly advanced pc during execute");
  }

  if (machine.cpu_next_pc() != kTargetAddress) {
    throw std::runtime_error("jalr carry-edge demo scheduled the wrong target");
  }

  if (machine.read_cpu_gpr(kLinkIndex) != kExpectedLinkValue) {
    throw std::runtime_error("jalr carry-edge demo wrote the wrong link value");
  }

  if (machine.read_cpu_gpr(kTargetRegisterIndex) != kTargetAddress) {
    throw std::runtime_error("jalr carry-edge demo changed the target register unexpectedly");
  }

  if (machine.read_cpu_gpr(kDelaySlotMarkerIndex) != 0) {
    throw std::runtime_error("jalr carry-edge demo changed unrelated state during execute");
  }
}

}  // namespace

void run_ordinary_jump_demos(Machine& machine) {
  constexpr std::uint32_t kCarryPc = 0x0ffffffcu;

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

  run_high_region_jump_target_execute_demo(
      machine,
      "j uses upper nibble from pc + 4 carry",
      kCarryPc,
      encode_j(0x00001234u),
      Machine::CpuInstructionIdentity::kJ,
      0x10001234u,
      false);

  run_high_region_jump_target_execute_demo(
      machine,
      "jal uses upper nibble from pc + 4 carry with link",
      kCarryPc,
      encode_jal(0x00005678u),
      Machine::CpuInstructionIdentity::kJal,
      0x10005678u,
      true);

  run_jal_link_carry_edge_demo(machine);
  run_jalr_link_carry_edge_demo(machine);
}

}  // namespace fn64::bootstrap_detail