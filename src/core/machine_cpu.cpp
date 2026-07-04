#include "machine.hpp"

#include <limits>
#include <stdexcept>
#include <string>

namespace fn64 {
namespace {

[[noreturn]] void fail_cpu_direct_rdram_address(
    const char* operation,
    CpuAddress cpu_address,
    std::size_t width,
    MachineFaultAccessIntent access_intent = MachineFaultAccessIntent::kNone) {
  throw MachineFault(
      MachineFaultKind::kCpuRdramAddressRejected,
      operation,
      cpu_address,
      width,
      std::string("RDRAM access out of range through CPU address: operation=") +
          operation +
          " address=" + std::to_string(cpu_address) +
          " width=" + std::to_string(width),
      access_intent);
}

[[noreturn]] void fail_cpu_data_address_rejected(
    const char* operation,
    CpuAddress cpu_address,
    std::size_t width,
    MachineFaultAccessIntent access_intent) {
  throw MachineFault(
      MachineFaultKind::kCpuRdramAddressRejected,
      operation,
      cpu_address,
      width,
      std::string("CPU data address has no supported local target: operation=") +
          operation +
          " address=" + std::to_string(cpu_address) +
          " width=" + std::to_string(width),
      access_intent);
}

[[noreturn]] void fail_unsupported_cpu_data_access(
    const char* operation,
    CpuAddress cpu_address,
    std::size_t width) {
  throw MachineFault(
      MachineFaultKind::kUnsupportedCpuDataAccess,
      operation,
      cpu_address,
      width,
      std::string("Unsupported CPU data access: operation=") +
          operation +
          " address=" + std::to_string(cpu_address) +
          " width=" + std::to_string(width));
}

[[noreturn]] void fail_pi_dma_length_overflow(std::uint32_t length_register_value) {
  throw std::out_of_range(
      "PI cartridge-to-RDRAM DMA length overflows 32-bit byte count: register=" +
      std::to_string(length_register_value));
}

[[noreturn]] void fail_sp_dma_sp_memory_span_out_of_range(
    const char* operation,
    std::uint32_t sp_memory_address,
    std::uint32_t byte_count) {
  throw std::out_of_range(
      std::string(operation) +
      " SP memory span out of range: address=" +
      std::to_string(sp_memory_address) +
      " byte_count=" + std::to_string(byte_count));
}

[[noreturn]] void fail_sp_dma_rdram_span_out_of_range(
    const char* operation,
    RdramOffset rdram_address,
    std::uint32_t byte_count) {
  throw std::out_of_range(
      std::string(operation) +
      " RDRAM span out of range: address=" +
      std::to_string(rdram_address) +
      " byte_count=" + std::to_string(byte_count));
}

[[noreturn]] void fail_si_pif_ram_address_unsupported(
    const char* operation,
    std::uint32_t pif_address) {
  throw std::out_of_range(
      std::string(operation) +
      " PIF RAM address is outside the supported local 64-byte window: address=" +
      std::to_string(pif_address));
}

[[noreturn]] void fail_si_dma_rdram_span_out_of_range(
    const char* operation,
    RdramOffset rdram_address,
    std::uint32_t byte_count) {
  throw std::out_of_range(
      std::string(operation) +
      " RDRAM span out of range: address=" +
      std::to_string(rdram_address) +
      " byte_count=" + std::to_string(byte_count));
}

[[noreturn]] void fail_pi_cart_rom_source_below_base(
    PiCartAddress pi_cart_address,
    PiCartAddress base_address) {
  throw std::out_of_range(
      "PI cart ROM source address is below supported local ROM base: address=" +
      std::to_string(pi_cart_address) +
      " base=" + std::to_string(base_address));
}

[[noreturn]] void fail_pi_cart_rom_source_span_overflow(
    PiCartAddress pi_cart_address,
    std::uint32_t byte_count) {
  throw std::out_of_range(
      "PI cart ROM source span overflows 32-bit address space: address=" +
      std::to_string(pi_cart_address) +
      " byte_count=" + std::to_string(byte_count));
}

[[noreturn]] void fail_pi_cart_rom_source_out_of_range(
    PiCartAddress pi_cart_address,
    CartridgeOffset cartridge_offset,
    std::uint32_t byte_count,
    std::size_t cartridge_size) {
  throw std::out_of_range(
      "PI cart ROM source span out of range: address=" +
      std::to_string(pi_cart_address) +
      " cartridge_offset=" + std::to_string(cartridge_offset) +
      " byte_count=" + std::to_string(byte_count) +
      " cartridge_size=" + std::to_string(cartridge_size));
}

[[noreturn]] void fail_cpu_gpr_index(std::size_t index) {
  throw std::out_of_range("CPU GPR index out of range: " + std::to_string(index));
}

[[noreturn]] void fail_unaligned_instruction_fetch(CpuAddress pc) {
  throw MachineFault(
      MachineFaultKind::kUnalignedInstructionFetch,
      "CPU instruction fetch",
      pc,
      4,
      "Unaligned CPU instruction fetch at PC " + std::to_string(pc),
      MachineFaultAccessIntent::kInstructionFetch);
}

[[noreturn]] void fail_unaligned_halfword_memory_access(
    const char* operation,
    CpuAddress address,
    MachineFaultAccessIntent access_intent) {
  throw MachineFault(
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      operation,
      address,
      2,
      std::string(operation) +
          " requires naturally aligned halfword address: " +
          std::to_string(address),
      access_intent);
}

[[noreturn]] void fail_unaligned_word_memory_access(
    const char* operation,
    CpuAddress address,
    MachineFaultAccessIntent access_intent) {
  throw MachineFault(
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      operation,
      address,
      4,
      std::string(operation) +
          " requires naturally aligned word address: " +
          std::to_string(address),
      access_intent);
}

[[noreturn]] void fail_unaligned_doubleword_memory_access(
    const char* operation,
    CpuAddress address,
    MachineFaultAccessIntent access_intent) {
  throw MachineFault(
      MachineFaultKind::kUnalignedCpuMemoryAccess,
      operation,
      address,
      8,
      std::string(operation) +
          " requires naturally aligned doubleword address: " +
          std::to_string(address),
      access_intent);
}

[[noreturn]] void fail_unaligned_control_transfer_target(
    const char* operation,
    CpuAddress address) {
  throw MachineFault(
      MachineFaultKind::kUnalignedControlTransferTarget,
      operation,
      address,
      4,
      std::string(operation) +
          " requires naturally aligned control-transfer target: " +
          std::to_string(address));
}

[[noreturn]] void fail_signed_arithmetic_overflow(const char* operation) {
  throw MachineFault(
      MachineFaultKind::kSignedArithmeticOverflow,
      operation,
      0,
      0,
      std::string(operation) + " overflow");
}

bool signed_cpu_add_overflows(
    CpuRegisterValue lhs,
    CpuRegisterValue rhs,
    CpuRegisterValue result) {
  constexpr CpuRegisterValue kSignBit = 0x8000000000000000ull;
  return ((~(lhs ^ rhs) & (lhs ^ result)) & kSignBit) != 0;
}

bool signed_cpu_sub_overflows(
    CpuRegisterValue lhs,
    CpuRegisterValue rhs,
    CpuRegisterValue result) {
  constexpr CpuRegisterValue kSignBit = 0x8000000000000000ull;
  return (((lhs ^ rhs) & (lhs ^ result)) & kSignBit) != 0;
}

CpuRegisterValue checked_signed_cpu_add(
    const char* operation,
    CpuRegisterValue lhs,
    CpuRegisterValue rhs) {
  const CpuRegisterValue result = lhs + rhs;
  if (signed_cpu_add_overflows(lhs, rhs, result)) {
    fail_signed_arithmetic_overflow(operation);
  }

  return result;
}

CpuRegisterValue checked_signed_cpu_sub(
    const char* operation,
    CpuRegisterValue lhs,
    CpuRegisterValue rhs) {
  const CpuRegisterValue result = lhs - rhs;
  if (signed_cpu_sub_overflows(lhs, rhs, result)) {
    fail_signed_arithmetic_overflow(operation);
  }

  return result;
}

std::uint8_t variable_shift_amount_u32(std::uint32_t value) {
  return static_cast<std::uint8_t>(value & 0x1fu);
}

std::uint8_t variable_shift_amount_cpu_value(CpuRegisterValue value) {
  return static_cast<std::uint8_t>(value & 0x3full);
}

std::uint32_t arithmetic_shift_right_u32(std::uint32_t value, std::uint8_t sa) {
  if (sa == 0) {
    return value;
  }

  const std::uint32_t shifted = value >> sa;
  if ((value & 0x80000000u) == 0) {
    return shifted;
  }

  const std::uint32_t fill_mask = 0xffffffffu << (32 - sa);
  return shifted | fill_mask;
}

CpuRegisterValue arithmetic_shift_right_cpu_value(
    CpuRegisterValue value,
    std::uint8_t sa) {
  if (sa == 0) {
    return value;
  }

  constexpr CpuRegisterValue kSignBit = 0x8000000000000000ull;
  const CpuRegisterValue shifted = value >> sa;
  if ((value & kSignBit) == 0) {
    return shifted;
  }

  const CpuRegisterValue fill_mask =
      static_cast<CpuRegisterValue>(0xffffffffffffffffull) << (64u - sa);
  return shifted | fill_mask;
}

std::int32_t i32_from_u32_bits(std::uint32_t value) {
  if ((value & 0x80000000u) == 0) {
    return static_cast<std::int32_t>(value);
  }

  const std::int64_t signed_value =
      static_cast<std::int64_t>(value) - 0x100000000ll;
  return static_cast<std::int32_t>(signed_value);
}

bool signed_i32_result_out_of_range(std::int64_t value) {
  return value < static_cast<std::int64_t>(std::numeric_limits<std::int32_t>::min()) ||
         value > static_cast<std::int64_t>(std::numeric_limits<std::int32_t>::max());
}

std::uint32_t u32_bits_from_i32_value(std::int64_t value) {
  if (value < 0) {
    return static_cast<std::uint32_t>(0x100000000ll + value);
  }

  return static_cast<std::uint32_t>(value);
}

std::int16_t i16_from_u16_bits(std::uint16_t value) {
  if ((value & 0x8000u) == 0) {
    return static_cast<std::int16_t>(value);
  }

  const std::int32_t signed_value =
      static_cast<std::int32_t>(value) - 0x10000;
  return static_cast<std::int16_t>(signed_value);
}

std::int64_t i64_from_cpu_value_bits(CpuRegisterValue value) {
  constexpr CpuRegisterValue kSignBit = 0x8000000000000000ull;
  if ((value & kSignBit) == 0) {
    return static_cast<std::int64_t>(value);
  }

  if (value == kSignBit) {
    return std::numeric_limits<std::int64_t>::min();
  }

  const CpuRegisterValue magnitude = (~value) + 1ull;
  return -static_cast<std::int64_t>(magnitude);
}

CpuRegisterValue sign_extend_u16_to_cpu_value(std::uint16_t value) {
  if ((value & 0x8000u) == 0) {
    return static_cast<CpuRegisterValue>(value);
  }

  return static_cast<CpuRegisterValue>(0xffffffffffff0000ull) |
         static_cast<CpuRegisterValue>(value);
}

CpuRegisterValue cpu_value_from_bool(bool value) {
  return value ? static_cast<CpuRegisterValue>(1) : static_cast<CpuRegisterValue>(0);
}

bool cpu_values_equal(CpuRegisterValue lhs, CpuRegisterValue rhs) {
  return lhs == rhs;
}

bool signed_cpu_value_greater_equal(CpuRegisterValue lhs, CpuRegisterValue rhs) {
  return i64_from_cpu_value_bits(lhs) >= i64_from_cpu_value_bits(rhs);
}

bool signed_cpu_value_less_than(CpuRegisterValue lhs, CpuRegisterValue rhs) {
  return i64_from_cpu_value_bits(lhs) < i64_from_cpu_value_bits(rhs);
}

bool unsigned_cpu_value_greater_equal(CpuRegisterValue lhs, CpuRegisterValue rhs) {
  return lhs >= rhs;
}

bool unsigned_cpu_value_less_than(CpuRegisterValue lhs, CpuRegisterValue rhs) {
  return lhs < rhs;
}

std::uint32_t sign_extend_u8_to_u32(std::uint8_t value) {
  if ((value & 0x80u) == 0) {
    return static_cast<std::uint32_t>(value);
  }

  return 0xffffff00u | static_cast<std::uint32_t>(value);
}

std::uint32_t sign_extend_u16_to_u32(std::uint16_t value) {
  if ((value & 0x8000u) == 0) {
    return static_cast<std::uint32_t>(value);
  }

  return 0xffff0000u | static_cast<std::uint32_t>(value);
}

CpuRegisterValue sign_extend_u32_to_cpu_value(std::uint32_t value) {
  if ((value & 0x80000000u) == 0) {
    return static_cast<CpuRegisterValue>(value);
  }

  return static_cast<CpuRegisterValue>(0xffffffff00000000ull) |
         static_cast<CpuRegisterValue>(value);
}

CpuRegisterValue zero_extend_u32_to_cpu_value(std::uint32_t value) {
  return static_cast<CpuRegisterValue>(value);
}

std::uint32_t low_u32(std::uint64_t value) {
  return static_cast<std::uint32_t>(value & 0xffffffffull);
}

std::uint32_t high_u32(std::uint64_t value) {
  return static_cast<std::uint32_t>(value >> 32);
}

struct DoubleCpuRegisterValueBits {
  CpuRegisterValue hi = 0;
  CpuRegisterValue lo = 0;
};

bool cpu_value_is_negative(CpuRegisterValue value) {
  return (value & 0x8000000000000000ull) != 0;
}

CpuRegisterValue negate_cpu_value_bits(CpuRegisterValue value) {
  return (~value) + 1ull;
}

DoubleCpuRegisterValueBits negate_double_cpu_value_bits(
    DoubleCpuRegisterValueBits value) {
  const CpuRegisterValue lo = (~value.lo) + 1ull;
  const CpuRegisterValue carry = (lo == 0) ? 1ull : 0ull;
  return DoubleCpuRegisterValueBits{(~value.hi) + carry, lo};
}

DoubleCpuRegisterValueBits multiply_u64_to_u128_bits(
    CpuRegisterValue lhs,
    CpuRegisterValue rhs) {
  constexpr CpuRegisterValue kWordMask = 0xffffffffull;

  const CpuRegisterValue lhs_lo = lhs & kWordMask;
  const CpuRegisterValue lhs_hi = lhs >> 32;
  const CpuRegisterValue rhs_lo = rhs & kWordMask;
  const CpuRegisterValue rhs_hi = rhs >> 32;

  const CpuRegisterValue p0 = lhs_lo * rhs_lo;
  const CpuRegisterValue p1 = lhs_lo * rhs_hi;
  const CpuRegisterValue p2 = lhs_hi * rhs_lo;
  const CpuRegisterValue p3 = lhs_hi * rhs_hi;

  const CpuRegisterValue middle =
      (p0 >> 32) + (p1 & kWordMask) + (p2 & kWordMask);
  const CpuRegisterValue lo = (p0 & kWordMask) | (middle << 32);
  const CpuRegisterValue hi = p3 + (p1 >> 32) + (p2 >> 32) + (middle >> 32);
  return DoubleCpuRegisterValueBits{hi, lo};
}

DoubleCpuRegisterValueBits multiply_i64_to_i128_bits(
    CpuRegisterValue lhs,
    CpuRegisterValue rhs) {
  const bool result_is_negative = cpu_value_is_negative(lhs) != cpu_value_is_negative(rhs);
  const CpuRegisterValue lhs_magnitude =
      cpu_value_is_negative(lhs) ? negate_cpu_value_bits(lhs) : lhs;
  const CpuRegisterValue rhs_magnitude =
      cpu_value_is_negative(rhs) ? negate_cpu_value_bits(rhs) : rhs;

  DoubleCpuRegisterValueBits product =
      multiply_u64_to_u128_bits(lhs_magnitude, rhs_magnitude);
  if (result_is_negative) {
    product = negate_double_cpu_value_bits(product);
  }

  return product;
}

bool signed_cpu_division_overflows(CpuRegisterValue dividend, CpuRegisterValue divisor) {
  return dividend == 0x8000000000000000ull && divisor == 0xffffffffffffffffull;
}

CpuAddress sequential_instruction_address(CpuAddress address) {
  return address + 4u;
}

CpuAddress link_return_address(CpuAddress current_pc) {
  return sequential_instruction_address(sequential_instruction_address(current_pc));
}

CpuAddress jump_target_address(CpuAddress current_pc, std::uint32_t jump_target) {
  const CpuAddress next_sequential = sequential_instruction_address(current_pc);
  return (next_sequential & 0xf0000000u) | ((jump_target & 0x03ffffffu) << 2);
}

CpuAddress branch_target_address(CpuAddress current_pc, std::int16_t immediate) {
  const std::int32_t offset_bytes = static_cast<std::int32_t>(immediate) * 4;
  return sequential_instruction_address(current_pc) +
         static_cast<std::uint32_t>(offset_bytes);
}

void validate_control_transfer_target_alignment(
    const char* operation,
    CpuAddress address) {
  if ((address & 0x3u) != 0) {
    fail_unaligned_control_transfer_target(operation, address);
  }
}

std::uint8_t u32_byte_be(std::uint32_t value, std::size_t byte_index) {
  const std::uint32_t shift = static_cast<std::uint32_t>((3u - byte_index) * 8u);
  return static_cast<std::uint8_t>((value >> shift) & 0xffu);
}

std::uint8_t cpu_value_byte_be(CpuRegisterValue value, std::size_t byte_index) {
  const std::size_t shift = (7u - byte_index) * 8u;
  return static_cast<std::uint8_t>((value >> shift) & 0xffu);
}

std::uint32_t replace_u32_byte_be(
    std::uint32_t value,
    std::size_t byte_index,
    std::uint8_t byte_value) {
  const std::uint32_t shift = static_cast<std::uint32_t>((3u - byte_index) * 8u);
  const std::uint32_t clear_mask = ~(0xffu << shift);
  return (value & clear_mask) |
         (static_cast<std::uint32_t>(byte_value) << shift);
}

CpuRegisterValue replace_cpu_value_byte_be(
    CpuRegisterValue value,
    std::size_t byte_index,
    std::uint8_t byte_value) {
  const std::size_t shift = (7u - byte_index) * 8u;
  const CpuRegisterValue clear_mask =
      ~(static_cast<CpuRegisterValue>(0xffu) << shift);
  return (value & clear_mask) |
         (static_cast<CpuRegisterValue>(byte_value) << shift);
}

}  // namespace

RdramOffset Machine::require_cpu_rdram_address(
    const char* operation,
    CpuAddress cpu_address,
    std::size_t width) {
  RdramOffset rdram_address = 0;
  if (!translate_cpu_rdram_address(cpu_address, width, rdram_address)) {
    fail_cpu_direct_rdram_address(operation, cpu_address, width);
  }

  return rdram_address;
}

Machine::CpuDataTarget Machine::require_cpu_data_target(
    const char* operation,
    CpuAddress cpu_address,
    std::size_t width,
    MachineFaultAccessIntent access_intent) {
  CpuPhysicalAddress physical_address = 0;
  if (!translate_direct_cpu_physical_address(cpu_address, physical_address)) {
    fail_cpu_data_address_rejected(operation, cpu_address, width, access_intent);
  }

  RdramOffset rdram_address = 0;
  if (translate_cpu_physical_rdram_address(physical_address, width, rdram_address)) {
    return CpuDataTarget{
        CpuDataTargetKind::kRdram,
        physical_address,
        rdram_address,
        0,
        0,
        0,
        0,
        0,
    };
  }

  CpuDataTargetKind sp_kind = CpuDataTargetKind::kSpDmem;
  std::uint32_t sp_memory_offset = 0;
  if (translate_cpu_physical_sp_memory_address(physical_address, width, sp_kind, sp_memory_offset)) {
    return CpuDataTarget{
        sp_kind,
        physical_address,
        0,
        sp_memory_offset,
        0,
        0,
        0,
        0,
    };
  }

  std::uint32_t sp_register_offset = 0;
  if (translate_cpu_physical_sp_register_address(
          physical_address,
          sp_register_offset)) {
    return CpuDataTarget{
        CpuDataTargetKind::kSpMmio,
        physical_address,
        0,
        0,
        sp_register_offset,
        0,
        0,
        0,
    };
  }

  std::uint32_t mi_register_offset = 0;
  if (translate_cpu_physical_mi_register_address(physical_address, mi_register_offset)) {
    return CpuDataTarget{
        CpuDataTargetKind::kMi,
        physical_address,
        0,
        0,
        0,
        mi_register_offset,
        0,
        0,
    };
  }

  std::uint32_t pi_register_offset = 0;
  if (translate_cpu_physical_pi_register_address(physical_address, pi_register_offset)) {
    return CpuDataTarget{
        CpuDataTargetKind::kPi,
        physical_address,
        0,
        0,
        0,
        0,
        pi_register_offset,
        0,
    };
  }

  std::uint32_t si_register_offset = 0;
  if (translate_cpu_physical_si_register_address(physical_address, si_register_offset)) {
    return CpuDataTarget{
        CpuDataTargetKind::kSi,
        physical_address,
        0,
        0,
        0,
        0,
        0,
        si_register_offset,
    };
  }

  fail_cpu_data_address_rejected(operation, cpu_address, width, access_intent);
}

const std::array<std::uint8_t, Machine::kSpMemorySizeBytes>& Machine::sp_memory_for_kind(
    CpuDataTargetKind kind) const {
  switch (kind) {
    case CpuDataTargetKind::kSpDmem:
      return sp_dmem_;

    case CpuDataTargetKind::kSpImem:
      return sp_imem_;

    case CpuDataTargetKind::kRdram:
    case CpuDataTargetKind::kSpMmio:
    case CpuDataTargetKind::kMi:
    case CpuDataTargetKind::kPi:
    case CpuDataTargetKind::kSi:
      break;
  }

  throw std::logic_error("CPU data target is not SP memory");
}

std::array<std::uint8_t, Machine::kSpMemorySizeBytes>& Machine::sp_memory_for_kind(
    CpuDataTargetKind kind) {
  switch (kind) {
    case CpuDataTargetKind::kSpDmem:
      return sp_dmem_;

    case CpuDataTargetKind::kSpImem:
      return sp_imem_;

    case CpuDataTargetKind::kRdram:
    case CpuDataTargetKind::kSpMmio:
    case CpuDataTargetKind::kMi:
    case CpuDataTargetKind::kPi:
    case CpuDataTargetKind::kSi:
      break;
  }

  throw std::logic_error("CPU data target is not SP memory");
}

std::uint8_t Machine::read_sp_memory_u8(
    CpuDataTargetKind kind,
    std::uint32_t offset) const {
  return sp_memory_for_kind(kind)[offset];
}

std::uint16_t Machine::read_sp_memory_u16_be(
    CpuDataTargetKind kind,
    std::uint32_t offset) const {
  return static_cast<std::uint16_t>(
      (static_cast<std::uint16_t>(read_sp_memory_u8(kind, offset)) << 8) |
      static_cast<std::uint16_t>(read_sp_memory_u8(kind, offset + 1u)));
}

std::uint32_t Machine::read_sp_memory_u32_be(
    CpuDataTargetKind kind,
    std::uint32_t offset) const {
  return (static_cast<std::uint32_t>(read_sp_memory_u8(kind, offset)) << 24) |
         (static_cast<std::uint32_t>(read_sp_memory_u8(kind, offset + 1u)) << 16) |
         (static_cast<std::uint32_t>(read_sp_memory_u8(kind, offset + 2u)) << 8) |
         static_cast<std::uint32_t>(read_sp_memory_u8(kind, offset + 3u));
}

CpuRegisterValue Machine::read_sp_memory_u64_be(
    CpuDataTargetKind kind,
    std::uint32_t offset) const {
  return (static_cast<CpuRegisterValue>(read_sp_memory_u32_be(kind, offset)) << 32) |
         static_cast<CpuRegisterValue>(read_sp_memory_u32_be(kind, offset + 4u));
}

void Machine::write_sp_memory_u8(
    CpuDataTargetKind kind,
    std::uint32_t offset,
    std::uint8_t value) {
  sp_memory_for_kind(kind)[offset] = value;
}

void Machine::write_sp_memory_u16_be(
    CpuDataTargetKind kind,
    std::uint32_t offset,
    std::uint16_t value) {
  write_sp_memory_u8(kind, offset, static_cast<std::uint8_t>((value >> 8) & 0xffu));
  write_sp_memory_u8(kind, offset + 1u, static_cast<std::uint8_t>(value & 0xffu));
}

void Machine::write_sp_memory_u32_be(
    CpuDataTargetKind kind,
    std::uint32_t offset,
    std::uint32_t value) {
  write_sp_memory_u8(kind, offset, static_cast<std::uint8_t>((value >> 24) & 0xffu));
  write_sp_memory_u8(kind, offset + 1u, static_cast<std::uint8_t>((value >> 16) & 0xffu));
  write_sp_memory_u8(kind, offset + 2u, static_cast<std::uint8_t>((value >> 8) & 0xffu));
  write_sp_memory_u8(kind, offset + 3u, static_cast<std::uint8_t>(value & 0xffu));
}

void Machine::write_sp_memory_u64_be(
    CpuDataTargetKind kind,
    std::uint32_t offset,
    CpuRegisterValue value) {
  write_sp_memory_u32_be(
      kind,
      offset,
      static_cast<std::uint32_t>((value >> 32) & 0xffffffffull));
  write_sp_memory_u32_be(
      kind,
      offset + 4u,
      static_cast<std::uint32_t>(value & 0xffffffffull));
}

std::uint32_t Machine::read_sp_register_u32(
    CpuPhysicalAddress physical_address,
    CpuAddress cpu_address) const {
  const std::uint32_t register_offset = physical_address - kSpRegisterPhysicalBase;
  switch (register_offset) {
    case kSpMemoryAddressRegisterOffset:
      return sp_mem_address_;

    case kSpDramAddressRegisterOffset:
      return sp_dram_address_;

    case kSpReadLengthRegisterOffset:
      return sp_rd_len_;

    case kSpWriteLengthRegisterOffset:
      return sp_wr_len_;

    case kSpStatusRegisterOffset:
      return sp_status_;

    default:
      fail_unsupported_cpu_data_access("SP word read", cpu_address, 4);
  }
}

void Machine::write_sp_register_u32(
    CpuPhysicalAddress physical_address,
    CpuAddress cpu_address,
    std::uint32_t value) {
  const std::uint32_t register_offset = physical_address - kSpRegisterPhysicalBase;
  switch (register_offset) {
    case kSpMemoryAddressRegisterOffset:
      sp_mem_address_ = value;
      return;

    case kSpDramAddressRegisterOffset:
      sp_dram_address_ = static_cast<RdramOffset>(value);
      return;

    case kSpReadLengthRegisterOffset:
      perform_sp_read_dma(value);
      latch_mi_interrupt_pending(kMiInterruptPendingSp);
      sp_rd_len_ = value;
      sp_status_ = 0;
      return;

    case kSpWriteLengthRegisterOffset:
      perform_sp_write_dma(value);
      latch_mi_interrupt_pending(kMiInterruptPendingSp);
      sp_wr_len_ = value;
      sp_status_ = 0;
      return;

    case kSpStatusRegisterOffset:
      // Local immediate-complete SP subset: status remains idle/no-error.
      sp_status_ = 0;
      return;

    default:
      fail_unsupported_cpu_data_access("SP word write", cpu_address, 4);
  }
}

Machine::SpDmaLengthCommand Machine::decode_sp_dma_length_command(
    std::uint32_t length_register_value) noexcept {
  SpDmaLengthCommand command;
  command.length = length_register_value & 0x00000fffu;
  command.count = (length_register_value >> 12) & 0x000000ffu;
  command.skip = (length_register_value >> 20) & 0x00000fffu;
  command.transfer_length_per_block = command.length + 1u;
  command.block_count = command.count + 1u;
  return command;
}

bool Machine::translate_sp_memory_dma_base(
    std::uint32_t sp_memory_address,
    CpuDataTargetKind& out_kind,
    std::uint32_t& out_sp_offset) noexcept {
  constexpr std::uint32_t kSpMemorySelectorBit = 0x1000u;
  constexpr std::uint32_t kSpMemorySupportedMask = 0x1fffu;
  constexpr std::uint32_t kSpMemoryOffsetMask = 0x0fffu;

  if ((sp_memory_address & ~kSpMemorySupportedMask) != 0) {
    return false;
  }

  const std::uint32_t offset = sp_memory_address & kSpMemoryOffsetMask;
  out_kind = ((sp_memory_address & kSpMemorySelectorBit) == 0)
                 ? CpuDataTargetKind::kSpDmem
                 : CpuDataTargetKind::kSpImem;
  out_sp_offset = offset;
  return true;
}

Machine::CpuDataTarget Machine::require_sp_memory_dma_blocks(
    const char* operation,
    std::uint32_t sp_memory_address,
    const SpDmaLengthCommand& command) {
  CpuDataTargetKind kind = CpuDataTargetKind::kSpDmem;
  std::uint32_t offset = 0;
  if (!translate_sp_memory_dma_base(sp_memory_address, kind, offset)) {
    fail_sp_dma_sp_memory_span_out_of_range(
        operation,
        sp_memory_address,
        command.transfer_length_per_block);
  }

  for (std::uint32_t block = 0; block < command.block_count; ++block) {
    const std::uint64_t block_offset =
        static_cast<std::uint64_t>(offset) +
        static_cast<std::uint64_t>(block) *
            static_cast<std::uint64_t>(command.transfer_length_per_block);
    if (block_offset > static_cast<std::uint64_t>(kSpMemorySizeBytes) ||
        static_cast<std::uint64_t>(command.transfer_length_per_block) >
            static_cast<std::uint64_t>(kSpMemorySizeBytes) - block_offset) {
      fail_sp_dma_sp_memory_span_out_of_range(
          operation,
          sp_memory_address,
          command.transfer_length_per_block);
    }
  }

  return CpuDataTarget{
      kind,
      0,
      0,
      offset,
      0,
      0,
      0,
      0,
  };
}

RdramOffset Machine::require_sp_dma_rdram_blocks(
    const char* operation,
    RdramOffset rdram_address,
    const SpDmaLengthCommand& command) {
  RdramOffset first_rdram_address = 0;
  for (std::uint32_t block = 0; block < command.block_count; ++block) {
    const std::uint64_t block_address =
        static_cast<std::uint64_t>(rdram_address) +
        static_cast<std::uint64_t>(block) *
            static_cast<std::uint64_t>(
                command.transfer_length_per_block + command.skip);
    if (block_address > 0xffffffffull) {
      fail_sp_dma_rdram_span_out_of_range(
          operation,
          rdram_address,
          command.transfer_length_per_block);
    }

    RdramOffset translated_rdram_address = 0;
    if (!translate_cpu_physical_rdram_address(
            static_cast<CpuPhysicalAddress>(block_address),
            command.transfer_length_per_block,
            translated_rdram_address)) {
      fail_sp_dma_rdram_span_out_of_range(
          operation,
          rdram_address,
          command.transfer_length_per_block);
    }

    if (block == 0) {
      first_rdram_address = translated_rdram_address;
    }
  }

  return first_rdram_address;
}

void Machine::perform_sp_read_dma(std::uint32_t length_register_value) {
  const SpDmaLengthCommand command =
      decode_sp_dma_length_command(length_register_value);
  const RdramOffset rdram_address =
      require_sp_dma_rdram_blocks("SP read DMA RDRAM source", sp_dram_address_, command);
  const CpuDataTarget sp_target =
      require_sp_memory_dma_blocks("SP read DMA memory destination", sp_mem_address_, command);

  // Local SP subset: SP read DMA decodes length/count/skip and immediately
  // copies deterministic blocks from physical RDRAM into local SP memory.
  // Timing, busy state, interrupt delivery, SP registers beyond this subset,
  // and RSP execution are not modeled here.
  for (std::uint32_t block = 0; block < command.block_count; ++block) {
    const std::uint32_t sp_block_offset =
        sp_target.sp_memory_offset + block * command.transfer_length_per_block;
    const RdramOffset rdram_block_address =
        rdram_address +
        block * (command.transfer_length_per_block + command.skip);
    for (std::uint32_t i = 0; i < command.transfer_length_per_block; ++i) {
      write_sp_memory_u8(
          sp_target.kind,
          sp_block_offset + i,
          read_rdram_u8(rdram_block_address + i));
    }
  }
}

void Machine::perform_sp_write_dma(std::uint32_t length_register_value) {
  const SpDmaLengthCommand command =
      decode_sp_dma_length_command(length_register_value);
  const CpuDataTarget sp_source =
      require_sp_memory_dma_blocks("SP write DMA memory source", sp_mem_address_, command);
  const RdramOffset rdram_address =
      require_sp_dma_rdram_blocks("SP write DMA RDRAM destination", sp_dram_address_, command);

  // Local SP subset: SP write DMA decodes length/count/skip and immediately
  // copies deterministic blocks from local SP memory into physical RDRAM.
  // RDRAM writes use the existing helpers, so overlapping local LL/SC
  // reservations are invalidated.
  for (std::uint32_t block = 0; block < command.block_count; ++block) {
    const std::uint32_t sp_block_offset =
        sp_source.sp_memory_offset + block * command.transfer_length_per_block;
    const RdramOffset rdram_block_address =
        rdram_address +
        block * (command.transfer_length_per_block + command.skip);
    for (std::uint32_t i = 0; i < command.transfer_length_per_block; ++i) {
      write_rdram_u8(
          rdram_block_address + i,
          read_sp_memory_u8(sp_source.kind, sp_block_offset + i));
    }
  }
}

std::uint32_t Machine::read_si_register_u32(
    std::uint32_t register_offset,
    CpuAddress cpu_address) const {
  switch (register_offset) {
    case kSiDramAddressRegisterOffset:
      return si_dram_address_;

    case kSiPifToDramRegisterOffset:
      return si_pif_to_dram_address_;

    case kSiDramToPifRegisterOffset:
      return si_dram_to_pif_address_;

    case kSiStatusRegisterOffset:
      return si_status_ & kSiSupportedStatusBits;

    default:
      fail_unsupported_cpu_data_access("SI word read", cpu_address, 4);
  }
}

void Machine::write_si_register_u32(
    std::uint32_t register_offset,
    CpuAddress cpu_address,
    std::uint32_t value) {
  switch (register_offset) {
    case kSiDramAddressRegisterOffset:
      si_dram_address_ = static_cast<RdramOffset>(value);
      return;

    case kSiPifToDramRegisterOffset:
      perform_si_pif_to_dram_dma(value);
      si_pif_to_dram_address_ = value;
      si_status_ |= kSiStatusInterruptPending;
      latch_mi_interrupt_pending(kMiInterruptPendingSi);
      return;

    case kSiDramToPifRegisterOffset:
      perform_si_dram_to_pif_dma(value);
      si_dram_to_pif_address_ = value;
      si_status_ |= kSiStatusInterruptPending;
      latch_mi_interrupt_pending(kMiInterruptPendingSi);
      return;

    case kSiStatusRegisterOffset:
      if ((value & kSiStatusInterruptClear) != 0) {
        si_status_ &= ~kSiStatusInterruptPending;
        clear_mi_interrupt_pending(kMiInterruptPendingSi);
      }
      return;

    default:
      fail_unsupported_cpu_data_access("SI word write", cpu_address, 4);
  }
}

void Machine::require_si_pif_ram_address(
    const char* operation,
    std::uint32_t pif_address) {
  if (pif_address != kSiSupportedPifRamAddress) {
    fail_si_pif_ram_address_unsupported(operation, pif_address);
  }
}

RdramOffset Machine::require_si_dma_rdram_span(
    const char* operation,
    RdramOffset rdram_address) {
  RdramOffset translated_rdram_address = 0;
  if (!translate_cpu_physical_rdram_address(
          rdram_address,
          kPifRamSizeBytes,
          translated_rdram_address)) {
    fail_si_dma_rdram_span_out_of_range(
        operation,
        rdram_address,
        static_cast<std::uint32_t>(kPifRamSizeBytes));
  }

  return translated_rdram_address;
}

void Machine::perform_si_dram_to_pif_dma(std::uint32_t pif_address) {
  require_si_pif_ram_address("SI RDRAM-to-PIF DMA", pif_address);
  const RdramOffset rdram_address =
      require_si_dma_rdram_span("SI RDRAM-to-PIF DMA source", si_dram_address_);

  // Local SI subset: exactly 64 bytes copy immediately from Machine-owned
  // physical RDRAM into local PIF RAM. No PIF command execution, timing, busy
  // delay, boot behavior, or CPU PIF memory mapping is modeled here.
  for (std::size_t i = 0; i < kPifRamSizeBytes; ++i) {
    pif_ram_[i] = read_rdram_u8(rdram_address + static_cast<RdramOffset>(i));
  }
}

void Machine::perform_si_pif_to_dram_dma(std::uint32_t pif_address) {
  require_si_pif_ram_address("SI PIF-to-RDRAM DMA", pif_address);
  const RdramOffset rdram_address =
      require_si_dma_rdram_span("SI PIF-to-RDRAM DMA destination", si_dram_address_);

  // RDRAM writes use the existing helpers, so overlapping local LL/SC
  // reservations are invalidated only after full preflight succeeds.
  for (std::size_t i = 0; i < kPifRamSizeBytes; ++i) {
    write_rdram_u8(
        rdram_address + static_cast<RdramOffset>(i),
        pif_ram_[i]);
  }
}

std::uint32_t Machine::read_mi_register_u32(
    std::uint32_t register_offset,
    CpuAddress cpu_address) const {
  switch (register_offset) {
    case kMiInterruptPendingRegisterOffset:
      return mi_interrupt_pending_ & kMiSupportedInterruptBits;

    case kMiInterruptMaskRegisterOffset:
      return mi_interrupt_mask_ & kMiSupportedInterruptBits;

    default:
      fail_unsupported_cpu_data_access("MI word read", cpu_address, 4);
  }
}

void Machine::write_mi_register_u32(
    std::uint32_t register_offset,
    CpuAddress cpu_address,
    std::uint32_t value) {
  switch (register_offset) {
    case kMiInterruptPendingRegisterOffset:
      mi_interrupt_pending_ &= ~(value & kMiSupportedInterruptBits);
      return;

    case kMiInterruptMaskRegisterOffset:
      mi_interrupt_mask_ = value & kMiSupportedInterruptBits;
      return;

    default:
      fail_unsupported_cpu_data_access("MI word write", cpu_address, 4);
  }
}

void Machine::latch_mi_interrupt_pending(std::uint32_t pending_bit) noexcept {
  // Local MI state is observable MMIO only. COP0 Cause observes a derived local
  // line, and the narrow local interrupt-entry seam may consume that line.
  mi_interrupt_pending_ |= pending_bit & kMiSupportedInterruptBits;
}

void Machine::clear_mi_interrupt_pending(std::uint32_t pending_bit) noexcept {
  mi_interrupt_pending_ &= ~(pending_bit & kMiSupportedInterruptBits);
}

std::uint32_t Machine::read_cop0_bad_vaddr() const noexcept {
  return cop0_bad_vaddr_;
}

std::uint32_t Machine::read_cop0_count() const noexcept {
  return cop0_count_;
}

std::uint32_t Machine::read_cop0_compare() const noexcept {
  return cop0_compare_;
}

std::uint32_t Machine::read_cop0_status() const noexcept {
  return cop0_status_ & kCop0SupportedStatusBits;
}

std::uint32_t Machine::read_cop0_cause() const noexcept {
  std::uint32_t cause =
      (static_cast<std::uint32_t>(cop0_exception_code_) <<
       kCop0CauseExceptionCodeShift) &
      kCop0CauseExceptionCodeMask;
  cause |=
      cop0_software_interrupt_pending_ & kCop0SoftwareInterruptPendingBits;
  if ((mi_interrupt_pending_ & mi_interrupt_mask_ & kMiSupportedInterruptBits) != 0) {
    cause |= kCop0CauseInterruptPending2;
  }
  if (cop0_timer_interrupt_pending_) {
    cause |= kCop0CauseInterruptPending7;
  }
  if (cop0_exception_branch_delay_) {
    cause |= kCop0CauseBranchDelay;
  }
  return cause;
}

std::uint32_t Machine::read_cop0_epc() const noexcept {
  return cop0_epc_;
}

void Machine::write_cop0_count(std::uint32_t value) noexcept {
  cop0_count_ = value;
}

void Machine::write_cop0_compare(std::uint32_t value) noexcept {
  cop0_compare_ = value;
  cop0_timer_interrupt_pending_ = false;
}

void Machine::write_cop0_status(std::uint32_t value) noexcept {
  cop0_status_ = value & kCop0SupportedStatusBits;
}

void Machine::write_cop0_cause(std::uint32_t value) noexcept {
  cop0_software_interrupt_pending_ = value & kCop0SoftwareInterruptPendingBits;
}

void Machine::write_cop0_epc(std::uint32_t value) noexcept {
  cop0_epc_ = static_cast<CpuAddress>(value);
}

void Machine::advance_cop0_count_after_committed_instruction() noexcept {
  ++cop0_count_;
  if (cop0_count_ == cop0_compare_) {
    cop0_timer_interrupt_pending_ = true;
  }
}

std::uint32_t Machine::local_cop0_interrupt_pending_lines() const noexcept {
  return read_cop0_cause() & kCop0SupportedInterruptPendingBits;
}

bool Machine::local_interrupt_pending() const noexcept {
  return local_cop0_interrupt_pending_lines() != 0;
}

bool Machine::local_interrupt_enabled() const noexcept {
  if (!local_interrupt_pending()) {
    return false;
  }

  const std::uint32_t enabled_pending =
      local_cop0_interrupt_pending_lines() &
      cop0_status_ &
      kCop0SupportedInterruptPendingBits;
  return enabled_pending != 0 &&
         ((cop0_status_ & kCop0StatusIe) != 0) &&
         ((cop0_status_ & kCop0StatusExl) == 0);
}

bool Machine::current_pc_allows_local_interrupt_entry() const noexcept {
  if (cpu_next_pc_ != sequential_instruction_address(cpu_pc_)) {
    return false;
  }

  if ((cpu_pc_ & 0x3u) != 0) {
    return false;
  }

  RdramOffset ignored = 0;
  return translate_cpu_rdram_address(cpu_pc_, 4, ignored);
}

bool Machine::try_enter_local_interrupt() noexcept {
  if (!local_interrupt_enabled() ||
      !current_pc_allows_local_interrupt_entry()) {
    return false;
  }

  // Minimal local interrupt entry only: no instruction is fetched/executed at
  // the interrupted PC, Cause reports interrupt ExcCode 0 with no BD state,
  // and MI pending bits remain owned by MI.
  cop0_epc_ = cpu_pc_;
  cop0_exception_code_ = kCop0ExceptionCodeInterrupt;
  cop0_exception_branch_delay_ = false;
  cop0_status_ |= kCop0StatusExl;
  cpu_pc_ = kLocalInterruptVectorPc;
  cpu_next_pc_ = kLocalInterruptVectorNextPc;
  return true;
}

bool Machine::local_synchronous_exception_entry_allowed(
    CpuAddress pc,
    CpuAddress next_pc) const noexcept {
  return next_pc == sequential_instruction_address(pc) &&
         ((cop0_status_ & kCop0StatusExl) == 0);
}

bool Machine::local_delay_slot_synchronous_exception_entry_allowed(
    CpuAddress pc,
    CpuAddress next_pc) const noexcept {
  return next_pc != sequential_instruction_address(pc) &&
         ((cop0_status_ & kCop0StatusExl) == 0) &&
         ((pc & 0x3u) == 0) &&
         pc >= 4u;
}

bool Machine::local_signed_overflow_exception_entry_allowed(
    CpuAddress pc,
    CpuAddress next_pc) const noexcept {
  return local_synchronous_exception_entry_allowed(pc, next_pc);
}

void Machine::enter_local_signed_overflow_exception(
    CpuAddress faulting_pc,
    bool branch_delay) noexcept {
  // Narrow local exception entry only. Signed arithmetic overflow does not write
  // BadVAddr; address rejection, stop/trap paths, unsupported operations, and
  // broad delay-slot fidelity remain unearned here.
  cop0_epc_ = branch_delay ? static_cast<CpuAddress>(faulting_pc - 4u) : faulting_pc;
  cop0_exception_code_ = kCop0ExceptionCodeSignedOverflow;
  cop0_exception_branch_delay_ = branch_delay;
  cop0_status_ |= kCop0StatusExl;
  cpu_pc_ = kLocalInterruptVectorPc;
  cpu_next_pc_ = kLocalInterruptVectorNextPc;
}

void Machine::enter_local_address_error_exception(
    CpuAddress faulting_pc,
    CpuAddress bad_vaddr,
    std::uint8_t exception_code,
    bool branch_delay) noexcept {
  // Narrow local address-error entry only: unaligned fetch/read/write,
  // control-transfer targets, CPU data target misses, and direct-alias fetch
  // target misses can report AdEL/AdES and BadVAddr. Blank/raw/non-direct fetch
  // rejection, TLB, and broad exception delivery remain unearned.
  cop0_epc_ = branch_delay ? static_cast<CpuAddress>(faulting_pc - 4u) : faulting_pc;
  cop0_bad_vaddr_ = bad_vaddr;
  cop0_exception_code_ = exception_code;
  cop0_exception_branch_delay_ = branch_delay;
  cop0_status_ |= kCop0StatusExl;
  cpu_pc_ = kLocalInterruptVectorPc;
  cpu_next_pc_ = kLocalInterruptVectorNextPc;
}

bool Machine::local_eret_can_return() const noexcept {
  return ((cop0_status_ & kCop0StatusExl) != 0) &&
         cpu_next_pc_ == sequential_instruction_address(cpu_pc_);
}

void Machine::return_from_local_interrupt_entry() {
  validate_control_transfer_target_alignment("ERET", cop0_epc_);
  cop0_status_ &= ~kCop0StatusExl;
  cpu_pc_ = cop0_epc_;
  cpu_next_pc_ = sequential_instruction_address(cop0_epc_);
}

std::uint32_t Machine::read_pi_register_u32(
    CpuPhysicalAddress physical_address,
    CpuAddress cpu_address) const {
  const std::uint32_t register_offset = physical_address - kPiPhysicalBase;
  switch (register_offset) {
    case kPiDramAddressRegisterOffset:
      return pi_dram_address_;

    case kPiCartAddressRegisterOffset:
      return pi_cart_address_;

    case kPiCartToRdramLengthRegisterOffset:
      return pi_cart_to_rdram_length_;

    case kPiStatusRegisterOffset:
      return pi_status_;

    default:
      fail_unsupported_cpu_data_access("PI word read", cpu_address, 4);
  }
}

void Machine::write_pi_register_u32(
    CpuPhysicalAddress physical_address,
    CpuAddress cpu_address,
    std::uint32_t value) {
  const std::uint32_t register_offset = physical_address - kPiPhysicalBase;
  switch (register_offset) {
    case kPiDramAddressRegisterOffset:
      pi_dram_address_ = static_cast<RdramOffset>(value);
      return;

    case kPiCartAddressRegisterOffset:
      pi_cart_address_ = static_cast<PiCartAddress>(value);
      return;

    case kPiCartToRdramLengthRegisterOffset:
      perform_pi_cart_to_rdram_dma(value);
      latch_mi_interrupt_pending(kMiInterruptPendingPi);
      pi_cart_to_rdram_length_ = value;
      pi_status_ = 0;
      return;

    case kPiStatusRegisterOffset:
      // Local immediate-complete PI subset: status remains idle/no-error.
      pi_status_ = 0;
      return;

    default:
      fail_unsupported_cpu_data_access("PI word write", cpu_address, 4);
  }
}

CartridgeOffset Machine::require_pi_cart_rom_source(
    PiCartAddress pi_cart_address,
    std::uint32_t byte_count) const {
  if (pi_cart_address < kPiCartRomBase) {
    fail_pi_cart_rom_source_below_base(pi_cart_address, kPiCartRomBase);
  }

  if (byte_count > 0) {
    const std::uint32_t last_byte_offset = byte_count - 1u;
    if (pi_cart_address >
        std::numeric_limits<PiCartAddress>::max() - last_byte_offset) {
      fail_pi_cart_rom_source_span_overflow(pi_cart_address, byte_count);
    }
  }

  const CartridgeOffset cartridge_offset =
      static_cast<CartridgeOffset>(pi_cart_address - kPiCartRomBase);
  const std::size_t cartridge_size = cartridge_.size_bytes();
  if (static_cast<std::size_t>(byte_count) > cartridge_size ||
      static_cast<std::size_t>(cartridge_offset) >
          cartridge_size - static_cast<std::size_t>(byte_count)) {
    fail_pi_cart_rom_source_out_of_range(
        pi_cart_address,
        cartridge_offset,
        byte_count,
        cartridge_size);
  }

  return cartridge_offset;
}

void Machine::perform_pi_cart_to_rdram_dma(std::uint32_t length_register_value) {
  const std::uint64_t transfer_count =
      static_cast<std::uint64_t>(length_register_value) + 1ull;
  if (transfer_count > 0xffffffffull) {
    fail_pi_dma_length_overflow(length_register_value);
  }

  const std::uint32_t byte_count = static_cast<std::uint32_t>(transfer_count);
  const CartridgeOffset cartridge_offset =
      require_pi_cart_rom_source(pi_cart_address_, byte_count);

  // Local PI subset: the cart register stores a PI cart-domain ROM address
  // whose supported 0x10000000 window translates to normalized Cartridge bytes.
  // The DRAM register is a physical RdramOffset, and the trigger copies exactly
  // length+1 bytes immediately. Timing, block rounding, interrupt delivery,
  // cart CPU mapping, and boot behavior are not modeled here.
  stage_cartridge_bytes_to_rdram(
      cartridge_offset,
      pi_dram_address_,
      byte_count);
}

std::uint8_t Machine::read_cpu_memory_u8(CpuAddress cpu_address) const {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU byte read",
      cpu_address,
      1,
      MachineFaultAccessIntent::kDataRead);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      return read_rdram_u8(target.rdram_offset);

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      return read_sp_memory_u8(target.kind, target.sp_memory_offset);

