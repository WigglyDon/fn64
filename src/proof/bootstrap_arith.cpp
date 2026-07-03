#include "bootstrap_common.hpp"

#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

void require_signed_overflow_fault(
    const MachineFault& fault,
    const char* label,
    const char* expected_operation) {
  if (fault.kind() != MachineFaultKind::kSignedArithmeticOverflow) {
    throw std::runtime_error(std::string(label) + " threw unexpected MachineFault kind");
  }

  if (fault.operation() != expected_operation) {
    throw std::runtime_error(std::string(label) + " threw unexpected MachineFault operation");
  }

  if (fault.access_size() != 0) {
    throw std::runtime_error(std::string(label) + " threw unexpected MachineFault access size");
  }
}

constexpr std::uint8_t kCop0StatusRegisterIndex = 12;
constexpr std::uint8_t kCop0StatusSourceIndex = 31;
constexpr std::uint32_t kCop0StatusExl = 0x00000002u;

constexpr CpuInstructionWord encode_cop0_transfer(
    std::uint8_t rs,
    std::uint8_t rt,
    std::uint8_t rd) {
  return (0x10u << 26) |
         (static_cast<CpuInstructionWord>(rs & 0x1fu) << 21) |
         (static_cast<CpuInstructionWord>(rt & 0x1fu) << 16) |
         (static_cast<CpuInstructionWord>(rd & 0x1fu) << 11);
}

constexpr CpuInstructionWord encode_mtc0(std::uint8_t rt, std::uint8_t rd) {
  return encode_cop0_transfer(0x04, rt, rd);
}

void stage_local_exl_for_overflow_fault_demo(
    Machine& machine,
    RdramOffset instruction_address,
    const char* label) {
  machine.stage_cpu_gpr(kCop0StatusSourceIndex, kCop0StatusExl);
  machine.stage_rdram_u32_be(
      instruction_address,
      encode_mtc0(kCop0StatusSourceIndex, kCop0StatusRegisterIndex));
  machine.stage_cpu_pc(cpu_rdram_alias(instruction_address));
  require_stepped(
      machine.step_cpu_instruction(),
      std::string(label) + "_set_exl_for_local_fault");
}

void run_register_immediate_arithmetic_compare_demo(Machine& machine) {
  constexpr std::size_t kAddiSourceIndex = 4;
  constexpr std::size_t kAddiResultIndex = 5;
  constexpr std::size_t kAddiuSourceIndex = 6;
  constexpr std::size_t kAddiuResultIndex = 7;
  constexpr std::size_t kSltiSourceIndex = 8;
  constexpr std::size_t kSltiResultIndex = 9;
  constexpr std::size_t kSltiuSourceIndex = 10;
  constexpr std::size_t kSltiuResultIndex = 11;

  constexpr std::uint32_t kAddiAddress = 0x00000440u;
  constexpr std::uint32_t kAddiuAddress = 0x00000444u;
  constexpr std::uint32_t kSltiAddress = 0x00000448u;
  constexpr std::uint32_t kSltiuAddress = 0x0000044cu;
  constexpr std::uint32_t kBreakAddress = 0x00000450u;
  constexpr std::uint32_t kAfterBreakAddress = 0x00000454u;

  constexpr std::uint32_t kAddiInstruction = encode_addi(
      static_cast<std::uint8_t>(kAddiResultIndex),
      static_cast<std::uint8_t>(kAddiSourceIndex),
      0xffffu);
  constexpr std::uint32_t kAddiuInstruction = encode_addiu(
      static_cast<std::uint8_t>(kAddiuResultIndex),
      static_cast<std::uint8_t>(kAddiuSourceIndex),
      0xfffeu);
  constexpr std::uint32_t kSltiInstruction = encode_slti(
      static_cast<std::uint8_t>(kSltiResultIndex),
      static_cast<std::uint8_t>(kSltiSourceIndex),
      0xffffu);
  constexpr std::uint32_t kSltiuInstruction = encode_sltiu(
      static_cast<std::uint8_t>(kSltiuResultIndex),
      static_cast<std::uint8_t>(kSltiuSourceIndex),
      0xffffu);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kAddiAddress));
  machine.stage_cpu_gpr(kAddiSourceIndex, 0x00000001u);
  machine.stage_cpu_gpr(kAddiResultIndex, 0u);
  machine.stage_cpu_gpr(kAddiuSourceIndex, 0x00000010u);
  machine.stage_cpu_gpr(kAddiuResultIndex, 0u);
  machine.stage_cpu_gpr(kSltiSourceIndex, 0xfffffffffffffffeull);
  machine.stage_cpu_gpr(kSltiResultIndex, 0u);
  machine.stage_cpu_gpr(kSltiuSourceIndex, 0x0000000100000000ull);
  machine.stage_cpu_gpr(kSltiuResultIndex, 0u);

  machine.stage_rdram_u32_be(kAddiAddress, kAddiInstruction);
  machine.stage_rdram_u32_be(kAddiuAddress, kAddiuInstruction);
  machine.stage_rdram_u32_be(kSltiAddress, kSltiInstruction);
  machine.stage_rdram_u32_be(kSltiuAddress, kSltiuInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout
      << "fn64 bootstrap reg-immediate arithmetic/compare demo: explicit negative-immediate "
         "ADDI/ADDIU/SLTI/SLTIU proof\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kAddiSourceIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kAddiResultIndex));
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kAddiuSourceIndex));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kAddiuResultIndex));
  print_hex64("  gpr[8]", machine.inspect_cpu_gpr(kSltiSourceIndex));
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kSltiResultIndex));
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kSltiuSourceIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kSltiuResultIndex));

  const std::uint32_t addi_raw = kAddiInstruction;

  print_hex32("  addi_raw", addi_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_addi");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kAddiResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kAddiuAddress)) {
    throw std::runtime_error("reg-immediate demo did not advance from ADDI to ADDIU");
  }

  if (machine.inspect_cpu_gpr(kAddiResultIndex) != 0u) {
    throw std::runtime_error("reg-immediate demo ADDI negative immediate result was wrong");
  }

  const std::uint32_t addiu_raw = kAddiuInstruction;

  print_hex32("  addiu_raw", addiu_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_addiu");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kAddiuResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kSltiAddress)) {
    throw std::runtime_error("reg-immediate demo did not advance from ADDIU to SLTI");
  }

  if (machine.inspect_cpu_gpr(kAddiuResultIndex) != 0x0000000eu) {
    throw std::runtime_error("reg-immediate demo ADDIU negative immediate result was wrong");
  }

  const std::uint32_t slti_raw = kSltiInstruction;

  print_hex32("  slti_raw", slti_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_slti");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kSltiResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kSltiuAddress)) {
    throw std::runtime_error("reg-immediate demo did not advance from SLTI to SLTIU");
  }

  if (machine.inspect_cpu_gpr(kSltiResultIndex) != 1u) {
    throw std::runtime_error("reg-immediate demo SLTI full-width negative immediate compare result was wrong");
  }

  const std::uint32_t sltiu_raw = kSltiuInstruction;

  print_hex32("  sltiu_raw", sltiu_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_sltiu");

  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kSltiuResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("reg-immediate demo did not advance from SLTIU to BREAK");
  }

  if (machine.inspect_cpu_gpr(kSltiuResultIndex) != 1u) {
    throw std::runtime_error("reg-immediate demo SLTIU full-width sign-extended immediate result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "reg_immediate_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);

  if (machine.cpu_pc() != cpu_rdram_alias(kAfterBreakAddress)) {
    throw std::runtime_error("reg-immediate demo did not advance past executed BREAK");
  }
}

void run_add_positive_overflow_demo(Machine& machine) {
  constexpr std::size_t kLhsIndex = 4;
  constexpr std::size_t kRhsIndex = 5;
  constexpr std::size_t kResultIndex = 6;

  constexpr std::uint32_t kAddAddress = 0x00000420u;
  constexpr std::uint32_t kAfterAddAddress = 0x00000424u;
  constexpr std::uint32_t kExlSetupAddress = 0x00000b20u;
  constexpr std::uint32_t kAddInstruction = encode_special(
      static_cast<std::uint8_t>(kLhsIndex),
      static_cast<std::uint8_t>(kRhsIndex),
      static_cast<std::uint8_t>(kResultIndex),
      0,
      0x20);

  machine.stage_cpu_pc(cpu_rdram_alias(kAddAddress));
  machine.stage_cpu_gpr(kLhsIndex, 0x7fffffffu);
  machine.stage_cpu_gpr(kRhsIndex, 0x00000001u);
  machine.stage_cpu_gpr(kResultIndex, 0x2468ace0u);

  machine.stage_rdram_u32_be(kAddAddress, kAddInstruction);
  stage_local_exl_for_overflow_fault_demo(
      machine,
      kExlSetupAddress,
      "add_positive_overflow_demo");
  machine.stage_cpu_pc(cpu_rdram_alias(kAddAddress));

  std::cout << "fn64 bootstrap register arithmetic demo: ADD positive overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kLhsIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kRhsIndex));
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kResultIndex));

  const std::uint32_t add_raw = kAddInstruction;

  print_hex32("  add_raw", add_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "  exception = " << fault.what() << '\n';
    require_signed_overflow_fault(fault, "add positive overflow demo", "ADD");
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != cpu_rdram_alias(kAddAddress)) {
      throw std::runtime_error("add positive overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != cpu_rdram_alias(kAfterAddAddress)) {
      throw std::runtime_error("add positive overflow demo did not restore next_pc after throw");
    }

    if (machine.inspect_cpu_gpr(kResultIndex) != 0x2468ace0u) {
      throw std::runtime_error(
          "add positive overflow demo unexpectedly changed the destination register");
    }

    return;
  }

  throw std::runtime_error("add positive overflow demo did not fail loudly");
}

void run_sub_negative_overflow_demo(Machine& machine) {
  constexpr std::size_t kLhsIndex = 4;
  constexpr std::size_t kRhsIndex = 5;
  constexpr std::size_t kResultIndex = 6;

  constexpr std::uint32_t kSubAddress = 0x00000430u;
  constexpr std::uint32_t kAfterSubAddress = 0x00000434u;
  constexpr std::uint32_t kExlSetupAddress = 0x00000b30u;
  constexpr std::uint32_t kSubInstruction = encode_special(
      static_cast<std::uint8_t>(kLhsIndex),
      static_cast<std::uint8_t>(kRhsIndex),
      static_cast<std::uint8_t>(kResultIndex),
      0,
      0x22);

  machine.stage_cpu_pc(cpu_rdram_alias(kSubAddress));
  machine.stage_cpu_gpr(kLhsIndex, 0x80000000u);
  machine.stage_cpu_gpr(kRhsIndex, 0x00000001u);
  machine.stage_cpu_gpr(kResultIndex, 0x13579bdfu);

  machine.stage_rdram_u32_be(kSubAddress, kSubInstruction);
  stage_local_exl_for_overflow_fault_demo(
      machine,
      kExlSetupAddress,
      "sub_negative_overflow_demo");
  machine.stage_cpu_pc(cpu_rdram_alias(kSubAddress));

  std::cout << "fn64 bootstrap register arithmetic demo: SUB negative overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kLhsIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kRhsIndex));
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kResultIndex));

  const std::uint32_t sub_raw = kSubInstruction;

  print_hex32("  sub_raw", sub_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "  exception = " << fault.what() << '\n';
    require_signed_overflow_fault(fault, "sub negative overflow demo", "SUB");
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != cpu_rdram_alias(kSubAddress)) {
      throw std::runtime_error("sub negative overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != cpu_rdram_alias(kAfterSubAddress)) {
      throw std::runtime_error("sub negative overflow demo did not restore next_pc after throw");
    }

    if (machine.inspect_cpu_gpr(kResultIndex) != 0x13579bdfu) {
      throw std::runtime_error(
          "sub negative overflow demo unexpectedly changed the destination register");
    }

    return;
  }

  throw std::runtime_error("sub negative overflow demo did not fail loudly");
}

