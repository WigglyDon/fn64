use core::fmt;

use crate::pif_firmware::{PifIpl2Copy, PifIpl2Profile};

pub(crate) const SP_IMEM_SIZE_BYTES: usize = 4 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpImemOffset(u32);

impl SpImemOffset {
    pub(crate) const fn new(value: u32) -> Self {
        Self(value)
    }

    pub(crate) const fn value(self) -> u32 {
        self.0
    }

    const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpImemByteProvenance {
    Unknown,
    UserSuppliedPifFirmware {
        profile: PifIpl2Profile,
        source_offset: u32,
    },
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

impl SpImemByteProvenance {
    pub(crate) const fn is_known(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct SpImemByteObservation {
    value: u8,
    provenance: SpImemByteProvenance,
}

#[cfg(test)]
impl SpImemByteObservation {
    pub(crate) const fn value(self) -> u8 {
        self.value
    }

    pub(crate) const fn provenance(self) -> SpImemByteProvenance {
        self.provenance
    }

    pub(crate) const fn is_known(self) -> bool {
        self.provenance.is_known()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpImemReadError {
    Unaligned {
        offset: SpImemOffset,
        width: usize,
    },
    OutOfRange {
        offset: SpImemOffset,
        width: usize,
    },
    UnknownByte {
        offset: SpImemOffset,
        width: usize,
        unknown_offset: SpImemOffset,
    },
}

impl SpImemReadError {
    pub(crate) const fn unknown_offset(self) -> Option<SpImemOffset> {
        match self {
            Self::UnknownByte { unknown_offset, .. } => Some(unknown_offset),
            Self::Unaligned { .. } | Self::OutOfRange { .. } => None,
        }
    }
}

impl fmt::Display for SpImemReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unaligned { offset, width } => write!(
                f,
                "SP IMEM aligned read rejected: offset={} width={}",
                offset.value(),
                width
            ),
            Self::OutOfRange { offset, width } => write!(
                f,
                "SP IMEM access out of range: offset={} width={}",
                offset.value(),
                width
            ),
            Self::UnknownByte {
                offset,
                width,
                unknown_offset,
            } => write!(
                f,
                "SP IMEM known read unavailable: offset={} width={} first_unknown_offset={}",
                offset.value(),
                width,
                unknown_offset.value()
            ),
        }
    }
}

impl std::error::Error for SpImemReadError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpImemPifIpl2CopyError {
    start_offset: u32,
    byte_count: usize,
}

impl SpImemPifIpl2CopyError {
    pub(crate) const fn start_offset(self) -> u32 {
        self.start_offset
    }

    pub(crate) const fn byte_count(self) -> usize {
        self.byte_count
    }
}

impl fmt::Display for SpImemPifIpl2CopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "profiled PIF IPL2 copy destination unavailable: start={} width={}",
            self.start_offset, self.byte_count
        )
    }
}

impl std::error::Error for SpImemPifIpl2CopyError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpImemKnownWord {
    value: u32,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl SpImemKnownWord {
    pub(crate) const fn value(self) -> u32 {
        self.value
    }

    #[cfg(test)]
    pub(crate) const fn byte_provenance(self) -> [SpImemByteProvenance; 4] {
        self.byte_provenance
    }
}

pub(crate) struct SpImem {
    bytes: [u8; SP_IMEM_SIZE_BYTES],
    byte_provenance: [SpImemByteProvenance; SP_IMEM_SIZE_BYTES],
}

impl SpImem {
    pub(crate) fn from_pif_ipl2_copy(
        copy: PifIpl2Copy<'_>,
    ) -> Result<Self, SpImemPifIpl2CopyError> {
        let layout = copy.layout();
        let start_offset = layout.sp_imem_start_offset();
        let start = start_offset as usize;
        let Some(end) = start.checked_add(copy.bytes().len()) else {
            return Err(SpImemPifIpl2CopyError {
                start_offset,
                byte_count: copy.bytes().len(),
            });
        };
        if end > SP_IMEM_SIZE_BYTES
            || end != layout.sp_imem_end_offset_exclusive() as usize
            || copy.bytes().len() != layout.byte_count()
        {
            return Err(SpImemPifIpl2CopyError {
                start_offset,
                byte_count: copy.bytes().len(),
            });
        }

        let mut sp_imem = Self::default();
        sp_imem.bytes[start..end].copy_from_slice(copy.bytes());
        for (index, provenance) in sp_imem.byte_provenance[start..end].iter_mut().enumerate() {
            *provenance = SpImemByteProvenance::UserSuppliedPifFirmware {
                profile: copy.profile(),
                source_offset: layout.source_start_offset() + index as u32,
            };
        }

        Ok(sp_imem)
    }

