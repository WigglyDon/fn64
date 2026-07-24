use core::fmt;
use std::collections::BTreeMap;

use crate::machine::{MachineSpImemOpaqueWordState, MachineSpImemStoreWordProvenance};
use crate::pif_firmware::{PifFirmwareClassification, PifIpl2Copy, PifIpl2Profile};

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
    PublicSyntheticColdX105Bootstrap {
        profile: PifIpl2Profile,
        source_offset: u32,
    },
    CpuStoreWord {
        provenance: MachineSpImemStoreWordProvenance,
    },
    CpuStoreByte {
        provenance: MachineSpImemStoreWordProvenance,
    },
    SpDma {
        record_index: u8,
    },
    OpaqueCpuStoreWord {
        aligned_offset: SpImemOffset,
    },
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

impl SpImemByteProvenance {
    pub(crate) const fn is_known(self) -> bool {
        !matches!(self, Self::Unknown | Self::OpaqueCpuStoreWord { .. })
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
    OpaqueWord {
        offset: SpImemOffset,
        state: MachineSpImemOpaqueWordState,
    },
    InconsistentOpaqueWord {
        offset: SpImemOffset,
    },
}

impl SpImemReadError {
    pub(crate) const fn unknown_offset(self) -> Option<SpImemOffset> {
        match self {
            Self::UnknownByte { unknown_offset, .. } => Some(unknown_offset),
            Self::Unaligned { .. }
            | Self::OutOfRange { .. }
            | Self::OpaqueWord { .. }
            | Self::InconsistentOpaqueWord { .. } => None,
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
            Self::OpaqueWord { offset, .. } => write!(
                f,
                "SP IMEM word value unavailable after opaque CPU store: offset={} width=4",
                offset.value()
            ),
            Self::InconsistentOpaqueWord { offset } => write!(
                f,
                "SP IMEM opaque word knowledge is inconsistent: offset={} width=4",
                offset.value()
            ),
        }
    }
}

impl std::error::Error for SpImemReadError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpImemStoreWordError {
    Unaligned { offset: SpImemOffset },
    OutOfRange { offset: SpImemOffset },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpImemStoreByteError {
    OutOfRange { offset: SpImemOffset },
    OpaqueWordPartialOverwrite { aligned_offset: SpImemOffset },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpImemStoreBytePlan {
    offset: SpImemOffset,
    value: u8,
    provenance: MachineSpImemStoreWordProvenance,
}

impl SpImemStoreBytePlan {
    pub(crate) const fn offset(self) -> SpImemOffset {
        self.offset
    }

    pub(crate) const fn value(self) -> u8 {
        self.value
    }

    pub(crate) const fn provenance(self) -> MachineSpImemStoreWordProvenance {
        self.provenance
    }
}

impl fmt::Display for SpImemStoreWordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unaligned { offset } => write!(
                f,
                "SP IMEM word store requires aligned local offset: {}",
                offset.value()
            ),
            Self::OutOfRange { offset } => write!(
                f,
                "SP IMEM word store exceeds local range: {}",
                offset.value()
            ),
        }
    }
}

impl std::error::Error for SpImemStoreWordError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpImemStoreWordPlan {
    offset: SpImemOffset,
    bytes: [u8; 4],
    provenance: MachineSpImemStoreWordProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpImemOpaqueStoreWordPlan {
    offset: SpImemOffset,
    state: MachineSpImemOpaqueWordState,
}

impl SpImemOpaqueStoreWordPlan {
    pub(crate) const fn offset(self) -> SpImemOffset {
        self.offset
    }

    pub(crate) const fn state(self) -> MachineSpImemOpaqueWordState {
        self.state
    }
}

impl SpImemStoreWordPlan {
    pub(crate) const fn offset(self) -> SpImemOffset {
        self.offset
    }

    pub(crate) const fn bytes(self) -> [u8; 4] {
        self.bytes
    }

