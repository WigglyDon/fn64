use super::address::CpuAddress;
use super::Cpu;
use crate::machine::MachineBootstrapGprSource;

pub const PRIMARY_INSTRUCTION_CACHE_SIZE_BYTES: usize = 0x4000;
pub const PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES: usize = 0x20;
pub const PRIMARY_INSTRUCTION_CACHE_LINE_COUNT: usize =
    PRIMARY_INSTRUCTION_CACHE_SIZE_BYTES / PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES;
pub const PRIMARY_DATA_CACHE_SIZE_BYTES: usize = 0x2000;
pub const PRIMARY_DATA_CACHE_LINE_SIZE_BYTES: usize = 0x10;
pub const PRIMARY_DATA_CACHE_LINE_COUNT: usize =
    PRIMARY_DATA_CACHE_SIZE_BYTES / PRIMARY_DATA_CACHE_LINE_SIZE_BYTES;

const PRIMARY_CACHE_TAG_LO_PHYSICAL_TAG_MASK: u32 = 0x0fff_ff00;
const PRIMARY_CACHE_TAG_LO_STATE_SHIFT: u32 = 6;
const PRIMARY_CACHE_TAG_LO_STATE_MASK: u32 = 0x3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop0TagWriteProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
}

impl MachineCop0TagWriteProvenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        Self {
            instruction_pc,
            source_gpr,
            source_lineage,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn source_gpr(self) -> u8 {
        self.source_gpr
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        self.source_lineage
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop0TagState {
    raw_word: u32,
    provenance: MachineCop0TagWriteProvenance,
}

impl MachineCop0TagState {
    pub(crate) const fn new(raw_word: u32, provenance: MachineCop0TagWriteProvenance) -> Self {
        Self {
            raw_word,
            provenance,
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn provenance(self) -> MachineCop0TagWriteProvenance {
        self.provenance
    }

    pub const fn primary_state(self) -> u8 {
        ((self.raw_word >> PRIMARY_CACHE_TAG_LO_STATE_SHIFT) & PRIMARY_CACHE_TAG_LO_STATE_MASK)
            as u8
    }

    pub const fn primary_physical_tag(self) -> u32 {
        (self.raw_word & PRIMARY_CACHE_TAG_LO_PHYSICAL_TAG_MASK) << 4
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePrimaryCacheIndexStoreTagTarget {
    Instruction,
    Data,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePrimaryCacheOperationProvenance {
    instruction_pc: CpuAddress,
    raw_instruction_word: u32,
    target: MachinePrimaryCacheIndexStoreTagTarget,
    base_gpr: u8,
    base_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    line_index: u16,
    tag_lo: MachineCop0TagState,
    tag_hi: MachineCop0TagState,
    delay_slot_owner: Option<CpuAddress>,
}

impl MachinePrimaryCacheOperationProvenance {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        raw_instruction_word: u32,
        target: MachinePrimaryCacheIndexStoreTagTarget,
        base_gpr: u8,
        base_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        line_index: u16,
        tag_lo: MachineCop0TagState,
        tag_hi: MachineCop0TagState,
        delay_slot_owner: Option<CpuAddress>,
    ) -> Self {
        Self {
            instruction_pc,
            raw_instruction_word,
            target,
            base_gpr,
            base_lineage,
            effective_address,
            cpu_address,
            line_index,
            tag_lo,
            tag_hi,
            delay_slot_owner,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn raw_instruction_word(self) -> u32 {
        self.raw_instruction_word
    }

    pub const fn target(self) -> MachinePrimaryCacheIndexStoreTagTarget {
        self.target
    }

    pub const fn base_gpr(self) -> u8 {
        self.base_gpr
    }

    pub const fn base_lineage(self) -> MachineBootstrapGprSource {
        self.base_lineage
    }

    pub const fn effective_address(self) -> u64 {
        self.effective_address
    }

    pub const fn cpu_address(self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn line_index(self) -> u16 {
        self.line_index
    }

    pub const fn tag_lo(self) -> MachineCop0TagState {
        self.tag_lo
    }

    pub const fn tag_hi(self) -> MachineCop0TagState {
        self.tag_hi
    }

    pub const fn delay_slot_owner(self) -> Option<CpuAddress> {
        self.delay_slot_owner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePrimaryInstructionCacheFillProvenance {
    requested_cpu_address: CpuAddress,
    physical_line_address: u32,
}

impl MachinePrimaryInstructionCacheFillProvenance {
    pub(crate) const fn new(requested_cpu_address: CpuAddress, physical_line_address: u32) -> Self {
        Self {
            requested_cpu_address,
            physical_line_address,
        }
    }

    pub const fn requested_cpu_address(self) -> CpuAddress {
        self.requested_cpu_address
    }

    pub const fn physical_line_address(self) -> u32 {
        self.physical_line_address
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePrimaryInstructionCacheLineState {
    Unavailable,
    Invalid {
        provenance: MachinePrimaryCacheOperationProvenance,
    },
    ValidDataUnavailable {
        physical_tag: u32,
        provenance: MachinePrimaryCacheOperationProvenance,
    },
    Valid {
        physical_tag: u32,
        data: [u8; PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES],
        provenance: MachinePrimaryInstructionCacheFillProvenance,
    },
}

impl MachinePrimaryInstructionCacheLineState {
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid { .. })
    }

    pub const fn is_valid(self) -> bool {
        matches!(self, Self::ValidDataUnavailable { .. } | Self::Valid { .. })
    }

    pub const fn physical_tag(self) -> Option<u32> {
        match self {
            Self::ValidDataUnavailable { physical_tag, .. } | Self::Valid { physical_tag, .. } => {
                Some(physical_tag)
            }
            Self::Unavailable | Self::Invalid { .. } => None,
        }
    }

    pub const fn operation_provenance(self) -> Option<MachinePrimaryCacheOperationProvenance> {
        match self {
            Self::Invalid { provenance } | Self::ValidDataUnavailable { provenance, .. } => {
                Some(provenance)
            }
            Self::Unavailable | Self::Valid { .. } => None,
        }
    }

    pub const fn fill_provenance(self) -> Option<MachinePrimaryInstructionCacheFillProvenance> {
        match self {
            Self::Valid { provenance, .. } => Some(provenance),
            Self::Unavailable | Self::Invalid { .. } | Self::ValidDataUnavailable { .. } => None,
        }
    }

    pub const fn data(self) -> Option<[u8; PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES]> {
        match self {
            Self::Valid { data, .. } => Some(data),
            Self::Unavailable | Self::Invalid { .. } | Self::ValidDataUnavailable { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePrimaryDataCacheLineState {
    Unavailable,
    Invalid {
        provenance: MachinePrimaryCacheOperationProvenance,
    },
    ValidClean {
        physical_tag: u32,
        provenance: MachinePrimaryCacheOperationProvenance,
    },
    ValidDirty {
        physical_tag: u32,
        provenance: MachinePrimaryCacheOperationProvenance,
    },
}

impl MachinePrimaryDataCacheLineState {
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid { .. })
    }

    pub const fn is_valid_clean(self) -> bool {
        matches!(self, Self::ValidClean { .. })
    }

    pub const fn is_valid_dirty(self) -> bool {
        matches!(self, Self::ValidDirty { .. })
    }

    pub const fn physical_tag(self) -> Option<u32> {
        match self {
            Self::ValidClean { physical_tag, .. } | Self::ValidDirty { physical_tag, .. } => {
                Some(physical_tag)
            }
            Self::Unavailable | Self::Invalid { .. } => None,
        }
    }

    pub const fn operation_provenance(self) -> Option<MachinePrimaryCacheOperationProvenance> {
        match self {
            Self::Invalid { provenance }
            | Self::ValidClean { provenance, .. }
            | Self::ValidDirty { provenance, .. } => Some(provenance),
            Self::Unavailable => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachinePrimaryCaches {
    instruction_lines:
        [MachinePrimaryInstructionCacheLineState; PRIMARY_INSTRUCTION_CACHE_LINE_COUNT],
    data_lines: [MachinePrimaryDataCacheLineState; PRIMARY_DATA_CACHE_LINE_COUNT],
}

impl MachinePrimaryCaches {
    pub(crate) const fn new() -> Self {
        Self {
            instruction_lines: [MachinePrimaryInstructionCacheLineState::Unavailable;
                PRIMARY_INSTRUCTION_CACHE_LINE_COUNT],
            data_lines: [MachinePrimaryDataCacheLineState::Unavailable;
                PRIMARY_DATA_CACHE_LINE_COUNT],
        }
    }

    pub const fn instruction_line_count(&self) -> usize {
        self.instruction_lines.len()
    }

    pub const fn data_line_count(&self) -> usize {
        self.data_lines.len()
    }

    pub fn instruction_line(
        &self,
        index: usize,
    ) -> Option<MachinePrimaryInstructionCacheLineState> {
        self.instruction_lines.get(index).copied()
    }

    pub fn data_line(&self, index: usize) -> Option<MachinePrimaryDataCacheLineState> {
        self.data_lines.get(index).copied()
    }

    pub(crate) fn apply_index_store_tag(
        &mut self,
        provenance: MachinePrimaryCacheOperationProvenance,
    ) {
        let line_index = usize::from(provenance.line_index());
        let tag_state = provenance.tag_lo().primary_state();
        let physical_tag = provenance.tag_lo().primary_physical_tag();
        match provenance.target() {
            MachinePrimaryCacheIndexStoreTagTarget::Instruction => {
                self.instruction_lines[line_index] = if tag_state == 0 {
                    MachinePrimaryInstructionCacheLineState::Invalid { provenance }
                } else {
                    MachinePrimaryInstructionCacheLineState::ValidDataUnavailable {
                        physical_tag,
                        provenance,
                    }
                };
            }
            MachinePrimaryCacheIndexStoreTagTarget::Data => {
                self.data_lines[line_index] = match tag_state {
                    0 => MachinePrimaryDataCacheLineState::Invalid { provenance },
                    2 => MachinePrimaryDataCacheLineState::ValidClean {
                        physical_tag,
                        provenance,
                    },
                    3 => MachinePrimaryDataCacheLineState::ValidDirty {
                        physical_tag,
                        provenance,
                    },
                    _ => unreachable!("unsupported primary D-cache tag state was preflighted"),
                };
            }
        }
    }

    pub(crate) fn lookup_instruction_word(&self, physical_address: u32) -> Option<u32> {
        let line_index = primary_instruction_cache_line_index(physical_address);
        let expected_tag = primary_instruction_cache_physical_tag(physical_address);
        let MachinePrimaryInstructionCacheLineState::Valid {
            physical_tag, data, ..
        } = self.instruction_lines[line_index]
        else {
            return None;
        };
        if physical_tag != expected_tag {
            return None;
        }
        let word_offset =
            (physical_address as usize) & (PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES - 1);
        Some(u32::from_be_bytes(
            data[word_offset..word_offset + 4]
                .try_into()
                .expect("aligned instruction word fits inside one cache line"),
        ))
    }

    pub(crate) fn apply_instruction_fill(
        &mut self,
        plan: MachinePrimaryInstructionCacheFillPlan,
    ) -> MachinePrimaryInstructionCacheLineState {
        let previous = self.instruction_lines[plan.line_index];
        self.instruction_lines[plan.line_index] = MachinePrimaryInstructionCacheLineState::Valid {
            physical_tag: plan.physical_tag,
            data: plan.data,
            provenance: plan.provenance,
        };
        previous
    }

    pub(crate) fn restore_instruction_line(
        &mut self,
        line_index: usize,
        state: MachinePrimaryInstructionCacheLineState,
    ) {
        self.instruction_lines[line_index] = state;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryInstructionCacheFillPlan {
    line_index: usize,
    physical_tag: u32,
    data: [u8; PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES],
    provenance: MachinePrimaryInstructionCacheFillProvenance,
}

impl MachinePrimaryInstructionCacheFillPlan {
    pub(crate) const fn new(
        requested_cpu_address: CpuAddress,
        physical_line_address: u32,
        data: [u8; PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES],
    ) -> Self {
        Self {
            line_index: primary_instruction_cache_line_index(physical_line_address),
            physical_tag: primary_instruction_cache_physical_tag(physical_line_address),
            data,
            provenance: MachinePrimaryInstructionCacheFillProvenance::new(
                requested_cpu_address,
                physical_line_address,
            ),
        }
    }

    pub(crate) const fn line_index(self) -> usize {
        self.line_index
    }

    pub(crate) const fn requested_word(self, physical_address: u32) -> u32 {
        let word_offset =
            (physical_address as usize) & (PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES - 1);
        u32::from_be_bytes([
            self.data[word_offset],
            self.data[word_offset + 1],
            self.data[word_offset + 2],
            self.data[word_offset + 3],
        ])
    }
}

pub(crate) const fn primary_instruction_cache_line_index(physical_address: u32) -> usize {
    ((physical_address as usize) / PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES)
        & (PRIMARY_INSTRUCTION_CACHE_LINE_COUNT - 1)
}

pub(crate) const fn primary_data_cache_line_index(physical_address: u32) -> usize {
    ((physical_address as usize) / PRIMARY_DATA_CACHE_LINE_SIZE_BYTES)
        & (PRIMARY_DATA_CACHE_LINE_COUNT - 1)
}

const fn primary_instruction_cache_physical_tag(physical_address: u32) -> u32 {
    physical_address & !((PRIMARY_INSTRUCTION_CACHE_SIZE_BYTES as u32) - 1)
}

impl Cpu {
    pub fn primary_caches(&self) -> &MachinePrimaryCaches {
        &self.primary_caches
    }

    pub(crate) fn apply_primary_cache_index_store_tag(
        &mut self,
        provenance: MachinePrimaryCacheOperationProvenance,
    ) {
        self.primary_caches.apply_index_store_tag(provenance);
    }

    pub(crate) fn lookup_primary_instruction_cache_word(
        &self,
        physical_address: u32,
    ) -> Option<u32> {
        self.primary_caches
            .lookup_instruction_word(physical_address)
    }

    pub(crate) fn apply_primary_instruction_cache_fill(
        &mut self,
        plan: MachinePrimaryInstructionCacheFillPlan,
    ) -> MachinePrimaryInstructionCacheLineState {
        self.primary_caches.apply_instruction_fill(plan)
    }

    pub(crate) fn restore_primary_instruction_cache_line(
        &mut self,
        line_index: usize,
        state: MachinePrimaryInstructionCacheLineState,
    ) {
        self.primary_caches
            .restore_instruction_line(line_index, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CpuInstructionIdentity;

    fn tag_state(raw_word: u32) -> MachineCop0TagState {
        MachineCop0TagState::new(
            raw_word,
            MachineCop0TagWriteProvenance::new(
                CpuAddress::new(0xa400_0400),
                0,
                MachineBootstrapGprSource::ArchitecturalZero,
            ),
        )
    }

    fn provenance(
        target: MachinePrimaryCacheIndexStoreTagTarget,
        line_index: u16,
        tag_lo_word: u32,
    ) -> MachinePrimaryCacheOperationProvenance {
        MachinePrimaryCacheOperationProvenance::new(
            CpuAddress::new(0xa400_0408),
            0xbd08_0000,
            target,
            8,
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(0xa400_03f4),
                identity: CpuInstructionIdentity::Addiu,
                source_gpr_a: Some(8),
                source_gpr_b: None,
            },
            0xffff_ffff_8000_0000,
            CpuAddress::new(0x8000_0000),
            line_index,
            tag_state(tag_lo_word),
            tag_state(0),
            None,
        )
    }

    #[test]
    fn construction_owns_exact_primary_cache_geometry_as_unavailable_truth() {
        let caches = MachinePrimaryCaches::new();
        assert_eq!(PRIMARY_INSTRUCTION_CACHE_SIZE_BYTES, 0x4000);
        assert_eq!(PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES, 0x20);
        assert_eq!(caches.instruction_line_count(), 512);
        assert_eq!(PRIMARY_DATA_CACHE_SIZE_BYTES, 0x2000);
        assert_eq!(PRIMARY_DATA_CACHE_LINE_SIZE_BYTES, 0x10);
        assert_eq!(caches.data_line_count(), 512);
        assert!((0..512).all(|index| caches.instruction_line(index).unwrap().is_unavailable()));
        assert!((0..512).all(|index| caches.data_line(index).unwrap().is_unavailable()));
    }

    #[test]
    fn zero_index_store_tag_invalidates_only_the_selected_lines() {
        let mut caches = MachinePrimaryCaches::new();
        caches.apply_index_store_tag(provenance(
            MachinePrimaryCacheIndexStoreTagTarget::Instruction,
            7,
            0,
        ));
        caches.apply_index_store_tag(provenance(
            MachinePrimaryCacheIndexStoreTagTarget::Data,
            11,
            0,
        ));
        assert!(caches.instruction_line(7).unwrap().is_invalid());
        assert!(caches.data_line(11).unwrap().is_invalid());
        assert!(caches.instruction_line(6).unwrap().is_unavailable());
        assert!(caches.data_line(10).unwrap().is_unavailable());
    }

    #[test]
    fn instruction_fill_uses_direct_mapped_index_tag_and_big_endian_words() {
        let mut caches = MachinePrimaryCaches::new();
        let mut data = [0_u8; PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES];
        data[4..8].copy_from_slice(&0x3c0b_b000_u32.to_be_bytes());
        let plan =
            MachinePrimaryInstructionCacheFillPlan::new(CpuAddress::new(0x8000_0004), 0, data);
        caches.apply_instruction_fill(plan);
        assert_eq!(caches.lookup_instruction_word(4), Some(0x3c0b_b000));
        assert_eq!(caches.lookup_instruction_word(0x4004), None);
        assert_eq!(
            caches
                .instruction_line(0)
                .unwrap()
                .fill_provenance()
                .unwrap()
                .requested_cpu_address(),
            CpuAddress::new(0x8000_0004)
        );
    }
}
