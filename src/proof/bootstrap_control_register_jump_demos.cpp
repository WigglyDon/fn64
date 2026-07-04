#include "bootstrap_common.hpp"

#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

void require_control_transfer_fault(
    const MachineFault& fault,
    const char* label) {
  if (fault.kind() != MachineFaultKind::kUnalignedControlTransferTarget) {
    throw std::runtime_error(std::string(label) + " threw unexpected MachineFault kind");
  }

  if (fault.access_size() != 4) {
    throw std::runtime_error(std::string(label) + " threw unexpected MachineFault access size");
  }
}

void run_jr_misaligned_target_demo(Machine& machine) {
  constexpr std::size_t kTargetRegisterIndex = 16;
  constexpr std::size_t kDelaySlotMarkerIndex = 17;
  constexpr std::uint32_t kJrAddress = 0x00000680u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000684u;
  constexpr std::uint32_t kMisalignedTargetAddress = 0x00000692u;

  constexpr std::uint32_t kJrInstruction = encode_jr(
      static_cast<std::uint8_t>(kTargetRegisterIndex));
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, 0x74a1u);

  machine.stage_cpu_pc(cpu_rdram_alias(kJrAddress));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kJrAddress + 8u));
  machine.stage_cpu_gpr(kTargetRegisterIndex, cpu_rdram_alias(kMisalignedTargetAddress));
  machine.stage_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.stage_cpu_gpr(31, 0);

  machine.stage_rdram_u32_be(kJrAddress, kJrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);

  std::cout << "fn64 bootstrap jump failure demo: special_jr misaligned register target\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[16]", machine.inspect_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[17]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(31));

  const std::uint32_t raw = kJrInstruction;

  print_hex32("  jr_raw", raw);

  const std::uint32_t preserved_pc = machine.cpu_pc();
  const std::uint32_t preserved_next_pc = machine.cpu_next_pc();

  try {
    static_cast<void>(machine.step_cpu_instruction());
    throw std::runtime_error("jr misaligned demo expected step_cpu_instruction to throw");
  } catch (const MachineFault& error) {
    std::cout << "  jr_misaligned_step threw: " << error.what() << '\n';
    require_control_transfer_fault(error, "jr_misaligned_step");
  }

  std::cout << "after failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[16]", machine.inspect_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[17]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(31));

  if (machine.cpu_pc() != preserved_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error("jr misaligned demo did not preserve pc/next_pc rollback");
  }

  if (machine.inspect_cpu_gpr(kTargetRegisterIndex) !=
      cpu_rdram_alias(kMisalignedTargetAddress)) {
    throw std::runtime_error("jr misaligned demo changed the target register");
  }

  if (machine.inspect_cpu_gpr(kDelaySlotMarkerIndex) != 0) {
    throw std::runtime_error("jr misaligned demo executed or modified the delay slot path");
  }

  if (machine.inspect_cpu_gpr(31) != 0) {
    throw std::runtime_error("jr misaligned demo unexpectedly touched the link register");
  }
}

