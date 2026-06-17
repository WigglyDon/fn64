#include "app.hpp"

#include <SDL3/SDL.h>

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

#include "bootstrap.hpp"
#include "cartridge.hpp"
#include "machine.hpp"

namespace fn64 {
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

bool load_cartridge_file(
    const std::filesystem::path& path,
    Cartridge& out_cartridge,
    std::string& error) {
  std::vector<std::uint8_t> raw_bytes;
  if (!read_file_bytes(path, raw_bytes, error)) {
    return false;
  }

  return load_cartridge(std::move(raw_bytes), out_cartridge, error);
}

}  // namespace

void App::print_usage() {
  std::cerr
      << "usage:\n"
      << "  fn64 --self-test\n"
      << "  fn64 --inspect-rom <rom-path>\n"
      << "  fn64 <rom-path>\n"
      << "\n"
      << "--self-test runs the internal CPU/RDRAM bootstrap demos and exits.\n"
      << "--inspect-rom loads a cartridge, prints session state, and exits without opening SDL.\n"
      << "<rom-path> loads a cartridge and opens the session window.\n"
      << "Cartridge execution is not wired yet.\n";
}

void App::print_loaded_cartridge(
    const std::filesystem::path& rom_path,
    const Cartridge& cartridge) {
  const RomMetadata& metadata = cartridge.metadata();

  std::cout
      << "fn64 cartridge session\n"
      << "  path: " << rom_path << '\n'
      << "  source layout: " << rom_source_layout_name(cartridge.source_layout()) << '\n'
      << "  size bytes: " << cartridge.size_bytes() << '\n'
      << "  image name: " << metadata.image_name << '\n'
      << "  cartridge id: " << metadata.cartridge_id << '\n'
      << "  country code: " << hex_u8(metadata.country_code) << '\n'
      << "  revision: " << static_cast<unsigned int>(metadata.revision) << '\n'
      << "  entry point: " << hex_u32(metadata.entry_point) << '\n'
      << "\n"
      << "Loaded cartridge bytes are owned by the machine.\n"
      << "Cartridge execution mapping is not wired yet.\n";
}

void App::print_machine_state(const Machine& machine) {
  std::cout
      << "\n"
      << "machine state\n"
      << "  powered on: " << (machine.powered_on() ? "yes" : "no") << '\n'
      << "  reset model: blank RDRAM power-on state\n"
      << "  N64 reset/PIF boot: not implemented\n"
      << "  rdram bytes: " << machine.rdram_size_bytes() << '\n'
      << "  cpu pc: " << hex_u32(machine.cpu_pc()) << '\n'
      << "  cpu next pc: " << hex_u32(machine.cpu_next_pc()) << '\n'
      << "  cpu fetch source: RDRAM physical/KSEG0/KSEG1 aliases only\n"
      << "  cartridge staging: explicit test/demo seam only\n"
      << "  cartridge execution: not wired\n";
}

bool App::init() {
  if (!SDL_Init(SDL_INIT_VIDEO)) {
    std::cerr << "SDL_Init failed: " << SDL_GetError() << '\n';
    return false;
  }

  window_ = SDL_CreateWindow("fn64", 960, 540, 0);
  if (window_ == nullptr) {
    std::cerr << "SDL_CreateWindow failed: " << SDL_GetError() << '\n';
    SDL_Quit();
    return false;
  }

  running_ = true;
  return true;
}

void App::shutdown() {
  if (window_ != nullptr) {
    SDL_DestroyWindow(window_);
    window_ = nullptr;
  }

  SDL_Quit();
  running_ = false;
}

void App::pump_events() {
  SDL_Event event;
  while (SDL_PollEvent(&event)) {
    if (event.type == SDL_EVENT_QUIT) {
      running_ = false;
    }
  }
}

int App::run_self_test() {
  Machine machine(Cartridge{});
  run_bootstrap_demos(machine);
  std::cout << "fn64 self-test: PASS\n";
  return 0;
}

int App::inspect_rom(const std::filesystem::path& rom_path) {
  Cartridge cartridge;
  std::string error;
  if (!load_cartridge_file(rom_path, cartridge, error)) {
    std::cerr << error << '\n';
    return 1;
  }

  print_loaded_cartridge(rom_path, cartridge);

  Machine machine(std::move(cartridge));
  print_machine_state(machine);

  std::cout << "\n"
            << "inspection mode: no SDL window opened, no bootstrap demos run, "
            << "no cartridge bytes staged, no CPU instructions stepped\n";

  return 0;
}

int App::run_rom_session(const std::filesystem::path& rom_path) {
  if (!init()) {
    return 1;
  }

  try {
    Cartridge cartridge;
    std::string error;
    if (!load_cartridge_file(rom_path, cartridge, error)) {
      std::cerr << error << '\n';
      shutdown();
      return 1;
    }

    print_loaded_cartridge(rom_path, cartridge);

    Machine machine(std::move(cartridge));
    print_machine_state(machine);

    while (running_) {
      pump_events();
      SDL_Delay(16);
    }
  } catch (...) {
    shutdown();
    throw;
  }

  shutdown();
  return 0;
}

int App::run(int argc, char** argv) {
  if (argc < 2) {
    print_usage();
    return 1;
  }

  const std::string argument = argv[1];
  if (argument == "--help" || argument == "-h") {
    if (argc != 2) {
      print_usage();
      return 1;
    }

    print_usage();
    return 0;
  }

  try {
    if (argument == "--self-test") {
      if (argc != 2) {
        print_usage();
        return 1;
      }

      return run_self_test();
    }

    if (argument == "--inspect-rom") {
      if (argc != 3) {
        print_usage();
        return 1;
      }

      return inspect_rom(std::filesystem::path(argv[2]));
    }

    if (argc != 2) {
      print_usage();
      return 1;
    }

    return run_rom_session(std::filesystem::path(argument));
  } catch (const std::exception& exception) {
    std::cerr << exception.what() << '\n';
    return 1;
  }
}

}  // namespace fn64