    #[cfg(test)]
    pub(crate) const fn size_bytes(&self) -> usize {
        self.bytes.len()
    }

    #[cfg(test)]
    pub(crate) fn observe_byte(
        &self,
        offset: SpImemOffset,
    ) -> Result<SpImemByteObservation, SpImemReadError> {
        let offset_usize = offset.as_usize();
        let Some(value) = self.bytes.get(offset_usize).copied() else {
            return Err(SpImemReadError::OutOfRange { offset, width: 1 });
        };
        let provenance = self.byte_provenance[offset_usize];

        Ok(SpImemByteObservation { value, provenance })
    }

    pub(crate) fn read_known_u32_be(
        &self,
        offset: SpImemOffset,
    ) -> Result<SpImemKnownWord, SpImemReadError> {
        if (offset.value() & 0x3) != 0 {
            return Err(SpImemReadError::Unaligned { offset, width: 4 });
        }

        let offset_usize = self.require_span(offset, 4)?;
        let byte_provenance =
            core::array::from_fn(|index| self.byte_provenance[offset_usize + index]);
        if let Some(index) = byte_provenance
            .iter()
            .position(|provenance| !provenance.is_known())
        {
            return Err(SpImemReadError::UnknownByte {
                offset,
                width: 4,
                unknown_offset: SpImemOffset::new(offset.value() + index as u32),
            });
        }

        let value = ((self.bytes[offset_usize] as u32) << 24)
            | ((self.bytes[offset_usize + 1] as u32) << 16)
            | ((self.bytes[offset_usize + 2] as u32) << 8)
            | self.bytes[offset_usize + 3] as u32;

        Ok(SpImemKnownWord {
            value,
            byte_provenance,
        })
    }

    fn require_span(&self, offset: SpImemOffset, width: usize) -> Result<usize, SpImemReadError> {
        let offset_usize = offset.as_usize();
        let Some(end) = offset_usize.checked_add(width) else {
            return Err(SpImemReadError::OutOfRange { offset, width });
        };
        if end > self.bytes.len() {
            return Err(SpImemReadError::OutOfRange { offset, width });
        }

        Ok(offset_usize)
    }

    #[cfg(test)]
    fn stage_known_bytes_for_test(
        &mut self,
        offset: SpImemOffset,
        bytes: &[u8],
    ) -> Result<(), SpImemReadError> {
        let offset_usize = self.require_span(offset, bytes.len())?;
        let end = offset_usize + bytes.len();
        self.bytes[offset_usize..end].copy_from_slice(bytes);
        self.byte_provenance[offset_usize..end]
            .fill(SpImemByteProvenance::GeneratedMachineTestStaging);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn stage_known_u32_be_for_test(
        &mut self,
        offset: SpImemOffset,
        value: u32,
    ) -> Result<(), SpImemReadError> {
        self.stage_known_bytes_for_test(offset, &value.to_be_bytes())
    }
}

impl Default for SpImem {
    fn default() -> Self {
        Self {
            bytes: [0; SP_IMEM_SIZE_BYTES],
            byte_provenance: [SpImemByteProvenance::Unknown; SP_IMEM_SIZE_BYTES],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pif_firmware::{PifFirmware, PIF_BOOT_ROM_SIZE_BYTES};

    fn generated_pif_firmware(seed: u8) -> Vec<u8> {
        (0..PIF_BOOT_ROM_SIZE_BYTES)
            .map(|index| seed.wrapping_add((index as u8).wrapping_mul(31)))
            .collect()
    }

    #[test]
    fn sp_imem_construction_has_exact_capacity_and_unknown_zero_backing() {
        let sp_imem = SpImem::default();

        assert_eq!(sp_imem.size_bytes(), SP_IMEM_SIZE_BYTES);
        assert_eq!(sp_imem.size_bytes(), 0x1000);
        for offset in [0, SP_IMEM_SIZE_BYTES - 1] {
            let observed = sp_imem
                .observe_byte(SpImemOffset::new(offset as u32))
                .unwrap();
            assert_eq!(observed.value(), 0);
            assert!(!observed.is_known());
            assert_eq!(observed.provenance(), SpImemByteProvenance::Unknown);
        }
    }

    #[test]
    fn sp_imem_byte_observation_enforces_first_last_and_past_end_bounds() {
        let sp_imem = SpImem::default();

        assert!(sp_imem.observe_byte(SpImemOffset::new(0)).is_ok());
        assert!(sp_imem
            .observe_byte(SpImemOffset::new((SP_IMEM_SIZE_BYTES - 1) as u32))
            .is_ok());
        assert_eq!(
            sp_imem.observe_byte(SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32)),
            Err(SpImemReadError::OutOfRange {
                offset: SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32),
                width: 1,
            })
        );
    }

