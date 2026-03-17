#include "rom.hpp"

#include <algorithm>
#include <cctype>
#include <fstream>
#include <utility>

namespace fn64 {

const char* rom_source_layout_name(RomSourceLayout layout) {
  switch (layout) {
    case RomSourceLayout::kBigEndian:
      return "big-endian cartridge order (.z64)";
    case RomSourceLayout::kByteSwapped16:
      return "16-bit byte-swapped (.v64)";
    case RomSourceLayout::kLittleEndian32:
      return "32-bit little-endian word order (.n64)";
  }

  return "unknown";
}

namespace {
constexpr std::size_t kRomHeaderSize = 0x40;
constexpr std::uint32_t kExpectedHeaderMagic = 0x80371240;

bool read_file_bytes(
    const std::filesystem::path& path,
    std::vector<std::uint8_t>& out_bytes,
    std::string& error
) {
  std::ifstream file(path, std::ios::binary | std::ios::ate);
  if (!file.is_open()) {
    error = "could not open file";
    return false;
  }

  const std::streamsize size = file.tellg();
  if (size < 0) {
    error = "could not determine file size";
    return false;
  }

  file.seekg(0, std::ios::beg);

  std::vector<std::uint8_t> bytes(static_cast<std::size_t>(size));
  if (size > 0 && !file.read(reinterpret_cast<char*>(bytes.data()), size)) {
    error = "could not read file bytes";
    return false;
  }

  out_bytes = std::move(bytes);
  return true;
}

bool detect_rom_source_layout(
    const std::vector<std::uint8_t>& raw_bytes,
    RomSourceLayout& out_layout,
    std::string& error
) {
  if (raw_bytes.size() < 4) {
    error = "file is too small to contain an N64 ROM header";
    return false;
  }

  const std::uint8_t b0 = raw_bytes[0];
  const std::uint8_t b1 = raw_bytes[1];
  const std::uint8_t b2 = raw_bytes[2];
  const std::uint8_t b3 = raw_bytes[3];

  if (b0 == 0x80 && b1 == 0x37 && b2 == 0x12 && b3 == 0x40) {
    out_layout = RomSourceLayout::kBigEndian;
    return true;
  }

  if (b0 == 0x37 && b1 == 0x80 && b2 == 0x40 && b3 == 0x12) {
    out_layout = RomSourceLayout::kByteSwapped16;
    return true;
  }

  if (b0 == 0x40 && b1 == 0x12 && b2 == 0x37 && b3 == 0x80) {
    out_layout = RomSourceLayout::kLittleEndian32;
    return true;
  }

  error =
      "unsupported ROM header byte layout; expected one of "
      "80 37 12 40, 37 80 40 12, or 40 12 37 80";
  return false;
}

std::vector<std::uint8_t> normalize_rom_bytes(
    const std::vector<std::uint8_t>& raw_bytes,
    RomSourceLayout layout
) {
  std::vector<std::uint8_t> normalized = raw_bytes;

  switch (layout) {
    case RomSourceLayout::kBigEndian:
      return normalized;

    case RomSourceLayout::kByteSwapped16:
      for (std::size_t i = 0; i < normalized.size(); i += 2) {
        std::swap(normalized[i], normalized[i + 1]);
      }
      return normalized;

    case RomSourceLayout::kLittleEndian32:
      for (std::size_t i = 0; i < normalized.size(); i += 4) {
        std::swap(normalized[i], normalized[i + 3]);
        std::swap(normalized[i + 1], normalized[i + 2]);
      }
      return normalized;
  }

  return normalized;
}

std::uint32_t read_be_u32(const std::vector<std::uint8_t>& bytes, std::size_t offset) {
  return (static_cast<std::uint32_t>(bytes[offset]) << 24) |
         (static_cast<std::uint32_t>(bytes[offset + 1]) << 16) |
         (static_cast<std::uint32_t>(bytes[offset + 2]) << 8) |
         static_cast<std::uint32_t>(bytes[offset + 3]);
}

std::string read_ascii_field(
    const std::vector<std::uint8_t>& bytes,
    std::size_t offset,
    std::size_t length
) {
  std::string value;
  value.reserve(length);

  for (std::size_t i = 0; i < length; ++i) {
    const unsigned char ch = bytes[offset + i];
    if (ch == 0) {
      break;
    }

    value.push_back(std::isprint(ch) ? static_cast<char>(ch) : '?');
  }

  while (!value.empty() && value.back() == ' ') {
    value.pop_back();
  }

  return value;
}

bool parse_rom_metadata(
    const std::vector<std::uint8_t>& normalized_bytes,
    RomMetadata& out_metadata,
    std::string& error
) {
  if (normalized_bytes.size() < kRomHeaderSize) {
    error = "normalized ROM is smaller than the 0x40-byte N64 header";
    return false;
  }

  RomMetadata metadata;
  metadata.header_magic = read_be_u32(normalized_bytes, 0x00);
  if (metadata.header_magic != kExpectedHeaderMagic) {
    error = "normalized ROM header magic mismatch; expected 0x80371240";
    return false;
  }

  metadata.clock_rate = read_be_u32(normalized_bytes, 0x04);
  metadata.entry_point = read_be_u32(normalized_bytes, 0x08);
  metadata.release_address = read_be_u32(normalized_bytes, 0x0C);
  metadata.crc1 = read_be_u32(normalized_bytes, 0x10);
  metadata.crc2 = read_be_u32(normalized_bytes, 0x14);
  metadata.image_name = read_ascii_field(normalized_bytes, 0x20, 20);
  metadata.cartridge_id = read_ascii_field(normalized_bytes, 0x3C, 2);
  metadata.country_code = normalized_bytes[0x3E];
  metadata.revision = normalized_bytes[0x3F];

  out_metadata = std::move(metadata);
  return true;
}

}  // namespace

bool load_normalized_rom_image(
    const std::filesystem::path& path,
    NormalizedRomImage& out_image,
    std::string& error
) {
  out_image = {};
  error.clear();

  std::vector<std::uint8_t> raw_bytes;
  if (!read_file_bytes(path, raw_bytes, error)) {
    return false;
  }

  if (raw_bytes.size() < kRomHeaderSize) {
    error = "file is too small to contain a complete 0x40-byte N64 ROM header";
    return false;
  }

  if ((raw_bytes.size() % 4) != 0) {
    error = "ROM size is not a multiple of 4 bytes";
    return false;
  }

  NormalizedRomImage image;
  image.path = path;

  if (!detect_rom_source_layout(raw_bytes, image.source_layout, error)) {
    return false;
  }

  image.bytes = normalize_rom_bytes(raw_bytes, image.source_layout);

  if (!parse_rom_metadata(image.bytes, image.metadata, error)) {
    return false;
  }

  out_image = std::move(image);
  return true;
}

}  // namespace fn64