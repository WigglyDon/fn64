#include "machine.hpp"

#include <stdexcept>
#include <string>
#include <utility>

namespace fn64 {
namespace {

[[noreturn]] void fail_rdram_access(RdramOffset address, std::size_t width) {
  throw std::out_of_range(
      "RDRAM access out of range: address=" + std::to_string(address) +
      " width=" + std::to_string(width));
}

[[noreturn]] void fail_cartridge_stage_span(
    const char* span_name,
    std::uint32_t start,
    std::uint32_t byte_count) {
  throw std::out_of_range(
      std::string("cartridge staging span overflows 32-bit address space: ") +
      span_name +
      " start=" + std::to_string(start) +
      " byte_count=" + std::to_string(byte_count));
}

[[noreturn]] void fail_cartridge_stage_range(
    const char* span_name,
    std::uint32_t start,
    std::uint32_t byte_count,
    std::size_t size) {
  throw std::out_of_range(
      std::string("cartridge staging span out of range: ") +
      span_name +
      " start=" + std::to_string(start) +
      " byte_count=" + std::to_string(byte_count) +
      " size=" + std::to_string(size));
}

void require_u32_span(
    const char* span_name,
    std::uint32_t start,
    std::uint32_t byte_count) {
  if (byte_count == 0) {
    return;
  }

  const std::uint64_t last = static_cast<std::uint64_t>(start) +
                             static_cast<std::uint64_t>(byte_count - 1u);
  if (last > 0xffffffffull) {
    fail_cartridge_stage_span(span_name, start, byte_count);
  }
}

void require_stage_span_inside_buffer(
    const char* span_name,
    std::uint32_t start,
    std::uint32_t byte_count,
    std::size_t size) {
  if (byte_count == 0) {
    return;
  }

  const std::uint64_t last = static_cast<std::uint64_t>(start) +
                             static_cast<std::uint64_t>(byte_count - 1u);
  if (last >= static_cast<std::uint64_t>(size)) {
    fail_cartridge_stage_range(span_name, start, byte_count, size);
  }
}

}  // namespace

MachineFault::MachineFault(
    MachineFaultKind kind,
    std::string operation,
    CpuAddress cpu_address,
    std::size_t access_size,
    std::string message,
    MachineFaultAccessIntent access_intent)
    : std::runtime_error(std::move(message)),
      kind_(kind),
      operation_(std::move(operation)),
      cpu_address_(cpu_address),
      access_size_(access_size),
      access_intent_(access_intent) {}

MachineFaultKind MachineFault::kind() const noexcept {
  return kind_;
}

const std::string& MachineFault::operation() const noexcept {
  return operation_;
}

CpuAddress MachineFault::cpu_address() const noexcept {
  return cpu_address_;
}

std::size_t MachineFault::access_size() const noexcept {
  return access_size_;
}

MachineFaultAccessIntent MachineFault::access_intent() const noexcept {
  return access_intent_;
}

Machine::Machine(Cartridge cartridge)
    : cartridge_(std::move(cartridge)) {
  reset_to_non_boot_power_on_state();
}

void Machine::reset_to_non_boot_power_on_state() {
  powered_on_ = true;
  rdram_.fill(0);
  sp_dmem_.fill(0);
  sp_imem_.fill(0);
  pif_ram_.fill(0);
  clear_cpu_rdram_reservation();
  sp_mem_address_ = 0;
  sp_dram_address_ = 0;
  sp_rd_len_ = 0;
  sp_wr_len_ = 0;
  sp_status_ = 0;
  mi_interrupt_pending_ = 0;
  mi_interrupt_mask_ = 0;
  cop0_count_ = 0;
  cop0_compare_ = 0;
  cop0_timer_interrupt_pending_ = false;
  cop0_status_ = 0;
  cop0_software_interrupt_pending_ = 0;
  cop0_epc_ = 0;
  cop0_bad_vaddr_ = 0;
  cop0_exception_code_ = 0;
  cop0_exception_branch_delay_ = false;
  pi_dram_address_ = 0;
  pi_cart_address_ = 0;
  pi_cart_to_rdram_length_ = 0;
  pi_status_ = 0;
  ai_dram_address_ = 0;
  ai_length_ = 0;
  ai_status_ = 0;
  si_dram_address_ = 0;
  si_pif_to_dram_address_ = 0;
  si_dram_to_pif_address_ = 0;
  si_status_ = 0;
  cpu_pc_ = kNonBootResetVectorPc;
  cpu_next_pc_ = kNonBootResetVectorNextPc;
  cpu_hi_ = 0;
  cpu_lo_ = 0;
  cpu_gprs_.fill(0);
}

bool Machine::powered_on() const {
  return powered_on_;
}

const Cartridge& Machine::cartridge() const {
  return cartridge_;
}

std::size_t Machine::rdram_size_bytes() const noexcept {
  return rdram_.size();
}

bool Machine::translate_direct_cpu_physical_address(
    CpuAddress cpu_address,
    CpuPhysicalAddress& out_physical_address) noexcept {
  constexpr CpuAddress kDirectSegmentMask = 0xe0000000u;
  constexpr CpuAddress kDirectSegmentOffsetMask = 0x1fffffffu;
  constexpr CpuAddress kKseg0RdramBase = 0x80000000u;
  constexpr CpuAddress kKseg1RdramBase = 0xa0000000u;

  const CpuAddress direct_segment = cpu_address & kDirectSegmentMask;
  if (direct_segment != kKseg0RdramBase && direct_segment != kKseg1RdramBase) {
    return false;
  }

  out_physical_address =
      static_cast<CpuPhysicalAddress>(cpu_address & kDirectSegmentOffsetMask);
  return true;
}

bool Machine::is_unavailable_pif_rom_reset_fetch(
    CpuAddress cpu_address,
    CpuPhysicalAddress physical_address) noexcept {
  return cpu_address == kNonBootResetVectorPc &&
         physical_address == kUnavailablePifRomResetPhysicalAddress;
}

bool Machine::translate_cpu_rdram_address(
    CpuAddress cpu_address,
    std::size_t width,
    RdramOffset& out_rdram_address) noexcept {
  CpuPhysicalAddress physical_address = 0;
  if (!translate_direct_cpu_physical_address(cpu_address, physical_address)) {
    return false;
  }

  return translate_cpu_physical_rdram_address(
      physical_address,
      width,
      out_rdram_address);
}

bool Machine::translate_cpu_physical_rdram_address(
    CpuPhysicalAddress physical_address,
    std::size_t width,
    RdramOffset& out_rdram_address) noexcept {
  if (width == 0 || width > kRdramSizeBytes) {
    return false;
  }

  const std::size_t offset = static_cast<std::size_t>(physical_address);
  const std::size_t last_offset = kRdramSizeBytes - width;
  if (offset > last_offset) {
    return false;
  }

  out_rdram_address = static_cast<RdramOffset>(physical_address);
  return true;
}

bool Machine::translate_cpu_physical_sp_memory_address(
    CpuPhysicalAddress physical_address,
    std::size_t width,
    CpuDataTargetKind& out_kind,
    std::uint32_t& out_sp_offset) noexcept {
  if (width == 0 || width > kSpMemorySizeBytes) {
    return false;
  }

  const auto translate_span =
      [physical_address, width](
          CpuPhysicalAddress base,
          CpuDataTargetKind kind,
          CpuDataTargetKind& out_span_kind,
          std::uint32_t& out_span_offset) noexcept {
        if (physical_address < base) {
          return false;
        }

        const std::uint32_t offset = physical_address - base;
        if (static_cast<std::size_t>(offset) > kSpMemorySizeBytes - width) {
          return false;
        }

        out_span_kind = kind;
        out_span_offset = offset;
        return true;
      };

  if (translate_span(
          kSpDmemPhysicalBase,
          CpuDataTargetKind::kSpDmem,
          out_kind,
          out_sp_offset)) {
    return true;
  }

  return translate_span(
      kSpImemPhysicalBase,
      CpuDataTargetKind::kSpImem,
      out_kind,
      out_sp_offset);
}

bool Machine::translate_cpu_physical_sp_register_address(
    CpuPhysicalAddress physical_address,
    std::uint32_t& out_register_offset) noexcept {
  if (physical_address < kSpRegisterPhysicalBase) {
    return false;
  }

  const std::uint32_t register_offset = physical_address - kSpRegisterPhysicalBase;
  if (register_offset >= kSpRegisterWindowSize) {
    return false;
  }

  out_register_offset = register_offset;
  return true;
}

bool Machine::translate_cpu_physical_pi_register_address(
    CpuPhysicalAddress physical_address,
    std::uint32_t& out_register_offset) noexcept {
  if (physical_address < kPiPhysicalBase) {
    return false;
  }

  const std::uint32_t register_offset = physical_address - kPiPhysicalBase;
  if (register_offset >= kPiRegisterWindowSize) {
    return false;
  }

  out_register_offset = register_offset;
  return true;
}

bool Machine::translate_cpu_physical_ai_register_address(
    CpuPhysicalAddress physical_address,
    std::uint32_t& out_register_offset) noexcept {
  if (physical_address < kAiPhysicalBase) {
    return false;
  }

  const std::uint32_t register_offset = physical_address - kAiPhysicalBase;
  if (register_offset >= kAiRegisterWindowSize) {
    return false;
  }

  out_register_offset = register_offset;
  return true;
}

bool Machine::translate_cpu_physical_mi_register_address(
    CpuPhysicalAddress physical_address,
    std::uint32_t& out_register_offset) noexcept {
  if (physical_address < kMiPhysicalBase) {
    return false;
  }

  const std::uint32_t register_offset = physical_address - kMiPhysicalBase;
  if (register_offset >= kMiRegisterWindowSize) {
    return false;
  }

  out_register_offset = register_offset;
  return true;
}

bool Machine::translate_cpu_physical_si_register_address(
    CpuPhysicalAddress physical_address,
    std::uint32_t& out_register_offset) noexcept {
  if (physical_address < kSiPhysicalBase) {
    return false;
  }

  const std::uint32_t register_offset = physical_address - kSiPhysicalBase;
  if (register_offset >= kSiRegisterWindowSize) {
    return false;
  }

  out_register_offset = register_offset;
  return true;
}

std::uint8_t Machine::read_rdram_u8(RdramOffset address) const {
  if (address >= rdram_.size()) {
    fail_rdram_access(address, 1);
  }

  return rdram_[address];
}

std::uint16_t Machine::read_rdram_u16_be(RdramOffset address) const {
  if (address > rdram_.size() - 2) {
    fail_rdram_access(address, 2);
  }

  return static_cast<std::uint16_t>(
      (static_cast<std::uint16_t>(rdram_[address]) << 8) |
      static_cast<std::uint16_t>(rdram_[address + 1]));
}

std::uint32_t Machine::inspect_rdram_u32_be(RdramOffset address) const {
  return read_rdram_u32_be(address);
}

std::uint32_t Machine::read_rdram_u32_be(RdramOffset address) const {
  if (address > rdram_.size() - 4) {
    fail_rdram_access(address, 4);
  }

  return (static_cast<std::uint32_t>(rdram_[address]) << 24) |
         (static_cast<std::uint32_t>(rdram_[address + 1]) << 16) |
         (static_cast<std::uint32_t>(rdram_[address + 2]) << 8) |
         static_cast<std::uint32_t>(rdram_[address + 3]);
}

CpuRegisterValue Machine::read_rdram_u64_be(RdramOffset address) const {
  if (address > rdram_.size() - 8) {
    fail_rdram_access(address, 8);
  }

  return (static_cast<CpuRegisterValue>(rdram_[address]) << 56) |
         (static_cast<CpuRegisterValue>(rdram_[address + 1]) << 48) |
         (static_cast<CpuRegisterValue>(rdram_[address + 2]) << 40) |
         (static_cast<CpuRegisterValue>(rdram_[address + 3]) << 32) |
         (static_cast<CpuRegisterValue>(rdram_[address + 4]) << 24) |
         (static_cast<CpuRegisterValue>(rdram_[address + 5]) << 16) |
         (static_cast<CpuRegisterValue>(rdram_[address + 6]) << 8) |
         static_cast<CpuRegisterValue>(rdram_[address + 7]);
}

void Machine::clear_cpu_rdram_reservation() noexcept {
  cpu_rdram_reservation_ = {};
}

void Machine::set_cpu_rdram_reservation(RdramOffset address, std::size_t width) noexcept {
  cpu_rdram_reservation_.valid = true;
  cpu_rdram_reservation_.rdram_offset = address;
  cpu_rdram_reservation_.width = width;
}

bool Machine::cpu_rdram_reservation_matches(
    RdramOffset address,
    std::size_t width) const noexcept {
  return cpu_rdram_reservation_.valid &&
         cpu_rdram_reservation_.rdram_offset == address &&
         cpu_rdram_reservation_.width == width;
}

void Machine::invalidate_cpu_rdram_reservation_for_write(
    RdramOffset address,
    std::size_t width) noexcept {
  if (!cpu_rdram_reservation_.valid || width == 0) {
    return;
  }

  const std::uint64_t write_begin = static_cast<std::uint64_t>(address);
  const std::uint64_t write_end = write_begin + static_cast<std::uint64_t>(width);
  const std::uint64_t reservation_begin =
      static_cast<std::uint64_t>(cpu_rdram_reservation_.rdram_offset);
  const std::uint64_t reservation_end =
      reservation_begin + static_cast<std::uint64_t>(cpu_rdram_reservation_.width);

  if (write_begin < reservation_end && reservation_begin < write_end) {
    clear_cpu_rdram_reservation();
  }
}

void Machine::write_rdram_u8(RdramOffset address, std::uint8_t value) {
  if (address >= rdram_.size()) {
    fail_rdram_access(address, 1);
  }

  invalidate_cpu_rdram_reservation_for_write(address, 1);
  rdram_[address] = value;
}

void Machine::write_rdram_u16_be(RdramOffset address, std::uint16_t value) {
  if (address > rdram_.size() - 2) {
    fail_rdram_access(address, 2);
  }

  invalidate_cpu_rdram_reservation_for_write(address, 2);
  rdram_[address] = static_cast<std::uint8_t>((value >> 8) & 0xff);
  rdram_[address + 1] = static_cast<std::uint8_t>(value & 0xff);
}

void Machine::stage_rdram_u32_be(RdramOffset address, std::uint32_t value) {
  write_rdram_u32_be(address, value);
}

void Machine::write_rdram_u32_be(RdramOffset address, std::uint32_t value) {
  if (address > rdram_.size() - 4) {
    fail_rdram_access(address, 4);
  }

  invalidate_cpu_rdram_reservation_for_write(address, 4);
  rdram_[address] = static_cast<std::uint8_t>((value >> 24) & 0xff);
  rdram_[address + 1] = static_cast<std::uint8_t>((value >> 16) & 0xff);
  rdram_[address + 2] = static_cast<std::uint8_t>((value >> 8) & 0xff);
  rdram_[address + 3] = static_cast<std::uint8_t>(value & 0xff);
}

void Machine::write_rdram_u64_be(RdramOffset address, CpuRegisterValue value) {
  if (address > rdram_.size() - 8) {
    fail_rdram_access(address, 8);
  }

  invalidate_cpu_rdram_reservation_for_write(address, 8);
  rdram_[address] = static_cast<std::uint8_t>((value >> 56) & 0xff);
  rdram_[address + 1] = static_cast<std::uint8_t>((value >> 48) & 0xff);
  rdram_[address + 2] = static_cast<std::uint8_t>((value >> 40) & 0xff);
  rdram_[address + 3] = static_cast<std::uint8_t>((value >> 32) & 0xff);
  rdram_[address + 4] = static_cast<std::uint8_t>((value >> 24) & 0xff);
  rdram_[address + 5] = static_cast<std::uint8_t>((value >> 16) & 0xff);
  rdram_[address + 6] = static_cast<std::uint8_t>((value >> 8) & 0xff);
  rdram_[address + 7] = static_cast<std::uint8_t>(value & 0xff);
}

void Machine::stage_cartridge_bytes_to_rdram(
    CartridgeOffset cartridge_offset,
    RdramOffset rdram_address,
    std::uint32_t byte_count) {
  require_u32_span("cartridge source", cartridge_offset, byte_count);
  require_u32_span("RDRAM destination", rdram_address, byte_count);
  require_stage_span_inside_buffer(
      "cartridge source",
      cartridge_offset,
      byte_count,
      cartridge_.size_bytes());
  require_stage_span_inside_buffer(
      "RDRAM destination",
      rdram_address,
      byte_count,
      rdram_.size());

  for (std::uint32_t i = 0; i < byte_count; ++i) {
    write_rdram_u8(rdram_address + i, cartridge_.read_u8(cartridge_offset + i));
  }
}

void Machine::stage_cartridge_ipl3_candidate_to_sp_dmem() {
  require_stage_span_inside_buffer(
      "cartridge IPL3 candidate source",
      kCartridgeCandidateIpl3StartOffset,
      kCartridgeCandidateIpl3ByteCount,
      cartridge_.size_bytes());
  require_stage_span_inside_buffer(
      "SP DMEM IPL3 candidate destination",
      kCartridgeCandidateIpl3StartOffset,
      kCartridgeCandidateIpl3ByteCount,
      sp_dmem_.size());

  for (std::uint32_t i = 0; i < kCartridgeCandidateIpl3ByteCount; ++i) {
    sp_dmem_[kCartridgeCandidateIpl3StartOffset + i] =
        cartridge_.read_u8(kCartridgeCandidateIpl3StartOffset + i);
  }
}

void Machine::enter_sp_dmem_ipl3_candidate() {
  cpu_pc_ = kSpDmemIpl3CandidateEntryPc;
  cpu_next_pc_ = kSpDmemIpl3CandidateEntryNextPc;
}

}  // namespace fn64
