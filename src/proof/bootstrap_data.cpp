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

std::uint32_t read_synthetic_be_u32(
    const std::vector<std::uint8_t>& bytes,
    std::size_t offset) {
  return (static_cast<std::uint32_t>(bytes[offset]) << 24) |
         (static_cast<std::uint32_t>(bytes[offset + 1]) << 16) |
         (static_cast<std::uint32_t>(bytes[offset + 2]) << 8) |
         static_cast<std::uint32_t>(bytes[offset + 3]);
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

std::vector<std::uint8_t> make_synthetic_normalized_rom_proof_image() {
  constexpr std::size_t kRomSize = 0x60;

  std::vector<std::uint8_t> rom(kRomSize, 0);
  write_be_u32(rom, 0x00, 0x80371240u);
  write_be_u32(rom, 0x04, 0x12345678u);
  write_be_u32(rom, 0x08, 0x80246000u);
  write_be_u32(rom, 0x0c, 0x00400000u);
  write_be_u32(rom, 0x10, 0x89abcdefu);
  write_be_u32(rom, 0x14, 0x01234567u);

  const std::string image_name = "FN64 ROM PROOF";
  for (std::size_t i = 0; i < image_name.size(); ++i) {
    rom[0x20 + i] = static_cast<std::uint8_t>(image_name[i]);
  }

  rom[0x3c] = static_cast<std::uint8_t>('F');
  rom[0x3d] = static_cast<std::uint8_t>('R');
  rom[0x3e] = 0x45u;
  rom[0x3f] = 0x07u;

  for (std::size_t offset = 0x40; offset < rom.size(); ++offset) {
    rom[offset] = static_cast<std::uint8_t>((offset * 3u + 0x11u) & 0xffu);
  }

  return rom;
}

std::vector<std::uint8_t> encode_synthetic_rom_source_layout(
    std::vector<std::uint8_t> normalized_bytes,
    RomSourceLayout layout) {
  switch (layout) {
    case RomSourceLayout::kBigEndian:
      return normalized_bytes;

    case RomSourceLayout::kByteSwapped16:
      for (std::size_t i = 0; i < normalized_bytes.size(); i += 2) {
        std::swap(normalized_bytes[i], normalized_bytes[i + 1]);
      }
      return normalized_bytes;

    case RomSourceLayout::kLittleEndian32:
      for (std::size_t i = 0; i < normalized_bytes.size(); i += 4) {
        std::swap(normalized_bytes[i], normalized_bytes[i + 3]);
        std::swap(normalized_bytes[i + 1], normalized_bytes[i + 2]);
      }
      return normalized_bytes;
  }

  return normalized_bytes;
}

void require_synthetic_rom_metadata_matches(
    const RomMetadata& metadata,
    const char* label) {
  if (metadata.header_magic != 0x80371240u) {
    throw std::runtime_error(std::string(label) + " header magic mismatch");
  }

  if (metadata.clock_rate != 0x12345678u ||
      metadata.entry_point != 0x80246000u ||
      metadata.release_address != 0x00400000u ||
      metadata.crc1 != 0x89abcdefu ||
      metadata.crc2 != 0x01234567u) {
    throw std::runtime_error(std::string(label) + " numeric metadata mismatch");
  }

  if (metadata.image_name != "FN64 ROM PROOF" ||
      metadata.cartridge_id != "FR" ||
      metadata.country_code != 0x45u ||
      metadata.revision != 0x07u) {
    throw std::runtime_error(std::string(label) + " text metadata mismatch");
  }
}

void require_rom_metadata_equal(
    const RomMetadata& actual,
    const RomMetadata& expected,
    const char* label) {
  if (actual.header_magic != expected.header_magic ||
      actual.clock_rate != expected.clock_rate ||
      actual.entry_point != expected.entry_point ||
      actual.release_address != expected.release_address ||
      actual.crc1 != expected.crc1 ||
      actual.crc2 != expected.crc2 ||
      actual.image_name != expected.image_name ||
      actual.cartridge_id != expected.cartridge_id ||
      actual.country_code != expected.country_code ||
      actual.revision != expected.revision) {
    throw std::runtime_error(std::string(label) + " metadata mismatch");
  }
}

void require_synthetic_cartridge_bytes_match(
    const Cartridge& cartridge,
    const std::vector<std::uint8_t>& normalized_bytes,
    const char* label) {
  const std::uint32_t offsets[] = {
      0x00u,
      0x01u,
      0x02u,
      0x03u,
      0x10u,
      0x13u,
      0x20u,
      0x2du,
      0x3cu,
      0x3fu,
      0x40u,
      0x41u,
      0x4eu,
      0x5fu,
  };

  for (const std::uint32_t offset : offsets) {
    if (cartridge.read_u8(offset) != normalized_bytes[offset]) {
      throw std::runtime_error(std::string(label) + " normalized byte mismatch");
    }
  }
}

void require_loaded_synthetic_rom(
    const std::vector<std::uint8_t>& normalized_bytes,
    RomSourceLayout source_layout,
    const char* label) {
  const std::vector<std::uint8_t> raw_bytes =
      encode_synthetic_rom_source_layout(normalized_bytes, source_layout);

  NormalizedRomImage normalized_image;
  std::string error;
  if (!normalize_rom_image(raw_bytes, normalized_image, error)) {
    throw std::runtime_error(
        std::string(label) + " normalize_rom_image failed: " + error);
  }

  if (normalized_image.source_layout != source_layout) {
    throw std::runtime_error(std::string(label) + " detected unexpected source layout");
  }

  if (normalized_image.bytes != normalized_bytes) {
    throw std::runtime_error(std::string(label) + " normalized bytes mismatch");
  }

  require_synthetic_rom_metadata_matches(normalized_image.metadata, label);

  Cartridge cartridge;
  if (!load_cartridge(raw_bytes, cartridge, error)) {
    throw std::runtime_error(std::string(label) + " load_cartridge failed: " + error);
  }

  if (!error.empty()) {
    throw std::runtime_error(std::string(label) + " left a stale load error");
  }

  if (cartridge.source_layout() != source_layout) {
    throw std::runtime_error(std::string(label) + " cartridge source layout mismatch");
  }

  if (cartridge.size_bytes() != normalized_bytes.size()) {
    throw std::runtime_error(std::string(label) + " cartridge size mismatch");
  }

  require_synthetic_rom_metadata_matches(cartridge.metadata(), label);
  require_synthetic_cartridge_bytes_match(cartridge, normalized_bytes, label);

  std::cout << "  " << label << " normalized "
            << rom_source_layout_name(cartridge.source_layout()) << '\n';
}

void require_empty_big_endian_cartridge(const Cartridge& cartridge, const char* label) {
  if (cartridge.source_layout() != RomSourceLayout::kBigEndian) {
    throw std::runtime_error(std::string(label) + " did not reset source layout");
  }

  if (cartridge.size_bytes() != 0) {
    throw std::runtime_error(std::string(label) + " did not reset to empty bytes");
  }

  const RomMetadata& metadata = cartridge.metadata();
  if (metadata.header_magic != 0 ||
      metadata.clock_rate != 0 ||
      metadata.entry_point != 0 ||
      metadata.release_address != 0 ||
      metadata.crc1 != 0 ||
      metadata.crc2 != 0 ||
      !metadata.image_name.empty() ||
      !metadata.cartridge_id.empty() ||
      metadata.country_code != 0 ||
      metadata.revision != 0) {
    throw std::runtime_error(std::string(label) + " did not reset metadata");
  }
}

void require_rejected_synthetic_rom(
    const std::vector<std::uint8_t>& known_good_raw_bytes,
    std::vector<std::uint8_t> rejected_raw_bytes,
    const char* label,
    const char* expected_error_substring) {
  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(known_good_raw_bytes, cartridge, error)) {
    throw std::runtime_error(
        std::string(label) + " could not seed output cartridge: " + error);
  }

  error.clear();
  if (load_cartridge(std::move(rejected_raw_bytes), cartridge, error)) {
    throw std::runtime_error(std::string(label) + " unexpectedly loaded");
  }

  std::cout << "  " << label << " rejected: " << error << '\n';

  if (error.find(expected_error_substring) == std::string::npos) {
    throw std::runtime_error(std::string(label) + " returned unexpected error");
  }

  require_empty_big_endian_cartridge(cartridge, label);
}

void run_synthetic_cartridge_read_guard_demo() {
  const std::vector<std::uint8_t> normalized_bytes =
      make_synthetic_normalized_rom_proof_image();
  const std::vector<std::uint8_t> raw_bytes =
      encode_synthetic_rom_source_layout(
          normalized_bytes,
          RomSourceLayout::kBigEndian);

  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(raw_bytes, cartridge, error)) {
    throw std::runtime_error(
        "synthetic cartridge read guard demo could not load generated ROM: " + error);
  }

  std::cout
      << "fn64 bootstrap synthetic cartridge read guard demo: read_u8 success and out-of-range guard\n";

  require_synthetic_cartridge_bytes_match(
      cartridge,
      normalized_bytes,
      "synthetic_cartridge_read_u8_success");

  const RomSourceLayout expected_layout = cartridge.source_layout();
  const std::size_t expected_size = cartridge.size_bytes();
  const RomMetadata expected_metadata = cartridge.metadata();

  try {
    static_cast<void>(cartridge.read_u8(static_cast<std::uint32_t>(expected_size)));
  } catch (const std::out_of_range& e) {
    std::cout << "  synthetic_cartridge_read_u8_out_of_range rejected: "
              << e.what() << '\n';

    if (std::string(e.what()).find("cartridge read out of range") ==
        std::string::npos) {
      throw std::runtime_error(
          "synthetic cartridge read_u8 guard returned unexpected error");
    }

    if (cartridge.source_layout() != expected_layout ||
        cartridge.size_bytes() != expected_size) {
      throw std::runtime_error(
          "synthetic cartridge read_u8 guard changed layout or size");
    }

    const RomMetadata& metadata = cartridge.metadata();
    if (metadata.header_magic != expected_metadata.header_magic ||
        metadata.clock_rate != expected_metadata.clock_rate ||
        metadata.entry_point != expected_metadata.entry_point ||
        metadata.release_address != expected_metadata.release_address ||
        metadata.crc1 != expected_metadata.crc1 ||
        metadata.crc2 != expected_metadata.crc2 ||
        metadata.image_name != expected_metadata.image_name ||
        metadata.cartridge_id != expected_metadata.cartridge_id ||
        metadata.country_code != expected_metadata.country_code ||
        metadata.revision != expected_metadata.revision) {
      throw std::runtime_error(
          "synthetic cartridge read_u8 guard changed metadata");
    }

    require_synthetic_cartridge_bytes_match(
        cartridge,
        normalized_bytes,
        "synthetic_cartridge_read_u8_guard_preserved_bytes");
    return;
  } catch (const std::exception& e) {
    throw std::runtime_error(
        std::string("synthetic cartridge read_u8 guard threw unexpected type: ") +
        e.what());
  }

  throw std::runtime_error(
      "synthetic cartridge read_u8 guard did not reject size offset");
}

std::uint32_t middle_rdram_word_address(const Machine& machine) {
  return static_cast<std::uint32_t>(
      (machine.rdram_size_bytes() / 2u) & ~static_cast<std::size_t>(3u));
}

std::uint32_t tail_rdram_word_address(const Machine& machine) {
  return static_cast<std::uint32_t>(machine.rdram_size_bytes() - 4u);
}

void require_machine_cartridge_matches_source(
    const Machine& machine,
    const Cartridge& source_cartridge,
    const std::vector<std::uint8_t>& normalized_bytes,
    const char* label) {
  const Cartridge& observed_cartridge = machine.cartridge();

  if (observed_cartridge.source_layout() != source_cartridge.source_layout()) {
    throw std::runtime_error(std::string(label) + " source layout mismatch");
  }

  if (observed_cartridge.size_bytes() != source_cartridge.size_bytes() ||
      observed_cartridge.size_bytes() != normalized_bytes.size()) {
    throw std::runtime_error(std::string(label) + " size mismatch");
  }

  require_rom_metadata_equal(
      observed_cartridge.metadata(),
      source_cartridge.metadata(),
      label);
  require_synthetic_rom_metadata_matches(observed_cartridge.metadata(), label);
  require_synthetic_cartridge_bytes_match(observed_cartridge, normalized_bytes, label);
}

void require_blank_machine_power_on_state(const Machine& machine, const char* label) {
  constexpr std::size_t kExpectedRdramSizeBytes = 4u * 1024u * 1024u;

  if (!machine.powered_on()) {
    throw std::runtime_error(std::string(label) + " is not powered on");
  }

  if (machine.rdram_size_bytes() != kExpectedRdramSizeBytes) {
    throw std::runtime_error(std::string(label) + " RDRAM size mismatch");
  }

  if (machine.cpu_pc() != Machine::kBlankInitialCpuPc ||
      machine.cpu_next_pc() != Machine::kBlankInitialCpuNextPc) {
    throw std::runtime_error(std::string(label) + " initial PC mismatch");
  }

  if (machine.inspect_cpu_hi() != 0 || machine.inspect_cpu_lo() != 0) {
    throw std::runtime_error(std::string(label) + " initial HI/LO mismatch");
  }

  const std::size_t gpr_indices[] = {0, 1, 8, 31};
  for (const std::size_t index : gpr_indices) {
    if (machine.inspect_cpu_gpr(index) != 0) {
      throw std::runtime_error(std::string(label) + " initial GPR mismatch");
    }
  }

  if (machine.inspect_rdram_u32_be(0x00000000u) != 0 ||
      machine.inspect_rdram_u32_be(middle_rdram_word_address(machine)) != 0 ||
      machine.inspect_rdram_u32_be(tail_rdram_word_address(machine)) != 0) {
    throw std::runtime_error(std::string(label) + " initial RDRAM mismatch");
  }
}

void run_machine_construction_isolation_demo() {
  constexpr std::uint32_t kLowRdramValue = 0x11223344u;
  constexpr std::uint32_t kMiddleRdramValue = 0xaabbccddu;
  constexpr std::uint32_t kStagePc = 0x00001000u;
  constexpr std::uint32_t kStageNextPc = 0x00001008u;
  constexpr std::uint32_t kStageHi = 0x13572468u;
  constexpr std::uint32_t kStageLo = 0x24681357u;
  constexpr std::uint32_t kStageGpr8 = 0x01020304u;
  constexpr std::uint32_t kStageGpr31 = 0x55667788u;
  constexpr std::uint32_t kProgramCartridgeOffset = 0x00000040u;

  const std::vector<std::uint8_t> normalized_bytes =
      make_synthetic_normalized_rom_proof_image();
  const std::vector<std::uint8_t> raw_bytes =
      encode_synthetic_rom_source_layout(
          normalized_bytes,
          RomSourceLayout::kBigEndian);

  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(raw_bytes, cartridge, error)) {
    throw std::runtime_error(
        "Machine construction demo could not load generated ROM: " + error);
  }

  std::cout
      << "fn64 bootstrap Machine construction demo: blank construction state and instance isolation\n";

  auto constructed_machine = std::make_unique<Machine>(cartridge);
  require_machine_cartridge_matches_source(
      *constructed_machine,
      cartridge,
      normalized_bytes,
      "machine_construction_cartridge_observation");
  require_blank_machine_power_on_state(
      *constructed_machine,
      "machine_construction_blank_power_on_state");

  auto machine_a = std::make_unique<Machine>(cartridge);
  auto machine_b = std::make_unique<Machine>(cartridge);
  require_blank_machine_power_on_state(*machine_a, "machine_a_initial_state");
  require_blank_machine_power_on_state(*machine_b, "machine_b_initial_state");

  const std::uint32_t kMiddleRdramAddress = middle_rdram_word_address(*machine_a);
  const std::uint32_t kTailRdramAddress = tail_rdram_word_address(*machine_a);
  const std::uint32_t kExpectedStagedCartridgeWord =
      read_synthetic_be_u32(normalized_bytes, kProgramCartridgeOffset);

  machine_a->stage_cpu_pc(kStagePc);
  machine_a->stage_cpu_next_pc(kStageNextPc);
  machine_a->stage_cpu_hi(kStageHi);
  machine_a->stage_cpu_lo(kStageLo);
  machine_a->stage_cpu_gpr(8, kStageGpr8);
  machine_a->stage_cpu_gpr(31, kStageGpr31);
  machine_a->stage_rdram_u32_be(0x00000000u, kLowRdramValue);
  machine_a->stage_rdram_u32_be(kMiddleRdramAddress, kMiddleRdramValue);
  machine_a->stage_cartridge_bytes_to_rdram(
      kProgramCartridgeOffset,
      kTailRdramAddress,
      4);

  print_control_flow_state(*machine_a);
  print_hex64("  machine_a_hi", machine_a->inspect_cpu_hi());
  print_hex64("  machine_a_lo", machine_a->inspect_cpu_lo());
  print_hex64("  machine_a_gpr[8]", machine_a->inspect_cpu_gpr(8));
  print_hex64("  machine_a_gpr[31]", machine_a->inspect_cpu_gpr(31));
  print_rdram_word(*machine_a, "  machine_a_rdram[0x00000000]", 0x00000000u);
  print_rdram_word(*machine_a, "  machine_a_rdram_middle", kMiddleRdramAddress);
  print_rdram_word(*machine_a, "  machine_a_rdram_tail", kTailRdramAddress);

  if (machine_a->cpu_pc() != kStagePc ||
      machine_a->cpu_next_pc() != kStageNextPc ||
      machine_a->inspect_cpu_hi() != kStageHi ||
      machine_a->inspect_cpu_lo() != kStageLo ||
      machine_a->inspect_cpu_gpr(8) != kStageGpr8 ||
      machine_a->inspect_cpu_gpr(31) != kStageGpr31) {
    throw std::runtime_error("machine instance isolation demo did not stage CPU state");
  }

  if (machine_a->inspect_rdram_u32_be(0x00000000u) != kLowRdramValue ||
      machine_a->inspect_rdram_u32_be(kMiddleRdramAddress) != kMiddleRdramValue ||
      machine_a->inspect_rdram_u32_be(kTailRdramAddress) != kExpectedStagedCartridgeWord) {
    throw std::runtime_error("machine instance isolation demo did not stage RDRAM state");
  }

  require_blank_machine_power_on_state(
      *machine_b,
      "machine_b_after_machine_a_staging");
  require_machine_cartridge_matches_source(
      *machine_b,
      cartridge,
      normalized_bytes,
      "machine_b_cartridge_after_machine_a_staging");
}

void require_machine_fault(
    const MachineFault& fault,
    const char* label,
    MachineFaultKind expected_kind,
    std::size_t expected_access_size) {
  if (fault.kind() != expected_kind) {
    throw std::runtime_error(std::string(label) + " threw unexpected MachineFault kind");
  }

  if (fault.access_size() != expected_access_size) {
    throw std::runtime_error(std::string(label) + " threw unexpected MachineFault access size");
  }
}

void require_step_machine_fault(
    Machine& machine,
    const char* label,
    MachineFaultKind expected_kind,
    std::size_t expected_access_size) {
  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "  " << label << " threw: " << fault.what() << '\n';
    require_machine_fault(fault, label, expected_kind, expected_access_size);
    return;
  } catch (const std::exception& e) {
    throw std::runtime_error(
        std::string(label) + " threw unexpected exception type: " + e.what());
  }

  throw std::runtime_error(std::string(label) + " did not throw MachineFault");
}

void require_step_cpu_rdram_address_fault(
    Machine& machine,
    const char* label,
    CpuAddress expected_cpu_address,
    std::size_t expected_access_size) {
  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "  " << label << " threw: " << fault.what() << '\n';
    require_machine_fault(fault, label, MachineFaultKind::kCpuRdramAddressRejected, expected_access_size);
    if (fault.cpu_address() != expected_cpu_address) {
      throw std::runtime_error(
          std::string(label) + " reported an unexpected CPU address");
    }
    return;
  } catch (const std::exception& e) {
    throw std::runtime_error(
        std::string(label) + " threw unexpected exception type: " + e.what());
  }

  throw std::runtime_error(std::string(label) + " did not throw CPU RDRAM address MachineFault");
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

void run_synthetic_rom_normalization_rejection_demo() {
  const std::vector<std::uint8_t> normalized_bytes =
      make_synthetic_normalized_rom_proof_image();
  const std::vector<std::uint8_t> known_good_raw_bytes =
      encode_synthetic_rom_source_layout(
          normalized_bytes,
          RomSourceLayout::kBigEndian);

  std::cout
      << "fn64 bootstrap synthetic ROM ingress demo: generated bytes only, no commercial ROM data\n";

  require_loaded_synthetic_rom(
      normalized_bytes,
      RomSourceLayout::kBigEndian,
      "synthetic_z64_big_endian");
  require_loaded_synthetic_rom(
      normalized_bytes,
      RomSourceLayout::kByteSwapped16,
      "synthetic_v64_byte_swapped16");
  require_loaded_synthetic_rom(
      normalized_bytes,
      RomSourceLayout::kLittleEndian32,
      "synthetic_n64_little_endian32");

  require_rejected_synthetic_rom(
      known_good_raw_bytes,
      std::vector<std::uint8_t>{0x80u, 0x37u, 0x12u},
      "synthetic_reject_too_small",
      "too small");

  std::vector<std::uint8_t> non_multiple_of_4 = known_good_raw_bytes;
  non_multiple_of_4.push_back(0x55u);
  require_rejected_synthetic_rom(
      known_good_raw_bytes,
      std::move(non_multiple_of_4),
      "synthetic_reject_non_multiple_of_4",
      "not a multiple of 4");

  std::vector<std::uint8_t> unsupported_layout = known_good_raw_bytes;
  unsupported_layout[0] = 0xdeu;
  unsupported_layout[1] = 0xadu;
  unsupported_layout[2] = 0xbeu;
  unsupported_layout[3] = 0xefu;
  require_rejected_synthetic_rom(
      known_good_raw_bytes,
      std::move(unsupported_layout),
      "synthetic_reject_unsupported_header_layout",
      "unsupported ROM header byte layout");
}

template <typename Action>
void require_public_guard_out_of_range_contains(
    const char* label,
    const char* expected_substring,
    Action action) {
  try {
    action();
  } catch (const std::out_of_range& e) {
    std::cout << "  " << label << " threw: " << e.what() << '\n';

    if (std::string(e.what()).find(expected_substring) == std::string::npos) {
      throw std::runtime_error(
          std::string(label) + " threw unexpected out_of_range text");
    }

    return;
  } catch (const std::exception& e) {
    throw std::runtime_error(
        std::string(label) + " threw unexpected exception type: " + e.what());
  }

  throw std::runtime_error(std::string(label) + " did not throw out_of_range");
}

struct PublicGuardState {
  CpuAddress pc = 0;
  CpuAddress next_pc = 0;
  CpuRegisterValue hi = 0;
  CpuRegisterValue lo = 0;
  CpuRegisterValue gpr4 = 0;
  CpuRegisterValue gpr31 = 0;
  std::uint32_t rdram_low = 0;
  std::uint32_t rdram_tail = 0;
};

PublicGuardState capture_public_guard_state(
    const Machine& machine,
    std::uint32_t low_rdram_address,
    std::uint32_t tail_rdram_address) {
  return PublicGuardState{
      machine.cpu_pc(),
      machine.cpu_next_pc(),
      machine.inspect_cpu_hi(),
      machine.inspect_cpu_lo(),
      machine.inspect_cpu_gpr(4),
      machine.inspect_cpu_gpr(31),
      machine.inspect_rdram_u32_be(low_rdram_address),
      machine.inspect_rdram_u32_be(tail_rdram_address)};
}

void require_public_guard_state_unchanged(
    const Machine& machine,
    const PublicGuardState& expected,
    std::uint32_t low_rdram_address,
    std::uint32_t tail_rdram_address,
    const char* label) {
  const PublicGuardState actual =
      capture_public_guard_state(machine, low_rdram_address, tail_rdram_address);

  if (actual.pc != expected.pc || actual.next_pc != expected.next_pc) {
    throw std::runtime_error(std::string(label) + " changed pc/next_pc");
  }

  if (actual.hi != expected.hi || actual.lo != expected.lo) {
    throw std::runtime_error(std::string(label) + " changed HI/LO");
  }

  if (actual.gpr4 != expected.gpr4 || actual.gpr31 != expected.gpr31) {
    throw std::runtime_error(std::string(label) + " changed selected GPR state");
  }

  if (actual.rdram_low != expected.rdram_low || actual.rdram_tail != expected.rdram_tail) {
    throw std::runtime_error(std::string(label) + " changed RDRAM sentinels");
  }
}

struct CpuRdramWordAccessCase {
  const char* label;
  std::uint32_t instruction_cpu_address;
  std::uint32_t instruction_rdram_address;
  std::uint32_t data_cpu_address;
  std::uint32_t data_rdram_address;
  std::uint32_t data_word;
};

void run_cpu_rdram_load_case(
    Machine& machine,
    const CpuRdramWordAccessCase& test_case) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 12;

  const std::uint32_t kLwInstruction = encode_lw(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(test_case.instruction_cpu_address);
  machine.stage_cpu_gpr(kBaseIndex, test_case.data_cpu_address);
  machine.stage_cpu_gpr(kTargetIndex, 0);

  machine.stage_rdram_u32_be(test_case.instruction_rdram_address, kLwInstruction);
  machine.stage_rdram_u32_be(test_case.instruction_rdram_address + 4u, kBreakInstruction);
  machine.stage_rdram_u32_be(test_case.data_rdram_address, test_case.data_word);

  std::cout << "  load case: " << test_case.label << '\n';
  print_control_flow_state(machine);
  print_hex32("    instruction_cpu_address", test_case.instruction_cpu_address);
  print_hex32("    instruction_rdram_address", test_case.instruction_rdram_address);
  print_hex32("    data_cpu_address", test_case.data_cpu_address);
  print_hex32("    data_rdram_address", test_case.data_rdram_address);
  print_rdram_word(machine, "    staged_data_word", test_case.data_rdram_address);

  require_stepped(machine.step_cpu_instruction(), test_case.label);

  print_control_flow_state(machine);
  print_hex64("    gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != test_case.instruction_cpu_address + 4u) {
    throw std::runtime_error(std::string(test_case.label) + " did not advance to BREAK");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) !=
      cpu_value_from_sign_extended_u32(test_case.data_word)) {
    throw std::runtime_error(std::string(test_case.label) + " loaded the wrong word");
  }

  require_stopped(machine.step_cpu_instruction(), test_case.label);
}

void run_cpu_rdram_store_case(
    Machine& machine,
    const CpuRdramWordAccessCase& test_case) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 13;
  constexpr std::uint32_t kInitialDataWord = 0x01020304u;

  const std::uint32_t kSwInstruction = encode_sw(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(test_case.instruction_cpu_address);
  machine.stage_cpu_gpr(kBaseIndex, test_case.data_cpu_address);
  machine.stage_cpu_gpr(kSourceIndex, test_case.data_word);

  machine.stage_rdram_u32_be(test_case.instruction_rdram_address, kSwInstruction);
  machine.stage_rdram_u32_be(test_case.instruction_rdram_address + 4u, kBreakInstruction);
  machine.stage_rdram_u32_be(test_case.data_rdram_address, kInitialDataWord);

  std::cout << "  store case: " << test_case.label << '\n';
  print_control_flow_state(machine);
  print_hex32("    instruction_cpu_address", test_case.instruction_cpu_address);
  print_hex32("    instruction_rdram_address", test_case.instruction_rdram_address);
  print_hex32("    data_cpu_address", test_case.data_cpu_address);
  print_hex32("    data_rdram_address", test_case.data_rdram_address);
  print_hex64("    gpr[13]", machine.inspect_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "    initial_data_word", test_case.data_rdram_address);

  require_stepped(machine.step_cpu_instruction(), test_case.label);

  print_control_flow_state(machine);
  print_rdram_word(machine, "    stored_data_word", test_case.data_rdram_address);

  if (machine.cpu_pc() != test_case.instruction_cpu_address + 4u) {
    throw std::runtime_error(std::string(test_case.label) + " did not advance to BREAK");
  }

  if (machine.inspect_rdram_u32_be(test_case.data_rdram_address) != test_case.data_word) {
    throw std::runtime_error(std::string(test_case.label) + " stored the wrong word");
  }

  require_stopped(machine.step_cpu_instruction(), test_case.label);
}

void run_kseg1_base_fetch_case(
    Machine& machine,
    const char* label,
    std::uint32_t kseg1_base) {
  constexpr std::uint32_t kOriInstruction = 0x34081234u;
  const std::uint32_t kBreakInstruction = encode_break();

  machine.stage_rdram_u32_be(0x00000000u, kOriInstruction);
  machine.stage_rdram_u32_be(0x00000004u, kBreakInstruction);
  machine.stage_cpu_pc(kseg1_base);
  machine.stage_cpu_next_pc(kseg1_base + 4u);
  machine.stage_cpu_gpr(8, 0);

  std::cout << "  fetch case: " << label << '\n';
  print_hex32("    pc", kseg1_base);
  print_hex32("    instruction_rdram_address", 0x00000000u);
  print_hex32("    staged_word", kOriInstruction);

  require_stepped(machine.step_cpu_instruction(), label);

  print_control_flow_state(machine);
  print_hex64("    gpr[8]", machine.inspect_cpu_gpr(8));

  if (machine.cpu_pc() != kseg1_base + 4u ||
      machine.cpu_next_pc() != kseg1_base + 8u) {
    throw std::runtime_error(std::string(label) + " did not step through KSEG1");
  }

  if (machine.inspect_cpu_gpr(8) != 0x00001234u) {
    throw std::runtime_error(std::string(label) + " did not execute through KSEG1");
  }

  require_stopped(machine.step_cpu_instruction(), label);
}

void run_low_cpu_fetch_rejection_case(Machine& machine) {
  constexpr std::uint32_t kLowCpuPc = 0x00000700u;
  constexpr std::uint32_t kLowCpuNextPc = 0x00000704u;
  constexpr std::uint32_t kInstruction = 0x34081234u;
  constexpr std::uint32_t kSentinelAddress = 0x00000780u;
  constexpr std::uint32_t kSentinelWord = 0xfeedc0deu;

  machine.stage_cpu_pc(kLowCpuPc);
  machine.stage_cpu_next_pc(kLowCpuNextPc);
  machine.stage_cpu_gpr(8, 0);
  machine.stage_rdram_u32_be(kLowCpuPc, kInstruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kSentinelWord);

  std::cout << "  reject case: low_cpu_fetch_rejected\n";
  print_control_flow_state(machine);
  print_hex32("    low_cpu_pc", kLowCpuPc);
  print_hex32("    instruction_rdram_offset", kLowCpuPc);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "    rejected: " << fault.what() << '\n';
    require_machine_fault(
        fault,
        "low_cpu_fetch_rejected",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);
    if (machine.cpu_pc() != kLowCpuPc ||
        machine.cpu_next_pc() != kLowCpuNextPc ||
        machine.inspect_cpu_gpr(8) != 0 ||
        machine.inspect_rdram_u32_be(kSentinelAddress) != kSentinelWord) {
      throw std::runtime_error("low CPU fetch rejection changed visible machine state");
    }
    return;
  } catch (const std::exception& exception) {
    throw std::runtime_error(
        std::string("low CPU fetch rejection threw unexpected exception type: ") +
        exception.what());
  }

  throw std::runtime_error("low CPU fetch unexpectedly executed");
}

