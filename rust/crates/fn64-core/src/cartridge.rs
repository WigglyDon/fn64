use std::fmt;

mod byte_order;
mod metadata;

use byte_order::{detect_rom_source_layout, normalize_rom_bytes};
use metadata::{parse_rom_metadata, ROM_HEADER_SIZE};

pub use byte_order::{rom_source_layout_name, RomSourceLayout};
pub use metadata::RomMetadata;

pub const CARTRIDGE_HEADER_ENTRY_WORD_OFFSET: u32 = 0x0000_0008;
pub const CARTRIDGE_CANDIDATE_IPL3_START_OFFSET: u32 = 0x0000_0040;
pub const CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE: u32 = 0x0000_1000;
pub const CARTRIDGE_CANDIDATE_IPL3_BYTE_COUNT: u32 =
    CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE - CARTRIDGE_CANDIDATE_IPL3_START_OFFSET;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedRomImage {
    pub source_layout: RomSourceLayout,
    pub bytes: Vec<u8>,
    pub metadata: RomMetadata,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CartridgeLoadError {
    HeaderTooSmall,
    SizeNotMultipleOf4,
    UnsupportedHeaderByteLayout,
    NormalizedHeaderTooSmall,
    NormalizedHeaderMagicMismatch,
}

impl fmt::Display for CartridgeLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            CartridgeLoadError::HeaderTooSmall => {
                "ROM image is too small to contain a complete 0x40-byte N64 ROM header"
            }
            CartridgeLoadError::SizeNotMultipleOf4 => {
                "ROM image size is not a multiple of 4 bytes"
            }
            CartridgeLoadError::UnsupportedHeaderByteLayout => {
                "unsupported ROM header byte layout; expected one of 80 37 12 40, 37 80 40 12, or 40 12 37 80"
            }
            CartridgeLoadError::NormalizedHeaderTooSmall => {
                "normalized ROM is smaller than the 0x40-byte N64 header"
            }
            CartridgeLoadError::NormalizedHeaderMagicMismatch => {
                "normalized ROM header magic mismatch; expected 0x80371240"
            }
        };
        f.write_str(message)
    }
}

impl std::error::Error for CartridgeLoadError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CartridgeReadError {
    OutOfRange {
        addr: u32,
        width: usize,
        size: usize,
    },
}

impl fmt::Display for CartridgeReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CartridgeReadError::OutOfRange { addr, width, size } => {
                write!(
                    f,
                    "cartridge read out of range: addr=0x{addr:08X} width={width} size={size}"
                )
            }
        }
    }
}

impl std::error::Error for CartridgeReadError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CartridgeEntryInspection {
    pub header_entry_word_available: bool,
    pub header_entry_word: u32,
    pub candidate_ipl3_span_available: bool,
    pub candidate_ipl3_start_offset: u32,
    pub candidate_ipl3_end_offset_exclusive: u32,
    pub candidate_ipl3_byte_count: u32,
    pub ipl3_first_word_available: bool,
    pub ipl3_first_word: u32,
}

