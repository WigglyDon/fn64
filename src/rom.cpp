#include "rom.hpp"

#include <fstream>
#include <utility>

namespace fn64 {

bool load_rom(const std::filesystem::path& path, Rom& out_rom, std::string& error) {
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

  out_rom.path = path;
  out_rom.bytes = std::move(bytes);
  error.clear();
  return true;
}

}  // namespace fn64