void run_low_cpu_data_load_rejection_case(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 12;
  constexpr std::uint32_t kLwCpuAddress = 0x80000790u;
  constexpr std::uint32_t kLwRdramAddress = 0x00000790u;
  constexpr std::uint32_t kLowDataCpuAddress = 0x000007c0u;
  constexpr std::uint32_t kDataWord = 0x10203040u;
  constexpr std::uint32_t kLwInstruction = encode_lw(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.stage_cpu_pc(kLwCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, kLowDataCpuAddress);
  machine.stage_cpu_gpr(kTargetIndex, 0);
  machine.stage_rdram_u32_be(kLwRdramAddress, kLwInstruction);
  machine.stage_rdram_u32_be(kLowDataCpuAddress, kDataWord);

  std::cout << "  reject case: low_cpu_lw_data_address_rejected\n";
  print_control_flow_state(machine);
  print_hex32("    low_data_cpu_address", kLowDataCpuAddress);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "    rejected: " << fault.what() << '\n';
    require_machine_fault(
        fault,
        "low_cpu_lw_data_address_rejected",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);
    if (machine.cpu_pc() != kLwCpuAddress ||
        machine.cpu_next_pc() != kLwCpuAddress + 4u ||
        machine.inspect_cpu_gpr(kTargetIndex) != 0 ||
        machine.inspect_rdram_u32_be(kLowDataCpuAddress) != kDataWord) {
      throw std::runtime_error("low CPU LW data rejection changed visible machine state");
    }
    return;
  } catch (const std::exception& exception) {
    throw std::runtime_error(
        std::string("low CPU LW data rejection threw unexpected exception type: ") +
        exception.what());
  }

  throw std::runtime_error("low CPU LW data address unexpectedly loaded");
}

void run_low_cpu_data_store_rejection_case(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 13;
  constexpr std::uint32_t kSwCpuAddress = 0x800007a0u;
  constexpr std::uint32_t kSwRdramAddress = 0x000007a0u;
  constexpr std::uint32_t kLowDataCpuAddress = 0x000007d0u;
  constexpr std::uint32_t kInitialDataWord = 0xaabbccddu;
  constexpr std::uint32_t kSourceWord = 0x55667788u;
  constexpr std::uint32_t kSwInstruction = encode_sw(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.stage_cpu_pc(kSwCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, kLowDataCpuAddress);
  machine.stage_cpu_gpr(kSourceIndex, kSourceWord);
  machine.stage_rdram_u32_be(kSwRdramAddress, kSwInstruction);
  machine.stage_rdram_u32_be(kLowDataCpuAddress, kInitialDataWord);

  std::cout << "  reject case: low_cpu_sw_data_address_rejected\n";
  print_control_flow_state(machine);
  print_hex32("    low_data_cpu_address", kLowDataCpuAddress);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "    rejected: " << fault.what() << '\n';
    require_machine_fault(
        fault,
        "low_cpu_sw_data_address_rejected",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);
    if (machine.cpu_pc() != kSwCpuAddress ||
        machine.cpu_next_pc() != kSwCpuAddress + 4u ||
        machine.inspect_rdram_u32_be(kLowDataCpuAddress) != kInitialDataWord) {
      throw std::runtime_error("low CPU SW data rejection changed visible machine state");
    }
    return;
  } catch (const std::exception& exception) {
    throw std::runtime_error(
        std::string("low CPU SW data rejection threw unexpected exception type: ") +
        exception.what());
  }

  throw std::runtime_error("low CPU SW data address unexpectedly stored");
}

CpuAddress kseg1_rdram_alias(RdramOffset offset) {
  return 0xa0000000u + offset;
}

void run_cpu_rdram_fetch_rejection_case(const char* label, CpuAddress rejected_pc) {
  constexpr CpuAddress kPreservedNextPc = 0x80000b04u;
  constexpr RdramOffset kSentinelAddress = 0x00000b80u;
  constexpr std::uint32_t kSentinelWord = 0xfeedc0deu;
  constexpr std::size_t kExpectedFetchSize = 4;

  auto machine_storage = std::make_unique<Machine>(Cartridge{});
  Machine& machine = *machine_storage;
  machine.stage_cpu_pc(rejected_pc);
  machine.stage_cpu_next_pc(kPreservedNextPc);
  machine.stage_cpu_gpr(8, 0x1111222233334444ull);
  machine.stage_rdram_u32_be(kSentinelAddress, kSentinelWord);

  std::cout << "  reject case: " << label << '\n';
  print_control_flow_state(machine);
  print_hex32("    rejected_pc", rejected_pc);

  require_step_cpu_rdram_address_fault(machine, label, rejected_pc, kExpectedFetchSize);

  if (machine.cpu_pc() != rejected_pc ||
      machine.cpu_next_pc() != kPreservedNextPc ||
      machine.inspect_cpu_gpr(8) != 0x1111222233334444ull ||
      machine.inspect_rdram_u32_be(kSentinelAddress) != kSentinelWord) {
    throw std::runtime_error(std::string(label) + " changed visible machine state");
  }
}

void run_cpu_rdram_load_rejection_case(
    const char* label,
    CpuInstructionWord instruction,
    CpuAddress data_cpu_address,
    std::size_t expected_access_size) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 12;
  constexpr CpuAddress kInstructionCpuAddress = 0x80000b00u;
  constexpr RdramOffset kInstructionRdramAddress = 0x00000b00u;
  constexpr CpuRegisterValue kTargetSentinel = 0x5555666677778888ull;
  constexpr RdramOffset kSentinelAddress = 0x00000b84u;
  constexpr std::uint32_t kSentinelWord = 0x10203040u;

  auto machine_storage = std::make_unique<Machine>(Cartridge{});
  Machine& machine = *machine_storage;
  machine.stage_cpu_pc(kInstructionCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, data_cpu_address);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);
  machine.stage_rdram_u32_be(kInstructionRdramAddress, instruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kSentinelWord);

  std::cout << "  reject case: " << label << '\n';
  print_control_flow_state(machine);
  print_hex32("    rejected_data_cpu_address", data_cpu_address);

  require_step_cpu_rdram_address_fault(machine, label, data_cpu_address, expected_access_size);

  if (machine.cpu_pc() != kInstructionCpuAddress ||
      machine.cpu_next_pc() != kInstructionCpuAddress + 4u ||
      machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel ||
      machine.inspect_rdram_u32_be(kSentinelAddress) != kSentinelWord) {
    throw std::runtime_error(std::string(label) + " changed visible machine state");
  }
}

void run_cpu_rdram_store_rejection_case(
    const char* label,
    CpuInstructionWord instruction,
    CpuAddress data_cpu_address,
    std::size_t expected_access_size) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 13;
  constexpr CpuAddress kInstructionCpuAddress = 0x80000b20u;
  constexpr RdramOffset kInstructionRdramAddress = 0x00000b20u;
  constexpr CpuRegisterValue kSourceValue = 0xaabbccddeeff0011ull;
  constexpr RdramOffset kSentinelAddress = 0x00000b88u;
  constexpr std::uint32_t kSentinelWord = 0x55667788u;

  auto machine_storage = std::make_unique<Machine>(Cartridge{});
  Machine& machine = *machine_storage;
  machine.stage_cpu_pc(kInstructionCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, data_cpu_address);
  machine.stage_cpu_gpr(kSourceIndex, kSourceValue);
  machine.stage_rdram_u32_be(kInstructionRdramAddress, instruction);
  machine.stage_rdram_u32_be(kSentinelAddress, kSentinelWord);

  std::cout << "  reject case: " << label << '\n';
  print_control_flow_state(machine);
  print_hex32("    rejected_data_cpu_address", data_cpu_address);

  require_step_cpu_rdram_address_fault(machine, label, data_cpu_address, expected_access_size);

  if (machine.cpu_pc() != kInstructionCpuAddress ||
      machine.cpu_next_pc() != kInstructionCpuAddress + 4u ||
      machine.inspect_cpu_gpr(kSourceIndex) != kSourceValue ||
      machine.inspect_rdram_u32_be(kSentinelAddress) != kSentinelWord) {
    throw std::runtime_error(std::string(label) + " changed visible machine state");
  }
}

void run_non_direct_cpu_address_rejection_demo() {
  std::cout << "fn64 bootstrap CPU address rejection demo: non-direct ranges stay local MachineFaults\n";

  run_cpu_rdram_fetch_rejection_case("kuseg_like_fetch_rejected", 0x40000100u);
  run_cpu_rdram_fetch_rejection_case("upper_non_direct_fetch_rejected", 0xc0000100u);
  run_cpu_rdram_fetch_rejection_case("upper_max_aligned_fetch_rejected", 0xfffffffcu);

  run_cpu_rdram_load_rejection_case(
      "kuseg_like_lw_rejected",
      encode_lw(12, 4, 0),
      0x40000100u,
      4);
  run_cpu_rdram_store_rejection_case(
      "upper_non_direct_sw_rejected",
      encode_sw(13, 4, 0),
      0xc0000100u,
      4);
  run_cpu_rdram_store_rejection_case(
      "all_ones_sb_rejected",
      encode_sb(13, 4, 0),
      0xffffffffu,
      1);
}

void run_cpu_rdram_span_boundary_rejection_demo() {
  constexpr RdramOffset kRdramSize = 0x00400000u;

  std::cout << "fn64 bootstrap CPU RDRAM span rejection demo: KSEG aliases must fit Machine RDRAM\n";

  run_cpu_rdram_fetch_rejection_case(
      "kseg0_fetch_end_plus_one_rejected",
      cpu_rdram_alias(kRdramSize));
  run_cpu_rdram_fetch_rejection_case(
      "kseg1_fetch_end_plus_one_rejected",
      kseg1_rdram_alias(kRdramSize));

  run_cpu_rdram_load_rejection_case(
      "kseg0_byte_end_plus_one_rejected",
      encode_lb(12, 4, 0),
      cpu_rdram_alias(kRdramSize),
      1);
  run_cpu_rdram_load_rejection_case(
      "kseg1_byte_end_plus_one_rejected",
      encode_lb(12, 4, 0),
      kseg1_rdram_alias(kRdramSize),
      1);
  run_cpu_rdram_load_rejection_case(
      "kseg0_halfword_end_plus_one_rejected",
      encode_lh(12, 4, 0),
      cpu_rdram_alias(kRdramSize),
      2);
  run_cpu_rdram_load_rejection_case(
      "kseg1_halfword_end_plus_one_rejected",
      encode_lh(12, 4, 0),
      kseg1_rdram_alias(kRdramSize),
      2);
  run_cpu_rdram_load_rejection_case(
      "kseg0_word_end_plus_one_rejected",
      encode_lw(12, 4, 0),
      cpu_rdram_alias(kRdramSize),
      4);
  run_cpu_rdram_load_rejection_case(
      "kseg1_word_end_plus_one_rejected",
      encode_lw(12, 4, 0),
      kseg1_rdram_alias(kRdramSize),
      4);
  run_cpu_rdram_load_rejection_case(
      "kseg0_doubleword_end_plus_one_rejected",
      encode_ld(12, 4, 0),
      cpu_rdram_alias(kRdramSize),
      8);
  run_cpu_rdram_load_rejection_case(
      "kseg1_doubleword_end_plus_one_rejected",
      encode_ld(12, 4, 0),
      kseg1_rdram_alias(kRdramSize),
      8);

  run_cpu_rdram_store_rejection_case(
      "kseg0_sb_end_plus_one_no_ghost",
      encode_sb(13, 4, 0),
      cpu_rdram_alias(kRdramSize),
      1);
  run_cpu_rdram_store_rejection_case(
      "kseg0_sh_end_plus_one_no_ghost",
      encode_sh(13, 4, 0),
      cpu_rdram_alias(kRdramSize),
      2);
  run_cpu_rdram_store_rejection_case(
      "kseg0_sw_end_plus_one_no_ghost",
      encode_sw(13, 4, 0),
      cpu_rdram_alias(kRdramSize),
      4);
  run_cpu_rdram_store_rejection_case(
      "kseg0_sd_end_plus_one_no_ghost",
      encode_sd(13, 4, 0),
      cpu_rdram_alias(kRdramSize),
      8);
}

void run_last_valid_load_case(
    const char* label,
    CpuInstructionWord instruction,
    CpuAddress data_cpu_address,
    CpuRegisterValue expected_value) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 12;
  constexpr CpuAddress kInstructionCpuAddress = 0x80000b40u;
  constexpr RdramOffset kInstructionRdramAddress = 0x00000b40u;
  constexpr RdramOffset kLastDoublewordAddress = 0x003ffff8u;
  constexpr RdramOffset kLastWordAddress = 0x003ffffcu;
  constexpr std::uint32_t kHighWord = 0x01020304u;
  constexpr std::uint32_t kLowWord = 0x11223344u;

  auto machine_storage = std::make_unique<Machine>(Cartridge{});
  Machine& machine = *machine_storage;
  machine.stage_cpu_pc(kInstructionCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, data_cpu_address);
  machine.stage_cpu_gpr(kTargetIndex, 0);
  machine.stage_rdram_u32_be(kInstructionRdramAddress, instruction);
  machine.stage_rdram_u32_be(kLastDoublewordAddress, kHighWord);
  machine.stage_rdram_u32_be(kLastWordAddress, kLowWord);

  std::cout << "  accepted boundary load: " << label << '\n';
  print_hex32("    data_cpu_address", data_cpu_address);

  require_stepped(machine.step_cpu_instruction(), label);

  if (machine.inspect_cpu_gpr(kTargetIndex) != expected_value) {
    throw std::runtime_error(std::string(label) + " loaded an unexpected value");
  }
}

void run_last_valid_store_case(
    const char* label,
    CpuInstructionWord instruction,
    CpuAddress data_cpu_address,
    CpuRegisterValue source_value,
    std::uint32_t expected_high_word,
    std::uint32_t expected_low_word) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 13;
  constexpr CpuAddress kInstructionCpuAddress = 0x80000b60u;
  constexpr RdramOffset kInstructionRdramAddress = 0x00000b60u;
  constexpr RdramOffset kLastDoublewordAddress = 0x003ffff8u;
  constexpr RdramOffset kLastWordAddress = 0x003ffffcu;

  auto machine_storage = std::make_unique<Machine>(Cartridge{});
  Machine& machine = *machine_storage;
  machine.stage_cpu_pc(kInstructionCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, data_cpu_address);
  machine.stage_cpu_gpr(kSourceIndex, source_value);
  machine.stage_rdram_u32_be(kInstructionRdramAddress, instruction);
  machine.stage_rdram_u32_be(kLastDoublewordAddress, 0x01020304u);
  machine.stage_rdram_u32_be(kLastWordAddress, 0x11223344u);

  std::cout << "  accepted boundary store: " << label << '\n';
  print_hex32("    data_cpu_address", data_cpu_address);

  require_stepped(machine.step_cpu_instruction(), label);

  if (machine.inspect_rdram_u32_be(kLastDoublewordAddress) != expected_high_word ||
      machine.inspect_rdram_u32_be(kLastWordAddress) != expected_low_word) {
    throw std::runtime_error(std::string(label) + " stored outside the expected RDRAM bytes");
  }
}

void run_cpu_rdram_last_valid_access_demo() {
  constexpr RdramOffset kRdramSize = 0x00400000u;

  std::cout << "fn64 bootstrap CPU RDRAM boundary demo: last valid alias accesses still work\n";

  run_last_valid_load_case(
      "kseg0_last_valid_byte_load",
      encode_lb(12, 4, 0),
      cpu_rdram_alias(kRdramSize - 1u),
      0x44u);
  run_last_valid_load_case(
      "kseg1_last_valid_halfword_load",
      encode_lh(12, 4, 0),
      kseg1_rdram_alias(kRdramSize - 2u),
      0x3344u);
  run_last_valid_load_case(
      "kseg0_last_valid_word_load",
      encode_lw(12, 4, 0),
      cpu_rdram_alias(kRdramSize - 4u),
      0x11223344u);
  run_last_valid_load_case(
      "kseg1_last_valid_doubleword_load",
      encode_ld(12, 4, 0),
      kseg1_rdram_alias(kRdramSize - 8u),
      0x0102030411223344ull);

  run_last_valid_store_case(
      "kseg0_last_valid_byte_store",
      encode_sb(13, 4, 0),
      cpu_rdram_alias(kRdramSize - 1u),
      0xaau,
      0x01020304u,
      0x112233aau);
  run_last_valid_store_case(
      "kseg1_last_valid_halfword_store",
      encode_sh(13, 4, 0),
      kseg1_rdram_alias(kRdramSize - 2u),
      0xbbeeu,
      0x01020304u,
      0x1122bbeeu);
  run_last_valid_store_case(
      "kseg0_last_valid_word_store",
      encode_sw(13, 4, 0),
      cpu_rdram_alias(kRdramSize - 4u),
      0xcafef00du,
      0x01020304u,
      0xcafef00du);
  run_last_valid_store_case(
      "kseg1_last_valid_doubleword_store",
      encode_sd(13, 4, 0),
      kseg1_rdram_alias(kRdramSize - 8u),
      0x0123456789abcdefull,
      0x01234567u,
      0x89abcdefu);
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

  if (staged_machine->inspect_rdram_u32_be(kProgramRdramAddress) != kOriInstruction) {
    throw std::runtime_error("cartridge staging demo did not copy ORI bytes into RDRAM");
  }

  if (staged_machine->inspect_rdram_u32_be(kProgramRdramAddress + 4u) != kBreakInstruction) {
    throw std::runtime_error("cartridge staging demo did not copy BREAK bytes into RDRAM");
  }

  require_stepped(staged_machine->step_cpu_instruction(), "cartridge_staging_demo_ori");

  if (staged_machine->inspect_cpu_gpr(kTargetRegister) != kImmediate) {
    throw std::runtime_error("cartridge staging demo ORI did not execute from staged bytes");
  }

  print_hex64("  gpr[8]", staged_machine->inspect_cpu_gpr(kTargetRegister));

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

  if (preflight_machine->inspect_rdram_u32_be(kSourceFailureRdramAddress) != kOriInstruction) {
    throw std::runtime_error("cartridge staging preflight demo did not preserve success copy");
  }

  if (preflight_machine->inspect_rdram_u32_be(kSourceFailureRdramAddress + 4u) !=
      kBreakInstruction) {
    throw std::runtime_error("cartridge staging preflight demo did not preserve success tail copy");
  }

  preflight_machine->stage_rdram_u32_be(kSourceFailureRdramAddress, kSourceFailureSentinel);
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

  if (preflight_machine->inspect_rdram_u32_be(kSourceFailureRdramAddress) !=
      kSourceFailureSentinel) {
    throw std::runtime_error(
        "cartridge staging source preflight changed RDRAM before failing");
  }

  preflight_machine->stage_rdram_u32_be(
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

  if (preflight_machine->inspect_rdram_u32_be(kDestinationFailureSentinelAddress) !=
      kDestinationFailureSentinel) {
    throw std::runtime_error(
        "cartridge staging destination preflight changed RDRAM before failing");
  }
}

void run_public_machine_stage_inspect_guard_demo() {
  constexpr std::uint32_t kProgramCartridgeOffset = 0x00000040u;
  constexpr std::uint32_t kSourceSpanOverflowOffset = 0xfffffffcu;
  constexpr std::uint32_t kDestinationSpanOverflowAddress = 0xfffffffcu;
  constexpr std::uint32_t kSpanOverflowByteCount = 8u;
  constexpr std::uint32_t kPc = 0x000009c0u;
  constexpr std::uint32_t kNextPc = 0x000009c4u;
  constexpr std::uint32_t kHi = 0x13572468u;
  constexpr std::uint32_t kLo = 0x24681357u;
  constexpr std::uint32_t kGpr4 = 0xaabbccddu;
  constexpr std::uint32_t kGpr31 = 0x11223344u;
  constexpr std::uint32_t kLowRdramAddress = 0x00000980u;
  constexpr std::uint32_t kLowRdramSentinel = 0xfeedc0deu;
  constexpr std::uint32_t kTailRdramSentinel = 0x55aa33ccu;

  const std::uint32_t kOriInstruction = encode_ori(8, 0, 0x1234u);
  const std::uint32_t kBreakInstruction = encode_break();

  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(
          make_bootstrap_cartridge_staging_rom(kOriInstruction, kBreakInstruction),
          cartridge,
          error)) {
    throw std::runtime_error(
        "public Machine guard demo could not load generated ROM: " + error);
  }

  auto guard_machine = std::make_unique<Machine>(std::move(cartridge));
  const std::uint32_t kTailRdramAddress =
      static_cast<std::uint32_t>(guard_machine->rdram_size_bytes() - 4);
  const std::uint32_t kInvalidRdramWordAddress =
      static_cast<std::uint32_t>(guard_machine->rdram_size_bytes() - 2);

  guard_machine->stage_cpu_pc(kPc);
  guard_machine->stage_cpu_next_pc(kNextPc);
  guard_machine->stage_cpu_hi(kHi);
  guard_machine->stage_cpu_lo(kLo);
  guard_machine->stage_cpu_gpr(4, kGpr4);
  guard_machine->stage_cpu_gpr(31, kGpr31);
  guard_machine->stage_rdram_u32_be(kLowRdramAddress, kLowRdramSentinel);
  guard_machine->stage_rdram_u32_be(kTailRdramAddress, kTailRdramSentinel);

  const PublicGuardState expected =
      capture_public_guard_state(*guard_machine, kLowRdramAddress, kTailRdramAddress);

  std::cout
      << "fn64 bootstrap public Machine guard demo: invalid public stage/inspect inputs leave state unchanged\n";
  print_control_flow_state(*guard_machine);
  print_hex64("  hi", guard_machine->inspect_cpu_hi());
  print_hex64("  lo", guard_machine->inspect_cpu_lo());
  print_hex64("  gpr[4]", guard_machine->inspect_cpu_gpr(4));
  print_hex64("  gpr[31]", guard_machine->inspect_cpu_gpr(31));
  print_rdram_word(*guard_machine, "  rdram[0x00000980]", kLowRdramAddress);
  print_rdram_word(*guard_machine, "  rdram_tail", kTailRdramAddress);

  require_public_guard_out_of_range_contains(
      "public_guard_stage_cpu_gpr_index",
      "CPU GPR index out of range",
      [&guard_machine]() { guard_machine->stage_cpu_gpr(32, 0x01020304u); });
  require_public_guard_state_unchanged(
      *guard_machine,
      expected,
      kLowRdramAddress,
      kTailRdramAddress,
      "public_guard_stage_cpu_gpr_index");

  require_public_guard_out_of_range_contains(
      "public_guard_inspect_cpu_gpr_index",
      "CPU GPR index out of range",
      [&guard_machine]() {
        static_cast<void>(guard_machine->inspect_cpu_gpr(32));
      });
  require_public_guard_state_unchanged(
      *guard_machine,
      expected,
      kLowRdramAddress,
      kTailRdramAddress,
      "public_guard_inspect_cpu_gpr_index");

  require_public_guard_out_of_range_contains(
      "public_guard_stage_rdram_u32_be_range",
      "RDRAM access out of range",
      [&guard_machine, kInvalidRdramWordAddress]() {
        guard_machine->stage_rdram_u32_be(kInvalidRdramWordAddress, 0x01020304u);
      });
  require_public_guard_state_unchanged(
      *guard_machine,
      expected,
      kLowRdramAddress,
      kTailRdramAddress,
      "public_guard_stage_rdram_u32_be_range");

  require_public_guard_out_of_range_contains(
      "public_guard_inspect_rdram_u32_be_range",
      "RDRAM access out of range",
      [&guard_machine, kInvalidRdramWordAddress]() {
        static_cast<void>(guard_machine->inspect_rdram_u32_be(kInvalidRdramWordAddress));
      });
  require_public_guard_state_unchanged(
      *guard_machine,
      expected,
      kLowRdramAddress,
      kTailRdramAddress,
      "public_guard_inspect_rdram_u32_be_range");

  require_public_guard_out_of_range_contains(
      "public_guard_cartridge_source_span_overflow",
      "cartridge staging span overflows 32-bit address space: cartridge source",
      [&guard_machine, kSourceSpanOverflowOffset, kLowRdramAddress, kSpanOverflowByteCount]() {
        guard_machine->stage_cartridge_bytes_to_rdram(
            kSourceSpanOverflowOffset,
            kLowRdramAddress,
            kSpanOverflowByteCount);
      });
  require_public_guard_state_unchanged(
      *guard_machine,
      expected,
      kLowRdramAddress,
      kTailRdramAddress,
      "public_guard_cartridge_source_span_overflow");

  require_public_guard_out_of_range_contains(
      "public_guard_cartridge_destination_span_overflow",
      "cartridge staging span overflows 32-bit address space: RDRAM destination",
      [&guard_machine,
       kProgramCartridgeOffset,
       kDestinationSpanOverflowAddress,
       kSpanOverflowByteCount]() {
        guard_machine->stage_cartridge_bytes_to_rdram(
            kProgramCartridgeOffset,
            kDestinationSpanOverflowAddress,
            kSpanOverflowByteCount);
      });
  require_public_guard_state_unchanged(
      *guard_machine,
      expected,
      kLowRdramAddress,
      kTailRdramAddress,
      "public_guard_cartridge_destination_span_overflow");

  guard_machine->stage_cartridge_bytes_to_rdram(
      kSourceSpanOverflowOffset,
      kDestinationSpanOverflowAddress,
      0);
  require_public_guard_state_unchanged(
      *guard_machine,
      expected,
      kLowRdramAddress,
      kTailRdramAddress,
      "public_guard_cartridge_zero_count_noop");
}

void run_cpu_rdram_translation_demo(Machine& machine) {
  constexpr std::uint32_t kKseg0RdramBase = 0x80000000u;
  constexpr std::uint32_t kKseg1RdramBase = 0xa0000000u;

  std::cout
      << "fn64 bootstrap CPU RDRAM translation demo: low CPU addresses reject; KSEG0/KSEG1 stay step-visible\n";

  run_low_cpu_fetch_rejection_case(machine);
  run_low_cpu_data_load_rejection_case(machine);
  run_low_cpu_data_store_rejection_case(machine);
  run_non_direct_cpu_address_rejection_demo();
  run_cpu_rdram_span_boundary_rejection_demo();
  run_cpu_rdram_last_valid_access_demo();
  run_kseg1_base_fetch_case(
      machine,
      "kseg1_base_fetch_executes_rdram_zero",
      kKseg1RdramBase);

  const CpuRdramWordAccessCase load_cases[] = {
      {
          "kseg0_fetch_kseg0_load",
          kKseg0RdramBase + 0x00000708u,
          0x00000708u,
          kKseg0RdramBase + 0x00000744u,
          0x00000744u,
          0x50607080u,
      },
      {
          "kseg1_fetch_kseg1_load",
          kKseg1RdramBase + 0x00000710u,
          0x00000710u,
          kKseg1RdramBase + 0x00000748u,
          0x00000748u,
          0xcafef00du,
      },
  };

  for (const CpuRdramWordAccessCase& test_case : load_cases) {
    run_cpu_rdram_load_case(machine, test_case);
  }

  const CpuRdramWordAccessCase store_cases[] = {
      {
          "kseg0_fetch_kseg0_store",
          kKseg0RdramBase + 0x00000728u,
          0x00000728u,
          kKseg0RdramBase + 0x00000764u,
          0x00000764u,
          0xa1b2c3d4u,
      },
      {
          "kseg1_fetch_kseg1_store",
          kKseg1RdramBase + 0x00000730u,
          0x00000730u,
          kKseg1RdramBase + 0x00000768u,
          0x00000768u,
          0x0badf00du,
      },
  };

  for (const CpuRdramWordAccessCase& test_case : store_cases) {
    run_cpu_rdram_store_case(machine, test_case);
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

  machine.stage_cpu_pc(kLwCpuAddress);
  machine.stage_cpu_gpr(kBaseIndex, kDataCpuAddress);
  machine.stage_cpu_gpr(kTargetIndex, 0x00000000u);

  machine.stage_rdram_u32_be(kLwRdramAddress, kLwInstruction);
  machine.stage_rdram_u32_be(kBreakRdramAddress, kBreakInstruction);
  machine.stage_rdram_u32_be(kDataRdramAddress, kDataWord);

  std::cout << "fn64 bootstrap CPU RDRAM alias demo: KSEG0 fetch and KSEG1 data access resolve to local RDRAM\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex32("  kseg0_fetch_pc", kLwCpuAddress);
  print_hex32("  expected_fetch_rdram", kLwRdramAddress);
  print_hex32("  kseg1_data_address", kDataCpuAddress);
  print_hex32("  expected_data_rdram", kDataRdramAddress);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000740]", kDataRdramAddress);

  const std::uint32_t lw_raw = kLwInstruction;

  print_hex32("  lw_raw", lw_raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_rdram_alias_demo_lw");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != kBreakCpuAddress) {
    throw std::runtime_error("CPU RDRAM alias demo did not advance to KSEG0 BREAK");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) !=
      cpu_value_from_sign_extended_u32(kDataWord)) {
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
  constexpr std::uint32_t kReverseLwrAddress = 0x000000bcu;
  constexpr std::uint32_t kReverseLwlAddress = 0x000000c0u;
  constexpr std::uint32_t kReverseBreakAddress = 0x000000c4u;

  constexpr std::uint32_t kDataWord0Address = 0x00000410u;
  constexpr std::uint32_t kDataWord1Address = 0x00000414u;
  constexpr std::uint32_t kMergedWordAddress = 0x00000412u;
  constexpr CpuRegisterValue kInitialTarget = 0x11223344aabbccddull;
  constexpr CpuRegisterValue kExpectedLwl =
      cpu_value_from_sign_extended_u32(0x8000ccddu);
  constexpr CpuRegisterValue kExpectedLwrPartial =
      0x11223344aabb007full;
  constexpr CpuRegisterValue kExpectedPair =
      cpu_value_from_sign_extended_u32(0x8000007fu);

  const std::uint32_t kLwlInstruction = encode_lwl(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0002u);
  const std::uint32_t kLwrInstruction = encode_lwr(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0005u);
  const std::uint32_t kReverseLwrInstruction = kLwrInstruction;
  const std::uint32_t kReverseLwlInstruction = kLwlInstruction;
  const std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLwlAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataWord0Address));
  machine.stage_cpu_gpr(kTargetIndex, kInitialTarget);

  machine.stage_rdram_u32_be(kLwlAddress, kLwlInstruction);
  machine.stage_rdram_u32_be(kLwrAddress, kLwrInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);
  machine.stage_rdram_u32_be(kReverseLwrAddress, kReverseLwrInstruction);
  machine.stage_rdram_u32_be(kReverseLwlAddress, kReverseLwlInstruction);
  machine.stage_rdram_u32_be(kReverseBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataWord0Address, 0x11228000u);
  machine.stage_rdram_u32_be(kDataWord1Address, 0x007f7788u);

  std::cout
      << "fn64 bootstrap unaligned load demo: LWL/LWR local 64-bit storage policy\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000410]", kDataWord0Address);
  print_rdram_word(machine, "  rdram[0x00000414]", kDataWord1Address);

  const std::uint32_t lwl_raw = kLwlInstruction;

  print_hex32("  lwl_raw", lwl_raw);

  require_stepped(machine.step_cpu_instruction(), "unaligned_load_demo_lwl");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLwrAddress)) {
    throw std::runtime_error("unaligned load demo did not advance to LWR");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kExpectedLwl) {
    throw std::runtime_error(
        "unaligned load demo LWL did not sign-extend the merged low word");
  }

  const std::uint32_t lwr_raw = kLwrInstruction;

  print_hex32("  lwr_raw", lwr_raw);

  require_stepped(machine.step_cpu_instruction(), "unaligned_load_demo_lwr");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000412]", kMergedWordAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("unaligned load demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kExpectedPair) {
    throw std::runtime_error(
        "unaligned load demo LWL/LWR pair did not produce a sign-extended word");
  }

  require_stopped(machine.step_cpu_instruction(), "unaligned_load_demo_break");

  machine.stage_cpu_pc(cpu_rdram_alias(kReverseLwrAddress));
  machine.stage_cpu_gpr(kTargetIndex, kInitialTarget);

  std::cout << "reverse pair before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kTargetIndex));

  print_hex32("  reverse_lwr_raw", kReverseLwrInstruction);

  require_stepped(machine.step_cpu_instruction(), "unaligned_load_demo_reverse_lwr");

  std::cout << "reverse pair after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kReverseLwlAddress)) {
    throw std::runtime_error("unaligned load reverse demo did not advance to LWL");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kExpectedLwrPartial) {
    throw std::runtime_error(
        "unaligned load reverse demo LWR did not preserve the previous high word");
  }

  print_hex32("  reverse_lwl_raw", kReverseLwlInstruction);

  require_stepped(machine.step_cpu_instruction(), "unaligned_load_demo_reverse_lwl");

  std::cout << "reverse pair after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kReverseBreakAddress)) {
    throw std::runtime_error(
        "unaligned load reverse demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kExpectedPair) {
    throw std::runtime_error(
        "unaligned load demo LWR/LWL pair did not produce a sign-extended word");
  }

  require_stopped(machine.step_cpu_instruction(), "unaligned_load_demo_reverse_break");
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
  constexpr CpuRegisterValue kStoreSource = 0xaabbccdd11223344ull;

  const std::uint32_t kSwlInstruction = encode_swl(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0002u);
  const std::uint32_t kSwrInstruction = encode_swr(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0005u);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kSwlAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataWord0Address));
  machine.stage_cpu_gpr(kSourceIndex, kStoreSource);

  machine.stage_rdram_u32_be(kSwlAddress, kSwlInstruction);
  machine.stage_rdram_u32_be(kSwrAddress, kSwrInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataWord0Address, 0x55667788u);
  machine.stage_rdram_u32_be(kDataWord1Address, 0x99aabbccu);

  std::cout << "fn64 bootstrap unaligned store demo: explicit local SWL/SWR shaping\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000430]", kDataWord0Address);
  print_rdram_word(machine, "  rdram[0x00000434]", kDataWord1Address);

  const std::uint32_t swl_raw = kSwlInstruction;

  print_hex32("  swl_raw", swl_raw);

  require_stepped(machine.step_cpu_instruction(), "unaligned_store_demo_swl");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000430]", kDataWord0Address);
  print_rdram_word(machine, "  rdram[0x00000434]", kDataWord1Address);

  if (machine.cpu_pc() != cpu_rdram_alias(kSwrAddress)) {
    throw std::runtime_error("unaligned store demo did not advance to SWR");
  }

  if (machine.inspect_rdram_u32_be(kDataWord0Address) != 0x55661122u) {
    throw std::runtime_error("unaligned store demo SWL shaping was wrong");
  }

  if (machine.inspect_rdram_u32_be(kDataWord1Address) != 0x99aabbccu) {
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

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("unaligned store demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_rdram_u32_be(kDataWord0Address) != 0x55661122u) {
    throw std::runtime_error("unaligned store demo SWR unexpectedly changed the left aligned word");
  }

  if (machine.inspect_rdram_u32_be(kDataWord1Address) != 0x3344bbccu) {
    throw std::runtime_error("unaligned store demo SWR shaping was wrong");
  }

  if (machine.inspect_rdram_u32_be(kMergedWordAddress) != 0x11223344u) {
    throw std::runtime_error("unaligned store demo SWL/SWR pair did not reconstruct the unaligned word");
  }

  require_stopped(machine.step_cpu_instruction(), "unaligned_store_demo_break");
}

