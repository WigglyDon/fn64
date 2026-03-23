#include "bootstrap_common.hpp"

#include <iostream>
#include <stdexcept>

namespace fn64::bootstrap_detail {
namespace {

void run_unaligned_load_word_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 9;

  constexpr std::uint32_t kLwlAddress = 0x000000b0u;
  constexpr std::uint32_t kLwrAddress = 0x000000b4u;
  constexpr std::uint32_t kBreakAddress = 0x000000b8u;

  constexpr std::uint32_t kDataWord0Address = 0x00000410u;
  constexpr std::uint32_t kDataWord1Address = 0x00000414u;
  constexpr std::uint32_t kMergedWordAddress = 0x00000412u;

  constexpr std::uint32_t kLwlInstruction = encode_lwl(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0002u);
  constexpr std::uint32_t kLwrInstruction = encode_lwr(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0005u);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kLwlAddress);
  machine.write_cpu_gpr(kBaseIndex, kDataWord0Address);
  machine.write_cpu_gpr(kTargetIndex, 0xaabbccddu);

  machine.write_rdram_u32_be(kLwlAddress, kLwlInstruction);
  machine.write_rdram_u32_be(kLwrAddress, kLwrInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kDataWord0Address, 0x10203040u);
  machine.write_rdram_u32_be(kDataWord1Address, 0x50607080u);

  std::cout << "fn64 bootstrap unaligned load demo: explicit local LWL/LWR merge\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[9]", machine.read_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000410]", kDataWord0Address);
  print_rdram_word(machine, "  rdram[0x00000414]", kDataWord1Address);

  const std::uint32_t lwl_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord lwl_decoded =
      Machine::decode_cpu_instruction_word(lwl_raw);
  const Machine::CpuInstructionIdentity lwl_identity =
      Machine::identify_cpu_instruction(lwl_decoded);

  print_hex32("  lwl_raw", lwl_raw);
  std::cout << "  lwl_identity = "
            << Machine::cpu_instruction_identity_name(lwl_identity) << '\n';

  if (lwl_identity != Machine::CpuInstructionIdentity::kLwl) {
    throw std::runtime_error("unaligned load demo did not identify LWL explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "unaligned_load_demo_lwl");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.read_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != kLwrAddress) {
    throw std::runtime_error("unaligned load demo did not advance to LWR");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != 0x3040ccddu) {
    throw std::runtime_error("unaligned load demo LWL merge result was wrong");
  }

  const std::uint32_t lwr_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord lwr_decoded =
      Machine::decode_cpu_instruction_word(lwr_raw);
  const Machine::CpuInstructionIdentity lwr_identity =
      Machine::identify_cpu_instruction(lwr_decoded);

  print_hex32("  lwr_raw", lwr_raw);
  std::cout << "  lwr_identity = "
            << Machine::cpu_instruction_identity_name(lwr_identity) << '\n';

  if (lwr_identity != Machine::CpuInstructionIdentity::kLwr) {
    throw std::runtime_error("unaligned load demo did not identify LWR explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "unaligned_load_demo_lwr");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.read_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000412]", kMergedWordAddress);

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("unaligned load demo did not advance to BREAK sentinel");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != 0x30405060u) {
    throw std::runtime_error("unaligned load demo LWL/LWR pair did not produce merged word");
  }

  require_stopped(machine.step_cpu_instruction(), "unaligned_load_demo_break");
}

