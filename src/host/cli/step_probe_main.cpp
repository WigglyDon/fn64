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

constexpr std::uint32_t encode_lui(
    std::uint8_t rt,
    std::uint16_t immediate) {
  return (0x0fu << 26) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(immediate);
}

constexpr std::uint32_t encode_lw(
    std::uint8_t rt,
    std::uint8_t base,
    std::uint16_t offset) {
  return (0x23u << 26) |
         (static_cast<std::uint32_t>(base) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(offset);
}

constexpr std::uint32_t encode_lbu(
    std::uint8_t rt,
    std::uint8_t base,
    std::uint16_t offset) {
  return (0x24u << 26) |
         (static_cast<std::uint32_t>(base) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(offset);
}

constexpr std::uint32_t encode_sb(
    std::uint8_t rt,
    std::uint8_t base,
    std::uint16_t offset) {
  return (0x28u << 26) |
         (static_cast<std::uint32_t>(base) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(offset);
}

constexpr std::uint32_t encode_sw(
    std::uint8_t rt,
    std::uint8_t base,
    std::uint16_t offset) {
  return (0x2bu << 26) |
         (static_cast<std::uint32_t>(base) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(offset);
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
      << "  gpr[4]: " << hex_u32(machine.inspect_cpu_gpr(4)) << '\n'
      << "  gpr[5]: " << hex_u32(machine.inspect_cpu_gpr(5)) << '\n'
      << "  gpr[6]: " << hex_u32(machine.inspect_cpu_gpr(6)) << '\n'
      << "  gpr[7]: " << hex_u32(machine.inspect_cpu_gpr(7)) << '\n'
      << "  gpr[8]: " << hex_u32(machine.inspect_cpu_gpr(8)) << '\n';
}

void print_fetch_view(
    const char* label,
    const fn64::Machine& machine,
    std::uint32_t rdram_offset) {
  std::cout
      << label << '\n'
      << "  fetch CPU pc: " << hex_u32(machine.cpu_pc()) << '\n'
      << "  staged RDRAM offset: " << hex_u32(rdram_offset) << '\n'
      << "  visible RDRAM word at offset: "
      << hex_u32(machine.inspect_rdram_u32_be(rdram_offset)) << '\n';
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
    constexpr std::uint32_t kCpuRdramAliasBase = 0x80000000u;
    constexpr std::uint32_t kLuiRdramOffset = 0x00000000u;
    constexpr std::uint32_t kOriRdramOffset = 0x00000004u;
    constexpr std::uint32_t kSwRdramOffset = 0x00000008u;
    constexpr std::uint32_t kLwRdramOffset = 0x0000000cu;
    constexpr std::uint32_t kByteOriRdramOffset = 0x00000010u;
    constexpr std::uint32_t kSbRdramOffset = 0x00000014u;
    constexpr std::uint32_t kLbuRdramOffset = 0x00000018u;
    constexpr std::uint32_t kBreakRdramOffset = 0x0000001cu;
    constexpr std::uint32_t kByteDataRdramOffset = 0x00000100u;
    constexpr std::uint32_t kWordDataRdramOffset = 0x00000104u;
    constexpr std::uint32_t kLuiCpuAddress = kCpuRdramAliasBase + kLuiRdramOffset;
    constexpr std::uint32_t kOriCpuAddress = kCpuRdramAliasBase + kOriRdramOffset;
    constexpr std::uint32_t kSwCpuAddress = kCpuRdramAliasBase + kSwRdramOffset;
    constexpr std::uint32_t kLwCpuAddress = kCpuRdramAliasBase + kLwRdramOffset;
    constexpr std::uint32_t kByteOriCpuAddress =
        kCpuRdramAliasBase + kByteOriRdramOffset;
    constexpr std::uint32_t kSbCpuAddress = kCpuRdramAliasBase + kSbRdramOffset;
    constexpr std::uint32_t kLbuCpuAddress = kCpuRdramAliasBase + kLbuRdramOffset;
    constexpr std::uint32_t kBreakCpuAddress = kCpuRdramAliasBase + kBreakRdramOffset;
    constexpr std::uint32_t kByteDataCpuAddress =
        kCpuRdramAliasBase + kByteDataRdramOffset + 2u;
    constexpr std::uint32_t kWordDataCpuAddress =
        kCpuRdramAliasBase + kWordDataRdramOffset;
    constexpr std::uint32_t kAfterBreakPc = kCpuRdramAliasBase + 0x00000020u;
    constexpr std::uint32_t kAfterBreakNextPc = kCpuRdramAliasBase + 0x00000024u;
    constexpr std::uint16_t kByteImmediateOffset = 0x0102u;
    constexpr std::uint16_t kWordImmediateOffset = 0x0104u;
    constexpr std::uint32_t kInitialByteDataWord = 0x00000000u;
    constexpr std::uint32_t kInitialWordDataWord = 0xDEADBEEFu;
    constexpr std::uint32_t kExpectedGpr4 = 0x00001234u;
    constexpr std::uint32_t kExpectedGpr5 = 0x00001234u;
    constexpr std::uint32_t kExpectedGpr7 = 0x000000abu;
    constexpr std::uint32_t kExpectedGpr8 = 0x000000abu;
    constexpr std::uint32_t kExpectedByteDataWord = 0x0000ab00u;

    const std::uint32_t lui_instruction = encode_lui(6, 0x8000u);
    const std::uint32_t ori_instruction = encode_ori(4, 0, 0x1234u);
    const std::uint32_t sw_instruction = encode_sw(4, 6, kWordImmediateOffset);
    const std::uint32_t lw_instruction = encode_lw(5, 6, kWordImmediateOffset);
    const std::uint32_t byte_ori_instruction = encode_ori(7, 0, 0x00abu);
    const std::uint32_t sb_instruction = encode_sb(7, 6, kByteImmediateOffset);
    const std::uint32_t lbu_instruction = encode_lbu(8, 6, kByteImmediateOffset);
    const std::uint32_t break_instruction = encode_break();

    machine.stage_cpu_pc(kLuiCpuAddress);
    machine.stage_rdram_u32_be(kLuiRdramOffset, lui_instruction);
    machine.stage_rdram_u32_be(kOriRdramOffset, ori_instruction);
    machine.stage_rdram_u32_be(kSwRdramOffset, sw_instruction);
    machine.stage_rdram_u32_be(kLwRdramOffset, lw_instruction);
    machine.stage_rdram_u32_be(kByteOriRdramOffset, byte_ori_instruction);
    machine.stage_rdram_u32_be(kSbRdramOffset, sb_instruction);
    machine.stage_rdram_u32_be(kLbuRdramOffset, lbu_instruction);
    machine.stage_rdram_u32_be(kBreakRdramOffset, break_instruction);
    machine.stage_rdram_u32_be(kByteDataRdramOffset, kInitialByteDataWord);
    machine.stage_rdram_u32_be(kWordDataRdramOffset, kInitialWordDataWord);

    std::cout
        << "\nstaged synthetic RDRAM instructions and CPU aliases\n"
        << "  fetch CPU alias base: " << hex_u32(kCpuRdramAliasBase) << '\n'
        << "  word data CPU address: " << hex_u32(kWordDataCpuAddress) << '\n'
        << "  word data RDRAM offset staged/inspected: "
        << hex_u32(kWordDataRdramOffset) << '\n'
        << "  byte data CPU address: " << hex_u32(kByteDataCpuAddress) << '\n'
        << "  byte data RDRAM offset staged/inspected word base: "
        << hex_u32(kByteDataRdramOffset) << '\n'
        << "  rdram[0x00000000]: " << hex_u32(machine.inspect_rdram_u32_be(kLuiRdramOffset)) << '\n'
        << "  rdram[0x00000004]: " << hex_u32(machine.inspect_rdram_u32_be(kOriRdramOffset)) << '\n'
        << "  rdram[0x00000008]: " << hex_u32(machine.inspect_rdram_u32_be(kSwRdramOffset)) << '\n'
        << "  rdram[0x0000000C]: " << hex_u32(machine.inspect_rdram_u32_be(kLwRdramOffset)) << '\n'
        << "  rdram[0x00000010]: "
        << hex_u32(machine.inspect_rdram_u32_be(kByteOriRdramOffset)) << '\n'
        << "  rdram[0x00000014]: " << hex_u32(machine.inspect_rdram_u32_be(kSbRdramOffset)) << '\n'
        << "  rdram[0x00000018]: " << hex_u32(machine.inspect_rdram_u32_be(kLbuRdramOffset)) << '\n'
        << "  rdram[0x0000001C]: "
        << hex_u32(machine.inspect_rdram_u32_be(kBreakRdramOffset)) << '\n'
        << "  byte data rdram[0x00000100] before steps: "
        << hex_u32(machine.inspect_rdram_u32_be(kByteDataRdramOffset)) << '\n'
        << "  word data rdram[0x00000104] before steps: "
        << hex_u32(machine.inspect_rdram_u32_be(kWordDataRdramOffset)) << '\n';

    print_machine_state("\nbefore step 1", machine);
    print_fetch_view("step 1 fetch view", machine, kLuiRdramOffset);

    const fn64::Machine::CpuInstructionStepResult first_result =
        machine.step_cpu_instruction();
    std::cout << "step 1 result: " << step_result_name(first_result) << '\n';
    print_machine_state("after step 1", machine);

    require_step_result(
        "step 1",
        first_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 1 pc", machine.cpu_pc(), kOriCpuAddress);
    require_equal("step 1 next pc", machine.cpu_next_pc(), kSwCpuAddress);
    require_equal("step 1 gpr[4]", machine.inspect_cpu_gpr(4), 0);
    require_equal("step 1 gpr[5]", machine.inspect_cpu_gpr(5), 0);
    require_equal("step 1 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 1 gpr[7]", machine.inspect_cpu_gpr(7), 0);
    require_equal("step 1 gpr[8]", machine.inspect_cpu_gpr(8), 0);
    require_equal(
        "step 1 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kInitialByteDataWord);
    require_equal(
        "step 1 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kInitialWordDataWord);

    print_fetch_view("step 2 fetch view", machine, kOriRdramOffset);
    const fn64::Machine::CpuInstructionStepResult second_result =
        machine.step_cpu_instruction();
    std::cout << "step 2 result: " << step_result_name(second_result) << '\n';
    print_machine_state("after step 2", machine);

    require_step_result(
        "step 2",
        second_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 2 pc", machine.cpu_pc(), kSwCpuAddress);
    require_equal("step 2 next pc", machine.cpu_next_pc(), kLwCpuAddress);
    require_equal("step 2 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);
    require_equal("step 2 gpr[5]", machine.inspect_cpu_gpr(5), 0);
    require_equal("step 2 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 2 gpr[7]", machine.inspect_cpu_gpr(7), 0);
    require_equal("step 2 gpr[8]", machine.inspect_cpu_gpr(8), 0);
    require_equal(
        "step 2 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kInitialByteDataWord);
    require_equal(
        "step 2 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kInitialWordDataWord);

    print_fetch_view("step 3 fetch view", machine, kSwRdramOffset);
    const fn64::Machine::CpuInstructionStepResult third_result =
        machine.step_cpu_instruction();
    std::cout << "step 3 result: " << step_result_name(third_result) << '\n';
    print_machine_state("after step 3", machine);

    require_step_result(
        "step 3",
        third_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 3 pc", machine.cpu_pc(), kLwCpuAddress);
    require_equal("step 3 next pc", machine.cpu_next_pc(), kByteOriCpuAddress);
    require_equal("step 3 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);
    require_equal("step 3 gpr[5]", machine.inspect_cpu_gpr(5), 0);
    require_equal("step 3 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 3 gpr[7]", machine.inspect_cpu_gpr(7), 0);
    require_equal("step 3 gpr[8]", machine.inspect_cpu_gpr(8), 0);
    require_equal(
        "step 3 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kInitialByteDataWord);
    require_equal(
        "step 3 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kExpectedGpr4);

    print_fetch_view("step 4 fetch view", machine, kLwRdramOffset);
    const fn64::Machine::CpuInstructionStepResult fourth_result =
        machine.step_cpu_instruction();
    std::cout << "step 4 result: " << step_result_name(fourth_result) << '\n';
    print_machine_state("after step 4", machine);

    require_step_result(
        "step 4",
        fourth_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 4 pc", machine.cpu_pc(), kByteOriCpuAddress);
    require_equal("step 4 next pc", machine.cpu_next_pc(), kSbCpuAddress);
    require_equal("step 4 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);
    require_equal("step 4 gpr[5]", machine.inspect_cpu_gpr(5), kExpectedGpr5);
    require_equal("step 4 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 4 gpr[7]", machine.inspect_cpu_gpr(7), 0);
    require_equal("step 4 gpr[8]", machine.inspect_cpu_gpr(8), 0);
    require_equal(
        "step 4 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kInitialByteDataWord);
    require_equal(
        "step 4 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kExpectedGpr4);

    print_fetch_view("step 5 fetch view", machine, kByteOriRdramOffset);
    const fn64::Machine::CpuInstructionStepResult fifth_result =
        machine.step_cpu_instruction();
    std::cout << "step 5 result: " << step_result_name(fifth_result) << '\n';
    print_machine_state("after step 5", machine);

    require_step_result(
        "step 5",
        fifth_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 5 pc", machine.cpu_pc(), kSbCpuAddress);
    require_equal("step 5 next pc", machine.cpu_next_pc(), kLbuCpuAddress);
    require_equal("step 5 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);
    require_equal("step 5 gpr[5]", machine.inspect_cpu_gpr(5), kExpectedGpr5);
    require_equal("step 5 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 5 gpr[7]", machine.inspect_cpu_gpr(7), kExpectedGpr7);
    require_equal("step 5 gpr[8]", machine.inspect_cpu_gpr(8), 0);
    require_equal(
        "step 5 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kInitialByteDataWord);
    require_equal(
        "step 5 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kExpectedGpr4);

    print_fetch_view("step 6 fetch view", machine, kSbRdramOffset);
    const fn64::Machine::CpuInstructionStepResult sixth_result =
        machine.step_cpu_instruction();
    std::cout << "step 6 result: " << step_result_name(sixth_result) << '\n';
    print_machine_state("after step 6", machine);

    require_step_result(
        "step 6",
        sixth_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 6 pc", machine.cpu_pc(), kLbuCpuAddress);
    require_equal("step 6 next pc", machine.cpu_next_pc(), kBreakCpuAddress);
    require_equal("step 6 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);
    require_equal("step 6 gpr[5]", machine.inspect_cpu_gpr(5), kExpectedGpr5);
    require_equal("step 6 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 6 gpr[7]", machine.inspect_cpu_gpr(7), kExpectedGpr7);
    require_equal("step 6 gpr[8]", machine.inspect_cpu_gpr(8), 0);
    require_equal(
        "step 6 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kExpectedByteDataWord);
    require_equal(
        "step 6 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kExpectedGpr4);

    print_fetch_view("step 7 fetch view", machine, kLbuRdramOffset);
    const fn64::Machine::CpuInstructionStepResult seventh_result =
        machine.step_cpu_instruction();
    std::cout << "step 7 result: " << step_result_name(seventh_result) << '\n';
    print_machine_state("after step 7", machine);

    require_step_result(
        "step 7",
        seventh_result,
        fn64::Machine::CpuInstructionStepResult::kStepped);
    require_equal("step 7 pc", machine.cpu_pc(), kBreakCpuAddress);
    require_equal("step 7 next pc", machine.cpu_next_pc(), kAfterBreakPc);
    require_equal("step 7 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);
    require_equal("step 7 gpr[5]", machine.inspect_cpu_gpr(5), kExpectedGpr5);
    require_equal("step 7 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 7 gpr[7]", machine.inspect_cpu_gpr(7), kExpectedGpr7);
    require_equal("step 7 gpr[8]", machine.inspect_cpu_gpr(8), kExpectedGpr8);
    require_equal(
        "step 7 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kExpectedByteDataWord);
    require_equal(
        "step 7 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kExpectedGpr4);

    print_fetch_view("step 8 fetch view", machine, kBreakRdramOffset);
    const fn64::Machine::CpuInstructionStepResult eighth_result =
        machine.step_cpu_instruction();
    std::cout << "step 8 result: " << step_result_name(eighth_result) << '\n';
    print_machine_state("after step 8", machine);

    require_step_result(
        "step 8",
        eighth_result,
        fn64::Machine::CpuInstructionStepResult::kStopped);
    require_equal("step 8 pc", machine.cpu_pc(), kAfterBreakPc);
    require_equal("step 8 next pc", machine.cpu_next_pc(), kAfterBreakNextPc);
    require_equal("step 8 gpr[4]", machine.inspect_cpu_gpr(4), kExpectedGpr4);
    require_equal("step 8 gpr[5]", machine.inspect_cpu_gpr(5), kExpectedGpr5);
    require_equal("step 8 gpr[6]", machine.inspect_cpu_gpr(6), kCpuRdramAliasBase);
    require_equal("step 8 gpr[7]", machine.inspect_cpu_gpr(7), kExpectedGpr7);
    require_equal("step 8 gpr[8]", machine.inspect_cpu_gpr(8), kExpectedGpr8);
    require_equal(
        "step 8 byte data word",
        machine.inspect_rdram_u32_be(kByteDataRdramOffset),
        kExpectedByteDataWord);
    require_equal(
        "step 8 word data word",
        machine.inspect_rdram_u32_be(kWordDataRdramOffset),
        kExpectedGpr4);

    std::cout
        << "final byte data rdram[0x00000100]: "
        << hex_u32(machine.inspect_rdram_u32_be(kByteDataRdramOffset)) << '\n'
        << "final word data rdram[0x00000104]: "
        << hex_u32(machine.inspect_rdram_u32_be(kWordDataRdramOffset)) << '\n'
        << "final lbu gpr[8]: " << hex_u32(machine.inspect_cpu_gpr(8)) << '\n';

    std::cout << "\nprobe result: PASS\n";
    return 0;
  } catch (const std::exception& exception) {
    std::cerr << "fn64_step_probe: " << exception.what() << '\n';
    return 1;
  }
}