struct PartialLoadLaneCase {
  const char* label;
  std::uint32_t instruction;
  CpuRegisterValue expected_gpr;
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
  constexpr CpuRegisterValue kInitialLoadTarget = 0x11223344aabbccddull;
  constexpr std::uint32_t kLoadMemoryWord = 0x8000007fu;
  constexpr CpuRegisterValue kStoreSource = 0xaabbccdd11223344ull;
  constexpr std::uint32_t kInitialStoreMemoryWord = 0x55667788u;

  const PartialLoadLaneCase kLoadCases[] = {
      {
          "LWL offset 0",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0000u),
          cpu_value_from_sign_extended_u32(0x8000007fu),
      },
      {
          "LWL offset 1",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0001u),
          cpu_value_from_sign_extended_u32(0x00007fddu),
      },
      {
          "LWL offset 2",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0002u),
          cpu_value_from_sign_extended_u32(0x007fccddu),
      },
      {
          "LWL offset 3",
          encode_lwl(kLoadTargetIndex, kBaseIndex, 0x0003u),
          cpu_value_from_sign_extended_u32(0x7fbbccddu),
      },
      {
          "LWR offset 0",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0000u),
          0x11223344aabbcc80ull,
      },
      {
          "LWR offset 1",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0001u),
          0x11223344aabb8000ull,
      },
      {
          "LWR offset 2",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0002u),
          0x11223344aa800000ull,
      },
      {
          "LWR offset 3",
          encode_lwr(kLoadTargetIndex, kBaseIndex, 0x0003u),
          cpu_value_from_sign_extended_u32(0x8000007fu),
      },
  };

  const PartialStoreLaneCase kStoreCases[] = {
      {
          "SWL offset 0",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0000u),
          0x11223344u,
      },
      {
          "SWL offset 1",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0001u),
          0x55112233u,
      },
      {
          "SWL offset 2",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0002u),
          0x55661122u,
      },
      {
          "SWL offset 3",
          encode_swl(kStoreSourceIndex, kBaseIndex, 0x0003u),
          0x55667711u,
      },
      {
          "SWR offset 0",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0000u),
          0x44667788u,
      },
      {
          "SWR offset 1",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0001u),
          0x33447788u,
      },
      {
          "SWR offset 2",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0002u),
          0x22334488u,
      },
      {
          "SWR offset 3",
          encode_swr(kStoreSourceIndex, kBaseIndex, 0x0003u),
          0x11223344u,
      },
  };

  std::cout
      << "fn64 bootstrap partial-word lane matrix demo: LWL/LWR/SWL/SWR local 64-bit storage policy\n";

  for (const PartialLoadLaneCase& test_case : kLoadCases) {
    machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataWordAddress));
    machine.stage_cpu_gpr(kLoadTargetIndex, kInitialLoadTarget);
    machine.stage_rdram_u32_be(kInstructionAddress, test_case.instruction);
    machine.stage_rdram_u32_be(kDataWordAddress, kLoadMemoryWord);

    std::cout << "load lane row: " << test_case.label << '\n';
    print_control_flow_state(machine);
    print_hex64("  gpr[16]", machine.inspect_cpu_gpr(kLoadTargetIndex));
    print_rdram_word(machine, "  rdram[0x00000640]", kDataWordAddress);

    require_stepped(machine.step_cpu_instruction(), test_case.label);

    print_hex64("  actual_gpr[16]", machine.inspect_cpu_gpr(kLoadTargetIndex));
    print_hex64("  expected_gpr[16]", test_case.expected_gpr);

    if (machine.cpu_pc() != cpu_rdram_alias(kAfterInstructionAddress)) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo did not advance after ") +
          test_case.label);
    }

    if (machine.inspect_cpu_gpr(kLoadTargetIndex) != test_case.expected_gpr) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo result was wrong for ") +
          test_case.label);
    }
  }

  for (const PartialStoreLaneCase& test_case : kStoreCases) {
    machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataWordAddress));
    machine.stage_cpu_gpr(kStoreSourceIndex, kStoreSource);
    machine.stage_rdram_u32_be(kInstructionAddress, test_case.instruction);
    machine.stage_rdram_u32_be(kDataWordAddress, kInitialStoreMemoryWord);

    std::cout << "store lane row: " << test_case.label << '\n';
    print_control_flow_state(machine);
    print_hex64("  gpr[17]", machine.inspect_cpu_gpr(kStoreSourceIndex));
    print_rdram_word(machine, "  rdram[0x00000640]", kDataWordAddress);

    require_stepped(machine.step_cpu_instruction(), test_case.label);

    print_rdram_word(machine, "  actual_rdram[0x00000640]", kDataWordAddress);
    print_hex32("  expected_rdram[0x00000640]", test_case.expected_memory_word);

    if (machine.cpu_pc() != cpu_rdram_alias(kAfterInstructionAddress)) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo did not advance after ") +
          test_case.label);
    }

    if (machine.inspect_rdram_u32_be(kDataWordAddress) != test_case.expected_memory_word) {
      throw std::runtime_error(
          std::string("partial-word lane matrix demo memory word was wrong for ") +
          test_case.label);
    }
  }
}

struct PartialDoublewordLoadLaneCase {
  const char* label;
  CpuInstructionWord instruction;
  CpuRegisterValue expected_gpr;
};

struct PartialDoublewordStoreLaneCase {
  const char* label;
  CpuInstructionWord instruction;
  RdramOffset expected_word0_address;
  std::uint32_t expected_word0;
  RdramOffset expected_word1_address;
  std::uint32_t expected_word1;
};

void stage_doubleword_partial_pattern(Machine& machine, RdramOffset address) {
  machine.stage_rdram_u32_be(address, 0x00010203u);
  machine.stage_rdram_u32_be(address + 4u, 0x04050607u);
  machine.stage_rdram_u32_be(address + 8u, 0x08090a0bu);
  machine.stage_rdram_u32_be(address + 12u, 0x0c0d0e0fu);
}

