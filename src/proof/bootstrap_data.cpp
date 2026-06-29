#include "bootstrap_common.hpp"

#include <exception>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

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

void write_be_u32(std::vector<std::uint8_t>& bytes, std::size_t offset, std::uint32_t value) {
  bytes[offset] = static_cast<std::uint8_t>((value >> 24) & 0xffu);
  bytes[offset + 1] = static_cast<std::uint8_t>((value >> 16) & 0xffu);
  bytes[offset + 2] = static_cast<std::uint8_t>((value >> 8) & 0xffu);
  bytes[offset + 3] = static_cast<std::uint8_t>(value & 0xffu);
}

std::vector<std::uint8_t> make_bootstrap_cartridge_staging_rom(
    std::uint32_t first_instruction,
    std::uint32_t second_instruction) {
  constexpr std::size_t kRomSize = 0x48;
  constexpr std::size_t kProgramOffset = 0x40;

  std::vector<std::uint8_t> rom(kRomSize, 0);
  write_be_u32(rom, 0x00, 0x80371240u);
  write_be_u32(rom, 0x04, 0x0000000fu);
  write_be_u32(rom, 0x08, 0x80000400u);
  write_be_u32(rom, 0x0c, 0x00000000u);
  write_be_u32(rom, 0x10, 0x00000000u);
  write_be_u32(rom, 0x14, 0x00000000u);

  const std::string image_name = "FN64 STAGE TEST";
  for (std::size_t i = 0; i < image_name.size(); ++i) {
    rom[0x20 + i] = static_cast<std::uint8_t>(image_name[i]);
  }

  rom[0x3c] = static_cast<std::uint8_t>('F');
  rom[0x3d] = static_cast<std::uint8_t>('6');
  rom[0x3e] = 0x45u;
  rom[0x3f] = 0x00u;

  write_be_u32(rom, kProgramOffset, first_instruction);
  write_be_u32(rom, kProgramOffset + 4, second_instruction);

  return rom;
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

void require_stage_exception_contains(
    Machine& machine,
    std::uint32_t cartridge_offset,
    std::uint32_t rdram_address,
    std::uint32_t byte_count,
    const char* label,
    const char* expected_substring) {
  try {
    machine.stage_cartridge_bytes_to_rdram(cartridge_offset, rdram_address, byte_count);
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

void require_cpu_rdram_translation(
    const char* label,
    std::uint32_t cpu_address,
    std::size_t width,
    std::uint32_t expected_rdram_address) {
  std::uint32_t translated = 0xffffffffu;
  if (!Machine::translate_cpu_rdram_address(cpu_address, width, translated)) {
    throw std::runtime_error(std::string(label) + " did not translate");
  }

  std::cout << "  " << label << '\n';
  print_hex32("    cpu_address", cpu_address);
  print_hex64("    width", width);
  print_hex32("    rdram_address", translated);

  if (translated != expected_rdram_address) {
    throw std::runtime_error(std::string(label) + " translated to the wrong RDRAM offset");
  }
}

void require_cpu_rdram_translation_failure(
    const char* label,
    std::uint32_t cpu_address,
    std::size_t width) {
  std::uint32_t translated = 0xffffffffu;
  if (Machine::translate_cpu_rdram_address(cpu_address, width, translated)) {
    throw std::runtime_error(std::string(label) + " unexpectedly translated");
  }

  std::cout << "  " << label << " rejected\n";
  print_hex32("    cpu_address", cpu_address);
  print_hex64("    width", width);
}

void run_cartridge_staging_demo() {
  constexpr std::uint32_t kProgramCartridgeOffset = 0x00000040u;
  constexpr std::uint32_t kProgramRdramAddress = 0x00000800u;
  constexpr std::uint32_t kProgramCpuAddress = 0x80000800u;
  constexpr std::uint32_t kProgramByteCount = 8u;
  constexpr std::uint8_t kTargetRegister = 8;
  constexpr std::uint16_t kImmediate = 0x1234u;

  const std::uint32_t kOriInstruction = encode_ori(kTargetRegister, 0, kImmediate);
  const std::uint32_t kBreakInstruction = encode_break();

  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(
          make_bootstrap_cartridge_staging_rom(kOriInstruction, kBreakInstruction),
          cartridge,
          error)) {
    throw std::runtime_error("cartridge staging demo could not load generated ROM: " + error);
  }

  auto staged_machine = std::make_unique<Machine>(std::move(cartridge));
  staged_machine->stage_cartridge_bytes_to_rdram(
      kProgramCartridgeOffset,
      kProgramRdramAddress,
      kProgramByteCount);
  staged_machine->stage_cpu_pc(kProgramCpuAddress);

  std::cout << "fn64 bootstrap cartridge staging demo: cartridge bytes stage into RDRAM and execute from KSEG0\n";
  print_hex32("  cartridge_offset", kProgramCartridgeOffset);
  print_hex32("  staged_rdram_address", kProgramRdramAddress);
  print_hex32("  staged_cpu_pc", kProgramCpuAddress);
  print_rdram_word(*staged_machine, "  staged_rdram[0x00000800]", kProgramRdramAddress);
  print_rdram_word(*staged_machine, "  staged_rdram[0x00000804]", kProgramRdramAddress + 4u);

  if (staged_machine->read_rdram_u32_be(kProgramRdramAddress) != kOriInstruction) {
    throw std::runtime_error("cartridge staging demo did not copy ORI bytes into RDRAM");
  }

  if (staged_machine->read_rdram_u32_be(kProgramRdramAddress + 4u) != kBreakInstruction) {
    throw std::runtime_error("cartridge staging demo did not copy BREAK bytes into RDRAM");
  }

  require_stepped(staged_machine->step_cpu_instruction(), "cartridge_staging_demo_ori");

  if (staged_machine->read_cpu_gpr(kTargetRegister) != kImmediate) {
    throw std::runtime_error("cartridge staging demo ORI did not execute from staged bytes");
  }

  print_hex64("  gpr[8]", staged_machine->read_cpu_gpr(kTargetRegister));

  require_stopped(staged_machine->step_cpu_instruction(), "cartridge_staging_demo_break");
}

void run_cartridge_staging_preflight_demo() {
  constexpr std::uint32_t kProgramCartridgeOffset = 0x00000040u;
  constexpr std::uint32_t kProgramByteCount = 8u;
  constexpr std::uint32_t kSourceFailureOffset = 0x00000046u;
  constexpr std::uint32_t kFailureByteCount = 4u;
  constexpr std::uint32_t kSourceFailureRdramAddress = 0x00000820u;
  constexpr std::uint32_t kSourceFailureSentinel = 0xaabbccddu;
  constexpr std::uint32_t kDestinationFailureSentinel = 0x11223344u;

  const std::uint32_t kOriInstruction = encode_ori(8, 0, 0x1234u);
  const std::uint32_t kBreakInstruction = encode_break();

  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(
          make_bootstrap_cartridge_staging_rom(kOriInstruction, kBreakInstruction),
          cartridge,
          error)) {
    throw std::runtime_error(
        "cartridge staging preflight demo could not load generated ROM: " + error);
  }

  auto preflight_machine = std::make_unique<Machine>(std::move(cartridge));
  const std::uint32_t kDestinationFailureSentinelAddress =
      static_cast<std::uint32_t>(preflight_machine->rdram_size_bytes() - 4);
  const std::uint32_t kDestinationFailureRdramAddress =
      static_cast<std::uint32_t>(preflight_machine->rdram_size_bytes() - 2);

  std::cout
      << "fn64 bootstrap cartridge staging preflight demo: failed staging leaves RDRAM unchanged\n";

  preflight_machine->stage_cartridge_bytes_to_rdram(
      kProgramCartridgeOffset,
      kSourceFailureRdramAddress,
      kProgramByteCount);
  print_rdram_word(
      *preflight_machine,
      "  successful_staged_rdram[0x00000820]",
      kSourceFailureRdramAddress);
  print_rdram_word(
      *preflight_machine,
      "  successful_staged_rdram[0x00000824]",
      kSourceFailureRdramAddress + 4u);

  if (preflight_machine->read_rdram_u32_be(kSourceFailureRdramAddress) != kOriInstruction) {
    throw std::runtime_error("cartridge staging preflight demo did not preserve success copy");
  }

  if (preflight_machine->read_rdram_u32_be(kSourceFailureRdramAddress + 4u) !=
      kBreakInstruction) {
    throw std::runtime_error("cartridge staging preflight demo did not preserve success tail copy");
  }

  preflight_machine->write_rdram_u32_be(kSourceFailureRdramAddress, kSourceFailureSentinel);
  require_stage_exception_contains(
      *preflight_machine,
      kSourceFailureOffset,
      kSourceFailureRdramAddress,
      kFailureByteCount,
      "cartridge_staging_source_preflight",
      "cartridge staging span out of range: cartridge source");

  print_rdram_word(
      *preflight_machine,
      "  source_failure_rdram[0x00000820]",
      kSourceFailureRdramAddress);

  if (preflight_machine->read_rdram_u32_be(kSourceFailureRdramAddress) !=
      kSourceFailureSentinel) {
    throw std::runtime_error(
        "cartridge staging source preflight changed RDRAM before failing");
  }

  preflight_machine->write_rdram_u32_be(
      kDestinationFailureSentinelAddress,
      kDestinationFailureSentinel);
  require_stage_exception_contains(
      *preflight_machine,
      kProgramCartridgeOffset,
      kDestinationFailureRdramAddress,
      kFailureByteCount,
      "cartridge_staging_destination_preflight",
      "cartridge staging span out of range: RDRAM destination");

  print_rdram_word(
      *preflight_machine,
      "  destination_failure_rdram_tail",
      kDestinationFailureSentinelAddress);

  if (preflight_machine->read_rdram_u32_be(kDestinationFailureSentinelAddress) !=
      kDestinationFailureSentinel) {
    throw std::runtime_error(
        "cartridge staging destination preflight changed RDRAM before failing");
  }
}

void run_cpu_rdram_translation_demo(Machine& machine) {
  constexpr std::size_t kRdramSizeBytes = 0x00400000u;
  constexpr std::uint32_t kLastWordRdramAddress = 0x003ffffcu;
  constexpr std::uint32_t kKseg0RdramBase = 0x80000000u;
  constexpr std::uint32_t kKseg1RdramBase = 0xa0000000u;
  constexpr std::uint32_t kFetchWord = 0x34081234u;

  std::cout
      << "fn64 bootstrap CPU RDRAM translation demo: physical/KSEG0/KSEG1 windows and rejection stay explicit\n";

  require_cpu_rdram_translation(
      "physical_zero_word",
      0x00000000u,
      4,
      0x00000000u);
  require_cpu_rdram_translation(
      "physical_tail_word",
      kLastWordRdramAddress,
      4,
      kLastWordRdramAddress);
  require_cpu_rdram_translation(
      "kseg0_zero_word",
      kKseg0RdramBase,
      4,
      0x00000000u);
  require_cpu_rdram_translation(
      "kseg0_tail_word",
      kKseg0RdramBase + kLastWordRdramAddress,
      4,
      kLastWordRdramAddress);
  require_cpu_rdram_translation(
      "kseg1_zero_word",
      kKseg1RdramBase,
      4,
      0x00000000u);
  require_cpu_rdram_translation(
      "kseg1_tail_word",
      kKseg1RdramBase + kLastWordRdramAddress,
      4,
      kLastWordRdramAddress);

  require_cpu_rdram_translation_failure(
      "zero_width",
      0x00000000u,
      0);
  require_cpu_rdram_translation_failure(
      "wider_than_rdram",
      0x00000000u,
      kRdramSizeBytes + 1u);
  require_cpu_rdram_translation_failure(
      "physical_word_crosses_tail",
      0x003ffffdu,
      4);
  require_cpu_rdram_translation_failure(
      "physical_past_rdram",
      0x00400000u,
      4);
  require_cpu_rdram_translation_failure(
      "kseg0_past_rdram",
      0x80400000u,
      4);
  require_cpu_rdram_translation_failure(
      "kseg1_past_rdram",
      0xa0400000u,
      4);

  machine.write_rdram_u32_be(0x00000000u, kFetchWord);
  machine.stage_cpu_pc(kKseg1RdramBase);
  machine.stage_cpu_next_pc(kKseg1RdramBase + 4u);
  machine.stage_cpu_gpr(8, 0);

  std::cout << "  kseg1_step_fetch\n";
  print_hex32("    pc", kKseg1RdramBase);
  print_hex32("    staged_word", kFetchWord);

  require_stepped(machine.step_cpu_instruction(), "kseg1_translation_demo_ori");

  print_control_flow_state(machine);
  print_hex64("    gpr[8]", machine.read_cpu_gpr(8));

  if (machine.cpu_pc() != kKseg1RdramBase + 4u ||
      machine.cpu_next_pc() != kKseg1RdramBase + 8u) {
    throw std::runtime_error("CPU RDRAM translation demo did not step through KSEG1");
  }

  if (machine.read_cpu_gpr(8) != 0x00001234u) {
    throw std::runtime_error("CPU RDRAM translation demo did not execute through KSEG1");
  }
}

void run_cpu_rdram_alias_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 12;

  constexpr std::uint32_t kLwCpuAddress = 0x80000700u;
  constexpr std::uint32_t kBreakCpuAddress = 0x80000704u;
  constexpr std::uint32_t kLwRdramAddress = 0x00000700u;
  constexpr std::uint32_t kBreakRdramAddress = 0x00000704u;

  constexpr std::uint32_t kDataCpuAddress = 0xa0000740u;
  constexpr std::uint32_t kDataRdramAddress = 0x00000740u;
  constexpr std::uint32_t kDataWord = 0xcafef00du;

  const std::uint32_t kLwInstruction = encode_lw(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const std::uint32_t kBreakInstruction = encode_break();

  std::uint32_t translated_fetch = 0;
  std::uint32_t translated_data = 0;
  if (!Machine::translate_cpu_rdram_address(kLwCpuAddress, 4, translated_fetch)) {
    throw std::runtime_error("CPU RDRAM alias demo could not translate KSEG0 fetch");
  }

  if (!Machine::translate_cpu_rdram_address(kDataCpuAddress, 4, translated_data)) {
    throw std::runtime_error("CPU RDRAM alias demo could not translate KSEG1 data");
  }

  if (translated_fetch != kLwRdramAddress || translated_data != kDataRdramAddress) {
    throw std::runtime_error("CPU RDRAM alias demo translated to the wrong RDRAM offset");
  }

  machine.stage_cpu_pc(kLwCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, kDataCpuAddress);
  machine.stage_cpu_gpr(kTargetIndex, 0x00000000u);

  machine.write_rdram_u32_be(kLwRdramAddress, kLwInstruction);
  machine.write_rdram_u32_be(kBreakRdramAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kDataRdramAddress, kDataWord);

  std::cout << "fn64 bootstrap CPU RDRAM alias demo: KSEG0 fetch and KSEG1 data access resolve to local RDRAM\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex32("  kseg0_fetch_pc", kLwCpuAddress);
  print_hex32("  translated_fetch_rdram", translated_fetch);
  print_hex32("  kseg1_data_address", kDataCpuAddress);
  print_hex32("  translated_data_rdram", translated_data);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000740]", kDataRdramAddress);

  const std::uint32_t lw_raw = kLwInstruction;

  print_hex32("  lw_raw", lw_raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_rdram_alias_demo_lw");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != kBreakCpuAddress) {
    throw std::runtime_error("CPU RDRAM alias demo did not advance to KSEG0 BREAK");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != kDataWord) {
    throw std::runtime_error("CPU RDRAM alias demo LW did not read through KSEG1");
  }

  require_stopped(machine.step_cpu_instruction(), "cpu_rdram_alias_demo_break");
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

  machine.stage_cpu_pc(kLwlAddress);
  machine.stage_cpu_gpr(kBaseIndex, kDataWord0Address);
  machine.stage_cpu_gpr(kTargetIndex, 0xaabbccddu);

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

  const std::uint32_t lwl_raw = kLwlInstruction;

  print_hex32("  lwl_raw", lwl_raw);

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

  const std::uint32_t lwr_raw = kLwrInstruction;

  print_hex32("  lwr_raw", lwr_raw);

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

  machine.stage_cpu_pc(kSwlAddress);
  machine.stage_cpu_gpr(kBaseIndex, kDataWord0Address);
  machine.stage_cpu_gpr(kSourceIndex, 0xa1b2c3d4u);

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

  const std::uint32_t swl_raw = kSwlInstruction;

  print_hex32("  swl_raw", swl_raw);

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

  const std::uint32_t swr_raw = kSwrInstruction;

  print_hex32("  swr_raw", swr_raw);

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

struct PartialLoadLaneCase {
  const char* label;
  std::uint32_t instruction;
  std::uint32_t expected_gpr;
};

struct PartialStoreLaneCase {
  const char* label;
  std::uint32_t instruction;
  std::uint32_t expected_memory_word;
};

void run_partial_word_lane_matrix_demo(Machine& machine) {
  constexpr std::uint8_t kBaseIndex = 4;
  constexpr std::uint8_t kLoadTargetIndex = 16;
  constexpr std::uint8_t kStoreSourceIndex = 17;

  constexpr std::uint32_t kInstructionAddress = 0x00000600u;
  constexpr std::uint32_t kAfterInstructionAddress = kInstructionAddress + 4u;
  constexpr std::uint32_t kDataWordAddress = 0x00000640u;
  constexpr std::uint32_t kInitialLoadTarget = 0xaabbccddu;
  constexpr std::uint32_t kLoadMemoryWord = 0x10203040u;
  constexpr std::uint32_t kStoreSource = 0xa1b2c3d4u;
  constexpr std::uint32_t kInitialStoreMemoryWord = 0x11223344u;

  const PartialLoadLaneCase kLoadCases[] = {
      {
          "LWL offset 0",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0000u),
          0x10203040u,
      },
      {
          "LWL offset 1",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0001u),
          0x203040ddu,
      },
      {
          "LWL offset 2",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0002u),
          0x3040ccddu,
      },
      {
          "LWL offset 3",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0003u),
          0x40bbccddu,
      },
      {
          "LWR offset 0",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0000u),
          0xaabbcc10u,
      },
      {
          "LWR offset 1",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0001u),
          0xaabb1020u,
      },
      {
          "LWR offset 2",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0002u),
          0xaa102030u,
      },
      {
          "LWR offset 3",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0003u),
          0x10203040u,
      },
  };

  const PartialStoreLaneCase kStoreCases[] = {
      {
          "SWL offset 0",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0000u),
          0xa1b2c3d4u,
      },
      {
          "SWL offset 1",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0001u),
          0x11a1b2c3u,
      },
      {
          "SWL offset 2",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0002u),
          0x1122a1b2u,
      },
      {
          "SWL offset 3",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0003u),
          0x112233a1u,
      },
      {
          "SWR offset 0",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0000u),
          0xd4223344u,
      },
      {
          "SWR offset 1",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0001u),
          0xc3d43344u,
      },
      {
          "SWR offset 2",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0002u),
          0xb2c3d444u,
      },
      {
          "SWR offset 3",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0003u),
          0xa1b2c3d4u,
      },
  };

  std::cout
      << "fn64 bootstrap partial-word lane matrix demo: LWL/LWR/SWL/SWR local byte lanes\n";

  for (const PartialLoadLaneCase& test_case : kLoadCases) {
    machine.stage_cpu_pc(kInstructionAddress);
    machine.stage_cpu_gpr(kBaseIndex, kDataWordAddress);
    machine.stage_cpu_gpr(kLoadTargetIndex, kInitialLoadTarget);
    machine.write_rdram_u32_be(kInstructionAddress, test_case.instruction);
    machine.write_rdram_u32_be(kDataWordAddress, kLoadMemoryWord);

    std::cout << "load lane row: " << test_case.label << '\n';
    print_control_flow_state(machine);
    print_hex64("  gpr[16]", machine.read_cpu_gpr(kLoadTargetIndex));
    print_rdram_word(machine, "  rdram[0x00000640]", kDataWordAddress);

    require_stepped(machine.step_cpu_instruction(), test_case.label);

    print_hex64("  actual_gpr[16]", machine.read_cpu_gpr(kLoadTargetIndex));
    print_hex32("  expected_gpr[16]", test_case.expected_gpr);

    if (machine.cpu_pc() != kAfterInstructionAddress) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo did not advance after ") +
          test_case.label);
    }

    if (machine.read_cpu_gpr(kLoadTargetIndex) != test_case.expected_gpr) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo result was wrong for ") +
          test_case.label);
    }
  }

  for (const PartialStoreLaneCase& test_case : kStoreCases) {
    machine.stage_cpu_pc(kInstructionAddress);
    machine.stage_cpu_gpr(kBaseIndex, kDataWordAddress);
    machine.stage_cpu_gpr(kStoreSourceIndex, kStoreSource);
    machine.write_rdram_u32_be(kInstructionAddress, test_case.instruction);
    machine.write_rdram_u32_be(kDataWordAddress, kInitialStoreMemoryWord);

    std::cout << "store lane row: " << test_case.label << '\n';
    print_control_flow_state(machine);
    print_hex64("  gpr[17]", machine.read_cpu_gpr(kStoreSourceIndex));
    print_rdram_word(machine, "  rdram[0x00000640]", kDataWordAddress);

    require_stepped(machine.step_cpu_instruction(), test_case.label);

    print_rdram_word(machine, "  actual_rdram[0x00000640]", kDataWordAddress);
    print_hex32("  expected_rdram[0x00000640]", test_case.expected_memory_word);

    if (machine.cpu_pc() != kAfterInstructionAddress) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo did not advance after ") +
          test_case.label);
    }

    if (machine.read_rdram_u32_be(kDataWordAddress) != test_case.expected_memory_word) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo memory word was wrong for ") +
          test_case.label);
    }
  }
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

  machine.stage_cpu_pc(kSwAddress);
  machine.stage_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0x89abcdefu);
  machine.stage_cpu_gpr(kTargetIndex, 0x00000000u);

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

  machine.stage_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0xdeadbeefu);
  machine.stage_cpu_gpr(kTargetIndex, 0x01234567u);
  machine.write_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap aligned word guard demo: explicit local SW/LW natural-alignment failure\n";

  machine.stage_cpu_pc(kSwAddress);

  std::cout << "before SW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kSourceIndex));
  print_hex32("  sw_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

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

  machine.stage_cpu_pc(kLwAddress);
  machine.stage_cpu_gpr(kTargetIndex, 0x01234567u);

  std::cout << "before LW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kTargetIndex));
  print_hex32("  lw_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

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

  machine.stage_cpu_pc(kSbAddress);
  machine.stage_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0x12345680u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

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

  machine.stage_cpu_pc(kShAddress);
  machine.stage_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

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

  machine.stage_cpu_gpr(kBaseIndex, kDataBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);
  machine.write_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap halfword guard demo: explicit local SH/LH/LHU natural-alignment failure\n";

  machine.stage_cpu_pc(kShAddress);

  std::cout << "before SH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[16]", machine.read_cpu_gpr(kSourceIndex));
  print_hex32("  sh_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x000004d0]", kDataBaseAddress);

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

  machine.stage_cpu_pc(kLhAddress);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);

  std::cout << "before LH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[17]", machine.read_cpu_gpr(kSignedTargetIndex));
  print_hex32("  lh_effective_address", kMisalignedAddress);

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

  machine.stage_cpu_pc(kLhuAddress);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  std::cout << "before LHU misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[18]", machine.read_cpu_gpr(kUnsignedTargetIndex));
  print_hex32("  lhu_effective_address", kMisalignedAddress);

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

  machine.stage_cpu_pc(kSwAddress);
  machine.stage_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0x76543210u);
  machine.stage_cpu_gpr(kTargetIndex, 0x00000000u);

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

  machine.stage_cpu_pc(kSbAddress);
  machine.stage_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0x12345680u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

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

  machine.stage_cpu_pc(kShAddress);
  machine.stage_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

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