void run_jalr_rd_equals_rs_misaligned_target_demo(Machine& machine) {
  constexpr std::size_t kAliasedRegisterIndex = 13;
  constexpr std::size_t kDelaySlotMarkerIndex = 14;
  constexpr std::size_t kTargetMarkerIndex = 15;

  constexpr std::uint32_t kJalrAddress = 0x00000700u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000704u;
  constexpr std::uint32_t kMisalignedTargetAddress = 0x00000712u;
  constexpr std::uint32_t kExpectedLinkValue = 0x00000708u;

  constexpr std::uint32_t kJalrInstruction = encode_jalr(
      static_cast<std::uint8_t>(kAliasedRegisterIndex),
      static_cast<std::uint8_t>(kAliasedRegisterIndex));
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, 0x74c1u);

  machine.stage_cpu_pc(cpu_rdram_alias(kJalrAddress));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kJalrAddress + 8u));
  machine.stage_cpu_gpr(kAliasedRegisterIndex, cpu_rdram_alias(kMisalignedTargetAddress));
  machine.stage_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.stage_cpu_gpr(kTargetMarkerIndex, 0);

  machine.stage_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);

  std::cout << "fn64 bootstrap jump failure demo: special_jalr rd == rs misaligned target reads original register before link\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));
  print_hex64("  gpr[14]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kTargetMarkerIndex));

  const std::uint32_t raw = kJalrInstruction;

  print_hex32("  jalr_raw", raw);

  const std::uint32_t preserved_pc = machine.cpu_pc();
  const std::uint32_t preserved_next_pc = machine.cpu_next_pc();

  try {
    static_cast<void>(machine.step_cpu_instruction());
    throw std::runtime_error("jalr rd == rs misaligned demo expected step_cpu_instruction to throw");
  } catch (const MachineFault& error) {
    std::cout << "  jalr_rd_equals_rs_misaligned_step threw: " << error.what() << '\n';
    require_control_transfer_fault(error, "jalr_rd_equals_rs_misaligned_step");
  }

  std::cout << "after failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));
  print_hex64("  gpr[14]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != preserved_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error("jalr rd == rs misaligned demo did not preserve pc/next_pc rollback");
  }

  if (machine.inspect_cpu_gpr(kAliasedRegisterIndex) !=
      cpu_rdram_alias(kMisalignedTargetAddress)) {
    throw std::runtime_error("jalr rd == rs misaligned demo changed the aliased target register");
  }

  if (machine.inspect_cpu_gpr(kAliasedRegisterIndex) ==
      cpu_rdram_alias(kExpectedLinkValue)) {
    throw std::runtime_error("jalr rd == rs misaligned demo leaked the speculative link value");
  }

  if (machine.inspect_cpu_gpr(kDelaySlotMarkerIndex) != 0) {
    throw std::runtime_error("jalr rd == rs misaligned demo executed or modified the delay slot path");
  }

  if (machine.inspect_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error("jalr rd == rs misaligned demo changed unrelated marker state");
  }
}

void run_jalr_rd31_misaligned_target_demo(Machine& machine) {
  constexpr std::size_t kTargetRegisterIndex = 18;
  constexpr std::size_t kDelaySlotMarkerIndex = 19;
  constexpr std::size_t kLinkIndex = 31;

  constexpr std::uint32_t kJalrAddress = 0x00000720u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000724u;
  constexpr std::uint32_t kMisalignedTargetAddress = 0x00000732u;
  constexpr std::uint32_t kInitialLinkValue = 0x13579bdfu;
  constexpr std::uint32_t kExpectedLinkValue = 0x00000728u;

  constexpr std::uint32_t kJalrInstruction = encode_jalr(
      static_cast<std::uint8_t>(kLinkIndex),
      static_cast<std::uint8_t>(kTargetRegisterIndex));
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, 0x74d1u);

  machine.stage_cpu_pc(cpu_rdram_alias(kJalrAddress));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kJalrAddress + 8u));
  machine.stage_cpu_gpr(kTargetRegisterIndex, cpu_rdram_alias(kMisalignedTargetAddress));
  machine.stage_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.stage_cpu_gpr(kLinkIndex, kInitialLinkValue);

  machine.stage_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);

  std::cout << "fn64 bootstrap jump failure demo: special_jalr rd = 31 misaligned register target leaves link untouched\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(kLinkIndex));

  const std::uint32_t raw = kJalrInstruction;

  print_hex32("  jalr_raw", raw);

  const std::uint32_t preserved_pc = machine.cpu_pc();
  const std::uint32_t preserved_next_pc = machine.cpu_next_pc();

  try {
    static_cast<void>(machine.step_cpu_instruction());
    throw std::runtime_error("jalr rd = 31 misaligned demo expected step_cpu_instruction to throw");
  } catch (const MachineFault& error) {
    std::cout << "  jalr_rd31_misaligned_step threw: " << error.what() << '\n';
    require_control_transfer_fault(error, "jalr_rd31_misaligned_step");
  }

  std::cout << "after failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(kLinkIndex));

  if (machine.cpu_pc() != preserved_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error("jalr rd = 31 misaligned demo did not preserve pc/next_pc rollback");
  }

  if (machine.inspect_cpu_gpr(kTargetRegisterIndex) !=
      cpu_rdram_alias(kMisalignedTargetAddress)) {
    throw std::runtime_error("jalr rd = 31 misaligned demo changed the target register");
  }

  if (machine.inspect_cpu_gpr(kDelaySlotMarkerIndex) != 0) {
    throw std::runtime_error("jalr rd = 31 misaligned demo executed or modified the delay slot path");
  }

  if (machine.inspect_cpu_gpr(kLinkIndex) != kInitialLinkValue) {
    throw std::runtime_error("jalr rd = 31 misaligned demo changed the link register");
  }

  if (machine.inspect_cpu_gpr(kLinkIndex) == cpu_rdram_alias(kExpectedLinkValue)) {
    throw std::runtime_error("jalr rd = 31 misaligned demo leaked the speculative link value");
  }
}