void run_addi_positive_overflow_demo(Machine& machine) {
  constexpr std::size_t kSourceIndex = 4;
  constexpr std::size_t kResultIndex = 5;

  constexpr std::uint32_t kAddiAddress = 0x00000460u;
  constexpr std::uint32_t kAfterAddiAddress = 0x00000464u;
  constexpr std::uint32_t kExlSetupAddress = 0x00000b60u;
  constexpr std::uint32_t kAddiInstruction = encode_addi(
      static_cast<std::uint8_t>(kResultIndex),
      static_cast<std::uint8_t>(kSourceIndex),
      0x0001u);

  machine.stage_cpu_pc(cpu_rdram_alias(kAddiAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0x7fffffffu);
  machine.stage_cpu_gpr(kResultIndex, 0x2468ace0u);

  machine.stage_rdram_u32_be(kAddiAddress, kAddiInstruction);
  stage_local_exl_for_overflow_fault_demo(
      machine,
      kExlSetupAddress,
      "addi_positive_overflow_demo");
  machine.stage_cpu_pc(cpu_rdram_alias(kAddiAddress));

  std::cout << "fn64 bootstrap reg-immediate arithmetic demo: ADDI positive overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kResultIndex));

  const std::uint32_t addi_raw = kAddiInstruction;

  print_hex32("  addi_raw", addi_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "  exception = " << fault.what() << '\n';
    require_signed_overflow_fault(fault, "addi positive overflow demo", "ADDI");
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != cpu_rdram_alias(kAddiAddress)) {
      throw std::runtime_error("addi positive overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != cpu_rdram_alias(kAfterAddiAddress)) {
      throw std::runtime_error("addi positive overflow demo did not restore next_pc after throw");
    }

    if (machine.inspect_cpu_gpr(kResultIndex) != 0x2468ace0u) {
      throw std::runtime_error(
          "addi positive overflow demo unexpectedly changed the destination register");
    }

    return;
  }

  throw std::runtime_error("addi positive overflow demo did not fail loudly");
}

void run_addi_negative_overflow_demo(Machine& machine) {
  constexpr std::size_t kSourceIndex = 4;
  constexpr std::size_t kResultIndex = 5;

  constexpr std::uint32_t kAddiAddress = 0x00000470u;
  constexpr std::uint32_t kAfterAddiAddress = 0x00000474u;
  constexpr std::uint32_t kExlSetupAddress = 0x00000b70u;
  constexpr std::uint32_t kAddiInstruction = encode_addi(
      static_cast<std::uint8_t>(kResultIndex),
      static_cast<std::uint8_t>(kSourceIndex),
      0xffffu);

  machine.stage_cpu_pc(cpu_rdram_alias(kAddiAddress));
  machine.stage_cpu_gpr(kSourceIndex, 0x80000000u);
  machine.stage_cpu_gpr(kResultIndex, 0x13579bdfu);

  machine.stage_rdram_u32_be(kAddiAddress, kAddiInstruction);
  stage_local_exl_for_overflow_fault_demo(
      machine,
      kExlSetupAddress,
      "addi_negative_overflow_demo");
  machine.stage_cpu_pc(cpu_rdram_alias(kAddiAddress));

  std::cout << "fn64 bootstrap reg-immediate arithmetic demo: ADDI negative overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kResultIndex));

  const std::uint32_t addi_raw = kAddiInstruction;

  print_hex32("  addi_raw", addi_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "  exception = " << fault.what() << '\n';
    require_signed_overflow_fault(fault, "addi negative overflow demo", "ADDI");
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != cpu_rdram_alias(kAddiAddress)) {
      throw std::runtime_error("addi negative overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != cpu_rdram_alias(kAfterAddiAddress)) {
      throw std::runtime_error("addi negative overflow demo did not restore next_pc after throw");
    }

    if (machine.inspect_cpu_gpr(kResultIndex) != 0x13579bdfu) {
      throw std::runtime_error(
          "addi negative overflow demo unexpectedly changed the destination register");
    }

    return;
  }

  throw std::runtime_error("addi negative overflow demo did not fail loudly");
}

void run_logic_immediate_unsigned_compare_demo(Machine& machine) {
  constexpr std::size_t kValueIndex = 4;
  constexpr std::size_t kAndResultIndex = 5;
  constexpr std::size_t kXorResultIndex = 6;
  constexpr std::size_t kMaxIndex = 10;
  constexpr std::size_t kOneIndex = 11;
  constexpr std::size_t kCompareResultIndex = 12;

  constexpr std::uint32_t kLuiAddress = 0x00000480u;
  constexpr std::uint32_t kOriAddress = 0x00000484u;
  constexpr std::uint32_t kAndiAddress = 0x00000488u;
  constexpr std::uint32_t kXoriAddress = 0x0000048cu;
  constexpr std::uint32_t kMaxLuiAddress = 0x00000490u;
  constexpr std::uint32_t kMaxOriAddress = 0x00000494u;
  constexpr std::uint32_t kOneOriAddress = 0x00000498u;
  constexpr std::uint32_t kSltuAddress = 0x0000049cu;
  constexpr std::uint32_t kBreakAddress = 0x000004a0u;
  constexpr std::uint32_t kAfterBreakAddress = 0x000004a4u;

  constexpr std::uint32_t kLuiInstruction = encode_lui(
      static_cast<std::uint8_t>(kValueIndex), 0xabcdu);
  constexpr std::uint32_t kOriInstruction = encode_ori(
      static_cast<std::uint8_t>(kValueIndex),
      static_cast<std::uint8_t>(kValueIndex),
      0x1234u);
  constexpr std::uint32_t kAndiInstruction = encode_andi(
      static_cast<std::uint8_t>(kAndResultIndex),
      static_cast<std::uint8_t>(kValueIndex),
      0x00f0u);
  constexpr std::uint32_t kXoriInstruction = encode_xori(
      static_cast<std::uint8_t>(kXorResultIndex),
      static_cast<std::uint8_t>(kValueIndex),
      0x00ffu);
  constexpr std::uint32_t kMaxLuiInstruction = encode_lui(
      static_cast<std::uint8_t>(kMaxIndex), 0xffffu);
  constexpr std::uint32_t kMaxOriInstruction = encode_ori(
      static_cast<std::uint8_t>(kMaxIndex),
      static_cast<std::uint8_t>(kMaxIndex),
      0xffffu);
  constexpr std::uint32_t kOneOriInstruction = encode_ori(
      static_cast<std::uint8_t>(kOneIndex), 0, 0x0001u);
  constexpr std::uint32_t kSltuInstruction = encode_sltu(
      static_cast<std::uint8_t>(kCompareResultIndex),
      static_cast<std::uint8_t>(kMaxIndex),
      static_cast<std::uint8_t>(kOneIndex));
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kLuiAddress));
  machine.stage_cpu_gpr(kValueIndex, 0);
  machine.stage_cpu_gpr(kAndResultIndex, 0);
  machine.stage_cpu_gpr(kXorResultIndex, 0);
  machine.stage_cpu_gpr(kMaxIndex, 0);
  machine.stage_cpu_gpr(kOneIndex, 0);
  machine.stage_cpu_gpr(kCompareResultIndex, 0);

  machine.stage_rdram_u32_be(kLuiAddress, kLuiInstruction);
  machine.stage_rdram_u32_be(kOriAddress, kOriInstruction);
  machine.stage_rdram_u32_be(kAndiAddress, kAndiInstruction);
  machine.stage_rdram_u32_be(kXoriAddress, kXoriInstruction);
  machine.stage_rdram_u32_be(kMaxLuiAddress, kMaxLuiInstruction);
  machine.stage_rdram_u32_be(kMaxOriAddress, kMaxOriInstruction);
  machine.stage_rdram_u32_be(kOneOriAddress, kOneOriInstruction);
  machine.stage_rdram_u32_be(kSltuAddress, kSltuInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap logic/immediate demo: explicit LUI/ORI/ANDI/XORI/SLTU proof\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kValueIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kAndResultIndex));
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kXorResultIndex));
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kMaxIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kOneIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kCompareResultIndex));

  const std::uint32_t lui_raw = kLuiInstruction;

  print_hex32("  lui_raw", lui_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_lui");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kValueIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kOriAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance from LUI to ORI");
  }

  if (machine.inspect_cpu_gpr(kValueIndex) !=
      cpu_value_from_sign_extended_u32(0xabcd0000u)) {
    throw std::runtime_error("logic/immediate demo LUI result was wrong");
  }

  const std::uint32_t ori_raw = kOriInstruction;

  print_hex32("  ori_raw", ori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_ori");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kValueIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kAndiAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance from ORI to ANDI");
  }

  if (machine.inspect_cpu_gpr(kValueIndex) !=
      cpu_value_from_sign_extended_u32(0xabcd1234u)) {
    throw std::runtime_error("logic/immediate demo ORI result was wrong");
  }

  const std::uint32_t andi_raw = kAndiInstruction;

  print_hex32("  andi_raw", andi_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_andi");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kAndResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kXoriAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance from ANDI to XORI");
  }

  if (machine.inspect_cpu_gpr(kAndResultIndex) !=
      cpu_value_from_zero_extended_u32(0x00000030u)) {
    throw std::runtime_error("logic/immediate demo ANDI result was wrong");
  }

  const std::uint32_t xori_raw = kXoriInstruction;

  print_hex32("  xori_raw", xori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_xori");

  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kXorResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kMaxLuiAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance from XORI to second LUI");
  }

  if (machine.inspect_cpu_gpr(kXorResultIndex) !=
      cpu_value_from_sign_extended_u32(0xabcd12cbu)) {
    throw std::runtime_error("logic/immediate demo XORI result was wrong");
  }

  const std::uint32_t max_lui_raw = kMaxLuiInstruction;

  print_hex32("  max_lui_raw", max_lui_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_max_lui");

  std::cout << "after step 5:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kMaxIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kMaxOriAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance from second LUI to second ORI");
  }

  if (machine.inspect_cpu_gpr(kMaxIndex) !=
      cpu_value_from_sign_extended_u32(0xffff0000u)) {
    throw std::runtime_error("logic/immediate demo second LUI result was wrong");
  }

  const std::uint32_t max_ori_raw = kMaxOriInstruction;

  print_hex32("  max_ori_raw", max_ori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_max_ori");

  std::cout << "after step 6:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kMaxIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kOneOriAddress)) {
    throw std::runtime_error(
        "logic/immediate demo did not advance from second ORI to one-building ORI");
  }

  if (machine.inspect_cpu_gpr(kMaxIndex) !=
      cpu_value_from_sign_extended_u32(0xffffffffu)) {
    throw std::runtime_error("logic/immediate demo second ORI result was wrong");
  }

  const std::uint32_t one_ori_raw = kOneOriInstruction;

  print_hex32("  one_ori_raw", one_ori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_one_ori");

  std::cout << "after step 7:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kOneIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kSltuAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance from one-building ORI to SLTU");
  }

  if (machine.inspect_cpu_gpr(kOneIndex) !=
      cpu_value_from_zero_extended_u32(0x00000001u)) {
    throw std::runtime_error("logic/immediate demo one-building ORI result was wrong");
  }

  const std::uint32_t sltu_raw = kSltuInstruction;

  print_hex32("  sltu_raw", sltu_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_sltu");

  std::cout << "after step 8:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kMaxIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kOneIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kCompareResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance from SLTU to BREAK");
  }

  if (machine.inspect_cpu_gpr(kCompareResultIndex) != 0) {
    throw std::runtime_error("logic/immediate demo SLTU did not prove unsigned compare semantics");
  }

  require_stopped(machine.step_cpu_instruction(), "logic_immediate_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);

  if (machine.cpu_pc() != cpu_rdram_alias(kAfterBreakAddress)) {
    throw std::runtime_error("logic/immediate demo did not advance past executed BREAK");
  }
}