void run_failed_partial_load_no_ghost_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 30;

  constexpr std::uint32_t kLwlAddress = 0x00000200u;
  constexpr std::uint32_t kLwrAddress = 0x00000204u;

  constexpr std::uint32_t kInvalidKseg1Address = 0xa0400000u;
  constexpr std::uint32_t kTargetSentinel = 0x89abcdefu;

  const std::uint32_t kLwlInstruction = encode_lwl(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const std::uint32_t kLwrInstruction = encode_lwr(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.write_rdram_u32_be(kLwlAddress, kLwlInstruction);
  machine.write_rdram_u32_be(kLwrAddress, kLwrInstruction);
  machine.stage_cpu_gpr(kBaseIndex, kInvalidKseg1Address);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout
      << "fn64 bootstrap failed partial-load no-ghost demo: LWL/LWR faults do not write target GPR or advance control state\n";

  machine.stage_cpu_pc(kLwlAddress);

  std::cout << "before LWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[30]", machine.read_cpu_gpr(kTargetIndex));
  print_hex32("  lwl_effective_address", kInvalidKseg1Address);

  require_step_exception_contains(
      machine,
      "failed_partial_load_demo_lwl",
      "RDRAM access out of range");

  std::cout << "after LWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[30]", machine.read_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != kLwlAddress) {
    throw std::runtime_error("failed partial-load no-ghost demo LWL changed PC on fault");
  }

  if (machine.cpu_next_pc() != kLwlAddress + 4u) {
    throw std::runtime_error("failed partial-load no-ghost demo LWL changed next_pc on fault");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != kTargetSentinel) {
    throw std::runtime_error("failed partial-load no-ghost demo LWL changed target GPR on fault");
  }

  machine.stage_cpu_pc(kLwrAddress);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout << "before LWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[30]", machine.read_cpu_gpr(kTargetIndex));
  print_hex32("  lwr_effective_address", kInvalidKseg1Address);

  require_step_exception_contains(
      machine,
      "failed_partial_load_demo_lwr",
      "RDRAM access out of range");

  std::cout << "after LWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[30]", machine.read_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != kLwrAddress) {
    throw std::runtime_error("failed partial-load no-ghost demo LWR changed PC on fault");
  }

  if (machine.cpu_next_pc() != kLwrAddress + 4u) {
    throw std::runtime_error("failed partial-load no-ghost demo LWR changed next_pc on fault");
  }

  if (machine.read_cpu_gpr(kTargetIndex) != kTargetSentinel) {
    throw std::runtime_error("failed partial-load no-ghost demo LWR changed target GPR on fault");
  }
}

