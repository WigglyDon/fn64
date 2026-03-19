#include "app.hpp"

#include <SDL3/SDL.h>

#include <cstdint>
#include <exception>
#include <filesystem>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>
#include <utility>

#include "cartridge.hpp"
#include "machine.hpp"

namespace fn64 {
namespace {

constexpr std::uint32_t encode_i_type(
    std::uint8_t opcode,
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint16_t immediate) {
  return (static_cast<std::uint32_t>(opcode) << 26) |
         (static_cast<std::uint32_t>(rs) << 21) |
         (static_cast<std::uint32_t>(rt) << 16) |
         static_cast<std::uint32_t>(immediate);
}

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

constexpr std::uint32_t encode_ori(
    std::uint8_t rt,
    std::uint8_t rs,
    std::uint16_t immediate) {
  return encode_i_type(0x0d, rs, rt, immediate);
}

constexpr std::uint32_t encode_jalr(
    std::uint8_t rd,
    std::uint8_t rs) {
  return encode_special(rs, 0, rd, 0, 0x09);
}

constexpr std::uint32_t encode_break() {
  return encode_special(0, 0, 0, 0, 0x0d);
}

constexpr std::uint32_t encode_sync() {
  return encode_special(0, 0, 0, 0, 0x0f);
}

void print_hex64(const char* label, std::uint64_t value) {
  std::cout << label << " = 0x"
            << std::hex << std::setw(16) << std::setfill('0') << value
            << std::dec << std::setfill(' ') << '\n';
}

void print_control_flow_state(const Machine& machine) {
  print_hex64("  pc", machine.cpu_pc());
  print_hex64("  next_pc", machine.cpu_next_pc());
}

void require_stepped(Machine::CpuInstructionStepResult result, const char* label) {
  if (result != Machine::CpuInstructionStepResult::kStepped) {
    throw std::runtime_error(std::string("bootstrap demo step failed: ") + label);
  }
}

void require_stopped(Machine::CpuInstructionStepResult result, const char* label) {
  if (result != Machine::CpuInstructionStepResult::kStopped) {
    throw std::runtime_error(std::string("bootstrap demo stop failed: ") + label);
  }
}

void run_break_stop_demo(Machine& machine) {
  constexpr std::uint32_t kSetupAddress = 0x000000c0u;
  constexpr std::uint32_t kBreakAddress = 0x000000c4u;
  constexpr std::uint32_t kAfterBreakAddress = 0x000000c8u;

  constexpr std::uint32_t kSetupInstruction = encode_ori(13, 0, 0x1234);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kSetupAddress);
  machine.write_cpu_gpr(13, 0);

  machine.write_rdram_u32_be(kSetupAddress, kSetupInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap break demo: explicit local stop\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);

  require_stepped(machine.step_cpu_instruction(), "break_demo_setup");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[13]", machine.read_cpu_gpr(13));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("break demo did not advance to BREAK");
  }

  const std::uint32_t break_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord break_instruction =
      Machine::decode_cpu_instruction_word(break_raw);
  const Machine::CpuInstructionIdentity break_identity =
      Machine::identify_cpu_instruction(break_instruction);

  std::cout << "  break_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << break_raw
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  break_identity = "
            << Machine::cpu_instruction_identity_name(break_identity) << '\n';

  if (break_identity != Machine::CpuInstructionIdentity::kSpecialBreak) {
    throw std::runtime_error("break demo did not identify BREAK explicitly");
  }

  require_stopped(machine.step_cpu_instruction(), "break_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);

  if (machine.cpu_pc() != kAfterBreakAddress) {
    throw std::runtime_error("break demo did not advance past the executed BREAK");
  }
}

void run_sync_noop_demo(Machine& machine) {
  constexpr std::uint32_t kSetupAddress = 0x000000e0u;
  constexpr std::uint32_t kSyncAddress = 0x000000e4u;
  constexpr std::uint32_t kAfterSyncAddress = 0x000000e8u;
  constexpr std::uint32_t kBreakAddress = 0x000000ecu;

  constexpr std::uint32_t kSetupInstruction = encode_ori(14, 0, 0x1357);
  constexpr std::uint32_t kSyncInstruction = encode_sync();
  constexpr std::uint32_t kAfterSyncInstruction = encode_ori(15, 0, 0x2468);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kSetupAddress);
  machine.write_cpu_gpr(14, 0);
  machine.write_cpu_gpr(15, 0);

