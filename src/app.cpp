#include "app.hpp"

#include <SDL3/SDL.h>

#include <exception>
#include <filesystem>
#include <iostream>
#include <string>
#include <utility>

#include "bootstrap.hpp"
#include "cartridge.hpp"
#include "machine.hpp"

namespace fn64 {

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
    run_bootstrap_demos(machine);

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