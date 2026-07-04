#include <cstdint>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "cartridge.hpp"
#include "machine.hpp"
#include "rom.hpp"

namespace {

std::string hex_u8(std::uint8_t value) {
  std::ostringstream stream;
  stream << "0x" << std::uppercase << std::hex << std::setw(2)
         << std::setfill('0') << static_cast<unsigned int>(value);
  return stream.str();
}

std::string hex_u32(std::uint32_t value) {
  std::ostringstream stream;
  stream << "0x" << std::uppercase << std::hex << std::setw(8)
         << std::setfill('0') << value;
  return stream.str();
}

bool read_file_bytes(
    const std::filesystem::path& path,
    std::vector<std::uint8_t>& out_bytes,
    std::string& error) {
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

void print_usage() {
  std::cerr << "usage: fn64_inspect <rom-path>\n";
}

void print_cartridge_summary(
    const std::filesystem::path& rom_path,
    const fn64::Cartridge& cartridge) {
  const fn64::RomMetadata& metadata = cartridge.metadata();

  std::cout
      << "fn64 no-window cartridge inspection\n"
      << "  path: " << rom_path << '\n'
      << "  source layout: "
      << fn64::rom_source_layout_name(cartridge.source_layout()) << '\n'
      << "  normalized size bytes: " << cartridge.size_bytes() << '\n'
      << "  header magic: " << hex_u32(metadata.header_magic) << '\n'
      << "  clock rate: " << hex_u32(metadata.clock_rate) << '\n'
      << "  entry point: " << hex_u32(metadata.entry_point) << '\n'
      << "  release address: " << hex_u32(metadata.release_address) << '\n'
      << "  crc1: " << hex_u32(metadata.crc1) << '\n'
      << "  crc2: " << hex_u32(metadata.crc2) << '\n'
      << "  image name: " << metadata.image_name << '\n'
      << "  cartridge id: " << metadata.cartridge_id << '\n'
      << "  country code: " << hex_u8(metadata.country_code) << '\n'
      << "  revision: " << static_cast<unsigned int>(metadata.revision) << '\n';
}

void print_cartridge_entry_inspection(const fn64::Cartridge& cartridge) {
  const fn64::CartridgeEntryInspection inspection =
      fn64::inspect_cartridge_entry(cartridge);

  std::cout << "\ncartridge entry inspection\n";
  if (inspection.header_entry_word_available) {
    std::cout << "  header entry word: "
              << hex_u32(inspection.header_entry_word)
              << " at cart[0x00000008]\n";
  } else {
    std::cout << "  header entry word: unavailable\n";
  }

  if (inspection.candidate_ipl3_span_available) {
    std::cout << "  candidate IPL3 span: cart["
              << hex_u32(inspection.candidate_ipl3_start_offset)
              << ".."
              << hex_u32(inspection.candidate_ipl3_end_offset_exclusive - 1u)
              << "] (" << inspection.candidate_ipl3_byte_count << " bytes)\n";
  } else {
    std::cout << "  candidate IPL3 span: unavailable\n";
  }

  if (inspection.ipl3_first_word_available) {
    std::cout << "  IPL3 first word: "
              << hex_u32(inspection.ipl3_first_word)
              << " at cart[0x00000040]\n";
  } else {
    std::cout << "  IPL3 first word: unavailable\n";
  }

  std::cout << "  observation only: no reset, boot, staging, or execution\n";
}

void print_machine_summary(const fn64::Machine& machine) {
  std::cout
      << "\n"
      << "initial machine state\n"
      << "  powered on: " << (machine.powered_on() ? "yes" : "no") << '\n'
      << "  reset model: blank RDRAM power-on state\n"
      << "  rdram bytes: " << machine.rdram_size_bytes() << '\n'
      << "  machine cartridge bytes: " << machine.cartridge().size_bytes() << '\n'
      << "  cpu pc: " << hex_u32(machine.cpu_pc()) << '\n'
      << "  cpu next pc: " << hex_u32(machine.cpu_next_pc()) << '\n'
      << "  cartridge execution: not wired\n";
}

}  // namespace

int main(int argc, char** argv) {
  if (argc != 2) {
    print_usage();
    return 1;
  }

  try {
    const std::filesystem::path rom_path = argv[1];

    std::vector<std::uint8_t> raw_bytes;
    std::string error;
    if (!read_file_bytes(rom_path, raw_bytes, error)) {
      std::cerr << error << '\n';
      return 1;
    }

    fn64::Cartridge cartridge;
    if (!fn64::load_cartridge(std::move(raw_bytes), cartridge, error)) {
      std::cerr << error << '\n';
      return 1;
    }

    print_cartridge_summary(rom_path, cartridge);
    print_cartridge_entry_inspection(cartridge);

    fn64::Machine machine(std::move(cartridge));
    print_machine_summary(machine);

    std::cout << "\n"
              << "inspection mode: no window runtime, no demos, no cartridge "
                 "bytes staged, no CPU instructions stepped\n";
    return 0;
  } catch (const std::exception& exception) {
    std::cerr << exception.what() << '\n';
    return 1;
  }
}