    case CpuDataTargetKind::kSpMmio:
      fail_unsupported_cpu_data_access("SP byte read", cpu_address, 1);

    case CpuDataTargetKind::kMi:
      fail_unsupported_cpu_data_access("MI byte read", cpu_address, 1);

    case CpuDataTargetKind::kPi:
      fail_unsupported_cpu_data_access("PI byte read", cpu_address, 1);

    case CpuDataTargetKind::kSi:
      fail_unsupported_cpu_data_access("SI byte read", cpu_address, 1);
  }

  throw std::logic_error("unknown CPU byte read target");
}

std::uint16_t Machine::read_cpu_memory_u16_be(CpuAddress cpu_address) const {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU halfword read",
      cpu_address,
      2,
      MachineFaultAccessIntent::kDataRead);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      return read_rdram_u16_be(target.rdram_offset);

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      return read_sp_memory_u16_be(target.kind, target.sp_memory_offset);

    case CpuDataTargetKind::kSpMmio:
      fail_unsupported_cpu_data_access("SP halfword read", cpu_address, 2);

    case CpuDataTargetKind::kMi:
      fail_unsupported_cpu_data_access("MI halfword read", cpu_address, 2);

    case CpuDataTargetKind::kPi:
      fail_unsupported_cpu_data_access("PI halfword read", cpu_address, 2);

    case CpuDataTargetKind::kSi:
      fail_unsupported_cpu_data_access("SI halfword read", cpu_address, 2);
  }

  throw std::logic_error("unknown CPU halfword read target");
}