    pub(crate) const fn provenance(self) -> MachineSpImemStoreWordProvenance {
        self.provenance
    }
}

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

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct SpImem {
    bytes: [u8; SP_IMEM_SIZE_BYTES],
    byte_provenance: [SpImemByteProvenance; SP_IMEM_SIZE_BYTES],
    opaque_words: BTreeMap<u32, MachineSpImemOpaqueWordState>,
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
            let source_offset = layout.source_start_offset() + index as u32;
            *provenance = match copy.classification() {
                PifFirmwareClassification::RawBootRom => {
                    SpImemByteProvenance::UserSuppliedPifFirmware {
                        profile: copy.profile(),
                        source_offset,
                    }
                }
                PifFirmwareClassification::PublicSyntheticColdX105Bootstrap => {
                    SpImemByteProvenance::PublicSyntheticColdX105Bootstrap {
                        profile: copy.profile(),
                        source_offset,
                    }
                }
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
        if let Some(state) = self.opaque_word_state_from_start(offset, offset_usize)? {
            return Err(SpImemReadError::OpaqueWord { offset, state });
        }
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

    pub(crate) fn read_known_u8(&self, offset: SpImemOffset) -> Result<u8, SpImemReadError> {
        let offset_usize = self.require_span(offset, 1)?;
        let provenance = self.byte_provenance[offset_usize];
        if !provenance.is_known() {
            return Err(SpImemReadError::UnknownByte {
                offset,
                width: 1,
                unknown_offset: offset,
            });
        }
        Ok(self.bytes[offset_usize])
    }

    pub(crate) fn opaque_word_state(
        &self,
        offset: SpImemOffset,
    ) -> Result<Option<MachineSpImemOpaqueWordState>, SpImemReadError> {
        if (offset.value() & 0x3) != 0 {
            return Err(SpImemReadError::Unaligned { offset, width: 4 });
        }
        let offset_usize = self.require_span(offset, 4)?;
        self.opaque_word_state_from_start(offset, offset_usize)
    }

    pub(crate) fn plan_cpu_store_word(
        &self,
        offset: SpImemOffset,
        value: u32,
        provenance: MachineSpImemStoreWordProvenance,
    ) -> Result<SpImemStoreWordPlan, SpImemStoreWordError> {
        if (offset.value() & 0x3) != 0 {
            return Err(SpImemStoreWordError::Unaligned { offset });
        }

        let offset_usize = offset.as_usize();
        let Some(end) = offset_usize.checked_add(4) else {
            return Err(SpImemStoreWordError::OutOfRange { offset });
        };
        if end > self.bytes.len() {
            return Err(SpImemStoreWordError::OutOfRange { offset });
        }

        Ok(SpImemStoreWordPlan {
            offset,
            bytes: value.to_be_bytes(),
            provenance,
        })
    }

    pub(crate) fn apply_cpu_store_word(&mut self, plan: SpImemStoreWordPlan) {
        let start = plan.offset().as_usize();
        let end = start + plan.bytes().len();
        self.opaque_words.remove(&plan.offset().value());
        self.bytes[start..end].copy_from_slice(&plan.bytes());
        self.byte_provenance[start..end].fill(SpImemByteProvenance::CpuStoreWord {
            provenance: plan.provenance(),
        });
    }

    pub(crate) fn plan_cpu_store_byte(
        &self,
        offset: SpImemOffset,
        value: u8,
        provenance: MachineSpImemStoreWordProvenance,
    ) -> Result<SpImemStoreBytePlan, SpImemStoreByteError> {
        let offset_usize = offset.as_usize();
        if offset_usize >= self.bytes.len() {
            return Err(SpImemStoreByteError::OutOfRange { offset });
        }
        let aligned_offset = SpImemOffset::new(offset.value() & !3);
        if self.opaque_words.contains_key(&aligned_offset.value()) {
            return Err(SpImemStoreByteError::OpaqueWordPartialOverwrite { aligned_offset });
        }
        Ok(SpImemStoreBytePlan {
            offset,
            value,
            provenance,
        })
    }

    pub(crate) fn apply_cpu_store_byte(&mut self, plan: SpImemStoreBytePlan) {
        let offset = plan.offset().as_usize();
        self.bytes[offset] = plan.value();
        self.byte_provenance[offset] = SpImemByteProvenance::CpuStoreByte {
            provenance: plan.provenance(),
        };
    }

    pub(crate) fn apply_sp_dma_byte(&mut self, offset: SpImemOffset, value: u8, record_index: u8) {
        let offset_usize = offset.as_usize();
        self.opaque_words.remove(&(offset.value() & !3));
        self.bytes[offset_usize] = value;
        self.byte_provenance[offset_usize] = SpImemByteProvenance::SpDma { record_index };
    }

    pub(crate) fn plan_cpu_store_opaque_word(
        &self,
        offset: SpImemOffset,
        state: MachineSpImemOpaqueWordState,
    ) -> Result<SpImemOpaqueStoreWordPlan, SpImemStoreWordError> {
        if (offset.value() & 0x3) != 0 {
            return Err(SpImemStoreWordError::Unaligned { offset });
        }
        if state.aligned_offset() != offset.value() {
            return Err(SpImemStoreWordError::OutOfRange { offset });
        }

        let offset_usize = offset.as_usize();
        let Some(end) = offset_usize.checked_add(4) else {
            return Err(SpImemStoreWordError::OutOfRange { offset });
        };
        if end > self.bytes.len() {
            return Err(SpImemStoreWordError::OutOfRange { offset });
        }

        Ok(SpImemOpaqueStoreWordPlan { offset, state })
    }

    pub(crate) fn apply_cpu_store_opaque_word(&mut self, plan: SpImemOpaqueStoreWordPlan) {
        let start = plan.offset().as_usize();
        let end = start + 4;
        self.opaque_words
            .insert(plan.offset().value(), plan.state());
        self.bytes[start..end].fill(0);
        self.byte_provenance[start..end].fill(SpImemByteProvenance::OpaqueCpuStoreWord {
            aligned_offset: plan.offset(),
        });
    }

    fn opaque_word_state_from_start(
        &self,
        offset: SpImemOffset,
        offset_usize: usize,
    ) -> Result<Option<MachineSpImemOpaqueWordState>, SpImemReadError> {
        let provenance =
            core::array::from_fn::<_, 4, _>(|index| self.byte_provenance[offset_usize + index]);
        let first_opaque = provenance.iter().find_map(|entry| match *entry {
            SpImemByteProvenance::OpaqueCpuStoreWord { aligned_offset } => Some(aligned_offset),
            _ => None,
        });
        let Some(aligned_offset) = first_opaque else {
            return Ok(None);
        };
        if aligned_offset != offset
            || provenance
                .iter()
                .any(|entry| *entry != SpImemByteProvenance::OpaqueCpuStoreWord { aligned_offset })
        {
            return Err(SpImemReadError::InconsistentOpaqueWord { offset });
        }
        let Some(state) = self.opaque_words.get(&offset.value()).copied() else {
            return Err(SpImemReadError::InconsistentOpaqueWord { offset });
        };
        if state.aligned_offset() != offset.value() {
            return Err(SpImemReadError::InconsistentOpaqueWord { offset });
        }
        Ok(Some(state))
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
            opaque_words: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::MachineBootstrapGprSource;
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

    #[test]
    fn cpu_store_word_plan_is_bounded_big_endian_and_replaces_only_selected_provenance() {
        let mut sp_imem = SpImem::default();
        sp_imem
            .stage_known_u32_be_for_test(SpImemOffset::new(0), 0x1112_1314)
            .unwrap();
        sp_imem
            .stage_known_u32_be_for_test(SpImemOffset::new(8), 0x2122_2324)
            .unwrap();
        let provenance = MachineSpImemStoreWordProvenance::new(
            crate::cpu::address::CpuAddress::new(0xa400_0050),
            10,
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: crate::cpu::address::CpuAddress::new(0xa400_004c),
                identity: crate::cpu::CpuInstructionIdentity::SpecialXor,
                source_gpr_a: Some(10),
                source_gpr_b: Some(8),
            },
        );

        let plan = sp_imem
            .plan_cpu_store_word(SpImemOffset::new(4), 0x89ab_cdef, provenance)
            .unwrap();
        assert_eq!(plan.bytes(), [0x89, 0xab, 0xcd, 0xef]);
        sp_imem.apply_cpu_store_word(plan);

        assert_eq!(
            sp_imem
                .read_known_u32_be(SpImemOffset::new(4))
                .unwrap()
                .value(),
            0x89ab_cdef
        );
        for (index, expected) in [0x89, 0xab, 0xcd, 0xef].into_iter().enumerate() {
            let observed = sp_imem
                .observe_byte(SpImemOffset::new(4 + index as u32))
                .unwrap();
            assert_eq!(observed.value(), expected);
            assert_eq!(
                observed.provenance(),
                SpImemByteProvenance::CpuStoreWord { provenance }
            );
        }
        assert_eq!(
            sp_imem.observe_byte(SpImemOffset::new(3)).unwrap().value(),
            0x14
        );
        assert_eq!(
            sp_imem
                .observe_byte(SpImemOffset::new(3))
                .unwrap()
                .provenance(),
            SpImemByteProvenance::GeneratedMachineTestStaging
        );
        assert_eq!(
            sp_imem.observe_byte(SpImemOffset::new(8)).unwrap().value(),
            0x21
        );
        assert_eq!(
            sp_imem
                .observe_byte(SpImemOffset::new(8))
                .unwrap()
                .provenance(),
            SpImemByteProvenance::GeneratedMachineTestStaging
        );
    }

    #[test]
    fn cpu_store_word_plan_rejects_unaligned_and_past_end_without_mutation() {
        let sp_imem = SpImem::default();
        let provenance = MachineSpImemStoreWordProvenance::new(
            crate::cpu::address::CpuAddress::new(0xa400_0050),
            0,
            MachineBootstrapGprSource::ArchitecturalZero,
        );

        assert_eq!(
            sp_imem.plan_cpu_store_word(SpImemOffset::new(1), 0, provenance),
            Err(SpImemStoreWordError::Unaligned {
                offset: SpImemOffset::new(1),
            })
        );
        assert_eq!(
            sp_imem.plan_cpu_store_word(
                SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32),
                0,
                provenance,
            ),
            Err(SpImemStoreWordError::OutOfRange {
                offset: SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32),
            })
        );
        assert_eq!(
            sp_imem
                .observe_byte(SpImemOffset::new(0))
                .unwrap()
                .provenance(),
            SpImemByteProvenance::Unknown
        );
    }

