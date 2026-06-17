#include "bootstrap_common.hpp"

#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

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

void run_syscall_stop_demo(Machine& machine) {
  constexpr std::uint32_t kSetupAddress = 0x00000100u;
  constexpr std::uint32_t kSyscallAddress = 0x00000104u;
  constexpr std::uint32_t kAfterSyscallAddress = 0x00000108u;
  constexpr std::uint32_t kAfterAfterSyscallAddress = 0x0000010cu;

  constexpr std::uint32_t kSetupInstruction = encode_ori(16, 0, 0x4242);
  constexpr std::uint32_t kSyscallInstruction = encode_syscall();
  constexpr std::uint32_t kAfterSyscallInstruction = encode_ori(17, 0, 0x7171);

  machine.write_cpu_pc(kSetupAddress);
  machine.write_cpu_gpr(16, 0);
  machine.write_cpu_gpr(17, 0);

  machine.write_rdram_u32_be(kSetupAddress, kSetupInstruction);
  machine.write_rdram_u32_be(kSyscallAddress, kSyscallInstruction);
  machine.write_rdram_u32_be(kAfterSyscallAddress, kAfterSyscallInstruction);

  std::cout << "fn64 bootstrap syscall demo: explicit local stop\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[16]", machine.read_cpu_gpr(16));
  print_hex64("  gpr[17]", machine.read_cpu_gpr(17));

  require_stepped(machine.step_cpu_instruction(), "syscall_demo_setup");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[16]", machine.read_cpu_gpr(16));
  print_hex64("  gpr[17]", machine.read_cpu_gpr(17));

  if (machine.cpu_pc() != kSyscallAddress) {
    throw std::runtime_error("syscall demo did not advance to SYSCALL");
  }

  const std::uint32_t syscall_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord syscall_instruction =
      Machine::decode_cpu_instruction_word(syscall_raw);
  const Machine::CpuInstructionIdentity syscall_identity =
      Machine::identify_cpu_instruction(syscall_instruction);

  std::cout << "  syscall_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << syscall_raw
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  syscall_identity = "
            << Machine::cpu_instruction_identity_name(syscall_identity) << '\n';

  if (syscall_identity != Machine::CpuInstructionIdentity::kSpecialSyscall) {
    throw std::runtime_error("syscall demo did not identify SYSCALL explicitly");
  }

  require_stopped(machine.step_cpu_instruction(), "syscall_demo_syscall");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[16]", machine.read_cpu_gpr(16));
  print_hex64("  gpr[17]", machine.read_cpu_gpr(17));

  if (machine.cpu_pc() != kAfterSyscallAddress) {
    throw std::runtime_error("syscall demo did not advance past the executed SYSCALL");
  }

  if (machine.cpu_next_pc() != kAfterAfterSyscallAddress) {
    throw std::runtime_error("syscall demo did not preserve sequential next_pc after stop");
  }

  if (machine.read_cpu_gpr(16) != 0x00004242u) {
    throw std::runtime_error("syscall demo unexpectedly changed gpr[16]");
  }

  if (machine.read_cpu_gpr(17) != 0) {
    throw std::runtime_error("syscall demo unexpectedly executed the post-SYSCALL instruction");
  }
}