  machine.write_rdram_u32_be(kSetupAddress, kSetupInstruction);
  machine.write_rdram_u32_be(kSyncAddress, kSyncInstruction);
  machine.write_rdram_u32_be(kAfterSyncAddress, kAfterSyncInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap sync demo: explicit local no-op\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[14]", machine.read_cpu_gpr(14));
  print_hex64("  gpr[15]", machine.read_cpu_gpr(15));

  require_stepped(machine.step_cpu_instruction(), "sync_demo_setup");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[14]", machine.read_cpu_gpr(14));
  print_hex64("  gpr[15]", machine.read_cpu_gpr(15));

  if (machine.cpu_pc() != kSyncAddress) {
    throw std::runtime_error("sync demo did not advance to SYNC");
  }

  const std::uint32_t sync_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord sync_instruction =
      Machine::decode_cpu_instruction_word(sync_raw);
  const Machine::CpuInstructionIdentity sync_identity =
      Machine::identify_cpu_instruction(sync_instruction);

  std::cout << "  sync_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << sync_raw
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  sync_identity = "
            << Machine::cpu_instruction_identity_name(sync_identity) << '\n';

  if (sync_identity != Machine::CpuInstructionIdentity::kSpecialSync) {
    throw std::runtime_error("sync demo did not identify SYNC explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "sync_demo_sync");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[14]", machine.read_cpu_gpr(14));
  print_hex64("  gpr[15]", machine.read_cpu_gpr(15));

  if (machine.cpu_pc() != kAfterSyncAddress) {
    throw std::runtime_error("sync demo did not advance past executed SYNC");
  }

  if (machine.read_cpu_gpr(14) != 0x00001357u) {
    throw std::runtime_error("sync demo unexpectedly changed gpr[14]");
  }

  if (machine.read_cpu_gpr(15) != 0) {
    throw std::runtime_error("sync demo unexpectedly changed gpr[15]");
  }

  require_stepped(machine.step_cpu_instruction(), "sync_demo_after_sync");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[14]", machine.read_cpu_gpr(14));
  print_hex64("  gpr[15]", machine.read_cpu_gpr(15));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("sync demo did not advance to the BREAK sentinel");
  }

  if (machine.read_cpu_gpr(15) != 0x00002468u) {
    throw std::runtime_error("sync demo post-SYNC instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "sync_demo_break_stop");
}

void run_jalr_encoded_rd_demo(Machine& machine) {
  constexpr std::uint32_t kLoadTargetAddress = 0x00000000u;
  constexpr std::uint32_t kJalrAddress = 0x00000004u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000008u;
  constexpr std::uint32_t kLinkReturnAddress = 0x0000000cu;
  constexpr std::uint32_t kTargetAddress = 0x00000010u;
  constexpr std::uint32_t kSentinelAddress = 0x00000014u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(4, 0, 0x0010);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(7, 4);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(5, 0, 0x5555);
  constexpr std::uint32_t kTargetInstruction = encode_ori(6, 0, 0x6666);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kLoadTargetAddress);
  machine.write_cpu_gpr(4, 0);
  machine.write_cpu_gpr(5, 0);
  machine.write_cpu_gpr(6, 0);
  machine.write_cpu_gpr(7, 0);
  machine.write_cpu_gpr(31, 0);

  machine.write_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.write_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kLinkReturnAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 1: encoded rd link register\n";
  std::cout << "  ori_target_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kLoadTargetInstruction
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  delay_slot_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kDelaySlotInstruction
            << std::dec << std::setfill(' ') << '\n';

  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(4));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(7));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_load_target");
  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(4));

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_issue_jalr");
  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[7]", machine.read_cpu_gpr(7));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  if (machine.cpu_pc() != kDelaySlotAddress) {
    throw std::runtime_error("jalr demo 1 did not move into the delay slot");
  }

  if (machine.cpu_next_pc() != kTargetAddress) {
    throw std::runtime_error("jalr demo 1 did not schedule the register target");
  }

  if (machine.read_cpu_gpr(7) != kLinkReturnAddress) {
    throw std::runtime_error("jalr demo 1 wrote the wrong link address");
  }

  if (machine.read_cpu_gpr(31) != 0) {
    throw std::runtime_error("jalr demo 1 unexpectedly touched gpr[31]");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_delay_slot");
  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.read_cpu_gpr(5));

  if (machine.cpu_pc() != kTargetAddress) {
    throw std::runtime_error("jalr demo 1 delay slot did not hand off to the target");
  }

  if (machine.read_cpu_gpr(5) != 0x00005555u) {
    throw std::runtime_error("jalr demo 1 delay slot did not execute");
  }

  require_stepped(machine.step_cpu_instruction(), "jalr_demo1_target");
  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(6));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error("jalr demo 1 ended at the wrong sentinel");
  }

  if (machine.read_cpu_gpr(6) != 0x00006666u) {
    throw std::runtime_error("jalr demo 1 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo1_break_stop");
}

