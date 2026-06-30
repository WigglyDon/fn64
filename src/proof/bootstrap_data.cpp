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
  print_hex32("  machine_a_hi", machine_a->inspect_cpu_hi());
  print_hex32("  machine_a_lo", machine_a->inspect_cpu_lo());
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
  std::uint32_t pc = 0;
  std::uint32_t next_pc = 0;
  std::uint32_t hi = 0;
  std::uint32_t lo = 0;
  std::uint32_t gpr4 = 0;
  std::uint32_t gpr31 = 0;
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

  if (machine.inspect_cpu_gpr(kTargetIndex) != test_case.data_word) {
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
  print_hex32("  hi", guard_machine->inspect_cpu_hi());
  print_hex32("  lo", guard_machine->inspect_cpu_lo());
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

  if (machine.inspect_cpu_gpr(kTargetIndex) != kDataWord) {
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

  machine.stage_cpu_pc(cpu_rdram_alias(kLwlAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataWord0Address));
  machine.stage_cpu_gpr(kTargetIndex, 0xaabbccddu);

  machine.stage_rdram_u32_be(kLwlAddress, kLwlInstruction);
  machine.stage_rdram_u32_be(kLwrAddress, kLwrInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataWord0Address, 0x10203040u);
  machine.stage_rdram_u32_be(kDataWord1Address, 0x50607080u);

  std::cout << "fn64 bootstrap unaligned load demo: explicit local LWL/LWR merge\n";
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

  if (machine.inspect_cpu_gpr(kTargetIndex) != 0x3040ccddu) {
    throw std::runtime_error("unaligned load demo LWL merge result was wrong");
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

  if (machine.inspect_cpu_gpr(kTargetIndex) != 0x30405060u) {
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

  machine.stage_cpu_pc(cpu_rdram_alias(kSwlAddress));
  machine.stage_cpu_gpr(kBaseIndex, cpu_rdram_alias(kDataWord0Address));
  machine.stage_cpu_gpr(kSourceIndex, 0xa1b2c3d4u);

  machine.stage_rdram_u32_be(kSwlAddress, kSwlInstruction);
  machine.stage_rdram_u32_be(kSwrAddress, kSwrInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  machine.stage_rdram_u32_be(kDataWord0Address, 0x11223344u);
  machine.stage_rdram_u32_be(kDataWord1Address, 0x55667788u);

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

  if (machine.inspect_rdram_u32_be(kDataWord0Address) != 0x1122a1b2u) {
    throw std::runtime_error("unaligned store demo SWL shaping was wrong");
  }

  if (machine.inspect_rdram_u32_be(kDataWord1Address) != 0x55667788u) {
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

  if (machine.inspect_rdram_u32_be(kDataWord0Address) != 0x1122a1b2u) {
    throw std::runtime_error("unaligned store demo SWR unexpectedly changed the left aligned word");
  }

  if (machine.inspect_rdram_u32_be(kDataWord1Address) != 0xc3d47788u) {
    throw std::runtime_error("unaligned store demo SWR shaping was wrong");
  }

  if (machine.inspect_rdram_u32_be(kMergedWordAddress) != 0xa1b2c3d4u) {
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
    print_hex32("  expected_gpr[16]", test_case.expected_gpr);

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

  if (machine.inspect_cpu_gpr(kTargetIndex) != 0x89abcdefu) {
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

  machine.stage_rdram_u32_be(kSwAddress, kSwInstruction);
  machine.stage_rdram_u32_be(kLwAddress, kLwInstruction);

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

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) != 0xffffff80u) {
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

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) != 0xffff8001u) {
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

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) != 0xffffff80u) {
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

  if (machine.inspect_cpu_gpr(kSignedTargetIndex) != 0xffff8001u) {
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
  constexpr std::uint32_t kTargetSentinel = 0x89abcdefu;

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
  machine.stage_cpu_gpr(kSourceIndex, 0xa1b2c3d4u);

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