void run_partial_doubleword_lane_demo(Machine& machine) {
  constexpr std::uint8_t kBaseIndex = 4;
  constexpr std::uint8_t kLoadTargetIndex = 18;
  constexpr std::uint8_t kStoreSourceIndex = 19;

  constexpr CpuAddress kInstructionAddress = 0x00000b00u;
  constexpr CpuAddress kPairLdlAddress = 0x00000b10u;
  constexpr CpuAddress kPairLdrAddress = 0x00000b14u;
  constexpr CpuAddress kReverseLdrAddress = 0x00000b20u;
  constexpr CpuAddress kReverseLdlAddress = 0x00000b24u;
  constexpr CpuAddress kStorePairSdlAddress = 0x00000b30u;
  constexpr CpuAddress kStorePairSdrAddress = 0x00000b34u;
  constexpr CpuAddress kStoreReverseSdrAddress = 0x00000b40u;
  constexpr CpuAddress kStoreReverseSdlAddress = 0x00000b44u;
  constexpr CpuAddress kAfterInstructionAddress = kInstructionAddress + 4u;

  constexpr RdramOffset kLoadDataAddress = 0x00000900u;
  constexpr RdramOffset kStoreDataAddress = 0x00000920u;
  constexpr CpuRegisterValue kInitialLoadTarget = 0xaabbccddeeff1122ull;
  constexpr CpuRegisterValue kStoreSource = 0x1122334455667788ull;
  constexpr CpuRegisterValue kExpectedPair = 0x0203040506070809ull;

  const PartialDoublewordLoadLaneCase kLoadCases[] = {
      {
          "LDL offset 2",
          encode_ldl(kLoadTargetIndex, kBaseIndex, 0x0002u),
          0x0203040506071122ull,
      },
      {
          "LDR offset 9",
          encode_ldr(kLoadTargetIndex, kBaseIndex, 0x0009u),
          0xaabbccddeeff0809ull,
      },
      {
          "LDL offset 0 full",
          encode_ldl(kLoadTargetIndex, kBaseIndex, 0x0000u),
          0x0001020304050607ull,
      },
      {
          "LDR offset 7 full",
          encode_ldr(kLoadTargetIndex, kBaseIndex, 0x0007u),
          0x0001020304050607ull,
      },
  };

  const PartialDoublewordStoreLaneCase kStoreCases[] = {
      {
          "SDL offset 2",
          encode_sdl(kStoreSourceIndex, kBaseIndex, 0x0002u),
          kStoreDataAddress,
          0xaabb1122u,
          kStoreDataAddress + 4u,
          0x33445566u,
      },
      {
          "SDR offset 9",
          encode_sdr(kStoreSourceIndex, kBaseIndex, 0x0009u),
          kStoreDataAddress + 8u,
          0x77884455u,
          kStoreDataAddress,
          0xaabbccddu,
      },
  };

  std::cout
      << "fn64 bootstrap partial-doubleword lane demo: LDL/LDR/SDL/SDR deterministic 64-bit byte-lane policy\n";

  for (const PartialDoublewordLoadLaneCase& test_case : kLoadCases) {
    machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kLoadDataAddress));
    machine.stage_cpu_gpr(kLoadTargetIndex, kInitialLoadTarget);
    machine.stage_rdram_u32_be(kInstructionAddress, test_case.instruction);
    stage_doubleword_partial_pattern(machine, kLoadDataAddress);

    std::cout << "load lane row: " << test_case.label << '\n';
    print_control_flow_state(machine);
    print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kLoadTargetIndex));
    print_rdram_word(machine, "  rdram[0x00000900]", kLoadDataAddress);
    print_rdram_word(machine, "  rdram[0x00000904]", kLoadDataAddress + 4u);
    print_rdram_word(machine, "  rdram[0x00000908]", kLoadDataAddress + 8u);

    require_stepped(machine.step_cpu_instruction(), test_case.label);

    print_hex64("  actual_gpr[18]", machine.inspect_cpu_gpr(kLoadTargetIndex));
    print_hex64("  expected_gpr[18]", test_case.expected_gpr);

    if (machine.cpu_pc() != cpu_rdram_alias(kAfterInstructionAddress)) {
      throw std::runtime_error(
          std::string("partial-doubleword lane demo did not advance after ") +
          test_case.label);
    }

    if (machine.inspect_cpu_gpr(kLoadTargetIndex) != test_case.expected_gpr) {
      throw std::runtime_error(
          std::string("partial-doubleword lane demo result was wrong for ") +
          test_case.label);
    }
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kPairLdlAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kLoadDataAddress));
  machine.stage_cpu_gpr(kLoadTargetIndex, kInitialLoadTarget);
  machine.stage_rdram_u32_be(kPairLdlAddress, encode_ldl(kLoadTargetIndex, kBaseIndex, 0x0002u));
  machine.stage_rdram_u32_be(kPairLdrAddress, encode_ldr(kLoadTargetIndex, kBaseIndex, 0x0009u));
  stage_doubleword_partial_pattern(machine, kLoadDataAddress);

  std::cout << "load complementary pair: LDL offset 2 then LDR offset 9\n";
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_ldl_then_ldr_ldl");
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_ldl_then_ldr_ldr");
  print_hex64("  pair_gpr[18]", machine.inspect_cpu_gpr(kLoadTargetIndex));

  if (machine.inspect_cpu_gpr(kLoadTargetIndex) != kExpectedPair) {
    throw std::runtime_error("partial-doubleword demo LDL/LDR pair result was wrong");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kReverseLdrAddress));
  machine.stage_cpu_gpr(kLoadTargetIndex, kInitialLoadTarget);
  machine.stage_rdram_u32_be(kReverseLdrAddress, encode_ldr(kLoadTargetIndex, kBaseIndex, 0x0009u));
  machine.stage_rdram_u32_be(kReverseLdlAddress, encode_ldl(kLoadTargetIndex, kBaseIndex, 0x0002u));

  std::cout << "load complementary pair: LDR offset 9 then LDL offset 2\n";
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_ldr_then_ldl_ldr");
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_ldr_then_ldl_ldl");
  print_hex64("  reverse_pair_gpr[18]", machine.inspect_cpu_gpr(kLoadTargetIndex));

  if (machine.inspect_cpu_gpr(kLoadTargetIndex) != kExpectedPair) {
    throw std::runtime_error("partial-doubleword demo LDR/LDL pair result was wrong");
  }

  for (const PartialDoublewordStoreLaneCase& test_case : kStoreCases) {
    machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kStoreDataAddress));
    machine.stage_cpu_gpr(kStoreSourceIndex, kStoreSource);
    machine.stage_rdram_u32_be(kInstructionAddress, test_case.instruction);
    machine.stage_rdram_u32_be(kStoreDataAddress, 0xaabbccddu);
    machine.stage_rdram_u32_be(kStoreDataAddress + 4u, 0xeeff0011u);
    machine.stage_rdram_u32_be(kStoreDataAddress + 8u, 0x22334455u);

    std::cout << "store lane row: " << test_case.label << '\n';
    print_control_flow_state(machine);
    print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kStoreSourceIndex));
    print_rdram_word(machine, "  rdram[0x00000920]", kStoreDataAddress);
    print_rdram_word(machine, "  rdram[0x00000924]", kStoreDataAddress + 4u);
    print_rdram_word(machine, "  rdram[0x00000928]", kStoreDataAddress + 8u);

    require_stepped(machine.step_cpu_instruction(), test_case.label);

    print_rdram_word(machine, "  actual_word0", test_case.expected_word0_address);
    print_hex32("  expected_word0", test_case.expected_word0);
    print_rdram_word(machine, "  actual_word1", test_case.expected_word1_address);
    print_hex32("  expected_word1", test_case.expected_word1);

    if (machine.cpu_pc() != cpu_rdram_alias(kAfterInstructionAddress)) {
      throw std::runtime_error(
          std::string("partial-doubleword lane demo did not advance after ") +
          test_case.label);
    }

    if (machine.inspect_rdram_u32_be(test_case.expected_word0_address) !=
            test_case.expected_word0 ||
        machine.inspect_rdram_u32_be(test_case.expected_word1_address) !=
            test_case.expected_word1) {
      throw std::runtime_error(
          std::string("partial-doubleword lane demo memory was wrong for ") +
          test_case.label);
    }
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kStorePairSdlAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kStoreDataAddress));
  machine.stage_cpu_gpr(kStoreSourceIndex, kStoreSource);
  machine.stage_rdram_u32_be(
      kStorePairSdlAddress,
      encode_sdl(kStoreSourceIndex, kBaseIndex, 0x0002u));
  machine.stage_rdram_u32_be(
      kStorePairSdrAddress,
      encode_sdr(kStoreSourceIndex, kBaseIndex, 0x0009u));
  machine.stage_rdram_u32_be(kStoreDataAddress, 0x00000000u);
  machine.stage_rdram_u32_be(kStoreDataAddress + 4u, 0x00000000u);
  machine.stage_rdram_u32_be(kStoreDataAddress + 8u, 0x00000000u);

  std::cout << "store complementary pair: SDL offset 2 then SDR offset 9\n";
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_sdl_then_sdr_sdl");
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_sdl_then_sdr_sdr");
  print_rdram_word(machine, "  pair_rdram[0x00000922]", kStoreDataAddress + 2u);
  print_rdram_word(machine, "  pair_rdram[0x00000926]", kStoreDataAddress + 6u);

  if (machine.inspect_rdram_u32_be(kStoreDataAddress + 2u) != 0x11223344u ||
      machine.inspect_rdram_u32_be(kStoreDataAddress + 6u) != 0x55667788u) {
    throw std::runtime_error("partial-doubleword demo SDL/SDR pair did not store full value");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kStoreReverseSdrAddress));
  machine.stage_rdram_u32_be(
      kStoreReverseSdrAddress,
      encode_sdr(kStoreSourceIndex, kBaseIndex, 0x0009u));
  machine.stage_rdram_u32_be(
      kStoreReverseSdlAddress,
      encode_sdl(kStoreSourceIndex, kBaseIndex, 0x0002u));
  machine.stage_rdram_u32_be(kStoreDataAddress, 0x00000000u);
  machine.stage_rdram_u32_be(kStoreDataAddress + 4u, 0x00000000u);
  machine.stage_rdram_u32_be(kStoreDataAddress + 8u, 0x00000000u);

  std::cout << "store complementary pair: SDR offset 9 then SDL offset 2\n";
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_sdr_then_sdl_sdr");
  require_stepped(machine.step_cpu_instruction(), "partial_doubleword_demo_sdr_then_sdl_sdl");
  print_rdram_word(machine, "  reverse_pair_rdram[0x00000922]", kStoreDataAddress + 2u);
  print_rdram_word(machine, "  reverse_pair_rdram[0x00000926]", kStoreDataAddress + 6u);

  if (machine.inspect_rdram_u32_be(kStoreDataAddress + 2u) != 0x11223344u ||
      machine.inspect_rdram_u32_be(kStoreDataAddress + 6u) != 0x55667788u) {
    throw std::runtime_error("partial-doubleword demo SDR/SDL pair did not store full value");
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

  machine.stage_cpu_pc(cpu_rdram_alias(kSwAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0x89abcdefu);
  machine.stage_cpu_gpr(kTargetIndex, 0x00000000u);

  machine.stage_rdram_u32_be(kSwAddress, kSwInstruction);
  machine.stage_rdram_u32_be(kLwAddress, kLwInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataBaseAddress, 0x01020304u);
  machine.stage_rdram_u32_be(kEffectiveAddress, 0x55667788u);

  std::cout << "fn64 bootstrap aligned word demo: explicit local SW/LW base+immediate\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000450]", kDataBaseAddress);
  print_rdram_word(machine, "  rdram[0x00000454]", kEffectiveAddress);

  require_stepped(machine.step_cpu_instruction(), "aligned_word_demo_sw");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000450]", kDataBaseAddress);
  print_rdram_word(machine, "  rdram[0x00000454]", kEffectiveAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kLwAddress)) {
    throw std::runtime_error("aligned word demo did not advance to LW");
  }

  if (machine.inspect_rdram_u32_be(kEffectiveAddress) != 0x89abcdefu) {
    throw std::runtime_error("aligned word demo SW store result was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "aligned_word_demo_lw");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x00000454]", kEffectiveAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("aligned word demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) !=
      cpu_value_from_sign_extended_u32(0x89abcdefu)) {
    throw std::runtime_error("aligned word demo LW load result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "aligned_word_demo_break");
}

void run_unsigned_word_load_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSignedTargetIndex = 12;
  constexpr std::size_t kUnsignedTargetIndex = 13;

  constexpr RdramOffset kLwAddress = 0x00000280u;
  constexpr RdramOffset kLwuAddress = 0x00000284u;
  constexpr RdramOffset kLwuZeroAddress = 0x00000288u;
  constexpr RdramOffset kBreakAddress = 0x0000028cu;

  constexpr RdramOffset kDataAddress = 0x000005a0u;
  constexpr std::uint16_t kOffset = 0x0000u;
  constexpr std::uint32_t kWordWithSignBit = 0xffffffffu;
  constexpr CpuRegisterValue kSignedWordValue = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kUnsignedWordValue = 0x00000000ffffffffull;

  const CpuInstructionWord kLwInstruction = encode_lw(
      static_cast<std::uint8_t>(kSignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const CpuInstructionWord kLwuInstruction = encode_lwu(
      static_cast<std::uint8_t>(kUnsignedTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const CpuInstructionWord kLwuZeroInstruction = encode_lwu(
      0,
      static_cast<std::uint8_t>(kBaseIndex),
      kOffset);
  const CpuInstructionWord kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLwAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
  machine.stage_cpu_gpr(kSignedTargetIndex, 0);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0);
  machine.stage_cpu_gpr(0, 0xffffffffffffffffull);

  machine.stage_rdram_u32_be(kLwAddress, kLwInstruction);
  machine.stage_rdram_u32_be(kLwuAddress, kLwuInstruction);
  machine.stage_rdram_u32_be(kLwuZeroAddress, kLwuZeroInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);
  machine.stage_rdram_u32_be(kDataAddress, kWordWithSignBit);

  std::cout
      << "fn64 bootstrap unsigned word load demo: LW sign-extends and LWU zero-extends the same RDRAM word\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_rdram_word(machine, "  rdram[0x000005a0]", kDataAddress);

  require_stepped(machine.step_cpu_instruction(), "unsigned_word_load_demo_lw");

  std::cout << "after LW step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLwuAddress)) {
    throw std::runtime_error("unsigned word load demo did not advance to LWU");
  }

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) != kSignedWordValue) {
    throw std::runtime_error("unsigned word load demo LW result was not sign-extended");
  }

  require_stepped(machine.step_cpu_instruction(), "unsigned_word_load_demo_lwu");

  std::cout << "after LWU step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLwuZeroAddress)) {
    throw std::runtime_error("unsigned word load demo did not advance to LWU zero row");
  }

  if (machine.inspect_cpu_gpr(kUnsignedTargetIndex) != kUnsignedWordValue) {
    throw std::runtime_error("unsigned word load demo LWU result was not zero-extended");
  }

  require_stepped(machine.step_cpu_instruction(), "unsigned_word_load_demo_lwu_zero");

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("unsigned word load demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(0) != 0) {
    throw std::runtime_error("unsigned word load demo LWU wrote to gpr[0]");
  }

  require_stopped(machine.step_cpu_instruction(), "unsigned_word_load_demo_break");
}

void run_aligned_doubleword_load_store_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kLoadTargetIndex = 5;
  constexpr std::size_t kStoreSourceIndex = 6;

  constexpr CpuAddress kLdAddress = 0x00000220u;
  constexpr CpuAddress kSdAddress = 0x00000224u;
  constexpr CpuAddress kLdZeroAddress = 0x00000228u;
  constexpr CpuAddress kBreakAddress = 0x0000022cu;

  constexpr RdramOffset kLoadDataAddress = 0x00000580u;
  constexpr RdramOffset kStoreDataAddress = 0x00000588u;
  constexpr std::uint16_t kLoadOffset = 0x0000u;
  constexpr std::uint16_t kStoreOffset = 0x0008u;

  constexpr CpuRegisterValue kLoadedValue = 0x1122334455667788ull;
  constexpr CpuRegisterValue kStoredValue = 0xaabbccddeeff0011ull;
  constexpr CpuRegisterValue kLoadTargetSentinel = 0x0102030405060708ull;

  const CpuInstructionWord kLdInstruction = encode_ld(
      static_cast<std::uint8_t>(kLoadTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kLoadOffset);
  const CpuInstructionWord kSdInstruction = encode_sd(
      static_cast<std::uint8_t>(kStoreSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kStoreOffset);
  const CpuInstructionWord kLdZeroInstruction = encode_ld(
      0,
      static_cast<std::uint8_t>(kBaseIndex),
      kStoreOffset);
  const CpuInstructionWord kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLdAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kLoadDataAddress));
  machine.stage_cpu_gpr(kLoadTargetIndex, kLoadTargetSentinel);
  machine.stage_cpu_gpr(kStoreSourceIndex, kStoredValue);
  machine.stage_cpu_gpr(0, 0xffffffffffffffffull);

  machine.stage_rdram_u32_be(kLdAddress, kLdInstruction);
  machine.stage_rdram_u32_be(kSdAddress, kSdInstruction);
  machine.stage_rdram_u32_be(kLdZeroAddress, kLdZeroInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kLoadDataAddress, 0x11223344u);
  machine.stage_rdram_u32_be(kLoadDataAddress + 4u, 0x55667788u);
  machine.stage_rdram_u32_be(kStoreDataAddress, 0x00000000u);
  machine.stage_rdram_u32_be(kStoreDataAddress + 4u, 0x00000000u);

  std::cout
      << "fn64 bootstrap aligned doubleword demo: LD/SD use Machine-owned RDRAM bytes\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kLoadTargetIndex));
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kStoreSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000580]", kLoadDataAddress);
  print_rdram_word(machine, "  rdram[0x00000584]", kLoadDataAddress + 4u);

  require_stepped(machine.step_cpu_instruction(), "aligned_doubleword_demo_ld");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kLoadTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kSdAddress)) {
    throw std::runtime_error("aligned doubleword demo did not advance to SD");
  }

  if (machine.inspect_cpu_gpr(kLoadTargetIndex) != kLoadedValue) {
    throw std::runtime_error("aligned doubleword demo LD result was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "aligned_doubleword_demo_sd");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000588]", kStoreDataAddress);
  print_rdram_word(machine, "  rdram[0x0000058c]", kStoreDataAddress + 4u);

  if (machine.cpu_pc() != cpu_rdram_alias(kLdZeroAddress)) {
    throw std::runtime_error("aligned doubleword demo did not advance to LD $0");
  }

  if (machine.inspect_rdram_u32_be(kStoreDataAddress) != 0xaabbccddu ||
      machine.inspect_rdram_u32_be(kStoreDataAddress + 4u) != 0xeeff0011u) {
    throw std::runtime_error("aligned doubleword demo SD store result was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "aligned_doubleword_demo_ld_zero");

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("aligned doubleword demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(0) != 0) {
    throw std::runtime_error("aligned doubleword demo LD wrote to gpr[0]");
  }

  require_stopped(machine.step_cpu_instruction(), "aligned_doubleword_demo_break");
}

void run_word_alignment_guard_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 11;
  constexpr std::size_t kTargetIndex = 12;

  constexpr std::uint32_t kSwAddress = 0x00000100u;
  constexpr std::uint32_t kLwAddress = 0x00000104u;
  constexpr std::uint32_t kLwuAddress = 0x00000108u;

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
  const std::uint32_t kLwuInstruction = encode_lwu(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);

  machine.stage_rdram_u32_be(kSwAddress, kSwInstruction);
  machine.stage_rdram_u32_be(kLwAddress, kLwInstruction);
  machine.stage_rdram_u32_be(kLwuAddress, kLwuInstruction);

  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0xdeadbeefu);
  machine.stage_cpu_gpr(kTargetIndex, 0x01234567u);
  machine.stage_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap aligned word guard demo: explicit local SW/LW natural-alignment failure\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kSwAddress));

  std::cout << "before SW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  sw_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

  require_step_machine_fault(
      machine,
      "word_alignment_demo_sw",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      4);

  std::cout << "after SW misaligned step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kSwAddress)) {
    throw std::runtime_error("word alignment guard demo SW changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSwAddress + 4u)) {
    throw std::runtime_error("word alignment guard demo SW changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kDataBaseAddress) != 0x11223344u) {
    throw std::runtime_error("word alignment guard demo SW changed memory on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLwAddress));
  machine.stage_cpu_gpr(kTargetIndex, 0x01234567u);

  std::cout << "before LW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  lw_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x00000470]", kDataBaseAddress);

  require_step_machine_fault(
      machine,
      "word_alignment_demo_lw",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      4);

  std::cout << "after LW misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLwAddress)) {
    throw std::runtime_error("word alignment guard demo LW changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLwAddress + 4u)) {
    throw std::runtime_error("word alignment guard demo LW changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != 0x01234567u) {
    throw std::runtime_error("word alignment guard demo LW changed target register on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLwuAddress));
  machine.stage_cpu_gpr(kTargetIndex, 0x89abcdef01234567ull);

  std::cout << "before LWU misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  lwu_effective_address", kMisalignedAddress);

  require_step_machine_fault(
      machine,
      "word_alignment_demo_lwu",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      4);

  std::cout << "after LWU misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLwuAddress)) {
    throw std::runtime_error("word alignment guard demo LWU changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLwuAddress + 4u)) {
    throw std::runtime_error("word alignment guard demo LWU changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != 0x89abcdef01234567ull) {
    throw std::runtime_error("word alignment guard demo LWU changed target register on fault");
  }
}

void run_doubleword_alignment_guard_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 11;
  constexpr std::size_t kTargetIndex = 12;

  constexpr CpuAddress kSdAddress = 0x00000240u;
  constexpr CpuAddress kLdAddress = 0x00000244u;

  constexpr RdramOffset kDataBaseAddress = 0x00000580u;
  constexpr std::uint16_t kMisalignedOffset = 0x0004u;
  constexpr CpuAddress kMisalignedAddress =
      cpu_rdram_alias(kDataBaseAddress) + kMisalignedOffset;

  constexpr CpuRegisterValue kSourceValue = 0xaabbccddeeff0011ull;
  constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;

  const CpuInstructionWord kSdInstruction = encode_sd(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);
  const CpuInstructionWord kLdInstruction = encode_ld(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      kMisalignedOffset);

  machine.stage_rdram_u32_be(kSdAddress, kSdInstruction);
  machine.stage_rdram_u32_be(kLdAddress, kLdInstruction);
  machine.stage_rdram_u32_be(kDataBaseAddress, 0x10203040u);
  machine.stage_rdram_u32_be(kDataBaseAddress + 4u, 0x50607080u);
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, kSourceValue);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout
      << "fn64 bootstrap doubleword guard demo: LD/SD natural-alignment failure is local MachineFault\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kSdAddress));

  std::cout << "before SD misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  sd_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x00000580]", kDataBaseAddress);
  print_rdram_word(machine, "  rdram[0x00000584]", kDataBaseAddress + 4u);

  require_step_machine_fault(
      machine,
      "doubleword_alignment_demo_sd",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      8);

  std::cout << "after SD misaligned step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000580]", kDataBaseAddress);
  print_rdram_word(machine, "  rdram[0x00000584]", kDataBaseAddress + 4u);

  if (machine.cpu_pc() != cpu_rdram_alias(kSdAddress)) {
    throw std::runtime_error("doubleword alignment guard demo SD changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSdAddress + 4u)) {
    throw std::runtime_error("doubleword alignment guard demo SD changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kDataBaseAddress) != 0x10203040u ||
      machine.inspect_rdram_u32_be(kDataBaseAddress + 4u) != 0x50607080u) {
    throw std::runtime_error("doubleword alignment guard demo SD changed memory on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLdAddress));
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout << "before LD misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  ld_effective_address", kMisalignedAddress);

  require_step_machine_fault(
      machine,
      "doubleword_alignment_demo_ld",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      8);

  std::cout << "after LD misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLdAddress)) {
    throw std::runtime_error("doubleword alignment guard demo LD changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLdAddress + 4u)) {
    throw std::runtime_error("doubleword alignment guard demo LD changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel) {
    throw std::runtime_error("doubleword alignment guard demo LD changed target register on fault");
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

  machine.stage_cpu_pc(cpu_rdram_alias(kSbAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0x12345680u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.stage_rdram_u32_be(kSbAddress, kSbInstruction);
  machine.stage_rdram_u32_be(kLbAddress, kLbInstruction);
  machine.stage_rdram_u32_be(kLbuAddress, kLbuInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap byte demo: explicit local SB/LB/LBU shaping and extension\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000490]", kDataBaseAddress);

  require_stepped(machine.step_cpu_instruction(), "byte_demo_sb");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000490]", kDataBaseAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kLbAddress)) {
    throw std::runtime_error("byte demo did not advance to LB");
  }

  if (machine.inspect_rdram_u32_be(kDataBaseAddress) != 0x11803344u) {
    throw std::runtime_error("byte demo SB shaping was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "byte_demo_lb");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[14]", machine.inspect_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLbuAddress)) {
    throw std::runtime_error("byte demo did not advance to LBU");
  }

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) !=
      cpu_value_from_sign_extended_u32(0xffffff80u)) {
    throw std::runtime_error("byte demo LB sign-extension result was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "byte_demo_lbu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("byte demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kUnsignedTargetIndex) != 0x00000080u) {
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

  machine.stage_cpu_pc(cpu_rdram_alias(kShAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.stage_rdram_u32_be(kShAddress, kShInstruction);
  machine.stage_rdram_u32_be(kLhAddress, kLhInstruction);
  machine.stage_rdram_u32_be(kLhuAddress, kLhuInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap halfword demo: explicit local SH/LH/LHU shaping and extension\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[16]", machine.inspect_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x000004b0]", kDataBaseAddress);

  require_stepped(machine.step_cpu_instruction(), "halfword_demo_sh");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x000004b0]", kDataBaseAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kLhAddress)) {
    throw std::runtime_error("halfword demo did not advance to LH");
  }

  if (machine.inspect_rdram_u32_be(kDataBaseAddress) != 0x11228001u) {
    throw std::runtime_error("halfword demo SH shaping was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "halfword_demo_lh");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[17]", machine.inspect_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLhuAddress)) {
    throw std::runtime_error("halfword demo did not advance to LHU");
  }

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) !=
      cpu_value_from_sign_extended_u32(0xffff8001u)) {
    throw std::runtime_error("halfword demo LH sign-extension result was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "halfword_demo_lhu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("halfword demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kUnsignedTargetIndex) != 0x00008001u) {
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

  machine.stage_rdram_u32_be(kShAddress, kShInstruction);
  machine.stage_rdram_u32_be(kLhAddress, kLhInstruction);
  machine.stage_rdram_u32_be(kLhuAddress, kLhuInstruction);

  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);
  machine.stage_rdram_u32_be(kDataBaseAddress, 0x11223344u);

  std::cout << "fn64 bootstrap halfword guard demo: explicit local SH/LH/LHU natural-alignment failure\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kShAddress));

  std::cout << "before SH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[16]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  sh_effective_address", kMisalignedAddress);
  print_rdram_word(machine, "  rdram[0x000004d0]", kDataBaseAddress);

  require_step_machine_fault(
      machine,
      "halfword_alignment_demo_sh",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      2);

  std::cout << "after SH misaligned step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x000004d0]", kDataBaseAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kShAddress)) {
    throw std::runtime_error("halfword alignment guard demo SH changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kShAddress + 4u)) {
    throw std::runtime_error("halfword alignment guard demo SH changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kDataBaseAddress) != 0x11223344u) {
    throw std::runtime_error("halfword alignment guard demo SH changed memory on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLhAddress));
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);

  std::cout << "before LH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[17]", machine.inspect_cpu_gpr(kSignedTargetIndex));
  print_hex32("  lh_effective_address", kMisalignedAddress);

  require_step_machine_fault(
      machine,
      "halfword_alignment_demo_lh",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      2);

  std::cout << "after LH misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[17]", machine.inspect_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLhAddress)) {
    throw std::runtime_error("halfword alignment guard demo LH changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLhAddress + 4u)) {
    throw std::runtime_error("halfword alignment guard demo LH changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) != 0xaaaaaaaau) {
    throw std::runtime_error("halfword alignment guard demo LH changed target register on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLhuAddress));
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  std::cout << "before LHU misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kUnsignedTargetIndex));
  print_hex32("  lhu_effective_address", kMisalignedAddress);

  require_step_machine_fault(
      machine,
      "halfword_alignment_demo_lhu",
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      2);

  std::cout << "after LHU misaligned step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLhuAddress)) {
    throw std::runtime_error("halfword alignment guard demo LHU changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLhuAddress + 4u)) {
    throw std::runtime_error("halfword alignment guard demo LHU changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kUnsignedTargetIndex) != 0xbbbbbbbbu) {
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

  machine.stage_cpu_pc(cpu_rdram_alias(kSwAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0x76543210u);
  machine.stage_cpu_gpr(kTargetIndex, 0x00000000u);

  machine.stage_rdram_u32_be(kSwAddress, kSwInstruction);
  machine.stage_rdram_u32_be(kLwAddress, kLwInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kEffectiveAddress, 0x01020304u);
  machine.stage_rdram_u32_be(kBaseAddress, 0x55667788u);

  std::cout << "fn64 bootstrap negative-offset word demo: explicit local SW/LW signed immediate address formation\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex32("  sw_immediate_raw", kNegativeOffset);
  print_hex32("  sw_effective_address", kEffectiveAddress);
  print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[20]", machine.inspect_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x000004f0]", kEffectiveAddress);
  print_rdram_word(machine, "  rdram[0x000004f4]", kBaseAddress);

  require_stepped(machine.step_cpu_instruction(), "negative_word_demo_sw");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x000004f0]", kEffectiveAddress);
  print_rdram_word(machine, "  rdram[0x000004f4]", kBaseAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kLwAddress)) {
    throw std::runtime_error("negative-offset word demo did not advance to LW");
  }

  if (machine.inspect_rdram_u32_be(kEffectiveAddress) != 0x76543210u) {
    throw std::runtime_error("negative-offset word demo SW store result was wrong");
  }

  if (machine.inspect_rdram_u32_be(kBaseAddress) != 0x55667788u) {
    throw std::runtime_error("negative-offset word demo touched the base word unexpectedly");
  }

  require_stepped(machine.step_cpu_instruction(), "negative_word_demo_lw");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[20]", machine.inspect_cpu_gpr(kTargetIndex));
  print_rdram_word(machine, "  rdram[0x000004f0]", kEffectiveAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("negative-offset word demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != 0x76543210u) {
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

  machine.stage_cpu_pc(cpu_rdram_alias(kSbAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0x12345680u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.stage_rdram_u32_be(kSbAddress, kSbInstruction);
  machine.stage_rdram_u32_be(kLbAddress, kLbInstruction);
  machine.stage_rdram_u32_be(kLbuAddress, kLbuInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataWordAddress, 0x11223344u);

  std::cout << "fn64 bootstrap negative-offset byte demo: explicit local SB/LB/LBU signed immediate address formation\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex32("  sb_immediate_raw", kNegativeOffset);
  print_hex32("  sb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[21]", machine.inspect_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000510]", kDataWordAddress);

  require_stepped(machine.step_cpu_instruction(), "negative_byte_demo_sb");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000510]", kDataWordAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kLbAddress)) {
    throw std::runtime_error("negative-offset byte demo did not advance to LB");
  }

  if (machine.inspect_rdram_u32_be(kDataWordAddress) != 0x11803344u) {
    throw std::runtime_error("negative-offset byte demo SB shaping was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "negative_byte_demo_lb");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[22]", machine.inspect_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLbuAddress)) {
    throw std::runtime_error("negative-offset byte demo did not advance to LBU");
  }

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) !=
      cpu_value_from_sign_extended_u32(0xffffff80u)) {
    throw std::runtime_error("negative-offset byte demo LB sign-extension result was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "negative_byte_demo_lbu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[23]", machine.inspect_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("negative-offset byte demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kUnsignedTargetIndex) != 0x00000080u) {
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

  machine.stage_cpu_pc(cpu_rdram_alias(kShAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kBaseAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0xabcd8001u);
  machine.stage_cpu_gpr(kSignedTargetIndex, 0xaaaaaaaau);
  machine.stage_cpu_gpr(kUnsignedTargetIndex, 0xbbbbbbbbu);

  machine.stage_rdram_u32_be(kShAddress, kShInstruction);
  machine.stage_rdram_u32_be(kLhAddress, kLhInstruction);
  machine.stage_rdram_u32_be(kLhuAddress, kLhuInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataWordAddress, 0x11223344u);

  std::cout << "fn64 bootstrap negative-offset halfword demo: explicit local SH/LH/LHU signed immediate address formation\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex32("  sh_immediate_raw", kNegativeOffset);
  print_hex32("  sh_effective_address", kEffectiveAddress);
  print_hex64("  gpr[24]", machine.inspect_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000530]", kDataWordAddress);

  require_stepped(machine.step_cpu_instruction(), "negative_halfword_demo_sh");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000530]", kDataWordAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kLhAddress)) {
    throw std::runtime_error("negative-offset halfword demo did not advance to LH");
  }

  if (machine.inspect_rdram_u32_be(kDataWordAddress) != 0x11228001u) {
    throw std::runtime_error("negative-offset halfword demo SH shaping was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "negative_halfword_demo_lh");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[25]", machine.inspect_cpu_gpr(kSignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLhuAddress)) {
    throw std::runtime_error("negative-offset halfword demo did not advance to LHU");
  }

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) !=
      cpu_value_from_sign_extended_u32(0xffff8001u)) {
    throw std::runtime_error("negative-offset halfword demo LH sign-extension result was wrong");
  }

  require_stepped(machine.step_cpu_instruction(), "negative_halfword_demo_lhu");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[26]", machine.inspect_cpu_gpr(kUnsignedTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("negative-offset halfword demo did not advance to BREAK sentinel");
  }

  if (machine.inspect_cpu_gpr(kUnsignedTargetIndex) != 0x00008001u) {
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
  constexpr CpuRegisterValue kTargetSentinel = 0x1122334489abcdefull;

  const std::uint32_t kLwlInstruction = encode_lwl(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const std::uint32_t kLwrInstruction = encode_lwr(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.stage_rdram_u32_be(kLwlAddress, kLwlInstruction);
  machine.stage_rdram_u32_be(kLwrAddress, kLwrInstruction);
  machine.stage_cpu_gpr(kBaseIndex, kInvalidKseg1Address);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout
      << "fn64 bootstrap failed partial-load no-ghost demo: LWL/LWR faults do not write target GPR or advance control state\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kLwlAddress));

  std::cout << "before LWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[30]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  lwl_effective_address", kInvalidKseg1Address);

  require_step_machine_fault(
      machine,
      "failed_partial_load_demo_lwl",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after LWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[30]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLwlAddress)) {
    throw std::runtime_error("failed partial-load no-ghost demo LWL changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLwlAddress + 4u)) {
    throw std::runtime_error("failed partial-load no-ghost demo LWL changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel) {
    throw std::runtime_error("failed partial-load no-ghost demo LWL changed target GPR on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLwrAddress));
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout << "before LWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[30]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  lwr_effective_address", kInvalidKseg1Address);

  require_step_machine_fault(
      machine,
      "failed_partial_load_demo_lwr",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after LWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[30]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLwrAddress)) {
    throw std::runtime_error("failed partial-load no-ghost demo LWR changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLwrAddress + 4u)) {
    throw std::runtime_error("failed partial-load no-ghost demo LWR changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel) {
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

  machine.stage_rdram_u32_be(kSwlAddress, kSwlInstruction);
  machine.stage_rdram_u32_be(kSwrAddress, kSwrInstruction);
  machine.stage_rdram_u32_be(kLowSentinelAddress, kLowSentinel);
  machine.stage_rdram_u32_be(kTailSentinelAddress, kTailSentinel);
  machine.stage_cpu_gpr(kBaseIndex, kInvalidKseg1Address);
  machine.stage_cpu_gpr(kSourceIndex, 0xaabbccdda1b2c3d4ull);

  std::cout
      << "fn64 bootstrap failed partial-store no-ghost demo: SWL/SWR faults do not mutate RDRAM or advance control state\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kSwlAddress));

  std::cout << "before SWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[29]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  swl_effective_address", kInvalidKseg1Address);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  require_step_machine_fault(
      machine,
      "failed_partial_store_demo_swl",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after SWL out-of-window step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kSwlAddress)) {
    throw std::runtime_error("failed partial-store no-ghost demo SWL changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSwlAddress + 4u)) {
    throw std::runtime_error("failed partial-store no-ghost demo SWL changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kLowSentinelAddress) != kLowSentinel ||
      machine.inspect_rdram_u32_be(kTailSentinelAddress) != kTailSentinel) {
    throw std::runtime_error("failed partial-store no-ghost demo SWL changed RDRAM on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kSwrAddress));

  std::cout << "before SWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[29]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  swr_effective_address", kInvalidKseg1Address);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  require_step_machine_fault(
      machine,
      "failed_partial_store_demo_swr",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after SWR out-of-window step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000570]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kSwrAddress)) {
    throw std::runtime_error("failed partial-store no-ghost demo SWR changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSwrAddress + 4u)) {
    throw std::runtime_error("failed partial-store no-ghost demo SWR changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kLowSentinelAddress) != kLowSentinel ||
      machine.inspect_rdram_u32_be(kTailSentinelAddress) != kTailSentinel) {
    throw std::runtime_error("failed partial-store no-ghost demo SWR changed RDRAM on fault");
  }
}

void run_failed_partial_doubleword_no_ghost_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 28;
  constexpr std::size_t kSourceIndex = 29;

  constexpr CpuAddress kLdlAddress = 0x000002d0u;
  constexpr CpuAddress kLdrAddress = 0x000002d4u;
  constexpr CpuAddress kSdlAddress = 0x000002d8u;
  constexpr CpuAddress kSdrAddress = 0x000002dcu;

  constexpr CpuAddress kInvalidKseg1Address = 0xa0400000u;
  constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;
  constexpr CpuRegisterValue kSourceValue = 0xaabbccddeeff0011ull;
  constexpr RdramOffset kLowSentinelAddress = 0x00000960u;
  constexpr RdramOffset kTailSentinelAddress = 0x003ffffcu;
  constexpr std::uint32_t kLowSentinel = 0x10203040u;
  constexpr std::uint32_t kTailSentinel = 0x50607080u;

  const CpuInstructionWord kLdlInstruction = encode_ldl(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const CpuInstructionWord kLdrInstruction = encode_ldr(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const CpuInstructionWord kSdlInstruction = encode_sdl(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const CpuInstructionWord kSdrInstruction = encode_sdr(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.stage_rdram_u32_be(kLdlAddress, kLdlInstruction);
  machine.stage_rdram_u32_be(kLdrAddress, kLdrInstruction);
  machine.stage_rdram_u32_be(kSdlAddress, kSdlInstruction);
  machine.stage_rdram_u32_be(kSdrAddress, kSdrInstruction);
  machine.stage_rdram_u32_be(kLowSentinelAddress, kLowSentinel);
  machine.stage_rdram_u32_be(kTailSentinelAddress, kTailSentinel);
  machine.stage_cpu_gpr(kBaseIndex, kInvalidKseg1Address);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);
  machine.stage_cpu_gpr(kSourceIndex, kSourceValue);

  std::cout
      << "fn64 bootstrap failed partial-doubleword no-ghost demo: LDL/LDR/SDL/SDR faults preserve visible state\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kLdlAddress));

  std::cout << "before LDL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  ldl_effective_address", kInvalidKseg1Address);

  require_step_machine_fault(
      machine,
      "failed_partial_doubleword_demo_ldl",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after LDL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLdlAddress)) {
    throw std::runtime_error("failed partial-doubleword no-ghost demo LDL changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLdlAddress + 4u)) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo LDL changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo LDL changed target GPR on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLdrAddress));
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout << "before LDR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  ldr_effective_address", kInvalidKseg1Address);

  require_step_machine_fault(
      machine,
      "failed_partial_doubleword_demo_ldr",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after LDR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLdrAddress)) {
    throw std::runtime_error("failed partial-doubleword no-ghost demo LDR changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLdrAddress + 4u)) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo LDR changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo LDR changed target GPR on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kSdlAddress));

  std::cout << "before SDL out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[29]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  sdl_effective_address", kInvalidKseg1Address);
  print_rdram_word(machine, "  rdram[0x00000960]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  require_step_machine_fault(
      machine,
      "failed_partial_doubleword_demo_sdl",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after SDL out-of-window step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000960]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kSdlAddress)) {
    throw std::runtime_error("failed partial-doubleword no-ghost demo SDL changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSdlAddress + 4u)) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo SDL changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kLowSentinelAddress) != kLowSentinel ||
      machine.inspect_rdram_u32_be(kTailSentinelAddress) != kTailSentinel) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo SDL changed RDRAM on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kSdrAddress));

  std::cout << "before SDR out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[29]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  sdr_effective_address", kInvalidKseg1Address);
  print_rdram_word(machine, "  rdram[0x00000960]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  require_step_machine_fault(
      machine,
      "failed_partial_doubleword_demo_sdr",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after SDR out-of-window step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000960]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kSdrAddress)) {
    throw std::runtime_error("failed partial-doubleword no-ghost demo SDR changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSdrAddress + 4u)) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo SDR changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kLowSentinelAddress) != kLowSentinel ||
      machine.inspect_rdram_u32_be(kTailSentinelAddress) != kTailSentinel) {
    throw std::runtime_error(
        "failed partial-doubleword no-ghost demo SDR changed RDRAM on fault");
  }
}

void run_failed_unsigned_word_load_no_ghost_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kTargetIndex = 28;

  constexpr RdramOffset kLwuAddress = 0x000002a0u;
  constexpr CpuAddress kInvalidKseg1Address = 0xa0400000u;
  constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;
  constexpr CpuRegisterValue kBaseSentinel = kInvalidKseg1Address;
  constexpr CpuRegisterValue kHiSentinel = 0x0102030405060708ull;
  constexpr CpuRegisterValue kLoSentinel = 0x8877665544332211ull;

  const CpuInstructionWord kLwuInstruction = encode_lwu(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.stage_rdram_u32_be(kLwuAddress, kLwuInstruction);
  machine.stage_cpu_gpr(kBaseIndex, kBaseSentinel);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);
  machine.stage_cpu_hi(kHiSentinel);
  machine.stage_cpu_lo(kLoSentinel);

  std::cout
      << "fn64 bootstrap failed unsigned word load no-ghost demo: LWU out-of-window fault does not mutate CPU state\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kLwuAddress));

  std::cout << "before LWU out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_hex32("  lwu_effective_address", kInvalidKseg1Address);

  require_step_machine_fault(
      machine,
      "failed_unsigned_word_load_demo_lwu",
      MachineFaultKind::kCpuRdramAddressRejected,
      4);

  std::cout << "after LWU out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());

  if (machine.cpu_pc() != cpu_rdram_alias(kLwuAddress)) {
    throw std::runtime_error("failed unsigned word load no-ghost demo LWU changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLwuAddress + 4u)) {
    throw std::runtime_error("failed unsigned word load no-ghost demo LWU changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kBaseIndex) != kBaseSentinel ||
      machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel ||
      machine.inspect_cpu_hi() != kHiSentinel ||
      machine.inspect_cpu_lo() != kLoSentinel) {
    throw std::runtime_error("failed unsigned word load no-ghost demo LWU changed CPU state");
  }
}

void run_failed_doubleword_no_ghost_demo(Machine& machine) {
  constexpr std::size_t kBaseIndex = 4;
  constexpr std::size_t kSourceIndex = 27;
  constexpr std::size_t kTargetIndex = 28;

  constexpr CpuAddress kSdAddress = 0x00000260u;
  constexpr CpuAddress kLdAddress = 0x00000264u;

  constexpr CpuAddress kInvalidKseg1Address = 0xa0400000u;
  constexpr RdramOffset kLowSentinelAddress = 0x00000590u;
  constexpr RdramOffset kTailSentinelAddress = 0x003ffff8u;
  constexpr std::uint32_t kLowSentinelHigh = 0x11223344u;
  constexpr std::uint32_t kLowSentinelLow = 0x55667788u;
  constexpr std::uint32_t kTailSentinelHigh = 0x99aabbccu;
  constexpr std::uint32_t kTailSentinelLow = 0xddeeff00u;
  constexpr CpuRegisterValue kSourceValue = 0xaabbccddeeff0011ull;
  constexpr CpuRegisterValue kTargetSentinel = 0x0102030405060708ull;

  const CpuInstructionWord kSdInstruction = encode_sd(
      static_cast<std::uint8_t>(kSourceIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);
  const CpuInstructionWord kLdInstruction = encode_ld(
      static_cast<std::uint8_t>(kTargetIndex),
      static_cast<std::uint8_t>(kBaseIndex),
      0x0000u);

  machine.stage_rdram_u32_be(kSdAddress, kSdInstruction);
  machine.stage_rdram_u32_be(kLdAddress, kLdInstruction);
  machine.stage_rdram_u32_be(kLowSentinelAddress, kLowSentinelHigh);
  machine.stage_rdram_u32_be(kLowSentinelAddress + 4u, kLowSentinelLow);
  machine.stage_rdram_u32_be(kTailSentinelAddress, kTailSentinelHigh);
  machine.stage_rdram_u32_be(kTailSentinelAddress + 4u, kTailSentinelLow);
  machine.stage_cpu_gpr(kBaseIndex, kInvalidKseg1Address);
  machine.stage_cpu_gpr(kSourceIndex, kSourceValue);
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout
      << "fn64 bootstrap failed doubleword no-ghost demo: LD/SD out-of-window faults do not mutate state\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kSdAddress));

  std::cout << "before SD out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[27]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  sd_effective_address", kInvalidKseg1Address);
  print_rdram_word(machine, "  rdram[0x00000590]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x00000594]", kLowSentinelAddress + 4u);
  print_rdram_word(machine, "  rdram[0x003ffff8]", kTailSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress + 4u);

  require_step_machine_fault(
      machine,
      "failed_doubleword_demo_sd",
      MachineFaultKind::kCpuRdramAddressRejected,
      8);

  std::cout << "after SD out-of-window step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000590]", kLowSentinelAddress);
  print_rdram_word(machine, "  rdram[0x00000594]", kLowSentinelAddress + 4u);
  print_rdram_word(machine, "  rdram[0x003ffff8]", kTailSentinelAddress);
  print_rdram_word(machine, "  rdram[0x003ffffc]", kTailSentinelAddress + 4u);

  if (machine.cpu_pc() != cpu_rdram_alias(kSdAddress)) {
    throw std::runtime_error("failed doubleword no-ghost demo SD changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSdAddress + 4u)) {
    throw std::runtime_error("failed doubleword no-ghost demo SD changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kLowSentinelAddress) != kLowSentinelHigh ||
      machine.inspect_rdram_u32_be(kLowSentinelAddress + 4u) != kLowSentinelLow ||
      machine.inspect_rdram_u32_be(kTailSentinelAddress) != kTailSentinelHigh ||
      machine.inspect_rdram_u32_be(kTailSentinelAddress + 4u) != kTailSentinelLow) {
    throw std::runtime_error("failed doubleword no-ghost demo SD changed RDRAM on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLdAddress));
  machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

  std::cout << "before LD out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));
  print_hex32("  ld_effective_address", kInvalidKseg1Address);

  require_step_machine_fault(
      machine,
      "failed_doubleword_demo_ld",
      MachineFaultKind::kCpuRdramAddressRejected,
      8);

  std::cout << "after LD out-of-window step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLdAddress)) {
    throw std::runtime_error("failed doubleword no-ghost demo LD changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLdAddress + 4u)) {
    throw std::runtime_error("failed doubleword no-ghost demo LD changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != kTargetSentinel) {
    throw std::runtime_error("failed doubleword no-ghost demo LD changed target register");
  }
}

void stage_rdram_u64_be(Machine& machine, RdramOffset address, CpuRegisterValue value) {
  machine.stage_rdram_u32_be(address, static_cast<std::uint32_t>(value >> 32));
  machine.stage_rdram_u32_be(address + 4u, static_cast<std::uint32_t>(value));
}

CpuRegisterValue inspect_rdram_u64_be(const Machine& machine, RdramOffset address) {
  return (static_cast<CpuRegisterValue>(machine.inspect_rdram_u32_be(address)) << 32) |
         static_cast<CpuRegisterValue>(machine.inspect_rdram_u32_be(address + 4u));
}

void stage_atomic_instruction(
    Machine& machine,
    RdramOffset address,
    CpuInstructionWord instruction) {
  machine.stage_rdram_u32_be(address, instruction);
}

void step_at(
    Machine& machine,
    RdramOffset address,
    const char* label) {
  machine.stage_cpu_pc(cpu_rdram_alias(address));
  require_stepped(machine.step_cpu_instruction(), label);
}

void require_rdram_word_equals(
    const Machine& machine,
    RdramOffset address,
    std::uint32_t expected,
    const char* label) {
  if (machine.inspect_rdram_u32_be(address) != expected) {
    throw std::runtime_error(std::string(label) + " RDRAM word mismatch");
  }
}

void require_rdram_doubleword_equals(
    const Machine& machine,
    RdramOffset address,
    CpuRegisterValue expected,
    const char* label) {
  if (inspect_rdram_u64_be(machine, address) != expected) {
    throw std::runtime_error(std::string(label) + " RDRAM doubleword mismatch");
  }
}

void require_gpr_equals(
    const Machine& machine,
    std::size_t index,
    CpuRegisterValue expected,
    const char* label) {
  if (machine.inspect_cpu_gpr(index) != expected) {
    throw std::runtime_error(std::string(label) + " GPR mismatch");
  }
}

constexpr CpuAddress kSyntheticPiMmioCpuBase = 0xa4600000u;
constexpr PiCartAddress kSyntheticPiCartRomBase = 0x10000000u;
constexpr std::uint16_t kPiDramRegisterOffset = 0x0000u;
constexpr std::uint16_t kPiCartRegisterOffset = 0x0004u;
constexpr std::uint16_t kPiCartToRdramLengthRegisterOffset = 0x000cu;
constexpr std::uint16_t kPiStatusRegisterOffset = 0x0010u;
constexpr std::uint16_t kPiUnknownRegisterOffset = 0x0014u;

constexpr PiCartAddress pi_cart_rom_address(CartridgeOffset cartridge_offset) {
  return kSyntheticPiCartRomBase + cartridge_offset;
}

constexpr CpuAddress kSyntheticSpDmemKseg0Base = 0x84000000u;
constexpr CpuAddress kSyntheticSpDmemKseg1Base = 0xa4000000u;
constexpr CpuAddress kSyntheticSpImemKseg0Base = 0x84001000u;
constexpr CpuAddress kSyntheticSpImemKseg1Base = 0xa4001000u;
constexpr CpuAddress kSyntheticSpMmioCpuBase = 0xa4040000u;
constexpr std::uint16_t kSpMemoryRegisterOffset = 0x0000u;
constexpr std::uint16_t kSpDramRegisterOffset = 0x0004u;
constexpr std::uint16_t kSpReadLengthRegisterOffset = 0x0008u;
constexpr std::uint16_t kSpWriteLengthRegisterOffset = 0x000cu;
constexpr std::uint16_t kSpStatusRegisterOffset = 0x0010u;
constexpr std::uint16_t kSpUnknownRegisterOffset = 0x0014u;
constexpr CpuAddress kSyntheticSpOutsideRegisterWindow = 0xa4040020u;

constexpr CpuAddress sp_dmem_cached_alias(std::uint16_t offset) {
  return kSyntheticSpDmemKseg0Base + offset;
}

constexpr CpuAddress sp_dmem_uncached_alias(std::uint16_t offset) {
  return kSyntheticSpDmemKseg1Base + offset;
}

constexpr CpuAddress sp_imem_cached_alias(std::uint16_t offset) {
  return kSyntheticSpImemKseg0Base + offset;
}

constexpr CpuAddress sp_imem_uncached_alias(std::uint16_t offset) {
  return kSyntheticSpImemKseg1Base + offset;
}

constexpr std::uint32_t encode_sp_dma_length_command(
    std::uint32_t length,
    std::uint32_t count,
    std::uint32_t skip) {
  return (length & 0x00000fffu) |
         ((count & 0x000000ffu) << 12) |
         ((skip & 0x00000fffu) << 20);
}

std::unique_ptr<Machine> make_pi_dma_proof_machine() {
  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(make_synthetic_normalized_rom_proof_image(), cartridge, error)) {
    throw std::runtime_error("PI DMA proof could not load synthetic cartridge: " + error);
  }

  return std::make_unique<Machine>(std::move(cartridge));
}

void require_step_out_of_range(Machine& machine, const char* label) {
  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::out_of_range& e) {
    std::cout << "  " << label << " threw: " << e.what() << '\n';
    return;
  } catch (const std::exception& e) {
    throw std::runtime_error(
        std::string(label) + " threw unexpected exception type: " + e.what());
  }

  throw std::runtime_error(std::string(label) + " did not throw out_of_range");
}

void stage_pi_sw_instruction(
    Machine& machine,
    RdramOffset instruction_address,
    std::uint8_t source_register,
    std::uint8_t base_register,
    std::uint16_t pi_register_offset) {
  machine.stage_rdram_u32_be(
      instruction_address,
      encode_sw(source_register, base_register, pi_register_offset));
}

void stage_pi_lw_instruction(
    Machine& machine,
    RdramOffset instruction_address,
    std::uint8_t target_register,
    std::uint8_t base_register,
    std::uint16_t pi_register_offset) {
  machine.stage_rdram_u32_be(
      instruction_address,
      encode_lw(target_register, base_register, pi_register_offset));
}

void stage_sp_sw_instruction(
    Machine& machine,
    RdramOffset instruction_address,
    std::uint8_t source_register,
    std::uint8_t base_register,
    std::uint16_t sp_register_offset) {
  machine.stage_rdram_u32_be(
      instruction_address,
      encode_sw(source_register, base_register, sp_register_offset));
}

void stage_sp_lw_instruction(
    Machine& machine,
    RdramOffset instruction_address,
    std::uint8_t target_register,
    std::uint8_t base_register,
    std::uint16_t sp_register_offset) {
  machine.stage_rdram_u32_be(
      instruction_address,
      encode_lw(target_register, base_register, sp_register_offset));
}

void run_cpu_driven_pi_dma_execution_demo() {
  std::cout << "fn64 bootstrap PI MMIO demo: CPU loader drives PI DMA then executes copied RDRAM code\n";

  constexpr std::size_t kPiBaseIndex = 4;
  constexpr std::size_t kValueIndex = 5;
  constexpr std::size_t kDramReadIndex = 6;
  constexpr std::size_t kCartReadIndex = 7;
  constexpr std::size_t kLengthReadIndex = 8;
  constexpr std::size_t kTransferredResultIndex = 9;
  constexpr std::size_t kFaultBaseIndex = 10;
  constexpr std::size_t kFaultTargetIndex = 11;

  constexpr RdramOffset kLoaderBase = 0x00000000u;
  constexpr RdramOffset kTransferredProgramAddress = 0x00000200u;
  constexpr CartridgeOffset kTransferredCartridgeOffset = 0x00000040u;
  constexpr PiCartAddress kTransferredPiCartAddress =
      pi_cart_rom_address(kTransferredCartridgeOffset);
  constexpr CpuAddress kTransferredCpuAddress =
      cpu_rdram_alias(kTransferredProgramAddress);
  constexpr std::uint32_t kLengthRegisterValue = 7u;
  constexpr CpuInstructionWord kTransferredOri =
      encode_ori(static_cast<std::uint8_t>(kTransferredResultIndex), 0, 0x5a5au);
  constexpr CpuInstructionWord kTransferredBreak = encode_break();
  constexpr CpuInstructionWord kNop = 0x00000000u;

  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(
          make_bootstrap_cartridge_staging_rom(kTransferredOri, kTransferredBreak),
          cartridge,
          error)) {
    throw std::runtime_error("CPU-driven PI DMA proof could not load synthetic cartridge: " + error);
  }

  auto machine_storage = std::make_unique<Machine>(std::move(cartridge));
  Machine& machine = *machine_storage;

  constexpr RdramOffset kLuiPiBaseAddress = kLoaderBase + 0x00u;
  constexpr RdramOffset kOriDramAddress = kLoaderBase + 0x04u;
  constexpr RdramOffset kSwDramAddress = kLoaderBase + 0x08u;
  constexpr RdramOffset kLwDramAddress = kLoaderBase + 0x0cu;
  constexpr RdramOffset kLuiCartAddress = kLoaderBase + 0x10u;
  constexpr RdramOffset kOriCartAddress = kLoaderBase + 0x14u;
  constexpr RdramOffset kSwCartAddress = kLoaderBase + 0x18u;
  constexpr RdramOffset kLwCartAddress = kLoaderBase + 0x1cu;
  constexpr RdramOffset kOriLengthAddress = kLoaderBase + 0x20u;
  constexpr RdramOffset kSwLengthAddress = kLoaderBase + 0x24u;
  constexpr RdramOffset kLwLengthAddress = kLoaderBase + 0x28u;
  constexpr RdramOffset kJumpAddress = kLoaderBase + 0x2cu;
  constexpr RdramOffset kDelaySlotAddress = kLoaderBase + 0x30u;
  constexpr RdramOffset kCartridgeLoadProbeAddress = kLoaderBase + 0x34u;

  machine.stage_rdram_u32_be(
      kLuiPiBaseAddress,
      encode_lui(static_cast<std::uint8_t>(kPiBaseIndex), 0xa460u));
  machine.stage_rdram_u32_be(
      kOriDramAddress,
      encode_ori(static_cast<std::uint8_t>(kValueIndex), 0, kTransferredProgramAddress));
  stage_pi_sw_instruction(
      machine,
      kSwDramAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiDramRegisterOffset);
  stage_pi_lw_instruction(
      machine,
      kLwDramAddress,
      static_cast<std::uint8_t>(kDramReadIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiDramRegisterOffset);
  machine.stage_rdram_u32_be(
      kLuiCartAddress,
      encode_lui(static_cast<std::uint8_t>(kValueIndex), 0x1000u));
  machine.stage_rdram_u32_be(
      kOriCartAddress,
      encode_ori(
          static_cast<std::uint8_t>(kValueIndex),
          static_cast<std::uint8_t>(kValueIndex),
          static_cast<std::uint16_t>(kTransferredCartridgeOffset)));
  stage_pi_sw_instruction(
      machine,
      kSwCartAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiCartRegisterOffset);
  stage_pi_lw_instruction(
      machine,
      kLwCartAddress,
      static_cast<std::uint8_t>(kCartReadIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiCartRegisterOffset);
  machine.stage_rdram_u32_be(
      kOriLengthAddress,
      encode_ori(static_cast<std::uint8_t>(kValueIndex), 0, kLengthRegisterValue));
  stage_pi_sw_instruction(
      machine,
      kSwLengthAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiCartToRdramLengthRegisterOffset);
  stage_pi_lw_instruction(
      machine,
      kLwLengthAddress,
      static_cast<std::uint8_t>(kLengthReadIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiCartToRdramLengthRegisterOffset);
  machine.stage_rdram_u32_be(kJumpAddress, encode_j(kTransferredCpuAddress));
  machine.stage_rdram_u32_be(kDelaySlotAddress, kNop);
  machine.stage_rdram_u32_be(
      kCartridgeLoadProbeAddress,
      encode_lw(
          static_cast<std::uint8_t>(kFaultTargetIndex),
          static_cast<std::uint8_t>(kFaultBaseIndex),
          0));

  if (machine.inspect_rdram_u32_be(kTransferredProgramAddress) != 0 ||
      machine.inspect_rdram_u32_be(kTransferredProgramAddress + 4u) != 0) {
    throw std::runtime_error("CPU-driven PI DMA proof destination was not blank before DMA");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLoaderBase));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kLoaderBase + 4u));

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_lui_pi_base");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_ori_dram");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_sw_dram");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_lw_dram");
  require_gpr_equals(machine, kDramReadIndex, kTransferredProgramAddress, "cpu_driven_pi_dma_lw_dram");

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_lui_cart");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_ori_cart");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_sw_cart");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_lw_cart");
  require_gpr_equals(machine, kCartReadIndex, kTransferredPiCartAddress, "cpu_driven_pi_dma_lw_cart");

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_ori_length");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_sw_length");

  require_rdram_word_equals(
      machine,
      kTransferredProgramAddress,
      kTransferredOri,
      "cpu_driven_pi_dma_transferred_ori");
  require_rdram_word_equals(
      machine,
      kTransferredProgramAddress + 4u,
      kTransferredBreak,
      "cpu_driven_pi_dma_transferred_break");

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_lw_length");
  require_gpr_equals(machine, kLengthReadIndex, kLengthRegisterValue, "cpu_driven_pi_dma_lw_length");

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_jump");
  if (machine.cpu_pc() != cpu_rdram_alias(kDelaySlotAddress) ||
      machine.cpu_next_pc() != kTransferredCpuAddress) {
    throw std::runtime_error("CPU-driven PI DMA proof jump cadence mismatch");
  }

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_delay_slot");
  if (machine.cpu_pc() != kTransferredCpuAddress ||
      machine.cpu_next_pc() != cpu_rdram_alias(kTransferredProgramAddress + 4u)) {
    throw std::runtime_error("CPU-driven PI DMA proof did not fetch transferred RDRAM program");
  }

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_transferred_ori");
  require_gpr_equals(
      machine,
      kTransferredResultIndex,
      0x5a5au,
      "cpu_driven_pi_dma_transferred_ori");

  require_stopped(machine.step_cpu_instruction(), "cpu_driven_pi_dma_transferred_break");

  machine.stage_cpu_pc(cpu_rdram_alias(kCartridgeLoadProbeAddress));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kCartridgeLoadProbeAddress + 4u));
  machine.stage_cpu_gpr(kFaultBaseIndex, 0xb0000040u);
  machine.stage_cpu_gpr(kFaultTargetIndex, 0xabcdef0123456789ull);
  require_step_machine_fault(
      machine,
      "cpu_driven_pi_dma_cartridge_range_not_mapped",
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
  require_gpr_equals(
      machine,
      kTransferredResultIndex,
      0x5a5au,
      "cpu_driven_pi_dma_cartridge_range_not_mapped");
  require_gpr_equals(
      machine,
      kFaultTargetIndex,
      0xabcdef0123456789ull,
      "cpu_driven_pi_dma_cartridge_range_not_mapped");
}

void run_cpu_driven_pi_sp_dma_chain_demo() {
  std::cout << "fn64 bootstrap PI/SP DMA demo: CPU loader chains PI DMA into SP IMEM data loads\n";

  constexpr std::size_t kPiBaseIndex = 4;
  constexpr std::size_t kValueIndex = 5;
  constexpr std::size_t kSpBaseIndex = 6;
  constexpr std::size_t kSpMemoryBaseIndex = 7;
  constexpr std::size_t kImemHighIndex = 8;
  constexpr std::size_t kImemLowIndex = 9;
  constexpr std::size_t kDmemBaseIndex = 10;
  constexpr std::size_t kDmemProbeIndex = 11;
  constexpr std::size_t kCartFaultBaseIndex = 12;
  constexpr std::size_t kCartFaultTargetIndex = 13;

  constexpr RdramOffset kLoaderBase = 0x00001a00u;
  constexpr RdramOffset kPiDmaRdramDestination = 0x00000300u;
  constexpr CartridgeOffset kPayloadCartridgeOffset = 0x00000040u;
  constexpr PiCartAddress kPayloadPiCartAddress =
      pi_cart_rom_address(kPayloadCartridgeOffset);
  constexpr std::uint32_t kLengthRegisterValue = 7u;
  constexpr std::uint32_t kPayloadHigh = 0x11223344u;
  constexpr std::uint32_t kPayloadLow = 0x55667788u;
  constexpr std::uint16_t kSpImemOffset = 0x0020u;
  constexpr std::uint32_t kSpImemDmaAddress = 0x00001020u;
  constexpr CpuAddress kSpImemDataAlias = sp_imem_uncached_alias(kSpImemOffset);
  constexpr CpuAddress kSpDmemDataAlias = sp_dmem_uncached_alias(kSpImemOffset);

  Cartridge cartridge;
  std::string error;
  if (!load_cartridge(
          make_bootstrap_cartridge_staging_rom(kPayloadHigh, kPayloadLow),
          cartridge,
          error)) {
    throw std::runtime_error("CPU-driven PI/SP DMA chain proof could not load synthetic cartridge: " + error);
  }

  auto machine_storage = std::make_unique<Machine>(std::move(cartridge));
  Machine& machine = *machine_storage;

  constexpr RdramOffset kLuiPiBaseAddress = kLoaderBase + 0x00u;
  constexpr RdramOffset kOriPiDramAddress = kLoaderBase + 0x04u;
  constexpr RdramOffset kSwPiDramAddress = kLoaderBase + 0x08u;
  constexpr RdramOffset kLuiPiCartAddress = kLoaderBase + 0x0cu;
  constexpr RdramOffset kOriPiCartAddress = kLoaderBase + 0x10u;
  constexpr RdramOffset kSwPiCartAddress = kLoaderBase + 0x14u;
  constexpr RdramOffset kOriPiLengthAddress = kLoaderBase + 0x18u;
  constexpr RdramOffset kSwPiLengthAddress = kLoaderBase + 0x1cu;
  constexpr RdramOffset kLuiSpBaseAddress = kLoaderBase + 0x20u;
  constexpr RdramOffset kOriSpMemoryAddress = kLoaderBase + 0x24u;
  constexpr RdramOffset kSwSpMemoryAddress = kLoaderBase + 0x28u;
  constexpr RdramOffset kOriSpDramAddress = kLoaderBase + 0x2cu;
  constexpr RdramOffset kSwSpDramAddress = kLoaderBase + 0x30u;
  constexpr RdramOffset kOriSpLengthAddress = kLoaderBase + 0x34u;
  constexpr RdramOffset kSwSpReadLengthAddress = kLoaderBase + 0x38u;
  constexpr RdramOffset kLuiSpImemAliasAddress = kLoaderBase + 0x3cu;
  constexpr RdramOffset kOriSpImemAliasAddress = kLoaderBase + 0x40u;
  constexpr RdramOffset kLwImemHighAddress = kLoaderBase + 0x44u;
  constexpr RdramOffset kLwImemLowAddress = kLoaderBase + 0x48u;
  constexpr RdramOffset kLuiSpDmemAliasAddress = kLoaderBase + 0x4cu;
  constexpr RdramOffset kOriSpDmemAliasAddress = kLoaderBase + 0x50u;
  constexpr RdramOffset kLwDmemProbeAddress = kLoaderBase + 0x54u;
  constexpr RdramOffset kBreakAddress = kLoaderBase + 0x58u;
  constexpr RdramOffset kCartridgeLoadProbeAddress = kLoaderBase + 0x5cu;

  machine.stage_rdram_u32_be(
      kLuiPiBaseAddress,
      encode_lui(static_cast<std::uint8_t>(kPiBaseIndex), 0xa460u));
  machine.stage_rdram_u32_be(
      kOriPiDramAddress,
      encode_ori(static_cast<std::uint8_t>(kValueIndex), 0, kPiDmaRdramDestination));
  stage_pi_sw_instruction(
      machine,
      kSwPiDramAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiDramRegisterOffset);
  machine.stage_rdram_u32_be(
      kLuiPiCartAddress,
      encode_lui(static_cast<std::uint8_t>(kValueIndex), 0x1000u));
  machine.stage_rdram_u32_be(
      kOriPiCartAddress,
      encode_ori(
          static_cast<std::uint8_t>(kValueIndex),
          static_cast<std::uint8_t>(kValueIndex),
          static_cast<std::uint16_t>(kPayloadCartridgeOffset)));
  stage_pi_sw_instruction(
      machine,
      kSwPiCartAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiCartRegisterOffset);
  machine.stage_rdram_u32_be(
      kOriPiLengthAddress,
      encode_ori(static_cast<std::uint8_t>(kValueIndex), 0, kLengthRegisterValue));
  stage_pi_sw_instruction(
      machine,
      kSwPiLengthAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kPiBaseIndex),
      kPiCartToRdramLengthRegisterOffset);

  machine.stage_rdram_u32_be(
      kLuiSpBaseAddress,
      encode_lui(static_cast<std::uint8_t>(kSpBaseIndex), 0xa404u));
  machine.stage_rdram_u32_be(
      kOriSpMemoryAddress,
      encode_ori(static_cast<std::uint8_t>(kValueIndex), 0, kSpImemDmaAddress));
  stage_sp_sw_instruction(
      machine,
      kSwSpMemoryAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpMemoryRegisterOffset);
  machine.stage_rdram_u32_be(
      kOriSpDramAddress,
      encode_ori(static_cast<std::uint8_t>(kValueIndex), 0, kPiDmaRdramDestination));
  stage_sp_sw_instruction(
      machine,
      kSwSpDramAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpDramRegisterOffset);
  machine.stage_rdram_u32_be(
      kOriSpLengthAddress,
      encode_ori(static_cast<std::uint8_t>(kValueIndex), 0, kLengthRegisterValue));
  stage_sp_sw_instruction(
      machine,
      kSwSpReadLengthAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpReadLengthRegisterOffset);

  machine.stage_rdram_u32_be(
      kLuiSpImemAliasAddress,
      encode_lui(static_cast<std::uint8_t>(kSpMemoryBaseIndex), 0xa400u));
  machine.stage_rdram_u32_be(
      kOriSpImemAliasAddress,
      encode_ori(
          static_cast<std::uint8_t>(kSpMemoryBaseIndex),
          static_cast<std::uint8_t>(kSpMemoryBaseIndex),
          0x1020u));
  machine.stage_rdram_u32_be(
      kLwImemHighAddress,
      encode_lw(
          static_cast<std::uint8_t>(kImemHighIndex),
          static_cast<std::uint8_t>(kSpMemoryBaseIndex),
          0));
  machine.stage_rdram_u32_be(
      kLwImemLowAddress,
      encode_lw(
          static_cast<std::uint8_t>(kImemLowIndex),
          static_cast<std::uint8_t>(kSpMemoryBaseIndex),
          4));

  machine.stage_rdram_u32_be(
      kLuiSpDmemAliasAddress,
      encode_lui(static_cast<std::uint8_t>(kDmemBaseIndex), 0xa400u));
  machine.stage_rdram_u32_be(
      kOriSpDmemAliasAddress,
      encode_ori(
          static_cast<std::uint8_t>(kDmemBaseIndex),
          static_cast<std::uint8_t>(kDmemBaseIndex),
          kSpImemOffset));
  machine.stage_rdram_u32_be(
      kLwDmemProbeAddress,
      encode_lw(
          static_cast<std::uint8_t>(kDmemProbeIndex),
          static_cast<std::uint8_t>(kDmemBaseIndex),
          0));
  machine.stage_rdram_u32_be(kBreakAddress, encode_break());
  machine.stage_rdram_u32_be(
      kCartridgeLoadProbeAddress,
      encode_lw(
          static_cast<std::uint8_t>(kCartFaultTargetIndex),
          static_cast<std::uint8_t>(kCartFaultBaseIndex),
          0));

  if (machine.inspect_rdram_u32_be(kPiDmaRdramDestination) != 0 ||
      machine.inspect_rdram_u32_be(kPiDmaRdramDestination + 4u) != 0) {
    throw std::runtime_error("CPU-driven PI/SP DMA chain proof RDRAM destination was not blank before DMA");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLoaderBase));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kLoaderBase + 4u));

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lui_pi_base");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_pi_dram");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_sw_pi_dram");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lui_pi_cart");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_pi_cart");
  require_gpr_equals(
      machine,
      kValueIndex,
      kPayloadPiCartAddress,
      "cpu_driven_pi_sp_dma_ori_pi_cart");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_sw_pi_cart");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_pi_length");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_sw_pi_length");

  require_rdram_word_equals(
      machine,
      kPiDmaRdramDestination,
      kPayloadHigh,
      "cpu_driven_pi_sp_dma_pi_high");
  require_rdram_word_equals(
      machine,
      kPiDmaRdramDestination + 4u,
      kPayloadLow,
      "cpu_driven_pi_sp_dma_pi_low");

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lui_sp_base");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_sp_memory");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_sw_sp_memory");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_sp_dram");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_sw_sp_dram");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_sp_length");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_sw_sp_read_length");

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lui_sp_imem_alias");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_sp_imem_alias");
  require_gpr_equals(
      machine,
      kSpMemoryBaseIndex,
      cpu_value_from_sign_extended_u32(kSpImemDataAlias),
      "cpu_driven_pi_sp_dma_ori_sp_imem_alias");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lw_imem_high");
  require_gpr_equals(
      machine,
      kImemHighIndex,
      cpu_value_from_sign_extended_u32(kPayloadHigh),
      "cpu_driven_pi_sp_dma_lw_imem_high");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lw_imem_low");
  require_gpr_equals(
      machine,
      kImemLowIndex,
      cpu_value_from_sign_extended_u32(kPayloadLow),
      "cpu_driven_pi_sp_dma_lw_imem_low");

  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lui_sp_dmem_alias");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_ori_sp_dmem_alias");
  require_gpr_equals(
      machine,
      kDmemBaseIndex,
      cpu_value_from_sign_extended_u32(kSpDmemDataAlias),
      "cpu_driven_pi_sp_dma_ori_sp_dmem_alias");
  require_stepped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_lw_dmem_probe");
  require_gpr_equals(machine, kDmemProbeIndex, 0, "cpu_driven_pi_sp_dma_lw_dmem_probe");

  require_stopped(machine.step_cpu_instruction(), "cpu_driven_pi_sp_dma_break");

  machine.stage_cpu_pc(kSpImemDataAlias);
  machine.stage_cpu_next_pc(kSpImemDataAlias + 4u);
  require_step_machine_fault(
      machine,
      "cpu_driven_pi_sp_dma_fetch_sp_imem_rejected",
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
  require_gpr_equals(
      machine,
      kImemHighIndex,
      cpu_value_from_sign_extended_u32(kPayloadHigh),
      "cpu_driven_pi_sp_dma_fetch_sp_imem_rejected");
  require_gpr_equals(
      machine,
      kImemLowIndex,
      cpu_value_from_sign_extended_u32(kPayloadLow),
      "cpu_driven_pi_sp_dma_fetch_sp_imem_rejected");

  machine.stage_cpu_pc(cpu_rdram_alias(kCartridgeLoadProbeAddress));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kCartridgeLoadProbeAddress + 4u));
  machine.stage_cpu_gpr(kCartFaultBaseIndex, 0xb0000040u);
  machine.stage_cpu_gpr(kCartFaultTargetIndex, 0x123456789abcdef0ull);
  require_step_machine_fault(
      machine,
      "cpu_driven_pi_sp_dma_cartridge_range_not_mapped",
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
  require_gpr_equals(
      machine,
      kCartFaultTargetIndex,
      0x123456789abcdef0ull,
      "cpu_driven_pi_sp_dma_cartridge_range_not_mapped");
}

