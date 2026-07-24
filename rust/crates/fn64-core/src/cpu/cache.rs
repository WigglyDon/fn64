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
pub enum MachinePrimaryCacheHitInvalidateTarget {
    Instruction,
    Data,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePrimaryCacheIndexInvalidateProvenance {
    instruction_pc: CpuAddress,
    raw_instruction_word: u32,
    target: MachinePrimaryCacheIndexStoreTagTarget,
    base_gpr: u8,
    base_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
    line_index: u16,
    delay_slot_owner: Option<CpuAddress>,
}

impl MachinePrimaryCacheIndexInvalidateProvenance {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        raw_instruction_word: u32,
        target: MachinePrimaryCacheIndexStoreTagTarget,
        base_gpr: u8,
        base_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
        line_index: u16,
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
            physical_address,
            line_index,
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

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }

    pub const fn line_index(self) -> u16 {
        self.line_index
    }

    pub const fn delay_slot_owner(self) -> Option<CpuAddress> {
        self.delay_slot_owner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePrimaryCacheHitInvalidateProvenance {
    instruction_pc: CpuAddress,
    raw_instruction_word: u32,
    target: MachinePrimaryCacheHitInvalidateTarget,
    base_gpr: u8,
    base_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
    line_index: u16,
    delay_slot_owner: Option<CpuAddress>,
}

impl MachinePrimaryCacheHitInvalidateProvenance {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        raw_instruction_word: u32,
        target: MachinePrimaryCacheHitInvalidateTarget,
        base_gpr: u8,
        base_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
        line_index: u16,
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
            physical_address,
            line_index,
            delay_slot_owner,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn raw_instruction_word(self) -> u32 {
        self.raw_instruction_word
    }

    pub const fn target(self) -> MachinePrimaryCacheHitInvalidateTarget {
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

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }

    pub const fn line_index(self) -> u16 {
        self.line_index
    }

    pub const fn delay_slot_owner(self) -> Option<CpuAddress> {
        self.delay_slot_owner
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePrimaryDataCacheFillProvenance {
    requested_cpu_address: CpuAddress,
    physical_line_address: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePrimaryDataCacheStoreWidth {
    Byte,
    Halfword,
    Word,
    Doubleword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePrimaryDataCacheStoreProvenance {
    instruction_pc: CpuAddress,
    raw_instruction_word: u32,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
    width: MachinePrimaryDataCacheStoreWidth,
    delay_slot_owner: Option<CpuAddress>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePrimaryDataCacheHitWritebackProvenance {
    instruction_pc: CpuAddress,
    raw_instruction_word: u32,
    base_gpr: u8,
    base_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
    line_index: u16,
    delay_slot_owner: Option<CpuAddress>,
}

impl MachinePrimaryDataCacheHitWritebackProvenance {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        raw_instruction_word: u32,
        base_gpr: u8,
        base_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
        line_index: u16,
        delay_slot_owner: Option<CpuAddress>,
    ) -> Self {
        Self {
            instruction_pc,
            raw_instruction_word,
            base_gpr,
            base_lineage,
            effective_address,
            cpu_address,
            physical_address,
            line_index,
            delay_slot_owner,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn raw_instruction_word(self) -> u32 {
        self.raw_instruction_word
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

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }

    pub const fn line_index(self) -> u16 {
        self.line_index
    }

    pub const fn delay_slot_owner(self) -> Option<CpuAddress> {
        self.delay_slot_owner
    }
}

impl MachinePrimaryDataCacheStoreProvenance {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        raw_instruction_word: u32,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
        width: MachinePrimaryDataCacheStoreWidth,
        delay_slot_owner: Option<CpuAddress>,
    ) -> Self {
        Self {
            instruction_pc,
            raw_instruction_word,
            source_gpr,
            source_lineage,
            effective_address,
            cpu_address,
            physical_address,
            width,
            delay_slot_owner,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn raw_instruction_word(self) -> u32 {
        self.raw_instruction_word
    }

    pub const fn source_gpr(self) -> u8 {
        self.source_gpr
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        self.source_lineage
    }

    pub const fn effective_address(self) -> u64 {
        self.effective_address
    }

    pub const fn cpu_address(self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }

    pub const fn width(self) -> MachinePrimaryDataCacheStoreWidth {
        self.width
    }

    pub const fn delay_slot_owner(self) -> Option<CpuAddress> {
        self.delay_slot_owner
    }
}

impl MachinePrimaryDataCacheFillProvenance {
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
    IndexInvalid {
        provenance: MachinePrimaryCacheIndexInvalidateProvenance,
    },
    HitInvalid {
        provenance: MachinePrimaryCacheHitInvalidateProvenance,
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
        matches!(
            self,
            Self::Invalid { .. } | Self::IndexInvalid { .. } | Self::HitInvalid { .. }
        )
    }

    pub const fn is_valid(self) -> bool {
        matches!(self, Self::ValidDataUnavailable { .. } | Self::Valid { .. })
    }

    pub const fn physical_tag(self) -> Option<u32> {
        match self {
            Self::ValidDataUnavailable { physical_tag, .. } | Self::Valid { physical_tag, .. } => {
                Some(physical_tag)
            }
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. } => None,
        }
    }

    pub const fn operation_provenance(self) -> Option<MachinePrimaryCacheOperationProvenance> {
        match self {
            Self::Invalid { provenance } | Self::ValidDataUnavailable { provenance, .. } => {
                Some(provenance)
            }
            Self::Unavailable
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. }
            | Self::Valid { .. } => None,
        }
    }

    pub const fn index_invalidation_provenance(
        self,
    ) -> Option<MachinePrimaryCacheIndexInvalidateProvenance> {
        match self {
            Self::IndexInvalid { provenance } => Some(provenance),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidDataUnavailable { .. }
            | Self::Valid { .. } => None,
        }
    }

    pub const fn hit_invalidation_provenance(
        self,
    ) -> Option<MachinePrimaryCacheHitInvalidateProvenance> {
        match self {
            Self::HitInvalid { provenance } => Some(provenance),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::ValidDataUnavailable { .. }
            | Self::Valid { .. } => None,
        }
    }

    pub const fn fill_provenance(self) -> Option<MachinePrimaryInstructionCacheFillProvenance> {
        match self {
            Self::Valid { provenance, .. } => Some(provenance),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidDataUnavailable { .. } => None,
        }
    }

    pub const fn data(self) -> Option<[u8; PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES]> {
        match self {
            Self::Valid { data, .. } => Some(data),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidDataUnavailable { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePrimaryDataCacheLineState {
    Unavailable,
    Invalid {
        provenance: MachinePrimaryCacheOperationProvenance,
    },
    IndexInvalid {
        provenance: MachinePrimaryCacheIndexInvalidateProvenance,
    },
    HitInvalid {
        provenance: MachinePrimaryCacheHitInvalidateProvenance,
    },
    ValidCleanDataUnavailable {
        physical_tag: u32,
        provenance: MachinePrimaryCacheOperationProvenance,
    },
    ValidDirtyDataUnavailable {
        physical_tag: u32,
        provenance: MachinePrimaryCacheOperationProvenance,
    },
    ValidClean {
        physical_tag: u32,
        data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        provenance: MachinePrimaryDataCacheFillProvenance,
    },
    ValidDirty {
        physical_tag: u32,
        data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        fill_provenance: MachinePrimaryDataCacheFillProvenance,
        store_provenance: MachinePrimaryDataCacheStoreProvenance,
    },
}

impl MachinePrimaryDataCacheLineState {
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    pub const fn is_invalid(self) -> bool {
        matches!(
            self,
            Self::Invalid { .. } | Self::IndexInvalid { .. } | Self::HitInvalid { .. }
        )
    }

    pub const fn is_valid_clean(self) -> bool {
        matches!(
            self,
            Self::ValidCleanDataUnavailable { .. } | Self::ValidClean { .. }
        )
    }

    pub const fn is_valid_dirty(self) -> bool {
        matches!(
            self,
            Self::ValidDirtyDataUnavailable { .. } | Self::ValidDirty { .. }
        )
    }

    pub const fn physical_tag(self) -> Option<u32> {
        match self {
            Self::ValidCleanDataUnavailable { physical_tag, .. }
            | Self::ValidDirtyDataUnavailable { physical_tag, .. }
            | Self::ValidClean { physical_tag, .. }
            | Self::ValidDirty { physical_tag, .. } => Some(physical_tag),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. } => None,
        }
    }

    pub const fn operation_provenance(self) -> Option<MachinePrimaryCacheOperationProvenance> {
        match self {
            Self::Invalid { provenance }
            | Self::ValidCleanDataUnavailable { provenance, .. }
            | Self::ValidDirtyDataUnavailable { provenance, .. } => Some(provenance),
            Self::Unavailable
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidClean { .. }
            | Self::ValidDirty { .. } => None,
        }
    }

    pub const fn index_invalidation_provenance(
        self,
    ) -> Option<MachinePrimaryCacheIndexInvalidateProvenance> {
        match self {
            Self::IndexInvalid { provenance } => Some(provenance),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidCleanDataUnavailable { .. }
            | Self::ValidDirtyDataUnavailable { .. }
            | Self::ValidClean { .. }
            | Self::ValidDirty { .. } => None,
        }
    }

    pub const fn hit_invalidation_provenance(
        self,
    ) -> Option<MachinePrimaryCacheHitInvalidateProvenance> {
        match self {
            Self::HitInvalid { provenance } => Some(provenance),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::ValidCleanDataUnavailable { .. }
            | Self::ValidDirtyDataUnavailable { .. }
            | Self::ValidClean { .. }
            | Self::ValidDirty { .. } => None,
        }
    }

    pub const fn fill_provenance(self) -> Option<MachinePrimaryDataCacheFillProvenance> {
        match self {
            Self::ValidClean { provenance, .. } => Some(provenance),
            Self::ValidDirty {
                fill_provenance, ..
            } => Some(fill_provenance),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidCleanDataUnavailable { .. }
            | Self::ValidDirtyDataUnavailable { .. } => None,
        }
    }

    pub const fn data(self) -> Option<[u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES]> {
        match self {
            Self::ValidClean { data, .. } | Self::ValidDirty { data, .. } => Some(data),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidCleanDataUnavailable { .. }
            | Self::ValidDirtyDataUnavailable { .. } => None,
        }
    }

    pub const fn store_provenance(self) -> Option<MachinePrimaryDataCacheStoreProvenance> {
        match self {
            Self::ValidDirty {
                store_provenance, ..
            } => Some(store_provenance),
            Self::Unavailable
            | Self::Invalid { .. }
            | Self::IndexInvalid { .. }
            | Self::HitInvalid { .. }
            | Self::ValidCleanDataUnavailable { .. }
            | Self::ValidDirtyDataUnavailable { .. }
            | Self::ValidClean { .. } => None,
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
                    2 => MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable {
                        physical_tag,
                        provenance,
                    },
                    3 => MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable {
                        physical_tag,
                        provenance,
                    },
                    _ => unreachable!("unsupported primary D-cache tag state was preflighted"),
                };
            }
        }
    }

    pub(crate) fn apply_index_invalidate(&mut self, plan: MachinePrimaryCacheIndexInvalidatePlan) {
        match plan.provenance.target() {
            MachinePrimaryCacheIndexStoreTagTarget::Instruction => {
                self.instruction_lines[plan.line_index] =
                    MachinePrimaryInstructionCacheLineState::IndexInvalid {
                        provenance: plan.provenance,
                    };
            }
            MachinePrimaryCacheIndexStoreTagTarget::Data => {
                self.data_lines[plan.line_index] = MachinePrimaryDataCacheLineState::IndexInvalid {
                    provenance: plan.provenance,
                };
            }
        }
    }

    pub(crate) fn plan_data_index_writeback_invalidate(
        &self,
        provenance: MachinePrimaryCacheIndexInvalidateProvenance,
    ) -> Result<
        MachinePrimaryDataCacheIndexWritebackInvalidatePlan,
        MachinePrimaryDataCacheAccessError,
    > {
        debug_assert_eq!(
            provenance.target(),
            MachinePrimaryCacheIndexStoreTagTarget::Data
        );
        let line_index = provenance.line_index() as usize;
        let state = self.data_lines[line_index];
        let writeback = match state {
            MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag,
                data,
                store_provenance,
                ..
            } => Some(MachinePrimaryDataCacheWritebackPlan::new(
                line_index,
                physical_tag,
                data,
                store_provenance,
            )),
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. } => {
                return Err(MachinePrimaryDataCacheAccessError { line_index, state });
            }
            MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidClean { .. } => None,
        };
        Ok(MachinePrimaryDataCacheIndexWritebackInvalidatePlan {
            invalidate: MachinePrimaryCacheIndexInvalidatePlan::new(provenance),
            writeback,
        })
    }

    pub(crate) fn apply_data_index_writeback_invalidate(
        &mut self,
        plan: MachinePrimaryDataCacheIndexWritebackInvalidatePlan,
    ) {
        self.apply_index_invalidate(plan.invalidate);
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

    pub(crate) fn lookup_data_word(&self, physical_address: u32) -> Option<u32> {
        let line_index = primary_data_cache_line_index(physical_address);
        let expected_tag = primary_data_cache_physical_tag(physical_address);
        let (physical_tag, data) = match self.data_lines[line_index] {
            MachinePrimaryDataCacheLineState::ValidClean {
                physical_tag, data, ..
            }
            | MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag, data, ..
            } => (physical_tag, data),
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. } => return None,
        };
        if physical_tag != expected_tag {
            return None;
        }
        let word_offset = (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        Some(u32::from_be_bytes(
            data[word_offset..word_offset + 4]
                .try_into()
                .expect("aligned data word fits inside one cache line"),
        ))
    }

    pub(crate) fn lookup_data_doubleword(&self, physical_address: u32) -> Option<u64> {
        let line_index = primary_data_cache_line_index(physical_address);
        let expected_tag = primary_data_cache_physical_tag(physical_address);
        let (physical_tag, data) = match self.data_lines[line_index] {
            MachinePrimaryDataCacheLineState::ValidClean {
                physical_tag, data, ..
            }
            | MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag, data, ..
            } => (physical_tag, data),
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. } => return None,
        };
        if physical_tag != expected_tag {
            return None;
        }
        let doubleword_offset =
            (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        Some(u64::from_be_bytes(
            data[doubleword_offset..doubleword_offset + 8]
                .try_into()
                .expect("aligned data doubleword fits inside one cache line"),
        ))
    }

    pub(crate) fn lookup_data_halfword(&self, physical_address: u32) -> Option<u16> {
        let line_index = primary_data_cache_line_index(physical_address);
        let expected_tag = primary_data_cache_physical_tag(physical_address);
        let (physical_tag, data) = match self.data_lines[line_index] {
            MachinePrimaryDataCacheLineState::ValidClean {
                physical_tag, data, ..
            }
            | MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag, data, ..
            } => (physical_tag, data),
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. } => return None,
        };
        if physical_tag != expected_tag {
            return None;
        }
        let halfword_offset =
            (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        Some(u16::from_be_bytes([
            data[halfword_offset],
            data[halfword_offset + 1],
        ]))
    }

    pub(crate) fn lookup_data_byte(&self, physical_address: u32) -> Option<u8> {
        let line_index = primary_data_cache_line_index(physical_address);
        let expected_tag = primary_data_cache_physical_tag(physical_address);
        let (physical_tag, data) = match self.data_lines[line_index] {
            MachinePrimaryDataCacheLineState::ValidClean {
                physical_tag, data, ..
            }
            | MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag, data, ..
            } => (physical_tag, data),
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. } => return None,
        };
        if physical_tag != expected_tag {
            return None;
        }
        let byte_offset = (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        Some(data[byte_offset])
    }

    pub(crate) fn plan_data_replacement(
        &self,
        physical_address: u32,
    ) -> Result<Option<MachinePrimaryDataCacheWritebackPlan>, MachinePrimaryDataCacheAccessError>
    {
        let line_index = primary_data_cache_line_index(physical_address);
        let expected_tag = primary_data_cache_physical_tag(physical_address);
        let state = self.data_lines[line_index];
        match state {
            MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. } => {
                Err(MachinePrimaryDataCacheAccessError { line_index, state })
            }
            MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable {
                physical_tag, ..
            } if physical_tag == expected_tag => {
                Err(MachinePrimaryDataCacheAccessError { line_index, state })
            }
            MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag,
                data,
                store_provenance,
                ..
            } if physical_tag != expected_tag => {
                Ok(Some(MachinePrimaryDataCacheWritebackPlan::new(
                    line_index,
                    physical_tag,
                    data,
                    store_provenance,
                )))
            }
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidClean { .. }
            | MachinePrimaryDataCacheLineState::ValidDirty { .. } => Ok(None),
        }
    }

    pub(crate) fn plan_data_store_word(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_word: u32,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.plan_data_store(
            requested_cpu_address,
            physical_address,
            fill_data,
            MachinePrimaryDataCacheStoreValue::Word(stored_word),
            provenance,
        )
    }

    pub(crate) fn plan_data_store_byte(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_byte: u8,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.plan_data_store(
            requested_cpu_address,
            physical_address,
            fill_data,
            MachinePrimaryDataCacheStoreValue::Byte(stored_byte),
            provenance,
        )
    }

    pub(crate) fn plan_data_store_halfword(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_halfword: u16,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.plan_data_store(
            requested_cpu_address,
            physical_address,
            fill_data,
            MachinePrimaryDataCacheStoreValue::Halfword(stored_halfword),
            provenance,
        )
    }

    pub(crate) fn plan_data_store_doubleword(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_doubleword: u64,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.plan_data_store(
            requested_cpu_address,
            physical_address,
            fill_data,
            MachinePrimaryDataCacheStoreValue::Doubleword(stored_doubleword),
            provenance,
        )
    }

    fn plan_data_store(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        value: MachinePrimaryDataCacheStoreValue,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        let line_index = primary_data_cache_line_index(physical_address);
        let physical_tag = primary_data_cache_physical_tag(physical_address);
        let state = self.data_lines[line_index];
        let (mut data, fill_provenance, writeback, cache_hit) = match state {
            MachinePrimaryDataCacheLineState::ValidClean {
                physical_tag: current_tag,
                data,
                provenance,
            } if current_tag == physical_tag => (data, provenance, None, true),
            MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag: current_tag,
                data,
                fill_provenance,
                ..
            } if current_tag == physical_tag => (data, fill_provenance, None, true),
            MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable {
                physical_tag: current_tag,
                ..
            }
            | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable {
                physical_tag: current_tag,
                ..
            } if current_tag == physical_tag => {
                return Err(MachinePrimaryDataCacheAccessError { line_index, state });
            }
            MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. } => {
                return Err(MachinePrimaryDataCacheAccessError { line_index, state });
            }
            MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag: current_tag,
                data,
                store_provenance,
                ..
            } => (
                fill_data,
                MachinePrimaryDataCacheFillProvenance::new(
                    requested_cpu_address,
                    physical_address & !((PRIMARY_DATA_CACHE_LINE_SIZE_BYTES as u32) - 1),
                ),
                Some(MachinePrimaryDataCacheWritebackPlan::new(
                    line_index,
                    current_tag,
                    data,
                    store_provenance,
                )),
                false,
            ),
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidClean { .. } => (
                fill_data,
                MachinePrimaryDataCacheFillProvenance::new(
                    requested_cpu_address,
                    physical_address & !((PRIMARY_DATA_CACHE_LINE_SIZE_BYTES as u32) - 1),
                ),
                None,
                false,
            ),
        };

        let byte_offset = (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        match value {
            MachinePrimaryDataCacheStoreValue::Byte(value) => data[byte_offset] = value,
            MachinePrimaryDataCacheStoreValue::Halfword(value) => {
                data[byte_offset..byte_offset + 2].copy_from_slice(&value.to_be_bytes());
            }
            MachinePrimaryDataCacheStoreValue::Word(value) => {
                data[byte_offset..byte_offset + 4].copy_from_slice(&value.to_be_bytes());
            }
            MachinePrimaryDataCacheStoreValue::Doubleword(value) => {
                data[byte_offset..byte_offset + 8].copy_from_slice(&value.to_be_bytes());
            }
        }

        Ok(MachinePrimaryDataCacheStorePlan {
            line_index,
            physical_tag,
            data,
            fill_provenance,
            store_provenance: provenance,
            writeback,
            cache_hit,
        })
    }

    pub(crate) fn apply_data_fill(&mut self, plan: MachinePrimaryDataCacheFillPlan) {
        self.data_lines[plan.line_index] = MachinePrimaryDataCacheLineState::ValidClean {
            physical_tag: plan.physical_tag,
            data: plan.data,
            provenance: plan.provenance,
        };
    }

    pub(crate) fn apply_data_store(&mut self, plan: MachinePrimaryDataCacheStorePlan) {
        self.data_lines[plan.line_index] = MachinePrimaryDataCacheLineState::ValidDirty {
            physical_tag: plan.physical_tag,
            data: plan.data,
            fill_provenance: plan.fill_provenance,
            store_provenance: plan.store_provenance,
        };
    }

    pub(crate) fn plan_data_hit_writeback(
        &self,
        physical_address: u32,
        provenance: MachinePrimaryDataCacheHitWritebackProvenance,
    ) -> Result<MachinePrimaryDataCacheHitWritebackPlan, MachinePrimaryDataCacheAccessError> {
        let line_index = primary_data_cache_line_index(physical_address);
        let expected_tag = primary_data_cache_physical_tag(physical_address);
        let state = self.data_lines[line_index];
        let (cache_hit, writeback, cleaned_line) = match state {
            MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable {
                physical_tag, ..
            } if physical_tag == expected_tag => {
                return Err(MachinePrimaryDataCacheAccessError { line_index, state });
            }
            MachinePrimaryDataCacheLineState::ValidDirty {
                physical_tag,
                data,
                fill_provenance,
                store_provenance,
            } if physical_tag == expected_tag => (
                true,
                Some(MachinePrimaryDataCacheWritebackPlan::new(
                    line_index,
                    physical_tag,
                    data,
                    store_provenance,
                )),
                Some(MachinePrimaryDataCacheLineState::ValidClean {
                    physical_tag,
                    data,
                    provenance: fill_provenance,
                }),
            ),
            MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable {
                physical_tag, ..
            }
            | MachinePrimaryDataCacheLineState::ValidClean { physical_tag, .. }
                if physical_tag == expected_tag =>
            {
                (true, None, None)
            }
            MachinePrimaryDataCacheLineState::Unavailable
            | MachinePrimaryDataCacheLineState::Invalid { .. }
            | MachinePrimaryDataCacheLineState::IndexInvalid { .. }
            | MachinePrimaryDataCacheLineState::HitInvalid { .. }
            | MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable { .. }
            | MachinePrimaryDataCacheLineState::ValidClean { .. }
            | MachinePrimaryDataCacheLineState::ValidDirty { .. } => (false, None, None),
        };
        Ok(MachinePrimaryDataCacheHitWritebackPlan {
            line_index,
            cache_hit,
            writeback,
            cleaned_line,
            provenance,
        })
    }

    pub(crate) fn apply_data_hit_writeback(
        &mut self,
        plan: MachinePrimaryDataCacheHitWritebackPlan,
    ) {
        if let Some(cleaned_line) = plan.cleaned_line {
            self.data_lines[plan.line_index] = cleaned_line;
        }
    }

    pub(crate) fn plan_hit_invalidate(
        &self,
        physical_address: u32,
        provenance: MachinePrimaryCacheHitInvalidateProvenance,
    ) -> MachinePrimaryCacheHitInvalidatePlan {
        let target = provenance.target();
        let (line_index, cache_hit) = match target {
            MachinePrimaryCacheHitInvalidateTarget::Instruction => {
                let line_index = primary_instruction_cache_line_index(physical_address);
                let expected_tag = primary_instruction_cache_physical_tag(physical_address);
                let cache_hit = matches!(
                    self.instruction_lines[line_index],
                    MachinePrimaryInstructionCacheLineState::ValidDataUnavailable {
                        physical_tag,
                        ..
                    } | MachinePrimaryInstructionCacheLineState::Valid {
                        physical_tag,
                        ..
                    } if physical_tag == expected_tag
                );
                (line_index, cache_hit)
            }
            MachinePrimaryCacheHitInvalidateTarget::Data => {
                let line_index = primary_data_cache_line_index(physical_address);
                let expected_tag = primary_data_cache_physical_tag(physical_address);
                let cache_hit = matches!(
                    self.data_lines[line_index],
                    MachinePrimaryDataCacheLineState::ValidCleanDataUnavailable {
                        physical_tag,
                        ..
                    } | MachinePrimaryDataCacheLineState::ValidDirtyDataUnavailable {
                        physical_tag,
                        ..
                    } | MachinePrimaryDataCacheLineState::ValidClean {
                        physical_tag,
                        ..
                    } | MachinePrimaryDataCacheLineState::ValidDirty {
                        physical_tag,
                        ..
                    } if physical_tag == expected_tag
                );
                (line_index, cache_hit)
            }
        };
        MachinePrimaryCacheHitInvalidatePlan {
            target,
            line_index,
            cache_hit,
            provenance,
        }
    }

    pub(crate) fn apply_hit_invalidate(&mut self, plan: MachinePrimaryCacheHitInvalidatePlan) {
        if !plan.cache_hit {
            return;
        }
        match plan.target {
            MachinePrimaryCacheHitInvalidateTarget::Instruction => {
                self.instruction_lines[plan.line_index] =
                    MachinePrimaryInstructionCacheLineState::HitInvalid {
                        provenance: plan.provenance,
                    };
            }
            MachinePrimaryCacheHitInvalidateTarget::Data => {
                self.data_lines[plan.line_index] = MachinePrimaryDataCacheLineState::HitInvalid {
                    provenance: plan.provenance,
                };
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryInstructionCacheFillPlan {
    line_index: usize,
    physical_tag: u32,
    data: [u8; PRIMARY_INSTRUCTION_CACHE_LINE_SIZE_BYTES],
    provenance: MachinePrimaryInstructionCacheFillProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryDataCacheFillPlan {
    line_index: usize,
    physical_tag: u32,
    data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
    provenance: MachinePrimaryDataCacheFillProvenance,
    writeback: Option<MachinePrimaryDataCacheWritebackPlan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryDataCacheAccessError {
    line_index: usize,
    state: MachinePrimaryDataCacheLineState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryDataCacheWritebackPlan {
    line_index: usize,
    physical_line_address: u32,
    data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
    latest_store: MachinePrimaryDataCacheStoreProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryDataCacheHitWritebackPlan {
    line_index: usize,
    cache_hit: bool,
    writeback: Option<MachinePrimaryDataCacheWritebackPlan>,
    cleaned_line: Option<MachinePrimaryDataCacheLineState>,
    provenance: MachinePrimaryDataCacheHitWritebackProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryCacheHitInvalidatePlan {
    target: MachinePrimaryCacheHitInvalidateTarget,
    line_index: usize,
    cache_hit: bool,
    provenance: MachinePrimaryCacheHitInvalidateProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryCacheIndexInvalidatePlan {
    line_index: usize,
    provenance: MachinePrimaryCacheIndexInvalidateProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryDataCacheIndexWritebackInvalidatePlan {
    invalidate: MachinePrimaryCacheIndexInvalidatePlan,
    writeback: Option<MachinePrimaryDataCacheWritebackPlan>,
}

impl MachinePrimaryDataCacheIndexWritebackInvalidatePlan {
    pub(crate) const fn writeback(self) -> Option<MachinePrimaryDataCacheWritebackPlan> {
        self.writeback
    }
}

impl MachinePrimaryCacheIndexInvalidatePlan {
    pub(crate) const fn new(provenance: MachinePrimaryCacheIndexInvalidateProvenance) -> Self {
        Self {
            line_index: provenance.line_index() as usize,
            provenance,
        }
    }
}

impl MachinePrimaryCacheHitInvalidatePlan {
    pub(crate) const fn cache_hit(self) -> bool {
        self.cache_hit
    }

    pub(crate) const fn provenance(self) -> MachinePrimaryCacheHitInvalidateProvenance {
        self.provenance
    }
}

impl MachinePrimaryDataCacheHitWritebackPlan {
    pub(crate) const fn cache_hit(self) -> bool {
        self.cache_hit
    }

    pub(crate) const fn writeback(self) -> Option<MachinePrimaryDataCacheWritebackPlan> {
        self.writeback
    }

    pub(crate) const fn provenance(self) -> MachinePrimaryDataCacheHitWritebackProvenance {
        self.provenance
    }
}

impl MachinePrimaryDataCacheWritebackPlan {
    const fn new(
        line_index: usize,
        physical_tag: u32,
        data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        latest_store: MachinePrimaryDataCacheStoreProvenance,
    ) -> Self {
        Self {
            line_index,
            physical_line_address: physical_tag
                | ((line_index * PRIMARY_DATA_CACHE_LINE_SIZE_BYTES) as u32),
            data,
            latest_store,
        }
    }

    pub(crate) const fn line_index(self) -> usize {
        self.line_index
    }

    pub(crate) const fn physical_line_address(self) -> u32 {
        self.physical_line_address
    }

    pub(crate) const fn data(self) -> [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES] {
        self.data
    }

    pub(crate) const fn latest_store(self) -> MachinePrimaryDataCacheStoreProvenance {
        self.latest_store
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MachinePrimaryDataCacheStoreValue {
    Byte(u8),
    Halfword(u16),
    Word(u32),
    Doubleword(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachinePrimaryDataCacheStorePlan {
    line_index: usize,
    physical_tag: u32,
    data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
    fill_provenance: MachinePrimaryDataCacheFillProvenance,
    store_provenance: MachinePrimaryDataCacheStoreProvenance,
    writeback: Option<MachinePrimaryDataCacheWritebackPlan>,
    cache_hit: bool,
}

impl MachinePrimaryDataCacheStorePlan {
    pub(crate) const fn writeback(self) -> Option<MachinePrimaryDataCacheWritebackPlan> {
        self.writeback
    }

    pub(crate) const fn cache_hit(self) -> bool {
        self.cache_hit
    }
}

impl MachinePrimaryDataCacheFillPlan {
    pub(crate) const fn new(
        requested_cpu_address: CpuAddress,
        physical_line_address: u32,
        data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
    ) -> Self {
        Self {
            line_index: primary_data_cache_line_index(physical_line_address),
            physical_tag: primary_data_cache_physical_tag(physical_line_address),
            data,
            provenance: MachinePrimaryDataCacheFillProvenance::new(
                requested_cpu_address,
                physical_line_address,
            ),
            writeback: None,
        }
    }

    pub(crate) const fn with_writeback(
        mut self,
        writeback: Option<MachinePrimaryDataCacheWritebackPlan>,
    ) -> Self {
        self.writeback = writeback;
        self
    }

    pub(crate) const fn writeback(self) -> Option<MachinePrimaryDataCacheWritebackPlan> {
        self.writeback
    }

    pub(crate) const fn requested_word(self, physical_address: u32) -> u32 {
        let word_offset = (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        u32::from_be_bytes([
            self.data[word_offset],
            self.data[word_offset + 1],
            self.data[word_offset + 2],
            self.data[word_offset + 3],
        ])
    }

    pub(crate) const fn requested_doubleword(self, physical_address: u32) -> u64 {
        let doubleword_offset =
            (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        u64::from_be_bytes([
            self.data[doubleword_offset],
            self.data[doubleword_offset + 1],
            self.data[doubleword_offset + 2],
            self.data[doubleword_offset + 3],
            self.data[doubleword_offset + 4],
            self.data[doubleword_offset + 5],
            self.data[doubleword_offset + 6],
            self.data[doubleword_offset + 7],
        ])
    }

    pub(crate) const fn requested_byte(self, physical_address: u32) -> u8 {
        let byte_offset = (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        self.data[byte_offset]
    }

    pub(crate) const fn requested_halfword(self, physical_address: u32) -> u16 {
        let halfword_offset =
            (physical_address as usize) & (PRIMARY_DATA_CACHE_LINE_SIZE_BYTES - 1);
        u16::from_be_bytes([self.data[halfword_offset], self.data[halfword_offset + 1]])
    }
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

const fn primary_data_cache_physical_tag(physical_address: u32) -> u32 {
    physical_address & !((PRIMARY_DATA_CACHE_SIZE_BYTES as u32) - 1)
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

    pub(crate) fn lookup_primary_data_cache_word(&self, physical_address: u32) -> Option<u32> {
        self.primary_caches.lookup_data_word(physical_address)
    }

    pub(crate) fn lookup_primary_data_cache_doubleword(
        &self,
        physical_address: u32,
    ) -> Option<u64> {
        self.primary_caches.lookup_data_doubleword(physical_address)
    }

    pub(crate) fn lookup_primary_data_cache_halfword(&self, physical_address: u32) -> Option<u16> {
        self.primary_caches.lookup_data_halfword(physical_address)
    }

    pub(crate) fn lookup_primary_data_cache_byte(&self, physical_address: u32) -> Option<u8> {
        self.primary_caches.lookup_data_byte(physical_address)
    }

    pub(crate) fn plan_primary_data_cache_replacement(
        &self,
        physical_address: u32,
    ) -> Result<Option<MachinePrimaryDataCacheWritebackPlan>, MachinePrimaryDataCacheAccessError>
    {
        self.primary_caches.plan_data_replacement(physical_address)
    }

    pub(crate) fn plan_primary_data_cache_store_word(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_word: u32,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.primary_caches.plan_data_store_word(
            requested_cpu_address,
            physical_address,
            fill_data,
            stored_word,
            provenance,
        )
    }

    pub(crate) fn plan_primary_data_cache_store_byte(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_byte: u8,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.primary_caches.plan_data_store_byte(
            requested_cpu_address,
            physical_address,
            fill_data,
            stored_byte,
            provenance,
        )
    }

    pub(crate) fn plan_primary_data_cache_store_halfword(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_halfword: u16,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.primary_caches.plan_data_store_halfword(
            requested_cpu_address,
            physical_address,
            fill_data,
            stored_halfword,
            provenance,
        )
    }

    pub(crate) fn plan_primary_data_cache_store_doubleword(
        &self,
        requested_cpu_address: CpuAddress,
        physical_address: u32,
        fill_data: [u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
        stored_doubleword: u64,
        provenance: MachinePrimaryDataCacheStoreProvenance,
    ) -> Result<MachinePrimaryDataCacheStorePlan, MachinePrimaryDataCacheAccessError> {
        self.primary_caches.plan_data_store_doubleword(
            requested_cpu_address,
            physical_address,
            fill_data,
            stored_doubleword,
            provenance,
        )
    }

    pub(crate) fn apply_primary_data_cache_fill(&mut self, plan: MachinePrimaryDataCacheFillPlan) {
        self.primary_caches.apply_data_fill(plan);
    }

    pub(crate) fn apply_primary_data_cache_store(
        &mut self,
        plan: MachinePrimaryDataCacheStorePlan,
    ) {
        self.primary_caches.apply_data_store(plan);
    }

    pub(crate) fn plan_primary_data_cache_hit_writeback(
        &self,
        physical_address: u32,
        provenance: MachinePrimaryDataCacheHitWritebackProvenance,
    ) -> Result<MachinePrimaryDataCacheHitWritebackPlan, MachinePrimaryDataCacheAccessError> {
        self.primary_caches
            .plan_data_hit_writeback(physical_address, provenance)
    }

    pub(crate) fn apply_primary_data_cache_hit_writeback(
        &mut self,
        plan: MachinePrimaryDataCacheHitWritebackPlan,
    ) {
        self.primary_caches.apply_data_hit_writeback(plan);
    }

    pub(crate) fn plan_primary_cache_hit_invalidate(
        &self,
        physical_address: u32,
        provenance: MachinePrimaryCacheHitInvalidateProvenance,
    ) -> MachinePrimaryCacheHitInvalidatePlan {
        self.primary_caches
            .plan_hit_invalidate(physical_address, provenance)
    }

    pub(crate) fn apply_primary_cache_hit_invalidate(
        &mut self,
        plan: MachinePrimaryCacheHitInvalidatePlan,
    ) {
        self.primary_caches.apply_hit_invalidate(plan);
    }

    pub(crate) fn apply_primary_cache_index_invalidate(
        &mut self,
        plan: MachinePrimaryCacheIndexInvalidatePlan,
    ) {
        self.primary_caches.apply_index_invalidate(plan);
    }

    pub(crate) fn plan_primary_data_cache_index_writeback_invalidate(
        &self,
        provenance: MachinePrimaryCacheIndexInvalidateProvenance,
    ) -> Result<
        MachinePrimaryDataCacheIndexWritebackInvalidatePlan,
        MachinePrimaryDataCacheAccessError,
    > {
        self.primary_caches
            .plan_data_index_writeback_invalidate(provenance)
    }

    pub(crate) fn apply_primary_data_cache_index_writeback_invalidate(
        &mut self,
        plan: MachinePrimaryDataCacheIndexWritebackInvalidatePlan,
    ) {
        self.primary_caches
            .apply_data_index_writeback_invalidate(plan);
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

    fn store_provenance(
        instruction_pc: u32,
        physical_address: u32,
        width: MachinePrimaryDataCacheStoreWidth,
    ) -> MachinePrimaryDataCacheStoreProvenance {
        MachinePrimaryDataCacheStoreProvenance::new(
            CpuAddress::new(instruction_pc),
            match width {
                MachinePrimaryDataCacheStoreWidth::Byte => 0xa10a_0001,
                MachinePrimaryDataCacheStoreWidth::Halfword => 0xa50a_0000,
                MachinePrimaryDataCacheStoreWidth::Word => 0xad0a_0000,
                MachinePrimaryDataCacheStoreWidth::Doubleword => 0xfd0a_0000,
            },
            10,
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(instruction_pc.wrapping_sub(4)),
                identity: CpuInstructionIdentity::Ori,
                source_gpr_a: Some(10),
                source_gpr_b: None,
            },
            u64::from(0x8000_0000 | physical_address),
            CpuAddress::new(0x8000_0000 | physical_address),
            physical_address,
            width,
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

    #[test]
    fn data_fill_uses_exact_clean_line_data_hits_and_direct_mapped_replacement() {
        let mut caches = MachinePrimaryCaches::new();
        let mut first = [0_u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES];
        first[4..8].copy_from_slice(&0x1122_3344_u32.to_be_bytes());
        let first_plan =
            MachinePrimaryDataCacheFillPlan::new(CpuAddress::new(0x8000_1004), 0x0000_1000, first);
        assert_eq!(first_plan.requested_word(0x0000_1004), 0x1122_3344);
        caches.apply_data_fill(first_plan);
        assert_eq!(caches.lookup_data_word(0x0000_1004), Some(0x1122_3344));
        assert_eq!(caches.lookup_data_word(0x0000_3004), None);

        let line = caches
            .data_line(primary_data_cache_line_index(0x0000_1000))
            .unwrap();
        assert!(line.is_valid_clean());
        assert!(!line.is_valid_dirty());
        assert_eq!(line.data(), Some(first));
        assert_eq!(
            line.fill_provenance(),
            Some(MachinePrimaryDataCacheFillProvenance::new(
                CpuAddress::new(0x8000_1004),
                0x0000_1000,
            ))
        );

        let mut replacement = [0_u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES];
        replacement[4..8].copy_from_slice(&0xaabb_ccdd_u32.to_be_bytes());
        caches.apply_data_fill(MachinePrimaryDataCacheFillPlan::new(
            CpuAddress::new(0x8000_3004),
            0x0000_3000,
            replacement,
        ));
        assert_eq!(caches.lookup_data_word(0x0000_1004), None);
        assert_eq!(caches.lookup_data_word(0x0000_3004), Some(0xaabb_ccdd));
        assert!(!caches
            .data_line(primary_data_cache_line_index(0x0000_3000))
            .unwrap()
            .is_valid_dirty());
    }

    #[test]
    fn data_store_hit_and_byte_patch_preserve_line_bytes_and_mark_exact_dirty_truth() {
        let mut caches = MachinePrimaryCaches::new();
        let mut initial = [0_u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES];
        initial[0..4].copy_from_slice(&0xaabb_ccdd_u32.to_be_bytes());
        caches.apply_data_fill(MachinePrimaryDataCacheFillPlan::new(
            CpuAddress::new(0x8010_0000),
            0x0010_0000,
            initial,
        ));

        let word_plan = caches
            .plan_data_store_word(
                CpuAddress::new(0x8010_0000),
                0x0010_0000,
                initial,
                0x1122_3344,
                store_provenance(
                    0x8000_1020,
                    0x0010_0000,
                    MachinePrimaryDataCacheStoreWidth::Word,
                ),
            )
            .unwrap();
        assert!(word_plan.cache_hit());
        assert_eq!(word_plan.writeback(), None);
        caches.apply_data_store(word_plan);
        assert_eq!(caches.lookup_data_word(0x0010_0000), Some(0x1122_3344));
        assert!(caches.data_line(0).unwrap().is_valid_dirty());

        let byte_plan = caches
            .plan_data_store_byte(
                CpuAddress::new(0x8010_0001),
                0x0010_0001,
                initial,
                0xaa,
                store_provenance(
                    0x8000_1024,
                    0x0010_0001,
                    MachinePrimaryDataCacheStoreWidth::Byte,
                ),
            )
            .unwrap();
        assert!(byte_plan.cache_hit());
        assert_eq!(byte_plan.writeback(), None);
        caches.apply_data_store(byte_plan);
        assert_eq!(caches.lookup_data_word(0x0010_0000), Some(0x11aa_3344));
        assert_eq!(caches.lookup_data_byte(0x0010_0001), Some(0xaa));
        let line = caches.data_line(0).unwrap();
        assert!(line.is_valid_dirty());
        assert_eq!(
            line.store_provenance().unwrap().width(),
            MachinePrimaryDataCacheStoreWidth::Byte
        );
        assert_eq!(
            line.store_provenance().unwrap().instruction_pc(),
            CpuAddress::new(0x8000_1024)
        );
    }

    #[test]
    fn conflicting_store_and_load_plans_expose_exact_atomic_dirty_writebacks() {
        let mut caches = MachinePrimaryCaches::new();
        let zero = [0_u8; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES];
        let first = caches
            .plan_data_store_word(
                CpuAddress::new(0x8010_0000),
                0x0010_0000,
                zero,
                0x1122_3344,
                store_provenance(
                    0x8000_1020,
                    0x0010_0000,
                    MachinePrimaryDataCacheStoreWidth::Word,
                ),
            )
            .unwrap();
        assert!(!first.cache_hit());
        caches.apply_data_store(first);

        let second = caches
            .plan_data_store_word(
                CpuAddress::new(0x8010_2000),
                0x0010_2000,
                zero,
                0x5566_7788,
                store_provenance(
                    0x8000_1024,
                    0x0010_2000,
                    MachinePrimaryDataCacheStoreWidth::Word,
                ),
            )
            .unwrap();
        assert!(!second.cache_hit());
        let first_writeback = second.writeback().unwrap();
        assert_eq!(first_writeback.line_index(), 0);
        assert_eq!(first_writeback.physical_line_address(), 0x0010_0000);
        assert_eq!(
            u32::from_be_bytes(first_writeback.data()[..4].try_into().unwrap()),
            0x1122_3344
        );
        caches.apply_data_store(second);

        let second_writeback = caches.plan_data_replacement(0x0010_0000).unwrap().unwrap();
        assert_eq!(second_writeback.physical_line_address(), 0x0010_2000);
        assert_eq!(
            u32::from_be_bytes(second_writeback.data()[..4].try_into().unwrap()),
            0x5566_7788
        );
        assert_eq!(
            second_writeback.latest_store().instruction_pc(),
            CpuAddress::new(0x8000_1024)
        );
    }

    #[test]
    fn data_unavailable_dirty_truth_rejects_replacement_without_mutation() {
        let mut caches = MachinePrimaryCaches::new();
        caches.apply_index_store_tag(provenance(
            MachinePrimaryCacheIndexStoreTagTarget::Data,
            0,
            3 << PRIMARY_CACHE_TAG_LO_STATE_SHIFT,
        ));
        let before = caches.clone();

        assert!(caches.plan_data_replacement(0x0010_0000).is_err());
        assert!(caches
            .plan_data_store_word(
                CpuAddress::new(0x8010_0000),
                0x0010_0000,
                [0; PRIMARY_DATA_CACHE_LINE_SIZE_BYTES],
                0x1122_3344,
                store_provenance(
                    0x8000_1020,
                    0x0010_0000,
                    MachinePrimaryDataCacheStoreWidth::Word,
                ),
            )
            .is_err());
        assert_eq!(caches, before);
    }
}
