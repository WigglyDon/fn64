#pragma once

struct SDL_Window;

namespace fn64 {

class App {
public:
  int run(int argc, char** argv);

private:
  bool init();
  void shutdown();
  void pump_events();

  SDL_Window* window_ = nullptr;
  bool running_ = false;
};

}  // namespace fn64