void run_pi_mmio_dma_success_demo() {
  std::cout << "fn64 bootstrap PI MMIO demo: local immediate cartridge-to-RDRAM DMA subset\n";

  auto machine_storage = make_pi_dma_proof_machine();
  Machine& machine = *machine_storage;

  constexpr std::size_t kPiBaseIndex = 4;
  constexpr std::size_t kValueIndex = 5;
  constexpr std::size_t kDramReadIndex = 6;
  constexpr std::size_t kCartReadIndex = 7;
  constexpr std::size_t kLengthReadIndex = 8;
  constexpr std::size_t kStatusReadIndex = 9;
  constexpr RdramOffset kSwDramAddress = 0x00001100u;
  constexpr RdramOffset kSwCartAddress = 0x00001104u;
  constexpr RdramOffset kSwLengthAddress = 0x00001108u;
  constexpr RdramOffset kLwDramAddress = 0x0000110cu;
  constexpr RdramOffset kLwCartAddress = 0x00001110u;
  constexpr RdramOffset kLwLengthAddress = 0x00001114u;
  constexpr RdramOffset kLwStatusAddress = 0x00001118u;
  constexpr RdramOffset kDmaDestination = 0x00001200u;
  constexpr RdramOffset kBaseDmaDestination = 0x00001220u;
  constexpr PiCartAddress kPiCartBaseSource = kSyntheticPiCartRomBase;
  constexpr PiCartAddress kPiCartSource = pi_cart_rom_address(0x00000040u);
  constexpr std::uint32_t kLengthRegisterValue = 7u;

  stage_pi_sw_instruction(machine, kSwDramAddress, kValueIndex, kPiBaseIndex, kPiDramRegisterOffset);
  stage_pi_sw_instruction(machine, kSwCartAddress, kValueIndex, kPiBaseIndex, kPiCartRegisterOffset);
  stage_pi_sw_instruction(machine, kSwLengthAddress, kValueIndex, kPiBaseIndex, kPiCartToRdramLengthRegisterOffset);
  stage_pi_lw_instruction(machine, kLwDramAddress, kDramReadIndex, kPiBaseIndex, kPiDramRegisterOffset);
  stage_pi_lw_instruction(machine, kLwCartAddress, kCartReadIndex, kPiBaseIndex, kPiCartRegisterOffset);
  stage_pi_lw_instruction(machine, kLwLengthAddress, kLengthReadIndex, kPiBaseIndex, kPiCartToRdramLengthRegisterOffset);
  stage_pi_lw_instruction(machine, kLwStatusAddress, kStatusReadIndex, kPiBaseIndex, kPiStatusRegisterOffset);
  machine.stage_cpu_gpr(kPiBaseIndex, kSyntheticPiMmioCpuBase);

  machine.stage_cpu_gpr(kValueIndex, kBaseDmaDestination);
  step_at(machine, kSwDramAddress, "pi_mmio_success_demo_sw_base_dram");
  machine.stage_cpu_gpr(kValueIndex, kPiCartBaseSource);
  step_at(machine, kSwCartAddress, "pi_mmio_success_demo_sw_base_cart");
  machine.stage_cpu_gpr(kValueIndex, kLengthRegisterValue);
  step_at(machine, kSwLengthAddress, "pi_mmio_success_demo_sw_base_length");

  require_rdram_word_equals(machine, kBaseDmaDestination, 0x80371240u, "pi_mmio_success_demo_dma_base_high");
  require_rdram_word_equals(machine, kBaseDmaDestination + 4u, 0x12345678u, "pi_mmio_success_demo_dma_base_low");

  machine.stage_cpu_gpr(kValueIndex, kDmaDestination);
  step_at(machine, kSwDramAddress, "pi_mmio_success_demo_sw_dram");
  machine.stage_cpu_gpr(kValueIndex, kPiCartSource);
  step_at(machine, kSwCartAddress, "pi_mmio_success_demo_sw_cart");
  machine.stage_cpu_gpr(kValueIndex, kLengthRegisterValue);
  step_at(machine, kSwLengthAddress, "pi_mmio_success_demo_sw_length");

  require_rdram_word_equals(machine, kDmaDestination, 0xd1d4d7dau, "pi_mmio_success_demo_dma_high");
  require_rdram_word_equals(machine, kDmaDestination + 4u, 0xdde0e3e6u, "pi_mmio_success_demo_dma_low");

  step_at(machine, kLwDramAddress, "pi_mmio_success_demo_lw_dram");
  step_at(machine, kLwCartAddress, "pi_mmio_success_demo_lw_cart");
  step_at(machine, kLwLengthAddress, "pi_mmio_success_demo_lw_length");
  step_at(machine, kLwStatusAddress, "pi_mmio_success_demo_lw_status");

  require_gpr_equals(machine, kDramReadIndex, kDmaDestination, "pi_mmio_success_demo_lw_dram");
  require_gpr_equals(machine, kCartReadIndex, kPiCartSource, "pi_mmio_success_demo_lw_cart");
  require_gpr_equals(machine, kLengthReadIndex, kLengthRegisterValue, "pi_mmio_success_demo_lw_length");
  require_gpr_equals(machine, kStatusReadIndex, 0, "pi_mmio_success_demo_lw_status");
}