void run_full_width_register_compare_demo(Machine& machine) {
  constexpr std::size_t kSignedLhsIndex = 4;
  constexpr std::size_t kSignedRhsIndex = 5;
  constexpr std::size_t kSignedResultIndex = 6;
  constexpr std::size_t kUnsignedLhsIndex = 7;
  constexpr std::size_t kUnsignedRhsIndex = 8;
  constexpr std::size_t kUnsignedResultIndex = 9;

  constexpr std::uint32_t kSltAddress = 0x00000720u;
  constexpr std::uint32_t kSltuAddress = 0x00000724u;
  constexpr std::uint32_t kBreakAddress = 0x00000728u;
  constexpr std::uint32_t kAfterBreakAddress = 0x0000072cu;

  constexpr CpuInstructionWord kSltInstruction = encode_special(
      static_cast<std::uint8_t>(kSignedLhsIndex),
      static_cast<std::uint8_t>(kSignedRhsIndex),
      static_cast<std::uint8_t>(kSignedResultIndex),
      0,
      0x2a);
  constexpr CpuInstructionWord kSltuInstruction = encode_sltu(
      static_cast<std::uint8_t>(kUnsignedResultIndex),
      static_cast<std::uint8_t>(kUnsignedLhsIndex),
      static_cast<std::uint8_t>(kUnsignedRhsIndex));
  constexpr CpuInstructionWord kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kSltAddress));
  machine.stage_cpu_gpr(kSignedLhsIndex, 0xffffffff00000000ull);
  machine.stage_cpu_gpr(kSignedRhsIndex, 0u);
  machine.stage_cpu_gpr(kSignedResultIndex, 0u);
  machine.stage_cpu_gpr(kUnsignedLhsIndex, 0x0000000100000000ull);
  machine.stage_cpu_gpr(kUnsignedRhsIndex, 0x00000000ffffffffull);
  machine.stage_cpu_gpr(kUnsignedResultIndex, 0u);

  machine.stage_rdram_u32_be(kSltAddress, kSltInstruction);
  machine.stage_rdram_u32_be(kSltuAddress, kSltuInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap full-width register compare demo: SLT/SLTU use 64-bit GPR values\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kSignedLhsIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kSignedRhsIndex));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kUnsignedLhsIndex));
  print_hex64("  gpr[8]", machine.inspect_cpu_gpr(kUnsignedRhsIndex));

  print_hex32("  slt_raw", kSltInstruction);
  require_stepped(machine.step_cpu_instruction(), "full_width_compare_demo_slt");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kSignedResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kSltuAddress)) {
    throw std::runtime_error("full-width compare demo did not advance from SLT to SLTU");
  }

  if (machine.inspect_cpu_gpr(kSignedResultIndex) != 1u) {
    throw std::runtime_error("full-width compare demo SLT did not use signed 64-bit values");
  }

  print_hex32("  sltu_raw", kSltuInstruction);
  require_stepped(machine.step_cpu_instruction(), "full_width_compare_demo_sltu");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kUnsignedResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("full-width compare demo did not advance from SLTU to BREAK");
  }

  if (machine.inspect_cpu_gpr(kUnsignedResultIndex) != 0u) {
    throw std::runtime_error("full-width compare demo SLTU ignored high register bits");
  }

  require_stopped(machine.step_cpu_instruction(), "full_width_compare_demo_break");

  if (machine.cpu_pc() != cpu_rdram_alias(kAfterBreakAddress)) {
    throw std::runtime_error("full-width compare demo did not advance past executed BREAK");
  }
}

void run_d_integer_add_sub_demo(Machine& machine) {
  constexpr std::uint8_t kDadduLhsIndex = 4;
  constexpr std::uint8_t kDadduRhsIndex = 5;
  constexpr std::uint8_t kDadduResultIndex = 6;
  constexpr std::uint8_t kDsubuLhsIndex = 7;
  constexpr std::uint8_t kDsubuRhsIndex = 8;
  constexpr std::uint8_t kDsubuResultIndex = 9;
  constexpr std::uint8_t kDaddiuSourceIndex = 10;
  constexpr std::uint8_t kDaddiuResultIndex = 11;
  constexpr std::uint8_t kDaddiuWrapSourceIndex = 12;
  constexpr std::uint8_t kDaddiuWrapResultIndex = 13;

  constexpr CpuAddress kDadduAddress = 0x00000730u;
  constexpr CpuAddress kDsubuAddress = 0x00000734u;
  constexpr CpuAddress kDaddiuAddress = 0x00000738u;
  constexpr CpuAddress kDaddiuWrapAddress = 0x0000073cu;
  constexpr CpuAddress kZeroDadduAddress = 0x00000740u;
  constexpr CpuAddress kBreakAddress = 0x00000744u;
  constexpr CpuAddress kAfterBreakAddress = 0x00000748u;

  constexpr CpuRegisterValue kDadduLhs = 0x0000000100000000ull;
  constexpr CpuRegisterValue kDadduRhs = 0x0000000200000003ull;
  constexpr CpuRegisterValue kDadduExpected = 0x0000000300000003ull;
  constexpr CpuRegisterValue kDsubuLhs = 0x0000000000000000ull;
  constexpr CpuRegisterValue kDsubuRhs = 0x0000000000000001ull;
  constexpr CpuRegisterValue kDsubuExpected = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kDaddiuSource = 0x0000000100000000ull;
  constexpr CpuRegisterValue kDaddiuExpected = 0x00000000ffffffffull;
  constexpr CpuRegisterValue kDaddiuWrapSource = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kDaddiuWrapExpected = 0x0000000000000000ull;

  constexpr CpuInstructionWord kDadduInstruction =
      encode_daddu(kDadduResultIndex, kDadduLhsIndex, kDadduRhsIndex);
  constexpr CpuInstructionWord kDsubuInstruction =
      encode_dsubu(kDsubuResultIndex, kDsubuLhsIndex, kDsubuRhsIndex);
  constexpr CpuInstructionWord kDaddiuInstruction =
      encode_daddiu(kDaddiuResultIndex, kDaddiuSourceIndex, 0xffffu);
  constexpr CpuInstructionWord kDaddiuWrapInstruction =
      encode_daddiu(kDaddiuWrapResultIndex, kDaddiuWrapSourceIndex, 0x0001u);
  constexpr CpuInstructionWord kZeroDadduInstruction =
      encode_daddu(0, kDadduLhsIndex, kDadduRhsIndex);
  constexpr CpuInstructionWord kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kDadduAddress));
  machine.stage_cpu_gpr(kDadduLhsIndex, kDadduLhs);
  machine.stage_cpu_gpr(kDadduRhsIndex, kDadduRhs);
  machine.stage_cpu_gpr(kDadduResultIndex, 0);
  machine.stage_cpu_gpr(kDsubuLhsIndex, kDsubuLhs);
  machine.stage_cpu_gpr(kDsubuRhsIndex, kDsubuRhs);
  machine.stage_cpu_gpr(kDsubuResultIndex, 0);
  machine.stage_cpu_gpr(kDaddiuSourceIndex, kDaddiuSource);
  machine.stage_cpu_gpr(kDaddiuResultIndex, 0);
  machine.stage_cpu_gpr(kDaddiuWrapSourceIndex, kDaddiuWrapSource);
  machine.stage_cpu_gpr(kDaddiuWrapResultIndex, 0x1111111122222222ull);
  machine.stage_cpu_gpr(0, 0xffffffffffffffffull);

  machine.stage_rdram_u32_be(kDadduAddress, kDadduInstruction);
  machine.stage_rdram_u32_be(kDsubuAddress, kDsubuInstruction);
  machine.stage_rdram_u32_be(kDaddiuAddress, kDaddiuInstruction);
  machine.stage_rdram_u32_be(kDaddiuWrapAddress, kDaddiuWrapInstruction);
  machine.stage_rdram_u32_be(kZeroDadduAddress, kZeroDadduInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout
      << "fn64 bootstrap D integer demo: DADDU/DSUBU/DADDIU execute over full 64-bit GPR values\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kDadduLhsIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kDadduRhsIndex));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kDsubuLhsIndex));
  print_hex64("  gpr[8]", machine.inspect_cpu_gpr(kDsubuRhsIndex));
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kDaddiuSourceIndex));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kDaddiuWrapSourceIndex));

  auto step_d_instruction = [&machine](CpuInstructionWord instruction, const char* label) {
    print_hex32("  instruction_raw", instruction);
    std::cout << "  instruction_label = " << label << '\n';
    require_stepped(machine.step_cpu_instruction(), std::string("d_integer_demo_") + label);
  };

  step_d_instruction(kDadduInstruction, "DADDU");

  if (machine.cpu_pc() != cpu_rdram_alias(kDsubuAddress)) {
    throw std::runtime_error("D integer demo did not advance from DADDU to DSUBU");
  }

  if (machine.inspect_cpu_gpr(kDadduResultIndex) != kDadduExpected) {
    throw std::runtime_error("D integer demo DADDU did not use full 64-bit operands");
  }

  step_d_instruction(kDsubuInstruction, "DSUBU");

  if (machine.cpu_pc() != cpu_rdram_alias(kDaddiuAddress)) {
    throw std::runtime_error("D integer demo did not advance from DSUBU to DADDIU");
  }

  if (machine.inspect_cpu_gpr(kDsubuResultIndex) != kDsubuExpected) {
    throw std::runtime_error("D integer demo DSUBU did not wrap modulo 2^64");
  }

  step_d_instruction(kDaddiuInstruction, "DADDIU negative immediate");

  if (machine.cpu_pc() != cpu_rdram_alias(kDaddiuWrapAddress)) {
    throw std::runtime_error("D integer demo did not advance to DADDIU wrap case");
  }

  if (machine.inspect_cpu_gpr(kDaddiuResultIndex) != kDaddiuExpected) {
    throw std::runtime_error("D integer demo DADDIU did not sign-extend the immediate");
  }

  step_d_instruction(kDaddiuWrapInstruction, "DADDIU wrap");

  if (machine.cpu_pc() != cpu_rdram_alias(kZeroDadduAddress)) {
    throw std::runtime_error("D integer demo did not advance to DADDU zero-register case");
  }

  if (machine.inspect_cpu_gpr(kDaddiuWrapResultIndex) != kDaddiuWrapExpected) {
    throw std::runtime_error("D integer demo DADDIU did not wrap modulo 2^64");
  }

  step_d_instruction(kZeroDadduInstruction, "DADDU $0");

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("D integer demo did not advance to BREAK");
  }

  if (machine.inspect_cpu_gpr(0) != 0) {
    throw std::runtime_error("D integer demo wrote to gpr[0]");
  }

  require_stopped(machine.step_cpu_instruction(), "d_integer_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kDadduResultIndex));
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kDsubuResultIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kDaddiuResultIndex));
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kDaddiuWrapResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kAfterBreakAddress)) {
    throw std::runtime_error("D integer demo did not advance past BREAK");
  }
}

