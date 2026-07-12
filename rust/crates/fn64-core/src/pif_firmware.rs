use core::fmt;

/// Bytes occupied by the read-only PIF Boot ROM in the N64 physical map.
pub const PIF_BOOT_ROM_SIZE_BYTES: usize = 0x07c0;

/// Complete PIF address-space image size, including 64 bytes of writable PIF RAM.
pub const PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES: usize = 0x0800;

const PIF_IPL2_COPY_SOURCE_START_OFFSET: u32 = 0x00d4;
const PIF_IPL2_NTSC_COPY_SOURCE_END_OFFSET_EXCLUSIVE: u32 = 0x071c;
const PIF_IPL2_PAL_MPAL_COPY_SOURCE_END_OFFSET_EXCLUSIVE: u32 = 0x0720;
const PIF_IPL2_COPY_SP_IMEM_START_OFFSET: u32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PifIpl2Profile {
    NtscPinned,
    PalPinned,
    MpalPinned,
}

impl PifIpl2Profile {
    pub const fn name(self) -> &'static str {
        match self {
            Self::NtscPinned => "NTSC_PINNED",
            Self::PalPinned => "PAL_PINNED",
            Self::MpalPinned => "MPAL_PINNED",
        }
    }

    pub const fn copy_layout(self) -> PifIpl2CopyLayout {
        let source_end_offset_exclusive = match self {
            Self::NtscPinned => PIF_IPL2_NTSC_COPY_SOURCE_END_OFFSET_EXCLUSIVE,
            Self::PalPinned | Self::MpalPinned => {
                PIF_IPL2_PAL_MPAL_COPY_SOURCE_END_OFFSET_EXCLUSIVE
            }
        };
        let byte_count = source_end_offset_exclusive - PIF_IPL2_COPY_SOURCE_START_OFFSET;

        PifIpl2CopyLayout {
            source_start_offset: PIF_IPL2_COPY_SOURCE_START_OFFSET,
            source_end_offset_exclusive,
            sp_imem_start_offset: PIF_IPL2_COPY_SP_IMEM_START_OFFSET,
            sp_imem_end_offset_exclusive: PIF_IPL2_COPY_SP_IMEM_START_OFFSET + byte_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PifIpl2CopyLayout {
    source_start_offset: u32,
    source_end_offset_exclusive: u32,
    sp_imem_start_offset: u32,
    sp_imem_end_offset_exclusive: u32,
}

impl PifIpl2CopyLayout {
    pub const fn source_start_offset(self) -> u32 {
        self.source_start_offset
    }

    pub const fn source_end_offset_exclusive(self) -> u32 {
        self.source_end_offset_exclusive
    }

    pub const fn sp_imem_start_offset(self) -> u32 {
        self.sp_imem_start_offset
    }

    pub const fn sp_imem_end_offset_exclusive(self) -> u32 {
        self.sp_imem_end_offset_exclusive
    }

    pub const fn byte_count(self) -> usize {
        (self.source_end_offset_exclusive - self.source_start_offset) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PifFirmwareClassification {
    RawBootRom,
}

impl PifFirmwareClassification {
    pub const fn name(self) -> &'static str {
        match self {
            Self::RawBootRom => "raw-boot-rom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePifFirmwareState {
    Absent,
    Accepted {
        classification: PifFirmwareClassification,
        size_bytes: usize,
    },
}

impl MachinePifFirmwareState {
    pub const fn is_absent(self) -> bool {
        matches!(self, Self::Absent)
    }

    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted { .. })
    }

    pub const fn classification(self) -> Option<PifFirmwareClassification> {
        match self {
            Self::Accepted { classification, .. } => Some(classification),
            Self::Absent => None,
        }
    }

    pub const fn size_bytes(self) -> Option<usize> {
        match self {
            Self::Accepted { size_bytes, .. } => Some(size_bytes),
            Self::Absent => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PifFirmwareValidationError {
    MalformedLength {
        actual_size_bytes: usize,
        required_size_bytes: usize,
    },
    UnsupportedFullAddressSpaceImage {
        actual_size_bytes: usize,
        boot_rom_size_bytes: usize,
        pif_ram_size_bytes: usize,
    },
}

impl PifFirmwareValidationError {
    pub const fn is_malformed(self) -> bool {
        matches!(self, Self::MalformedLength { .. })
    }

    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::UnsupportedFullAddressSpaceImage { .. })
    }

    pub const fn actual_size_bytes(self) -> usize {
        match self {
            Self::MalformedLength {
                actual_size_bytes, ..
            }
            | Self::UnsupportedFullAddressSpaceImage {
                actual_size_bytes, ..
            } => actual_size_bytes,
        }
    }
}

impl fmt::Display for PifFirmwareValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MalformedLength {
                actual_size_bytes,
                required_size_bytes,
            } => write!(
                f,
                "malformed PIF firmware input: expected exactly {required_size_bytes} raw Boot ROM bytes, got {actual_size_bytes}"
            ),
            Self::UnsupportedFullAddressSpaceImage {
                actual_size_bytes,
                boot_rom_size_bytes,
                pif_ram_size_bytes,
            } => write!(
                f,
                "unsupported PIF firmware layout: {actual_size_bytes}-byte full address-space image includes {pif_ram_size_bytes} bytes of writable PIF RAM; expected {boot_rom_size_bytes} raw Boot ROM bytes"
            ),
        }
    }
}

impl std::error::Error for PifFirmwareValidationError {}

#[derive(Debug)]
pub(crate) struct PifFirmware {
    bytes: Box<[u8]>,
}

impl PifFirmware {
    pub(crate) fn from_owned_bytes(
        owned_bytes: Vec<u8>,
    ) -> Result<Self, PifFirmwareValidationError> {
        match owned_bytes.len() {
            PIF_BOOT_ROM_SIZE_BYTES => Ok(Self {
                bytes: owned_bytes.into_boxed_slice(),
            }),
            PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES => Err(
                PifFirmwareValidationError::UnsupportedFullAddressSpaceImage {
                    actual_size_bytes: PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
                    boot_rom_size_bytes: PIF_BOOT_ROM_SIZE_BYTES,
                    pif_ram_size_bytes: PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES
                        - PIF_BOOT_ROM_SIZE_BYTES,
                },
            ),
            actual_size_bytes => Err(PifFirmwareValidationError::MalformedLength {
                actual_size_bytes,
                required_size_bytes: PIF_BOOT_ROM_SIZE_BYTES,
            }),
        }
    }

    pub(crate) fn state(&self) -> MachinePifFirmwareState {
        MachinePifFirmwareState::Accepted {
            classification: PifFirmwareClassification::RawBootRom,
            size_bytes: self.bytes.len(),
        }
    }

    pub(crate) fn ipl2_copy(&self, profile: PifIpl2Profile) -> PifIpl2Copy<'_> {
        let layout = profile.copy_layout();
        let source_start = layout.source_start_offset() as usize;
        let source_end = layout.source_end_offset_exclusive() as usize;

        PifIpl2Copy {
            profile,
            layout,
            bytes: &self.bytes[source_start..source_end],
        }
    }

    #[cfg(test)]
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PifIpl2Copy<'a> {
    profile: PifIpl2Profile,
    layout: PifIpl2CopyLayout,
    bytes: &'a [u8],
}

