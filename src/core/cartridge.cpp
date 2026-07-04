#include "cartridge.hpp"

#include <iomanip>
#include <sstream>
#include <stdexcept>
#include <utility>

namespace fn64 {
namespace {

std::string hex_u32(std::uint32_t value) {
  std::ostringstream stream;
  stream << "0x" << std::uppercase << std::hex << std::setw(8) << std::setfill('0') << value;
  return stream.str();
}

bool span_available(std::size_t size, std::uint32_t offset, std::uint32_t byte_count) {
  const std::size_t span_offset = static_cast<std::size_t>(offset);
  const std::size_t span_byte_count = static_cast<std::size_t>(byte_count);
  return span_offset <= size && span_byte_count <= (size - span_offset);
}

std::uint32_t read_cartridge_u32_be(const Cartridge& cartridge, std::uint32_t offset) {
  return (static_cast<std::uint32_t>(cartridge.read_u8(offset)) << 24) |
         (static_cast<std::uint32_t>(cartridge.read_u8(offset + 1u)) << 16) |
         (static_cast<std::uint32_t>(cartridge.read_u8(offset + 2u)) << 8) |
         static_cast<std::uint32_t>(cartridge.read_u8(offset + 3u));
}

void require_readable_range(std::size_t image_size, std::uint32_t cart_addr, std::size_t width) {
  const std::size_t offset = static_cast<std::size_t>(cart_addr);

  if (offset > image_size || width > (image_size - offset)) {
    std::ostringstream stream;
    stream << "cartridge read out of range: addr=" << hex_u32(cart_addr)
           << " width=" << width
           << " size=" << image_size;
    throw std::out_of_range(stream.str());
  }
}

}  // namespace

Cartridge::Cartridge(
    RomSourceLayout source_layout,
    std::vector<std::uint8_t> image,
    RomMetadata metadata
)
    : source_layout_(source_layout),
      image_(std::move(image)),
      metadata_(std::move(metadata)) {}

RomSourceLayout Cartridge::source_layout() const noexcept {
  return source_layout_;
}

const RomMetadata& Cartridge::metadata() const noexcept {
  return metadata_;
}

std::size_t Cartridge::size_bytes() const noexcept {
  return image_.size();
}

std::uint8_t Cartridge::read_u8(std::uint32_t cart_addr) const {
  require_readable_range(image_.size(), cart_addr, 1);
  const std::size_t offset = static_cast<std::size_t>(cart_addr);
  return image_[offset];
}

bool load_cartridge(
    std::vector<std::uint8_t> raw_bytes,
    Cartridge& out_cartridge,
    std::string& error
) {
  NormalizedRomImage image;
  if (!normalize_rom_image(std::move(raw_bytes), image, error)) {
    out_cartridge = Cartridge(RomSourceLayout::kBigEndian, {}, {});
    return false;
  }

  out_cartridge = Cartridge(
      image.source_layout,
      std::move(image.bytes),
      std::move(image.metadata)
  );
  return true;
}

CartridgeEntryInspection inspect_cartridge_entry(const Cartridge& cartridge) {
  CartridgeEntryInspection inspection;
  const std::size_t size = cartridge.size_bytes();

  if (span_available(size, kCartridgeHeaderEntryWordOffset, 4)) {
    inspection.header_entry_word_available = true;
    inspection.header_entry_word =
        read_cartridge_u32_be(cartridge, kCartridgeHeaderEntryWordOffset);
  }

  if (span_available(
          size,
          kCartridgeCandidateIpl3StartOffset,
          kCartridgeCandidateIpl3ByteCount)) {
    inspection.candidate_ipl3_span_available = true;
  }

  if (span_available(size, kCartridgeCandidateIpl3StartOffset, 4)) {
    inspection.ipl3_first_word_available = true;
    inspection.ipl3_first_word =
        read_cartridge_u32_be(cartridge, kCartridgeCandidateIpl3StartOffset);
  }

  return inspection;
}

}  // namespace fn64