void require_d_signed_overflow_case(
    Machine& machine,
    const char* label,
    const char* operation,
    CpuAddress instruction_address,
    CpuInstructionWord instruction,
    std::uint8_t lhs_index,
    CpuRegisterValue lhs,
    bool has_rhs,
    std::uint8_t rhs_index,
    CpuRegisterValue rhs,
    std::uint8_t destination_index,
    CpuRegisterValue destination_sentinel) {
  constexpr CpuRegisterValue kHiSentinel = 0x0102030405060708ull;
  constexpr CpuRegisterValue kLoSentinel = 0x8877665544332211ull;

  machine.stage_cpu_pc(cpu_rdram_alias(instruction_address));
  machine.stage_cpu_gpr(lhs_index, lhs);
  if (has_rhs) {
    machine.stage_cpu_gpr(rhs_index, rhs);
  }
  machine.stage_cpu_gpr(destination_index, destination_sentinel);
  machine.stage_cpu_hi(kHiSentinel);
  machine.stage_cpu_lo(kLoSentinel);
  machine.stage_rdram_u32_be(instruction_address, instruction);
  stage_local_exl_for_overflow_fault_demo(
      machine,
      instruction_address + 0x00000400u,
      label);
  machine.stage_cpu_pc(cpu_rdram_alias(instruction_address));

  std::cout << "fn64 bootstrap D signed overflow demo: " << label << '\n';
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  lhs", machine.inspect_cpu_gpr(lhs_index));
  if (has_rhs) {
    print_hex64("  rhs", machine.inspect_cpu_gpr(rhs_index));
  }
  print_hex64("  destination", machine.inspect_cpu_gpr(destination_index));
  print_hex32("  instruction_raw", instruction);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const MachineFault& fault) {
    std::cout << "  exception = " << fault.what() << '\n';
    require_signed_overflow_fault(fault, label, operation);

    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  destination", machine.inspect_cpu_gpr(destination_index));
    print_hex64("  hi", machine.inspect_cpu_hi());
    print_hex64("  lo", machine.inspect_cpu_lo());

    if (machine.cpu_pc() != cpu_rdram_alias(instruction_address)) {
      throw std::runtime_error(std::string(label) + " did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != cpu_rdram_alias(instruction_address + 4u)) {
      throw std::runtime_error(std::string(label) + " did not restore next_pc after throw");
    }

    if (machine.inspect_cpu_gpr(destination_index) != destination_sentinel) {
      throw std::runtime_error(std::string(label) + " changed destination on overflow");
    }

    if (machine.inspect_cpu_hi() != kHiSentinel ||
        machine.inspect_cpu_lo() != kLoSentinel) {
      throw std::runtime_error(std::string(label) + " changed HI/LO on overflow");
    }

    return;
  }

  throw std::runtime_error(std::string(label) + " did not throw signed overflow");
}

void run_trapping_d_integer_add_sub_demo(Machine& machine) {
  constexpr std::uint8_t kDaddLhsIndex = 4;
  constexpr std::uint8_t kDaddRhsIndex = 5;
  constexpr std::uint8_t kDaddResultIndex = 6;
  constexpr std::uint8_t kDaddiSourceIndex = 7;
  constexpr std::uint8_t kDaddiResultIndex = 8;
  constexpr std::uint8_t kDsubLhsIndex = 9;
  constexpr std::uint8_t kDsubRhsIndex = 10;
  constexpr std::uint8_t kDsubResultIndex = 11;

  constexpr CpuAddress kDaddAddress = 0x000007b0u;
  constexpr CpuAddress kDaddiAddress = 0x000007b4u;
  constexpr CpuAddress kDsubAddress = 0x000007b8u;
  constexpr CpuAddress kZeroDaddAddress = 0x000007bcu;
  constexpr CpuAddress kBreakAddress = 0x000007c0u;
  constexpr CpuAddress kAfterBreakAddress = 0x000007c4u;

  constexpr CpuRegisterValue kDaddLhs = 0x0000000100000000ull;
  constexpr CpuRegisterValue kDaddRhs = 0x0000000200000003ull;
  constexpr CpuRegisterValue kDaddExpected = 0x0000000300000003ull;
  constexpr CpuRegisterValue kDaddiSource = 0x0000000100000000ull;
  constexpr CpuRegisterValue kDaddiExpected = 0x00000000ffffffffull;
  constexpr CpuRegisterValue kDsubLhs = 0x0000000300000003ull;
  constexpr CpuRegisterValue kDsubRhs = 0x0000000100000000ull;
  constexpr CpuRegisterValue kDsubExpected = 0x0000000200000003ull;

  const CpuInstructionWord kDaddInstruction =
      encode_dadd(kDaddResultIndex, kDaddLhsIndex, kDaddRhsIndex);
  const CpuInstructionWord kDaddiInstruction =
      encode_daddi(kDaddiResultIndex, kDaddiSourceIndex, 0xffffu);
  const CpuInstructionWord kDsubInstruction =
      encode_dsub(kDsubResultIndex, kDsubLhsIndex, kDsubRhsIndex);
  const CpuInstructionWord kZeroDaddInstruction =
      encode_dadd(0, kDaddLhsIndex, kDaddRhsIndex);
  const CpuInstructionWord kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kDaddAddress));
  machine.stage_cpu_gpr(kDaddLhsIndex, kDaddLhs);
  machine.stage_cpu_gpr(kDaddRhsIndex, kDaddRhs);
  machine.stage_cpu_gpr(kDaddResultIndex, 0);
  machine.stage_cpu_gpr(kDaddiSourceIndex, kDaddiSource);
  machine.stage_cpu_gpr(kDaddiResultIndex, 0);
  machine.stage_cpu_gpr(kDsubLhsIndex, kDsubLhs);
  machine.stage_cpu_gpr(kDsubRhsIndex, kDsubRhs);
  machine.stage_cpu_gpr(kDsubResultIndex, 0);
  machine.stage_cpu_gpr(0, 0xffffffffffffffffull);

  machine.stage_rdram_u32_be(kDaddAddress, kDaddInstruction);
  machine.stage_rdram_u32_be(kDaddiAddress, kDaddiInstruction);
  machine.stage_rdram_u32_be(kDsubAddress, kDsubInstruction);
  machine.stage_rdram_u32_be(kZeroDaddAddress, kZeroDaddInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout
      << "fn64 bootstrap trapping D integer demo: DADD/DADDI/DSUB execute with local signed-overflow faults\n";
  std::cout << "before step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kDaddLhsIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kDaddRhsIndex));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kDaddiSourceIndex));
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kDsubLhsIndex));
  print_hex64("  gpr[10]", machine.inspect_cpu_gpr(kDsubRhsIndex));

  auto step_d_instruction = [&machine](CpuInstructionWord instruction, const char* label) {
    print_hex32("  instruction_raw", instruction);
    std::cout << "  instruction_label = " << label << '\n';
    require_stepped(machine.step_cpu_instruction(), std::string("trapping_d_demo_") + label);
  };

  step_d_instruction(kDaddInstruction, "DADD");

  if (machine.cpu_pc() != cpu_rdram_alias(kDaddiAddress)) {
    throw std::runtime_error("trapping D demo did not advance from DADD to DADDI");
  }

  if (machine.inspect_cpu_gpr(kDaddResultIndex) != kDaddExpected) {
    throw std::runtime_error("trapping D demo DADD did not use full 64-bit operands");
  }

  step_d_instruction(kDaddiInstruction, "DADDI negative immediate");

  if (machine.cpu_pc() != cpu_rdram_alias(kDsubAddress)) {
    throw std::runtime_error("trapping D demo did not advance from DADDI to DSUB");
  }

  if (machine.inspect_cpu_gpr(kDaddiResultIndex) != kDaddiExpected) {
    throw std::runtime_error("trapping D demo DADDI did not sign-extend the immediate");
  }

  step_d_instruction(kDsubInstruction, "DSUB");

  if (machine.cpu_pc() != cpu_rdram_alias(kZeroDaddAddress)) {
    throw std::runtime_error("trapping D demo did not advance from DSUB to DADD $0");
  }

  if (machine.inspect_cpu_gpr(kDsubResultIndex) != kDsubExpected) {
    throw std::runtime_error("trapping D demo DSUB did not use full 64-bit operands");
  }

  step_d_instruction(kZeroDaddInstruction, "DADD $0");

  if (machine.cpu_pc() != cpu_rdram_alias(kBreakAddress)) {
    throw std::runtime_error("trapping D demo did not advance to BREAK");
  }

  if (machine.inspect_cpu_gpr(0) != 0) {
    throw std::runtime_error("trapping D demo wrote to gpr[0]");
  }

  require_stopped(machine.step_cpu_instruction(), "trapping_d_demo_break");

  std::cout << "after step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kDaddResultIndex));
  print_hex64("  gpr[8]", machine.inspect_cpu_gpr(kDaddiResultIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kDsubResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kAfterBreakAddress)) {
    throw std::runtime_error("trapping D demo did not advance past BREAK");
  }

  require_d_signed_overflow_case(
      machine,
      "DADD overflow",
      "DADD",
      0x000007d0u,
      encode_dadd(14, 12, 13),
      12,
      0x7fffffffffffffffull,
      true,
      13,
      0x0000000000000001ull,
      14,
      0x0123456789abcdefull);

  require_d_signed_overflow_case(
      machine,
      "DADDI overflow",
      "DADDI",
      0x000007d4u,
      encode_daddi(16, 15, 0x0001u),
      15,
      0x7fffffffffffffffull,
      false,
      0,
      0,
      16,
      0xfedcba9876543210ull);

  require_d_signed_overflow_case(
      machine,
      "DSUB overflow",
      "DSUB",
      0x000007d8u,
      encode_dsub(19, 17, 18),
      17,
      0x8000000000000000ull,
      true,
      18,
      0x0000000000000001ull,
      19,
      0x0badf00dc001d00dull);
}

