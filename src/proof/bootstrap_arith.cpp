#include "bootstrap_common.hpp"

#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fn64::bootstrap_detail {
namespace {

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

  machine.write_cpu_pc(kAddiAddress);
  machine.write_cpu_gpr(kAddiSourceIndex, 0x00000001u);
  machine.write_cpu_gpr(kAddiResultIndex, 0u);
  machine.write_cpu_gpr(kAddiuSourceIndex, 0x00000010u);
  machine.write_cpu_gpr(kAddiuResultIndex, 0u);
  machine.write_cpu_gpr(kSltiSourceIndex, 0xfffffffeu);
  machine.write_cpu_gpr(kSltiResultIndex, 0u);
  machine.write_cpu_gpr(kSltiuSourceIndex, 0u);
  machine.write_cpu_gpr(kSltiuResultIndex, 0u);

  machine.write_rdram_u32_be(kAddiAddress, kAddiInstruction);
  machine.write_rdram_u32_be(kAddiuAddress, kAddiuInstruction);
  machine.write_rdram_u32_be(kSltiAddress, kSltiInstruction);
  machine.write_rdram_u32_be(kSltiuAddress, kSltiuInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout
      << "fn64 bootstrap reg-immediate arithmetic/compare demo: explicit negative-immediate "
         "ADDI/ADDIU/SLTI/SLTIU proof\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kAddiSourceIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kAddiResultIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kAddiuSourceIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kAddiuResultIndex));
  print_hex64("  gpr[8]", machine.read_cpu_gpr(kSltiSourceIndex));
  print_hex64("  gpr[9]", machine.read_cpu_gpr(kSltiResultIndex));
  print_hex64("  gpr[10]", machine.read_cpu_gpr(kSltiuSourceIndex));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kSltiuResultIndex));

  const std::uint32_t addi_raw = kAddiInstruction;

  print_hex32("  addi_raw", addi_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_addi");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kAddiResultIndex));

  if (machine.cpu_pc() != kAddiuAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from ADDI to ADDIU");
  }

  if (machine.read_cpu_gpr(kAddiResultIndex) != 0u) {
    throw std::runtime_error("reg-immediate demo ADDI negative immediate result was wrong");
  }

  const std::uint32_t addiu_raw = kAddiuInstruction;

  print_hex32("  addiu_raw", addiu_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_addiu");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kAddiuResultIndex));

  if (machine.cpu_pc() != kSltiAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from ADDIU to SLTI");
  }

  if (machine.read_cpu_gpr(kAddiuResultIndex) != 0x0000000eu) {
    throw std::runtime_error("reg-immediate demo ADDIU negative immediate result was wrong");
  }

  const std::uint32_t slti_raw = kSltiInstruction;

  print_hex32("  slti_raw", slti_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_slti");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.read_cpu_gpr(kSltiResultIndex));

  if (machine.cpu_pc() != kSltiuAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from SLTI to SLTIU");
  }

  if (machine.read_cpu_gpr(kSltiResultIndex) != 1u) {
    throw std::runtime_error("reg-immediate demo SLTI negative immediate compare result was wrong");
  }

  const std::uint32_t sltiu_raw = kSltiuInstruction;

  print_hex32("  sltiu_raw", sltiu_raw);

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_sltiu");

  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kSltiuResultIndex));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from SLTIU to BREAK");
  }

  if (machine.read_cpu_gpr(kSltiuResultIndex) != 1u) {
    throw std::runtime_error("reg-immediate demo SLTIU negative immediate result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "reg_immediate_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);

  if (machine.cpu_pc() != kAfterBreakAddress) {
    throw std::runtime_error("reg-immediate demo did not advance past executed BREAK");
  }
}

