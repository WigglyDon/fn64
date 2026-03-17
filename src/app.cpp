#include "app.hpp"

#include <SDL3/SDL.h>

#include <cstdlib>
#include <iostream>
#include <string>

#include "rom.hpp"

namespace fn64 {
namespace {
constexpr int kWindowWidth = 640;
constexpr int kWindowHeight = 480;
constexpr char kWindowTitle[] = "fn64";
}

int App::run(int argc, char** argv) {
  if (argc > 2) {
    std::cerr << "usage: fn64 [path-to-rom]\n";
    return EXIT_FAILURE;
  }

  if (argc == 2) {
    Rom rom;
    std::string error;
    if (!load_rom(argv[1], rom, error)) {
      std::cerr << "failed to load ROM: " << error << '\n';
      return EXIT_FAILURE;
    }

    std::cout << "loaded ROM: " << rom.path.string()
              << " (" << rom.bytes.size() << " bytes)\n";
  } else {
    std::cout << "no ROM path provided; opening empty shell window\n";
  }

  if (!init()) {
    return EXIT_FAILURE;
  }

  while (running_) {
    pump_events();
    SDL_Delay(16);
  }

  shutdown();
  return EXIT_SUCCESS;
}

bool App::init() {
  if (!SDL_Init(SDL_INIT_VIDEO)) {
    std::cerr << "SDL_Init failed: " << SDL_GetError() << '\n';
    return false;
  }

  window_ = SDL_CreateWindow(kWindowTitle, kWindowWidth, kWindowHeight, 0);
  if (window_ == nullptr) {
    std::cerr << "SDL_CreateWindow failed: " << SDL_GetError() << '\n';
    shutdown();
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

}  // namespace fn64