std::uint32_t Machine::read_cpu_memory_u32_be(CpuAddress cpu_address) const {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU word read",
      cpu_address,
      4,
      MachineFaultAccessIntent::kDataRead);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      return read_rdram_u32_be(target.rdram_offset);

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      return read_sp_memory_u32_be(target.kind, target.sp_memory_offset);

    case CpuDataTargetKind::kSpMmio:
      return read_sp_register_u32(target.physical_address, cpu_address);

    case CpuDataTargetKind::kMi:
      return read_mi_register_u32(target.mi_register_offset, cpu_address);

    case CpuDataTargetKind::kPi:
      return read_pi_register_u32(target.physical_address, cpu_address);

    case CpuDataTargetKind::kSi:
      return read_si_register_u32(target.si_register_offset, cpu_address);
  }

  throw std::logic_error("unknown CPU word read target");
}

CpuRegisterValue Machine::read_cpu_memory_u64_be(CpuAddress cpu_address) const {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU doubleword read",
      cpu_address,
      8,
      MachineFaultAccessIntent::kDataRead);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      return read_rdram_u64_be(target.rdram_offset);

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      return read_sp_memory_u64_be(target.kind, target.sp_memory_offset);

    case CpuDataTargetKind::kSpMmio:
      fail_unsupported_cpu_data_access("SP doubleword read", cpu_address, 8);

    case CpuDataTargetKind::kMi:
      fail_unsupported_cpu_data_access("MI doubleword read", cpu_address, 8);

    case CpuDataTargetKind::kPi:
      fail_unsupported_cpu_data_access("PI doubleword read", cpu_address, 8);

    case CpuDataTargetKind::kSi:
      fail_unsupported_cpu_data_access("SI doubleword read", cpu_address, 8);
  }

  throw std::logic_error("unknown CPU doubleword read target");
}

