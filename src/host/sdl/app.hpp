#pragma once

#include <filesystem>

struct SDL_Window;

namespace fn64 {

class Cartridge;
class Machine;

class App {
public:
  int run(int argc, char** argv);

private:
  static void print_usage();
  static void print_loaded_cartridge(
      const std::filesystem::path& rom_path,
      const Cartridge& cartridge);
  static void print_machine_state(const Machine& machine);

  int run_self_test();
  int inspect_rom(const std::filesystem::path& rom_path);
  int run_rom_session(const std::filesystem::path& rom_path);

  bool init();
  void shutdown();
  void pump_events();

  SDL_Window* window_ = nullptr;
  bool running_ = false;
};

}  // namespace fn64
