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

}  // namespace fn64