void run_d_shift_demo(Machine& machine) {
  constexpr std::uint8_t kDsllSourceIndex = 4;
  constexpr std::uint8_t kDsllResultIndex = 5;
  constexpr std::uint8_t kDsrlSourceIndex = 6;
  constexpr std::uint8_t kDsrlResultIndex = 7;
  constexpr std::uint8_t kDsraSourceIndex = 8;
  constexpr std::uint8_t kDsraResultIndex = 9;
  constexpr std::uint8_t kDsll32SourceIndex = 10;
  constexpr std::uint8_t kDsll32ResultIndex = 11;
  constexpr std::uint8_t kDsrl32SourceIndex = 12;
  constexpr std::uint8_t kDsrl32ResultIndex = 13;
  constexpr std::uint8_t kDsra32SourceIndex = 14;
  constexpr std::uint8_t kDsra32ResultIndex = 15;
  constexpr std::uint8_t kVariableShiftIndex = 16;
  constexpr std::uint8_t kDsllvSourceIndex = 17;
  constexpr std::uint8_t kDsllvResultIndex = 18;
  constexpr std::uint8_t kDsrlvSourceIndex = 19;
  constexpr std::uint8_t kDsrlvResultIndex = 20;
  constexpr std::uint8_t kDsravSourceIndex = 21;
  constexpr std::uint8_t kDsravResultIndex = 22;
  constexpr std::uint8_t kZeroSourceIndex = 23;

  constexpr CpuAddress kDsllAddress = 0x00000750u;
  constexpr CpuAddress kDsrlAddress = 0x00000754u;
  constexpr CpuAddress kDsraAddress = 0x00000758u;
  constexpr CpuAddress kDsll32Address = 0x0000075cu;
  constexpr CpuAddress kDsrl32Address = 0x00000760u;
  constexpr CpuAddress kDsra32Address = 0x00000764u;
  constexpr CpuAddress kDsllvAddress = 0x00000768u;
  constexpr CpuAddress kDsrlvAddress = 0x0000076cu;
  constexpr CpuAddress kDsravAddress = 0x00000770u;
  constexpr CpuAddress kZeroDsllAddress = 0x00000774u;
  constexpr CpuAddress kBreakAddress = 0x00000778u;
  constexpr CpuAddress kAfterBreakAddress = 0x0000077cu;

  constexpr CpuRegisterValue kDsllSource = 0x0000000100000001ull;
  constexpr CpuRegisterValue kDsllExpected = 0x0000001000000010ull;
  constexpr CpuRegisterValue kDsrlSource = 0x8000000000000000ull;
  constexpr CpuRegisterValue kDsrlExpected = 0x0800000000000000ull;
  constexpr CpuRegisterValue kDsraSource = 0x8000000000000000ull;
  constexpr CpuRegisterValue kDsraExpected = 0xf800000000000000ull;
  constexpr CpuRegisterValue kDsll32Source = 0x0000000000000001ull;
  constexpr CpuRegisterValue kDsll32Expected = 0x0000000100000000ull;
  constexpr CpuRegisterValue kDsrl32Source = 0x8000000000000000ull;
  constexpr CpuRegisterValue kDsrl32Expected = 0x0000000080000000ull;
  constexpr CpuRegisterValue kDsra32Source = 0x8000000000000000ull;
  constexpr CpuRegisterValue kDsra32Expected = 0xffffffff80000000ull;
  constexpr CpuRegisterValue kVariableShiftValue = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kDsllvSource = 0x0000000000000001ull;
  constexpr CpuRegisterValue kDsllvExpected = 0x8000000000000000ull;
  constexpr CpuRegisterValue kDsrlvSource = 0x8000000000000000ull;
  constexpr CpuRegisterValue kDsrlvExpected = 0x0000000000000001ull;
  constexpr CpuRegisterValue kDsravSource = 0x8000000000000000ull;
  constexpr CpuRegisterValue kDsravExpected = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kZeroAttempt = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kZeroSource = 0x1111111122222222ull;

  constexpr CpuInstructionWord kDsllInstruction =
      encode_dsll(kDsllResultIndex, kDsllSourceIndex, 4);
  constexpr CpuInstructionWord kDsrlInstruction =
      encode_dsrl(kDsrlResultIndex, kDsrlSourceIndex, 4);
  constexpr CpuInstructionWord kDsraInstruction =
      encode_dsra(kDsraResultIndex, kDsraSourceIndex, 4);
  constexpr CpuInstructionWord kDsll32Instruction =
      encode_dsll32(kDsll32ResultIndex, kDsll32SourceIndex, 0);
  constexpr CpuInstructionWord kDsrl32Instruction =
      encode_dsrl32(kDsrl32ResultIndex, kDsrl32SourceIndex, 0);
  constexpr CpuInstructionWord kDsra32Instruction =
      encode_dsra32(kDsra32ResultIndex, kDsra32SourceIndex, 0);
  constexpr CpuInstructionWord kDsllvInstruction =
      encode_dsllv(kDsllvResultIndex, kDsllvSourceIndex, kVariableShiftIndex);
  constexpr CpuInstructionWord kDsrlvInstruction =
      encode_dsrlv(kDsrlvResultIndex, kDsrlvSourceIndex, kVariableShiftIndex);
  constexpr CpuInstructionWord kDsravInstruction =
      encode_dsrav(kDsravResultIndex, kDsravSourceIndex, kVariableShiftIndex);
  constexpr CpuInstructionWord kZeroDsllInstruction =
      encode_dsll(0, kZeroSourceIndex, 4);
  constexpr CpuInstructionWord kBreakInstruction = encode_break();

  machine.stage_cpu_pc(cpu_rdram_alias(kDsllAddress));
  machine.stage_cpu_gpr(0, kZeroAttempt);
  machine.stage_cpu_gpr(kDsllSourceIndex, kDsllSource);
  machine.stage_cpu_gpr(kDsllResultIndex, 0);
  machine.stage_cpu_gpr(kDsrlSourceIndex, kDsrlSource);
  machine.stage_cpu_gpr(kDsrlResultIndex, 0);
  machine.stage_cpu_gpr(kDsraSourceIndex, kDsraSource);
  machine.stage_cpu_gpr(kDsraResultIndex, 0);
  machine.stage_cpu_gpr(kDsll32SourceIndex, kDsll32Source);
  machine.stage_cpu_gpr(kDsll32ResultIndex, 0);
  machine.stage_cpu_gpr(kDsrl32SourceIndex, kDsrl32Source);
  machine.stage_cpu_gpr(kDsrl32ResultIndex, 0);
  machine.stage_cpu_gpr(kDsra32SourceIndex, kDsra32Source);
  machine.stage_cpu_gpr(kDsra32ResultIndex, 0);
  machine.stage_cpu_gpr(kVariableShiftIndex, kVariableShiftValue);
  machine.stage_cpu_gpr(kDsllvSourceIndex, kDsllvSource);
  machine.stage_cpu_gpr(kDsllvResultIndex, 0);
  machine.stage_cpu_gpr(kDsrlvSourceIndex, kDsrlvSource);
  machine.stage_cpu_gpr(kDsrlvResultIndex, 0);
  machine.stage_cpu_gpr(kDsravSourceIndex, kDsravSource);
  machine.stage_cpu_gpr(kDsravResultIndex, 0);
  machine.stage_cpu_gpr(kZeroSourceIndex, kZeroSource);

  machine.stage_rdram_u32_be(kDsllAddress, kDsllInstruction);
  machine.stage_rdram_u32_be(kDsrlAddress, kDsrlInstruction);
  machine.stage_rdram_u32_be(kDsraAddress, kDsraInstruction);
  machine.stage_rdram_u32_be(kDsll32Address, kDsll32Instruction);
  machine.stage_rdram_u32_be(kDsrl32Address, kDsrl32Instruction);
  machine.stage_rdram_u32_be(kDsra32Address, kDsra32Instruction);
  machine.stage_rdram_u32_be(kDsllvAddress, kDsllvInstruction);
  machine.stage_rdram_u32_be(kDsrlvAddress, kDsrlvInstruction);
  machine.stage_rdram_u32_be(kDsravAddress, kDsravInstruction);
  machine.stage_rdram_u32_be(kZeroDsllAddress, kZeroDsllInstruction);
  machine.stage_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout
      << "fn64 bootstrap D shift demo: DSLL/DSRL/DSRA cluster uses full 64-bit GPR values\n";
  std::cout << "before step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kDsllSourceIndex));
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kDsrlSourceIndex));
  print_hex64("  gpr[8]", machine.inspect_cpu_gpr(kDsraSourceIndex));
  print_hex64("  gpr[16]", machine.inspect_cpu_gpr(kVariableShiftIndex));

  auto step_d_shift_instruction = [&machine](
      CpuInstructionWord instruction,
      const char* label,
      CpuAddress expected_pc,
      std::uint8_t result_index,
      CpuRegisterValue expected_value) {
    print_hex32("  instruction_raw", instruction);
    std::cout << "  instruction_label = " << label << '\n';
    require_stepped(machine.step_cpu_instruction(), std::string("d_shift_demo_") + label);

    if (machine.cpu_pc() != cpu_rdram_alias(expected_pc)) {
      throw std::runtime_error(
          std::string("D shift demo advanced to the wrong PC after ") + label);
    }

    if (machine.inspect_cpu_gpr(result_index) != expected_value) {
      throw std::runtime_error(
          std::string("D shift demo produced the wrong result for ") + label);
    }
  };

  step_d_shift_instruction(
      kDsllInstruction,
      "DSLL",
      kDsrlAddress,
      kDsllResultIndex,
      kDsllExpected);
  step_d_shift_instruction(
      kDsrlInstruction,
      "DSRL",
      kDsraAddress,
      kDsrlResultIndex,
      kDsrlExpected);
  step_d_shift_instruction(
      kDsraInstruction,
      "DSRA",
      kDsll32Address,
      kDsraResultIndex,
      kDsraExpected);
  step_d_shift_instruction(
      kDsll32Instruction,
      "DSLL32",
      kDsrl32Address,
      kDsll32ResultIndex,
      kDsll32Expected);
  step_d_shift_instruction(
      kDsrl32Instruction,
      "DSRL32",
      kDsra32Address,
      kDsrl32ResultIndex,
      kDsrl32Expected);
  step_d_shift_instruction(
      kDsra32Instruction,
      "DSRA32",
      kDsllvAddress,
      kDsra32ResultIndex,
      kDsra32Expected);
  step_d_shift_instruction(
      kDsllvInstruction,
      "DSLLV",
      kDsrlvAddress,
      kDsllvResultIndex,
      kDsllvExpected);
  step_d_shift_instruction(
      kDsrlvInstruction,
      "DSRLV",
      kDsravAddress,
      kDsrlvResultIndex,
      kDsrlvExpected);
  step_d_shift_instruction(
      kDsravInstruction,
      "DSRAV",
      kZeroDsllAddress,
      kDsravResultIndex,
      kDsravExpected);
  step_d_shift_instruction(
      kZeroDsllInstruction,
      "DSLL $0",
      kBreakAddress,
      0,
      0);

  require_stopped(machine.step_cpu_instruction(), "d_shift_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kDsllResultIndex));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kDsrlResultIndex));
  print_hex64("  gpr[9]", machine.inspect_cpu_gpr(kDsraResultIndex));
  print_hex64("  gpr[11]", machine.inspect_cpu_gpr(kDsll32ResultIndex));
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kDsrl32ResultIndex));
  print_hex64("  gpr[15]", machine.inspect_cpu_gpr(kDsra32ResultIndex));
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kDsllvResultIndex));
  print_hex64("  gpr[20]", machine.inspect_cpu_gpr(kDsrlvResultIndex));
  print_hex64("  gpr[22]", machine.inspect_cpu_gpr(kDsravResultIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kAfterBreakAddress)) {
    throw std::runtime_error("D shift demo did not advance past BREAK");
  }
}

