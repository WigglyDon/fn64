use super::CartridgeLoadError;

pub(super) const ROM_HEADER_SIZE: usize = 0x40;

const EXPECTED_HEADER_MAGIC: u32 = 0x8037_1240;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CartridgePiDomain1Timing {
    latency: u8,
    pulse_width: u8,
    page_size: u8,
    release_duration: u8,
}

impl CartridgePiDomain1Timing {
    pub const fn from_header_configuration_word(header_configuration_word: u32) -> Self {
        Self {
            latency: (header_configuration_word >> 24) as u8,
            pulse_width: (header_configuration_word >> 8) as u8,
            page_size: ((header_configuration_word >> 16) & 0x0f) as u8,
            release_duration: (header_configuration_word & 0x03) as u8,
        }
    }

    pub const fn latency(self) -> u8 {
        self.latency
    }

    pub const fn pulse_width(self) -> u8 {
        self.pulse_width
    }

    pub const fn page_size(self) -> u8 {
        self.page_size
    }

    pub const fn release_duration(self) -> u8 {
        self.release_duration
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RomMetadata {
    pub header_magic: u32,
    pub pi_domain_one_timing: CartridgePiDomain1Timing,
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
        pi_domain_one_timing: CartridgePiDomain1Timing::from_header_configuration_word(
            header_magic,
        ),
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
