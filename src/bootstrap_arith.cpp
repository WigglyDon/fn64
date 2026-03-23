#include "bootstrap_common.hpp"

#include <exception>
#include <iostream>
#include <stdexcept>

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
      0x0001u);
  constexpr std::uint32_t kSltiInstruction = encode_slti(
      static_cast<std::uint8_t>(kSltiResultIndex),
      static_cast<std::uint8_t>(kSltiSourceIndex),
      0x0001u);
  constexpr std::uint32_t kSltiuInstruction = encode_sltiu(
      static_cast<std::uint8_t>(kSltiuResultIndex),
      static_cast<std::uint8_t>(kSltiuSourceIndex),
      0xffffu);
  constexpr std::uint32_t kBreakInstruction = encode_break();

  machine.write_cpu_pc(kAddiAddress);
  machine.write_cpu_gpr(kAddiSourceIndex, 0x00000001u);
  machine.write_cpu_gpr(kAddiResultIndex, 0u);
  machine.write_cpu_gpr(kAddiuSourceIndex, 0xffffffffu);
  machine.write_cpu_gpr(kAddiuResultIndex, 0u);
  machine.write_cpu_gpr(kSltiSourceIndex, 0xffffffffu);
  machine.write_cpu_gpr(kSltiResultIndex, 0u);
  machine.write_cpu_gpr(kSltiuSourceIndex, 0u);
  machine.write_cpu_gpr(kSltiuResultIndex, 0u);

  machine.write_rdram_u32_be(kAddiAddress, kAddiInstruction);
  machine.write_rdram_u32_be(kAddiuAddress, kAddiuInstruction);
  machine.write_rdram_u32_be(kSltiAddress, kSltiInstruction);
  machine.write_rdram_u32_be(kSltiuAddress, kSltiuInstruction);
  machine.write_rdram_u32_be(kBreakAddress, kBreakInstruction);

  std::cout << "fn64 bootstrap reg-immediate arithmetic/compare demo: explicit ADDI/ADDIU/SLTI/SLTIU proof\n";
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

  const std::uint32_t addi_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord addi_decoded =
      Machine::decode_cpu_instruction_word(addi_raw);
  const Machine::CpuInstructionIdentity addi_identity =
      Machine::identify_cpu_instruction(addi_decoded);

  print_hex32("  addi_raw", addi_raw);
  std::cout << "  addi_identity = "
            << Machine::cpu_instruction_identity_name(addi_identity) << '\n';

  if (addi_identity != Machine::CpuInstructionIdentity::kAddi) {
    throw std::runtime_error("reg-immediate demo did not identify ADDI explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_addi");

  std::cout << "after step 1:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kAddiResultIndex));

  if (machine.cpu_pc() != kAddiuAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from ADDI to ADDIU");
  }

  if (machine.read_cpu_gpr(kAddiResultIndex) != 0u) {
    throw std::runtime_error("reg-immediate demo ADDI signed immediate result was wrong");
  }

  const std::uint32_t addiu_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord addiu_decoded =
      Machine::decode_cpu_instruction_word(addiu_raw);
  const Machine::CpuInstructionIdentity addiu_identity =
      Machine::identify_cpu_instruction(addiu_decoded);

  print_hex32("  addiu_raw", addiu_raw);
  std::cout << "  addiu_identity = "
            << Machine::cpu_instruction_identity_name(addiu_identity) << '\n';

  if (addiu_identity != Machine::CpuInstructionIdentity::kAddiu) {
    throw std::runtime_error("reg-immediate demo did not identify ADDIU explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_addiu");

  std::cout << "after step 2:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[7]", machine.read_cpu_gpr(kAddiuResultIndex));

  if (machine.cpu_pc() != kSltiAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from ADDIU to SLTI");
  }

  if (machine.read_cpu_gpr(kAddiuResultIndex) != 0u) {
    throw std::runtime_error("reg-immediate demo ADDIU wrapping result was wrong");
  }

  const std::uint32_t slti_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord slti_decoded =
      Machine::decode_cpu_instruction_word(slti_raw);
  const Machine::CpuInstructionIdentity slti_identity =
      Machine::identify_cpu_instruction(slti_decoded);

  print_hex32("  slti_raw", slti_raw);
  std::cout << "  slti_identity = "
            << Machine::cpu_instruction_identity_name(slti_identity) << '\n';

  if (slti_identity != Machine::CpuInstructionIdentity::kSlti) {
    throw std::runtime_error("reg-immediate demo did not identify SLTI explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_slti");

  std::cout << "after step 3:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[9]", machine.read_cpu_gpr(kSltiResultIndex));

  if (machine.cpu_pc() != kSltiuAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from SLTI to SLTIU");
  }

  if (machine.read_cpu_gpr(kSltiResultIndex) != 1u) {
    throw std::runtime_error("reg-immediate demo SLTI signed compare result was wrong");
  }

  const std::uint32_t sltiu_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord sltiu_decoded =
      Machine::decode_cpu_instruction_word(sltiu_raw);
  const Machine::CpuInstructionIdentity sltiu_identity =
      Machine::identify_cpu_instruction(sltiu_decoded);

  print_hex32("  sltiu_raw", sltiu_raw);
  std::cout << "  sltiu_identity = "
            << Machine::cpu_instruction_identity_name(sltiu_identity) << '\n';

  if (sltiu_identity != Machine::CpuInstructionIdentity::kSltiu) {
    throw std::runtime_error("reg-immediate demo did not identify SLTIU explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "reg_immediate_demo_sltiu");

  std::cout << "after step 4:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[11]", machine.read_cpu_gpr(kSltiuResultIndex));

  if (machine.cpu_pc() != kBreakAddress) {
    throw std::runtime_error("reg-immediate demo did not advance from SLTIU to BREAK");
  }

  if (machine.read_cpu_gpr(kSltiuResultIndex) != 1u) {
    throw std::runtime_error("reg-immediate demo SLTIU unsigned compare result was wrong");
  }

  require_stopped(machine.step_cpu_instruction(), "reg_immediate_demo_break");

  std::cout << "after stop:\n";
  print_control_flow_state(machine);

  if (machine.cpu_pc() != kAfterBreakAddress) {
    throw std::runtime_error("reg-immediate demo did not advance past executed BREAK");
  }
}

void run_addi_overflow_demo(Machine& machine) {
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

  std::cout << "fn64 bootstrap reg-immediate arithmetic demo: ADDI overflow fails loudly\n";
  std::cout << "before failing step:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[4]", machine.read_cpu_gpr(kSourceIndex));
  print_hex64("  gpr[5]", machine.read_cpu_gpr(kResultIndex));

  const std::uint32_t addi_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord addi_decoded =
      Machine::decode_cpu_instruction_word(addi_raw);
  const Machine::CpuInstructionIdentity addi_identity =
      Machine::identify_cpu_instruction(addi_decoded);

  print_hex32("  addi_raw", addi_raw);
  std::cout << "  addi_identity = "
            << Machine::cpu_instruction_identity_name(addi_identity) << '\n';

  if (addi_identity != Machine::CpuInstructionIdentity::kAddi) {
    throw std::runtime_error("addi overflow demo did not identify ADDI explicitly");
  }

  try {
    static_cast<void>(machine.step_cpu_instruction());
  } catch (const std::exception& exception) {
    std::cout << "  exception = " << exception.what() << '\n';
    std::cout << "after failing step:\n";
    print_control_flow_state(machine);
    print_hex64("  gpr[5]", machine.read_cpu_gpr(kResultIndex));

    if (machine.cpu_pc() != kAddiAddress) {
      throw std::runtime_error("addi overflow demo did not restore pc after throw");
    }

    if (machine.cpu_next_pc() != kAfterAddiAddress) {
      throw std::runtime_error("addi overflow demo did not restore next_pc after throw");
    }

    if (machine.read_cpu_gpr(kResultIndex) != 0x2468ace0u) {
      throw std::runtime_error("addi overflow demo unexpectedly changed the destination register");
    }

    return;
  }

  throw std::runtime_error("addi overflow demo did not fail loudly");
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

  const std::uint32_t lui_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord lui_decoded =
      Machine::decode_cpu_instruction_word(lui_raw);
  const Machine::CpuInstructionIdentity lui_identity =
      Machine::identify_cpu_instruction(lui_decoded);

  print_hex32("  lui_raw", lui_raw);
  std::cout << "  lui_identity = "
            << Machine::cpu_instruction_identity_name(lui_identity) << '\n';

  if (lui_identity != Machine::CpuInstructionIdentity::kLui) {
    throw std::runtime_error("logic/immediate demo did not identify LUI explicitly");
  }

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

  const std::uint32_t ori_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord ori_decoded =
      Machine::decode_cpu_instruction_word(ori_raw);
  const Machine::CpuInstructionIdentity ori_identity =
      Machine::identify_cpu_instruction(ori_decoded);

  print_hex32("  ori_raw", ori_raw);
  std::cout << "  ori_identity = "
            << Machine::cpu_instruction_identity_name(ori_identity) << '\n';

  if (ori_identity != Machine::CpuInstructionIdentity::kOri) {
    throw std::runtime_error("logic/immediate demo did not identify ORI explicitly");
  }

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

  const std::uint32_t andi_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord andi_decoded =
      Machine::decode_cpu_instruction_word(andi_raw);
  const Machine::CpuInstructionIdentity andi_identity =
      Machine::identify_cpu_instruction(andi_decoded);

  print_hex32("  andi_raw", andi_raw);
  std::cout << "  andi_identity = "
            << Machine::cpu_instruction_identity_name(andi_identity) << '\n';

  if (andi_identity != Machine::CpuInstructionIdentity::kAndi) {
    throw std::runtime_error("logic/immediate demo did not identify ANDI explicitly");
  }

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

  const std::uint32_t xori_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord xori_decoded =
      Machine::decode_cpu_instruction_word(xori_raw);
  const Machine::CpuInstructionIdentity xori_identity =
      Machine::identify_cpu_instruction(xori_decoded);

  print_hex32("  xori_raw", xori_raw);
  std::cout << "  xori_identity = "
            << Machine::cpu_instruction_identity_name(xori_identity) << '\n';

  if (xori_identity != Machine::CpuInstructionIdentity::kXori) {
    throw std::runtime_error("logic/immediate demo did not identify XORI explicitly");
  }

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

  const std::uint32_t max_lui_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord max_lui_decoded =
      Machine::decode_cpu_instruction_word(max_lui_raw);
  const Machine::CpuInstructionIdentity max_lui_identity =
      Machine::identify_cpu_instruction(max_lui_decoded);

  print_hex32("  max_lui_raw", max_lui_raw);
  std::cout << "  max_lui_identity = "
            << Machine::cpu_instruction_identity_name(max_lui_identity) << '\n';

  if (max_lui_identity != Machine::CpuInstructionIdentity::kLui) {
    throw std::runtime_error("logic/immediate demo did not identify second LUI explicitly");
  }

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

  const std::uint32_t max_ori_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord max_ori_decoded =
      Machine::decode_cpu_instruction_word(max_ori_raw);
  const Machine::CpuInstructionIdentity max_ori_identity =
      Machine::identify_cpu_instruction(max_ori_decoded);

  print_hex32("  max_ori_raw", max_ori_raw);
  std::cout << "  max_ori_identity = "
            << Machine::cpu_instruction_identity_name(max_ori_identity) << '\n';

  if (max_ori_identity != Machine::CpuInstructionIdentity::kOri) {
    throw std::runtime_error("logic/immediate demo did not identify second ORI explicitly");
  }

  require_stepped(machine.step_cpu_instruction(), "logic_immediate_demo_max_ori");

  std::cout << "after step 6:\n";
  print_control_flow_state(machine);
  print_hex64("  gpr[10]", machine.read_cpu_gpr(kMaxIndex));

  if (machine.cpu_pc() != kOneOriAddress) {
    throw std::runtime_error("logic/immediate demo did not advance from second ORI to one-building ORI");
  }

  if (machine.read_cpu_gpr(kMaxIndex) != 0xffffffffu) {
    throw std::runtime_error("logic/immediate demo second ORI result was wrong");
  }

  const std::uint32_t one_ori_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord one_ori_decoded =
      Machine::decode_cpu_instruction_word(one_ori_raw);
  const Machine::CpuInstructionIdentity one_ori_identity =
      Machine::identify_cpu_instruction(one_ori_decoded);

  print_hex32("  one_ori_raw", one_ori_raw);
  std::cout << "  one_ori_identity = "
            << Machine::cpu_instruction_identity_name(one_ori_identity) << '\n';

  if (one_ori_identity != Machine::CpuInstructionIdentity::kOri) {
    throw std::runtime_error("logic/immediate demo did not identify one-building ORI explicitly");
  }

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

  const std::uint32_t sltu_raw = machine.fetch_cpu_instruction_word();
  const Machine::DecodedCpuInstructionWord sltu_decoded =
      Machine::decode_cpu_instruction_word(sltu_raw);
  const Machine::CpuInstructionIdentity sltu_identity =
      Machine::identify_cpu_instruction(sltu_decoded);

  print_hex32("  sltu_raw", sltu_raw);
  std::cout << "  sltu_identity = "
            << Machine::cpu_instruction_identity_name(sltu_identity) << '\n';

  if (sltu_identity != Machine::CpuInstructionIdentity::kSpecialSltu) {
    throw std::runtime_error("logic/immediate demo did not identify SLTU explicitly");
  }

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

}  // namespace

void run_arithmetic_demos(Machine& machine) {
  run_register_immediate_arithmetic_compare_demo(machine);
  run_addi_overflow_demo(machine);
  run_logic_immediate_unsigned_compare_demo(machine);
}

}  // namespace fn64::bootstrap_detail