void run_add_positive_overflow_demo(Machine& machine) {
  constexpr std::size_t kLhsIndex = 4;
  constexpr std::size_t kRhsIndex = 5;
  constexpr std::size_t kResultIndex = 6;

  constexpr std::uint32_t kAddAddress = 0x00000420u;
  constexpr std::uint32_t kAfterAddAddress = 0x00000424u;
  constexpr std::uint32_t kAddInstruction = encode_special(
      static_cast<std::uint8_t>(kLhsIndex),
      static_cast<std::uint8_t>(kRhsIndex),
      static_cast<std::uint8_t>(kResultIndex),
      0,
      0x20);

  machine.write_cpu_pc(kAddAddress);
  machine.write_cpu_gpr(kLhsIndex, 0x7fffffffu);
  machine.write_cpu_gpr(kRhsIndex, 0x00000001u);
  machine.write_cpu_gpr(kResultIndex, 0x2468ace0u);

  machine.write_rdram_u32_be(kAddAddress, kAddInstruction);

  std::cout << "fn64 bootstrap register arithmetic demo: ADD positive overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kLhsIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kRhsIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kResultIndex));

  const std::uint32_t add_raw = kAddInstruction;

  print_hex32("  add_raw", add_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::exception& exception) {
    std::cout << "  exception = " << exception.what() << '\n';
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != kAddAddress) {
      throw std::runtime_error("add positive overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != kAfterAddAddress) {
      throw std::runtime_error("add positive overflow demo did not restore next_pc after throw");
    }

    if (machine.read_cpu_gpr(kResultIndex) != 0x2468ace0u) {
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
  constexpr std::uint32_t kSubInstruction = encode_special(
      static_cast<std::uint8_t>(kLhsIndex),
      static_cast<std::uint8_t>(kRhsIndex),
      static_cast<std::uint8_t>(kResultIndex),
      0,
      0x22);

  machine.write_cpu_pc(kSubAddress);
  machine.write_cpu_gpr(kLhsIndex, 0x80000000u);
  machine.write_cpu_gpr(kRhsIndex, 0x00000001u);
  machine.write_cpu_gpr(kResultIndex, 0x13579bdfu);

  machine.write_rdram_u32_be(kSubAddress, kSubInstruction);

  std::cout << "fn64 bootstrap register arithmetic demo: SUB negative overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kLhsIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kRhsIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kResultIndex));

  const std::uint32_t sub_raw = kSubInstruction;

  print_hex32("  sub_raw", sub_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::exception& exception) {
    std::cout << "  exception = " << exception.what() << '\n';
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[6]", machine.read_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != kSubAddress) {
      throw std::runtime_error("sub negative overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != kAfterSubAddress) {
      throw std::runtime_error("sub negative overflow demo did not restore next_pc after throw");
    }

    if (machine.read_cpu_gpr(kResultIndex) != 0x13579bdfu) {
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
  constexpr std::uint32_t kAddiInstruction = encode_addi(
      static_cast<std::uint8_t>(kResultIndex),
      static_cast<std::uint8_t>(kSourceIndex),
      0x0001u);

  machine.write_cpu_pc(kAddiAddress);
  machine.write_cpu_gpr(kSourceIndex, 0x7fffffffu);
  machine.write_cpu_gpr(kResultIndex, 0x2468ace0u);

  machine.write_rdram_u32_be(kAddiAddress, kAddiInstruction);

  std::cout << "fn64 bootstrap reg-immediate arithmetic demo: ADDI positive overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kResultIndex));

  const std::uint32_t addi_raw = kAddiInstruction;

  print_hex32("  addi_raw", addi_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::exception& exception) {
    std::cout << "  exception = " << exception.what() << '\n';
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[5]", machine.read_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != kAddiAddress) {
      throw std::runtime_error("addi positive overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != kAfterAddiAddress) {
      throw std::runtime_error("addi positive overflow demo did not restore next_pc after throw");
    }

    if (machine.read_cpu_gpr(kResultIndex) != 0x2468ace0u) {
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
  constexpr std::uint32_t kAddiInstruction = encode_addi(
      static_cast<std::uint8_t>(kResultIndex),
      static_cast<std::uint8_t>(kSourceIndex),
      0xffffu);

  machine.write_cpu_pc(kAddiAddress);
  machine.write_cpu_gpr(kSourceIndex, 0x80000000u);
  machine.write_cpu_gpr(kResultIndex, 0x13579bdfu);

  machine.write_rdram_u32_be(kAddiAddress, kAddiInstruction);

  std::cout << "fn64 bootstrap reg-immediate arithmetic demo: ADDI negative overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kResultIndex));

  const std::uint32_t addi_raw = kAddiInstruction;

  print_hex32("  addi_raw", addi_raw);

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::exception& exception) {
    std::cout << "  exception = " << exception.what() << '\n';
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[5]", machine.read_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != kAddiAddress) {
      throw std::runtime_error("addi negative overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != kAfterAddiAddress) {
      throw std::runtime_error("addi negative overflow demo did not restore next_pc after throw");
    }

    if (machine.read_cpu_gpr(kResultIndex) != 0x13579bdfu) {
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

  machine.write_cpu_pc(kLuiAddress);
  machine.write_cpu_gpr(kValueIndex, 0);
  machine.write_cpu_gpr(kAndResultIndex, 0);
  machine.write_cpu_gpr(kXorResultIndex, 0);
  machine.write_cpu_gpr(kMaxIndex, 0);
  machine.write_cpu_gpr(kOneIndex, 0);
  machine.write_cpu_gpr(kCompareResultIndex, 0);

  machine.write_rdram_u32_be(kLuiAddress, kLuiInstruction);
  machine.write_rdram_u32_be(kOriAddress, kOriInstruction);
  machine.write_rdram_u32_be(kAndiAddress, kAndiInstruction);
  machine.write_rdram_u32_be(kXoriAddress, kXoriInstruction);
  machine.write_rdram_u32_be(kMaxLuiAddress, kMaxLuiInstruction);
  machine.write_rdram_u32_be(kMaxOriAddress, kMaxOriInstruction);
  machine.write_rdram_u32_be(kOneOriAddress, kOneOriInstruction);
  machine.write_rdram_u32_be(kSltuAddress, kSltuInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap logic/immediate demo: explicit LUI/ORI/ANDI/XORI/SLTU proof\n";
  std::cout << "before step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kValueIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kAndResultIndex));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kXorResultIndex));
  print_hex64("  gpr[10]", machine.read_cpu_gpr(kMaxIndex));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kOneIndex));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kCompareResultIndex));

  const std::uint32_t lui_raw = kLuiInstruction;

  print_hex32("  lui_raw", lui_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_lui");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kValueIndex));

  if (machine.cpu_pc() != kOriAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from LUI to ORI");
  }

  if (machine.read_cpu_gpr(kValueIndex) != 0xabcd0000u) {
    throw std::runtime_error("logic/immediate demo LUI result was wrong");
  }

  const std::uint32_t ori_raw = kOriInstruction;

  print_hex32("  ori_raw", ori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_ori");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kValueIndex));

  if (machine.cpu_pc() != kAndiAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from ORI to ANDI");
  }

  if (machine.read_cpu_gpr(kValueIndex) != 0xabcd1234u) {
    throw std::runtime_error("logic/immediate demo ORI result was wrong");
  }

  const std::uint32_t andi_raw = kAndiInstruction;

  print_hex32("  andi_raw", andi_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_andi");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kAndResultIndex));

  if (machine.cpu_pc() != kXoriAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from ANDI to XORI");
  }

  if (machine.read_cpu_gpr(kAndResultIndex) != 0x00000030u) {
    throw std::runtime_error("logic/immediate demo ANDI result was wrong");
  }

  const std::uint32_t xori_raw = kXoriInstruction;

  print_hex32("  xori_raw", xori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_xori");

  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kXorResultIndex));

  if (machine.cpu_pc() != kMaxLuiAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from XORI to second LUI");
  }

  if (machine.read_cpu_gpr(kXorResultIndex) != 0xabcd12cbu) {
    throw std::runtime_error("logic/immediate demo XORI result was wrong");
  }

  const std::uint32_t max_lui_raw = kMaxLuiInstruction;

  print_hex32("  max_lui_raw", max_lui_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_max_lui");

  std::cout << "after step 5:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[10]", machine.read_cpu_gpr(kMaxIndex));

  if (machine.cpu_pc() != kMaxOriAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from second LUI to second ORI");
  }

  if (machine.read_cpu_gpr(kMaxIndex) != 0xffff0000u) {
    throw std::runtime_error("logic/immediate demo second LUI result was wrong");
  }

  const std::uint32_t max_ori_raw = kMaxOriInstruction;

  print_hex32("  max_ori_raw", max_ori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_max_ori");

  std::cout << "after step 6:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[10]", machine.read_cpu_gpr(kMaxIndex));

  if (machine.cpu_pc() != kOneOriAddress) {
    throw std::runtime_error(
        "logic/immediate demo did not advance from second ORI to one-building ORI");
  }

  if (machine.read_cpu_gpr(kMaxIndex) != 0xffffffffu) {
    throw std::runtime_error("logic/immediate demo second ORI result was wrong");
  }

  const std::uint32_t one_ori_raw = kOneOriInstruction;

  print_hex32("  one_ori_raw", one_ori_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_one_ori");

  std::cout << "after step 7:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kOneIndex));

  if (machine.cpu_pc() != kSltuAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from one-building ORI to SLTU");
  }

  if (machine.read_cpu_gpr(kOneIndex) != 0x00000001u) {
    throw std::runtime_error("logic/immediate demo one-building ORI result was wrong");
  }

  const std::uint32_t sltu_raw = kSltuInstruction;

  print_hex32("  sltu_raw", sltu_raw);

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_sltu");

  std::cout << "after step 8:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[10]", machine.read_cpu_gpr(kMaxIndex));
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kOneIndex));
  print_hex64("  gpr[12]", machine.read_cpu_gpr(kCompareResultIndex));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from SLTU to BREAK");
  }

  if (machine.read_cpu_gpr(kCompareResultIndex) != 0) {
    throw std::runtime_error("logic/immediate demo SLTU did not prove unsigned compare semantics");
  }

  require_stopped(machine.step_cpu_instruction(), "logic_immediate_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);

  if (machine.cpu_pc() != kAfterBreakAddress) {
    throw std::runtime_error("logic/immediate demo did not advance past executed BREAK");
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

  machine.write_cpu_pc(kPc);
  machine.write_cpu_next_pc(kNextPc);
  machine.write_cpu_gpr(kSourceIndex, kSourceValue);
  machine.write_cpu_gpr(kZeroIndex, 0xffffffffu);
  machine.write_rdram_u32_be(kPc, kInstruction);

  const std::uint32_t raw = kInstruction;

  std::cout << "fn64 bootstrap CPU step demo: single ORI keeps zero register explicit\n";
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.read_cpu_gpr(kZeroIndex));
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kSourceIndex));
  print_hex32("  ori_raw", raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_step_single_ori");

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[0]", machine.read_cpu_gpr(kZeroIndex));
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kSourceIndex));

  if (machine.cpu_pc() != kNextPc) {
    throw std::runtime_error("CPU step demo did not advance pc after ORI");
  }

  if (machine.cpu_next_pc() != kNextPc + 4u) {
    throw std::runtime_error("CPU step demo did not advance next_pc after ORI");
  }

  if (machine.read_cpu_gpr(kZeroIndex) != 0) {
    throw std::runtime_error("CPU step demo wrote to gpr[0]");
  }

  if (machine.read_cpu_gpr(kSourceIndex) != kSourceValue) {
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

  machine.write_cpu_pc(kPc);
  machine.write_cpu_next_pc(kNextPc);
  machine.write_cpu_gpr(kAliasedIndex, kOriginalValue);
  machine.write_rdram_u32_be(kPc, kInstruction);

  const std::uint32_t raw = kInstruction;

  std::cout
      << "fn64 bootstrap CPU step demo: ADDIU with rs == rt reads before writeback\n";
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kAliasedIndex));
  print_hex32("  addiu_raw", raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_step_aliased_addiu");

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kAliasedIndex));

  if (machine.cpu_pc() != kNextPc) {
    throw std::runtime_error("CPU step aliased ADDIU demo did not advance pc");
  }

  if (machine.cpu_next_pc() != kNextPc + 4u) {
    throw std::runtime_error(
        "CPU step aliased ADDIU demo did not advance next_pc");
  }

  if (machine.read_cpu_gpr(kAliasedIndex) != kExpectedValue) {
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

  machine.write_cpu_pc(kPc);
  machine.write_cpu_next_pc(kNextPc);
  machine.write_cpu_gpr(kAliasedIndex, kOriginalValue);
  machine.write_rdram_u32_be(kPc, kInstruction);

  const std::uint32_t raw = kInstruction;

  std::cout
      << "fn64 bootstrap CPU step demo: SLTIU with rs == rt reads before writeback\n";
  std::cout << "before step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kAliasedIndex));
  print_hex32("  sltiu_raw", raw);

  require_stepped(machine.step_cpu_instruction(), "cpu_step_aliased_sltiu");

  std::cout << "after step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kAliasedIndex));

  if (machine.cpu_pc() != kNextPc) {
    throw std::runtime_error("CPU step aliased SLTIU demo did not advance pc");
  }

  if (machine.cpu_next_pc() != kNextPc + 4u) {
    throw std::runtime_error(
        "CPU step aliased SLTIU demo did not advance next_pc");
  }

  if (machine.read_cpu_gpr(kAliasedIndex) != kExpectedValue) {
    throw std::runtime_error(
        "CPU step aliased SLTIU demo did not read original source before writeback");
  }
}