void run_cpu_local_single_ori_step_demo(Machine& machine) {
  constexpr std::uint8_t kZeroIndex = 0;
  constexpr std::uint8_t kSourceIndex = 4;

  constexpr std::uint32_t kPc = 0x00000680u;
  constexpr std::uint32_t kNextPc = 0x00000684u;
  constexpr std::uint32_t kSourceValue = 0x12340000u;
  constexpr std::uint32_t kInstruction = encode_ori(
      kZeroIndex,
      kSourceIndex,
      0x00ffu);

  machine.stage_cpu_pc(cpu_rdram_alias(kPc));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kNextPc));
  machine.stage_cpu_gpr(kSourceIndex, kSourceValue);
  machine.stage_cpu_gpr(kZeroIndex, 0xffffffffu);
  machine.stage_rdram_u32_be(kPc, kInstruction);

  const std::uint32_t raw = kInstruction;

  std::cout << "fn64 bootstrap CPU step demo: single ORI keeps zero register explicit\n";
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.inspect_cpu_gpr(kZeroIndex));
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kSourceIndex));
  print_hex32("  ori_raw", raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_step_single_ori");

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.inspect_cpu_gpr(kZeroIndex));
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kSourceIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kNextPc)) {
    throw std::runtime_error("CPU step demo did not advance pc after ORI");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kNextPc + 4u)) {
    throw std::runtime_error("CPU step demo did not advance next_pc after ORI");
  }

  if (machine.inspect_cpu_gpr(kZeroIndex) != 0) {
    throw std::runtime_error("CPU step demo wrote to gpr[0]");
  }

  if (machine.inspect_cpu_gpr(kSourceIndex) != kSourceValue) {
    throw std::runtime_error("CPU step demo changed the source register");
  }
}

void run_cpu_local_addiu_aliased_source_target_step_demo(Machine& machine) {
  constexpr std::uint8_t kAliasedIndex = 5;

  constexpr std::uint32_t kPc = 0x00000690u;
  constexpr std::uint32_t kNextPc = 0x00000694u;
  constexpr std::uint32_t kOriginalValue = 0x00000010u;
  constexpr std::uint32_t kExpectedValue = 0x0000000fu;
  constexpr std::uint32_t kInstruction = encode_addiu(
      kAliasedIndex,
      kAliasedIndex,
      0xffffu);

  machine.stage_cpu_pc(cpu_rdram_alias(kPc));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kNextPc));
  machine.stage_cpu_gpr(kAliasedIndex, kOriginalValue);
  machine.stage_rdram_u32_be(kPc, kInstruction);

  const std::uint32_t raw = kInstruction;

  std::cout
      << "fn64 bootstrap CPU step demo: ADDIU with rs == rt reads before writeback\n";
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kAliasedIndex));
  print_hex32("  addiu_raw", raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_step_aliased_addiu");

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kAliasedIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kNextPc)) {
    throw std::runtime_error("CPU step aliased ADDIU demo did not advance pc");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kNextPc + 4u)) {
    throw std::runtime_error(
        "CPU step aliased ADDIU demo did not advance next_pc");
  }

  if (machine.inspect_cpu_gpr(kAliasedIndex) != kExpectedValue) {
    throw std::runtime_error(
        "CPU step aliased ADDIU demo did not read original source before writeback");
  }
}

void run_cpu_local_sltiu_aliased_source_target_step_demo(Machine& machine) {
  constexpr std::uint8_t kAliasedIndex = 6;

  constexpr std::uint32_t kPc = 0x000006a0u;
  constexpr std::uint32_t kNextPc = 0x000006a4u;
  constexpr std::uint32_t kOriginalValue = 0x00000002u;
  constexpr std::uint32_t kExpectedValue = 0x00000000u;
  constexpr std::uint32_t kInstruction = encode_sltiu(
      kAliasedIndex,
      kAliasedIndex,
      0x0001u);

  machine.stage_cpu_pc(cpu_rdram_alias(kPc));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kNextPc));
  machine.stage_cpu_gpr(kAliasedIndex, kOriginalValue);
  machine.stage_rdram_u32_be(kPc, kInstruction);

  const std::uint32_t raw = kInstruction;

  std::cout
      << "fn64 bootstrap CPU step demo: SLTIU with rs == rt reads before writeback\n";
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kAliasedIndex));
  print_hex32("  sltiu_raw", raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_step_aliased_sltiu");

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kAliasedIndex));

  if (machine.cpu_pc() != cpu_rdram_alias(kNextPc)) {
    throw std::runtime_error("CPU step aliased SLTIU demo did not advance pc");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kNextPc + 4u)) {
    throw std::runtime_error(
        "CPU step aliased SLTIU demo did not advance next_pc");
  }

  if (machine.inspect_cpu_gpr(kAliasedIndex) != kExpectedValue) {
    throw std::runtime_error(
        "CPU step aliased SLTIU demo did not read original source before writeback");
  }
}

void step_hilo_instruction(
    Machine& machine,
    std::uint32_t instruction,
    const char* label) {
  const std::uint32_t instruction_address = machine.cpu_pc();
  machine.stage_rdram_u32_be(rdram_offset_from_cpu_address(instruction_address), instruction);

  const std::uint32_t raw = instruction;

  print_hex32("  instruction_raw", raw);
  std::cout << "  instruction_label = " << label << '\n';

  require_stepped(machine.step_cpu_instruction(), std::string("hilo_step_") + label);
}

void run_cpu_register_value_width_demo(Machine& machine) {
  constexpr CpuAddress kPc = 0x000006f0u;
  constexpr CpuAddress kNextPc = 0x000006f4u;

  constexpr std::uint8_t kHighSourceIndex = 12;
  constexpr std::uint8_t kWordResultIndex = 13;
  constexpr std::uint8_t kHiSourceIndex = 14;
  constexpr std::uint8_t kLoSourceIndex = 15;
  constexpr std::uint8_t kHiReadIndex = 16;
  constexpr std::uint8_t kLoReadIndex = 17;
  constexpr std::uint8_t kSignSourceIndex = 18;
  constexpr std::uint8_t kSignResultIndex = 19;
  constexpr std::uint8_t kShiftSourceIndex = 20;
  constexpr std::uint8_t kShiftResultIndex = 21;
  constexpr std::uint8_t kLuiResultIndex = 22;

  constexpr CpuRegisterValue kHighSourceValue = 0x12345678000012abull;
  constexpr CpuRegisterValue kInitialWordResultValue = 0xfedcba9800000000ull;
  constexpr CpuRegisterValue kSignSourceValue = 0x123456787fffffffull;
  constexpr CpuRegisterValue kInitialSignResultValue = 0x1111111122222222ull;
  constexpr CpuRegisterValue kShiftSourceValue = 0x5555555500000001ull;
  constexpr CpuRegisterValue kInitialShiftResultValue = 0x3333333344444444ull;
  constexpr CpuRegisterValue kInitialLuiResultValue = 0x7777777788888888ull;
  constexpr CpuRegisterValue kHiValue = 0x0123456789abcdefull;
  constexpr CpuRegisterValue kLoValue = 0xfedcba9876543210ull;
  constexpr CpuRegisterValue kMthiValue = 0x0badf00d12345678ull;
  constexpr CpuRegisterValue kMtloValue = 0xc001d00d87654321ull;
  constexpr CpuRegisterValue kZeroAttempt = 0xffffffffffffffffull;

  constexpr CpuRegisterValue kExpectedPositiveAddiuResult =
      cpu_value_from_sign_extended_u32(0x000012ffu);
  constexpr CpuRegisterValue kExpectedNegativeAddiuResult =
      cpu_value_from_sign_extended_u32(0x80000000u);
  constexpr CpuRegisterValue kExpectedShiftResult =
      cpu_value_from_sign_extended_u32(0x80000000u);
  constexpr CpuRegisterValue kExpectedLuiResult =
      cpu_value_from_sign_extended_u32(0x80000000u);

  constexpr CpuInstructionWord kPositiveAddiuInstruction = encode_addiu(
      kWordResultIndex,
      kHighSourceIndex,
      0x0054u);
  constexpr CpuInstructionWord kNegativeAddiuInstruction = encode_addiu(
      kSignResultIndex,
      kSignSourceIndex,
      0x0001u);
  constexpr CpuInstructionWord kShiftInstruction = encode_special(
      0,
      kShiftSourceIndex,
      kShiftResultIndex,
      31,
      0x00);
  constexpr CpuInstructionWord kLuiInstruction = encode_lui(kLuiResultIndex, 0x8000u);

  machine.stage_cpu_pc(cpu_rdram_alias(kPc));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kNextPc));
  machine.stage_cpu_gpr(0, kZeroAttempt);
  machine.stage_cpu_gpr(kHighSourceIndex, kHighSourceValue);
  machine.stage_cpu_gpr(kWordResultIndex, kInitialWordResultValue);
  machine.stage_cpu_gpr(kSignSourceIndex, kSignSourceValue);
  machine.stage_cpu_gpr(kSignResultIndex, kInitialSignResultValue);
  machine.stage_cpu_gpr(kShiftSourceIndex, kShiftSourceValue);
  machine.stage_cpu_gpr(kShiftResultIndex, kInitialShiftResultValue);
  machine.stage_cpu_gpr(kLuiResultIndex, kInitialLuiResultValue);
  machine.stage_cpu_gpr(kHiSourceIndex, kMthiValue);
  machine.stage_cpu_gpr(kLoSourceIndex, kMtloValue);
  machine.stage_cpu_gpr(kHiReadIndex, 0);
  machine.stage_cpu_gpr(kLoReadIndex, 0);
  machine.stage_cpu_hi(kHiValue);
  machine.stage_cpu_lo(kLoValue);

  std::cout
      << "fn64 bootstrap CPU register value width demo: 64-bit storage with local 32-bit word execution\n";
  std::cout << "before step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.inspect_cpu_gpr(0));
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kHighSourceIndex));
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kWordResultIndex));
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kSignSourceIndex));
  print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kSignResultIndex));
  print_hex64("  gpr[20]", machine.inspect_cpu_gpr(kShiftSourceIndex));
  print_hex64("  gpr[21]", machine.inspect_cpu_gpr(kShiftResultIndex));
  print_hex64("  gpr[22]", machine.inspect_cpu_gpr(kLuiResultIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());

  if (machine.inspect_cpu_gpr(0) != 0) {
    throw std::runtime_error("CPU register width demo did not keep gpr[0] at full zero");
  }

  if (machine.inspect_cpu_gpr(kHighSourceIndex) != kHighSourceValue ||
      machine.inspect_cpu_hi() != kHiValue ||
      machine.inspect_cpu_lo() != kLoValue) {
    throw std::runtime_error("CPU register width demo did not preserve staged 64-bit values");
  }

  step_hilo_instruction(
      machine,
      encode_special(0, 0, kHiReadIndex, 0, 0x10),
      "MFHI 64-bit value");
  step_hilo_instruction(
      machine,
      encode_special(0, 0, kLoReadIndex, 0, 0x12),
      "MFLO 64-bit value");

  if (machine.inspect_cpu_gpr(kHiReadIndex) != kHiValue ||
      machine.inspect_cpu_gpr(kLoReadIndex) != kLoValue) {
    throw std::runtime_error("CPU register width demo did not move full HI/LO values into GPRs");
  }

  step_hilo_instruction(
      machine,
      encode_special(kHiSourceIndex, 0, 0, 0, 0x11),
      "MTHI 64-bit value");
  step_hilo_instruction(
      machine,
      encode_special(kLoSourceIndex, 0, 0, 0, 0x13),
      "MTLO 64-bit value");

  if (machine.inspect_cpu_hi() != kMthiValue || machine.inspect_cpu_lo() != kMtloValue) {
    throw std::runtime_error("CPU register width demo did not move full GPR values into HI/LO");
  }

  auto step_word_policy_instruction =
      [&machine](CpuInstructionWord instruction, const char* label) {
        machine.stage_rdram_u32_be(
            rdram_offset_from_cpu_address(machine.cpu_pc()),
            instruction);
        print_hex32("  instruction_raw", instruction);
        std::cout << "  instruction_label = " << label << '\n';
        require_stepped(
            machine.step_cpu_instruction(),
            std::string("register_value_width_demo_") + label);
      };

  step_word_policy_instruction(kPositiveAddiuInstruction, "positive_addiu");

  if (machine.inspect_cpu_gpr(kWordResultIndex) != kExpectedPositiveAddiuResult) {
    throw std::runtime_error(
        "CPU register width demo did not sign-extend the positive ADDIU word result");
  }

  step_word_policy_instruction(kNegativeAddiuInstruction, "negative_addiu");

  if (machine.inspect_cpu_gpr(kSignResultIndex) != kExpectedNegativeAddiuResult) {
    throw std::runtime_error(
        "CPU register width demo did not sign-extend the negative ADDIU word result");
  }

  step_word_policy_instruction(kShiftInstruction, "sll_bit31");

  if (machine.inspect_cpu_gpr(kShiftResultIndex) != kExpectedShiftResult) {
    throw std::runtime_error(
        "CPU register width demo did not sign-extend the SLL word result");
  }

  step_word_policy_instruction(kLuiInstruction, "lui_8000");

  if (machine.inspect_cpu_gpr(kLuiResultIndex) != kExpectedLuiResult) {
    throw std::runtime_error(
        "CPU register width demo did not sign-extend the LUI 0x8000 word result");
  }

  std::cout << "after step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[12]", machine.inspect_cpu_gpr(kHighSourceIndex));
  print_hex64("  gpr[13]", machine.inspect_cpu_gpr(kWordResultIndex));
  print_hex64("  gpr[16]", machine.inspect_cpu_gpr(kHiReadIndex));
  print_hex64("  gpr[17]", machine.inspect_cpu_gpr(kLoReadIndex));
  print_hex64("  gpr[18]", machine.inspect_cpu_gpr(kSignSourceIndex));
  print_hex64("  gpr[19]", machine.inspect_cpu_gpr(kSignResultIndex));
  print_hex64("  gpr[20]", machine.inspect_cpu_gpr(kShiftSourceIndex));
  print_hex64("  gpr[21]", machine.inspect_cpu_gpr(kShiftResultIndex));
  print_hex64("  gpr[22]", machine.inspect_cpu_gpr(kLuiResultIndex));
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());

  if (machine.inspect_cpu_gpr(kHighSourceIndex) != kHighSourceValue) {
    throw std::runtime_error("CPU register width demo changed the 64-bit word source register");
  }

  constexpr std::uint32_t kSteppedInstructionCount = 8u;
  constexpr CpuAddress kExpectedFinalPc = kPc + (kSteppedInstructionCount * 4u);
  constexpr CpuAddress kExpectedFinalNextPc = kExpectedFinalPc + 4u;

  if (machine.cpu_pc() != cpu_rdram_alias(kExpectedFinalPc) ||
      machine.cpu_next_pc() != cpu_rdram_alias(kExpectedFinalNextPc)) {
    throw std::runtime_error("CPU register width demo did not advance through the step sequence");
  }
}