void run_special_register_trap_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t trap_instruction,
    Machine::CpuInstructionIdentity expected_identity,
    std::uint32_t rs_value,
    std::uint32_t rt_value,
    bool expect_taken,
    std::uint16_t fallthrough_marker) {
  constexpr std::size_t kRsIndex = 4;
  constexpr std::size_t kRtIndex = 5;
  constexpr std::size_t kMarkerIndex = 6;

  const std::uint32_t kTrapAddress = base_address;
  const std::uint32_t kAfterTrapAddress = base_address + 4u;
  const std::uint32_t kSentinelAddress = base_address + 8u;

  const std::uint32_t kAfterTrapInstruction = encode_ori(
      static_cast<std::uint8_t>(kMarkerIndex), 0, fallthrough_marker);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kTrapAddress);
  machine.write_cpu_gpr(kRsIndex, rs_value);
  machine.write_cpu_gpr(kRtIndex, rt_value);
  machine.write_cpu_gpr(kMarkerIndex, 0);

  machine.write_rdram_u32_be(kTrapAddress, trap_instruction);
  machine.write_rdram_u32_be(kAfterTrapAddress, kAfterTrapInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap trap demo: " << label << '\n';
  std::cout << "before trap step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kRsIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kRtIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

  const std::uint32_t trap_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord trap_decoded =
      Machine::decode_cpu_instruction_word(trap_raw);
  const Machine::CpuInstructionIdentity trap_identity =
      Machine::identify_cpu_instruction(trap_decoded);

  std::cout << "  trap_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << trap_raw
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  trap_identity = "
            << Machine::cpu_instruction_identity_name(trap_identity) << '\n';

  if (trap_identity != expected_identity) {
    throw std::runtime_error(
        std::string("trap demo identified the wrong instruction: ") + label);
  }

  if (expect_taken) {
    require_stopped(machine.step_cpu_instruction(), std::string(label) + "_trap_stop");

    std::cout << "after stop:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

    if (machine.cpu_pc() != kAfterTrapAddress) {
      throw std::runtime_error(std::string("trap demo stop advanced to the wrong pc: ") + label);
    }

    if (machine.cpu_next_pc() != kSentinelAddress) {
      throw std::runtime_error(std::string("trap demo stop advanced to the wrong next_pc: ") + label);
    }

    if (machine.read_cpu_gpr(kMarkerIndex) != 0) {
      throw std::runtime_error(std::string("trap demo stop unexpectedly executed fallthrough: ") + label);
    }

    return;
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_trap_fallthrough");

  std::cout << "after trap step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

  if (machine.cpu_pc() != kAfterTrapAddress) {
    throw std::runtime_error(std::string("trap demo did not fall through to the next instruction: ") + label);
  }

  if (machine.cpu_next_pc() != kSentinelAddress) {
    throw std::runtime_error(std::string("trap demo did not preserve sequential next_pc on fallthrough: ") + label);
  }

  if (machine.read_cpu_gpr(kMarkerIndex) != 0) {
    throw std::runtime_error(std::string("trap demo unexpectedly touched the fallthrough marker early: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_post_trap");

  std::cout << "after fallthrough step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error(std::string("trap demo did not execute the fallthrough instruction: ") + label);
  }

  if (machine.read_cpu_gpr(kMarkerIndex) != static_cast<std::uint32_t>(fallthrough_marker)) {
    throw std::runtime_error(std::string("trap demo wrote the wrong fallthrough marker: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_break_stop");
}

void run_special_register_trap_demos(Machine& machine) {
  run_special_register_trap_demo(
      machine,
      "special_tge not taken signed compare",
      0x00000140u,
      encode_special_register_trap(4, 5, 0x30),
      Machine::CpuInstructionIdentity::kSpecialTge,
      0xffffffffu,
      0x00000001u,
      false,
      0x7001u);

  run_special_register_trap_demo(
      machine,
      "special_tgeu taken unsigned compare",
      0x00000160u,
      encode_special_register_trap(4, 5, 0x31),
      Machine::CpuInstructionIdentity::kSpecialTgeu,
      0xffffffffu,
      0x00000001u,
      true,
      0x7002u);

  run_special_register_trap_demo(
      machine,
      "special_tlt taken signed compare",
      0x00000180u,
      encode_special_register_trap(4, 5, 0x32),
      Machine::CpuInstructionIdentity::kSpecialTlt,
      0xffffffffu,
      0x00000001u,
      true,
      0x7003u);

  run_special_register_trap_demo(
      machine,
      "special_tltu not taken unsigned compare",
      0x000001a0u,
      encode_special_register_trap(4, 5, 0x33),
      Machine::CpuInstructionIdentity::kSpecialTltu,
      0xffffffffu,
      0x00000001u,
      false,
      0x7004u);

  run_special_register_trap_demo(
      machine,
      "special_teq taken equality compare",
      0x000001c0u,
      encode_special_register_trap(4, 5, 0x34),
      Machine::CpuInstructionIdentity::kSpecialTeq,
      0x12345678u,
      0x12345678u,
      true,
      0x7005u);

  run_special_register_trap_demo(
      machine,
      "special_tne not taken inequality compare",
      0x000001e0u,
      encode_special_register_trap(4, 5, 0x36),
      Machine::CpuInstructionIdentity::kSpecialTne,
      0x12345678u,
      0x12345678u,
      false,
      0x7006u);
}

void run_regimm_immediate_trap_demo(
    Machine& machine,
    const char* label,
    std::uint32_t base_address,
    std::uint32_t trap_instruction,
    Machine::CpuInstructionIdentity expected_identity,
    std::uint32_t rs_value,
    bool expect_taken,
    std::uint16_t fallthrough_marker) {
  constexpr std::size_t kRsIndex = 4;
  constexpr std::size_t kMarkerIndex = 6;

  const std::uint32_t kTrapAddress = base_address;
  const std::uint32_t kAfterTrapAddress = base_address + 4u;
  const std::uint32_t kSentinelAddress = base_address + 8u;

  const std::uint32_t kAfterTrapInstruction = encode_ori(
      static_cast<std::uint8_t>(kMarkerIndex), 0, fallthrough_marker);
  const std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kTrapAddress);
  machine.write_cpu_gpr(kRsIndex, rs_value);
  machine.write_cpu_gpr(kMarkerIndex, 0);

  machine.write_rdram_u32_be(kTrapAddress, trap_instruction);
  machine.write_rdram_u32_be(kAfterTrapAddress, kAfterTrapInstruction);
  machine.write_rdram_u32_be(kSentinelAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap regimm immediate trap demo: " << label << '\n';
  std::cout << "before trap step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kRsIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

  const std::uint32_t trap_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord trap_decoded =
      Machine::decode_cpu_instruction_word(trap_raw);
  const Machine::CpuInstructionIdentity trap_identity =
      Machine::identify_cpu_instruction(trap_decoded);

  std::cout << "  trap_raw = 0x"
            << std::hex << std::setw(8) << std::setfill('0') << trap_raw
            << std::dec << std::setfill(' ') << '\n';
  std::cout << "  trap_identity = "
            << Machine::cpu_instruction_identity_name(trap_identity) << '\n';

  if (trap_identity != expected_identity) {
    throw std::runtime_error(
        std::string("regimm immediate trap demo identified the wrong instruction: ") + label);
  }

  if (expect_taken) {
    require_stopped(machine.step_cpu_instruction(), std::string(label) + "_trap_stop");

    std::cout << "after stop:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

    if (machine.cpu_pc() != kAfterTrapAddress) {
      throw std::runtime_error(
          std::string("regimm immediate trap demo stop advanced to the wrong pc: ") + label);
    }

    if (machine.cpu_next_pc() != kSentinelAddress) {
      throw std::runtime_error(
          std::string("regimm immediate trap demo stop advanced to the wrong next_pc: ") + label);
    }

    if (machine.read_cpu_gpr(kMarkerIndex) != 0) {
      throw std::runtime_error(
          std::string("regimm immediate trap demo stop unexpectedly executed fallthrough: ") + label);
    }

    return;
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_trap_fallthrough");

  std::cout << "after trap step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

  if (machine.cpu_pc() != kAfterTrapAddress) {
    throw std::runtime_error(
        std::string("regimm immediate trap demo did not fall through to the next instruction: ") + label);
  }

  if (machine.cpu_next_pc() != kSentinelAddress) {
    throw std::runtime_error(
        std::string("regimm immediate trap demo did not preserve sequential next_pc on fallthrough: ") + label);
  }

  if (machine.read_cpu_gpr(kMarkerIndex) != 0) {
    throw std::runtime_error(
        std::string("regimm immediate trap demo unexpectedly touched the fallthrough marker early: ") + label);
  }

  require_stepped(machine.step_cpu_instruction(), std::string(label) + "_post_trap");

  std::cout << "after fallthrough step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kMarkerIndex));

  if (machine.cpu_pc() != kSentinelAddress) {
    throw std::runtime_error(
        std::string("regimm immediate trap demo did not execute the fallthrough instruction: ") + label);
  }

  if (machine.read_cpu_gpr(kMarkerIndex) != static_cast<std::uint32_t>(fallthrough_marker)) {
    throw std::runtime_error(
        std::string("regimm immediate trap demo wrote the wrong fallthrough marker: ") + label);
  }

  require_stopped(machine.step_cpu_instruction(), std::string(label) + "_break_stop");
}

void run_regimm_immediate_trap_demos(Machine& machine) {
  run_regimm_immediate_trap_demo(
      machine,
      "regimm_tgei not taken signed compare",
      0x00000200u,
      encode_regimm_immediate_trap(4, 0x08, 0x0001u),
      Machine::CpuInstructionIdentity::kRegimmTgei,
      0xffffffffu,
      false,
      0x7101u);

  run_regimm_immediate_trap_demo(
      machine,
      "regimm_tgeiu taken unsigned compare with sign-extended immediate",
      0x00000220u,
      encode_regimm_immediate_trap(4, 0x09, 0xffffu),
      Machine::CpuInstructionIdentity::kRegimmTgeiu,
      0xffffffffu,
      true,
      0x7102u);

  run_regimm_immediate_trap_demo(
      machine,
      "regimm_tlti taken signed compare",
      0x00000240u,
      encode_regimm_immediate_trap(4, 0x0a, 0x0001u),
      Machine::CpuInstructionIdentity::kRegimmTlti,
      0xffffffffu,
      true,
      0x7103u);

  run_regimm_immediate_trap_demo(
      machine,
      "regimm_tltiu not taken unsigned compare with sign-extended immediate",
      0x00000260u,
      encode_regimm_immediate_trap(4, 0x0b, 0xffffu),
      Machine::CpuInstructionIdentity::kRegimmTltiu,
      0xffffffffu,
      false,
      0x7104u);

  run_regimm_immediate_trap_demo(
      machine,
      "regimm_teqi taken equality compare",
      0x00000280u,
      encode_regimm_immediate_trap(4, 0x0c, 0xffffu),
      Machine::CpuInstructionIdentity::kRegimmTeqi,
      0xffffffffu,
      true,
      0x7105u);

  run_regimm_immediate_trap_demo(
      machine,
      "regimm_tnei not taken inequality compare",
      0x000002a0u,
      encode_regimm_immediate_trap(4, 0x0e, 0xffffu),
      Machine::CpuInstructionIdentity::kRegimmTnei,
      0xffffffffu,
      false,
      0x7106u);
}

}  // namespace

void run_trap_demos(Machine& machine) {
  run_break_stop_demo(machine);
  run_sync_noop_demo(machine);
  run_syscall_stop_demo(machine);
  run_special_register_trap_demos(machine);
  run_regimm_immediate_trap_demos(machine);
}

}  // namespace fn64::bootstrap_detail