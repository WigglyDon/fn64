use core::fmt;

/// Bytes occupied by the read-only PIF Boot ROM in the N64 physical map.
pub const PIF_BOOT_ROM_SIZE_BYTES: usize = 0x07c0;

/// Complete PIF address-space image size, including 64 bytes of writable PIF RAM.
pub const PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES: usize = 0x0800;

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

    #[cfg(test)]
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
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
}