void run_pi_dma_reservation_demo() {
  std::cout << "fn64 bootstrap PI MMIO demo: DMA writes interact with local LL/SC reservations\n";

  {
    auto machine_storage = make_pi_dma_proof_machine();
    Machine& machine = *machine_storage;
    constexpr std::size_t kPiBaseIndex = 4;
    constexpr std::size_t kValueIndex = 5;
    constexpr std::size_t kLlTargetIndex = 6;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00001140u;
    constexpr RdramOffset kSwDramAddress = 0x00001144u;
    constexpr RdramOffset kSwCartAddress = 0x00001148u;
    constexpr RdramOffset kSwLengthAddress = 0x0000114cu;
    constexpr RdramOffset kScAddress = 0x00001150u;
    constexpr RdramOffset kDataAddress = 0x00001240u;
    constexpr PiCartAddress kPiCartSource = pi_cart_rom_address(0x00000040u);

    machine.stage_rdram_u32_be(kLlAddress, encode_ll(kLlTargetIndex, kPiBaseIndex, 0));
    stage_pi_sw_instruction(machine, kSwDramAddress, kValueIndex, kPiBaseIndex, kPiDramRegisterOffset);
    stage_pi_sw_instruction(machine, kSwCartAddress, kValueIndex, kPiBaseIndex, kPiCartRegisterOffset);
    stage_pi_sw_instruction(machine, kSwLengthAddress, kValueIndex, kPiBaseIndex, kPiCartToRdramLengthRegisterOffset);
    machine.stage_rdram_u32_be(kScAddress, encode_sc(kScSourceIndex, kPiBaseIndex, 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);

    machine.stage_cpu_gpr(kPiBaseIndex, cpu_rdram_alias(kDataAddress));
    step_at(machine, kLlAddress, "pi_dma_reservation_demo_ll");

    machine.stage_cpu_gpr(kPiBaseIndex, kSyntheticPiMmioCpuBase);
    machine.stage_cpu_gpr(kValueIndex, kDataAddress);
    step_at(machine, kSwDramAddress, "pi_dma_reservation_demo_sw_dram");
    machine.stage_cpu_gpr(kValueIndex, kPiCartSource);
    step_at(machine, kSwCartAddress, "pi_dma_reservation_demo_sw_cart");
    machine.stage_cpu_gpr(kValueIndex, 3u);
    step_at(machine, kSwLengthAddress, "pi_dma_reservation_demo_sw_length");

    machine.stage_cpu_gpr(kPiBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);
    step_at(machine, kScAddress, "pi_dma_reservation_demo_overlap_sc");

    require_gpr_equals(machine, kScSourceIndex, 0, "pi_dma_reservation_demo_overlap_sc");
    require_rdram_word_equals(machine, kDataAddress, 0xd1d4d7dau, "pi_dma_reservation_demo_overlap_sc");
  }

  {
    auto machine_storage = make_pi_dma_proof_machine();
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kPiBaseIndex = 8;
    constexpr std::size_t kValueIndex = 5;
    constexpr std::size_t kLlTargetIndex = 6;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00001180u;
    constexpr RdramOffset kSwDramAddress = 0x00001184u;
    constexpr RdramOffset kSwCartAddress = 0x00001188u;
    constexpr RdramOffset kSwLengthAddress = 0x0000118cu;
    constexpr RdramOffset kScAddress = 0x00001190u;
    constexpr RdramOffset kReservedAddress = 0x00001280u;
    constexpr RdramOffset kDmaAddress = 0x000012c0u;
    constexpr PiCartAddress kPiCartSource = pi_cart_rom_address(0x00000040u);

    machine.stage_rdram_u32_be(kLlAddress, encode_ll(kLlTargetIndex, kBaseIndex, 0));
    stage_pi_sw_instruction(machine, kSwDramAddress, kValueIndex, kPiBaseIndex, kPiDramRegisterOffset);
    stage_pi_sw_instruction(machine, kSwCartAddress, kValueIndex, kPiBaseIndex, kPiCartRegisterOffset);
    stage_pi_sw_instruction(machine, kSwLengthAddress, kValueIndex, kPiBaseIndex, kPiCartToRdramLengthRegisterOffset);
    machine.stage_rdram_u32_be(kScAddress, encode_sc(kScSourceIndex, kBaseIndex, 0));
    machine.stage_rdram_u32_be(kReservedAddress, 0x11112222u);
    machine.stage_rdram_u32_be(kDmaAddress, 0x33334444u);

    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kReservedAddress));
    machine.stage_cpu_gpr(kPiBaseIndex, kSyntheticPiMmioCpuBase);
    step_at(machine, kLlAddress, "pi_dma_reservation_demo_non_overlap_ll");

    machine.stage_cpu_gpr(kValueIndex, kDmaAddress);
    step_at(machine, kSwDramAddress, "pi_dma_reservation_demo_non_overlap_sw_dram");
    machine.stage_cpu_gpr(kValueIndex, kPiCartSource);
    step_at(machine, kSwCartAddress, "pi_dma_reservation_demo_non_overlap_sw_cart");
    machine.stage_cpu_gpr(kValueIndex, 3u);
    step_at(machine, kSwLengthAddress, "pi_dma_reservation_demo_non_overlap_sw_length");

    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);
    step_at(machine, kScAddress, "pi_dma_reservation_demo_non_overlap_sc");

    require_gpr_equals(machine, kScSourceIndex, 1, "pi_dma_reservation_demo_non_overlap_sc");
    require_rdram_word_equals(machine, kReservedAddress, 0xaabbccddu, "pi_dma_reservation_demo_non_overlap_sc");
    require_rdram_word_equals(machine, kDmaAddress, 0xd1d4d7dau, "pi_dma_reservation_demo_non_overlap_sc");
  }
}

void run_pi_dma_failure_demo() {
  std::cout << "fn64 bootstrap PI MMIO demo: failed DMA preflight leaves RDRAM and reservation untouched\n";

  const auto require_failed_dma_preserves_reservation =
      [](const char* label,
         RdramOffset dma_destination,
         PiCartAddress pi_cart_source,
         std::uint32_t length_register_value) {
        auto machine_storage = make_pi_dma_proof_machine();
        Machine& machine = *machine_storage;
        constexpr std::size_t kBaseIndex = 4;
        constexpr std::size_t kPiBaseIndex = 8;
        constexpr std::size_t kValueIndex = 5;
        constexpr std::size_t kLlTargetIndex = 6;
        constexpr std::size_t kScSourceIndex = 7;
        constexpr RdramOffset kLlAddress = 0x000011c0u;
        constexpr RdramOffset kSwDramAddress = 0x000011c4u;
        constexpr RdramOffset kSwCartAddress = 0x000011c8u;
        constexpr RdramOffset kSwLengthAddress = 0x000011ccu;
        constexpr RdramOffset kScAddress = 0x000011d0u;
        constexpr RdramOffset kReservedAddress = 0x00001300u;
        constexpr RdramOffset kSentinelAddress = 0x00001340u;

        machine.stage_rdram_u32_be(kLlAddress, encode_ll(kLlTargetIndex, kBaseIndex, 0));
        stage_pi_sw_instruction(machine, kSwDramAddress, kValueIndex, kPiBaseIndex, kPiDramRegisterOffset);
        stage_pi_sw_instruction(machine, kSwCartAddress, kValueIndex, kPiBaseIndex, kPiCartRegisterOffset);
        stage_pi_sw_instruction(machine, kSwLengthAddress, kValueIndex, kPiBaseIndex, kPiCartToRdramLengthRegisterOffset);
        machine.stage_rdram_u32_be(kScAddress, encode_sc(kScSourceIndex, kBaseIndex, 0));
        machine.stage_rdram_u32_be(kReservedAddress, 0x11112222u);
        machine.stage_rdram_u32_be(kSentinelAddress, 0x55667788u);

        machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kReservedAddress));
        machine.stage_cpu_gpr(kPiBaseIndex, kSyntheticPiMmioCpuBase);
        step_at(machine, kLlAddress, label);

        machine.stage_cpu_gpr(kValueIndex, dma_destination);
        step_at(machine, kSwDramAddress, label);
        machine.stage_cpu_gpr(kValueIndex, pi_cart_source);
        step_at(machine, kSwCartAddress, label);
        machine.stage_cpu_gpr(kValueIndex, length_register_value);
        machine.stage_cpu_pc(cpu_rdram_alias(kSwLengthAddress));
        require_step_out_of_range(machine, label);

        require_rdram_word_equals(machine, kSentinelAddress, 0x55667788u, label);
        machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);
        step_at(machine, kScAddress, label);
        require_gpr_equals(machine, kScSourceIndex, 1, label);
        require_rdram_word_equals(machine, kReservedAddress, 0xaabbccddu, label);
      };

  require_failed_dma_preserves_reservation(
      "pi_dma_failure_demo_raw_offset_like_source_rejected",
      0x00001380u,
      0x00000040u,
      7u);
  require_failed_dma_preserves_reservation(
      "pi_dma_failure_demo_source_below_base",
      0x00001380u,
      0x0ffffff0u,
      7u);
  require_failed_dma_preserves_reservation(
      "pi_dma_failure_demo_source_out_of_range",
      0x00001380u,
      pi_cart_rom_address(0x0000005cu),
      7u);
  require_failed_dma_preserves_reservation(
      "pi_dma_failure_demo_source_span_overflow",
      0x00001380u,
      0xfffffffcu,
      7u);
  require_failed_dma_preserves_reservation(
      "pi_dma_failure_demo_destination_out_of_range",
      0x003ffffcu,
      pi_cart_rom_address(0x00000040u),
      7u);
  require_failed_dma_preserves_reservation(
      "pi_dma_failure_demo_destination_span_overflow",
      0xfffffffcu,
      pi_cart_rom_address(0x00000040u),
      7u);
  require_failed_dma_preserves_reservation(
      "pi_dma_failure_demo_length_overflow",
      0x00001380u,
      pi_cart_rom_address(0x00000040u),
      0xffffffffu);
}

void run_pi_mmio_fault_demo() {
  std::cout << "fn64 bootstrap PI MMIO demo: unsupported PI widths and fetch stay local faults\n";

  const auto require_pi_machine_fault =
      [](const char* label,
         CpuInstructionWord instruction,
         CpuAddress base_value,
         MachineFaultKind expected_kind,
         std::size_t expected_access_size) {
        auto machine_storage = make_pi_dma_proof_machine();
        Machine& machine = *machine_storage;
        constexpr std::size_t kBaseIndex = 4;
        constexpr std::size_t kSourceIndex = 5;
        constexpr std::size_t kTargetIndex = 6;
        constexpr RdramOffset kInstructionAddress = 0x00001200u;
        constexpr RdramOffset kSentinelAddress = 0x000013c0u;
        constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;
        constexpr std::uint32_t kRdramSentinel = 0xaabbccddu;

        machine.stage_rdram_u32_be(kInstructionAddress, instruction);
        machine.stage_rdram_u32_be(kSentinelAddress, kRdramSentinel);
        machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
        machine.stage_cpu_gpr(kBaseIndex, base_value);
        machine.stage_cpu_gpr(kSourceIndex, 0x0102030405060708ull);
        machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

        require_step_machine_fault(machine, label, expected_kind, expected_access_size);

        if (machine.cpu_pc() != cpu_rdram_alias(kInstructionAddress) ||
            machine.cpu_next_pc() != cpu_rdram_alias(kInstructionAddress + 4u)) {
          throw std::runtime_error(std::string(label) + " changed pc/next_pc on fault");
        }

        require_gpr_equals(machine, kTargetIndex, kTargetSentinel, label);
        require_rdram_word_equals(machine, kSentinelAddress, kRdramSentinel, label);
      };

  require_pi_machine_fault(
      "pi_mmio_fault_demo_sb_unsupported_width",
      encode_sb(5, 4, kPiDramRegisterOffset),
      kSyntheticPiMmioCpuBase,
      MachineFaultKind::kUnsupportedCpuDataAccess,
      1);
  require_pi_machine_fault(
      "pi_mmio_fault_demo_ld_unsupported_width",
      encode_ld(6, 4, kPiDramRegisterOffset),
      kSyntheticPiMmioCpuBase,
      MachineFaultKind::kUnsupportedCpuDataAccess,
      8);
  require_pi_machine_fault(
      "pi_mmio_fault_demo_unknown_register",
      encode_lw(6, 4, kPiUnknownRegisterOffset),
      kSyntheticPiMmioCpuBase,
      MachineFaultKind::kUnsupportedCpuDataAccess,
      4);
  require_pi_machine_fault(
      "pi_mmio_fault_demo_unaligned_sw",
      encode_sw(5, 4, 2),
      kSyntheticPiMmioCpuBase,
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      4);
  require_pi_machine_fault(
      "pi_mmio_fault_demo_ll_rejected",
      encode_ll(6, 4, kPiDramRegisterOffset),
      kSyntheticPiMmioCpuBase,
      MachineFaultKind::kUnsupportedCpuDataAccess,
      4);
  require_pi_machine_fault(
      "pi_mmio_fault_demo_sc_rejected",
      encode_sc(5, 4, kPiDramRegisterOffset),
      kSyntheticPiMmioCpuBase,
      MachineFaultKind::kUnsupportedCpuDataAccess,
      4);
  require_pi_machine_fault(
      "pi_mmio_fault_demo_cartridge_range_not_mapped",
      encode_lw(6, 4, 0),
      0xb0000040u,
      MachineFaultKind::kCpuRdramAddressRejected,
      4);

  {
    auto machine_storage = make_pi_dma_proof_machine();
    Machine& machine = *machine_storage;
    constexpr RdramOffset kSentinelAddress = 0x000013e0u;
    machine.stage_cpu_pc(kSyntheticPiMmioCpuBase);
    machine.stage_rdram_u32_be(kSentinelAddress, 0x12345678u);
    require_step_machine_fault(
        machine,
        "pi_mmio_fault_demo_fetch_rejected",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);
    require_rdram_word_equals(machine, kSentinelAddress, 0x12345678u, "pi_mmio_fault_demo_fetch_rejected");
  }
}

void run_sp_mmio_dma_success_demo() {
  std::cout << "fn64 bootstrap SP MMIO demo: local immediate RDRAM/SP DMA subset\n";

  auto machine_storage = std::make_unique<Machine>(Cartridge{});
  Machine& machine = *machine_storage;

  constexpr std::size_t kSpBaseIndex = 4;
  constexpr std::size_t kValueIndex = 5;
  constexpr std::size_t kRegisterReadIndex = 6;
  constexpr std::size_t kSpMemoryBaseIndex = 7;
  constexpr std::size_t kSpMemoryReadIndex = 8;
  constexpr std::size_t kSpMemorySourceIndex = 9;

  constexpr RdramOffset kSwSpMemAddress = 0x00001700u;
  constexpr RdramOffset kSwDramAddress = 0x00001704u;
  constexpr RdramOffset kSwReadLengthAddress = 0x00001708u;
  constexpr RdramOffset kSwWriteLengthAddress = 0x0000170cu;
  constexpr RdramOffset kLwSpMemAddress = 0x00001710u;
  constexpr RdramOffset kLwDramAddress = 0x00001714u;
  constexpr RdramOffset kLwReadLengthAddress = 0x00001718u;
  constexpr RdramOffset kLwWriteLengthAddress = 0x0000171cu;
  constexpr RdramOffset kLwStatusAddress = 0x00001720u;
  constexpr RdramOffset kLoadSpAddress = 0x00001724u;
  constexpr RdramOffset kStoreSpAddress = 0x00001728u;

  constexpr RdramOffset kDmemReadSource = 0x00001800u;
  constexpr RdramOffset kImemReadSource = 0x00001808u;
  constexpr RdramOffset kDmemWriteDestination = 0x00001820u;
  constexpr RdramOffset kImemWriteDestination = 0x00001830u;
  constexpr RdramOffset kDmemBlockReadSource = 0x00001840u;
  constexpr RdramOffset kImemBlockReadSource = 0x00001850u;
  constexpr RdramOffset kDmemBlockWriteDestination = 0x00001870u;
  constexpr std::uint32_t kDmemAddress = 0x00000100u;
  constexpr std::uint32_t kImemAddress = 0x00001020u;
  constexpr std::uint32_t kDmemWriteAddress = 0x00000180u;
  constexpr std::uint32_t kImemWriteAddress = 0x00001040u;
  constexpr std::uint32_t kLengthRegisterValue = 7u;
  constexpr std::uint32_t kBlockSkipCommand =
      encode_sp_dma_length_command(3u, 1u, 4u);

  stage_sp_sw_instruction(
      machine,
      kSwSpMemAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpMemoryRegisterOffset);
  stage_sp_sw_instruction(
      machine,
      kSwDramAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpDramRegisterOffset);
  stage_sp_sw_instruction(
      machine,
      kSwReadLengthAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpReadLengthRegisterOffset);
  stage_sp_sw_instruction(
      machine,
      kSwWriteLengthAddress,
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpWriteLengthRegisterOffset);
  stage_sp_lw_instruction(
      machine,
      kLwSpMemAddress,
      static_cast<std::uint8_t>(kRegisterReadIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpMemoryRegisterOffset);
  stage_sp_lw_instruction(
      machine,
      kLwDramAddress,
      static_cast<std::uint8_t>(kRegisterReadIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpDramRegisterOffset);
  stage_sp_lw_instruction(
      machine,
      kLwReadLengthAddress,
      static_cast<std::uint8_t>(kRegisterReadIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpReadLengthRegisterOffset);
  stage_sp_lw_instruction(
      machine,
      kLwWriteLengthAddress,
      static_cast<std::uint8_t>(kRegisterReadIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpWriteLengthRegisterOffset);
  stage_sp_lw_instruction(
      machine,
      kLwStatusAddress,
      static_cast<std::uint8_t>(kRegisterReadIndex),
      static_cast<std::uint8_t>(kSpBaseIndex),
      kSpStatusRegisterOffset);
  machine.stage_rdram_u32_be(
      kLoadSpAddress,
      encode_ld(
          static_cast<std::uint8_t>(kSpMemoryReadIndex),
          static_cast<std::uint8_t>(kSpMemoryBaseIndex),
          0));
  machine.stage_rdram_u32_be(
      kStoreSpAddress,
      encode_sd(
          static_cast<std::uint8_t>(kSpMemorySourceIndex),
          static_cast<std::uint8_t>(kSpMemoryBaseIndex),
          0));

  machine.stage_cpu_gpr(kSpBaseIndex, kSyntheticSpMmioCpuBase);
  stage_rdram_u64_be(machine, kDmemReadSource, 0x1122334455667788ull);
  stage_rdram_u64_be(machine, kImemReadSource, 0x99aabbccddeeff00ull);
  machine.stage_rdram_u32_be(kDmemBlockReadSource, 0x01020304u);
  machine.stage_rdram_u32_be(kDmemBlockReadSource + 4u, 0xaabbccddu);
  machine.stage_rdram_u32_be(kDmemBlockReadSource + 8u, 0x05060708u);
  machine.stage_rdram_u32_be(kImemBlockReadSource, 0x11223344u);
  machine.stage_rdram_u32_be(kImemBlockReadSource + 4u, 0xaabbccddu);
  machine.stage_rdram_u32_be(kImemBlockReadSource + 8u, 0x55667788u);

  machine.stage_cpu_gpr(kValueIndex, kDmemAddress);
  step_at(machine, kSwSpMemAddress, "sp_mmio_success_demo_sw_dmem_address");
  machine.stage_cpu_gpr(kValueIndex, kDmemReadSource);
  step_at(machine, kSwDramAddress, "sp_mmio_success_demo_sw_read_dram");
  machine.stage_cpu_gpr(kValueIndex, kLengthRegisterValue);
  step_at(machine, kSwReadLengthAddress, "sp_mmio_success_demo_sw_read_length");
  machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0100u));
  step_at(machine, kLoadSpAddress, "sp_mmio_success_demo_load_dmem");
  require_gpr_equals(
      machine,
      kSpMemoryReadIndex,
      0x1122334455667788ull,
      "sp_mmio_success_demo_load_dmem");

  machine.stage_cpu_gpr(kValueIndex, kImemAddress);
  step_at(machine, kSwSpMemAddress, "sp_mmio_success_demo_sw_imem_address");
  machine.stage_cpu_gpr(kValueIndex, kImemReadSource);
  step_at(machine, kSwDramAddress, "sp_mmio_success_demo_sw_imem_read_dram");
  machine.stage_cpu_gpr(kValueIndex, kLengthRegisterValue);
  step_at(machine, kSwReadLengthAddress, "sp_mmio_success_demo_sw_imem_read_length");
  machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_imem_uncached_alias(0x0020u));
  step_at(machine, kLoadSpAddress, "sp_mmio_success_demo_load_imem");
  require_gpr_equals(
      machine,
      kSpMemoryReadIndex,
      0x99aabbccddeeff00ull,
      "sp_mmio_success_demo_load_imem");

  machine.stage_cpu_gpr(kValueIndex, kDmemAddress + 0x0200u);
  step_at(machine, kSwSpMemAddress, "sp_mmio_success_demo_sw_dmem_block_address");
  machine.stage_cpu_gpr(kValueIndex, kDmemBlockReadSource);
  step_at(machine, kSwDramAddress, "sp_mmio_success_demo_sw_dmem_block_read_dram");
  machine.stage_cpu_gpr(kValueIndex, kBlockSkipCommand);
  step_at(machine, kSwReadLengthAddress, "sp_mmio_success_demo_sw_dmem_block_read_length");
  machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0300u));
  step_at(machine, kLoadSpAddress, "sp_mmio_success_demo_load_dmem_block");
  require_gpr_equals(
      machine,
      kSpMemoryReadIndex,
      0x0102030405060708ull,
      "sp_mmio_success_demo_load_dmem_block");

  machine.stage_cpu_gpr(kValueIndex, kImemAddress + 0x0200u);
  step_at(machine, kSwSpMemAddress, "sp_mmio_success_demo_sw_imem_block_address");
  machine.stage_cpu_gpr(kValueIndex, kImemBlockReadSource);
  step_at(machine, kSwDramAddress, "sp_mmio_success_demo_sw_imem_block_read_dram");
  machine.stage_cpu_gpr(kValueIndex, kBlockSkipCommand);
  step_at(machine, kSwReadLengthAddress, "sp_mmio_success_demo_sw_imem_block_read_length");
  machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_imem_uncached_alias(0x0220u));
  step_at(machine, kLoadSpAddress, "sp_mmio_success_demo_load_imem_block");
  require_gpr_equals(
      machine,
      kSpMemoryReadIndex,
      0x1122334455667788ull,
      "sp_mmio_success_demo_load_imem_block");

  machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0180u));
  machine.stage_cpu_gpr(kSpMemorySourceIndex, 0xaabbccddeeff0011ull);
  step_at(machine, kStoreSpAddress, "sp_mmio_success_demo_seed_dmem");
  machine.stage_cpu_gpr(kValueIndex, kDmemWriteAddress);
  step_at(machine, kSwSpMemAddress, "sp_mmio_success_demo_sw_dmem_write_address");
  machine.stage_cpu_gpr(kValueIndex, kDmemWriteDestination);
  step_at(machine, kSwDramAddress, "sp_mmio_success_demo_sw_write_dram");
  machine.stage_cpu_gpr(kValueIndex, kLengthRegisterValue);
  step_at(machine, kSwWriteLengthAddress, "sp_mmio_success_demo_sw_write_length");
  require_rdram_doubleword_equals(
      machine,
      kDmemWriteDestination,
      0xaabbccddeeff0011ull,
      "sp_mmio_success_demo_write_dmem");

  machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_imem_uncached_alias(0x0040u));
  machine.stage_cpu_gpr(kSpMemorySourceIndex, 0x0102030405060708ull);
  step_at(machine, kStoreSpAddress, "sp_mmio_success_demo_seed_imem");
  machine.stage_cpu_gpr(kValueIndex, kImemWriteAddress);
  step_at(machine, kSwSpMemAddress, "sp_mmio_success_demo_sw_imem_write_address");
  machine.stage_cpu_gpr(kValueIndex, kImemWriteDestination);
  step_at(machine, kSwDramAddress, "sp_mmio_success_demo_sw_imem_write_dram");
  machine.stage_cpu_gpr(kValueIndex, kLengthRegisterValue);
  step_at(machine, kSwWriteLengthAddress, "sp_mmio_success_demo_sw_imem_write_length");
  require_rdram_doubleword_equals(
      machine,
      kImemWriteDestination,
      0x0102030405060708ull,
      "sp_mmio_success_demo_write_imem");

  machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0380u));
  machine.stage_cpu_gpr(kSpMemorySourceIndex, 0x2233445566778899ull);
  step_at(machine, kStoreSpAddress, "sp_mmio_success_demo_seed_dmem_block");
  machine.stage_rdram_u32_be(kDmemBlockWriteDestination, 0xaaaaaaaau);
  machine.stage_rdram_u32_be(kDmemBlockWriteDestination + 4u, 0xbbbbbbbbu);
  machine.stage_rdram_u32_be(kDmemBlockWriteDestination + 8u, 0xccccccccu);
  machine.stage_cpu_gpr(kValueIndex, 0x00000380u);
  step_at(machine, kSwSpMemAddress, "sp_mmio_success_demo_sw_dmem_block_write_address");
  machine.stage_cpu_gpr(kValueIndex, kDmemBlockWriteDestination);
  step_at(machine, kSwDramAddress, "sp_mmio_success_demo_sw_dmem_block_write_dram");
  machine.stage_cpu_gpr(kValueIndex, kBlockSkipCommand);
  step_at(machine, kSwWriteLengthAddress, "sp_mmio_success_demo_sw_dmem_block_write_length");
  require_rdram_word_equals(
      machine,
      kDmemBlockWriteDestination,
      0x22334455u,
      "sp_mmio_success_demo_write_dmem_block_high");
  require_rdram_word_equals(
      machine,
      kDmemBlockWriteDestination + 4u,
      0xbbbbbbbbu,
      "sp_mmio_success_demo_write_dmem_block_gap");
  require_rdram_word_equals(
      machine,
      kDmemBlockWriteDestination + 8u,
      0x66778899u,
      "sp_mmio_success_demo_write_dmem_block_low");

  step_at(machine, kLwSpMemAddress, "sp_mmio_success_demo_lw_mem_address");
  require_gpr_equals(
      machine,
      kRegisterReadIndex,
      0x00000380u,
      "sp_mmio_success_demo_lw_mem_address");
  step_at(machine, kLwDramAddress, "sp_mmio_success_demo_lw_dram_address");
  require_gpr_equals(
      machine,
      kRegisterReadIndex,
      kDmemBlockWriteDestination,
      "sp_mmio_success_demo_lw_dram_address");
  step_at(machine, kLwReadLengthAddress, "sp_mmio_success_demo_lw_read_length");
  require_gpr_equals(
      machine,
      kRegisterReadIndex,
      kBlockSkipCommand,
      "sp_mmio_success_demo_lw_read_length");
  step_at(machine, kLwWriteLengthAddress, "sp_mmio_success_demo_lw_write_length");
  require_gpr_equals(
      machine,
      kRegisterReadIndex,
      kBlockSkipCommand,
      "sp_mmio_success_demo_lw_write_length");
  step_at(machine, kLwStatusAddress, "sp_mmio_success_demo_lw_status");
  require_gpr_equals(machine, kRegisterReadIndex, 0, "sp_mmio_success_demo_lw_status");
}

void run_sp_dma_failure_demo() {
  std::cout << "fn64 bootstrap SP MMIO demo: failed DMA preflight is no-ghost\n";

  constexpr std::uint32_t kSingleEightByteBlockCommand = 7u;
  constexpr std::uint32_t kTwoFourByteBlocksNoSkipCommand =
      encode_sp_dma_length_command(3u, 1u, 0u);

  const auto require_failed_sp_dma_preserves_state =
      [](const char* label,
         bool write_direction,
         std::uint32_t sp_memory_address,
         RdramOffset dram_address,
         std::uint32_t length_register_value) {
        auto machine_storage = std::make_unique<Machine>(Cartridge{});
        Machine& machine = *machine_storage;
        constexpr std::size_t kRdramBaseIndex = 4;
        constexpr std::size_t kSpBaseIndex = 5;
        constexpr std::size_t kValueIndex = 6;
        constexpr std::size_t kLlTargetIndex = 7;
        constexpr std::size_t kScSourceIndex = 8;
        constexpr std::size_t kSpMemoryBaseIndex = 9;
        constexpr std::size_t kSpMemorySourceIndex = 10;
        constexpr std::size_t kSpMemoryReadIndex = 11;
        constexpr RdramOffset kSeedSpAddress = 0x00001740u;
        constexpr RdramOffset kLoadSpAddress = 0x00001744u;
        constexpr RdramOffset kLlAddress = 0x00001748u;
        constexpr RdramOffset kSwMemAddress = 0x0000174cu;
        constexpr RdramOffset kSwDramAddress = 0x00001750u;
        constexpr RdramOffset kSwReadLengthAddress = 0x00001754u;
        constexpr RdramOffset kSwWriteLengthAddress = 0x00001758u;
        constexpr RdramOffset kScAddress = 0x0000175cu;
        constexpr RdramOffset kReservedAddress = 0x00001900u;
        constexpr RdramOffset kRdramSentinelAddress = 0x00001920u;
        constexpr CpuRegisterValue kSpSentinel = 0x1122334455667788ull;
        constexpr std::uint32_t kRdramSentinel = 0xaabbccddu;

        machine.stage_rdram_u32_be(
            kSeedSpAddress,
            encode_sd(
                static_cast<std::uint8_t>(kSpMemorySourceIndex),
                static_cast<std::uint8_t>(kSpMemoryBaseIndex),
                0));
        machine.stage_rdram_u32_be(
            kLoadSpAddress,
            encode_ld(
                static_cast<std::uint8_t>(kSpMemoryReadIndex),
                static_cast<std::uint8_t>(kSpMemoryBaseIndex),
                0));
        machine.stage_rdram_u32_be(
            kLlAddress,
            encode_ll(
                static_cast<std::uint8_t>(kLlTargetIndex),
                static_cast<std::uint8_t>(kRdramBaseIndex),
                0));
        stage_sp_sw_instruction(
            machine,
            kSwMemAddress,
            static_cast<std::uint8_t>(kValueIndex),
            static_cast<std::uint8_t>(kSpBaseIndex),
            kSpMemoryRegisterOffset);
        stage_sp_sw_instruction(
            machine,
            kSwDramAddress,
            static_cast<std::uint8_t>(kValueIndex),
            static_cast<std::uint8_t>(kSpBaseIndex),
            kSpDramRegisterOffset);
        stage_sp_sw_instruction(
            machine,
            kSwReadLengthAddress,
            static_cast<std::uint8_t>(kValueIndex),
            static_cast<std::uint8_t>(kSpBaseIndex),
            kSpReadLengthRegisterOffset);
        stage_sp_sw_instruction(
            machine,
            kSwWriteLengthAddress,
            static_cast<std::uint8_t>(kValueIndex),
            static_cast<std::uint8_t>(kSpBaseIndex),
            kSpWriteLengthRegisterOffset);
        machine.stage_rdram_u32_be(
            kScAddress,
            encode_sc(
                static_cast<std::uint8_t>(kScSourceIndex),
                static_cast<std::uint8_t>(kRdramBaseIndex),
                0));

        machine.stage_rdram_u32_be(kReservedAddress, 0x11112222u);
        machine.stage_rdram_u32_be(kRdramSentinelAddress, kRdramSentinel);
        machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0080u));
        machine.stage_cpu_gpr(kSpMemorySourceIndex, kSpSentinel);
        step_at(machine, kSeedSpAddress, label);

        machine.stage_cpu_gpr(kSpBaseIndex, kSyntheticSpMmioCpuBase);
        machine.stage_cpu_gpr(kValueIndex, sp_memory_address);
        step_at(machine, kSwMemAddress, label);
        machine.stage_cpu_gpr(kValueIndex, dram_address);
        step_at(machine, kSwDramAddress, label);
        machine.stage_cpu_gpr(kRdramBaseIndex, cpu_rdram_alias(kReservedAddress));
        step_at(machine, kLlAddress, label);

        machine.stage_cpu_gpr(kValueIndex, length_register_value);
        machine.stage_cpu_pc(
            cpu_rdram_alias(write_direction ? kSwWriteLengthAddress : kSwReadLengthAddress));
        require_step_out_of_range(machine, label);

        machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0080u));
        step_at(machine, kLoadSpAddress, label);
        require_gpr_equals(machine, kSpMemoryReadIndex, kSpSentinel, label);
        require_rdram_word_equals(machine, kRdramSentinelAddress, kRdramSentinel, label);

        machine.stage_cpu_gpr(kScSourceIndex, 0xccddeeffu);
        step_at(machine, kScAddress, label);
        require_gpr_equals(machine, kScSourceIndex, 1, label);
        require_rdram_word_equals(machine, kReservedAddress, 0xccddeeffu, label);
      };

  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_read_sp_destination_crosses_region",
      false,
      0x00000ffcu,
      0x00001940u,
      kSingleEightByteBlockCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_read_sp_destination_out_of_range",
      false,
      0x00002000u,
      0x00001940u,
      kSingleEightByteBlockCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_read_later_sp_block_crosses_region",
      false,
      0x00000ffcu,
      0x00001940u,
      kTwoFourByteBlocksNoSkipCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_read_rdram_source_out_of_range",
      false,
      0x00000100u,
      0x003ffffcu,
      kSingleEightByteBlockCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_read_later_rdram_block_out_of_range",
      false,
      0x00000100u,
      0x003ffffcu,
      kTwoFourByteBlocksNoSkipCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_write_sp_source_crosses_region",
      true,
      0x00000ffcu,
      0x00001940u,
      kSingleEightByteBlockCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_write_rdram_destination_out_of_range",
      true,
      0x00000100u,
      0x003ffffcu,
      kSingleEightByteBlockCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_write_later_rdram_block_out_of_range",
      true,
      0x00000100u,
      0x003ffffcu,
      kTwoFourByteBlocksNoSkipCommand);
  require_failed_sp_dma_preserves_state(
      "sp_dma_failure_demo_large_command_span_out_of_range",
      false,
      0x00000100u,
      0x00001940u,
      0xffffffffu);
}

