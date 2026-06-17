#include <exception>
#include <iostream>

#include "bootstrap.hpp"
#include "cartridge.hpp"
#include "machine.hpp"

int main() {
  try {
    fn64::Machine machine(fn64::Cartridge{});
    fn64::run_bootstrap_demos(machine);
    std::cout << "fn64 self-test: PASS\n";
    return 0;
  } catch (const std::exception& exception) {
    std::cerr << "fn64 self-test: FAIL\n"
              << exception.what() << '\n';
    return 1;
  }
}
