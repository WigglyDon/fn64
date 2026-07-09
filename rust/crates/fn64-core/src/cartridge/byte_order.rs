use super::CartridgeLoadError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RomSourceLayout {
    BigEndian,
    ByteSwapped16,
    LittleEndian32,
}

pub fn rom_source_layout_name(layout: RomSourceLayout) -> &'static str {
    match layout {
        RomSourceLayout::BigEndian => "big-endian cartridge order (.z64)",
        RomSourceLayout::ByteSwapped16 => "16-bit byte-swapped (.v64)",
        RomSourceLayout::LittleEndian32 => "32-bit little-endian word order (.n64)",
    }
}

pub(super) fn detect_rom_source_layout(
    raw_bytes: &[u8],
) -> Result<RomSourceLayout, CartridgeLoadError> {
    match &raw_bytes[0..4] {
        [0x80, 0x37, 0x12, 0x40] => Ok(RomSourceLayout::BigEndian),
        [0x37, 0x80, 0x40, 0x12] => Ok(RomSourceLayout::ByteSwapped16),
        [0x40, 0x12, 0x37, 0x80] => Ok(RomSourceLayout::LittleEndian32),
        _ => Err(CartridgeLoadError::UnsupportedHeaderByteLayout),
    }
}

pub(super) fn normalize_rom_bytes(mut raw_bytes: Vec<u8>, layout: RomSourceLayout) -> Vec<u8> {
    match layout {
        RomSourceLayout::BigEndian => raw_bytes,
        RomSourceLayout::ByteSwapped16 => {
            for chunk in raw_bytes.chunks_exact_mut(2) {
                chunk.swap(0, 1);
            }
            raw_bytes
        }
        RomSourceLayout::LittleEndian32 => {
            for chunk in raw_bytes.chunks_exact_mut(4) {
                chunk.swap(0, 3);
                chunk.swap(1, 2);
            }
            raw_bytes
        }
    }
}
