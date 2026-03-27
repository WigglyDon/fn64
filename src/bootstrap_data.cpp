#include "bootstrap_common.hpp"

#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

constexpr std::uint32_t encode_i_type(
    std::uint8_t opcode,
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return (static_cast<std::uint32_t>(opcode) << 26) |
         (static_cast<std::uint32_t>(rs) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(immediate);
}

constexpr std::uint32_t encode_lb(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x20, rt, rs, immediate);
}

constexpr std::uint32_t encode_lh(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x21, rt, rs, immediate);
}

constexpr std::uint32_t encode_lw(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x23, rt, rs, immediate);
}

constexpr std::uint32_t encode_lbu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x24, rt, rs, immediate);
}

constexpr std::uint32_t encode_lhu(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x25, rt, rs, immediate);
}

constexpr std::uint32_t encode_sb(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x28, rt, rs, immediate);
}

constexpr std::uint32_t encode_sh(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x29, rt, rs, immediate);
}

constexpr std::uint32_t encode_sw(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x2b, rt, rs, immediate);
}

void print_and_require_current_instruction_identity(
    Machine& machine,
    Machine::CpuInstructionIdentity expected_identity,
    const char* raw_label,
    const char* identity_label,
    const char* failure_label) {
  const std::uint32_t raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord decoded =
      Machine::decode_cpu_instruction_word(raw);
  const Machine::CpuInstructionIdentity identity =
      Machine::identify_cpu_instruction(decoded);

  print_hex32(raw_label, raw);
  std::cout << "  " << identity_label << " = "
            << Machine::cpu_instruction_identity_name(identity) << '\n';

  if (identity != expected_identity) {
    throw std::runtime_error(failure_label);
  }
}

void require_step_runtime_error_contains(
    Machine& machine,
    const char* label,
    const char* expected_substring) {
  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::runtime_error& e) {
    std::cout << "  " << label << " threw: " << e.what() << '\n';

    if (std::string(e.what()).find(expected_substring) == std::string::npos) {
      throw std::runtime_error(
          std::string(label) + " threw unexpected runtime_error text");
    }

    return;
  }

  throw std::runtime_error(std::string(label) + " did not throw runtime_error");
}

void require_step_exception_contains(
    Machine& machine,
    const char* label,
    const char* expected_substring) {
  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::exception& e) {
    std::cout << "  " << label << " threw: " << e.what() << '\n';

    if (std::string(e.what()).find(expected_substring) == std::string::npos) {
      throw std::runtime_error(
          std::string(label) + " threw unexpected exception text");
    }

    return;
  }

  throw std::runtime_error(std::string(label) + " did not throw exception");
}

void run_unaligned_load_word_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 9;

  constexpr std::uint32_t kLwlAddress = 0x000000b0u;
  constexpr std::uint32_t kLwrAddress = 0x000000b4u;
  constexpr std::uint32_t kBreakAddress = 0x000000b8u;

  constexpr std::uint32_t kDataWord0Address = 0x00000410u;
  constexpr std::uint32_t kDataWord1Address = 0x00000414u;
  constexpr std::uint32_t kMergedWordAddress = 0x00000412u;

  const std::uint32_t kLwlInstruction = encode_lwl(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0002u);
  const std::uint32_t kLwrInstruction = encode_lwr(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0005u);
  const std::uint32_t kBreakInstruction = encode_break();

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

  const std::uint32_t kSwlInstruction = encode_swl(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0002u);
  const std::uint32_t kSwrInstruction = encode_swr(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0005u);
  const std::uint32_t kBreakInstruction = encode_break();

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

