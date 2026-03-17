#pragma once

#include <cstdint>
#include <filesystem>
#include <string>
#include <vector>

namespace fn64 {

struct Rom {
  std::filesystem::path path;
  std::vector<std::uint8_t> bytes;
};

bool load_rom(const std::filesystem::path& path, Rom& out_rom, std::string& error);

}  // namespace fn64
