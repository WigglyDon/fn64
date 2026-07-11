use core::fmt;

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
}