void run_aligned_word_load_store_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 11;
  constexpr std::size_t kTargetIndex = 12;

  constexpr std::uint32_t kSwAddress = 0x000000e0u;
  constexpr std::uint32_t kLwAddress = 0x000000e4u;
  constexpr std::uint32_t kBreakAddress = 0x000000e8u;

  constexpr std::uint32_t kDataBaseAddress = 0x00000450u;
  constexpr std::uint16_t kOffset = 0x0004u;
  constexpr std::uint32_t kEffectiveAddress = kDataBaseAddress + kOffset;

  const std::uint32_t kSwInstruction = encode_sw(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kLwInstruction = encode_lw(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kSwAddress);
  machine.write_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0x89abcdefu);
  machine.write_cpu_gpr(kTargetIndex, 0x00000000u);

  machine.write_rdram_u32_be(kSwAddress, kSwInstruction);
  machine.write_rdram_u32_be(kLwAddress, kLwInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kDataBaseAddress, 0x01020304u);
  machine.write_rdram_u32_be(kEffectiveAddress, 0x55667788u);

  std::cout << "fn64 bootstrap aligned word demo: explicit local SW/LW base+immediate\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000450]", kDataBaseAddress);
  print_rdram_word(machine, "  rdram[0x00000454]", kEffectiveAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSw,
      "  sw_raw",
      "sw_identity",
      "aligned word demo did not identify SW explicitly");

  require_stepped(machine.step_cpu_instruction(), "aligned_word_demo_sw");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000450]", kDataBaseAddress);
  print_rdram_word(machine, "  rdram[0x00000454]", kEffectiveAddress);

  if (machine.cpu_pc() != kLwAddress) {
    throw std::runtime_error("aligned word demo did not advance to LW");
  }

  if (machine.read_rdram_u32_be(kEffectiveAddress) != 0x89abcdefu) {
    throw std::runtime_error("aligned word demo SW store result was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLw,
      "  lw_raw",
      "lw_identity",
      "aligned word demo did not identify LW explicitly");

  require_stepped(machine.step_cpu_instruction(), "aligned_word_demo_lw");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000454]", kEffectiveAddress);

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("aligned word demo did not advance to BREAK sentinel");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != 0x89abcdefu) {
    throw std::runtime_error("aligned word demo LW load result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "aligned_word_demo_break");
}

void run_word_alignment_guard_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 11;
  constexpr std::size_t kTargetIndex = 12;

  constexpr std::uint32_t kSwAddress = 0x00000100u;
  constexpr std::uint32_t kLwAddress = 0x00000104u;

  constexpr std::uint32_t kDataBaseAddress = 0x00000470u;
  constexpr std::uint16_t kMisalignedOffset = 0x0003u;
  constexpr std::uint32_t kMisalignedAddress = kDataBaseAddress + kMisalignedOffset;

  const std::uint32_t kSwInstruction = encode_sw(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);
  const std::uint32_t kLwInstruction = encode_lw(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);

  machine.write_rdram_u32_be(kSwAddress, kSwInstruction);
  machine.write_rdram_u32_be(kLwAddress, kLwInstruction);

  machine.write_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0xdeadbeefu);
  machine.write_cpu_gpr(kTargetIndex, 0x01234567u);
  machine.write_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap aligned word guard demo: explicit local SW/LW natural-alignment failure\n";

  machine.write_cpu_pc(kSwAddress);

  std::cout << "before SW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kSourceIndex));
  print_hex32("  sw_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSw,
      "  sw_raw",
      "sw_identity",
      "word alignment guard demo did not identify SW explicitly");

  require_step_runtime_error_contains(
      machine,
      "word_alignment_demo_sw",
      "requires naturally aligned word address");

  std::cout << "after SW misaligned step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

  if (machine.cpu_pc() != kSwAddress) {
    throw std::runtime_error("word alignment guard demo SW changed PC on fault");
  }

  if (machine.cpu_next_pc() != kSwAddress + 4u) {
    throw std::runtime_error("word alignment guard demo SW changed next_pc on fault");
  }

  if (machine.read_rdram_u32_be(kDataBaseAddress) != 0x11223344u) {
    throw std::runtime_error("word alignment guard demo SW changed memory on fault");
  }

  machine.write_cpu_pc(kLwAddress);
  machine.write_cpu_gpr(kTargetIndex, 0x01234567u);

  std::cout << "before LW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kTargetIndex));
  print_hex32("  lw_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLw,
      "  lw_raw",
      "lw_identity",
      "word alignment guard demo did not identify LW explicitly");

  require_step_runtime_error_contains(
      machine,
      "word_alignment_demo_lw",
      "requires naturally aligned word address");

  std::cout << "after LW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != kLwAddress) {
    throw std::runtime_error("word alignment guard demo LW changed PC on fault");
  }

  if (machine.cpu_next_pc() != kLwAddress + 4u) {
    throw std::runtime_error("word alignment guard demo LW changed next_pc on fault");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != 0x01234567u) {
    throw std::runtime_error("word alignment guard demo LW changed target register on fault");
  }
}