    fn opaque_state(
        offset: u32,
        instruction_pc: u32,
        source_gpr: u8,
    ) -> MachineSpImemOpaqueWordState {
        MachineSpImemOpaqueWordState::from_cpu_store(
            offset,
            crate::cpu::address::CpuAddress::new(instruction_pc),
            source_gpr,
            MachineBootstrapGprSource::UnknownPifProduced,
            0xffff_ffff_a400_1000 + u64::from(offset),
            crate::cpu::address::CpuAddress::new(0xa400_1000 + offset),
            0x0400_1000 + offset,
        )
    }

    #[test]
    fn opaque_word_canonicalizes_private_bytes_and_exposes_only_one_coherent_state() {
        let mut sp_imem = SpImem::default();
        sp_imem
            .stage_known_u32_be_for_test(SpImemOffset::new(4), 0x89ab_cdef)
            .unwrap();
        let state = opaque_state(4, 0xa400_0890, 2);

        let plan = sp_imem
            .plan_cpu_store_opaque_word(SpImemOffset::new(4), state)
            .unwrap();
        sp_imem.apply_cpu_store_opaque_word(plan);

        assert_eq!(
            sp_imem.opaque_word_state(SpImemOffset::new(4)),
            Ok(Some(state))
        );
        assert_eq!(
            sp_imem.read_known_u32_be(SpImemOffset::new(4)),
            Err(SpImemReadError::OpaqueWord {
                offset: SpImemOffset::new(4),
                state,
            })
        );
        for offset in 4..8 {
            let observation = sp_imem.observe_byte(SpImemOffset::new(offset)).unwrap();
            assert_eq!(observation.value(), 0);
            assert!(!observation.is_known());
            assert_eq!(
                observation.provenance(),
                SpImemByteProvenance::OpaqueCpuStoreWord {
                    aligned_offset: SpImemOffset::new(4),
                }
            );
        }
    }

