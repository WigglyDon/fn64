#include "bootstrap_common.hpp"

#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {

void print_hex64(const char* label, std::uint64_t value) {
  std::cout << label << " = 0x"
            << std::hex << std::setw(16) << std::setfill('0') << value
            << std::dec << std::setfill(' ') << '\n';
}

void print_hex32(const char* label, std::uint32_t value) {
  std::cout << label << " = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << value
            << std::dec << std::setfill(' ') << '\n';
}

void print_control_flow_state(const Machine& machine) {
  print_hex64("  pc", machine.cpu_pc());
  print_hex64("  next_pc", machine.cpu_next_pc());
}

void print_rdram_word(const Machine& machine, const char* label, std::uint32_t address) {
  print_hex32(label, machine.read_rdram_u32_be(address));
}

void require_stepped(Machine::CpuInstructionStepResult result, const std::string& label) {
  if (result != Machine::CpuInstructionStepResult::kStepped) {
    throw std::runtime_error(std::string("bootstrap demo step failed: ") + label);
  }
}

void require_stopped(Machine::CpuInstructionStepResult result, const std::string& label) {
  if (result != Machine::CpuInstructionStepResult::kStopped) {
    throw std::runtime_error(std::string("bootstrap demo stop failed: ") + label);
  }
}

}  // namespace fn64::bootstrap_detail