void run_byte_load_store_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 13;
  constexpr std::size_t kSignedTargetIndex = 14;
  constexpr std::size_t kUnsignedTargetIndex = 15;

  constexpr std::uint32_t kSbAddress = 0x00000120u;
  constexpr std::uint32_t kLbAddress = 0x00000124u;
  constexpr std::uint32_t kLbuAddress = 0x00000128u;
  constexpr std::uint32_t kBreakAddress = 0x0000012cu;

  constexpr std::uint32_t kDataBaseAddress = 0x00000490u;
  constexpr std::uint16_t kOffset = 0x0001u;

  const std::uint32_t kSbInstruction = encode_sb(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kLbInstruction = encode_lb(
      static_cast<std::uint8_t>(kSignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kLbuInstruction = encode_lbu(
      static_cast<std::uint8_t>(kUnsignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kSbAddress);
  machine.write_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0x12345680u);
  machine.write_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.write_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.write_rdram_u32_be(kSbAddress, kSbInstruction);
  machine.write_rdram_u32_be(kLbAddress, kLbInstruction);
  machine.write_rdram_u32_be(kLbuAddress, kLbuInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap byte demo: explicit local SB/LB/LBU shaping and extension\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[13]", machine.read_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000490]", kDataBaseAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSb,
      "  sb_raw",
      "sb_identity",
      "byte demo did not identify SB explicitly");

  require_stepped(machine.step_cpu_instruction(), "byte_demo_sb");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000490]", kDataBaseAddress);

  if (machine.cpu_pc() != kLbAddress) {
    throw std::runtime_error("byte demo did not advance to LB");
  }

  if (machine.read_rdram_u32_be(kDataBaseAddress) != 0x11803344u) {
    throw std::runtime_error("byte demo SB shaping was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLb,
      "  lb_raw",
      "lb_identity",
      "byte demo did not identify LB explicitly");

  require_stepped(machine.step_cpu_instruction(), "byte_demo_lb");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[14]", machine.read_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != kLbuAddress) {
    throw std::runtime_error("byte demo did not advance to LBU");
  }

  if (machine.read_cpu_gpr(kSignedTargetIndex) != 0xffffff80u) {
    throw std::runtime_error("byte demo LB sign-extension result was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLbu,
      "  lbu_raw",
      "lbu_identity",
      "byte demo did not identify LBU explicitly");

  require_stepped(machine.step_cpu_instruction(), "byte_demo_lbu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[15]", machine.read_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("byte demo did not advance to BREAK sentinel");
  }

  if (machine.read_cpu_gpr(kUnsignedTargetIndex) != 0x00000080u) {
    throw std::runtime_error("byte demo LBU zero-extension result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "byte_demo_break");
}

void run_halfword_load_store_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 16;
  constexpr std::size_t kSignedTargetIndex = 17;
  constexpr std::size_t kUnsignedTargetIndex = 18;

  constexpr std::uint32_t kShAddress = 0x00000140u;
  constexpr std::uint32_t kLhAddress = 0x00000144u;
  constexpr std::uint32_t kLhuAddress = 0x00000148u;
  constexpr std::uint32_t kBreakAddress = 0x0000014cu;

  constexpr std::uint32_t kDataBaseAddress = 0x000004b0u;
  constexpr std::uint16_t kOffset = 0x0002u;

  const std::uint32_t kShInstruction = encode_sh(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kLhInstruction = encode_lh(
      static_cast<std::uint8_t>(kSignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kLhuInstruction = encode_lhu(
      static_cast<std::uint8_t>(kUnsignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kShAddress);
  machine.write_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.write_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.write_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.write_rdram_u32_be(kShAddress, kShInstruction);
  machine.write_rdram_u32_be(kLhAddress, kLhInstruction);
  machine.write_rdram_u32_be(kLhuAddress, kLhuInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap halfword demo: explicit local SH/LH/LHU shaping and extension\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[16]", machine.read_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x000004b0]", kDataBaseAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSh,
      "  sh_raw",
      "sh_identity",
      "halfword demo did not identify SH explicitly");

  require_stepped(machine.step_cpu_instruction(), "halfword_demo_sh");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x000004b0]", kDataBaseAddress);

  if (machine.cpu_pc() != kLhAddress) {
    throw std::runtime_error("halfword demo did not advance to LH");
  }

  if (machine.read_rdram_u32_be(kDataBaseAddress) != 0x11228001u) {
    throw std::runtime_error("halfword demo SH shaping was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLh,
      "  lh_raw",
      "lh_identity",
      "halfword demo did not identify LH explicitly");

  require_stepped(machine.step_cpu_instruction(), "halfword_demo_lh");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[17]", machine.read_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != kLhuAddress) {
    throw std::runtime_error("halfword demo did not advance to LHU");
  }

  if (machine.read_cpu_gpr(kSignedTargetIndex) != 0xffff8001u) {
    throw std::runtime_error("halfword demo LH sign-extension result was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLhu,
      "  lhu_raw",
      "lhu_identity",
      "halfword demo did not identify LHU explicitly");

  require_stepped(machine.step_cpu_instruction(), "halfword_demo_lhu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.read_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("halfword demo did not advance to BREAK sentinel");
  }

  if (machine.read_cpu_gpr(kUnsignedTargetIndex) != 0x00008001u) {
    throw std::runtime_error("halfword demo LHU zero-extension result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "halfword_demo_break");
}

void run_halfword_alignment_guard_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 16;
  constexpr std::size_t kSignedTargetIndex = 17;
  constexpr std::size_t kUnsignedTargetIndex = 18;

  constexpr std::uint32_t kShAddress = 0x00000160u;
  constexpr std::uint32_t kLhAddress = 0x00000164u;
  constexpr std::uint32_t kLhuAddress = 0x00000168u;

  constexpr std::uint32_t kDataBaseAddress = 0x000004d0u;
  constexpr std::uint16_t kMisalignedOffset = 0x0001u;
  constexpr std::uint32_t kMisalignedAddress = kDataBaseAddress + kMisalignedOffset;

  const std::uint32_t kShInstruction = encode_sh(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);
  const std::uint32_t kLhInstruction = encode_lh(
      static_cast<std::uint8_t>(kSignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);
  const std::uint32_t kLhuInstruction = encode_lhu(
      static_cast<std::uint8_t>(kUnsignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);

  machine.write_rdram_u32_be(kShAddress, kShInstruction);
  machine.write_rdram_u32_be(kLhAddress, kLhInstruction);
  machine.write_rdram_u32_be(kLhuAddress, kLhuInstruction);

  machine.write_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.write_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.write_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);
  machine.write_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap halfword guard demo: explicit local SH/LH/LHU natural-alignment failure\n";

  machine.write_cpu_pc(kShAddress);

  std::cout << "before SH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[16]", machine.read_cpu_gpr(kSourceIndex));
  print_hex32("  sh_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x000004d0]", kDataBaseAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSh,
      "  sh_raw",
      "sh_identity",
      "halfword alignment guard demo did not identify SH explicitly");

  require_step_runtime_error_contains(
      machine,
      "halfword_alignment_demo_sh",
      "requires naturally aligned halfword address");

  std::cout << "after SH misaligned step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x000004d0]", kDataBaseAddress);

  if (machine.cpu_pc() != kShAddress) {
    throw std::runtime_error("halfword alignment guard demo SH changed PC on fault");
  }

  if (machine.cpu_next_pc() != kShAddress + 4u) {
    throw std::runtime_error("halfword alignment guard demo SH changed next_pc on fault");
  }

  if (machine.read_rdram_u32_be(kDataBaseAddress) != 0x11223344u) {
    throw std::runtime_error("halfword alignment guard demo SH changed memory on fault");
  }

  machine.write_cpu_pc(kLhAddress);
  machine.write_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);

  std::cout << "before LH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[17]", machine.read_cpu_gpr(kSignedTargetIndex));
  print_hex32("  lh_effective_address", kMisalignedAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLh,
      "  lh_raw",
      "lh_identity",
      "halfword alignment guard demo did not identify LH explicitly");

  require_step_runtime_error_contains(
      machine,
      "halfword_alignment_demo_lh",
      "requires naturally aligned halfword address");

  std::cout << "after LH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[17]", machine.read_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != kLhAddress) {
    throw std::runtime_error("halfword alignment guard demo LH changed PC on fault");
  }

  if (machine.cpu_next_pc() != kLhAddress + 4u) {
    throw std::runtime_error("halfword alignment guard demo LH changed next_pc on fault");
  }

  if (machine.read_cpu_gpr(kSignedTargetIndex) != 0xaaaaaaaau) {
    throw std::runtime_error("halfword alignment guard demo LH changed target register on fault");
  }

  machine.write_cpu_pc(kLhuAddress);
  machine.write_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  std::cout << "before LHU misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[18]", machine.read_cpu_gpr(kUnsignedTargetIndex));
  print_hex32("  lhu_effective_address", kMisalignedAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLhu,
      "  lhu_raw",
      "lhu_identity",
      "halfword alignment guard demo did not identify LHU explicitly");

  require_step_runtime_error_contains(
      machine,
      "halfword_alignment_demo_lhu",
      "requires naturally aligned halfword address");

  std::cout << "after LHU misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.read_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != kLhuAddress) {
    throw std::runtime_error("halfword alignment guard demo LHU changed PC on fault");
  }

  if (machine.cpu_next_pc() != kLhuAddress + 4u) {
    throw std::runtime_error("halfword alignment guard demo LHU changed next_pc on fault");
  }

  if (machine.read_cpu_gpr(kUnsignedTargetIndex) != 0xbbbbbbbbu) {
    throw std::runtime_error("halfword alignment guard demo LHU changed target register on fault");
  }
}

void run_negative_word_load_store_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 19;
  constexpr std::size_t kTargetIndex = 20;

  constexpr std::uint32_t kSwAddress = 0x00000180u;
  constexpr std::uint32_t kLwAddress = 0x00000184u;
  constexpr std::uint32_t kBreakAddress = 0x00000188u;

  constexpr std::uint32_t kEffectiveAddress = 0x000004f0u;
  constexpr std::uint32_t kBaseAddress = 0x000004f4u;
  constexpr std::uint16_t kNegativeOffset = 0xfffcu;

  const std::uint32_t kSwInstruction = encode_sw(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kLwInstruction = encode_lw(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kSwAddress);
  machine.write_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0x76543210u);
  machine.write_cpu_gpr(kTargetIndex, 0x00000000u);

  machine.write_rdram_u32_be(kSwAddress, kSwInstruction);
  machine.write_rdram_u32_be(kLwAddress, kLwInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kEffectiveAddress, 0x01020304u);
  machine.write_rdram_u32_be(kBaseAddress, 0x55667788u);

  std::cout << "fn64 bootstrap negative-offset word demo: explicit local SW/LW signed immediate address formation\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex32("  sw_immediate_raw", kNegativeOffset);
  print_hex32("  sw_effective_address", kEffectiveAddress);
  print_hex64("  gpr[19]", machine.read_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[20]", machine.read_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x000004f0]", kEffectiveAddress);
  print_rdram_word(machine, "  rdram[0x000004f4]", kBaseAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSw,
      "  sw_raw",
      "sw_identity",
      "negative-offset word demo did not identify SW explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_word_demo_sw");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x000004f0]", kEffectiveAddress);
  print_rdram_word(machine, "  rdram[0x000004f4]", kBaseAddress);

  if (machine.cpu_pc() != kLwAddress) {
    throw std::runtime_error("negative-offset word demo did not advance to LW");
  }

  if (machine.read_rdram_u32_be(kEffectiveAddress) != 0x76543210u) {
    throw std::runtime_error("negative-offset word demo SW store result was wrong");
  }

  if (machine.read_rdram_u32_be(kBaseAddress) != 0x55667788u) {
    throw std::runtime_error("negative-offset word demo touched the base word unexpectedly");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLw,
      "  lw_raw",
      "lw_identity",
      "negative-offset word demo did not identify LW explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_word_demo_lw");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[20]", machine.read_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x000004f0]", kEffectiveAddress);

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("negative-offset word demo did not advance to BREAK sentinel");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != 0x76543210u) {
    throw std::runtime_error("negative-offset word demo LW load result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "negative_word_demo_break");
}

void run_negative_byte_load_store_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 21;
  constexpr std::size_t kSignedTargetIndex = 22;
  constexpr std::size_t kUnsignedTargetIndex = 23;

  constexpr std::uint32_t kSbAddress = 0x000001a0u;
  constexpr std::uint32_t kLbAddress = 0x000001a4u;
  constexpr std::uint32_t kLbuAddress = 0x000001a8u;
  constexpr std::uint32_t kBreakAddress = 0x000001acu;

  constexpr std::uint32_t kDataWordAddress = 0x00000510u;
  constexpr std::uint32_t kBaseAddress = 0x00000512u;
  constexpr std::uint32_t kEffectiveAddress = 0x00000511u;
  constexpr std::uint16_t kNegativeOffset = 0xffffu;

  const std::uint32_t kSbInstruction = encode_sb(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kLbInstruction = encode_lb(
      static_cast<std::uint8_t>(kSignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kLbuInstruction = encode_lbu(
      static_cast<std::uint8_t>(kUnsignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kSbAddress);
  machine.write_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0x12345680u);
  machine.write_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.write_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.write_rdram_u32_be(kSbAddress, kSbInstruction);
  machine.write_rdram_u32_be(kLbAddress, kLbInstruction);
  machine.write_rdram_u32_be(kLbuAddress, kLbuInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kDataWordAddress, 0x11223344u);

  std::cout << "fn64 bootstrap negative-offset byte demo: explicit local SB/LB/LBU signed immediate address formation\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex32("  sb_immediate_raw", kNegativeOffset);
  print_hex32("  sb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[21]", machine.read_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000510]", kDataWordAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSb,
      "  sb_raw",
      "sb_identity",
      "negative-offset byte demo did not identify SB explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_byte_demo_sb");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000510]", kDataWordAddress);

  if (machine.cpu_pc() != kLbAddress) {
    throw std::runtime_error("negative-offset byte demo did not advance to LB");
  }

  if (machine.read_rdram_u32_be(kDataWordAddress) != 0x11803344u) {
    throw std::runtime_error("negative-offset byte demo SB shaping was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLb,
      "  lb_raw",
      "lb_identity",
      "negative-offset byte demo did not identify LB explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_byte_demo_lb");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[22]", machine.read_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != kLbuAddress) {
    throw std::runtime_error("negative-offset byte demo did not advance to LBU");
  }

  if (machine.read_cpu_gpr(kSignedTargetIndex) != 0xffffff80u) {
    throw std::runtime_error("negative-offset byte demo LB sign-extension result was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLbu,
      "  lbu_raw",
      "lbu_identity",
      "negative-offset byte demo did not identify LBU explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_byte_demo_lbu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[23]", machine.read_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("negative-offset byte demo did not advance to BREAK sentinel");
  }

  if (machine.read_cpu_gpr(kUnsignedTargetIndex) != 0x00000080u) {
    throw std::runtime_error("negative-offset byte demo LBU zero-extension result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "negative_byte_demo_break");
}

void run_negative_halfword_load_store_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 24;
  constexpr std::size_t kSignedTargetIndex = 25;
  constexpr std::size_t kUnsignedTargetIndex = 26;

  constexpr std::uint32_t kShAddress = 0x000001c0u;
  constexpr std::uint32_t kLhAddress = 0x000001c4u;
  constexpr std::uint32_t kLhuAddress = 0x000001c8u;
  constexpr std::uint32_t kBreakAddress = 0x000001ccu;

  constexpr std::uint32_t kDataWordAddress = 0x00000530u;
  constexpr std::uint32_t kBaseAddress = 0x00000534u;
  constexpr std::uint32_t kEffectiveAddress = 0x00000532u;
  constexpr std::uint16_t kNegativeOffset = 0xfffeu;

  const std::uint32_t kShInstruction = encode_sh(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kLhInstruction = encode_lh(
      static_cast<std::uint8_t>(kSignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kLhuInstruction = encode_lhu(
      static_cast<std::uint8_t>(kUnsignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kShAddress);
  machine.write_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.write_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.write_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.write_rdram_u32_be(kShAddress, kShInstruction);
  machine.write_rdram_u32_be(kLhAddress, kLhInstruction);
  machine.write_rdram_u32_be(kLhuAddress, kLhuInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.write_rdram_u32_be(kDataWordAddress, 0x11223344u);

  std::cout << "fn64 bootstrap negative-offset halfword demo: explicit local SH/LH/LHU signed immediate address formation\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex32("  sh_immediate_raw", kNegativeOffset);
  print_hex32("  sh_effective_address", kEffectiveAddress);
  print_hex64("  gpr[24]", machine.read_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000530]", kDataWordAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSh,
      "  sh_raw",
      "sh_identity",
      "negative-offset halfword demo did not identify SH explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_halfword_demo_sh");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000530]", kDataWordAddress);

  if (machine.cpu_pc() != kLhAddress) {
    throw std::runtime_error("negative-offset halfword demo did not advance to LH");
  }

  if (machine.read_rdram_u32_be(kDataWordAddress) != 0x11228001u) {
    throw std::runtime_error("negative-offset halfword demo SH shaping was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLh,
      "  lh_raw",
      "lh_identity",
      "negative-offset halfword demo did not identify LH explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_halfword_demo_lh");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[25]", machine.read_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != kLhuAddress) {
    throw std::runtime_error("negative-offset halfword demo did not advance to LHU");
  }

  if (machine.read_cpu_gpr(kSignedTargetIndex) != 0xffff8001u) {
    throw std::runtime_error("negative-offset halfword demo LH sign-extension result was wrong");
  }

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLhu,
      "  lhu_raw",
      "lhu_identity",
      "negative-offset halfword demo did not identify LHU explicitly");

  require_stepped(machine.step_cpu_instruction(), "negative_halfword_demo_lhu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[26]", machine.read_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("negative-offset halfword demo did not advance to BREAK sentinel");
  }

  if (machine.read_cpu_gpr(kUnsignedTargetIndex) != 0x00008001u) {
    throw std::runtime_error("negative-offset halfword demo LHU zero-extension result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "negative_halfword_demo_break");
}

void run_negative_out_of_range_guard_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 27;
  constexpr std::size_t kTargetIndex = 28;

  constexpr std::uint32_t kSbAddress = 0x000001e0u;
  constexpr std::uint32_t kLbAddress = 0x000001e4u;

  constexpr std::uint32_t kBaseAddress = 0x00000000u;
  constexpr std::uint16_t kNegativeOffset = 0xffffu;
  constexpr std::uint32_t kEffectiveAddress = 0xffffffffu;
  constexpr std::uint32_t kSentinelAddress = 0x00000550u;

  const std::uint32_t kSbInstruction = encode_sb(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);
  const std::uint32_t kLbInstruction = encode_lb(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kNegativeOffset);

  machine.write_rdram_u32_be(kSbAddress, kSbInstruction);
  machine.write_rdram_u32_be(kLbAddress, kLbInstruction);

  machine.write_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.write_cpu_gpr(kSourceIndex, 0x00000080u);
  machine.write_cpu_gpr(kTargetIndex, 0x89abcdefu);
  machine.write_rdram_u32_be(kSentinelAddress, 0x11223344u);

  std::cout << "fn64 bootstrap negative-offset guard demo: explicit local out-of-range rollback on signed immediate address formation\n";

  machine.write_cpu_pc(kSbAddress);

  std::cout << "before SB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex32("  sb_immediate_raw", kNegativeOffset);
  print_hex32("  sb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[27]", machine.read_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000550]", kSentinelAddress);

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kSb,
      "  sb_raw",
      "sb_identity",
      "negative-offset guard demo did not identify SB explicitly");

  require_step_exception_contains(
      machine,
      "negative_out_of_range_demo_sb",
      "RDRAM access out of range");

  std::cout << "after SB out-of-range step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000550]", kSentinelAddress);

  if (machine.cpu_pc() != kSbAddress) {
    throw std::runtime_error("negative-offset guard demo SB changed PC on fault");
  }

  if (machine.cpu_next_pc() != kSbAddress + 4u) {
    throw std::runtime_error("negative-offset guard demo SB changed next_pc on fault");
  }

  if (machine.read_rdram_u32_be(kSentinelAddress) != 0x11223344u) {
    throw std::runtime_error("negative-offset guard demo SB changed memory on fault");
  }

  machine.write_cpu_pc(kLbAddress);
  machine.write_cpu_gpr(kTargetIndex, 0x89abcdefu);

  std::cout << "before LB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex32("  lb_immediate_raw", kNegativeOffset);
  print_hex32("  lb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[28]", machine.read_cpu_gpr(kTargetIndex));

  print_and_require_current_instruction_identity(
      machine,
      Machine::CpuInstructionIdentity::kLb,
      "  lb_raw",
      "lb_identity",
      "negative-offset guard demo did not identify LB explicitly");

  require_step_exception_contains(
      machine,
      "negative_out_of_range_demo_lb",
      "RDRAM access out of range");

  std::cout << "after LB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[28]", machine.read_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != kLbAddress) {
    throw std::runtime_error("negative-offset guard demo LB changed PC on fault");
  }

  if (machine.cpu_next_pc() != kLbAddress + 4u) {
    throw std::runtime_error("negative-offset guard demo LB changed next_pc on fault");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != 0x89abcdefu) {
    throw std::runtime_error("negative-offset guard demo LB changed target register on fault");
  }
}

}  // namespace

void run_data_demos(Machine& machine) {
  run_unaligned_load_word_demo(machine);
  run_unaligned_store_word_demo(machine);
  run_aligned_word_load_store_demo(machine);
  run_word_alignment_guard_demo(machine);
  run_byte_load_store_demo(machine);
  run_halfword_load_store_demo(machine);
  run_halfword_alignment_guard_demo(machine);
  run_negative_word_load_store_demo(machine);
  run_negative_byte_load_store_demo(machine);
  run_negative_halfword_load_store_demo(machine);
  run_negative_out_of_range_guard_demo(machine);
}

}  // namespace fn64::bootstrap_detail