void run_jalr_rd_equals_rs_demo(Machine& machine) {
  constexpr std::size_t kAliasedRegisterIndex = 13;
  constexpr std::size_t kDelaySlotMarkerIndex = 14;
  constexpr std::size_t kTargetMarkerIndex = 15;

  constexpr std::uint32_t kLoadTargetAddress = 0x000000c0u;
  constexpr std::uint32_t kJalrAddress = 0x000000c4u;
  constexpr std::uint32_t kDelaySlotAddress = 0x000000c8u;
  constexpr std::uint32_t kLinkReturnAddress = 0x000000ccu;
  constexpr std::uint32_t kTargetAddress = 0x000000d0u;
  constexpr std::uint32_t kSentinelAddress = 0x000000d4u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(0, 0, 0);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(
      static_cast<std::uint8_t>(kAliasedRegisterIndex),
      static_cast<std::uint8_t>(kAliasedRegisterIndex));
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, 0xd1a1u);
  constexpr std::uint32_t kTargetInstruction = encode_ori(
      static_cast<std::uint8_t>(kTargetMarkerIndex), 0, 0xd1a2u);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLoadTargetAddress));
  machine.stage_cpu_gpr(kAliasedRegisterIndex, cpu_rdram_alias(kTargetAddress));
  machine.stage_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.stage_cpu_gpr(kTargetMarkerIndex, 0);

  machine.stage_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.stage_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.stage_rdram_u32_be(kLinkReturnAddress, kBreakInstruction);
  machine.stage_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 4: rd == rs reads target before link overwrite\n";
  std::cout << "  prelude_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kLoadTargetInstruction
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';

  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));
  print_hex64("  gpr[14]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kTargetMarkerIndex));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo4_prelude");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));

  if (machine.inspect_cpu_gpr(kAliasedRegisterIndex) !=
      cpu_rdram_alias(kTargetAddress)) {
    throw std::runtime_error("jalr demo 4 failed to seed the aliased target register");
  }

  std::cout << "before step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo4_issue_jalr");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));
  print_hex64("  gpr[14]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kDelaySlotAddress)) {
    throw std::runtime_error("jalr demo 4 did not move into the delay slot");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kTargetAddress)) {
    throw std::runtime_error("jalr demo 4 did not schedule the original register target");
  }

  if (machine.inspect_cpu_gpr(kAliasedRegisterIndex) !=
      cpu_rdram_alias(kLinkReturnAddress)) {
    throw std::runtime_error("jalr demo 4 did not overwrite the aliased register with the link value");
  }

  if (machine.cpu_next_pc() == machine.inspect_cpu_gpr(kAliasedRegisterIndex)) {
    throw std::runtime_error(
        "jalr demo 4 scheduled the newly written link value instead of the original target");
  }

  if (machine.inspect_cpu_gpr(kDelaySlotMarkerIndex) != 0 ||
      machine.inspect_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error("jalr demo 4 changed marker registers too early");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo4_delay_slot");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));
  print_hex64("  gpr[14]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kTargetAddress)) {
    throw std::runtime_error(
        "jalr demo 4 delay slot did not hand off to the original register target");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSentinelAddress)) {
    throw std::runtime_error("jalr demo 4 did not preserve sequential next_pc at the target");
  }

  if (machine.cpu_pc() == machine.inspect_cpu_gpr(kAliasedRegisterIndex)) {
    throw std::runtime_error(
        "jalr demo 4 handed off to the post-link register value instead of the original target");
  }

  if (machine.inspect_cpu_gpr(kAliasedRegisterIndex) !=
      cpu_rdram_alias(kLinkReturnAddress)) {
    throw std::runtime_error("jalr demo 4 changed the aliased register after the delay slot");
  }

  if (machine.inspect_cpu_gpr(kDelaySlotMarkerIndex) != 0x0000d1a1u) {
    throw std::runtime_error("jalr demo 4 delay slot did not execute");
  }

  if (machine.inspect_cpu_gpr(kTargetMarkerIndex) != 0) {
    throw std::runtime_error("jalr demo 4 executed the target too early");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo4_target");

  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kAliasedRegisterIndex));
  print_hex64("  gpr[14]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kTargetMarkerIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kSentinelAddress)) {
    throw std::runtime_error("jalr demo 4 ended at the wrong sentinel");
  }

  if (machine.inspect_cpu_gpr(kAliasedRegisterIndex) !=
      cpu_rdram_alias(kLinkReturnAddress)) {
    throw std::runtime_error("jalr demo 4 changed the aliased register after the target");
  }

  if (machine.inspect_cpu_gpr(kTargetMarkerIndex) != 0x0000d1a2u) {
    throw std::runtime_error("jalr demo 4 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo4_break_stop");
}