void run_failed_partial_store_no_ghost_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 29;

  constexpr std::uint32_t kSwlAddress = 0x000001f0u;
  constexpr std::uint32_t kSwrAddress = 0x000001f4u;

  constexpr std::uint32_t kInvalidKseg1Address = 0xa0400000u;
  constexpr std::uint32_t kLowSentinelAddress = 0x00000570u;
  constexpr std::uint32_t kTailSentinelAddress = 0x003ffffcu;
  constexpr std::uint32_t kLowSentinel = 0x10203040u;
  constexpr std::uint32_t kTailSentinel = 0x50607080u;

  const std::uint32_t kSwlInstruction = encode_swl(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const std::uint32_t kSwrInstruction = encode_swr(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.write_rdram_u32_be(kSwlAddress, kSwlInstruction);
  machine.write_rdram_u32_be(kSwrAddress, kSwrInstruction);
  machine.write_rdram_u32_be(kLowSentinelAddress, kLowSentinel);
  machine.write_rdram_u32_be(kTailSentinelAddress, kTailSentinel);
  machine.stage_cpu_gpr(kBaseIndex, kInvalidKseg1Address);
  machine.stage_cpu_gpr(kSourceIndex, 0xa1b2c3d4u);

  std::cout
      << "fn64 bootstrap failed partial-store no-ghost demo: SWL/SWR faults do not mutate RDRAM or advance control state\n";

  machine.stage_cpu_pc(kSwlAddress);

  std::cout << "before SWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[29]", machine.read_cpu_gpr(kSourceIndex));
  print_hex32("  swl_effective_address", kInvalidKseg1Address);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  require_step_exception_contains(
      machine,
      "failed_partial_store_demo_swl",
      "RDRAM access out of range");

  std::cout << "after SWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  if (machine.cpu_pc() != kSwlAddress) {
    throw std::runtime_error("failed partial-store no-ghost demo SWL changed PC on fault");
  }

  if (machine.cpu_next_pc() != kSwlAddress + 4u) {
    throw std::runtime_error("failed partial-store no-ghost demo SWL changed next_pc on fault");
  }

  if (machine.read_rdram_u32_be(kLowSentinelAddress) != kLowSentinel ||
      machine.read_rdram_u32_be(kTailSentinelAddress) != kTailSentinel) {
    throw std::runtime_error("failed partial-store no-ghost demo SWL changed RDRAM on fault");
  }

  machine.stage_cpu_pc(kSwrAddress);

  std::cout << "before SWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[29]", machine.read_cpu_gpr(kSourceIndex));
  print_hex32("  swr_effective_address", kInvalidKseg1Address);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  require_step_exception_contains(
      machine,
      "failed_partial_store_demo_swr",
      "RDRAM access out of range");

  std::cout << "after SWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  if (machine.cpu_pc() != kSwrAddress) {
    throw std::runtime_error("failed partial-store no-ghost demo SWR changed PC on fault");
  }

  if (machine.cpu_next_pc() != kSwrAddress + 4u) {
    throw std::runtime_error("failed partial-store no-ghost demo SWR changed next_pc on fault");
  }

  if (machine.read_rdram_u32_be(kLowSentinelAddress) != kLowSentinel ||
      machine.read_rdram_u32_be(kTailSentinelAddress) != kTailSentinel) {
    throw std::runtime_error("failed partial-store no-ghost demo SWR changed RDRAM on fault");
  }
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

  machine.stage_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0x00000080u);
  machine.stage_cpu_gpr(kTargetIndex, 0x89abcdefu);
  machine.write_rdram_u32_be(kSentinelAddress, 0x11223344u);

  std::cout << "fn64 bootstrap negative-offset guard demo: explicit local out-of-range rollback on signed immediate address formation\n";

  machine.stage_cpu_pc(kSbAddress);

  std::cout << "before SB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex32("  sb_immediate_raw", kNegativeOffset);
  print_hex32("  sb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[27]", machine.read_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000550]", kSentinelAddress);

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

  machine.stage_cpu_pc(kLbAddress);
  machine.stage_cpu_gpr(kTargetIndex, 0x89abcdefu);

  std::cout << "before LB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kBaseIndex));
  print_hex32("  lb_immediate_raw", kNegativeOffset);
  print_hex32("  lb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[28]", machine.read_cpu_gpr(kTargetIndex));

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
  run_cartridge_staging_demo();
  run_cartridge_staging_preflight_demo();
  run_cpu_rdram_translation_demo(machine);
  run_cpu_rdram_alias_demo(machine);
  run_unaligned_load_word_demo(machine);
  run_unaligned_store_word_demo(machine);
  run_partial_word_lane_matrix_demo(machine);
  run_aligned_word_load_store_demo(machine);
  run_word_alignment_guard_demo(machine);
  run_byte_load_store_demo(machine);
  run_halfword_load_store_demo(machine);
  run_halfword_alignment_guard_demo(machine);
  run_negative_word_load_store_demo(machine);
  run_negative_byte_load_store_demo(machine);
  run_negative_halfword_load_store_demo(machine);
  run_failed_partial_load_no_ghost_demo(machine);
  run_failed_partial_store_no_ghost_demo(machine);
  run_negative_out_of_range_guard_demo(machine);
}

}  // namespace fn64::bootstrap_detail