void run_hilo_arithmetic_demo(Machine& machine) {
  constexpr std::uint32_t kPc = 0x000006b0u;
  constexpr std::uint32_t kNextPc = 0x000006b4u;

  constexpr std::uint8_t kHiSourceIndex = 4;
  constexpr std::uint8_t kLoSourceIndex = 5;
  constexpr std::uint8_t kHiReadIndex = 6;
  constexpr std::uint8_t kLoReadIndex = 7;
  constexpr std::uint8_t kLhsIndex = 8;
  constexpr std::uint8_t kRhsIndex = 9;

  constexpr CpuRegisterValue kMthiValue = 0x89abcdef01234567ull;
  constexpr CpuRegisterValue kMtloValue = 0xfedcba9876543210ull;
  constexpr CpuRegisterValue kMultExpectedHi =
      cpu_value_from_sign_extended_u32(0xffffffffu);
  constexpr CpuRegisterValue kMultExpectedLo =
      cpu_value_from_sign_extended_u32(0xfffffffau);
  constexpr CpuRegisterValue kMultuExpectedHi =
      cpu_value_from_sign_extended_u32(0x00000001u);
  constexpr CpuRegisterValue kMultuExpectedLo =
      cpu_value_from_sign_extended_u32(0xfffffffeu);
  constexpr CpuRegisterValue kDivExpectedHi =
      cpu_value_from_sign_extended_u32(0xfffffffdu);
  constexpr CpuRegisterValue kDivExpectedLo =
      cpu_value_from_sign_extended_u32(0xfffffffeu);
  constexpr CpuRegisterValue kDivuExpectedHi =
      cpu_value_from_sign_extended_u32(0x00000000u);
  constexpr CpuRegisterValue kDivuExpectedLo =
      cpu_value_from_sign_extended_u32(0xffffffffu);
  constexpr CpuRegisterValue kDivZeroHi = 0x13579bdf2468ace0ull;
  constexpr CpuRegisterValue kDivZeroLo = 0xfedcba9800c0ffeeull;
  constexpr CpuRegisterValue kDivuZeroHi = 0x0badc0de12345678ull;
  constexpr CpuRegisterValue kDivuZeroLo = 0xc001d00d00c0ffeeull;

  machine.stage_cpu_pc(cpu_rdram_alias(kPc));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kNextPc));
  machine.stage_cpu_gpr(kHiSourceIndex, kMthiValue);
  machine.stage_cpu_gpr(kLoSourceIndex, kMtloValue);
  machine.stage_cpu_gpr(kHiReadIndex, 0);
  machine.stage_cpu_gpr(kLoReadIndex, 0);
  machine.stage_cpu_gpr(0, 0xffffffffu);

  std::cout << "fn64 bootstrap HI/LO arithmetic demo: Machine-owned HI/LO state transitions\n";
  std::cout << "before step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_hex64("  gpr[0]", machine.inspect_cpu_gpr(0));
  print_hex64("  gpr[4]", machine.inspect_cpu_gpr(kHiSourceIndex));
  print_hex64("  gpr[5]", machine.inspect_cpu_gpr(kLoSourceIndex));

  step_hilo_instruction(
      machine,
      encode_special(kHiSourceIndex, 0, 0, 0, 0x11),
      "MTHI");
  step_hilo_instruction(
      machine,
      encode_special(kLoSourceIndex, 0, 0, 0, 0x13),
      "MTLO");

  if (machine.inspect_cpu_hi() != kMthiValue || machine.inspect_cpu_lo() != kMtloValue) {
    throw std::runtime_error("HI/LO demo did not write HI/LO with MTHI/MTLO");
  }

  step_hilo_instruction(
      machine,
      encode_special(0, 0, kHiReadIndex, 0, 0x10),
      "MFHI");
  step_hilo_instruction(
      machine,
      encode_special(0, 0, kLoReadIndex, 0, 0x12),
      "MFLO");

  if (machine.inspect_cpu_gpr(kHiReadIndex) != kMthiValue ||
      machine.inspect_cpu_gpr(kLoReadIndex) != kMtloValue) {
    throw std::runtime_error("HI/LO demo did not read HI/LO with MFHI/MFLO");
  }

  step_hilo_instruction(
      machine,
      encode_special(0, 0, 0, 0, 0x10),
      "MFHI $0");
  step_hilo_instruction(
      machine,
      encode_special(0, 0, 0, 0, 0x12),
      "MFLO $0");

  if (machine.inspect_cpu_gpr(0) != 0) {
    throw std::runtime_error("HI/LO demo wrote HI/LO reads into gpr[0]");
  }

  machine.stage_cpu_gpr(kLhsIndex, cpu_value_from_sign_extended_u32(0xfffffffeu));
  machine.stage_cpu_gpr(kRhsIndex, 0x00000003u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x18),
      "MULT");

  if (machine.inspect_cpu_hi() != kMultExpectedHi || machine.inspect_cpu_lo() != kMultExpectedLo) {
    throw std::runtime_error("HI/LO demo signed MULT result was wrong");
  }

  machine.stage_cpu_gpr(kLhsIndex, cpu_value_from_sign_extended_u32(0xffffffffu));
  machine.stage_cpu_gpr(kRhsIndex, 0x00000002u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x19),
      "MULTU");

  if (machine.inspect_cpu_hi() != kMultuExpectedHi || machine.inspect_cpu_lo() != kMultuExpectedLo) {
    throw std::runtime_error("HI/LO demo unsigned MULTU result was wrong");
  }

  machine.stage_cpu_gpr(kLhsIndex, cpu_value_from_sign_extended_u32(0xfffffff3u));
  machine.stage_cpu_gpr(kRhsIndex, 0x00000005u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1a),
      "DIV");

  if (machine.inspect_cpu_hi() != kDivExpectedHi || machine.inspect_cpu_lo() != kDivExpectedLo) {
    throw std::runtime_error("HI/LO demo signed DIV result was wrong");
  }

  machine.stage_cpu_gpr(kLhsIndex, cpu_value_from_sign_extended_u32(0xffffffffu));
  machine.stage_cpu_gpr(kRhsIndex, 0x00000001u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1b),
      "DIVU");

  if (machine.inspect_cpu_hi() != kDivuExpectedHi || machine.inspect_cpu_lo() != kDivuExpectedLo) {
    throw std::runtime_error("HI/LO demo unsigned DIVU result was wrong");
  }

  machine.stage_cpu_hi(kDivZeroHi);
  machine.stage_cpu_lo(kDivZeroLo);
  machine.stage_cpu_gpr(kLhsIndex, 0x80000000u);
  machine.stage_cpu_gpr(kRhsIndex, 0);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1a),
      "DIV by zero");

  if (machine.inspect_cpu_hi() != kDivZeroHi || machine.inspect_cpu_lo() != kDivZeroLo) {
    throw std::runtime_error("HI/LO demo DIV by zero changed HI/LO");
  }

  machine.stage_cpu_hi(kDivuZeroHi);
  machine.stage_cpu_lo(kDivuZeroLo);
  machine.stage_cpu_gpr(kLhsIndex, 0xffffffffu);
  machine.stage_cpu_gpr(kRhsIndex, 0);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1b),
      "DIVU by zero");

  if (machine.inspect_cpu_hi() != kDivuZeroHi || machine.inspect_cpu_lo() != kDivuZeroLo) {
    throw std::runtime_error("HI/LO demo DIVU by zero changed HI/LO");
  }

  std::cout << "after step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_hex64("  gpr[0]", machine.inspect_cpu_gpr(0));
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kHiReadIndex));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kLoReadIndex));

  constexpr std::uint32_t kSteppedInstructionCount = 12u;
  constexpr std::uint32_t kExpectedFinalPc = kPc + (kSteppedInstructionCount * 4u);
  constexpr std::uint32_t kExpectedFinalNextPc = kExpectedFinalPc + 4u;

  if (machine.cpu_pc() != cpu_rdram_alias(kExpectedFinalPc)) {
    throw std::runtime_error("HI/LO demo did not advance pc through the step sequence");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kExpectedFinalNextPc)) {
    throw std::runtime_error("HI/LO demo did not advance next_pc through the step sequence");
  }
}