void Machine::write_cpu_memory_u8(CpuAddress cpu_address, std::uint8_t value) {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU byte write",
      cpu_address,
      1,
      MachineFaultAccessIntent::kDataWrite);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      write_rdram_u8(target.rdram_offset, value);
      return;

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      write_sp_memory_u8(target.kind, target.sp_memory_offset, value);
      return;

    case CpuDataTargetKind::kSpMmio:
      fail_unsupported_cpu_data_access("SP byte write", cpu_address, 1);

    case CpuDataTargetKind::kMi:
      fail_unsupported_cpu_data_access("MI byte write", cpu_address, 1);

    case CpuDataTargetKind::kPi:
      fail_unsupported_cpu_data_access("PI byte write", cpu_address, 1);

    case CpuDataTargetKind::kSi:
      fail_unsupported_cpu_data_access("SI byte write", cpu_address, 1);
  }

  throw std::logic_error("unknown CPU byte write target");
}

void Machine::write_cpu_memory_u16_be(CpuAddress cpu_address, std::uint16_t value) {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU halfword write",
      cpu_address,
      2,
      MachineFaultAccessIntent::kDataWrite);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      write_rdram_u16_be(target.rdram_offset, value);
      return;

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      write_sp_memory_u16_be(target.kind, target.sp_memory_offset, value);
      return;

    case CpuDataTargetKind::kSpMmio:
      fail_unsupported_cpu_data_access("SP halfword write", cpu_address, 2);

    case CpuDataTargetKind::kMi:
      fail_unsupported_cpu_data_access("MI halfword write", cpu_address, 2);

    case CpuDataTargetKind::kPi:
      fail_unsupported_cpu_data_access("PI halfword write", cpu_address, 2);

    case CpuDataTargetKind::kSi:
      fail_unsupported_cpu_data_access("SI halfword write", cpu_address, 2);
  }

  throw std::logic_error("unknown CPU halfword write target");
}

