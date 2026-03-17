#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace fn64 {

enum class RomSourceLayout {
  kBigEndian,
  kByteSwapped16,
  kLittleEndian32,
};

const char* rom_source_layout_name(RomSourceLayout layout);

struct RomMetadata {
  std::uint32_t header_magic = 0;
  std::uint32_t clock_rate = 0;
  std::uint32_t entry_point = 0;
  std::uint32_t release_address = 0;
  std::uint32_t crc1 = 0;
  std::uint32_t crc2 = 0;
  std::string image_name;
  std::string cartridge_id;
  std::uint8_t country_code = 0;
  std::uint8_t revision = 0;
};

struct NormalizedRomImage {
  std::filesystem::path path;
  RomSourceLayout source_layout = RomSourceLayout::kBigEndian;
  std::vector<std::uint8_t> bytes;  // Canonical big-endian cartridge byte order.
  RomMetadata metadata;
};

bool load_normalized_rom_image(
    const std::filesystem::path& path,
    NormalizedRomImage& out_image,
    std::string& error
);

}  // namespace fn64