    #[test]
    fn zero_backing_does_not_make_an_aligned_word_architecturally_known() {
        let sp_imem = SpImem::default();

        assert_eq!(
            sp_imem.read_known_u32_be(SpImemOffset::new(0)),
            Err(SpImemReadError::UnknownByte {
                offset: SpImemOffset::new(0),
                width: 4,
                unknown_offset: SpImemOffset::new(0),
            })
        );
    }

    #[test]
    fn generated_known_words_read_first_last_and_n64_big_endian_order() {
        let mut sp_imem = SpImem::default();
        let last = SpImemOffset::new((SP_IMEM_SIZE_BYTES - 4) as u32);

        sp_imem
            .stage_known_u32_be_for_test(SpImemOffset::new(0), 0x0123_4567)
            .unwrap();
        sp_imem
            .stage_known_u32_be_for_test(last, 0x89ab_cdef)
            .unwrap();

        let first_word = sp_imem.read_known_u32_be(SpImemOffset::new(0)).unwrap();
        let last_word = sp_imem.read_known_u32_be(last).unwrap();
        assert_eq!(first_word.value(), 0x0123_4567);
        assert_eq!(last_word.value(), 0x89ab_cdef);
        assert!(first_word
            .byte_provenance()
            .iter()
            .all(|source| { *source == SpImemByteProvenance::GeneratedMachineTestStaging }));
        assert_eq!(
            sp_imem.observe_byte(SpImemOffset::new(0)).unwrap().value(),
            0x01
        );
        assert_eq!(
            sp_imem.observe_byte(SpImemOffset::new(3)).unwrap().value(),
            0x67
        );
    }

    #[test]
    fn aligned_word_read_rejects_unaligned_and_out_of_range_offsets() {
        let sp_imem = SpImem::default();

        assert_eq!(
            sp_imem.read_known_u32_be(SpImemOffset::new(1)),
            Err(SpImemReadError::Unaligned {
                offset: SpImemOffset::new(1),
                width: 4,
            })
        );
        assert_eq!(
            sp_imem.read_known_u32_be(SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32)),
            Err(SpImemReadError::OutOfRange {
                offset: SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32),
                width: 4,
            })
        );
    }

    #[test]
    fn word_knownness_requires_all_four_consumed_bytes() {
        let mut sp_imem = SpImem::default();

        sp_imem
            .stage_known_bytes_for_test(SpImemOffset::new(0), &[0x11, 0x22, 0x33])
            .unwrap();

        assert_eq!(
            sp_imem.read_known_u32_be(SpImemOffset::new(0)),
            Err(SpImemReadError::UnknownByte {
                offset: SpImemOffset::new(0),
                width: 4,
                unknown_offset: SpImemOffset::new(3),
            })
        );
    }

    #[test]
    fn profiled_pif_copy_creates_byte_exact_known_range_and_source_offsets() {
        let bytes = generated_pif_firmware(0x29);
        let firmware = PifFirmware::from_owned_bytes(bytes.clone()).unwrap();
        let layout = PifIpl2Profile::NtscPinned.copy_layout();
        let sp_imem =
            SpImem::from_pif_ipl2_copy(firmware.ipl2_copy(PifIpl2Profile::NtscPinned)).unwrap();

        for destination_offset in 0..layout.byte_count() {
            let observation = sp_imem
                .observe_byte(SpImemOffset::new(destination_offset as u32))
                .unwrap();
            let source_offset = layout.source_start_offset() + destination_offset as u32;
            assert_eq!(observation.value(), bytes[source_offset as usize]);
            assert_eq!(
                observation.provenance(),
                SpImemByteProvenance::UserSuppliedPifFirmware {
                    profile: PifIpl2Profile::NtscPinned,
                    source_offset,
                }
            );
        }

        let first_untouched = sp_imem
            .observe_byte(SpImemOffset::new(layout.sp_imem_end_offset_exclusive()))
            .unwrap();
        assert_eq!(first_untouched.value(), 0);
        assert_eq!(first_untouched.provenance(), SpImemByteProvenance::Unknown);
    }
}