void Machine::write_cpu_memory_u32_be(CpuAddress cpu_address, std::uint32_t value) {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU word write",
      cpu_address,
      4,
      MachineFaultAccessIntent::kDataWrite);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      write_rdram_u32_be(target.rdram_offset, value);
      return;

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      write_sp_memory_u32_be(target.kind, target.sp_memory_offset, value);
      return;

    case CpuDataTargetKind::kSpMmio:
      write_sp_register_u32(target.physical_address, cpu_address, value);
      return;

    case CpuDataTargetKind::kMi:
      write_mi_register_u32(target.mi_register_offset, cpu_address, value);
      return;

    case CpuDataTargetKind::kPi:
      write_pi_register_u32(target.physical_address, cpu_address, value);
      return;

    case CpuDataTargetKind::kSi:
      write_si_register_u32(target.si_register_offset, cpu_address, value);
      return;
  }

  throw std::logic_error("unknown CPU word write target");
}

void Machine::write_cpu_memory_u64_be(CpuAddress cpu_address, CpuRegisterValue value) {
  const CpuDataTarget target = require_cpu_data_target(
      "CPU doubleword write",
      cpu_address,
      8,
      MachineFaultAccessIntent::kDataWrite);
  switch (target.kind) {
    case CpuDataTargetKind::kRdram:
      write_rdram_u64_be(target.rdram_offset, value);
      return;

    case CpuDataTargetKind::kSpDmem:
    case CpuDataTargetKind::kSpImem:
      write_sp_memory_u64_be(target.kind, target.sp_memory_offset, value);
      return;

    case CpuDataTargetKind::kSpMmio:
      fail_unsupported_cpu_data_access("SP doubleword write", cpu_address, 8);

    case CpuDataTargetKind::kMi:
      fail_unsupported_cpu_data_access("MI doubleword write", cpu_address, 8);

    case CpuDataTargetKind::kPi:
      fail_unsupported_cpu_data_access("PI doubleword write", cpu_address, 8);

    case CpuDataTargetKind::kSi:
      fail_unsupported_cpu_data_access("SI doubleword write", cpu_address, 8);
  }

  throw std::logic_error("unknown CPU doubleword write target");
}

CpuAddress Machine::cpu_pc() const {
  return cpu_pc_;
}

CpuAddress Machine::cpu_next_pc() const {
  return cpu_next_pc_;
}

CpuRegisterValue Machine::inspect_cpu_hi() const {
  return cpu_hi();
}

CpuRegisterValue Machine::inspect_cpu_lo() const {
  return cpu_lo();
}

CpuRegisterValue Machine::inspect_cpu_gpr(std::size_t index) const {
  return read_cpu_gpr_value(index);
}

CpuRegisterValue Machine::cpu_hi() const {
  return cpu_hi_;
}

CpuRegisterValue Machine::cpu_lo() const {
  return cpu_lo_;
}

CpuRegisterValue Machine::read_cpu_gpr_value(std::size_t index) const {
  if (index >= cpu_gprs_.size()) {
    fail_cpu_gpr_index(index);
  }

  if (index == 0) {
    return 0;
  }

  return cpu_gprs_[index];
}

std::uint32_t Machine::read_cpu_gpr_word(std::size_t index) const {
  return static_cast<std::uint32_t>(read_cpu_gpr_value(index));
}

void Machine::stage_cpu_pc(CpuAddress value) {
  write_cpu_pc(value);
}

void Machine::stage_cpu_next_pc(CpuAddress value) {
  write_cpu_next_pc(value);
}

void Machine::stage_cpu_hi(CpuRegisterValue value) {
  write_cpu_hi(value);
}

void Machine::stage_cpu_lo(CpuRegisterValue value) {
  write_cpu_lo(value);
}

void Machine::stage_cpu_gpr(std::size_t index, CpuRegisterValue value) {
  write_cpu_gpr_value(index, value);
}

void Machine::write_cpu_pc(CpuAddress value) {
  cpu_pc_ = value;
  cpu_next_pc_ = sequential_instruction_address(value);
}

void Machine::write_cpu_next_pc(CpuAddress value) {
  cpu_next_pc_ = value;
}

void Machine::write_cpu_hi(CpuRegisterValue value) {
  cpu_hi_ = value;
}

void Machine::write_cpu_lo(CpuRegisterValue value) {
  cpu_lo_ = value;
}

void Machine::write_cpu_hi_word_sign_extended_result(std::uint32_t value) {
  write_cpu_hi(sign_extend_u32_to_cpu_value(value));
}

void Machine::write_cpu_lo_word_sign_extended_result(std::uint32_t value) {
  write_cpu_lo(sign_extend_u32_to_cpu_value(value));
}

void Machine::write_cpu_gpr_value(std::size_t index, CpuRegisterValue value) {
  if (index >= cpu_gprs_.size()) {
    fail_cpu_gpr_index(index);
  }

  if (index == 0) {
    return;
  }

  cpu_gprs_[index] = value;
}

void Machine::write_cpu_gpr_word_sign_extended_result(
    std::size_t index,
    std::uint32_t value) {
  write_cpu_gpr_value(index, sign_extend_u32_to_cpu_value(value));
}

void Machine::write_cpu_gpr_word_zero_extended_result(
    std::size_t index,
    std::uint32_t value) {
  write_cpu_gpr_value(index, zero_extend_u32_to_cpu_value(value));
}

void Machine::write_cpu_gpr_partial_word_sign_extended_result(
    std::size_t index,
    std::uint32_t value) {
  write_cpu_gpr_value(index, sign_extend_u32_to_cpu_value(value));
}

void Machine::write_cpu_gpr_partial_word_preserve_high_result(
    std::size_t index,
    std::uint32_t value,
    CpuRegisterValue previous_value) {
  constexpr CpuRegisterValue kHighWordMask =
      static_cast<CpuRegisterValue>(0xffffffff00000000ull);
  write_cpu_gpr_value(
      index,
      (previous_value & kHighWordMask) | static_cast<CpuRegisterValue>(value));
}

CpuInstructionWord Machine::fetch_cpu_instruction_word() const {
  const CpuAddress pc = cpu_pc();

  if ((pc & 0x3u) != 0) {
    fail_unaligned_instruction_fetch(pc);
  }

  CpuPhysicalAddress physical_address = 0;
  if (!translate_direct_cpu_physical_address(pc, physical_address)) {
    fail_cpu_direct_rdram_address(
        "CPU instruction fetch",
        pc,
        4,
        MachineFaultAccessIntent::kInstructionFetch);
  }

  RdramOffset rdram_address = 0;
  if (!translate_cpu_physical_rdram_address(physical_address, 4, rdram_address)) {
    fail_cpu_direct_rdram_address(
        "CPU instruction fetch",
        pc,
        4,
        MachineFaultAccessIntent::kInstructionFetchDirectTargetMiss);
  }

  return read_rdram_u32_be(rdram_address);
}

Machine::DecodedCpuInstructionWord Machine::decode_cpu_instruction_word(CpuInstructionWord raw) {
  DecodedCpuInstructionWord instruction;
  instruction.raw = raw;
  instruction.opcode = static_cast<std::uint8_t>((raw >> 26) & 0x3f);
  instruction.rs = static_cast<std::uint8_t>((raw >> 21) & 0x1f);
  instruction.rt = static_cast<std::uint8_t>((raw >> 16) & 0x1f);
  instruction.rd = static_cast<std::uint8_t>((raw >> 11) & 0x1f);
  instruction.sa = static_cast<std::uint8_t>((raw >> 6) & 0x1f);
  instruction.funct = static_cast<std::uint8_t>(raw & 0x3f);
  instruction.immediate_u16 = static_cast<std::uint16_t>(raw & 0xffff);
  instruction.immediate_i16 = i16_from_u16_bits(instruction.immediate_u16);
  instruction.jump_target = raw & 0x03ffffff;
  return instruction;
}

Machine::CpuInstructionIdentity Machine::identify_cpu_instruction(
    const DecodedCpuInstructionWord& instruction) {
  switch (instruction.opcode) {
    case 0x00:
      switch (instruction.funct) {
        case 0x00: return CpuInstructionIdentity::kSpecialSll;
        case 0x02: return CpuInstructionIdentity::kSpecialSrl;
        case 0x03: return CpuInstructionIdentity::kSpecialSra;
        case 0x04: return CpuInstructionIdentity::kSpecialSllv;
        case 0x06: return CpuInstructionIdentity::kSpecialSrlv;
        case 0x07: return CpuInstructionIdentity::kSpecialSrav;
        case 0x08: return CpuInstructionIdentity::kSpecialJr;
        case 0x09: return CpuInstructionIdentity::kSpecialJalr;
        case 0x0c: return CpuInstructionIdentity::kSpecialSyscall;
        case 0x0d: return CpuInstructionIdentity::kSpecialBreak;
        case 0x0f: return CpuInstructionIdentity::kSpecialSync;
        case 0x10: return CpuInstructionIdentity::kSpecialMfhi;
        case 0x11: return CpuInstructionIdentity::kSpecialMthi;
        case 0x12: return CpuInstructionIdentity::kSpecialMflo;
        case 0x13: return CpuInstructionIdentity::kSpecialMtlo;
        case 0x14: return CpuInstructionIdentity::kSpecialDsllv;
        case 0x16: return CpuInstructionIdentity::kSpecialDsrlv;
        case 0x17: return CpuInstructionIdentity::kSpecialDsrav;
        case 0x18: return CpuInstructionIdentity::kSpecialMult;
        case 0x19: return CpuInstructionIdentity::kSpecialMultu;
        case 0x1a: return CpuInstructionIdentity::kSpecialDiv;
        case 0x1b: return CpuInstructionIdentity::kSpecialDivu;
        case 0x1c: return CpuInstructionIdentity::kSpecialDmult;
        case 0x1d: return CpuInstructionIdentity::kSpecialDmultu;
        case 0x1e: return CpuInstructionIdentity::kSpecialDdiv;
        case 0x1f: return CpuInstructionIdentity::kSpecialDdivu;
        case 0x20: return CpuInstructionIdentity::kSpecialAdd;
        case 0x21: return CpuInstructionIdentity::kSpecialAddu;
        case 0x22: return CpuInstructionIdentity::kSpecialSub;
        case 0x23: return CpuInstructionIdentity::kSpecialSubu;
        case 0x24: return CpuInstructionIdentity::kSpecialAnd;
        case 0x25: return CpuInstructionIdentity::kSpecialOr;
        case 0x26: return CpuInstructionIdentity::kSpecialXor;
        case 0x27: return CpuInstructionIdentity::kSpecialNor;
        case 0x2a: return CpuInstructionIdentity::kSpecialSlt;
        case 0x2b: return CpuInstructionIdentity::kSpecialSltu;
        case 0x2c: return CpuInstructionIdentity::kSpecialDadd;
        case 0x2d: return CpuInstructionIdentity::kSpecialDaddu;
        case 0x2e: return CpuInstructionIdentity::kSpecialDsub;
        case 0x2f: return CpuInstructionIdentity::kSpecialDsubu;
        case 0x30: return CpuInstructionIdentity::kSpecialTge;
        case 0x31: return CpuInstructionIdentity::kSpecialTgeu;
        case 0x32: return CpuInstructionIdentity::kSpecialTlt;
        case 0x33: return CpuInstructionIdentity::kSpecialTltu;
        case 0x34: return CpuInstructionIdentity::kSpecialTeq;
        case 0x36: return CpuInstructionIdentity::kSpecialTne;
        case 0x38: return CpuInstructionIdentity::kSpecialDsll;
        case 0x3a: return CpuInstructionIdentity::kSpecialDsrl;
        case 0x3b: return CpuInstructionIdentity::kSpecialDsra;
        case 0x3c: return CpuInstructionIdentity::kSpecialDsll32;
        case 0x3e: return CpuInstructionIdentity::kSpecialDsrl32;
        case 0x3f: return CpuInstructionIdentity::kSpecialDsra32;
        default: return CpuInstructionIdentity::kSpecialUnknown;
      }

    case 0x01:
      switch (instruction.rt) {
        case 0x00: return CpuInstructionIdentity::kRegimmBltz;
        case 0x01: return CpuInstructionIdentity::kRegimmBgez;
        case 0x02: return CpuInstructionIdentity::kRegimmBltzl;
        case 0x03: return CpuInstructionIdentity::kRegimmBgezl;
        case 0x08: return CpuInstructionIdentity::kRegimmTgei;
        case 0x09: return CpuInstructionIdentity::kRegimmTgeiu;
        case 0x0a: return CpuInstructionIdentity::kRegimmTlti;
        case 0x0b: return CpuInstructionIdentity::kRegimmTltiu;
        case 0x0c: return CpuInstructionIdentity::kRegimmTeqi;
        case 0x0e: return CpuInstructionIdentity::kRegimmTnei;
        case 0x10: return CpuInstructionIdentity::kRegimmBltzal;
        case 0x11: return CpuInstructionIdentity::kRegimmBgezal;
        case 0x12: return CpuInstructionIdentity::kRegimmBltzall;
        case 0x13: return CpuInstructionIdentity::kRegimmBgezall;
        default: return CpuInstructionIdentity::kRegimmUnknown;
      }

    case 0x02: return CpuInstructionIdentity::kJ;
    case 0x03: return CpuInstructionIdentity::kJal;
    case 0x04: return CpuInstructionIdentity::kBeq;
    case 0x05: return CpuInstructionIdentity::kBne;
    case 0x06: return CpuInstructionIdentity::kBlez;
    case 0x07: return CpuInstructionIdentity::kBgtz;
    case 0x08: return CpuInstructionIdentity::kAddi;
    case 0x09: return CpuInstructionIdentity::kAddiu;
    case 0x0a: return CpuInstructionIdentity::kSlti;
    case 0x0b: return CpuInstructionIdentity::kSltiu;
    case 0x0c: return CpuInstructionIdentity::kAndi;
    case 0x0d: return CpuInstructionIdentity::kOri;
    case 0x0e: return CpuInstructionIdentity::kXori;
    case 0x0f: return CpuInstructionIdentity::kLui;
    case 0x10:
      switch (instruction.rs) {
        case 0x00: return CpuInstructionIdentity::kCop0Mfc0;
        case 0x04: return CpuInstructionIdentity::kCop0Mtc0;
        case 0x10:
          if (instruction.raw == 0x42000018u) {
            return CpuInstructionIdentity::kCop0Eret;
          }
          return CpuInstructionIdentity::kCop0;
        default: return CpuInstructionIdentity::kCop0;
      }

    case 0x11: return CpuInstructionIdentity::kCop1;
    case 0x12: return CpuInstructionIdentity::kCop2;
    case 0x13: return CpuInstructionIdentity::kCop3;
    case 0x14: return CpuInstructionIdentity::kBeql;
    case 0x15: return CpuInstructionIdentity::kBnel;
    case 0x16: return CpuInstructionIdentity::kBlezl;
    case 0x17: return CpuInstructionIdentity::kBgtzl;
    case 0x18: return CpuInstructionIdentity::kDaddi;
    case 0x19: return CpuInstructionIdentity::kDaddiu;
    case 0x1a: return CpuInstructionIdentity::kLdl;
    case 0x1b: return CpuInstructionIdentity::kLdr;
    case 0x20: return CpuInstructionIdentity::kLb;
    case 0x21: return CpuInstructionIdentity::kLh;
    case 0x22: return CpuInstructionIdentity::kLwl;
    case 0x23: return CpuInstructionIdentity::kLw;
    case 0x24: return CpuInstructionIdentity::kLbu;
    case 0x25: return CpuInstructionIdentity::kLhu;
    case 0x26: return CpuInstructionIdentity::kLwr;
    case 0x27: return CpuInstructionIdentity::kLwu;
    case 0x28: return CpuInstructionIdentity::kSb;
    case 0x29: return CpuInstructionIdentity::kSh;
    case 0x2a: return CpuInstructionIdentity::kSwl;
    case 0x2b: return CpuInstructionIdentity::kSw;
    case 0x2c: return CpuInstructionIdentity::kSdl;
    case 0x2d: return CpuInstructionIdentity::kSdr;
    case 0x2e: return CpuInstructionIdentity::kSwr;
    case 0x2f: return CpuInstructionIdentity::kCache;
    case 0x30: return CpuInstructionIdentity::kLl;
    case 0x31: return CpuInstructionIdentity::kLwc1;
    case 0x32: return CpuInstructionIdentity::kLwc2;
    case 0x34: return CpuInstructionIdentity::kLld;
    case 0x35: return CpuInstructionIdentity::kLdc1;
    case 0x36: return CpuInstructionIdentity::kLdc2;
    case 0x37: return CpuInstructionIdentity::kLd;
    case 0x38: return CpuInstructionIdentity::kSc;
    case 0x39: return CpuInstructionIdentity::kSwc1;
    case 0x3a: return CpuInstructionIdentity::kSwc2;
    case 0x3c: return CpuInstructionIdentity::kScd;
    case 0x3d: return CpuInstructionIdentity::kSdc1;
    case 0x3e: return CpuInstructionIdentity::kSdc2;
    case 0x3f: return CpuInstructionIdentity::kSd;
    default: return CpuInstructionIdentity::kUnknownPrimary;
  }
}