void run_jalr_rd31_demo(Machine& machine) {
  constexpr std::uint32_t kLoadTargetAddress = 0x00000040u;
  constexpr std::uint32_t kJalrAddress = 0x00000044u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000048u;
  constexpr std::uint32_t kLinkReturnAddress = 0x0000004cu;
  constexpr std::uint32_t kTargetAddress = 0x00000050u;
  constexpr std::uint32_t kSentinelAddress = 0x00000054u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(4, 0, 0x0050);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(31, 4);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(8, 0, 0x8888);
  constexpr std::uint32_t kTargetInstruction = encode_ori(9, 0, 0x9999);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kLoadTargetAddress);
  machine.write_cpu_gpr(4, 0);
  machine.write_cpu_gpr(8, 0);
  machine.write_cpu_gpr(9, 0);
  machine.write_cpu_gpr(31, 0);

  machine.write_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.write_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kLinkReturnAddress, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 2: rd = 31 normal link case\n";
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';

  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_load_target");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_issue_jalr");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_delay_slot");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo2_target");

  print_control_flow_state(machine);
  print_hex64("  gpr[8]", machine.read_cpu_gpr(8));
  print_hex64("  gpr[9]", machine.read_cpu_gpr(9));
  print_hex64("  gpr[31]", machine.read_cpu_gpr(31));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error("jalr demo 2 ended at the wrong sentinel");
  }

  if (machine.read_cpu_gpr(31) != kLinkReturnAddress) {
    throw std::runtime_error("jalr demo 2 wrote the wrong return address into gpr[31]");
  }

  if (machine.read_cpu_gpr(8) != 0x00008888u) {
    throw std::runtime_error("jalr demo 2 delay slot did not execute");
  }

  if (machine.read_cpu_gpr(9) != 0x00009999u) {
    throw std::runtime_error("jalr demo 2 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo2_break_stop");
}

void run_jalr_rd0_demo(Machine& machine) {
  constexpr std::uint32_t kLoadTargetAddress = 0x00000080u;
  constexpr std::uint32_t kJalrAddress = 0x00000084u;
  constexpr std::uint32_t kDelaySlotAddress = 0x00000088u;
  constexpr std::uint32_t kTargetAddress = 0x00000090u;
  constexpr std::uint32_t kSentinelAddress = 0x00000094u;

  constexpr std::uint32_t kLoadTargetInstruction = encode_ori(10, 0, 0x0090);
  constexpr std::uint32_t kJalrInstruction = encode_jalr(0, 10);
  constexpr std::uint32_t kDelaySlotInstruction = encode_ori(11, 0, 0xabcd);
  constexpr std::uint32_t kTargetInstruction = encode_ori(12, 0, 0xdcba);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kLoadTargetAddress);
  machine.write_cpu_gpr(10, 0);
  machine.write_cpu_gpr(11, 0);
  machine.write_cpu_gpr(12, 0);

  machine.write_rdram_u32_be(kLoadTargetAddress, kLoadTargetInstruction);
  machine.write_rdram_u32_be(kJalrAddress, kJalrInstruction);
  machine.write_rdram_u32_be(kDelaySlotAddress, kDelaySlotInstruction);
  machine.write_rdram_u32_be(kJalrAddress + 8u, kBreakInstruction);
  machine.write_rdram_u32_be(kTargetAddress, kTargetInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap jalr demo 3: rd = 0 discards link through normal gpr[0] behavior\n";
  std::cout << "  jalr_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << kJalrInstruction
            << std::dec << std::setfill(' ') << '\n';

  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_load_target");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_issue_jalr");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_delay_slot");
  require_stepped(machine.step_cpu_instruction(), "jalr_demo3_target");

  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.read_cpu_gpr(0));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(11));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(12));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error("jalr demo 3 ended at the wrong sentinel");
  }

  if (machine.read_cpu_gpr(0) != 0) {
    throw std::runtime_error("jalr demo 3 unexpectedly changed gpr[0]");
  }

  if (machine.read_cpu_gpr(11) != 0x0000abcdu) {
    throw std::runtime_error("jalr demo 3 delay slot did not execute");
  }

  if (machine.read_cpu_gpr(12) != 0x0000dcbau) {
    throw std::runtime_error("jalr demo 3 target instruction did not execute");
  }

  require_stopped(machine.step_cpu_instruction(), "jalr_demo3_break_stop");
}

void run_single_step_demo(Machine& machine) {
  run_break_stop_demo(machine);
  run_sync_noop_demo(machine);
  run_jalr_encoded_rd_demo(machine);
  run_jalr_rd31_demo(machine);
  run_jalr_rd0_demo(machine);
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