void run_jalr_encoded_rd_demo(Machine& machine) {
  constexpr std::uint32_t kLoadTargetAddress = 0x00000000u;
  constexpr std::uint32_t kJalrAddress = 0x00000004u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000008u;
  constexpr std::uint32_t kLinkReturnAddress = 0x0000000cu;
  constexpr std::uint32_t kTargetAddress = 0x00000010u;
  constexpr std::uint32_t kSentinelAddress = 0x00000014u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(0, 0, 0);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(7, 4);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(5, 0, 0x5555);
  constexpr std::uint32_t kTargetInstruction = encode_ori(6, 0, 0x6666);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLoadTargetAddress));
  machine.stage_cpu_gpr(4, cpu_rdram_alias(kTargetAddress));
  machine.stage_cpu_gpr(5, 0);
  machine.stage_cpu_gpr(6, 0);
  machine.stage_cpu_gpr(7, 0);
  machine.stage_cpu_gpr(31, 0);

  machine.stage_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.stage_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.stage_rdram_u32_be(kLinkReturnAddress, kBreakInstruction);
  machine.stage_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 1: encoded rd link register\n";
  std::cout << "  prelude_raw = 0x"
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
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(4));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(7));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(31));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_prelude");
  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(4));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_issue_jalr");
  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(7));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(31));

  if (machine.cpu_pc() != cpu_rdram_alias(kDelaySlotAddress)) {
    throw std::runtime_error("jalr demo 1 did not move into the delay slot");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kTargetAddress)) {
    throw std::runtime_error("jalr demo 1 did not schedule the register target");
  }

  if (machine.inspect_cpu_gpr(7) != cpu_rdram_alias(kLinkReturnAddress)) {
    throw std::runtime_error("jalr demo 1 wrote the wrong link address");
  }

  if (machine.inspect_cpu_gpr(31) != 0) {
    throw std::runtime_error("jalr demo 1 unexpectedly touched gpr[31]");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_delay_slot");
  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(5));

  if (machine.cpu_pc() != cpu_rdram_alias(kTargetAddress)) {
    throw std::runtime_error("jalr demo 1 delay slot did not hand off to the target");
  }

  if (machine.inspect_cpu_gpr(5) != 0x00005555u) {
    throw std::runtime_error("jalr demo 1 delay slot did not execute");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_target");
  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(6));

  if (machine.cpu_pc() != cpu_rdram_alias(kSentinelAddress)) {
    throw std::runtime_error("jalr demo 1 ended at the wrong sentinel");
  }

  if (machine.inspect_cpu_gpr(6) != 0x00006666u) {
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

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(0, 0, 0);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(31, 4);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(8, 0, 0x8888);
  constexpr std::uint32_t kTargetInstruction = encode_ori(9, 0, 0x9999);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLoadTargetAddress));
  machine.stage_cpu_gpr(4, cpu_rdram_alias(kTargetAddress));
  machine.stage_cpu_gpr(8, 0);
  machine.stage_cpu_gpr(9, 0);
  machine.stage_cpu_gpr(31, 0);

  machine.stage_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.stage_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.stage_rdram_u32_be(kLinkReturnAddress, kBreakInstruction);
  machine.stage_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 2: rd = 31 normal link case\n";
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';

  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_prelude");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_issue_jalr");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_delay_slot");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_target");

  print_control_flow_state(machine);
  print_hex64("  gpr[8]", machine.inspect_cpu_gpr(8));
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(9));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(31));

  if (machine.cpu_pc() != cpu_rdram_alias(kSentinelAddress)) {
    throw std::runtime_error("jalr demo 2 ended at the wrong sentinel");
  }

  if (machine.inspect_cpu_gpr(31) != cpu_rdram_alias(kLinkReturnAddress)) {
    throw std::runtime_error("jalr demo 2 wrote the wrong return address into gpr[31]");
  }

  if (machine.inspect_cpu_gpr(8) != 0x00008888u) {
    throw std::runtime_error("jalr demo 2 delay slot did not execute");
  }

  if (machine.inspect_cpu_gpr(9) != 0x00009999u) {
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

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(0, 0, 0);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(0, 10);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(11, 0, 0xabcd);
  constexpr std::uint32_t kTargetInstruction = encode_ori(12, 0, 0xdcba);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLoadTargetAddress));
  machine.stage_cpu_gpr(10, cpu_rdram_alias(kTargetAddress));
  machine.stage_cpu_gpr(11, 0);
  machine.stage_cpu_gpr(12, 0);

  machine.stage_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.stage_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.stage_rdram_u32_be(kJalrAddress + 8u, kBreakInstruction);
  machine.stage_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 3: rd = 0 discards link through normal gpr[0] behavior\n";
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';

  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_prelude");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_issue_jalr");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_delay_slot");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_target");

  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.inspect_cpu_gpr(0));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(11));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(12));

  if (machine.cpu_pc() != cpu_rdram_alias(kSentinelAddress)) {
    throw std::runtime_error("jalr demo 3 ended at the wrong sentinel");
  }

  if (machine.inspect_cpu_gpr(0) != 0) {
    throw std::runtime_error("jalr demo 3 unexpectedly changed gpr[0]");
  }

  if (machine.inspect_cpu_gpr(11) != 0x0000abcdu) {
    throw std::runtime_error("jalr demo 3 delay slot did not execute");
  }

  if (machine.inspect_cpu_gpr(12) != 0x0000dcbau) {
    throw std::runtime_error("jalr demo 3 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo3_break_stop");
}