void run_d_hilo_arithmetic_demo(Machine& machine) {
  constexpr CpuAddress kPc = 0x00000a00u;
  constexpr CpuAddress kNextPc = 0x00000a04u;

  constexpr std::uint8_t kLhsIndex = 4;
  constexpr std::uint8_t kRhsIndex = 5;
  constexpr std::uint8_t kHiReadIndex = 6;
  constexpr std::uint8_t kLoReadIndex = 7;

  constexpr CpuRegisterValue kDmultPositiveLhs = 0x0000000100000000ull;
  constexpr CpuRegisterValue kDmultPositiveRhs = 0x0000000200000003ull;
  constexpr CpuRegisterValue kDmultPositiveExpectedHi = 0x0000000000000002ull;
  constexpr CpuRegisterValue kDmultPositiveExpectedLo = 0x0000000300000000ull;
  constexpr CpuRegisterValue kDmultNegativeExpectedHi = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kDmultNegativeExpectedLo = 0xfffffffffffffffeull;
  constexpr CpuRegisterValue kDmultuExpectedHi = 0x0000000000000001ull;
  constexpr CpuRegisterValue kDmultuExpectedLo = 0xfffffffffffffffeull;
  constexpr CpuRegisterValue kDdivExpectedHi = 0xffffffffffffffffull;
  constexpr CpuRegisterValue kDdivExpectedLo = 0xfffffffffffffffdull;
  constexpr CpuRegisterValue kDdivuExpectedHi = 0x0000000000000001ull;
  constexpr CpuRegisterValue kDdivuExpectedLo = 0x7fffffffffffffffull;
  constexpr CpuRegisterValue kDdivZeroHi = 0x123456789abcdef0ull;
  constexpr CpuRegisterValue kDdivZeroLo = 0x0fedcba987654321ull;
  constexpr CpuRegisterValue kDdivuZeroHi = 0x0102030405060708ull;
  constexpr CpuRegisterValue kDdivuZeroLo = 0x8877665544332211ull;
  constexpr CpuRegisterValue kDdivOverflowHi = 0xa5a5a5a55a5a5a5aull;
  constexpr CpuRegisterValue kDdivOverflowLo = 0x1122334455667788ull;

  machine.stage_cpu_pc(cpu_rdram_alias(kPc));
  machine.stage_cpu_next_pc(cpu_rdram_alias(kNextPc));
  machine.stage_cpu_gpr(kHiReadIndex, 0);
  machine.stage_cpu_gpr(kLoReadIndex, 0);

  std::cout
      << "fn64 bootstrap D HI/LO arithmetic demo: DMULT/DMULTU/DDIV/DDIVU execute over full 64-bit values\n";
  std::cout << "before step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());

  machine.stage_cpu_gpr(kLhsIndex, kDmultPositiveLhs);
  machine.stage_cpu_gpr(kRhsIndex, kDmultPositiveRhs);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1c),
      "DMULT positive");

  if (machine.inspect_cpu_hi() != kDmultPositiveExpectedHi ||
      machine.inspect_cpu_lo() != kDmultPositiveExpectedLo) {
    throw std::runtime_error("D HI/LO demo signed DMULT positive result was wrong");
  }

  machine.stage_cpu_gpr(kLhsIndex, 0xffffffffffffffffull);
  machine.stage_cpu_gpr(kRhsIndex, 0x0000000000000002ull);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1c),
      "DMULT negative");

  if (machine.inspect_cpu_hi() != kDmultNegativeExpectedHi ||
      machine.inspect_cpu_lo() != kDmultNegativeExpectedLo) {
    throw std::runtime_error("D HI/LO demo signed DMULT negative result was wrong");
  }

  machine.stage_cpu_gpr(kLhsIndex, 0xffffffffffffffffull);
  machine.stage_cpu_gpr(kRhsIndex, 0x0000000000000002ull);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1d),
      "DMULTU");

  if (machine.inspect_cpu_hi() != kDmultuExpectedHi ||
      machine.inspect_cpu_lo() != kDmultuExpectedLo) {
    throw std::runtime_error("D HI/LO demo unsigned DMULTU result was wrong");
  }

  step_hilo_instruction(
      machine,
      encode_special(0, 0, kHiReadIndex, 0, 0x10),
      "MFHI after DMULTU");
  step_hilo_instruction(
      machine,
      encode_special(0, 0, kLoReadIndex, 0, 0x12),
      "MFLO after DMULTU");

  if (machine.inspect_cpu_gpr(kHiReadIndex) != kDmultuExpectedHi ||
      machine.inspect_cpu_gpr(kLoReadIndex) != kDmultuExpectedLo) {
    throw std::runtime_error("D HI/LO demo MFHI/MFLO did not move full D product halves");
  }

  machine.stage_cpu_gpr(kLhsIndex, 0xfffffffffffffff9ull);
  machine.stage_cpu_gpr(kRhsIndex, 0x0000000000000002ull);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1e),
      "DDIV");

  if (machine.inspect_cpu_hi() != kDdivExpectedHi ||
      machine.inspect_cpu_lo() != kDdivExpectedLo) {
    throw std::runtime_error("D HI/LO demo signed DDIV result was wrong");
  }

  machine.stage_cpu_gpr(kLhsIndex, 0xffffffffffffffffull);
  machine.stage_cpu_gpr(kRhsIndex, 0x0000000000000002ull);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1f),
      "DDIVU");

  if (machine.inspect_cpu_hi() != kDdivuExpectedHi ||
      machine.inspect_cpu_lo() != kDdivuExpectedLo) {
    throw std::runtime_error("D HI/LO demo unsigned DDIVU result was wrong");
  }

  machine.stage_cpu_hi(kDdivZeroHi);
  machine.stage_cpu_lo(kDdivZeroLo);
  machine.stage_cpu_gpr(kLhsIndex, 0x8000000000000000ull);
  machine.stage_cpu_gpr(kRhsIndex, 0);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1e),
      "DDIV by zero");

  if (machine.inspect_cpu_hi() != kDdivZeroHi || machine.inspect_cpu_lo() != kDdivZeroLo) {
    throw std::runtime_error("D HI/LO demo DDIV by zero changed HI/LO");
  }

  machine.stage_cpu_hi(kDdivuZeroHi);
  machine.stage_cpu_lo(kDdivuZeroLo);
  machine.stage_cpu_gpr(kLhsIndex, 0xffffffffffffffffull);
  machine.stage_cpu_gpr(kRhsIndex, 0);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1f),
      "DDIVU by zero");

  if (machine.inspect_cpu_hi() != kDdivuZeroHi ||
      machine.inspect_cpu_lo() != kDdivuZeroLo) {
    throw std::runtime_error("D HI/LO demo DDIVU by zero changed HI/LO");
  }

  machine.stage_cpu_hi(kDdivOverflowHi);
  machine.stage_cpu_lo(kDdivOverflowLo);
  machine.stage_cpu_gpr(kLhsIndex, 0x8000000000000000ull);
  machine.stage_cpu_gpr(kRhsIndex, 0xffffffffffffffffull);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1e),
      "DDIV signed overflow edge");

  if (machine.inspect_cpu_hi() != kDdivOverflowHi ||
      machine.inspect_cpu_lo() != kDdivOverflowLo) {
    throw std::runtime_error("D HI/LO demo DDIV signed overflow edge changed HI/LO");
  }

  std::cout << "after step sequence:\n";
  print_control_flow_state(machine);
  print_hex64("  hi", machine.inspect_cpu_hi());
  print_hex64("  lo", machine.inspect_cpu_lo());
  print_hex64("  gpr[6]", machine.inspect_cpu_gpr(kHiReadIndex));
  print_hex64("  gpr[7]", machine.inspect_cpu_gpr(kLoReadIndex));

  constexpr std::uint32_t kSteppedInstructionCount = 10u;
  constexpr CpuAddress kExpectedFinalPc = kPc + (kSteppedInstructionCount * 4u);
  constexpr CpuAddress kExpectedFinalNextPc = kExpectedFinalPc + 4u;

  if (machine.cpu_pc() != cpu_rdram_alias(kExpectedFinalPc)) {
    throw std::runtime_error("D HI/LO demo did not advance pc through the step sequence");
  }

  if (machine.cpu_next_pc() != cpu_rdram_alias(kExpectedFinalNextPc)) {
    throw std::runtime_error("D HI/LO demo did not advance next_pc through the step sequence");
  }
}

}  // namespace

void run_arithmetic_demos(Machine& machine) {
  run_cpu_local_single_ori_step_demo(machine);
  run_cpu_local_addiu_aliased_source_target_step_demo(machine);
  run_cpu_local_sltiu_aliased_source_target_step_demo(machine);
  run_register_immediate_arithmetic_compare_demo(machine);
  run_add_positive_overflow_demo(machine);
  run_sub_negative_overflow_demo(machine);
  run_addi_positive_overflow_demo(machine);
  run_addi_negative_overflow_demo(machine);
  run_logic_immediate_unsigned_compare_demo(machine);
  run_full_width_register_compare_demo(machine);
  run_d_integer_add_sub_demo(machine);
  run_trapping_d_integer_add_sub_demo(machine);
  run_d_shift_demo(machine);
  run_cpu_register_value_width_demo(machine);
  run_hilo_arithmetic_demo(machine);
  run_d_hilo_arithmetic_demo(machine);
}

}  // namespace fn64::bootstrap_detail
