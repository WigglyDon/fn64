#include "machine.hpp"

#include <stdexcept>
#include <string>
#include <utility>

namespace fn64 {
namespace {

[[noreturn]] void fail_rdram_access(std::uint32_t address, std::size_t width) {
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
    std::uint32_t cpu_address,
    std::size_t access_size,
    std::string message)
    : std::runtime_error(std::move(message)),
      kind_(kind),
      operation_(std::move(operation)),
      cpu_address_(cpu_address),
      access_size_(access_size) {}

MachineFaultKind MachineFault::kind() const noexcept {
  return kind_;
}

const std::string& MachineFault::operation() const noexcept {
  return operation_;
}

std::uint32_t MachineFault::cpu_address() const noexcept {
  return cpu_address_;
}

std::size_t MachineFault::access_size() const noexcept {
  return access_size_;
}

Machine::Machine(Cartridge cartridge)
    : cartridge_(std::move(cartridge)) {
  reset_to_blank_rdram_power_on_state();
}

void Machine::reset_to_blank_rdram_power_on_state() {
  powered_on_ = true;
  rdram_.fill(0);
  cpu_pc_ = kBlankInitialCpuPc;
  cpu_next_pc_ = kBlankInitialCpuNextPc;
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

bool Machine::translate_cpu_rdram_address(
    std::uint32_t cpu_address,
    std::size_t width,
    std::uint32_t& out_rdram_address) noexcept {
  if (width == 0 || width > kRdramSizeBytes) {
    return false;
  }

  constexpr std::uint32_t kDirectSegmentMask = 0xe0000000u;
  constexpr std::uint32_t kDirectSegmentOffsetMask = 0x1fffffffu;
  constexpr std::uint32_t kKseg0RdramBase = 0x80000000u;
  constexpr std::uint32_t kKseg1RdramBase = 0xa0000000u;

  const std::uint32_t direct_segment = cpu_address & kDirectSegmentMask;
  if (direct_segment != kKseg0RdramBase && direct_segment != kKseg1RdramBase) {
    return false;
  }

  const std::size_t offset =
      static_cast<std::size_t>(cpu_address & kDirectSegmentOffsetMask);
  const std::size_t last_offset = kRdramSizeBytes - width;
  if (offset > last_offset) {
    return false;
  }

  out_rdram_address = static_cast<std::uint32_t>(offset);
  return true;
}

std::uint8_t Machine::read_rdram_u8(std::uint32_t address) const {
  if (address >= rdram_.size()) {
    fail_rdram_access(address, 1);
  }

  return rdram_[address];
}

std::uint16_t Machine::read_rdram_u16_be(std::uint32_t address) const {
  if (address > rdram_.size() - 2) {
    fail_rdram_access(address, 2);
  }

  return static_cast<std::uint16_t>(
      (static_cast<std::uint16_t>(rdram_[address]) << 8) |
      static_cast<std::uint16_t>(rdram_[address + 1]));
}

std::uint32_t Machine::inspect_rdram_u32_be(std::uint32_t address) const {
  return read_rdram_u32_be(address);
}

std::uint32_t Machine::read_rdram_u32_be(std::uint32_t address) const {
  if (address > rdram_.size() - 4) {
    fail_rdram_access(address, 4);
  }

  return (static_cast<std::uint32_t>(rdram_[address]) << 24) |
         (static_cast<std::uint32_t>(rdram_[address + 1]) << 16) |
         (static_cast<std::uint32_t>(rdram_[address + 2]) << 8) |
         static_cast<std::uint32_t>(rdram_[address + 3]);
}

void Machine::write_rdram_u8(std::uint32_t address, std::uint8_t value) {
  if (address >= rdram_.size()) {
    fail_rdram_access(address, 1);
  }

  rdram_[address] = value;
}

void Machine::write_rdram_u16_be(std::uint32_t address, std::uint16_t value) {
  if (address > rdram_.size() - 2) {
    fail_rdram_access(address, 2);
  }

  rdram_[address] = static_cast<std::uint8_t>((value >> 8) & 0xff);
  rdram_[address + 1] = static_cast<std::uint8_t>(value & 0xff);
}

void Machine::stage_rdram_u32_be(std::uint32_t address, std::uint32_t value) {
  write_rdram_u32_be(address, value);
}

void Machine::write_rdram_u32_be(std::uint32_t address, std::uint32_t value) {
  if (address > rdram_.size() - 4) {
    fail_rdram_access(address, 4);
  }

  rdram_[address] = static_cast<std::uint8_t>((value >> 24) & 0xff);
  rdram_[address + 1] = static_cast<std::uint8_t>((value >> 16) & 0xff);
  rdram_[address + 2] = static_cast<std::uint8_t>((value >> 8) & 0xff);
  rdram_[address + 3] = static_cast<std::uint8_t>(value & 0xff);
}

void Machine::stage_cartridge_bytes_to_rdram(
    std::uint32_t cartridge_offset,
    std::uint32_t rdram_address,
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

}  // namespace fn64
