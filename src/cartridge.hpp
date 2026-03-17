#pragma once

#include <cstddef>
#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

#include "rom.hpp"

namespace fn64 {

class Cartridge {
public:
  Cartridge() = default;

  const std::filesystem::path& path() const noexcept;
  RomSourceLayout source_layout() const noexcept;
  const RomMetadata& metadata() const noexcept;

  std::size_t size_bytes() const noexcept;

  std::uint8_t read_u8(std::uint32_t cart_addr) const;
  std::uint16_t read_u16_be(std::uint32_t cart_addr) const;
  std::uint32_t read_u32_be(std::uint32_t cart_addr) const;

private:
  Cartridge(
      std::filesystem::path path,
      RomSourceLayout source_layout,
      std::vector<std::uint8_t> image,
      RomMetadata metadata
  );

  std::filesystem::path path_;
  RomSourceLayout source_layout_ = RomSourceLayout::kBigEndian;
  std::vector<std::uint8_t> image_;
  RomMetadata metadata_;

  friend bool load_cartridge(
      const std::filesystem::path& path,
      Cartridge& out_cartridge,
      std::string& error
  );
};

bool load_cartridge(
    const std::filesystem::path& path,
    Cartridge& out_cartridge,
    std::string& error
);

}  // namespace fn64