void step_hilo_instruction(
    Machine& machine,
    std::uint32_t instruction,
    const char* label) {
  const std::uint32_t instruction_address = machine.cpu_pc();
  machine.write_rdram_u32_be(instruction_address, instruction);

  const std::uint32_t raw = instruction;

  print_hex32("  instruction_raw", raw);
  std::cout << "  instruction_label = " << label << '\n';

  require_stepped(machine.step_cpu_instruction(), std::string("hilo_step_") + label);
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

  constexpr std::uint32_t kMthiValue = 0x89abcdefu;
  constexpr std::uint32_t kMtloValue = 0x01234567u;
  constexpr std::uint32_t kMultExpectedHi = 0xffffffffu;
  constexpr std::uint32_t kMultExpectedLo = 0xfffffffau;
  constexpr std::uint32_t kMultuExpectedHi = 0x00000001u;
  constexpr std::uint32_t kMultuExpectedLo = 0xfffffffeu;
  constexpr std::uint32_t kDivExpectedHi = 0xfffffffdu;
  constexpr std::uint32_t kDivExpectedLo = 0xfffffffeu;
  constexpr std::uint32_t kDivuExpectedHi = 0x00000003u;
  constexpr std::uint32_t kDivuExpectedLo = 0x00000002u;
  constexpr std::uint32_t kDivZeroHi = 0x13579bdfu;
  constexpr std::uint32_t kDivZeroLo = 0x2468ace0u;
  constexpr std::uint32_t kDivuZeroHi = 0x0badc0deu;
  constexpr std::uint32_t kDivuZeroLo = 0x00c0ffeeu;

  machine.write_cpu_pc(kPc);
  machine.write_cpu_next_pc(kNextPc);
  machine.write_cpu_gpr(kHiSourceIndex, kMthiValue);
  machine.write_cpu_gpr(kLoSourceIndex, kMtloValue);
  machine.write_cpu_gpr(kHiReadIndex, 0);
  machine.write_cpu_gpr(kLoReadIndex, 0);
  machine.write_cpu_gpr(0, 0xffffffffu);

  std::cout << "fn64 bootstrap HI/LO arithmetic demo: Machine-owned HI/LO state transitions\n";
  std::cout << "before step sequence:\n";
  print_control_flow_state(machine);
  print_hex32("  hi", machine.cpu_hi());
  print_hex32("  lo", machine.cpu_lo());
  print_hex64("  gpr[0]", machine.read_cpu_gpr(0));
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kHiSourceIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kLoSourceIndex));

  step_hilo_instruction(
      machine,
      encode_special(kHiSourceIndex, 0, 0, 0, 0x11),
      "MTHI");
  step_hilo_instruction(
      machine,
      encode_special(kLoSourceIndex, 0, 0, 0, 0x13),
      "MTLO");

  if (machine.cpu_hi() != kMthiValue || machine.cpu_lo() != kMtloValue) {
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

  if (machine.read_cpu_gpr(kHiReadIndex) != kMthiValue ||
      machine.read_cpu_gpr(kLoReadIndex) != kMtloValue) {
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

  if (machine.read_cpu_gpr(0) != 0) {
    throw std::runtime_error("HI/LO demo wrote HI/LO reads into gpr[0]");
  }

  machine.write_cpu_gpr(kLhsIndex, 0xfffffffeu);
  machine.write_cpu_gpr(kRhsIndex, 0x00000003u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x18),
      "MULT");

  if (machine.cpu_hi() != kMultExpectedHi || machine.cpu_lo() != kMultExpectedLo) {
    throw std::runtime_error("HI/LO demo signed MULT result was wrong");
  }

  machine.write_cpu_gpr(kLhsIndex, 0xffffffffu);
  machine.write_cpu_gpr(kRhsIndex, 0x00000002u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x19),
      "MULTU");

  if (machine.cpu_hi() != kMultuExpectedHi || machine.cpu_lo() != kMultuExpectedLo) {
    throw std::runtime_error("HI/LO demo unsigned MULTU result was wrong");
  }

  machine.write_cpu_gpr(kLhsIndex, 0xfffffff3u);
  machine.write_cpu_gpr(kRhsIndex, 0x00000005u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1a),
      "DIV");

  if (machine.cpu_hi() != kDivExpectedHi || machine.cpu_lo() != kDivExpectedLo) {
    throw std::runtime_error("HI/LO demo signed DIV result was wrong");
  }

  machine.write_cpu_gpr(kLhsIndex, 0x0000000du);
  machine.write_cpu_gpr(kRhsIndex, 0x00000005u);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1b),
      "DIVU");

  if (machine.cpu_hi() != kDivuExpectedHi || machine.cpu_lo() != kDivuExpectedLo) {
    throw std::runtime_error("HI/LO demo unsigned DIVU result was wrong");
  }

  machine.write_cpu_hi(kDivZeroHi);
  machine.write_cpu_lo(kDivZeroLo);
  machine.write_cpu_gpr(kLhsIndex, 0x80000000u);
  machine.write_cpu_gpr(kRhsIndex, 0);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1a),
      "DIV by zero");

  if (machine.cpu_hi() != kDivZeroHi || machine.cpu_lo() != kDivZeroLo) {
    throw std::runtime_error("HI/LO demo DIV by zero changed HI/LO");
  }

  machine.write_cpu_hi(kDivuZeroHi);
  machine.write_cpu_lo(kDivuZeroLo);
  machine.write_cpu_gpr(kLhsIndex, 0xffffffffu);
  machine.write_cpu_gpr(kRhsIndex, 0);
  step_hilo_instruction(
      machine,
      encode_special(kLhsIndex, kRhsIndex, 0, 0, 0x1b),
      "DIVU by zero");

  if (machine.cpu_hi() != kDivuZeroHi || machine.cpu_lo() != kDivuZeroLo) {
    throw std::runtime_error("HI/LO demo DIVU by zero changed HI/LO");
  }

  std::cout << "after step sequence:\n";
  print_control_flow_state(machine);
  print_hex32("  hi", machine.cpu_hi());
  print_hex32("  lo", machine.cpu_lo());
  print_hex64("  gpr[0]", machine.read_cpu_gpr(0));
  print_hex64("  gpr[6]", machine.read_cpu_gpr(kHiReadIndex));
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kLoReadIndex));

  constexpr std::uint32_t kSteppedInstructionCount = 12u;
  constexpr std::uint32_t kExpectedFinalPc = kPc + (kSteppedInstructionCount * 4u);
  constexpr std::uint32_t kExpectedFinalNextPc = kExpectedFinalPc + 4u;

  if (machine.cpu_pc() != kExpectedFinalPc) {
    throw std::runtime_error("HI/LO demo did not advance pc through the step sequence");
  }

  if (machine.cpu_next_pc() != kExpectedFinalNextPc) {
    throw std::runtime_error("HI/LO demo did not advance next_pc through the step sequence");
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
  run_hilo_arithmetic_demo(machine);
}

}  // namespace fn64::bootstrap_detail