    #[test]
    fn identical_opaque_store_erases_prior_byte_ghosts_and_replaces_provenance_once() {
        let mut first = SpImem::default();
        let mut second = SpImem::default();
        first
            .stage_known_u32_be_for_test(SpImemOffset::new(8), 0x1112_1314)
            .unwrap();
        second
            .stage_known_u32_be_for_test(SpImemOffset::new(8), 0xa1a2_a3a4)
            .unwrap();
        let first_state = opaque_state(8, 0xa400_0894, 3);
        for owner in [&mut first, &mut second] {
            let plan = owner
                .plan_cpu_store_opaque_word(SpImemOffset::new(8), first_state)
                .unwrap();
            owner.apply_cpu_store_opaque_word(plan);
        }
        assert!(first == second);

        let replacement = opaque_state(8, 0xa400_0990, 7);
        let plan = first
            .plan_cpu_store_opaque_word(SpImemOffset::new(8), replacement)
            .unwrap();
        first.apply_cpu_store_opaque_word(plan);
        assert_eq!(
            first.opaque_word_state(SpImemOffset::new(8)),
            Ok(Some(replacement))
        );
        assert!(first != second);
    }

    #[test]
    fn known_full_word_overwrite_replaces_only_selected_opaque_word() {
        let mut sp_imem = SpImem::default();
        for (offset, pc, gpr) in [(0, 0xa400_0890, 2), (4, 0xa400_0894, 3)] {
            let state = opaque_state(offset, pc, gpr);
            let plan = sp_imem
                .plan_cpu_store_opaque_word(SpImemOffset::new(offset), state)
                .unwrap();
            sp_imem.apply_cpu_store_opaque_word(plan);
        }
        let provenance = MachineSpImemStoreWordProvenance::new(
            crate::cpu::address::CpuAddress::new(0xa400_08a0),
            6,
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: crate::cpu::address::CpuAddress::new(0xa400_0150),
                identity: crate::cpu::CpuInstructionIdentity::Lui,
                source_gpr_a: None,
                source_gpr_b: None,
            },
        );
        let plan = sp_imem
            .plan_cpu_store_word(SpImemOffset::new(0), 0x0123_4567, provenance)
            .unwrap();
        sp_imem.apply_cpu_store_word(plan);

