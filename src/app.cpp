#include "app.hpp"

#include <SDL3/SDL.h>

#include <cstdint>
#include <exception>
#include <filesystem>
#include <iomanip>
#include <iostream>
#include <string>
#include <utility>

#include "cartridge.hpp"
#include "machine.hpp"

namespace fn64 {
namespace {

constexpr std::uint32_t encode_special(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint8_t rd,
    std::uint8_t sa,
    std::uint8_t funct) {
  return (static_cast<std::uint32_t>(0x00) << 26) |
         (static_cast<std::uint32_t>(rs) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         (static_cast<std::uint32_t>(rd) << 11) |
         (static_cast<std::uint32_t>(sa) << 6) |
         static_cast<std::uint32_t>(funct);
}

constexpr std::uint32_t encode_multu(std::uint8_t rs, std::uint8_t rt) {
  return encode_special(rs, rt, 0x00, 0x00, 0x19);
}

void print_hex64(const char* label, std::uint64_t value) {
  std::cout << label << " = 0x"
            << std::hex << std::setw(16) << std::setfill('0') << value
            << std::dec << std::setfill(' ') << '\n';
}

void print_demo_cpu_state(const char* heading, const Machine& machine) {
  std::cout << heading << '\n';
  print_hex64("  pc", machine.cpu_pc());
  print_hex64("  hi", machine.cpu_hi());
  print_hex64("  lo", machine.cpu_lo());
  print_hex64("  gpr[1]", machine.read_cpu_gpr(1));
  print_hex64("  gpr[2]", machine.read_cpu_gpr(2));
  print_hex64("  gpr[3]", machine.read_cpu_gpr(3));
}

void run_single_step_demo(Machine& machine) {
  constexpr std::uint32_t kDemoInstructionAddress = 0x00000000;
  constexpr std::uint32_t kDemoInstruction = encode_multu(1, 2);

  machine.write_cpu_pc(kDemoInstructionAddress);
  machine.write_cpu_hi(0xaaaaaaaa);
  machine.write_cpu_lo(0x55555555);
  machine.write_cpu_gpr(1, 0xffffffff);
  machine.write_cpu_gpr(2, 0x00000002);
  machine.write_cpu_gpr(3, 0);
  machine.write_rdram_u32_be(kDemoInstructionAddress, kDemoInstruction);

  std::cout << "fn64 bootstrap single-step demo\n";
  std::cout << "  instruction_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kDemoInstruction
            << std::dec << std::setfill(' ') << '\n';

  print_demo_cpu_state("before step:", machine);

  const auto step_result = machine.step_cpu_instruction();
  (void)step_result;

  print_demo_cpu_state("after step:", machine);
}

}  // namespace

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

int App::run(int argc, char** argv) {
  if (argc < 2) {
    std::cerr << "usage: fn64 <rom-path>\n";
    return 1;
  }

  if (!init()) {
    return 1;
  }

  try {
    const std::filesystem::path rom_path = argv[1];

    Cartridge cartridge;
    std::string error;
    if (!load_cartridge(rom_path, cartridge, error)) {
      std::cerr << error << '\n';
      shutdown();
      return 1;
    }

    Machine machine(std::move(cartridge));

    run_single_step_demo(machine);

    while (running_) {
      pump_events();
      SDL_Delay(16);
    }
  } catch (const std::exception& exception) {
    std::cerr << exception.what() << '\n';
    shutdown();
    return 1;
  }

  shutdown();
  return 0;
}

}  // namespace fn64