Machine::CpuInstructionExecutionResult Machine::execute_cpu_instruction(
    CpuInstructionIdentity identity,
    const DecodedCpuInstructionWord& instruction) {
  const auto trap_execution_result = [](bool trap_taken) {
    return trap_taken ? CpuInstructionExecutionResult::kStopped
                      : CpuInstructionExecutionResult::kExecuted;
  };

  switch (identity) {
    case CpuInstructionIdentity::kSpecialSll: {
      const std::uint32_t value = read_cpu_gpr_word(instruction.rt) << instruction.sa;
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSrl: {
      const std::uint32_t value = read_cpu_gpr_word(instruction.rt) >> instruction.sa;
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSra: {
      const std::uint32_t value =
          arithmetic_shift_right_u32(read_cpu_gpr_word(instruction.rt), instruction.sa);
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSllv: {
      const std::uint8_t sa =
          variable_shift_amount_u32(read_cpu_gpr_word(instruction.rs));
      const std::uint32_t value = read_cpu_gpr_word(instruction.rt) << sa;
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSrlv: {
      const std::uint8_t sa =
          variable_shift_amount_u32(read_cpu_gpr_word(instruction.rs));
      const std::uint32_t value = read_cpu_gpr_word(instruction.rt) >> sa;
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSrav: {
      const std::uint8_t sa =
          variable_shift_amount_u32(read_cpu_gpr_word(instruction.rs));
      const std::uint32_t value =
          arithmetic_shift_right_u32(read_cpu_gpr_word(instruction.rt), sa);
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsll: {
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt) << instruction.sa;
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsrl: {
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt) >> instruction.sa;
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsra: {
      const CpuRegisterValue value =
          arithmetic_shift_right_cpu_value(read_cpu_gpr_value(instruction.rt), instruction.sa);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsll32: {
      const std::uint8_t sa = static_cast<std::uint8_t>(instruction.sa + 32u);
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt) << sa;
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsrl32: {
      const std::uint8_t sa = static_cast<std::uint8_t>(instruction.sa + 32u);
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt) >> sa;
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsra32: {
      const std::uint8_t sa = static_cast<std::uint8_t>(instruction.sa + 32u);
      const CpuRegisterValue value =
          arithmetic_shift_right_cpu_value(read_cpu_gpr_value(instruction.rt), sa);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsllv: {
      const std::uint8_t sa =
          variable_shift_amount_cpu_value(read_cpu_gpr_value(instruction.rs));
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt) << sa;
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsrlv: {
      const std::uint8_t sa =
          variable_shift_amount_cpu_value(read_cpu_gpr_value(instruction.rs));
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt) >> sa;
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsrav: {
      const std::uint8_t sa =
          variable_shift_amount_cpu_value(read_cpu_gpr_value(instruction.rs));
      const CpuRegisterValue value =
          arithmetic_shift_right_cpu_value(read_cpu_gpr_value(instruction.rt), sa);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialJr: {
      const CpuAddress target = read_cpu_gpr_word(instruction.rs);
      validate_control_transfer_target_alignment("JR", target);
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialJalr: {
      const CpuAddress target = read_cpu_gpr_word(instruction.rs);
      validate_control_transfer_target_alignment("JALR", target);
      write_cpu_gpr_value(instruction.rd, link_return_address(cpu_pc()));
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSyscall:
      return CpuInstructionExecutionResult::kStopped;

    case CpuInstructionIdentity::kSpecialBreak:
      return CpuInstructionExecutionResult::kStopped;

    case CpuInstructionIdentity::kSpecialSync:
      return CpuInstructionExecutionResult::kExecuted;

    case CpuInstructionIdentity::kCop0Mfc0:
      switch (instruction.rd) {
        case kCop0BadVaddrRegisterIndex:
          write_cpu_gpr_word_sign_extended_result(instruction.rt, read_cop0_bad_vaddr());
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0CountRegisterIndex:
          write_cpu_gpr_word_sign_extended_result(instruction.rt, read_cop0_count());
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0CompareRegisterIndex:
          write_cpu_gpr_word_sign_extended_result(instruction.rt, read_cop0_compare());
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0StatusRegisterIndex:
          write_cpu_gpr_word_sign_extended_result(instruction.rt, read_cop0_status());
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0CauseRegisterIndex:
          write_cpu_gpr_word_sign_extended_result(instruction.rt, read_cop0_cause());
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0EpcRegisterIndex:
          write_cpu_gpr_word_sign_extended_result(instruction.rt, read_cop0_epc());
          return CpuInstructionExecutionResult::kExecuted;

        default:
          return CpuInstructionExecutionResult::kUnsupported;
      }

    case CpuInstructionIdentity::kCop0Mtc0:
      switch (instruction.rd) {
        case kCop0CountRegisterIndex:
          write_cop0_count(read_cpu_gpr_word(instruction.rt));
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0CompareRegisterIndex:
          write_cop0_compare(read_cpu_gpr_word(instruction.rt));
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0StatusRegisterIndex:
          write_cop0_status(read_cpu_gpr_word(instruction.rt));
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0CauseRegisterIndex:
          write_cop0_cause(read_cpu_gpr_word(instruction.rt));
          return CpuInstructionExecutionResult::kExecuted;

        case kCop0EpcRegisterIndex:
          write_cop0_epc(read_cpu_gpr_word(instruction.rt));
          return CpuInstructionExecutionResult::kExecuted;

        default:
          return CpuInstructionExecutionResult::kUnsupported;
      }

    case CpuInstructionIdentity::kSpecialMfhi: {
      write_cpu_gpr_value(instruction.rd, cpu_hi());
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMthi: {
      write_cpu_hi(read_cpu_gpr_value(instruction.rs));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMflo: {
      write_cpu_gpr_value(instruction.rd, cpu_lo());
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMtlo: {
      write_cpu_lo(read_cpu_gpr_value(instruction.rs));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMult: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rs)));
      const std::int64_t rhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rt)));
      const std::uint64_t product = static_cast<std::uint64_t>(lhs * rhs);
      write_cpu_lo_word_sign_extended_result(low_u32(product));
      write_cpu_hi_word_sign_extended_result(high_u32(product));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialMultu: {
      const std::uint64_t lhs = static_cast<std::uint64_t>(read_cpu_gpr_word(instruction.rs));
      const std::uint64_t rhs = static_cast<std::uint64_t>(read_cpu_gpr_word(instruction.rt));
      const std::uint64_t product = lhs * rhs;
      write_cpu_lo_word_sign_extended_result(low_u32(product));
      write_cpu_hi_word_sign_extended_result(high_u32(product));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDiv: {
      const std::uint32_t divisor_u32 = read_cpu_gpr_word(instruction.rt);
      if (divisor_u32 == 0) {
        return CpuInstructionExecutionResult::kExecuted;
      }

      const std::int64_t dividend =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rs)));
      const std::int64_t divisor =
          static_cast<std::int64_t>(i32_from_u32_bits(divisor_u32));
      const std::int64_t quotient = dividend / divisor;
      const std::int64_t remainder = dividend % divisor;
      write_cpu_lo_word_sign_extended_result(static_cast<std::uint32_t>(quotient));
      write_cpu_hi_word_sign_extended_result(static_cast<std::uint32_t>(remainder));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDivu: {
      const std::uint32_t divisor = read_cpu_gpr_word(instruction.rt);
      if (divisor == 0) {
        return CpuInstructionExecutionResult::kExecuted;
      }

      const std::uint32_t dividend = read_cpu_gpr_word(instruction.rs);
      write_cpu_lo_word_sign_extended_result(dividend / divisor);
      write_cpu_hi_word_sign_extended_result(dividend % divisor);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDmult: {
      const DoubleCpuRegisterValueBits product =
          multiply_i64_to_i128_bits(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt));
      write_cpu_hi(product.hi);
      write_cpu_lo(product.lo);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDmultu: {
      const DoubleCpuRegisterValueBits product =
          multiply_u64_to_u128_bits(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt));
      write_cpu_hi(product.hi);
      write_cpu_lo(product.lo);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDdiv: {
      const CpuRegisterValue divisor_bits = read_cpu_gpr_value(instruction.rt);
      const CpuRegisterValue dividend_bits = read_cpu_gpr_value(instruction.rs);
      if (divisor_bits == 0 || signed_cpu_division_overflows(dividend_bits, divisor_bits)) {
        return CpuInstructionExecutionResult::kExecuted;
      }

      const std::int64_t dividend = i64_from_cpu_value_bits(dividend_bits);
      const std::int64_t divisor = i64_from_cpu_value_bits(divisor_bits);
      const std::int64_t quotient = dividend / divisor;
      const std::int64_t remainder = dividend % divisor;
      write_cpu_lo(static_cast<CpuRegisterValue>(quotient));
      write_cpu_hi(static_cast<CpuRegisterValue>(remainder));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDdivu: {
      const CpuRegisterValue divisor = read_cpu_gpr_value(instruction.rt);
      if (divisor == 0) {
        return CpuInstructionExecutionResult::kExecuted;
      }

      const CpuRegisterValue dividend = read_cpu_gpr_value(instruction.rs);
      write_cpu_lo(dividend / divisor);
      write_cpu_hi(dividend % divisor);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialTge:
      return trap_execution_result(
          signed_cpu_value_greater_equal(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt)));

    case CpuInstructionIdentity::kSpecialTgeu:
      return trap_execution_result(
          unsigned_cpu_value_greater_equal(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt)));

    case CpuInstructionIdentity::kSpecialTlt:
      return trap_execution_result(
          signed_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt)));

    case CpuInstructionIdentity::kSpecialTltu:
      return trap_execution_result(
          unsigned_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt)));

    case CpuInstructionIdentity::kSpecialTeq:
      return trap_execution_result(
          cpu_values_equal(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt)));

    case CpuInstructionIdentity::kSpecialTne:
      return trap_execution_result(
          !cpu_values_equal(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt)));

    case CpuInstructionIdentity::kRegimmTgei:
      return trap_execution_result(
          signed_cpu_value_greater_equal(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16)));

    case CpuInstructionIdentity::kRegimmTgeiu:
      return trap_execution_result(
          unsigned_cpu_value_greater_equal(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16)));

    case CpuInstructionIdentity::kRegimmTlti:
      return trap_execution_result(
          signed_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16)));

    case CpuInstructionIdentity::kRegimmTltiu:
      return trap_execution_result(
          unsigned_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16)));

    case CpuInstructionIdentity::kRegimmTeqi:
      return trap_execution_result(
          cpu_values_equal(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16)));

    case CpuInstructionIdentity::kRegimmTnei:
      return trap_execution_result(
          !cpu_values_equal(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16)));

    case CpuInstructionIdentity::kRegimmBltz: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value < 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBgez: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value >= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBltzl: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value < 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kRegimmBgezl: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value >= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kRegimmBltzal: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      write_cpu_gpr_value(31, link_return_address(cpu_pc()));
      if (value < 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBgezal: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      write_cpu_gpr_value(31, link_return_address(cpu_pc()));
      if (value >= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kRegimmBltzall: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value < 0) {
        write_cpu_gpr_value(31, link_return_address(cpu_pc()));
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kRegimmBgezall: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value >= 0) {
        write_cpu_gpr_value(31, link_return_address(cpu_pc()));
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kJ: {
      const CpuAddress target = jump_target_address(cpu_pc(), instruction.jump_target);
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kJal: {
      write_cpu_gpr_value(31, link_return_address(cpu_pc()));
      const CpuAddress target = jump_target_address(cpu_pc(), instruction.jump_target);
      write_cpu_next_pc(target);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBeq: {
      if (cpu_values_equal(read_cpu_gpr_value(instruction.rs), read_cpu_gpr_value(instruction.rt))) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBne: {
      if (!cpu_values_equal(read_cpu_gpr_value(instruction.rs), read_cpu_gpr_value(instruction.rt))) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBlez: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value <= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBgtz: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value > 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kBeql: {
      if (cpu_values_equal(read_cpu_gpr_value(instruction.rs), read_cpu_gpr_value(instruction.rt))) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kBnel: {
      if (!cpu_values_equal(read_cpu_gpr_value(instruction.rs), read_cpu_gpr_value(instruction.rt))) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kBlezl: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value <= 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kBgtzl: {
      const std::int64_t value = i64_from_cpu_value_bits(read_cpu_gpr_value(instruction.rs));
      if (value > 0) {
        write_cpu_next_pc(branch_target_address(cpu_pc(), instruction.immediate_i16));
        return CpuInstructionExecutionResult::kExecuted;
      }
      return CpuInstructionExecutionResult::kBranchLikelyNotTaken;
    }

    case CpuInstructionIdentity::kSpecialAdd: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rs)));
      const std::int64_t rhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rt)));
      const std::int64_t value = lhs + rhs;

      if (signed_i32_result_out_of_range(value)) {
        fail_signed_arithmetic_overflow("ADD");
      }

      write_cpu_gpr_word_sign_extended_result(instruction.rd, u32_bits_from_i32_value(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialAddu: {
      const std::uint32_t value =
          read_cpu_gpr_word(instruction.rs) + read_cpu_gpr_word(instruction.rt);
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSub: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rs)));
      const std::int64_t rhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rt)));
      const std::int64_t value = lhs - rhs;

      if (signed_i32_result_out_of_range(value)) {
        fail_signed_arithmetic_overflow("SUB");
      }

      write_cpu_gpr_word_sign_extended_result(instruction.rd, u32_bits_from_i32_value(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSubu: {
      const std::uint32_t value =
          read_cpu_gpr_word(instruction.rs) - read_cpu_gpr_word(instruction.rt);
      write_cpu_gpr_word_sign_extended_result(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDadd: {
      const CpuRegisterValue value =
          checked_signed_cpu_add(
              "DADD",
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt));
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDaddu: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) + read_cpu_gpr_value(instruction.rt);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsub: {
      const CpuRegisterValue value =
          checked_signed_cpu_sub(
              "DSUB",
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt));
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialDsubu: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) - read_cpu_gpr_value(instruction.rt);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialAnd: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) & read_cpu_gpr_value(instruction.rt);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialOr: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) | read_cpu_gpr_value(instruction.rt);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialXor: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) ^ read_cpu_gpr_value(instruction.rt);
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialNor: {
      const CpuRegisterValue value =
          ~(read_cpu_gpr_value(instruction.rs) | read_cpu_gpr_value(instruction.rt));
      write_cpu_gpr_value(instruction.rd, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSlt: {
      const bool value =
          signed_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt));
      write_cpu_gpr_value(instruction.rd, cpu_value_from_bool(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSpecialSltu: {
      const bool value =
          unsigned_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              read_cpu_gpr_value(instruction.rt));
      write_cpu_gpr_value(instruction.rd, cpu_value_from_bool(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kAddi: {
      const std::int64_t lhs =
          static_cast<std::int64_t>(i32_from_u32_bits(read_cpu_gpr_word(instruction.rs)));
      const std::int64_t rhs = static_cast<std::int64_t>(instruction.immediate_i16);
      const std::int64_t value = lhs + rhs;

      if (signed_i32_result_out_of_range(value)) {
        fail_signed_arithmetic_overflow("ADDI");
      }

      write_cpu_gpr_word_sign_extended_result(instruction.rt, u32_bits_from_i32_value(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kAddiu: {
      const std::uint32_t value =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      write_cpu_gpr_word_sign_extended_result(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kDaddi: {
      const CpuRegisterValue value =
          checked_signed_cpu_add(
              "DADDI",
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16));
      write_cpu_gpr_value(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kDaddiu: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) +
          sign_extend_u16_to_cpu_value(instruction.immediate_u16);
      write_cpu_gpr_value(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSlti: {
      const bool value =
          signed_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16));
      write_cpu_gpr_value(instruction.rt, cpu_value_from_bool(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSltiu: {
      const bool value =
          unsigned_cpu_value_less_than(
              read_cpu_gpr_value(instruction.rs),
              sign_extend_u16_to_cpu_value(instruction.immediate_u16));
      write_cpu_gpr_value(instruction.rt, cpu_value_from_bool(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kAndi: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) &
          static_cast<CpuRegisterValue>(instruction.immediate_u16);
      write_cpu_gpr_value(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kOri: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) |
          static_cast<CpuRegisterValue>(instruction.immediate_u16);
      write_cpu_gpr_value(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kXori: {
      const CpuRegisterValue value =
          read_cpu_gpr_value(instruction.rs) ^
          static_cast<CpuRegisterValue>(instruction.immediate_u16);
      write_cpu_gpr_value(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLui: {
      const std::uint32_t value =
          static_cast<std::uint32_t>(instruction.immediate_u16) << 16;
      write_cpu_gpr_word_sign_extended_result(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLb: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint8_t value = read_cpu_memory_u8(effective_address);
      write_cpu_gpr_word_sign_extended_result(instruction.rt, sign_extend_u8_to_u32(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLbu: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint8_t value = read_cpu_memory_u8(effective_address);
      write_cpu_gpr_word_zero_extended_result(instruction.rt, static_cast<std::uint32_t>(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLh: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x1u) != 0) {
        fail_unaligned_halfword_memory_access(
            "LH",
            effective_address,
            MachineFaultAccessIntent::kDataRead);
      }

      const std::uint16_t value = read_cpu_memory_u16_be(effective_address);
      write_cpu_gpr_word_sign_extended_result(instruction.rt, sign_extend_u16_to_u32(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLhu: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x1u) != 0) {
        fail_unaligned_halfword_memory_access(
            "LHU",
            effective_address,
            MachineFaultAccessIntent::kDataRead);
      }

      const std::uint16_t value = read_cpu_memory_u16_be(effective_address);
      write_cpu_gpr_word_zero_extended_result(instruction.rt, static_cast<std::uint32_t>(value));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLwl: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = 4u - byte_offset;

      static_cast<void>(require_cpu_data_target(
          "LWL effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataRead));

      std::uint32_t value = read_cpu_gpr_word(instruction.rt);
      for (std::uint32_t i = 0; i < byte_count; ++i) {
        const std::uint8_t memory_byte = read_cpu_memory_u8(effective_address + i);
        value = replace_u32_byte_be(value, static_cast<std::size_t>(i), memory_byte);
      }

      // This local LWL lane rule always replaces byte 0 of the low word, so the
      // merged word's sign bit is known and can define the full stored value.
      write_cpu_gpr_partial_word_sign_extended_result(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLw: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x3u) != 0) {
        fail_unaligned_word_memory_access(
            "LW",
            effective_address,
            MachineFaultAccessIntent::kDataRead);
      }

      const std::uint32_t value = read_cpu_memory_u32_be(effective_address);
      write_cpu_gpr_word_sign_extended_result(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLl: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x3u) != 0) {
        fail_unaligned_word_memory_access(
            "LL",
            effective_address,
            MachineFaultAccessIntent::kDataRead);
      }

      const CpuDataTarget target = require_cpu_data_target(
          "LL",
          effective_address,
          4,
          MachineFaultAccessIntent::kDataRead);
      switch (target.kind) {
        case CpuDataTargetKind::kRdram: {
          const std::uint32_t value = read_rdram_u32_be(target.rdram_offset);
          write_cpu_gpr_word_sign_extended_result(instruction.rt, value);
          set_cpu_rdram_reservation(target.rdram_offset, 4);
          return CpuInstructionExecutionResult::kExecuted;
        }

        case CpuDataTargetKind::kSpDmem:
        case CpuDataTargetKind::kSpImem:
        case CpuDataTargetKind::kSpMmio:
        case CpuDataTargetKind::kMi:
        case CpuDataTargetKind::kPi:
        case CpuDataTargetKind::kSi:
          fail_unsupported_cpu_data_access("LL", effective_address, 4);
      }

      throw std::logic_error("unknown LL data target");
    }

    case CpuInstructionIdentity::kLwu: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x3u) != 0) {
        fail_unaligned_word_memory_access(
            "LWU",
            effective_address,
            MachineFaultAccessIntent::kDataRead);
      }

      const std::uint32_t value = read_cpu_memory_u32_be(effective_address);
      write_cpu_gpr_word_zero_extended_result(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLwr: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t aligned_address = effective_address & ~0x3u;
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = byte_offset + 1u;
      const std::uint32_t first_register_byte = 4u - byte_count;

      static_cast<void>(require_cpu_data_target(
          "LWR effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataRead));

      const CpuRegisterValue previous_value = read_cpu_gpr_value(instruction.rt);
      std::uint32_t value = static_cast<std::uint32_t>(previous_value);
      for (std::uint32_t i = 0; i < byte_count; ++i) {
        const std::uint8_t memory_byte = read_cpu_memory_u8(aligned_address + i);
        value = replace_u32_byte_be(
            value,
            static_cast<std::size_t>(first_register_byte + i),
            memory_byte);
      }

      // LWR only replaces byte 0 when offset 3 loads all four bytes. Partial
      // LWR rows keep the prior high 32 bits while replacing the low-word lanes.
      if (byte_offset == 3u) {
        write_cpu_gpr_partial_word_sign_extended_result(instruction.rt, value);
      } else {
        write_cpu_gpr_partial_word_preserve_high_result(
            instruction.rt,
            value,
            previous_value);
      }
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLdl: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t byte_offset = effective_address & 0x7u;
      const std::uint32_t byte_count = 8u - byte_offset;

      static_cast<void>(require_cpu_data_target(
          "LDL effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataRead));

      CpuRegisterValue value = read_cpu_gpr_value(instruction.rt);
      for (std::uint32_t i = 0; i < byte_count; ++i) {
        const std::uint8_t memory_byte = read_cpu_memory_u8(effective_address + i);
        value = replace_cpu_value_byte_be(value, static_cast<std::size_t>(i), memory_byte);
      }

      write_cpu_gpr_value(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLdr: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const CpuAddress aligned_address = effective_address & ~0x7u;
      const std::uint32_t byte_offset = effective_address & 0x7u;
      const std::uint32_t byte_count = byte_offset + 1u;
      const std::uint32_t first_register_byte = 8u - byte_count;

      static_cast<void>(require_cpu_data_target(
          "LDR effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataRead));

      CpuRegisterValue value = read_cpu_gpr_value(instruction.rt);
      for (std::uint32_t i = 0; i < byte_count; ++i) {
        const std::uint8_t memory_byte = read_cpu_memory_u8(aligned_address + i);
        value = replace_cpu_value_byte_be(
            value,
            static_cast<std::size_t>(first_register_byte + i),
            memory_byte);
      }

      write_cpu_gpr_value(instruction.rt, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLd: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x7u) != 0) {
        fail_unaligned_doubleword_memory_access(
            "LD",
            effective_address,
            MachineFaultAccessIntent::kDataRead);
      }

      write_cpu_gpr_value(instruction.rt, read_cpu_memory_u64_be(effective_address));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kLld: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x7u) != 0) {
        fail_unaligned_doubleword_memory_access(
            "LLD",
            effective_address,
            MachineFaultAccessIntent::kDataRead);
      }

      const CpuDataTarget target = require_cpu_data_target(
          "LLD",
          effective_address,
          8,
          MachineFaultAccessIntent::kDataRead);
      switch (target.kind) {
        case CpuDataTargetKind::kRdram: {
          const CpuRegisterValue value = read_rdram_u64_be(target.rdram_offset);
          write_cpu_gpr_value(instruction.rt, value);
          set_cpu_rdram_reservation(target.rdram_offset, 8);
          return CpuInstructionExecutionResult::kExecuted;
        }

        case CpuDataTargetKind::kSpDmem:
        case CpuDataTargetKind::kSpImem:
        case CpuDataTargetKind::kSpMmio:
        case CpuDataTargetKind::kMi:
        case CpuDataTargetKind::kPi:
        case CpuDataTargetKind::kSi:
          fail_unsupported_cpu_data_access("LLD", effective_address, 8);
      }

      throw std::logic_error("unknown LLD data target");
    }

    case CpuInstructionIdentity::kSb: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint8_t value =
          static_cast<std::uint8_t>(read_cpu_gpr_word(instruction.rt) & 0xffu);
      write_cpu_memory_u8(effective_address, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSh: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x1u) != 0) {
        fail_unaligned_halfword_memory_access(
            "SH",
            effective_address,
            MachineFaultAccessIntent::kDataWrite);
      }

      const std::uint16_t value =
          static_cast<std::uint16_t>(read_cpu_gpr_word(instruction.rt) & 0xffffu);
      write_cpu_memory_u16_be(effective_address, value);
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSwl: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = 4u - byte_offset;
      const std::uint32_t value = read_cpu_gpr_word(instruction.rt);

      static_cast<void>(require_cpu_data_target(
          "SWL effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataWrite));

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        static_cast<void>(require_cpu_data_target(
            "SWL byte preflight",
            effective_address + i,
            1,
            MachineFaultAccessIntent::kDataWrite));
      }

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        write_cpu_memory_u8(
            effective_address + i,
            u32_byte_be(value, static_cast<std::size_t>(i)));
      }

      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSw: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x3u) != 0) {
        fail_unaligned_word_memory_access(
            "SW",
            effective_address,
            MachineFaultAccessIntent::kDataWrite);
      }

      write_cpu_memory_u32_be(effective_address, read_cpu_gpr_word(instruction.rt));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSc: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t source_value = read_cpu_gpr_word(instruction.rt);

      if ((effective_address & 0x3u) != 0) {
        fail_unaligned_word_memory_access(
            "SC",
            effective_address,
            MachineFaultAccessIntent::kDataWrite);
      }

      const CpuDataTarget target = require_cpu_data_target(
          "SC",
          effective_address,
          4,
          MachineFaultAccessIntent::kDataWrite);
      switch (target.kind) {
        case CpuDataTargetKind::kRdram: {
          const bool reservation_matched =
              cpu_rdram_reservation_matches(target.rdram_offset, 4);
          clear_cpu_rdram_reservation();

          if (reservation_matched) {
            write_rdram_u32_be(target.rdram_offset, source_value);
          }

          write_cpu_gpr_value(instruction.rt, reservation_matched ? 1u : 0u);
          return CpuInstructionExecutionResult::kExecuted;
        }

        case CpuDataTargetKind::kSpDmem:
        case CpuDataTargetKind::kSpImem:
        case CpuDataTargetKind::kSpMmio:
        case CpuDataTargetKind::kMi:
        case CpuDataTargetKind::kPi:
        case CpuDataTargetKind::kSi:
          fail_unsupported_cpu_data_access("SC", effective_address, 4);
      }

      throw std::logic_error("unknown SC data target");
    }

    case CpuInstructionIdentity::kSwr: {
      const std::uint32_t effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t aligned_address = effective_address & ~0x3u;
      const std::uint32_t byte_offset = effective_address & 0x3u;
      const std::uint32_t byte_count = byte_offset + 1u;
      const std::uint32_t first_register_byte = 4u - byte_count;
      const std::uint32_t value = read_cpu_gpr_word(instruction.rt);

      static_cast<void>(require_cpu_data_target(
          "SWR effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataWrite));

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        static_cast<void>(require_cpu_data_target(
            "SWR byte preflight",
            aligned_address + i,
            1,
            MachineFaultAccessIntent::kDataWrite));
      }

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        write_cpu_memory_u8(
            aligned_address + i,
            u32_byte_be(value, static_cast<std::size_t>(first_register_byte + i)));
      }

      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSdl: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const std::uint32_t byte_offset = effective_address & 0x7u;
      const std::uint32_t byte_count = 8u - byte_offset;
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt);

      static_cast<void>(require_cpu_data_target(
          "SDL effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataWrite));

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        static_cast<void>(require_cpu_data_target(
            "SDL byte preflight",
            effective_address + i,
            1,
            MachineFaultAccessIntent::kDataWrite));
      }

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        write_cpu_memory_u8(
            effective_address + i,
            cpu_value_byte_be(value, static_cast<std::size_t>(i)));
      }

      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSdr: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const CpuAddress aligned_address = effective_address & ~0x7u;
      const std::uint32_t byte_offset = effective_address & 0x7u;
      const std::uint32_t byte_count = byte_offset + 1u;
      const std::uint32_t first_register_byte = 8u - byte_count;
      const CpuRegisterValue value = read_cpu_gpr_value(instruction.rt);

      static_cast<void>(require_cpu_data_target(
          "SDR effective preflight",
          effective_address,
          1,
          MachineFaultAccessIntent::kDataWrite));

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        static_cast<void>(require_cpu_data_target(
            "SDR byte preflight",
            aligned_address + i,
            1,
            MachineFaultAccessIntent::kDataWrite));
      }

      for (std::uint32_t i = 0; i < byte_count; ++i) {
        write_cpu_memory_u8(
            aligned_address + i,
            cpu_value_byte_be(value, static_cast<std::size_t>(first_register_byte + i)));
      }

      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kSd: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);

      if ((effective_address & 0x7u) != 0) {
        fail_unaligned_doubleword_memory_access(
            "SD",
            effective_address,
            MachineFaultAccessIntent::kDataWrite);
      }

      write_cpu_memory_u64_be(effective_address, read_cpu_gpr_value(instruction.rt));
      return CpuInstructionExecutionResult::kExecuted;
    }

    case CpuInstructionIdentity::kScd: {
      const CpuAddress effective_address =
          read_cpu_gpr_word(instruction.rs) +
          sign_extend_u16_to_u32(instruction.immediate_u16);
      const CpuRegisterValue source_value = read_cpu_gpr_value(instruction.rt);

      if ((effective_address & 0x7u) != 0) {
        fail_unaligned_doubleword_memory_access(
            "SCD",
            effective_address,
            MachineFaultAccessIntent::kDataWrite);
      }

      const CpuDataTarget target = require_cpu_data_target(
          "SCD",
          effective_address,
          8,
          MachineFaultAccessIntent::kDataWrite);
      switch (target.kind) {
        case CpuDataTargetKind::kRdram: {
          const bool reservation_matched =
              cpu_rdram_reservation_matches(target.rdram_offset, 8);
          clear_cpu_rdram_reservation();

          if (reservation_matched) {
            write_rdram_u64_be(target.rdram_offset, source_value);
          }

          write_cpu_gpr_value(instruction.rt, reservation_matched ? 1u : 0u);
          return CpuInstructionExecutionResult::kExecuted;
        }

        case CpuDataTargetKind::kSpDmem:
        case CpuDataTargetKind::kSpImem:
        case CpuDataTargetKind::kSpMmio:
        case CpuDataTargetKind::kMi:
        case CpuDataTargetKind::kPi:
        case CpuDataTargetKind::kSi:
          fail_unsupported_cpu_data_access("SCD", effective_address, 8);
      }

      throw std::logic_error("unknown SCD data target");
    }

    default:
      return CpuInstructionExecutionResult::kUnsupported;
  }
}

Machine::CpuInstructionStepResult Machine::step_cpu_instruction() {
  // Step cadence: fetch uses the current pc. Ordinary execution commits
  // pc = old next_pc and next_pc = old next_pc + 4. Control-flow execution
  // runs while old pc is still current and writes the pending next_pc target,
  // so the next step fetches the delay slot and the following step fetches the
  // target. A not-taken branch-likely annuls the delay slot by committing
  // pc = old next_pc + 4. Unsupported identities and execution-time faults
  // restore old pc/next_pc; fetch faults happen before speculative mutation.
  // kStopped uses the same committed pc/next_pc cadence as an executed
  // instruction. Link instructions write during execute, with register-target
  // links reading the target before any link writeback.
  // kInterrupted is a local pre-fetch interrupt entry: no instruction is
  // fetched or executed from the interrupted PC, EPC stores that PC, EXL is
  // set, and pc/next_pc move to the local vector.
  // kException is a local signed-overflow or earned address-error exception
  // entry with EXL clear: the faulting instruction does not commit, Count does
  // not advance, EPC stores the faulting PC or the preceding branch/control PC
  // for the narrow earned delay-slot case, the earned COP0 state is updated,
  // EXL is set, and pc/next_pc move to the local vector.
  // ERET is a narrow return from that local entry only. It runs before normal
  // speculative pc/next_pc movement so the support check uses the current
  // cadence, not a delay-slot/general-exception model.
  if (try_enter_local_interrupt()) {
    return CpuInstructionStepResult::kInterrupted;
  }

  const std::uint32_t current_pc = cpu_pc_;
  const std::uint32_t current_next_pc = cpu_next_pc_;

  std::uint32_t raw = 0;
  try {
    raw = fetch_cpu_instruction_word();
  } catch (const MachineFault& fault) {
    if (fault.kind() == MachineFaultKind::kUnalignedInstructionFetch &&
        fault.access_intent() == MachineFaultAccessIntent::kInstructionFetch &&
        local_synchronous_exception_entry_allowed(current_pc, current_next_pc)) {
      enter_local_address_error_exception(
          current_pc,
          fault.cpu_address(),
          kCop0ExceptionCodeAddressErrorLoad,
          false);
      return CpuInstructionStepResult::kException;
    }

    if (fault.kind() == MachineFaultKind::kCpuRdramAddressRejected &&
        fault.access_intent() == MachineFaultAccessIntent::kInstructionFetchDirectTargetMiss &&
        local_synchronous_exception_entry_allowed(current_pc, current_next_pc)) {
      enter_local_address_error_exception(
          current_pc,
          fault.cpu_address(),
          kCop0ExceptionCodeAddressErrorLoad,
          false);
      return CpuInstructionStepResult::kException;
    }

    throw;
  }
  const DecodedCpuInstructionWord instruction = decode_cpu_instruction_word(raw);
  const CpuInstructionIdentity identity = identify_cpu_instruction(instruction);

  if (identity == CpuInstructionIdentity::kCop0Eret) {
    if (!local_eret_can_return()) {
      return CpuInstructionStepResult::kUnsupported;
    }

    return_from_local_interrupt_entry();
    advance_cop0_count_after_committed_instruction();
    return CpuInstructionStepResult::kStepped;
  }

  cpu_next_pc_ = sequential_instruction_address(current_next_pc);

  CpuInstructionExecutionResult execution_result = CpuInstructionExecutionResult::kUnsupported;
  try {
    execution_result = execute_cpu_instruction(identity, instruction);
  } catch (const MachineFault& fault) {
    // Restore public control-flow state before either rethrowing the local
    // fault or converting the one earned synchronous source into local COP0
    // exception entry.
    cpu_pc_ = current_pc;
    cpu_next_pc_ = current_next_pc;

    if (fault.kind() == MachineFaultKind::kSignedArithmeticOverflow) {
      if (local_signed_overflow_exception_entry_allowed(current_pc, current_next_pc)) {
        enter_local_signed_overflow_exception(current_pc, false);
        return CpuInstructionStepResult::kException;
      }

      if (local_delay_slot_synchronous_exception_entry_allowed(
              current_pc,
              current_next_pc)) {
        enter_local_signed_overflow_exception(current_pc, true);
        return CpuInstructionStepResult::kException;
      }
    }

    if (fault.kind() == MachineFaultKind::kUnalignedCpuMemoryAccess ||
        fault.kind() == MachineFaultKind::kCpuRdramAddressRejected) {
      const bool ordinary_exception_entry =
          local_synchronous_exception_entry_allowed(current_pc, current_next_pc);
      const bool delay_slot_exception_entry =
          local_delay_slot_synchronous_exception_entry_allowed(
              current_pc,
              current_next_pc);

      if (fault.access_intent() == MachineFaultAccessIntent::kDataRead) {
        if (ordinary_exception_entry || delay_slot_exception_entry) {
          enter_local_address_error_exception(
              current_pc,
              fault.cpu_address(),
              kCop0ExceptionCodeAddressErrorLoad,
              delay_slot_exception_entry);
          return CpuInstructionStepResult::kException;
        }
      }

      if (fault.access_intent() == MachineFaultAccessIntent::kDataWrite) {
        if (ordinary_exception_entry || delay_slot_exception_entry) {
          enter_local_address_error_exception(
              current_pc,
              fault.cpu_address(),
              kCop0ExceptionCodeAddressErrorStore,
              delay_slot_exception_entry);
          return CpuInstructionStepResult::kException;
        }
      }
    }

    if (fault.kind() == MachineFaultKind::kUnalignedControlTransferTarget) {
      const bool ordinary_exception_entry =
          local_synchronous_exception_entry_allowed(current_pc, current_next_pc);
      const bool delay_slot_exception_entry =
          local_delay_slot_synchronous_exception_entry_allowed(
              current_pc,
              current_next_pc);
      if (ordinary_exception_entry || delay_slot_exception_entry) {
        enter_local_address_error_exception(
            current_pc,
            fault.cpu_address(),
            kCop0ExceptionCodeAddressErrorLoad,
            delay_slot_exception_entry);
        return CpuInstructionStepResult::kException;
      }
    }

    throw;
  } catch (...) {
    // Restore public control-flow state before rethrowing a local Machine
    // fault or any other execution-time failure from the step boundary.
    cpu_pc_ = current_pc;
    cpu_next_pc_ = current_next_pc;
    throw;
  }

  if (execution_result == CpuInstructionExecutionResult::kUnsupported) {
    // Unsupported/unknown instructions are reported without committing the
    // speculative pc/next_pc movement prepared for a normal local step.
    cpu_pc_ = current_pc;
    cpu_next_pc_ = current_next_pc;
    return CpuInstructionStepResult::kUnsupported;
  }

  if (execution_result == CpuInstructionExecutionResult::kBranchLikelyNotTaken) {
    const std::uint32_t skipped_delay_slot_pc = cpu_next_pc_;
    cpu_pc_ = skipped_delay_slot_pc;
    cpu_next_pc_ = sequential_instruction_address(skipped_delay_slot_pc);
    advance_cop0_count_after_committed_instruction();
    return CpuInstructionStepResult::kStepped;
  }

  cpu_pc_ = current_next_pc;
  advance_cop0_count_after_committed_instruction();

  if (execution_result == CpuInstructionExecutionResult::kStopped) {
    return CpuInstructionStepResult::kStopped;
  }

  return CpuInstructionStepResult::kStepped;
}

}  // namespace fn64