void run_jalr_misaligned_target_demo(Machine& machine) {
  constexpr std::size_t kTargetRegisterIndex = 18;
  constexpr std::size_t kDelaySlotMarkerIndex = 19;
  constexpr std::size_t kLinkIndex = 20;
  constexpr std::uint32_t kJalrAddress = 0x000006a0u;
  constexpr std::uint32_t kDelaySlotAddress = 0x000006a4u;
  constexpr std::uint32_t kMisalignedTargetAddress = 0x000006b2u;
  constexpr std::uint32_t kExpectedLinkValue = 0x000006a8u;

  constexpr std::uint32_t kJalrInstruction = encode_jalr(
      static_cast<std::uint8_t>(kLinkIndex),
      static_cast<std::uint8_t>(kTargetRegisterIndex));
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(
      static_cast<std::uint8_t>(kDelaySlotMarkerIndex), 0, 0x74b1u);

  machine.stage_cpu_pc(cpu_rdram_alias(kJalrAddress));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kJalrAddress + 8u));
  machine.stage_cpu_gpr(kTargetRegisterIndex, cpu_rdram_alias(kMisalignedTargetAddress));
  machine.stage_cpu_gpr(kDelaySlotMarkerIndex, 0);
  machine.stage_cpu_gpr(kLinkIndex, 0);
  machine.stage_cpu_gpr(31, 0);

  machine.stage_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.stage_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);

  std::cout << "fn64 bootstrap jump failure demo: special_jalr misaligned register target\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[20]", machine.inspect_cpu_gpr(kLinkIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(31));

  const std::uint32_t raw = kJalrInstruction;

  print_hex32("  jalr_raw", raw);

  const std::uint32_t preserved_pc = machine.cpu_pc();
  const std::uint32_t preserved_next_pc = machine.cpu_next_pc();

  try {
    static_cast<void>(machine.step_cpu_instruction());
    throw std::runtime_error("jalr misaligned demo expected step_cpu_instruction to throw");
  } catch (const MachineFault& error) {
    std::cout << "  jalr_misaligned_step threw: " << error.what() << '\n';
    require_control_transfer_fault(error, "jalr_misaligned_step");
  }

  std::cout << "after failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kTargetRegisterIndex));
  print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kDelaySlotMarkerIndex));
  print_hex64("  gpr[20]", machine.inspect_cpu_gpr(kLinkIndex));
  print_hex64("  gpr[31]", machine.inspect_cpu_gpr(31));

  if (machine.cpu_pc() != preserved_pc || machine.cpu_next_pc() != preserved_next_pc) {
    throw std::runtime_error("jalr misaligned demo did not preserve pc/next_pc rollback");
  }

  if (machine.inspect_cpu_gpr(kTargetRegisterIndex) !=
      cpu_rdram_alias(kMisalignedTargetAddress)) {
    throw std::runtime_error("jalr misaligned demo changed the target register");
  }

  if (machine.inspect_cpu_gpr(kDelaySlotMarkerIndex) != 0) {
    throw std::runtime_error("jalr misaligned demo executed or modified the delay slot path");
  }

  if (machine.inspect_cpu_gpr(kLinkIndex) != 0) {
    throw std::runtime_error("jalr misaligned demo leaked the link write through rollback");
  }

  if (machine.inspect_cpu_gpr(kLinkIndex) == cpu_rdram_alias(kExpectedLinkValue)) {
    throw std::runtime_error("jalr misaligned demo preserved the speculative link value");
  }

  if (machine.inspect_cpu_gpr(31) != 0) {
    throw std::runtime_error("jalr misaligned demo unexpectedly touched gpr[31]");
  }
}

}  // namespace

void run_control_register_jump_demos(Machine& machine) {
  run_jr_misaligned_target_demo(machine);
  run_jalr_rd_equals_rs_misaligned_target_demo(machine);
  run_jalr_rd31_misaligned_target_demo(machine);
  run_jalr_rd_equals_rs_demo(machine);
  run_jalr_encoded_rd_demo(machine);
  run_jalr_rd31_demo(machine);
  run_jalr_rd0_demo(machine);
  run_jalr_misaligned_target_demo(machine);
}

}  // namespace fn64::bootstrap_detail
