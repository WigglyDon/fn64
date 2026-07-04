#pragma once

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "rom.hpp"

namespace fn64 {

inline constexpr std::uint32_t kCartridgeHeaderEntryWordOffset = 0x00000008u;
inline constexpr std::uint32_t kCartridgeCandidateIpl3StartOffset = 0x00000040u;
inline constexpr std::uint32_t kCartridgeCandidateIpl3EndOffsetExclusive = 0x00001000u;
inline constexpr std::uint32_t kCartridgeCandidateIpl3ByteCount =
    kCartridgeCandidateIpl3EndOffsetExclusive - kCartridgeCandidateIpl3StartOffset;

struct CartridgeEntryInspection {
  bool header_entry_word_available = false;
  std::uint32_t header_entry_word = 0;
  bool candidate_ipl3_span_available = false;
  std::uint32_t candidate_ipl3_start_offset = kCartridgeCandidateIpl3StartOffset;
  std::uint32_t candidate_ipl3_end_offset_exclusive =
      kCartridgeCandidateIpl3EndOffsetExclusive;
  std::uint32_t candidate_ipl3_byte_count = kCartridgeCandidateIpl3ByteCount;
  bool ipl3_first_word_available = false;
  std::uint32_t ipl3_first_word = 0;
};

class Cartridge {
public:
  Cartridge() = default;

  RomSourceLayout source_layout() const noexcept;
  const RomMetadata& metadata() const noexcept;

  std::size_t size_bytes() const noexcept;

  std::uint8_t read_u8(std::uint32_t cart_addr) const;

private:
  Cartridge(
      RomSourceLayout source_layout,
      std::vector<std::uint8_t> image,
      RomMetadata metadata
  );

  RomSourceLayout source_layout_ = RomSourceLayout::kBigEndian;
  std::vector<std::uint8_t> image_;
  RomMetadata metadata_;

  friend bool load_cartridge(
      std::vector<std::uint8_t> raw_bytes,
      Cartridge& out_cartridge,
      std::string& error
  );
};

bool load_cartridge(
    std::vector<std::uint8_t> raw_bytes,
    Cartridge& out_cartridge,
    std::string& error
);

CartridgeEntryInspection inspect_cartridge_entry(const Cartridge& cartridge);

}  // namespace fn64