void run_sp_dma_reservation_demo() {
  std::cout << "fn64 bootstrap SP MMIO demo: DMA preserves the local RDRAM reservation domain\n";

  constexpr std::uint32_t kBlockSkipCommand =
      encode_sp_dma_length_command(3u, 1u, 4u);

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kRdramBaseIndex = 4;
    constexpr std::size_t kSpBaseIndex = 5;
    constexpr std::size_t kValueIndex = 6;
    constexpr std::size_t kLlTargetIndex = 7;
    constexpr std::size_t kScSourceIndex = 8;
    constexpr std::size_t kSpMemoryBaseIndex = 9;
    constexpr std::size_t kSpMemorySourceIndex = 10;
    constexpr RdramOffset kSeedSpAddress = 0x00001780u;
    constexpr RdramOffset kLlAddress = 0x00001784u;
    constexpr RdramOffset kSwMemAddress = 0x00001788u;
    constexpr RdramOffset kSwDramAddress = 0x0000178cu;
    constexpr RdramOffset kSwWriteLengthAddress = 0x00001790u;
    constexpr RdramOffset kScAddress = 0x00001794u;
    constexpr RdramOffset kReservedAddress = 0x00001980u;

    machine.stage_rdram_u32_be(
        kSeedSpAddress,
        encode_sd(
            static_cast<std::uint8_t>(kSpMemorySourceIndex),
            static_cast<std::uint8_t>(kSpMemoryBaseIndex),
            0));
    machine.stage_rdram_u32_be(
        kLlAddress,
        encode_ll(
            static_cast<std::uint8_t>(kLlTargetIndex),
            static_cast<std::uint8_t>(kRdramBaseIndex),
            0));
    stage_sp_sw_instruction(
        machine,
        kSwMemAddress,
        static_cast<std::uint8_t>(kValueIndex),
        static_cast<std::uint8_t>(kSpBaseIndex),
        kSpMemoryRegisterOffset);
    stage_sp_sw_instruction(
        machine,
        kSwDramAddress,
        static_cast<std::uint8_t>(kValueIndex),
        static_cast<std::uint8_t>(kSpBaseIndex),
        kSpDramRegisterOffset);
    stage_sp_sw_instruction(
        machine,
        kSwWriteLengthAddress,
        static_cast<std::uint8_t>(kValueIndex),
        static_cast<std::uint8_t>(kSpBaseIndex),
        kSpWriteLengthRegisterOffset);
    machine.stage_rdram_u32_be(
        kScAddress,
        encode_sc(
            static_cast<std::uint8_t>(kScSourceIndex),
            static_cast<std::uint8_t>(kRdramBaseIndex),
            0));

    machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0200u));
    machine.stage_cpu_gpr(kSpMemorySourceIndex, 0x0102030405060708ull);
    step_at(machine, kSeedSpAddress, "sp_dma_reservation_demo_seed_dmem");

    machine.stage_cpu_gpr(kRdramBaseIndex, cpu_rdram_alias(kReservedAddress));
    machine.stage_rdram_u32_be(kReservedAddress, 0x11112222u);
    step_at(machine, kLlAddress, "sp_dma_reservation_demo_overlap_ll");

    machine.stage_cpu_gpr(kSpBaseIndex, kSyntheticSpMmioCpuBase);
    machine.stage_cpu_gpr(kValueIndex, 0x00000200u);
    step_at(machine, kSwMemAddress, "sp_dma_reservation_demo_overlap_sw_mem");
    machine.stage_cpu_gpr(kValueIndex, kReservedAddress);
    step_at(machine, kSwDramAddress, "sp_dma_reservation_demo_overlap_sw_dram");
    machine.stage_cpu_gpr(kValueIndex, kBlockSkipCommand);
    step_at(machine, kSwWriteLengthAddress, "sp_dma_reservation_demo_overlap_sw_length");

    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);
    step_at(machine, kScAddress, "sp_dma_reservation_demo_overlap_sc");
    require_gpr_equals(machine, kScSourceIndex, 0, "sp_dma_reservation_demo_overlap_sc");
    require_rdram_word_equals(
        machine,
        kReservedAddress,
        0x01020304u,
        "sp_dma_reservation_demo_overlap_sc_high");
    require_rdram_word_equals(
        machine,
        kReservedAddress + 8u,
        0x05060708u,
        "sp_dma_reservation_demo_overlap_sc_low");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kRdramBaseIndex = 4;
    constexpr std::size_t kSpBaseIndex = 5;
    constexpr std::size_t kValueIndex = 6;
    constexpr std::size_t kLlTargetIndex = 7;
    constexpr std::size_t kScSourceIndex = 8;
    constexpr std::size_t kSpMemoryBaseIndex = 9;
    constexpr std::size_t kSpMemoryReadIndex = 10;
    constexpr RdramOffset kLlAddress = 0x000017c0u;
    constexpr RdramOffset kSwMemAddress = 0x000017c4u;
    constexpr RdramOffset kSwDramAddress = 0x000017c8u;
    constexpr RdramOffset kSwReadLengthAddress = 0x000017ccu;
    constexpr RdramOffset kLoadSpAddress = 0x000017d0u;
    constexpr RdramOffset kScAddress = 0x000017d4u;
    constexpr RdramOffset kReservedAddress = 0x000019c0u;
    constexpr CpuRegisterValue kReservedInitial = 0x1122334455667788ull;

    machine.stage_rdram_u32_be(
        kLlAddress,
        encode_ll(
            static_cast<std::uint8_t>(kLlTargetIndex),
            static_cast<std::uint8_t>(kRdramBaseIndex),
            0));
    stage_sp_sw_instruction(
        machine,
        kSwMemAddress,
        static_cast<std::uint8_t>(kValueIndex),
        static_cast<std::uint8_t>(kSpBaseIndex),
        kSpMemoryRegisterOffset);
    stage_sp_sw_instruction(
        machine,
        kSwDramAddress,
        static_cast<std::uint8_t>(kValueIndex),
        static_cast<std::uint8_t>(kSpBaseIndex),
        kSpDramRegisterOffset);
    stage_sp_sw_instruction(
        machine,
        kSwReadLengthAddress,
        static_cast<std::uint8_t>(kValueIndex),
        static_cast<std::uint8_t>(kSpBaseIndex),
        kSpReadLengthRegisterOffset);
    machine.stage_rdram_u32_be(
        kLoadSpAddress,
        encode_ld(
            static_cast<std::uint8_t>(kSpMemoryReadIndex),
            static_cast<std::uint8_t>(kSpMemoryBaseIndex),
            0));
    machine.stage_rdram_u32_be(
        kScAddress,
        encode_sc(
            static_cast<std::uint8_t>(kScSourceIndex),
            static_cast<std::uint8_t>(kRdramBaseIndex),
            0));
    stage_rdram_u64_be(machine, kReservedAddress, kReservedInitial);

    machine.stage_cpu_gpr(kRdramBaseIndex, cpu_rdram_alias(kReservedAddress));
    step_at(machine, kLlAddress, "sp_dma_reservation_demo_read_preserve_ll");

    machine.stage_cpu_gpr(kSpBaseIndex, kSyntheticSpMmioCpuBase);
    machine.stage_cpu_gpr(kValueIndex, 0x00000240u);
    step_at(machine, kSwMemAddress, "sp_dma_reservation_demo_read_sw_mem");
    machine.stage_cpu_gpr(kValueIndex, kReservedAddress);
    step_at(machine, kSwDramAddress, "sp_dma_reservation_demo_read_sw_dram");
    machine.stage_cpu_gpr(kValueIndex, 7u);
    step_at(machine, kSwReadLengthAddress, "sp_dma_reservation_demo_read_sw_length");

    machine.stage_cpu_gpr(kSpMemoryBaseIndex, sp_dmem_uncached_alias(0x0240u));
    step_at(machine, kLoadSpAddress, "sp_dma_reservation_demo_read_load_sp");
    require_gpr_equals(
        machine,
        kSpMemoryReadIndex,
        kReservedInitial,
        "sp_dma_reservation_demo_read_load_sp");

    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);
    step_at(machine, kScAddress, "sp_dma_reservation_demo_read_sc");
    require_gpr_equals(machine, kScSourceIndex, 1, "sp_dma_reservation_demo_read_sc");
    require_rdram_word_equals(
        machine,
        kReservedAddress,
        0xaabbccddu,
        "sp_dma_reservation_demo_read_sc");
  }
}

void run_sp_mmio_fault_demo() {
  std::cout << "fn64 bootstrap SP MMIO demo: unsupported widths, atomics, and fetch stay local faults\n";

  const auto require_sp_mmio_machine_fault =
      [](const char* label,
         CpuInstructionWord instruction,
         MachineFaultKind expected_kind,
         std::size_t expected_access_size) {
        auto machine_storage = std::make_unique<Machine>(Cartridge{});
        Machine& machine = *machine_storage;
        constexpr std::size_t kBaseIndex = 4;
        constexpr std::size_t kSourceIndex = 5;
        constexpr std::size_t kTargetIndex = 6;
        constexpr RdramOffset kInstructionAddress = 0x000017f0u;
        constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;

        machine.stage_rdram_u32_be(kInstructionAddress, instruction);
        machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
        machine.stage_cpu_next_pc(cpu_rdram_alias(kInstructionAddress + 4u));
        machine.stage_cpu_gpr(kBaseIndex, kSyntheticSpMmioCpuBase);
        machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddeeff0011ull);
        machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

        require_step_machine_fault(machine, label, expected_kind, expected_access_size);

        if (machine.cpu_pc() != cpu_rdram_alias(kInstructionAddress) ||
            machine.cpu_next_pc() != cpu_rdram_alias(kInstructionAddress + 4u)) {
          throw std::runtime_error(std::string(label) + " changed pc/next_pc on fault");
        }

        require_gpr_equals(machine, kTargetIndex, kTargetSentinel, label);
      };

  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_sb_unsupported_width",
      encode_sb(5, 4, kSpMemoryRegisterOffset),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      1);
  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_ld_unsupported_width",
      encode_ld(6, 4, kSpMemoryRegisterOffset),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      8);
  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_unknown_register",
      encode_lw(6, 4, kSpUnknownRegisterOffset),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      4);
  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_unaligned_sw",
      encode_sw(5, 4, 2),
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      4);
  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_ll_rejected",
      encode_ll(6, 4, kSpMemoryRegisterOffset),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      4);
  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_lld_rejected",
      encode_lld(6, 4, kSpMemoryRegisterOffset),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      8);
  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_sc_rejected",
      encode_sc(5, 4, kSpMemoryRegisterOffset),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      4);
  require_sp_mmio_machine_fault(
      "sp_mmio_fault_demo_scd_rejected",
      encode_scd(5, 4, kSpMemoryRegisterOffset),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      8);

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    machine.stage_cpu_pc(kSyntheticSpMmioCpuBase);
    machine.stage_cpu_next_pc(kSyntheticSpMmioCpuBase + 4u);
    require_step_machine_fault(
        machine,
        "sp_mmio_fault_demo_fetch_rejected",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);
  }
}

void run_sp_memory_data_demo() {
  std::cout << "fn64 bootstrap SP memory demo: CPU data reaches local DMEM/IMEM only\n";

  const auto stage_next =
      [](Machine& machine,
         RdramOffset& instruction_address,
         CpuInstructionWord instruction,
         const char* label) {
        machine.stage_rdram_u32_be(instruction_address, instruction);
        step_at(machine, instruction_address, label);
        instruction_address += 4u;
      };

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kSourceIndex = 5;
    constexpr std::size_t kTargetIndex = 6;
    constexpr RdramOffset kInstructionBase = 0x00001400u;
    RdramOffset instruction_address = kInstructionBase;

    machine.stage_cpu_gpr(kBaseIndex, sp_dmem_uncached_alias(0));

    machine.stage_cpu_gpr(kSourceIndex, 0x11223344556677abull);
    stage_next(machine, instruction_address, encode_sb(5, 4, 0x0010u), "sp_memory_demo_sb_dmem");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0010u), "sp_memory_demo_lbu_dmem");
    require_gpr_equals(machine, kTargetIndex, 0xabu, "sp_memory_demo_lbu_dmem");

    machine.stage_cpu_gpr(kSourceIndex, 0x112233445555beefull);
    stage_next(machine, instruction_address, encode_sh(5, 4, 0x0020u), "sp_memory_demo_sh_dmem");
    stage_next(machine, instruction_address, encode_lhu(6, 4, 0x0020u), "sp_memory_demo_lhu_dmem");
    require_gpr_equals(machine, kTargetIndex, 0xbeefu, "sp_memory_demo_lhu_dmem");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0020u), "sp_memory_demo_lhu_dmem_high_byte");
    require_gpr_equals(machine, kTargetIndex, 0xbeu, "sp_memory_demo_lhu_dmem_high_byte");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0021u), "sp_memory_demo_lhu_dmem_low_byte");
    require_gpr_equals(machine, kTargetIndex, 0xefu, "sp_memory_demo_lhu_dmem_low_byte");

    machine.stage_cpu_gpr(kSourceIndex, 0x11223344u);
    stage_next(machine, instruction_address, encode_sw(5, 4, 0x0030u), "sp_memory_demo_sw_dmem");
    stage_next(machine, instruction_address, encode_lwu(6, 4, 0x0030u), "sp_memory_demo_lwu_dmem");
    require_gpr_equals(machine, kTargetIndex, 0x11223344u, "sp_memory_demo_lwu_dmem");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0030u), "sp_memory_demo_lwu_dmem_high_byte");
    require_gpr_equals(machine, kTargetIndex, 0x11u, "sp_memory_demo_lwu_dmem_high_byte");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0033u), "sp_memory_demo_lwu_dmem_low_byte");
    require_gpr_equals(machine, kTargetIndex, 0x44u, "sp_memory_demo_lwu_dmem_low_byte");

    machine.stage_cpu_gpr(kSourceIndex, 0x0102030405060708ull);
    stage_next(machine, instruction_address, encode_sd(5, 4, 0x0040u), "sp_memory_demo_sd_dmem");
    stage_next(machine, instruction_address, encode_ld(6, 4, 0x0040u), "sp_memory_demo_ld_dmem");
    require_gpr_equals(machine, kTargetIndex, 0x0102030405060708ull, "sp_memory_demo_ld_dmem");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0040u), "sp_memory_demo_ld_dmem_high_byte");
    require_gpr_equals(machine, kTargetIndex, 0x01u, "sp_memory_demo_ld_dmem_high_byte");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0047u), "sp_memory_demo_ld_dmem_low_byte");
    require_gpr_equals(machine, kTargetIndex, 0x08u, "sp_memory_demo_ld_dmem_low_byte");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kSourceIndex = 5;
    constexpr std::size_t kTargetIndex = 6;
    constexpr RdramOffset kInstructionBase = 0x00001480u;
    RdramOffset instruction_address = kInstructionBase;

    machine.stage_cpu_gpr(kBaseIndex, sp_dmem_cached_alias(0));

    machine.stage_cpu_gpr(kSourceIndex, 0x000000ffu);
    stage_next(machine, instruction_address, encode_sb(5, 4, 0x0fffu), "sp_memory_demo_last_byte_store");
    stage_next(machine, instruction_address, encode_lbu(6, 4, 0x0fffu), "sp_memory_demo_last_byte_load");
    require_gpr_equals(machine, kTargetIndex, 0xffu, "sp_memory_demo_last_byte_load");

    machine.stage_cpu_gpr(kSourceIndex, 0x0000abcdu);
    stage_next(machine, instruction_address, encode_sh(5, 4, 0x0ffeu), "sp_memory_demo_last_half_store");
    stage_next(machine, instruction_address, encode_lhu(6, 4, 0x0ffeu), "sp_memory_demo_last_half_load");
    require_gpr_equals(machine, kTargetIndex, 0xabcdu, "sp_memory_demo_last_half_load");

    machine.stage_cpu_gpr(kSourceIndex, 0x89abcdefu);
    stage_next(machine, instruction_address, encode_sw(5, 4, 0x0ffcu), "sp_memory_demo_last_word_store");
    stage_next(machine, instruction_address, encode_lwu(6, 4, 0x0ffcu), "sp_memory_demo_last_word_load");
    require_gpr_equals(machine, kTargetIndex, 0x89abcdefu, "sp_memory_demo_last_word_load");

    machine.stage_cpu_gpr(kSourceIndex, 0x1122334455667788ull);
    stage_next(machine, instruction_address, encode_sd(5, 4, 0x0ff8u), "sp_memory_demo_last_doubleword_store");
    stage_next(machine, instruction_address, encode_ld(6, 4, 0x0ff8u), "sp_memory_demo_last_doubleword_load");
    require_gpr_equals(machine, kTargetIndex, 0x1122334455667788ull, "sp_memory_demo_last_doubleword_load");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kDmemBaseIndex = 4;
    constexpr std::size_t kImemBaseIndex = 5;
    constexpr std::size_t kSourceIndex = 6;
    constexpr std::size_t kDmemTargetIndex = 7;
    constexpr std::size_t kImemTargetIndex = 8;
    constexpr RdramOffset kInstructionBase = 0x00001500u;
    RdramOffset instruction_address = kInstructionBase;

    machine.stage_cpu_gpr(kDmemBaseIndex, sp_dmem_cached_alias(0));
    machine.stage_cpu_gpr(kImemBaseIndex, sp_imem_cached_alias(0));

    machine.stage_cpu_gpr(kSourceIndex, 0x11223344u);
    stage_next(machine, instruction_address, encode_sw(6, 4, 0), "sp_memory_demo_dmem_alias_sw");
    machine.stage_cpu_gpr(kDmemBaseIndex, sp_dmem_uncached_alias(0));
    stage_next(machine, instruction_address, encode_lwu(7, 4, 0), "sp_memory_demo_dmem_alias_lwu");
    require_gpr_equals(machine, kDmemTargetIndex, 0x11223344u, "sp_memory_demo_dmem_alias_lwu");

    machine.stage_cpu_gpr(kSourceIndex, 0x55667788u);
    stage_next(machine, instruction_address, encode_sw(6, 5, 0), "sp_memory_demo_imem_sw");
    machine.stage_cpu_gpr(kImemBaseIndex, sp_imem_uncached_alias(0));
    stage_next(machine, instruction_address, encode_lwu(8, 5, 0), "sp_memory_demo_imem_lwu");
    require_gpr_equals(machine, kImemTargetIndex, 0x55667788u, "sp_memory_demo_imem_lwu");

    stage_next(machine, instruction_address, encode_lwu(7, 4, 0), "sp_memory_demo_dmem_independent_lwu");
    require_gpr_equals(machine, kDmemTargetIndex, 0x11223344u, "sp_memory_demo_dmem_independent_lwu");

    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddeeff0011ull);
    stage_next(machine, instruction_address, encode_sd(6, 5, 0x0008u), "sp_memory_demo_imem_sd");
    stage_next(machine, instruction_address, encode_ld(8, 5, 0x0008u), "sp_memory_demo_imem_ld");
    require_gpr_equals(machine, kImemTargetIndex, 0xaabbccddeeff0011ull, "sp_memory_demo_imem_ld");
  }

  const auto require_sp_machine_fault =
      [](const char* label,
         CpuInstructionWord instruction,
         CpuAddress base_value,
         MachineFaultKind expected_kind,
         std::size_t expected_access_size) {
        auto machine_storage = std::make_unique<Machine>(Cartridge{});
        Machine& machine = *machine_storage;
        constexpr std::size_t kBaseIndex = 4;
        constexpr std::size_t kSourceIndex = 5;
        constexpr std::size_t kTargetIndex = 6;
        constexpr RdramOffset kInstructionAddress = 0x00001580u;
        constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;

        machine.stage_rdram_u32_be(kInstructionAddress, instruction);
        machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
        machine.stage_cpu_next_pc(cpu_rdram_alias(kInstructionAddress + 4u));
        machine.stage_cpu_gpr(kBaseIndex, base_value);
        machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddeeff0011ull);
        machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);

        require_step_machine_fault(machine, label, expected_kind, expected_access_size);

        if (machine.cpu_pc() != cpu_rdram_alias(kInstructionAddress) ||
            machine.cpu_next_pc() != cpu_rdram_alias(kInstructionAddress + 4u)) {
          throw std::runtime_error(std::string(label) + " changed pc/next_pc on fault");
        }

        require_gpr_equals(machine, kTargetIndex, kTargetSentinel, label);
      };

  require_sp_machine_fault(
      "sp_memory_demo_sp_register_outside_window_rejected",
      encode_lw(6, 4, 0),
      kSyntheticSpOutsideRegisterWindow,
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
  require_sp_machine_fault(
      "sp_memory_demo_outside_sp_ranges_rejected",
      encode_lw(6, 4, 0),
      0xa4002000u,
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
  require_sp_machine_fault(
      "sp_memory_demo_ll_to_sp_rejected",
      encode_ll(6, 4, 0),
      sp_dmem_uncached_alias(0),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      4);
  require_sp_machine_fault(
      "sp_memory_demo_lld_to_sp_rejected",
      encode_lld(6, 4, 0),
      sp_imem_uncached_alias(0),
      MachineFaultKind::kUnsupportedCpuDataAccess,
      8);

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kSourceIndex = 5;
    constexpr std::size_t kTargetIndex = 6;
    constexpr RdramOffset kSwAddress = 0x000015a0u;
    constexpr RdramOffset kBadSwAddress = 0x000015a4u;
    constexpr RdramOffset kLwAddress = 0x000015a8u;

    machine.stage_rdram_u32_be(kSwAddress, encode_sw(5, 4, 0));
    machine.stage_rdram_u32_be(kBadSwAddress, encode_sw(5, 4, 0));
    machine.stage_rdram_u32_be(kLwAddress, encode_lwu(6, 4, 0));

    machine.stage_cpu_gpr(kBaseIndex, sp_dmem_uncached_alias(0x0080u));
    machine.stage_cpu_gpr(kSourceIndex, 0x11223344u);
    step_at(machine, kSwAddress, "sp_memory_demo_failed_store_seed");

    machine.stage_cpu_gpr(kBaseIndex, kSyntheticSpOutsideRegisterWindow);
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddu);
    machine.stage_cpu_pc(cpu_rdram_alias(kBadSwAddress));
    machine.stage_cpu_next_pc(cpu_rdram_alias(kBadSwAddress + 4u));
    require_step_machine_fault(
        machine,
        "sp_memory_demo_failed_store_no_ghost",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);

    machine.stage_cpu_gpr(kBaseIndex, sp_dmem_uncached_alias(0x0080u));
    step_at(machine, kLwAddress, "sp_memory_demo_failed_store_no_ghost_lwu");
    require_gpr_equals(machine, kTargetIndex, 0x11223344u, "sp_memory_demo_failed_store_no_ghost_lwu");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kRdramBaseIndex = 4;
    constexpr std::size_t kSpBaseIndex = 5;
    constexpr std::size_t kLlTargetIndex = 6;
    constexpr std::size_t kStoreSourceIndex = 7;
    constexpr std::size_t kScSourceIndex = 8;
    constexpr std::size_t kSpReadIndex = 9;
    constexpr RdramOffset kLlAddress = 0x000015c0u;
    constexpr RdramOffset kSwSpAddress = 0x000015c4u;
    constexpr RdramOffset kLwuSpAddress = 0x000015c8u;
    constexpr RdramOffset kScAddress = 0x000015ccu;
    constexpr RdramOffset kReservedAddress = 0x00001680u;

    machine.stage_rdram_u32_be(
        kLlAddress,
        encode_ll(
            static_cast<std::uint8_t>(kLlTargetIndex),
            static_cast<std::uint8_t>(kRdramBaseIndex),
            0));
    machine.stage_rdram_u32_be(kSwSpAddress, encode_sw(7, 5, 0));
    machine.stage_rdram_u32_be(kLwuSpAddress, encode_lwu(9, 5, 0));
    machine.stage_rdram_u32_be(kScAddress, encode_sc(8, 4, 0));
    machine.stage_rdram_u32_be(kReservedAddress, 0x11112222u);
    machine.stage_cpu_gpr(kRdramBaseIndex, cpu_rdram_alias(kReservedAddress));
    machine.stage_cpu_gpr(kSpBaseIndex, sp_dmem_uncached_alias(0x0100u));

    step_at(machine, kLlAddress, "sp_memory_demo_reservation_ll");
    machine.stage_cpu_gpr(kStoreSourceIndex, 0x55667788u);
    step_at(machine, kSwSpAddress, "sp_memory_demo_reservation_sp_sw");
    step_at(machine, kLwuSpAddress, "sp_memory_demo_reservation_sp_lwu");
    require_gpr_equals(machine, kSpReadIndex, 0x55667788u, "sp_memory_demo_reservation_sp_lwu");

    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);
    step_at(machine, kScAddress, "sp_memory_demo_reservation_sc");

    require_gpr_equals(machine, kScSourceIndex, 1, "sp_memory_demo_reservation_sc");
    require_rdram_word_equals(machine, kReservedAddress, 0xaabbccddu, "sp_memory_demo_reservation_sc");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kSpBaseIndex = 4;
    constexpr std::size_t kRdramBaseIndex = 5;
    constexpr std::size_t kLlTargetIndex = 6;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlSpAddress = 0x00001600u;
    constexpr RdramOffset kScAddress = 0x00001604u;
    constexpr RdramOffset kDataAddress = 0x000016c0u;
    constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;

    machine.stage_rdram_u32_be(kLlSpAddress, encode_ll(6, 4, 0));
    machine.stage_rdram_u32_be(kScAddress, encode_sc(7, 5, 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kSpBaseIndex, sp_dmem_uncached_alias(0));
    machine.stage_cpu_gpr(kRdramBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kLlTargetIndex, kTargetSentinel);
    machine.stage_cpu_pc(cpu_rdram_alias(kLlSpAddress));
    machine.stage_cpu_next_pc(cpu_rdram_alias(kLlSpAddress + 4u));
    require_step_machine_fault(
        machine,
        "sp_memory_demo_ll_rejects_without_reservation",
        MachineFaultKind::kUnsupportedCpuDataAccess,
        4);
    require_gpr_equals(
        machine,
        kLlTargetIndex,
        kTargetSentinel,
        "sp_memory_demo_ll_rejects_without_reservation");

    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);
    step_at(machine, kScAddress, "sp_memory_demo_ll_rejects_then_sc_fails");
    require_gpr_equals(machine, kScSourceIndex, 0, "sp_memory_demo_ll_rejects_then_sc_fails");
    require_rdram_word_equals(machine, kDataAddress, 0x11112222u, "sp_memory_demo_ll_rejects_then_sc_fails");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kSourceIndex = 5;
    constexpr std::size_t kTargetIndex = 6;
    constexpr RdramOffset kSeedSwAddress = 0x00001620u;
    constexpr RdramOffset kBadScAddress = 0x00001624u;
    constexpr RdramOffset kReadLwuAddress = 0x00001628u;
    constexpr RdramOffset kSeedSdAddress = 0x0000162cu;
    constexpr RdramOffset kBadScdAddress = 0x00001630u;
    constexpr RdramOffset kReadLdAddress = 0x00001634u;

    machine.stage_rdram_u32_be(kSeedSwAddress, encode_sw(5, 4, 0x0120u));
    machine.stage_rdram_u32_be(kBadScAddress, encode_sc(5, 4, 0x0120u));
    machine.stage_rdram_u32_be(kReadLwuAddress, encode_lwu(6, 4, 0x0120u));
    machine.stage_rdram_u32_be(kSeedSdAddress, encode_sd(5, 4, 0x0130u));
    machine.stage_rdram_u32_be(kBadScdAddress, encode_scd(5, 4, 0x0130u));
    machine.stage_rdram_u32_be(kReadLdAddress, encode_ld(6, 4, 0x0130u));

    machine.stage_cpu_gpr(kBaseIndex, sp_dmem_uncached_alias(0));
    machine.stage_cpu_gpr(kSourceIndex, 0x11223344u);
    step_at(machine, kSeedSwAddress, "sp_memory_demo_sc_rejected_seed");
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddu);
    machine.stage_cpu_pc(cpu_rdram_alias(kBadScAddress));
    machine.stage_cpu_next_pc(cpu_rdram_alias(kBadScAddress + 4u));
    require_step_machine_fault(
        machine,
        "sp_memory_demo_sc_to_sp_rejected",
        MachineFaultKind::kUnsupportedCpuDataAccess,
        4);
    step_at(machine, kReadLwuAddress, "sp_memory_demo_sc_to_sp_rejected_lwu");
    require_gpr_equals(machine, kTargetIndex, 0x11223344u, "sp_memory_demo_sc_to_sp_rejected_lwu");

    machine.stage_cpu_gpr(kSourceIndex, 0x0102030405060708ull);
    step_at(machine, kSeedSdAddress, "sp_memory_demo_scd_rejected_seed");
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddeeff0011ull);
    machine.stage_cpu_pc(cpu_rdram_alias(kBadScdAddress));
    machine.stage_cpu_next_pc(cpu_rdram_alias(kBadScdAddress + 4u));
    require_step_machine_fault(
        machine,
        "sp_memory_demo_scd_to_sp_rejected",
        MachineFaultKind::kUnsupportedCpuDataAccess,
        8);
    step_at(machine, kReadLdAddress, "sp_memory_demo_scd_to_sp_rejected_ld");
    require_gpr_equals(
        machine,
        kTargetIndex,
        0x0102030405060708ull,
        "sp_memory_demo_scd_to_sp_rejected_ld");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    machine.stage_cpu_pc(sp_dmem_uncached_alias(0));
    machine.stage_cpu_next_pc(sp_dmem_uncached_alias(4));
    require_step_machine_fault(
        machine,
        "sp_memory_demo_dmem_fetch_rejected",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    machine.stage_cpu_pc(sp_imem_uncached_alias(0));
    machine.stage_cpu_next_pc(sp_imem_uncached_alias(4));
    require_step_machine_fault(
        machine,
        "sp_memory_demo_imem_fetch_rejected",
        MachineFaultKind::kCpuRdramAddressRejected,
        4);
  }
}