void run_unaligned_store_word_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 10;

  constexpr std::uint32_t kSwlAddress = 0x000000d0u;
  constexpr std::uint32_t kSwrAddress = 0x000000d4u;
  constexpr std::uint32_t kBreakAddress = 0x000000d8u;

  constexpr std::uint32_t kDataWord0Address = 0x00000430u;
  constexpr std::uint32_t kDataWord1Address = 0x00000434u;
  constexpr std::uint32_t kMergedWordAddress = 0x00000432u;

  constexpr std::uint32_t kSwlInstruction = encode_swl(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0002u);
  constexpr std::uint32_t kSwrInstruction = encode_swr(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0005u);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kSwlAddress);
  machine.write_cpu_gpr(kBaseIndex, kDataWord0Address);
  machine.write_cpu_gpr(kSourceIndex, 0xa1b2c3d4u);

  machine.write_rdram_u32_be(kSwlAddress, kSwlInstruction);
  machine.write_rdram_u32_be(kSwrAddress, kSwrInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kDataWord0Address, 0x11223344u);
  machine.write_rdram_u32_be(kDataWord1Address, 0x55667788u);

  std::cout << "fn64 bootstrap unaligned store demo: explicit local SWL/SWR shaping\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[10]", machine.read_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000430]", kDataWord0Address);
  print_rdram_word(machine, "  rdram[0x00000434]", kDataWord1Address);

  const std::uint32_t swl_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord swl_decoded =
      Machine::decode_cpu_instruction_word(swl_raw);
  const Machine::CpuInstructionIdentity swl_identity =
      Machine::identify_cpu_instruction(swl_decoded);

  print_hex32("  swl_raw", swl_raw);
  std::cout << "  swl_identity = "
            << Machine::cpu_instruction_identity_name(swl_identity) << '\n';

  if (swl_identity != Machine::CpuInstructionIdentity::kSwl) {
    throw std::runtime_error("unaligned store demo did not identify SWL explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "unaligned_store_demo_swl");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000430]", kDataWord0Address);
  print_rdram_word(machine, "  rdram[0x00000434]", kDataWord1Address);

  if (machine.cpu_pc() != kSwrAddress) {
    throw std::runtime_error("unaligned store demo did not advance to SWR");
  }

  if (machine.read_rdram_u32_be(kDataWord0Address) != 0x1122a1b2u) {
    throw std::runtime_error("unaligned store demo SWL shaping was wrong");
  }

  if (machine.read_rdram_u32_be(kDataWord1Address) != 0x55667788u) {
    throw std::runtime_error("unaligned store demo SWL touched the wrong aligned word");
  }

  const std::uint32_t swr_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord swr_decoded =
      Machine::decode_cpu_instruction_word(swr_raw);
  const Machine::CpuInstructionIdentity swr_identity =
      Machine::identify_cpu_instruction(swr_decoded);

  print_hex32("  swr_raw", swr_raw);
  std::cout << "  swr_identity = "
            << Machine::cpu_instruction_identity_name(swr_identity) << '\n';

  if (swr_identity != Machine::CpuInstructionIdentity::kSwr) {
    throw std::runtime_error("unaligned store demo did not identify SWR explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "unaligned_store_demo_swr");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000430]", kDataWord0Address);
  print_rdram_word(machine, "  rdram[0x00000434]", kDataWord1Address);
  print_rdram_word(machine, "  rdram[0x00000432]", kMergedWordAddress);

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("unaligned store demo did not advance to BREAK sentinel");
  }

  if (machine.read_rdram_u32_be(kDataWord0Address) != 0x1122a1b2u) {
    throw std::runtime_error("unaligned store demo SWR unexpectedly changed the left aligned word");
  }

  if (machine.read_rdram_u32_be(kDataWord1Address) != 0xc3d47788u) {
    throw std::runtime_error("unaligned store demo SWR shaping was wrong");
  }

  if (machine.read_rdram_u32_be(kMergedWordAddress) != 0xa1b2c3d4u) {
    throw std::runtime_error("unaligned store demo SWL/SWR pair did not reconstruct the unaligned word");
  }

  require_stopped(machine.step_cpu_instruction(), "unaligned_store_demo_break");
}

}  // namespace

void run_data_demos(Machine& machine) {
  run_unaligned_load_word_demo(machine);
  run_unaligned_store_word_demo(machine);
}

}  // namespace fn64::bootstrap_detail