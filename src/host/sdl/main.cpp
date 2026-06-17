#include <SDL3/SDL_main.h>

#include "app.hpp"

int main(int argc, char** argv) {
  fn64::App app;
  return app.run(argc, argv);
}