impl Default for CartridgeEntryInspection {
    fn default() -> Self {
        Self {
            header_entry_word_available: false,
            header_entry_word: 0,
            candidate_ipl3_span_available: false,
            candidate_ipl3_start_offset: CARTRIDGE_CANDIDATE_IPL3_START_OFFSET,
            candidate_ipl3_end_offset_exclusive: CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE,
            candidate_ipl3_byte_count: CARTRIDGE_CANDIDATE_IPL3_BYTE_COUNT,
            ipl3_first_word_available: false,
            ipl3_first_word: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cartridge {
    source_layout: RomSourceLayout,
    image: Vec<u8>,
    metadata: RomMetadata,
}

impl Cartridge {
    pub fn source_layout(&self) -> RomSourceLayout {
        self.source_layout
    }

    pub fn metadata(&self) -> &RomMetadata {
        &self.metadata
    }

    pub fn size_bytes(&self) -> usize {
        self.image.len()
    }

    pub fn read_u8(&self, cart_addr: u32) -> Result<u8, CartridgeReadError> {
        let offset = cart_addr as usize;
        if offset >= self.image.len() {
            return Err(CartridgeReadError::OutOfRange {
                addr: cart_addr,
                width: 1,
                size: self.image.len(),
            });
        }

        Ok(self.image[offset])
    }

    pub fn read_u32_be(&self, cart_addr: u32) -> Result<u32, CartridgeReadError> {
        let Some(final_addr) = cart_addr.checked_add(3) else {
            return Err(CartridgeReadError::OutOfRange {
                addr: cart_addr,
                width: 4,
                size: self.image.len(),
            });
        };
        if final_addr as usize >= self.image.len() {
            return Err(CartridgeReadError::OutOfRange {
                addr: cart_addr,
                width: 4,
                size: self.image.len(),
            });
        }

        Ok(u32::from_be_bytes([
            self.image[cart_addr as usize],
            self.image[cart_addr as usize + 1],
            self.image[cart_addr as usize + 2],
            self.image[cart_addr as usize + 3],
        ]))
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self {
            source_layout: RomSourceLayout::BigEndian,
            image: Vec::new(),
            metadata: RomMetadata::default(),
        }
    }
}

pub fn load_cartridge(raw_bytes: Vec<u8>) -> Result<Cartridge, CartridgeLoadError> {
    let image = normalize_rom_image(raw_bytes)?;
    Ok(Cartridge {
        source_layout: image.source_layout,
        image: image.bytes,
        metadata: image.metadata,
    })
}

pub fn normalize_rom_image(raw_bytes: Vec<u8>) -> Result<NormalizedRomImage, CartridgeLoadError> {
    if raw_bytes.len() < ROM_HEADER_SIZE {
        return Err(CartridgeLoadError::HeaderTooSmall);
    }

    if !raw_bytes.len().is_multiple_of(4) {
        return Err(CartridgeLoadError::SizeNotMultipleOf4);
    }

    let source_layout = detect_rom_source_layout(&raw_bytes)?;
    let bytes = normalize_rom_bytes(raw_bytes, source_layout);
    let metadata = parse_rom_metadata(&bytes)?;

    Ok(NormalizedRomImage {
        source_layout,
        bytes,
        metadata,
    })
}

pub fn inspect_cartridge_entry(cartridge: &Cartridge) -> CartridgeEntryInspection {
    let mut inspection = CartridgeEntryInspection::default();
    let size = cartridge.size_bytes();

    if span_available(size, CARTRIDGE_HEADER_ENTRY_WORD_OFFSET, 4) {
        inspection.header_entry_word_available = true;
        inspection.header_entry_word =
            read_cartridge_u32_be(cartridge, CARTRIDGE_HEADER_ENTRY_WORD_OFFSET)
                .expect("span was checked before reading header entry word");
    }

    if span_available(
        size,
        CARTRIDGE_CANDIDATE_IPL3_START_OFFSET,
        CARTRIDGE_CANDIDATE_IPL3_BYTE_COUNT,
    ) {
        inspection.candidate_ipl3_span_available = true;
    }

    if span_available(size, CARTRIDGE_CANDIDATE_IPL3_START_OFFSET, 4) {
        inspection.ipl3_first_word_available = true;
        inspection.ipl3_first_word =
            read_cartridge_u32_be(cartridge, CARTRIDGE_CANDIDATE_IPL3_START_OFFSET)
                .expect("span was checked before reading IPL3 first word");
    }

    inspection
}

fn span_available(size: usize, offset: u32, byte_count: u32) -> bool {
    let span_offset = offset as usize;
    let span_byte_count = byte_count as usize;
    span_offset <= size && span_byte_count <= size - span_offset
}

fn read_cartridge_u32_be(cartridge: &Cartridge, offset: u32) -> Result<u32, CartridgeReadError> {
    cartridge.read_u32_be(offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset] = ((value >> 24) & 0xff) as u8;
        bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
        bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
        bytes[offset + 3] = (value & 0xff) as u8;
    }

    fn make_synthetic_normalized_rom_proof_image() -> Vec<u8> {
        let mut rom = vec![0; 0x60];
        write_be_u32(&mut rom, 0x00, 0x8037_1240);
        write_be_u32(&mut rom, 0x04, 0x1234_5678);
        write_be_u32(&mut rom, 0x08, 0x8024_6000);
        write_be_u32(&mut rom, 0x0C, 0x0040_0000);
        write_be_u32(&mut rom, 0x10, 0x89AB_CDEF);
        write_be_u32(&mut rom, 0x14, 0x0123_4567);

        for (index, ch) in b"FN64 ROM PROOF".iter().enumerate() {
            rom[0x20 + index] = *ch;
        }

        rom[0x3C] = b'F';
        rom[0x3D] = b'R';
        rom[0x3E] = 0x45;
        rom[0x3F] = 0x07;

        for (offset, byte) in rom.iter_mut().enumerate().skip(0x40) {
            *byte = ((offset * 3 + 0x11) & 0xff) as u8;
        }

        rom
    }

    fn make_synthetic_metadata_edge_rom() -> Vec<u8> {
        let mut rom = make_synthetic_normalized_rom_proof_image();
        for byte in &mut rom[0x20..0x34] {
            *byte = 0;
        }
        for byte in &mut rom[0x3C..0x3E] {
            *byte = 0;
        }

        rom[0x20] = b'E';
        rom[0x21] = 0x01;
        rom[0x22] = b' ';
        rom[0x23] = b' ';
        rom[0x3C] = b'Q';
        rom[0x3D] = b' ';
        rom
    }

    fn make_synthetic_normalized_entry_inspection_rom() -> Vec<u8> {
        let mut rom = vec![0; CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE as usize];
        write_be_u32(&mut rom, 0x00, 0x8037_1240);
        write_be_u32(&mut rom, 0x04, 0x0102_0304);
        write_be_u32(
            &mut rom,
            CARTRIDGE_HEADER_ENTRY_WORD_OFFSET as usize,
            0x8070_1234,
        );
        write_be_u32(&mut rom, 0x0C, 0x0506_0708);
        write_be_u32(&mut rom, 0x10, 0x1112_1314);
        write_be_u32(&mut rom, 0x14, 0x1516_1718);

        for (index, ch) in b"FN64 ENTRY PROOF".iter().enumerate() {
            rom[0x20 + index] = *ch;
        }

        rom[0x3C] = b'E';
        rom[0x3D] = b'I';
        rom[0x3E] = 0x45;
        rom[0x3F] = 0x03;

        for (offset, byte) in rom
            .iter_mut()
            .enumerate()
            .take(CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE as usize)
            .skip(CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize)
        {
            *byte = ((offset * 5 + 0x23) & 0xff) as u8;
        }
        write_be_u32(
            &mut rom,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize,
            0x3C1A_8000,
        );

        rom
    }

    fn encode_synthetic_rom_source_layout(
        mut normalized_bytes: Vec<u8>,
        layout: RomSourceLayout,
    ) -> Vec<u8> {
        match layout {
            RomSourceLayout::BigEndian => normalized_bytes,
            RomSourceLayout::ByteSwapped16 => {
                for chunk in normalized_bytes.chunks_exact_mut(2) {
                    chunk.swap(0, 1);
                }
                normalized_bytes
            }
            RomSourceLayout::LittleEndian32 => {
                for chunk in normalized_bytes.chunks_exact_mut(4) {
                    chunk.swap(0, 3);
                    chunk.swap(1, 2);
                }
                normalized_bytes
            }
        }
    }

    fn assert_metadata_matches(metadata: &RomMetadata) {
        assert_eq!(metadata.header_magic, 0x8037_1240);
        assert_eq!(metadata.clock_rate, 0x1234_5678);
        assert_eq!(metadata.entry_point, 0x8024_6000);
        assert_eq!(metadata.release_address, 0x0040_0000);
        assert_eq!(metadata.crc1, 0x89AB_CDEF);
        assert_eq!(metadata.crc2, 0x0123_4567);
        assert_eq!(metadata.image_name, "FN64 ROM PROOF");
        assert_eq!(metadata.cartridge_id, "FR");
        assert_eq!(metadata.country_code, 0x45);
        assert_eq!(metadata.revision, 0x07);
    }

    #[test]
    fn normalizes_supported_source_layouts_and_loads_cartridge_bytes() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        for layout in [
            RomSourceLayout::BigEndian,
            RomSourceLayout::ByteSwapped16,
            RomSourceLayout::LittleEndian32,
        ] {
            let raw_bytes = encode_synthetic_rom_source_layout(normalized_bytes.clone(), layout);

            let normalized_image =
                normalize_rom_image(raw_bytes.clone()).expect("synthetic ROM should normalize");
            assert_eq!(normalized_image.source_layout, layout);
            assert_eq!(normalized_image.bytes, normalized_bytes);
            assert_metadata_matches(&normalized_image.metadata);

            let cartridge = load_cartridge(raw_bytes).expect("synthetic cartridge should load");
            assert_eq!(cartridge.source_layout(), layout);
            assert_eq!(cartridge.size_bytes(), normalized_bytes.len());
            assert_metadata_matches(cartridge.metadata());
            for (offset, expected) in normalized_bytes.iter().enumerate() {
                assert_eq!(cartridge.read_u8(offset as u32).unwrap(), *expected);
            }
        }
    }

    #[test]
    fn rejects_unsupported_or_malformed_rom_inputs() {
        assert_eq!(
            normalize_rom_image(Vec::new()).unwrap_err(),
            CartridgeLoadError::HeaderTooSmall
        );

        assert_eq!(
            normalize_rom_image(vec![0x80, 0x37, 0x12]).unwrap_err(),
            CartridgeLoadError::HeaderTooSmall
        );

        let mut short_header = vec![0; 0x3f];
        short_header[0] = 0x80;
        short_header[1] = 0x37;
        short_header[2] = 0x12;
        short_header[3] = 0x40;
        assert_eq!(
            normalize_rom_image(short_header).unwrap_err(),
            CartridgeLoadError::HeaderTooSmall
        );

        let mut odd_sized = make_synthetic_normalized_rom_proof_image();
        odd_sized.push(0);
        assert_eq!(
            normalize_rom_image(odd_sized).unwrap_err(),
            CartridgeLoadError::SizeNotMultipleOf4
        );

        let unsupported = vec![0; ROM_HEADER_SIZE];
        assert_eq!(
            normalize_rom_image(unsupported).unwrap_err(),
            CartridgeLoadError::UnsupportedHeaderByteLayout
        );
    }

    #[test]
    fn extracts_metadata_name_fields_like_cpp_ascii_reader() {
        let cartridge = load_cartridge(make_synthetic_metadata_edge_rom()).unwrap();
        let metadata = cartridge.metadata();

        assert_eq!(metadata.image_name, "E?");
        assert_eq!(metadata.cartridge_id, "Q");
    }

    #[test]
    fn entry_inspection_reports_available_and_unavailable_spans() {
        let normalized_bytes = make_synthetic_normalized_entry_inspection_rom();
        let cartridge = load_cartridge(normalized_bytes.clone()).unwrap();
        let inspection = inspect_cartridge_entry(&cartridge);

        assert!(inspection.header_entry_word_available);
        assert_eq!(inspection.header_entry_word, 0x8070_1234);
        assert!(inspection.candidate_ipl3_span_available);
        assert_eq!(
            inspection.candidate_ipl3_start_offset,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET
        );
        assert_eq!(
            inspection.candidate_ipl3_end_offset_exclusive,
            CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE
        );
        assert_eq!(
            inspection.candidate_ipl3_byte_count,
            CARTRIDGE_CANDIDATE_IPL3_BYTE_COUNT
        );
        assert!(inspection.ipl3_first_word_available);
        assert_eq!(inspection.ipl3_first_word, 0x3C1A_8000);

        let short = load_cartridge(make_synthetic_normalized_rom_proof_image()).unwrap();
        let short_inspection = inspect_cartridge_entry(&short);
        assert!(short_inspection.header_entry_word_available);
        assert_eq!(short_inspection.header_entry_word, 0x8024_6000);
        assert!(!short_inspection.candidate_ipl3_span_available);
        assert!(short_inspection.ipl3_first_word_available);
    }

    #[test]
    fn cartridge_read_is_range_checked() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let cartridge = load_cartridge(normalized_bytes.clone()).unwrap();

        assert_eq!(cartridge.read_u8(0).unwrap(), 0x80);
        assert_eq!(cartridge.read_u32_be(0), Ok(0x8037_1240));
        assert_eq!(cartridge.read_u32_be(8), Ok(0x8024_6000));
        assert_eq!(
            cartridge
                .read_u8((normalized_bytes.len() - 1) as u32)
                .unwrap(),
            *normalized_bytes.last().unwrap()
        );
        assert_eq!(
            cartridge
                .read_u8(normalized_bytes.len() as u32)
                .unwrap_err(),
            CartridgeReadError::OutOfRange {
                addr: normalized_bytes.len() as u32,
                width: 1,
                size: normalized_bytes.len()
            }
        );
        assert_eq!(
            cartridge
                .read_u8(normalized_bytes.len() as u32)
                .unwrap_err()
                .to_string(),
            "cartridge read out of range: addr=0x00000060 width=1 size=96"
        );
        assert_eq!(
            cartridge
                .read_u32_be((normalized_bytes.len() - 3) as u32)
                .unwrap_err(),
            CartridgeReadError::OutOfRange {
                addr: (normalized_bytes.len() - 3) as u32,
                width: 4,
                size: normalized_bytes.len(),
            }
        );
        assert_eq!(
            cartridge.read_u32_be(u32::MAX).unwrap_err(),
            CartridgeReadError::OutOfRange {
                addr: u32::MAX,
                width: 4,
                size: normalized_bytes.len(),
            }
        );
    }

    #[test]
    fn default_cartridge_is_empty_big_endian_without_metadata() {
        let cartridge = Cartridge::default();

        assert_eq!(cartridge.source_layout(), RomSourceLayout::BigEndian);
        assert_eq!(cartridge.size_bytes(), 0);
        assert_eq!(cartridge.metadata(), &RomMetadata::default());
        assert_eq!(
            cartridge.read_u8(0).unwrap_err(),
            CartridgeReadError::OutOfRange {
                addr: 0,
                width: 1,
                size: 0
            }
        );
    }
}
