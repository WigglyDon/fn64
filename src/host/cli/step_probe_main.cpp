#include <cstddef>
#include <cstdint>
#include <exception>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "cartridge.hpp"
#include "machine.hpp"

namespace {

std::string hex_u32(std::uint32_t value) {
  std::ostringstream stream;
  stream << "0x" << std::uppercase << std::hex << std::setw(8)
         << std::setfill('0') << value;
  return stream.str();
}

constexpr std::uint32_t encode_ori(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return (0x0du << 26) |
         (static_cast<std::uint32_t>(rs) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(immediate);
}

constexpr std::uint32_t encode_break() {
  return 0x0000000du;
}

std::vector<std::uint8_t> make_synthetic_cartridge_bytes() {
  std::vector<std::uint8_t> bytes(0x40, 0);
  bytes[0x00] = 0x80;
  bytes[0x01] = 0x37;
  bytes[0x02] = 0x12;
  bytes[0x03] = 0x40;
  bytes[0x3C] = 'S';
  bytes[0x3D] = 'P';
  bytes[0x3E] = 0x45;
  bytes[0x3F] = 1;

  const std::string image_name = "FN64 STEP PROBE";
  for (std::size_t i = 0; i < image_name.size(); ++i) {
    bytes[0x20 + i] = static_cast<std::uint8_t>(image_name[i]);
  }

  return bytes;
}

const char* step_result_name(fn64::Machine::CpuInstructionStepResult result) {
  switch (result) {
    case fn64::Machine::CpuInstructionStepResult::kStepped:
      return "kStepped";
    case fn64::Machine::CpuInstructionStepResult::kStopped:
      return "kStopped";
    case fn64::Machine::CpuInstructionStepResult::kUnsupported:
      return "kUnsupported";
  }

  return "unknown";
}

void print_machine_state(const char* label, const fn64::Machine& machine) {
  std::cout
      << label << '\n'
      << "  pc: " << hex_u32(machine.cpu_pc()) << '\n'
      << "  next pc: " << hex_u32(machine.cpu_next_pc()) << '\n'
      << "  gpr[4]: " << hex_u32(machine.inspect_cpu_gpr(4)) << '\n';
}

void require_equal(
    const char* label,
    std::uint32_t observed,
    std::uint32_t expected) {
  if (observed != expected) {
    throw std::runtime_error(
        std::string(label) +
        " expected " + hex_u32(expected) +
        " but observed " + hex_u32(observed));
  }
}

void require_step_result(
    const char* label,
    fn64::Machine::CpuInstructionStepResult observed,
    fn64::Machine::CpuInstructionStepResult expected) {
  if (observed != expected) {
    throw std::runtime_error(
        std::string(label) +
        " expected " + step_result_name(expected) +
        " but observed " + step_result_name(observed));
  }
}

void print_usage() {
  std::cerr << "usage: fn64_step_probe\n";
}

}  // namespace

int main(int argc, char** argv) {
  static_cast<void>(argv);

  if (argc != 1) {
    print_usage();
    return 1;
  }

  try {
    std::cout
        << "fn64 synthetic no-window Machine step probe\n"
        << "  no ROM path loaded\n"
        << "  synthetic cartridge bytes generated in memory only\n"
        << "  no cartridge bytes staged\n"
        << "  no cartridge execution mapping\n"
        << "  no boot/PIF/BIOS behavior\n"
        << "  no SDL/window runtime\n";

    fn64::Cartridge cartridge;
    std::string error;
    if (!fn64::load_cartridge(
            make_synthetic_cartridge_bytes(),
            cartridge,
            error)) {
      throw std::runtime_error("could not create synthetic cartridge: " + error);
    }

    fn64::Machine machine(std::move(cartridge));
    constexpr std::uint32_t kOriAddress = 0x00000000u;
    constexpr std::uint32_t kBreakAddress = 0x00000004u;
    constexpr std::uint32_t kAfterBreakPc = 0x00000008u;
    constexpr std::uint32_t kAfterBreakNextPc = 0x0000000cu;
    constexpr std::uint32_t kExpectedGpr4 = 0x00001234u;

    const std::uint32_t ori_instruction = encode_ori(4, 0, 0x1234u);
    const std::uint32_t break_instruction = encode_break();

    machine.stage_rdram_u32_be(kOriAddress, ori_instruction);
    machine.stage_rdram_u32_be(kBreakAddress, break_instruction);

    std::cout
        << "\nstaged synthetic RDRAM instructions\n"
        << "  rdram[0x00000000]: " << hex_u32(machine.inspect_rdram_u32_be(kOriAddress)) << '\n'
        << "  rdram[0x00000004]: " << hex_u32(machine.inspect_rdram_u32_be(kBreakAddress)) << '\n';

    print_machine_state("\nbefore step 1", machine);

    const fn64::Machine::CpuInstructionStepResult first_result =
        machine.step_cpu_instruction();
    std::cout << "step 1 result: " << step_result_name(first_result) << '\n';
    print_machine_state("after step 1", machine);

    require_step_result(
        "step 1",
        first_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 1 pc", machine.cpu_pc(), kBreakAddress);
    require_equal("step 1 next pc", machine.cpu_next_pc(), kAfterBreakPc);
    require_equal("step 1 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);

    const fn64::Machine::CpuInstructionStepResult second_result =
        machine.step_cpu_instruction();
    std::cout << "step 2 result: " << step_result_name(second_result) << '\n';
    print_machine_state("after step 2", machine);

    require_step_result(
        "step 2",
        second_result,
        fn64::Machine::CpuInstructionStepResult::kStopped);
    require_equal("step 2 pc", machine.cpu_pc(), kAfterBreakPc);
    require_equal("step 2 next pc", machine.cpu_next_pc(), kAfterBreakNextPc);
    require_equal("step 2 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);

    std::cout << "\nprobe result: PASS\n";
    return 0;
  } catch (const std::exception& exception) {
    std::cerr << "fn64_step_probe: " << exception.what() << '\n';
    return 1;
  }
}
