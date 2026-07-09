use super::CartridgeLoadError;

pub(super) const ROM_HEADER_SIZE: usize = 0x40;

const EXPECTED_HEADER_MAGIC: u32 = 0x8037_1240;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RomMetadata {
    pub header_magic: u32,
    pub clock_rate: u32,
    pub entry_point: u32,
    pub release_address: u32,
    pub crc1: u32,
    pub crc2: u32,
    pub image_name: String,
    pub cartridge_id: String,
    pub country_code: u8,
    pub revision: u8,
}

pub(super) fn parse_rom_metadata(
    normalized_bytes: &[u8],
) -> Result<RomMetadata, CartridgeLoadError> {
    if normalized_bytes.len() < ROM_HEADER_SIZE {
        return Err(CartridgeLoadError::NormalizedHeaderTooSmall);
    }

    let header_magic = read_be_u32(normalized_bytes, 0x00);
    if header_magic != EXPECTED_HEADER_MAGIC {
        return Err(CartridgeLoadError::NormalizedHeaderMagicMismatch);
    }

    Ok(RomMetadata {
        header_magic,
        clock_rate: read_be_u32(normalized_bytes, 0x04),
        entry_point: read_be_u32(normalized_bytes, 0x08),
        release_address: read_be_u32(normalized_bytes, 0x0C),
        crc1: read_be_u32(normalized_bytes, 0x10),
        crc2: read_be_u32(normalized_bytes, 0x14),
        image_name: read_ascii_field(normalized_bytes, 0x20, 20),
        cartridge_id: read_ascii_field(normalized_bytes, 0x3C, 2),
        country_code: normalized_bytes[0x3E],
        revision: normalized_bytes[0x3F],
    })
}

fn read_be_u32(bytes: &[u8], offset: usize) -> u32 {
    ((bytes[offset] as u32) << 24)
        | ((bytes[offset + 1] as u32) << 16)
        | ((bytes[offset + 2] as u32) << 8)
        | (bytes[offset + 3] as u32)
}

fn read_ascii_field(bytes: &[u8], offset: usize, length: usize) -> String {
    let mut value = String::with_capacity(length);

    for ch in &bytes[offset..offset + length] {
        if *ch == 0 {
            break;
        }

        if ch.is_ascii_graphic() || *ch == b' ' {
            value.push(*ch as char);
        } else {
            value.push('?');
        }
    }

    while value.ends_with(' ') {
        value.pop();
    }

    value
}