        assert_eq!(sp_imem.opaque_word_state(SpImemOffset::new(0)), Ok(None));
        assert_eq!(
            sp_imem
                .read_known_u32_be(SpImemOffset::new(0))
                .unwrap()
                .value(),
            0x0123_4567
        );
        assert_eq!(
            sp_imem.opaque_word_state(SpImemOffset::new(4)),
            Ok(Some(opaque_state(4, 0xa400_0894, 3)))
        );
    }

    #[test]
    fn opaque_plan_and_coherence_reject_invalid_owner_state() {
        let mut sp_imem = SpImem::default();
        let state = opaque_state(0, 0xa400_0890, 2);
        assert_eq!(
            sp_imem.plan_cpu_store_opaque_word(SpImemOffset::new(1), state),
            Err(SpImemStoreWordError::Unaligned {
                offset: SpImemOffset::new(1),
            })
        );
        assert_eq!(
            sp_imem.plan_cpu_store_opaque_word(
                SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32),
                opaque_state(SP_IMEM_SIZE_BYTES as u32, 0xa400_0890, 2),
            ),
            Err(SpImemStoreWordError::OutOfRange {
                offset: SpImemOffset::new(SP_IMEM_SIZE_BYTES as u32),
            })
        );

        let plan = sp_imem
            .plan_cpu_store_opaque_word(SpImemOffset::new(0), state)
            .unwrap();
        sp_imem.apply_cpu_store_opaque_word(plan);
        sp_imem.byte_provenance[3] = SpImemByteProvenance::Unknown;
        assert_eq!(
            sp_imem.opaque_word_state(SpImemOffset::new(0)),
            Err(SpImemReadError::InconsistentOpaqueWord {
                offset: SpImemOffset::new(0),
            })
        );
        assert_eq!(
            sp_imem.read_known_u32_be(SpImemOffset::new(0)),
            Err(SpImemReadError::InconsistentOpaqueWord {
                offset: SpImemOffset::new(0),
            })
        );
    }
}