void run_load_link_store_conditional_success_demo() {
  std::cout << "fn64 bootstrap LL/SC demo: local single-Machine RDRAM reservation success paths\n";

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kResultIndex = 5;
    constexpr RdramOffset kLlAddress = 0x00000a00u;
    constexpr RdramOffset kScAddress = 0x00000a04u;
    constexpr RdramOffset kDataAddress = 0x00000a80u;
    constexpr std::uint32_t kInitialWord = 0x8000007fu;
    constexpr CpuRegisterValue kScSource = 0x11223344aabbccddu;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kResultIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kResultIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, kInitialWord);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));

    step_at(machine, kLlAddress, "ll_sc_demo_ll");
    require_gpr_equals(
        machine,
        kResultIndex,
        cpu_value_from_sign_extended_u32(kInitialWord),
        "ll_sc_demo_ll");

    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_uncached_alias(kDataAddress));
    machine.stage_cpu_gpr(kResultIndex, kScSource);
    step_at(machine, kScAddress, "ll_sc_demo_sc_alias_success");

    require_gpr_equals(machine, kResultIndex, 1, "ll_sc_demo_sc_alias_success");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        static_cast<std::uint32_t>(kScSource),
        "ll_sc_demo_sc_alias_success");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kResultIndex = 6;
    constexpr RdramOffset kLldAddress = 0x00000a20u;
    constexpr RdramOffset kScdAddress = 0x00000a24u;
    constexpr RdramOffset kDataAddress = 0x00000ac0u;
    constexpr CpuRegisterValue kInitialValue = 0x1122334455667788ull;
    constexpr CpuRegisterValue kScdSource = 0xaabbccddeeff0011ull;

    stage_atomic_instruction(
        machine,
        kLldAddress,
        encode_lld(static_cast<std::uint8_t>(kResultIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScdAddress,
        encode_scd(static_cast<std::uint8_t>(kResultIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, kInitialValue);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));

    step_at(machine, kLldAddress, "ll_sc_demo_lld");
    require_gpr_equals(machine, kResultIndex, kInitialValue, "ll_sc_demo_lld");

    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_uncached_alias(kDataAddress));
    machine.stage_cpu_gpr(kResultIndex, kScdSource);
    step_at(machine, kScdAddress, "ll_sc_demo_scd_alias_success");

    require_gpr_equals(machine, kResultIndex, 1, "ll_sc_demo_scd_alias_success");
    require_rdram_doubleword_equals(
        machine,
        kDataAddress,
        kScdSource,
        "ll_sc_demo_scd_alias_success");
  }
}

void run_store_conditional_failure_demo() {
  std::cout << "fn64 bootstrap LL/SC demo: reservation miss and width mismatch paths\n";

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kSourceIndex = 7;
    constexpr RdramOffset kScAddress = 0x00000b00u;
    constexpr RdramOffset kDataAddress = 0x00000b80u;
    constexpr std::uint32_t kInitialWord = 0x11112222u;

    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, kInitialWord);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddu);
    step_at(machine, kScAddress, "sc_failure_demo_no_reservation");

    require_gpr_equals(machine, kSourceIndex, 0, "sc_failure_demo_no_reservation");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        kInitialWord,
        "sc_failure_demo_no_reservation");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kSourceIndex = 7;
    constexpr RdramOffset kScdAddress = 0x00000b20u;
    constexpr RdramOffset kDataAddress = 0x00000bc0u;
    constexpr CpuRegisterValue kInitialValue = 0x0102030405060708ull;

    stage_atomic_instruction(
        machine,
        kScdAddress,
        encode_scd(static_cast<std::uint8_t>(kSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, kInitialValue);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddeeff0011ull);
    step_at(machine, kScdAddress, "sc_failure_demo_scd_no_reservation");

    require_gpr_equals(machine, kSourceIndex, 0, "sc_failure_demo_scd_no_reservation");
    require_rdram_doubleword_equals(
        machine,
        kDataAddress,
        kInitialValue,
        "sc_failure_demo_scd_no_reservation");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr std::size_t kSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00000b40u;
    constexpr RdramOffset kScAddress = 0x00000b44u;
    constexpr RdramOffset kDataAddress = 0x00000c00u;
    constexpr std::uint32_t kSecondWord = 0x33334444u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 4));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_rdram_u32_be(kDataAddress + 4u, kSecondWord);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddu);

    step_at(machine, kLlAddress, "sc_failure_demo_ll");
    step_at(machine, kScAddress, "sc_failure_demo_different_offset");

    require_gpr_equals(machine, kSourceIndex, 0, "sc_failure_demo_different_offset");
    require_rdram_word_equals(
        machine,
        kDataAddress + 4u,
        kSecondWord,
        "sc_failure_demo_different_offset");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr std::size_t kSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00000b60u;
    constexpr RdramOffset kScdAddress = 0x00000b64u;
    constexpr RdramOffset kDataAddress = 0x00000c40u;
    constexpr CpuRegisterValue kInitialValue = 0x0102030405060708ull;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScdAddress,
        encode_scd(static_cast<std::uint8_t>(kSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, kInitialValue);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddeeff0011ull);

    step_at(machine, kLlAddress, "sc_failure_demo_width_ll");
    step_at(machine, kScdAddress, "sc_failure_demo_ll_then_scd");

    require_gpr_equals(machine, kSourceIndex, 0, "sc_failure_demo_ll_then_scd");
    require_rdram_doubleword_equals(machine, kDataAddress, kInitialValue, "sc_failure_demo_ll_then_scd");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLldTargetIndex = 5;
    constexpr std::size_t kSourceIndex = 7;
    constexpr RdramOffset kLldAddress = 0x00000b80u;
    constexpr RdramOffset kScAddress = 0x00000b84u;
    constexpr RdramOffset kDataAddress = 0x00000c80u;
    constexpr std::uint32_t kInitialWord = 0x01020304u;

    stage_atomic_instruction(
        machine,
        kLldAddress,
        encode_lld(static_cast<std::uint8_t>(kLldTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, 0x0102030405060708ull);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kSourceIndex, 0xaabbccddu);

    step_at(machine, kLldAddress, "sc_failure_demo_width_lld");
    step_at(machine, kScAddress, "sc_failure_demo_lld_then_sc");

    require_gpr_equals(machine, kSourceIndex, 0, "sc_failure_demo_lld_then_sc");
    require_rdram_word_equals(machine, kDataAddress, kInitialWord, "sc_failure_demo_lld_then_sc");
  }
}

void run_store_conditional_invalidation_demo() {
  std::cout << "fn64 bootstrap LL/SC demo: overlapping RDRAM writes invalidate reservations\n";

  const auto require_word_store_invalidates =
      [](const char* label,
         CpuInstructionWord store_instruction,
         CpuRegisterValue store_source,
         std::uint32_t expected_after_store) {
        auto machine_storage = std::make_unique<Machine>(Cartridge{});
        Machine& machine = *machine_storage;
        constexpr std::size_t kBaseIndex = 4;
        constexpr std::size_t kLlTargetIndex = 5;
        constexpr std::size_t kStoreSourceIndex = 6;
        constexpr std::size_t kScSourceIndex = 7;
        constexpr RdramOffset kLlAddress = 0x00000c00u;
        constexpr RdramOffset kStoreAddress = 0x00000c04u;
        constexpr RdramOffset kScAddress = 0x00000c08u;
        constexpr RdramOffset kDataAddress = 0x00000d00u;

        stage_atomic_instruction(
            machine,
            kLlAddress,
            encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
        stage_atomic_instruction(machine, kStoreAddress, store_instruction);
        stage_atomic_instruction(
            machine,
            kScAddress,
            encode_sc(static_cast<std::uint8_t>(kScSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
        machine.stage_rdram_u32_be(kDataAddress, 0x01020304u);
        machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
        machine.stage_cpu_gpr(kStoreSourceIndex, store_source);
        machine.stage_cpu_gpr(kScSourceIndex, 0x88776655u);

        step_at(machine, kLlAddress, label);
        step_at(machine, kStoreAddress, label);
        require_rdram_word_equals(machine, kDataAddress, expected_after_store, label);
        step_at(machine, kScAddress, label);

        require_gpr_equals(machine, kScSourceIndex, 0, label);
        require_rdram_word_equals(machine, kDataAddress, expected_after_store, label);
      };

  require_word_store_invalidates(
      "ll_sc_invalidation_demo_sw",
      encode_sw(6, 4, 0),
      0xaabbccddu,
      0xaabbccddu);
  require_word_store_invalidates(
      "ll_sc_invalidation_demo_sb",
      encode_sb(6, 4, 1),
      0x000000aau,
      0x01aa0304u);
  require_word_store_invalidates(
      "ll_sc_invalidation_demo_sh",
      encode_sh(6, 4, 2),
      0x0000bbccu,
      0x0102bbccu);
  require_word_store_invalidates(
      "ll_sc_invalidation_demo_swl",
      encode_swl(6, 4, 1),
      0xaabbccddu,
      0x01aabbccu);

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kOtherBaseIndex = 8;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr std::size_t kStoreSourceIndex = 6;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00000c20u;
    constexpr RdramOffset kStoreAddress = 0x00000c24u;
    constexpr RdramOffset kScAddress = 0x00000c28u;
    constexpr RdramOffset kReservedAddress = 0x00000d20u;
    constexpr RdramOffset kOtherAddress = 0x00000d40u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kStoreAddress,
        encode_sw(static_cast<std::uint8_t>(kStoreSourceIndex), static_cast<std::uint8_t>(kOtherBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kScSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kReservedAddress, 0x11112222u);
    machine.stage_rdram_u32_be(kOtherAddress, 0x33334444u);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kReservedAddress));
    machine.stage_cpu_gpr(kOtherBaseIndex, cpu_rdram_alias(kOtherAddress));
    machine.stage_cpu_gpr(kStoreSourceIndex, 0x55667788u);
    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);

    step_at(machine, kLlAddress, "ll_sc_invalidation_demo_non_overlap_ll");
    step_at(machine, kStoreAddress, "ll_sc_invalidation_demo_non_overlap_sw");
    step_at(machine, kScAddress, "ll_sc_invalidation_demo_non_overlap_sc");

    require_gpr_equals(machine, kScSourceIndex, 1, "ll_sc_invalidation_demo_non_overlap_sc");
    require_rdram_word_equals(
        machine,
        kReservedAddress,
        0xaabbccddu,
        "ll_sc_invalidation_demo_non_overlap_sc");
    require_rdram_word_equals(
        machine,
        kOtherAddress,
        0x55667788u,
        "ll_sc_invalidation_demo_non_overlap_sc");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLldTargetIndex = 5;
    constexpr std::size_t kStoreSourceIndex = 6;
    constexpr std::size_t kScdSourceIndex = 7;
    constexpr RdramOffset kLldAddress = 0x00000c40u;
    constexpr RdramOffset kSdAddress = 0x00000c44u;
    constexpr RdramOffset kScdAddress = 0x00000c48u;
    constexpr RdramOffset kDataAddress = 0x00000d80u;
    constexpr CpuRegisterValue kStoreValue = 0x0102030405060708ull;

    stage_atomic_instruction(
        machine,
        kLldAddress,
        encode_lld(static_cast<std::uint8_t>(kLldTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kSdAddress,
        encode_sd(static_cast<std::uint8_t>(kStoreSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScdAddress,
        encode_scd(static_cast<std::uint8_t>(kScdSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, 0xaabbccddeeff0011ull);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kStoreSourceIndex, kStoreValue);
    machine.stage_cpu_gpr(kScdSourceIndex, 0x1122334455667788ull);

    step_at(machine, kLldAddress, "ll_sc_invalidation_demo_lld");
    step_at(machine, kSdAddress, "ll_sc_invalidation_demo_sd");
    require_rdram_doubleword_equals(machine, kDataAddress, kStoreValue, "ll_sc_invalidation_demo_sd");
    step_at(machine, kScdAddress, "ll_sc_invalidation_demo_scd_after_sd");

    require_gpr_equals(machine, kScdSourceIndex, 0, "ll_sc_invalidation_demo_scd_after_sd");
    require_rdram_doubleword_equals(
        machine,
        kDataAddress,
        kStoreValue,
        "ll_sc_invalidation_demo_scd_after_sd");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLldTargetIndex = 5;
    constexpr std::size_t kStoreSourceIndex = 6;
    constexpr std::size_t kScdSourceIndex = 7;
    constexpr RdramOffset kLldAddress = 0x00000c60u;
    constexpr RdramOffset kSdlAddress = 0x00000c64u;
    constexpr RdramOffset kScdAddress = 0x00000c68u;
    constexpr RdramOffset kDataAddress = 0x00000dc0u;

    stage_atomic_instruction(
        machine,
        kLldAddress,
        encode_lld(static_cast<std::uint8_t>(kLldTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kSdlAddress,
        encode_sdl(static_cast<std::uint8_t>(kStoreSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 2));
    stage_atomic_instruction(
        machine,
        kScdAddress,
        encode_scd(static_cast<std::uint8_t>(kScdSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, 0xaaaaaaaabbbbbbbbull);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kStoreSourceIndex, 0x1122334455667788ull);
    machine.stage_cpu_gpr(kScdSourceIndex, 0x99aabbccddeeff00ull);

    step_at(machine, kLldAddress, "ll_sc_invalidation_demo_lld_partial");
    step_at(machine, kSdlAddress, "ll_sc_invalidation_demo_sdl");
    require_rdram_doubleword_equals(
        machine,
        kDataAddress,
        0xaaaa112233445566ull,
        "ll_sc_invalidation_demo_sdl");
    step_at(machine, kScdAddress, "ll_sc_invalidation_demo_scd_after_sdl");

    require_gpr_equals(machine, kScdSourceIndex, 0, "ll_sc_invalidation_demo_scd_after_sdl");
    require_rdram_doubleword_equals(
        machine,
        kDataAddress,
        0xaaaa112233445566ull,
        "ll_sc_invalidation_demo_scd_after_sdl");
  }
}

void run_public_staging_reservation_demo() {
  std::cout << "fn64 bootstrap LL/SC demo: public staging interactions with local reservations\n";

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00000d00u;
    constexpr RdramOffset kScAddress = 0x00000d04u;
    constexpr RdramOffset kDataAddress = 0x00000e00u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kScSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);

    step_at(machine, kLlAddress, "public_staging_demo_ll");
    machine.stage_rdram_u32_be(kDataAddress, 0x33334444u);
    step_at(machine, kScAddress, "public_staging_demo_stage_overlap_sc");

    require_gpr_equals(machine, kScSourceIndex, 0, "public_staging_demo_stage_overlap_sc");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        0x33334444u,
        "public_staging_demo_stage_overlap_sc");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00000d20u;
    constexpr RdramOffset kScAddress = 0x00000d24u;
    constexpr RdramOffset kDataAddress = 0x00000e20u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kScSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);

    step_at(machine, kLlAddress, "public_staging_demo_failed_stage_ll");
    try {
      machine.stage_rdram_u32_be(0x003ffffeu, 0x33334444u);
    } catch (const std::exception& e) {
      std::cout << "  failed public RDRAM staging threw: " << e.what() << '\n';
    }
    step_at(machine, kScAddress, "public_staging_demo_failed_stage_sc");

    require_gpr_equals(machine, kScSourceIndex, 1, "public_staging_demo_failed_stage_sc");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        0xaabbccddu,
        "public_staging_demo_failed_stage_sc");
  }

  {
    const std::vector<std::uint8_t> rom =
        make_bootstrap_cartridge_staging_rom(0xdeadbeefu, 0x00000000u);
    Cartridge cartridge;
    std::string error;
    if (!load_cartridge(rom, cartridge, error)) {
      throw std::runtime_error("public staging reservation demo could not load cartridge: " + error);
    }

    auto machine_storage = std::make_unique<Machine>(std::move(cartridge));

    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00000d40u;
    constexpr RdramOffset kScAddress = 0x00000d44u;
    constexpr RdramOffset kDataAddress = 0x00000e40u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kScSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);

    step_at(machine, kLlAddress, "public_staging_demo_cartridge_ll");
    machine.stage_cartridge_bytes_to_rdram(0x40u, kDataAddress, 4);
    step_at(machine, kScAddress, "public_staging_demo_cartridge_overlap_sc");

    require_gpr_equals(machine, kScSourceIndex, 0, "public_staging_demo_cartridge_overlap_sc");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        0xdeadbeefu,
        "public_staging_demo_cartridge_overlap_sc");
  }
}

void run_store_conditional_register_order_demo() {
  std::cout << "fn64 bootstrap LL/SC demo: SC/SCD source and address are read before result write\n";

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseAndSourceIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr RdramOffset kLlAddress = 0x00000d80u;
    constexpr RdramOffset kScAddress = 0x00000d84u;
    constexpr RdramOffset kDataAddress = 0x00000e80u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseAndSourceIndex), 0));
    stage_atomic_instruction(
        machine,
        kScAddress,
        encode_sc(static_cast<std::uint8_t>(kBaseAndSourceIndex), static_cast<std::uint8_t>(kBaseAndSourceIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kBaseAndSourceIndex, cpu_rdram_alias(kDataAddress));

    step_at(machine, kLlAddress, "sc_register_order_demo_ll");
    step_at(machine, kScAddress, "sc_register_order_demo_sc_rt_is_base");

    require_gpr_equals(machine, kBaseAndSourceIndex, 1, "sc_register_order_demo_sc_rt_is_base");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        cpu_rdram_alias(kDataAddress),
        "sc_register_order_demo_sc_rt_is_base");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr RdramOffset kLlAddress = 0x00000da0u;
    constexpr RdramOffset kScAddress = 0x00000da4u;
    constexpr RdramOffset kDataAddress = 0x00000ea0u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(machine, kScAddress, encode_sc(0, static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));

    step_at(machine, kLlAddress, "sc_register_order_demo_zero_ll");
    step_at(machine, kScAddress, "sc_register_order_demo_zero_sc_success");

    require_gpr_equals(machine, 0, 0, "sc_register_order_demo_zero_sc_success");
    require_rdram_word_equals(machine, kDataAddress, 0, "sc_register_order_demo_zero_sc_success");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr RdramOffset kScAddress = 0x00000dc0u;
    constexpr RdramOffset kDataAddress = 0x00000ec0u;

    stage_atomic_instruction(machine, kScAddress, encode_sc(0, static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    step_at(machine, kScAddress, "sc_register_order_demo_zero_sc_failure");

    require_gpr_equals(machine, 0, 0, "sc_register_order_demo_zero_sc_failure");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        0x11112222u,
        "sc_register_order_demo_zero_sc_failure");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLldTargetIndex = 5;
    constexpr RdramOffset kLldAddress = 0x00000dd0u;
    constexpr RdramOffset kScdAddress = 0x00000dd4u;
    constexpr RdramOffset kDataAddress = 0x00000ee0u;

    stage_atomic_instruction(
        machine,
        kLldAddress,
        encode_lld(static_cast<std::uint8_t>(kLldTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(machine, kScdAddress, encode_scd(0, static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, 0x1122334455667788ull);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));

    step_at(machine, kLldAddress, "sc_register_order_demo_zero_lld");
    step_at(machine, kScdAddress, "sc_register_order_demo_zero_scd_success");

    require_gpr_equals(machine, 0, 0, "sc_register_order_demo_zero_scd_success");
    require_rdram_doubleword_equals(
        machine,
        kDataAddress,
        0,
        "sc_register_order_demo_zero_scd_success");
  }

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr RdramOffset kScdAddress = 0x00000df0u;
    constexpr RdramOffset kDataAddress = 0x00000ef0u;
    constexpr CpuRegisterValue kInitialValue = 0x1122334455667788ull;

    stage_atomic_instruction(machine, kScdAddress, encode_scd(0, static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_rdram_u64_be(machine, kDataAddress, kInitialValue);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    step_at(machine, kScdAddress, "sc_register_order_demo_zero_scd_failure");

    require_gpr_equals(machine, 0, 0, "sc_register_order_demo_zero_scd_failure");
    require_rdram_doubleword_equals(
        machine,
        kDataAddress,
        kInitialValue,
        "sc_register_order_demo_zero_scd_failure");
  }
}

void run_load_link_store_conditional_fault_demo() {
  std::cout << "fn64 bootstrap LL/SC demo: local MachineFault paths do not mutate visible state\n";

  const auto require_fault_case =
      [](const char* label,
         CpuInstructionWord instruction,
         CpuAddress base_value,
         MachineFaultKind expected_kind,
         std::size_t expected_access_size) {
        auto machine_storage = std::make_unique<Machine>(Cartridge{});
        Machine& machine = *machine_storage;
        constexpr std::size_t kBaseIndex = 4;
        constexpr std::size_t kTargetIndex = 7;
        constexpr RdramOffset kInstructionAddress = 0x00000e00u;
        constexpr RdramOffset kSentinelAddress = 0x00000f00u;
        constexpr CpuRegisterValue kTargetSentinel = 0x1122334455667788ull;
        constexpr std::uint32_t kRdramSentinel = 0xaabbccddu;

        stage_atomic_instruction(machine, kInstructionAddress, instruction);
        machine.stage_cpu_pc(cpu_rdram_alias(kInstructionAddress));
        machine.stage_cpu_gpr(kBaseIndex, base_value);
        machine.stage_cpu_gpr(kTargetIndex, kTargetSentinel);
        machine.stage_rdram_u32_be(kSentinelAddress, kRdramSentinel);

        require_step_machine_fault(machine, label, expected_kind, expected_access_size);

        if (machine.cpu_pc() != cpu_rdram_alias(kInstructionAddress) ||
            machine.cpu_next_pc() != cpu_rdram_alias(kInstructionAddress + 4u)) {
          throw std::runtime_error(std::string(label) + " changed pc/next_pc on fault");
        }

        require_gpr_equals(machine, kTargetIndex, kTargetSentinel, label);
        require_rdram_word_equals(machine, kSentinelAddress, kRdramSentinel, label);
      };

  require_fault_case(
      "ll_fault_demo_misaligned_ll",
      encode_ll(7, 4, 0),
      cpu_rdram_alias(0x00001001u),
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      4);
  require_fault_case(
      "ll_fault_demo_out_of_range_ll",
      encode_ll(7, 4, 0),
      cpu_rdram_alias(0x00400000u),
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
  require_fault_case(
      "ll_fault_demo_misaligned_lld",
      encode_lld(7, 4, 0),
      cpu_rdram_alias(0x00001004u),
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      8);
  require_fault_case(
      "ll_fault_demo_out_of_range_lld",
      encode_lld(7, 4, 0),
      cpu_rdram_alias(0x00400000u),
      MachineFaultKind::kCpuRdramAddressRejected,
      8);
  require_fault_case(
      "ll_fault_demo_misaligned_sc",
      encode_sc(7, 4, 0),
      cpu_rdram_alias(0x00001001u),
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      4);
  require_fault_case(
      "ll_fault_demo_out_of_range_sc",
      encode_sc(7, 4, 0),
      cpu_rdram_alias(0x00400000u),
      MachineFaultKind::kCpuRdramAddressRejected,
      4);
  require_fault_case(
      "ll_fault_demo_misaligned_scd",
      encode_scd(7, 4, 0),
      cpu_rdram_alias(0x00001004u),
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      8);
  require_fault_case(
      "ll_fault_demo_out_of_range_scd",
      encode_scd(7, 4, 0),
      cpu_rdram_alias(0x00400000u),
      MachineFaultKind::kCpuRdramAddressRejected,
      8);

  {
    auto machine_storage = std::make_unique<Machine>(Cartridge{});
    Machine& machine = *machine_storage;
    constexpr std::size_t kBaseIndex = 4;
    constexpr std::size_t kLlTargetIndex = 5;
    constexpr std::size_t kScSourceIndex = 7;
    constexpr RdramOffset kLlAddress = 0x00000e40u;
    constexpr RdramOffset kBadScAddress = 0x00000e44u;
    constexpr RdramOffset kGoodScAddress = 0x00000e48u;
    constexpr RdramOffset kDataAddress = 0x00000f40u;

    stage_atomic_instruction(
        machine,
        kLlAddress,
        encode_ll(static_cast<std::uint8_t>(kLlTargetIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    stage_atomic_instruction(
        machine,
        kBadScAddress,
        encode_sc(static_cast<std::uint8_t>(kScSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 1));
    stage_atomic_instruction(
        machine,
        kGoodScAddress,
        encode_sc(static_cast<std::uint8_t>(kScSourceIndex), static_cast<std::uint8_t>(kBaseIndex), 0));
    machine.stage_rdram_u32_be(kDataAddress, 0x11112222u);
    machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataAddress));
    machine.stage_cpu_gpr(kScSourceIndex, 0xaabbccddu);

    step_at(machine, kLlAddress, "ll_fault_demo_sc_preserve_reservation_ll");
    machine.stage_cpu_pc(cpu_rdram_alias(kBadScAddress));
    require_step_machine_fault(
        machine,
        "ll_fault_demo_sc_preserve_reservation_fault",
        MachineFaultKind::kUnalignedCpuMemoryAccess,
        4);
    step_at(machine, kGoodScAddress, "ll_fault_demo_sc_preserve_reservation_sc");

    require_gpr_equals(machine, kScSourceIndex, 1, "ll_fault_demo_sc_preserve_reservation_sc");
    require_rdram_word_equals(
        machine,
        kDataAddress,
        0xaabbccddu,
        "ll_fault_demo_sc_preserve_reservation_sc");
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

  machine.stage_rdram_u32_be(kSbAddress, kSbInstruction);
  machine.stage_rdram_u32_be(kLbAddress, kLbInstruction);

  machine.stage_cpu_gpr(kBaseIndex, kBaseAddress);
  machine.stage_cpu_gpr(kSourceIndex, 0x00000080u);
  machine.stage_cpu_gpr(kTargetIndex, 0x89abcdefu);
  machine.stage_rdram_u32_be(kSentinelAddress, 0x11223344u);

  std::cout << "fn64 bootstrap negative-offset guard demo: explicit local out-of-range rollback on signed immediate address formation\n";

  machine.stage_cpu_pc(cpu_rdram_alias(kSbAddress));

  std::cout << "before SB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex32("  sb_immediate_raw", kNegativeOffset);
  print_hex32("  sb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[27]", machine.inspect_cpu_gpr(kSourceIndex));
  print_rdram_word(machine, "  rdram[0x00000550]", kSentinelAddress);

  require_step_machine_fault(
      machine,
      "negative_out_of_range_demo_sb",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after SB out-of-range step:\n";
  print_control_flow_state(machine);
  print_rdram_word(machine, "  rdram[0x00000550]", kSentinelAddress);

  if (machine.cpu_pc() != cpu_rdram_alias(kSbAddress)) {
    throw std::runtime_error("negative-offset guard demo SB changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kSbAddress + 4u)) {
    throw std::runtime_error("negative-offset guard demo SB changed next_pc on fault");
  }

  if (machine.inspect_rdram_u32_be(kSentinelAddress) != 0x11223344u) {
    throw std::runtime_error("negative-offset guard demo SB changed memory on fault");
  }

  machine.stage_cpu_pc(cpu_rdram_alias(kLbAddress));
  machine.stage_cpu_gpr(kTargetIndex, 0x89abcdefu);

  std::cout << "before LB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kBaseIndex));
  print_hex32("  lb_immediate_raw", kNegativeOffset);
  print_hex32("  lb_effective_address", kEffectiveAddress);
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));

  require_step_machine_fault(
      machine,
      "negative_out_of_range_demo_lb",
      MachineFaultKind::kCpuRdramAddressRejected,
      1);

  std::cout << "after LB out-of-range step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[28]", machine.inspect_cpu_gpr(kTargetIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kLbAddress)) {
    throw std::runtime_error("negative-offset guard demo LB changed PC on fault");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kLbAddress + 4u)) {
    throw std::runtime_error("negative-offset guard demo LB changed next_pc on fault");
  }

  if (machine.inspect_cpu_gpr(kTargetIndex) != 0x89abcdefu) {
    throw std::runtime_error("negative-offset guard demo LB changed target register on fault");
  }
}

}  // namespace

void run_data_demos(Machine& machine) {
  run_synthetic_rom_normalization_rejection_demo();
  run_synthetic_cartridge_read_guard_demo();
  run_machine_construction_isolation_demo();
  run_cartridge_staging_demo();
  run_cartridge_staging_preflight_demo();
  run_public_machine_stage_inspect_guard_demo();
  run_cpu_rdram_translation_demo(machine);
  run_cpu_rdram_alias_demo(machine);
  run_unaligned_load_word_demo(machine);
  run_unaligned_store_word_demo(machine);
  run_partial_word_lane_matrix_demo(machine);
  run_partial_doubleword_lane_demo(machine);
  run_aligned_word_load_store_demo(machine);
  run_unsigned_word_load_demo(machine);
  run_aligned_doubleword_load_store_demo(machine);
  run_word_alignment_guard_demo(machine);
  run_doubleword_alignment_guard_demo(machine);
  run_byte_load_store_demo(machine);
  run_halfword_load_store_demo(machine);
  run_halfword_alignment_guard_demo(machine);
  run_negative_word_load_store_demo(machine);
  run_negative_byte_load_store_demo(machine);
  run_negative_halfword_load_store_demo(machine);
  run_failed_partial_load_no_ghost_demo(machine);
  run_failed_partial_store_no_ghost_demo(machine);
  run_failed_partial_doubleword_no_ghost_demo(machine);
  run_failed_unsigned_word_load_no_ghost_demo(machine);
  run_failed_doubleword_no_ghost_demo(machine);
  run_load_link_store_conditional_success_demo();
  run_store_conditional_failure_demo();
  run_store_conditional_invalidation_demo();
  run_public_staging_reservation_demo();
  run_store_conditional_register_order_demo();
  run_load_link_store_conditional_fault_demo();
  run_sp_memory_data_demo();
  run_sp_mmio_dma_success_demo();
  run_sp_dma_failure_demo();
  run_sp_dma_reservation_demo();
  run_sp_mmio_fault_demo();
  run_cpu_driven_pi_dma_execution_demo();
  run_cpu_driven_pi_sp_dma_chain_demo();
  run_pi_mmio_dma_success_demo();
  run_pi_dma_reservation_demo();
  run_pi_dma_failure_demo();
  run_pi_mmio_fault_demo();
  run_negative_out_of_range_guard_demo(machine);
}

}  // namespace fn64::bootstrap_detail