impl<'a> PifIpl2Copy<'a> {
    pub(crate) const fn profile(self) -> PifIpl2Profile {
        self.profile
    }

    pub(crate) const fn layout(self) -> PifIpl2CopyLayout {
        self.layout
    }

    pub(crate) const fn bytes(self) -> &'a [u8] {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generated_pattern(seed: u8, size: usize) -> Vec<u8> {
        (0..size)
            .map(|index| seed.wrapping_add((index as u8).wrapping_mul(37)))
            .collect()
    }

    #[test]
    fn raw_boot_rom_length_is_accepted_and_preserved_byte_exactly() {
        let bytes = generated_pattern(0x31, PIF_BOOT_ROM_SIZE_BYTES);
        let firmware = PifFirmware::from_owned_bytes(bytes.clone()).unwrap();

        assert_eq!(firmware.bytes(), bytes);
        assert_eq!(
            firmware.state(),
            MachinePifFirmwareState::Accepted {
                classification: PifFirmwareClassification::RawBootRom,
                size_bytes: PIF_BOOT_ROM_SIZE_BYTES,
            }
        );
    }

    #[test]
    fn malformed_lengths_reject_without_guessing_a_variant() {
        for size in [
            0,
            1,
            PIF_BOOT_ROM_SIZE_BYTES - 1,
            PIF_BOOT_ROM_SIZE_BYTES + 1,
        ] {
            let error = PifFirmware::from_owned_bytes(generated_pattern(0x42, size)).unwrap_err();

            assert!(error.is_malformed());
            assert!(!error.is_unsupported());
            assert_eq!(error.actual_size_bytes(), size);
        }
    }

    #[test]
    fn full_address_space_image_is_structurally_named_but_unsupported() {
        let error = PifFirmware::from_owned_bytes(generated_pattern(
            0x53,
            PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
        ))
        .unwrap_err();

        assert!(!error.is_malformed());
        assert!(error.is_unsupported());
        assert_eq!(
            error,
            PifFirmwareValidationError::UnsupportedFullAddressSpaceImage {
                actual_size_bytes: PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
                boot_rom_size_bytes: PIF_BOOT_ROM_SIZE_BYTES,
                pif_ram_size_bytes: 0x40,
            }
        );
    }

    #[test]
    fn validation_does_not_select_by_hash_or_generated_content() {
        let first = PifFirmware::from_owned_bytes(generated_pattern(0x11, PIF_BOOT_ROM_SIZE_BYTES))
            .unwrap();
        let second =
            PifFirmware::from_owned_bytes(generated_pattern(0xa7, PIF_BOOT_ROM_SIZE_BYTES))
                .unwrap();

        assert_eq!(first.state(), second.state());
        assert_ne!(first.bytes(), second.bytes());
    }

    #[test]
    fn pinned_profiles_own_exact_semantic_names_and_copy_layouts() {
        for (profile, name, source_end, byte_count) in [
            (PifIpl2Profile::NtscPinned, "NTSC_PINNED", 0x071c, 0x0648),
            (PifIpl2Profile::PalPinned, "PAL_PINNED", 0x0720, 0x064c),
            (PifIpl2Profile::MpalPinned, "MPAL_PINNED", 0x0720, 0x064c),
        ] {
            let layout = profile.copy_layout();
            assert_eq!(profile.name(), name);
            assert_eq!(layout.source_start_offset(), 0x00d4);
            assert_eq!(layout.source_end_offset_exclusive(), source_end);
            assert_eq!(layout.sp_imem_start_offset(), 0);
            assert_eq!(layout.sp_imem_end_offset_exclusive(), byte_count);
            assert_eq!(layout.byte_count(), byte_count as usize);
            assert!(layout.source_end_offset_exclusive() as usize <= PIF_BOOT_ROM_SIZE_BYTES);
        }
    }

    #[test]
    fn selected_profile_copies_the_complete_generated_source_slice_only() {
        let bytes = generated_pattern(0x65, PIF_BOOT_ROM_SIZE_BYTES);

        for profile in [
            PifIpl2Profile::NtscPinned,
            PifIpl2Profile::PalPinned,
            PifIpl2Profile::MpalPinned,
        ] {
            let firmware = PifFirmware::from_owned_bytes(bytes.clone()).unwrap();
            let copy = firmware.ipl2_copy(profile);
            let layout = profile.copy_layout();

            assert_eq!(copy.profile(), profile);
            assert_eq!(copy.layout(), layout);
            assert_eq!(copy.bytes().len(), layout.byte_count());
            assert_eq!(
                copy.bytes(),
                &bytes[layout.source_start_offset() as usize
                    ..layout.source_end_offset_exclusive() as usize]
            );
        }
    }
}
