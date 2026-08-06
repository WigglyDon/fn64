use core::fmt;
use std::sync::Arc;

use crate::dpc::MachineDpcCounterIdentity;
use crate::mi::{
    MachineMiInterruptPendingSource, MachineMiInterruptSource, MachineMiInterruptState,
    MachineMiRspBreakInterruptSource,
};
use crate::sp::{MachineSpDramAddressSource, MachineSpSemaphoreSource, MachineSpStatusState};
use crate::sp_dmem::{
    MachineSpDmemByteKnowledge, MachineSpDmemByteKnowledgeDescriptor,
    MachineSpDmemByteKnowledgeSource, SpDmemOffset,
};
use crate::sp_imem::SpImemByteProvenance;

pub const RSP_SCALAR_REGISTER_COUNT: usize = 32;
pub const RSP_VECTOR_REGISTER_COUNT: usize = 32;
pub const RSP_VECTOR_REGISTER_BYTE_COUNT: usize = 16;
pub const RSP_LOCAL_ADDRESS_MASK: u16 = 0x0fff;
pub const RSP_INSTRUCTION_ALIGNMENT_MASK: u16 = 0x0003;
pub const RSP_COP0_OPCODE: u8 = 0x10;
pub const RSP_COP0_MFC0_TRANSFER_SELECTOR: u8 = 0;
pub const RSP_COP0_MTC0_TRANSFER_SELECTOR: u8 = 4;
pub const RSP_COP0_SP_MEMORY_ADDRESS_INDEX: u8 = 0;
pub const RSP_COP0_SP_DRAM_ADDRESS_INDEX: u8 = 1;
pub const RSP_COP0_SP_READ_LENGTH_INDEX: u8 = 2;
pub const RSP_COP0_SP_WRITE_LENGTH_INDEX: u8 = 3;
pub const RSP_COP0_SP_STATUS_INDEX: u8 = 4;
pub const RSP_COP0_SP_DMA_FULL_INDEX: u8 = 5;
pub const RSP_COP0_SP_DMA_BUSY_INDEX: u8 = 6;
pub const RSP_COP0_SP_SEMAPHORE_INDEX: u8 = 7;
pub const RSP_COP0_DPC_STATUS_INDEX: u8 = 11;
pub const RSP_SCALAR_BREAK_WORD: u32 = 0x0000_000d;
pub const RSP_SCALAR_BREAK_FUNCTION: u8 = 0x0d;
pub const RSP_SCALAR_BREAK_CODE_MASK: u32 = 0x000f_ffff;
pub const RSP_SCALAR_REGIMM_OPCODE: u8 = 0x01;
pub const RSP_SCALAR_J_OPCODE: u8 = 0x02;
pub const RSP_SCALAR_BLTZ_SELECTOR: u8 = 0;
pub const RSP_SCALAR_BGEZ_SELECTOR: u8 = 1;
pub const RSP_SCALAR_BGEZAL_SELECTOR: u8 = 0x11;
pub const RSP_SCALAR_BNE_OPCODE: u8 = 0x05;
pub const RSP_SCALAR_ADDI_OPCODE: u8 = 0x08;
pub const RSP_SCALAR_ORI_OPCODE: u8 = 0x0d;
pub const RSP_SCALAR_XORI_OPCODE: u8 = 0x0e;
pub const RSP_SCALAR_LUI_OPCODE: u8 = 0x0f;
pub const RSP_SCALAR_LW_OPCODE: u8 = 0x23;
pub const RSP_SCALAR_LW_BYTE_COUNT: usize = 4;
pub const RSP_VECTOR_LOAD_OPCODE: u8 = 0x32;
pub const RSP_VECTOR_LQV_SUBOPCODE: u8 = 4;
pub const RSP_VECTOR_COMPUTE_OPCODE: u8 = 0x12;
pub const RSP_VECTOR_VSUB_FUNCTION: u8 = 0x11;
pub const RSP_VECTOR_VADDC_FUNCTION: u8 = 0x14;
pub(crate) const RSP_NTSC_X105_POST_BOOT_GPR_11_INDEX: usize = 11;
pub(crate) const RSP_NTSC_X105_POST_BOOT_GPR_11_VALUE: u32 = 0;
pub const RSP_VECTOR_LANE_COUNT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspUnavailableSource {
    ConstructionOrReset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspControlRegister {
    SpMemoryAddress,
    SpDramAddress,
    SpReadLength,
    SpWriteLength,
    SpStatus,
    SpDmaFull,
    SpDmaBusy,
    SpSemaphore,
    DpcStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspMfc0ControlSource {
    SpDramAddress {
        value: u32,
        source: MachineSpDramAddressSource,
    },
    SpDmaBusy {
        busy: bool,
    },
    SpDmaFull {
        full: bool,
    },
    SpSemaphore {
        old_set: bool,
        source: MachineSpSemaphoreSource,
    },
}

impl MachineRspMfc0ControlSource {
    pub const fn register(self) -> MachineRspControlRegister {
        match self {
            Self::SpDramAddress { .. } => MachineRspControlRegister::SpDramAddress,
            Self::SpDmaBusy { .. } => MachineRspControlRegister::SpDmaBusy,
            Self::SpDmaFull { .. } => MachineRspControlRegister::SpDmaFull,
            Self::SpSemaphore { .. } => MachineRspControlRegister::SpSemaphore,
        }
    }

    pub const fn result_value(self) -> u32 {
        match self {
            Self::SpDramAddress { value, .. } => value,
            Self::SpDmaBusy { busy } => busy as u32,
            Self::SpDmaFull { full } => full as u32,
            Self::SpSemaphore { old_set, .. } => old_set as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRspMfc0ResultSource {
    instruction_pc: u16,
    control_source: MachineRspMfc0ControlSource,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspMfc0ResultSource {
    pub const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub const fn control_register(self) -> MachineRspControlRegister {
        self.control_source.register()
    }

    pub const fn control_source(self) -> MachineRspMfc0ControlSource {
        self.control_source
    }

    pub fn instruction_source(self) -> MachineRspInstructionSource {
        classify_instruction_source(self.byte_provenance)
    }

    #[cfg(test)]
    pub(crate) const fn byte_provenance(self) -> [SpImemByteProvenance; 4] {
        self.byte_provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspScalarRegisterSource {
    ArchitecturalZero,
    CleanRoomHleNtscX105PinnedPostBoot,
    Mfc0(MachineRspMfc0ResultSource),
    Lui(Box<MachineRspLuiSource>),
    Addi(Box<MachineRspAddiSource>),
    Lw(Box<MachineRspScalarLwSource>),
    Ori(Box<MachineRspOriSource>),
    Xori(Box<MachineRspXoriSource>),
    Sll(Box<MachineRspSllSource>),
    Bgezal(Box<MachineRspBgezalSource>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspBgezalSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    signed_offset: i16,
    link_value: u32,
}

impl MachineRspBgezalSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub const fn signed_offset(&self) -> i16 {
        self.signed_offset
    }

    pub const fn link_value(&self) -> u32 {
        self.link_value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspSllSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    shift_amount: u8,
}

impl MachineRspSllSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub const fn shift_amount(&self) -> u8 {
        self.shift_amount
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspLuiSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    immediate: u16,
}

impl MachineRspLuiSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn immediate(&self) -> u16 {
        self.immediate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspAddiSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    signed_immediate: i16,
}

impl MachineRspAddiSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub const fn signed_immediate(&self) -> i16 {
        self.signed_immediate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspXoriSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    immediate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspOriSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    immediate: u16,
}

impl MachineRspOriSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub const fn immediate(&self) -> u16 {
        self.immediate
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }
}

impl MachineRspXoriSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub const fn immediate(&self) -> u16 {
        self.immediate
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspMtc0Source {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    control_register: MachineRspControlRegister,
}

impl MachineRspMtc0Source {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub const fn control_register(&self) -> MachineRspControlRegister {
        self.control_register
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspScalarRegisterState {
    Available {
        value: u32,
        source: MachineRspScalarRegisterSource,
    },
    Unavailable {
        source: MachineRspUnavailableSource,
    },
}

impl MachineRspScalarRegisterState {
    pub const fn value(&self) -> Option<u32> {
        match self {
            Self::Available { value, .. } => Some(*value),
            Self::Unavailable { .. } => None,
        }
    }

    pub fn source(&self) -> Option<MachineRspScalarRegisterSource> {
        match self {
            Self::Available { source, .. } => Some(source.clone()),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available { .. })
    }

    pub const fn unavailable_source(&self) -> Option<MachineRspUnavailableSource> {
        match self {
            Self::Unavailable { source } => Some(*source),
            Self::Available { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspScalarLwSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; RSP_SCALAR_LW_BYTE_COUNT],
    base_gpr: u8,
    base_value: u32,
    base_source: MachineRspScalarRegisterSource,
    signed_offset: i16,
    local_dmem_address: u16,
    dmem_knowledge: [MachineSpDmemByteKnowledgeDescriptor; RSP_SCALAR_LW_BYTE_COUNT],
}

impl MachineRspScalarLwSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn base_gpr(&self) -> u8 {
        self.base_gpr
    }

    pub const fn base_value(&self) -> u32 {
        self.base_value
    }

    pub fn base_source(&self) -> MachineRspScalarRegisterSource {
        self.base_source.clone()
    }

    pub const fn signed_offset(&self) -> i16 {
        self.signed_offset
    }

    pub const fn local_dmem_address(&self) -> u16 {
        self.local_dmem_address
    }

    pub const fn dmem_knowledge(
        &self,
    ) -> [MachineSpDmemByteKnowledgeDescriptor; RSP_SCALAR_LW_BYTE_COUNT] {
        self.dmem_knowledge
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(
        &self,
    ) -> [SpImemByteProvenance; RSP_SCALAR_LW_BYTE_COUNT] {
        self.instruction_provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspLqvSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    base_gpr: u8,
    base_value: u32,
    base_source: MachineRspScalarRegisterSource,
    element: u8,
    signed_offset: i8,
    local_dmem_address: u16,
    dmem_knowledge: [MachineSpDmemByteKnowledgeDescriptor; RSP_VECTOR_REGISTER_BYTE_COUNT],
}

impl MachineRspLqvSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn base_gpr(&self) -> u8 {
        self.base_gpr
    }

    pub const fn base_value(&self) -> u32 {
        self.base_value
    }

    pub fn base_source(&self) -> MachineRspScalarRegisterSource {
        self.base_source.clone()
    }

    pub const fn element(&self) -> u8 {
        self.element
    }

    pub const fn signed_offset(&self) -> i8 {
        self.signed_offset
    }

    pub const fn local_dmem_address(&self) -> u16 {
        self.local_dmem_address
    }

    pub const fn dmem_knowledge(
        &self,
    ) -> [MachineSpDmemByteKnowledgeDescriptor; RSP_VECTOR_REGISTER_BYTE_COUNT] {
        self.dmem_knowledge
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspVectorRegisterSource {
    Lqv(Box<MachineRspLqvSource>),
    Vsub(Arc<MachineRspVectorArithmeticSource>),
    Vaddc(Arc<MachineRspVectorArithmeticSource>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspVectorUnavailableSource {
    ConstructionOrReset,
    Lqv(Box<MachineRspLqvSource>),
    Vsub(Arc<MachineRspVectorArithmeticSource>),
    Vaddc(Arc<MachineRspVectorArithmeticSource>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspVectorRegisterState {
    Available {
        bytes: [u8; RSP_VECTOR_REGISTER_BYTE_COUNT],
        source: MachineRspVectorRegisterSource,
    },
    Unavailable {
        source: MachineRspVectorUnavailableSource,
    },
}

impl MachineRspVectorRegisterState {
    pub const fn bytes(&self) -> Option<&[u8; RSP_VECTOR_REGISTER_BYTE_COUNT]> {
        match self {
            Self::Available { bytes, .. } => Some(bytes),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn available_source(&self) -> Option<&MachineRspVectorRegisterSource> {
        match self {
            Self::Available { source, .. } => Some(source),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn unavailable_source(&self) -> Option<&MachineRspVectorUnavailableSource> {
        match self {
            Self::Available { .. } => None,
            Self::Unavailable { source } => Some(source),
        }
    }

    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspVectorUnitState {
    registers: [MachineRspVectorRegisterState; RSP_VECTOR_REGISTER_COUNT],
}

impl MachineRspVectorUnitState {
    pub const fn register_count(&self) -> usize {
        self.registers.len()
    }

    pub fn register(&self, index: usize) -> Option<&MachineRspVectorRegisterState> {
        self.registers.get(index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspVectorArithmeticSource {
    instruction_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    identity: MachineRspInstructionIdentity,
    destination_vector: u8,
    source_vector_a: u8,
    source_a: MachineRspVectorRegisterState,
    source_vector_b: u8,
    source_b: Option<MachineRspVectorRegisterState>,
    element: u8,
    vsub_borrow_input: Option<MachineRspVcoHalfState>,
    result_available: bool,
}

impl MachineRspVectorArithmeticSource {
    pub const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn identity(&self) -> MachineRspInstructionIdentity {
        self.identity
    }

    pub const fn destination_vector(&self) -> u8 {
        self.destination_vector
    }

    pub const fn source_vector_a(&self) -> u8 {
        self.source_vector_a
    }

    pub const fn source_a(&self) -> &MachineRspVectorRegisterState {
        &self.source_a
    }

    pub const fn source_vector_b(&self) -> u8 {
        self.source_vector_b
    }

    pub const fn source_b(&self) -> Option<&MachineRspVectorRegisterState> {
        self.source_b.as_ref()
    }

    pub const fn sources_alias(&self) -> bool {
        self.source_b.is_none()
    }

    pub const fn element(&self) -> u8 {
        self.element
    }

    pub const fn vsub_borrow_input(&self) -> Option<&MachineRspVcoHalfState> {
        self.vsub_borrow_input.as_ref()
    }

    pub const fn result_available(&self) -> bool {
        self.result_available
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspAccumulatorSliceSource {
    Vsub(Arc<MachineRspVectorArithmeticSource>),
    Vaddc(Arc<MachineRspVectorArithmeticSource>),
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspAccumulatorSliceUnavailableSource {
    ConstructionOrReset,
    Vsub(Arc<MachineRspVectorArithmeticSource>),
    Vaddc(Arc<MachineRspVectorArithmeticSource>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspAccumulatorSliceState {
    Available {
        value: u16,
        source: MachineRspAccumulatorSliceSource,
    },
    Unavailable {
        source: MachineRspAccumulatorSliceUnavailableSource,
    },
}

impl MachineRspAccumulatorSliceState {
    pub const fn value(&self) -> Option<u16> {
        match self {
            Self::Available { value, .. } => Some(*value),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available { .. })
    }

    pub const fn available_source(&self) -> Option<&MachineRspAccumulatorSliceSource> {
        match self {
            Self::Available { source, .. } => Some(source),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn unavailable_source(&self) -> Option<&MachineRspAccumulatorSliceUnavailableSource> {
        match self {
            Self::Available { .. } => None,
            Self::Unavailable { source } => Some(source),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspAccumulatorLaneState {
    high: MachineRspAccumulatorSliceState,
    middle: MachineRspAccumulatorSliceState,
    low: MachineRspAccumulatorSliceState,
}

impl MachineRspAccumulatorLaneState {
    pub const fn high(&self) -> &MachineRspAccumulatorSliceState {
        &self.high
    }

    pub const fn middle(&self) -> &MachineRspAccumulatorSliceState {
        &self.middle
    }

    pub const fn low(&self) -> &MachineRspAccumulatorSliceState {
        &self.low
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspAccumulatorState {
    lanes: [MachineRspAccumulatorLaneState; RSP_VECTOR_LANE_COUNT],
}

impl MachineRspAccumulatorState {
    pub const fn lane_count(&self) -> usize {
        self.lanes.len()
    }

    pub fn lane(&self, index: usize) -> Option<&MachineRspAccumulatorLaneState> {
        self.lanes.get(index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspVcoHalfSource {
    VsubClear(Arc<MachineRspVectorArithmeticSource>),
    VaddcCarry(Arc<MachineRspVectorArithmeticSource>),
    VaddcNotEqualClear(Arc<MachineRspVectorArithmeticSource>),
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspVcoHalfUnavailableSource {
    ConstructionOrReset,
    VaddcCarry(Arc<MachineRspVectorArithmeticSource>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspVcoHalfState {
    Available {
        value: u8,
        source: MachineRspVcoHalfSource,
    },
    Unavailable {
        source: MachineRspVcoHalfUnavailableSource,
    },
}

impl MachineRspVcoHalfState {
    pub const fn value(&self) -> Option<u8> {
        match self {
            Self::Available { value, .. } => Some(*value),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available { .. })
    }

    pub const fn available_source(&self) -> Option<&MachineRspVcoHalfSource> {
        match self {
            Self::Available { source, .. } => Some(source),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn unavailable_source(&self) -> Option<&MachineRspVcoHalfUnavailableSource> {
        match self {
            Self::Available { .. } => None,
            Self::Unavailable { source } => Some(source),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspVcoState {
    carry_or_borrow: MachineRspVcoHalfState,
    not_equal: MachineRspVcoHalfState,
}

impl MachineRspVcoState {
    pub const fn carry_or_borrow(&self) -> &MachineRspVcoHalfState {
        &self.carry_or_borrow
    }

    pub const fn not_equal(&self) -> &MachineRspVcoHalfState {
        &self.not_equal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspVccSource {
    ConstructionOrReset,
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspVccState {
    Available {
        value: u16,
        source: MachineRspVccSource,
    },
    Unavailable {
        source: MachineRspVccSource,
    },
}

impl MachineRspVccState {
    pub const fn value(self) -> Option<u16> {
        match self {
            Self::Available { value, .. } => Some(value),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available { .. })
    }

    pub const fn source(self) -> MachineRspVccSource {
        match self {
            Self::Available { source, .. } | Self::Unavailable { source } => source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspVceSource {
    ConstructionOrReset,
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspVceState {
    Available {
        value: u8,
        source: MachineRspVceSource,
    },
    Unavailable {
        source: MachineRspVceSource,
    },
}

impl MachineRspVceState {
    pub const fn value(self) -> Option<u8> {
        match self {
            Self::Available { value, .. } => Some(value),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available { .. })
    }

    pub const fn source(self) -> MachineRspVceSource {
        match self {
            Self::Available { source, .. } | Self::Unavailable { source } => source,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspAccumulatorAndFlagsState {
    accumulator: MachineRspAccumulatorState,
    vco: MachineRspVcoState,
    vcc: MachineRspVccState,
    vce: MachineRspVceState,
}

impl MachineRspAccumulatorAndFlagsState {
    pub const fn accumulator(&self) -> &MachineRspAccumulatorState {
        &self.accumulator
    }

    pub const fn vco(&self) -> &MachineRspVcoState {
        &self.vco
    }

    pub const fn vcc(&self) -> MachineRspVccState {
        self.vcc
    }

    pub const fn vce(&self) -> MachineRspVceState {
        self.vce
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MachineRspBranchSource {
    J {
        instruction_pc: u16,
        instruction_provenance: [SpImemByteProvenance; 4],
        delay_slot_pc: u16,
        target_pc: u16,
    },
    Bltz {
        instruction_pc: u16,
        instruction_provenance: [SpImemByteProvenance; 4],
        source_gpr: u8,
        source_value: u32,
        source: MachineRspScalarRegisterSource,
        signed_offset: i16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
    },
    Bgez {
        instruction_pc: u16,
        instruction_provenance: [SpImemByteProvenance; 4],
        source_gpr: u8,
        source_value: u32,
        source: MachineRspScalarRegisterSource,
        signed_offset: i16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
    },
    Bgezal {
        instruction_pc: u16,
        instruction_provenance: [SpImemByteProvenance; 4],
        source_gpr: u8,
        source_value: u32,
        source: MachineRspScalarRegisterSource,
        signed_offset: i16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
        link_value: u32,
    },
    Bne {
        instruction_pc: u16,
        instruction_provenance: [SpImemByteProvenance; 4],
        source_gpr_a: u8,
        source_value_a: u32,
        source_a: MachineRspScalarRegisterSource,
        source_gpr_b: u8,
        source_value_b: u32,
        source_b: MachineRspScalarRegisterSource,
        signed_offset: i16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
    },
}

impl MachineRspBranchSource {
    pub const fn instruction_pc(&self) -> u16 {
        match self {
            Self::J { instruction_pc, .. }
            | Self::Bltz { instruction_pc, .. }
            | Self::Bgez { instruction_pc, .. }
            | Self::Bgezal { instruction_pc, .. }
            | Self::Bne { instruction_pc, .. } => *instruction_pc,
        }
    }

    pub const fn identity(&self) -> MachineRspInstructionIdentity {
        match self {
            Self::J { .. } => MachineRspInstructionIdentity::J,
            Self::Bltz { .. } => MachineRspInstructionIdentity::Bltz,
            Self::Bgez { .. } => MachineRspInstructionIdentity::Bgez,
            Self::Bgezal { .. } => MachineRspInstructionIdentity::Bgezal,
            Self::Bne { .. } => MachineRspInstructionIdentity::Bne,
        }
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(match self {
            Self::J {
                instruction_provenance,
                ..
            }
            | Self::Bltz {
                instruction_provenance,
                ..
            }
            | Self::Bgez {
                instruction_provenance,
                ..
            }
            | Self::Bgezal {
                instruction_provenance,
                ..
            }
            | Self::Bne {
                instruction_provenance,
                ..
            } => *instruction_provenance,
        })
    }

    pub const fn source_gpr_a(&self) -> Option<u8> {
        match self {
            Self::J { .. } => None,
            Self::Bltz { source_gpr, .. }
            | Self::Bgez { source_gpr, .. }
            | Self::Bgezal { source_gpr, .. } => Some(*source_gpr),
            Self::Bne { source_gpr_a, .. } => Some(*source_gpr_a),
        }
    }

    pub const fn source_value_a(&self) -> Option<u32> {
        match self {
            Self::J { .. } => None,
            Self::Bltz { source_value, .. }
            | Self::Bgez { source_value, .. }
            | Self::Bgezal { source_value, .. } => Some(*source_value),
            Self::Bne { source_value_a, .. } => Some(*source_value_a),
        }
    }

    pub fn source_a(&self) -> Option<MachineRspScalarRegisterSource> {
        match self {
            Self::J { .. } => None,
            Self::Bltz { source, .. } | Self::Bgez { source, .. } | Self::Bgezal { source, .. } => {
                Some(source.clone())
            }
            Self::Bne { source_a, .. } => Some(source_a.clone()),
        }
    }

    pub const fn source_gpr_b(&self) -> Option<u8> {
        match self {
            Self::J { .. } | Self::Bltz { .. } | Self::Bgez { .. } | Self::Bgezal { .. } => None,
            Self::Bne { source_gpr_b, .. } => Some(*source_gpr_b),
        }
    }

    pub const fn source_value_b(&self) -> Option<u32> {
        match self {
            Self::J { .. } | Self::Bltz { .. } | Self::Bgez { .. } | Self::Bgezal { .. } => None,
            Self::Bne { source_value_b, .. } => Some(*source_value_b),
        }
    }

    pub fn source_b(&self) -> Option<MachineRspScalarRegisterSource> {
        match self {
            Self::J { .. } | Self::Bltz { .. } | Self::Bgez { .. } | Self::Bgezal { .. } => None,
            Self::Bne { source_b, .. } => Some(source_b.clone()),
        }
    }

    pub const fn signed_offset(&self) -> Option<i16> {
        match self {
            Self::J { .. } => None,
            Self::Bltz { signed_offset, .. }
            | Self::Bgez { signed_offset, .. }
            | Self::Bgezal { signed_offset, .. }
            | Self::Bne { signed_offset, .. } => Some(*signed_offset),
        }
    }

    pub const fn delay_slot_pc(&self) -> u16 {
        match self {
            Self::J { delay_slot_pc, .. }
            | Self::Bltz { delay_slot_pc, .. }
            | Self::Bgez { delay_slot_pc, .. }
            | Self::Bgezal { delay_slot_pc, .. }
            | Self::Bne { delay_slot_pc, .. } => *delay_slot_pc,
        }
    }

    pub const fn target_pc(&self) -> u16 {
        match self {
            Self::J { target_pc, .. }
            | Self::Bltz { target_pc, .. }
            | Self::Bgez { target_pc, .. }
            | Self::Bgezal { target_pc, .. }
            | Self::Bne { target_pc, .. } => *target_pc,
        }
    }

    pub const fn taken(&self) -> bool {
        match self {
            Self::J { .. } => true,
            Self::Bltz { taken, .. }
            | Self::Bgez { taken, .. }
            | Self::Bgezal { taken, .. }
            | Self::Bne { taken, .. } => *taken,
        }
    }

    const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        match self {
            Self::J {
                instruction_provenance,
                ..
            }
            | Self::Bltz {
                instruction_provenance,
                ..
            }
            | Self::Bgez {
                instruction_provenance,
                ..
            }
            | Self::Bgezal {
                instruction_provenance,
                ..
            }
            | Self::Bne {
                instruction_provenance,
                ..
            } => *instruction_provenance,
        }
    }

    fn bgezal_link(&self) -> Option<(u32, MachineRspBgezalSource)> {
        let Self::Bgezal {
            instruction_pc,
            instruction_provenance,
            source_gpr,
            source_value,
            source,
            signed_offset,
            taken,
            link_value,
            ..
        } = self
        else {
            return None;
        };
        taken.then(|| {
            (
                *link_value,
                MachineRspBgezalSource {
                    instruction_pc: *instruction_pc,
                    instruction_provenance: *instruction_provenance,
                    source_gpr: *source_gpr,
                    source_value: *source_value,
                    source: source.clone(),
                    signed_offset: *signed_offset,
                    link_value: *link_value,
                },
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineRspDelaySlotContext {
    branch: MachineRspBranchSource,
}

impl MachineRspDelaySlotContext {
    pub(crate) fn new(branch: MachineRspBranchSource) -> Self {
        Self { branch }
    }

    pub const fn owner_pc(&self) -> u16 {
        self.branch.instruction_pc()
    }

    pub const fn identity(&self) -> MachineRspInstructionIdentity {
        self.branch.identity()
    }

    pub const fn delay_slot_pc(&self) -> u16 {
        self.branch.delay_slot_pc()
    }

    pub const fn target_pc(&self) -> u16 {
        self.branch.target_pc()
    }

    pub const fn taken(&self) -> bool {
        self.branch.taken()
    }

    pub fn instruction_source(&self) -> MachineRspInstructionSource {
        self.branch.instruction_source()
    }

    pub const fn source_gpr_a(&self) -> Option<u8> {
        self.branch.source_gpr_a()
    }

    pub const fn source_value_a(&self) -> Option<u32> {
        self.branch.source_value_a()
    }

    pub fn source_a(&self) -> Option<MachineRspScalarRegisterSource> {
        self.branch.source_a()
    }

    pub const fn source_gpr_b(&self) -> Option<u8> {
        self.branch.source_gpr_b()
    }

    pub const fn source_value_b(&self) -> Option<u32> {
        self.branch.source_value_b()
    }

    pub fn source_b(&self) -> Option<MachineRspScalarRegisterSource> {
        self.branch.source_b()
    }

    pub const fn signed_offset(&self) -> Option<i16> {
        self.branch.signed_offset()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspInstructionIdentity {
    Mfc0,
    Mtc0,
    Lui,
    Addi,
    J,
    Bltz,
    Bgez,
    Bgezal,
    Bne,
    Ori,
    Xori,
    Sll,
    Lqv,
    Vsub,
    Vaddc,
    Lw,
    Nop,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspInstructionSource {
    UserSuppliedPifFirmware,
    PublicSyntheticColdX105Bootstrap,
    CpuStoreWord,
    CpuStoreByte,
    SpDma {
        record_index: u8,
    },
    MixedKnown,
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRspBreakSource {
    instruction_pc: u16,
    prior_next_pc: u16,
    instruction_provenance: [SpImemByteProvenance; 4],
    raw_word: u32,
    pre_break_status: MachineSpStatusState,
    pre_break_mi_sp_pending: bool,
    pre_break_mi_sp_pending_source: Option<MachineMiInterruptPendingSource>,
    interrupt_signaled: bool,
}

impl MachineRspBreakSource {
    pub const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub const fn prior_next_pc(self) -> u16 {
        self.prior_next_pc
    }

    pub fn instruction_source(self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    #[cfg(test)]
    pub(crate) const fn byte_provenance(self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn pre_break_status(self) -> MachineSpStatusState {
        self.pre_break_status
    }

    pub const fn pre_break_mi_sp_pending(self) -> bool {
        self.pre_break_mi_sp_pending
    }

    pub const fn pre_break_mi_sp_pending_source(self) -> Option<MachineMiInterruptPendingSource> {
        self.pre_break_mi_sp_pending_source
    }

    pub const fn interrupt_on_break(self) -> bool {
        self.pre_break_status.interrupt_on_break()
    }

    pub const fn interrupt_signaled(self) -> bool {
        self.interrupt_signaled
    }

    pub(crate) const fn mi_interrupt_source(self) -> MachineMiRspBreakInterruptSource {
        MachineMiRspBreakInterruptSource::new(
            self.instruction_pc,
            self.prior_next_pc,
            self.instruction_provenance,
        )
    }

    const fn instruction_provenance(self) -> [SpImemByteProvenance; 4] {
        self.instruction_provenance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MachineRspLastInstructionDestination {
    ScalarMfc0 {
        destination_gpr: u8,
        control_register: MachineRspControlRegister,
    },
    VectorLqv {
        destination_vector: u8,
    },
    VectorArithmetic {
        destination_vector: u8,
    },
    ScalarLw {
        destination_gpr: u8,
    },
    ScalarMtc0 {
        source_gpr: u8,
        control_register: MachineRspControlRegister,
        source_index: usize,
    },
    ScalarLui {
        destination_gpr: u8,
    },
    ScalarAddi {
        destination_gpr: u8,
    },
    Branch,
    BranchAndLink {
        destination_gpr: u8,
    },
    ScalarOri {
        destination_gpr: u8,
    },
    ScalarXori {
        destination_gpr: u8,
    },
    ScalarSll {
        destination_gpr: u8,
    },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRspLastInstructionState {
    instruction_pc: u16,
    identity: MachineRspInstructionIdentity,
    destination: MachineRspLastInstructionDestination,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspLastInstructionState {
    pub const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub const fn identity(self) -> MachineRspInstructionIdentity {
        self.identity
    }

    pub const fn destination_gpr(self) -> Option<u8> {
        match self.destination {
            MachineRspLastInstructionDestination::ScalarMfc0 {
                destination_gpr, ..
            } => Some(destination_gpr),
            MachineRspLastInstructionDestination::ScalarLw { destination_gpr } => {
                Some(destination_gpr)
            }
            MachineRspLastInstructionDestination::ScalarLui { destination_gpr }
            | MachineRspLastInstructionDestination::ScalarAddi { destination_gpr } => {
                Some(destination_gpr)
            }
            MachineRspLastInstructionDestination::ScalarOri { destination_gpr }
            | MachineRspLastInstructionDestination::ScalarXori { destination_gpr }
            | MachineRspLastInstructionDestination::ScalarSll { destination_gpr } => {
                Some(destination_gpr)
            }
            MachineRspLastInstructionDestination::BranchAndLink { destination_gpr } => {
                Some(destination_gpr)
            }
            MachineRspLastInstructionDestination::VectorLqv { .. }
            | MachineRspLastInstructionDestination::VectorArithmetic { .. }
            | MachineRspLastInstructionDestination::ScalarMtc0 { .. }
            | MachineRspLastInstructionDestination::Branch
            | MachineRspLastInstructionDestination::None => None,
        }
    }

    pub const fn control_register(self) -> Option<MachineRspControlRegister> {
        match self.destination {
            MachineRspLastInstructionDestination::ScalarMfc0 {
                control_register, ..
            } => Some(control_register),
            MachineRspLastInstructionDestination::ScalarMtc0 {
                control_register, ..
            } => Some(control_register),
            MachineRspLastInstructionDestination::VectorLqv { .. }
            | MachineRspLastInstructionDestination::VectorArithmetic { .. }
            | MachineRspLastInstructionDestination::ScalarLw { .. }
            | MachineRspLastInstructionDestination::ScalarLui { .. }
            | MachineRspLastInstructionDestination::ScalarAddi { .. }
            | MachineRspLastInstructionDestination::Branch
            | MachineRspLastInstructionDestination::BranchAndLink { .. }
            | MachineRspLastInstructionDestination::ScalarOri { .. }
            | MachineRspLastInstructionDestination::ScalarXori { .. }
            | MachineRspLastInstructionDestination::ScalarSll { .. }
            | MachineRspLastInstructionDestination::None => None,
        }
    }

    pub const fn source_gpr(self) -> Option<u8> {
        match self.destination {
            MachineRspLastInstructionDestination::ScalarMtc0 { source_gpr, .. } => Some(source_gpr),
            _ => None,
        }
    }

    pub const fn mtc0_source_index(self) -> Option<usize> {
        match self.destination {
            MachineRspLastInstructionDestination::ScalarMtc0 { source_index, .. } => {
                Some(source_index)
            }
            _ => None,
        }
    }

    pub const fn destination_vector(self) -> Option<u8> {
        match self.destination {
            MachineRspLastInstructionDestination::ScalarMfc0 { .. }
            | MachineRspLastInstructionDestination::ScalarMtc0 { .. }
            | MachineRspLastInstructionDestination::ScalarLw { .. }
            | MachineRspLastInstructionDestination::ScalarLui { .. }
            | MachineRspLastInstructionDestination::ScalarAddi { .. }
            | MachineRspLastInstructionDestination::Branch
            | MachineRspLastInstructionDestination::BranchAndLink { .. }
            | MachineRspLastInstructionDestination::ScalarOri { .. }
            | MachineRspLastInstructionDestination::ScalarXori { .. }
            | MachineRspLastInstructionDestination::ScalarSll { .. }
            | MachineRspLastInstructionDestination::None => None,
            MachineRspLastInstructionDestination::VectorLqv { destination_vector } => {
                Some(destination_vector)
            }
            MachineRspLastInstructionDestination::VectorArithmetic { destination_vector } => {
                Some(destination_vector)
            }
        }
    }

    pub fn source(self) -> MachineRspInstructionSource {
        classify_instruction_source(self.byte_provenance)
    }

    #[cfg(test)]
    pub(crate) const fn byte_provenance(self) -> [SpImemByteProvenance; 4] {
        self.byte_provenance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspStepOutcome {
    ScalarMfc0Committed {
        instruction_pc: u16,
        destination_gpr: u8,
        control_register: MachineRspControlRegister,
        result_value: u32,
    },
    VectorLqvCommitted {
        instruction_pc: u16,
        destination_vector: u8,
        local_dmem_address: u16,
        result_available: bool,
    },
    ScalarLwCommitted {
        instruction_pc: u16,
        destination_gpr: u8,
        local_dmem_address: u16,
        result_value: u32,
    },
    ScalarMtc0Committed {
        instruction_pc: u16,
        source_gpr: u8,
        source_value: u32,
        control_register: MachineRspControlRegister,
        source_index: usize,
    },
    ScalarLuiCommitted {
        instruction_pc: u16,
        destination_gpr: u8,
        result_value: u32,
    },
    ScalarAddiCommitted {
        instruction_pc: u16,
        destination_gpr: u8,
        result_value: u32,
    },
    ScalarJCommitted {
        instruction_pc: u16,
        delay_slot_pc: u16,
        target_pc: u16,
    },
    ScalarBltzCommitted {
        instruction_pc: u16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
    },
    ScalarBgezCommitted {
        instruction_pc: u16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
    },
    ScalarBgezalCommitted {
        instruction_pc: u16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
        link_value: Option<u32>,
    },
    ScalarBneCommitted {
        instruction_pc: u16,
        delay_slot_pc: u16,
        target_pc: u16,
        taken: bool,
    },
    ScalarOriCommitted {
        instruction_pc: u16,
        destination_gpr: u8,
        result_value: u32,
    },
    ScalarXoriCommitted {
        instruction_pc: u16,
        destination_gpr: u8,
        result_value: u32,
    },
    ScalarSllCommitted {
        instruction_pc: u16,
        destination_gpr: u8,
        result_value: u32,
    },
    VectorVsubCommitted {
        instruction_pc: u16,
        destination_vector: u8,
        result_available: bool,
    },
    VectorVaddcCommitted {
        instruction_pc: u16,
        destination_vector: u8,
        result_available: bool,
    },
    NopCommitted {
        instruction_pc: u16,
    },
    BreakCommitted {
        instruction_pc: u16,
        interrupt_on_break: bool,
        interrupt_signaled: bool,
    },
}

impl MachineRspStepOutcome {
    pub const fn identity(self) -> MachineRspInstructionIdentity {
        match self {
            Self::ScalarMfc0Committed { .. } => MachineRspInstructionIdentity::Mfc0,
            Self::ScalarMtc0Committed { .. } => MachineRspInstructionIdentity::Mtc0,
            Self::ScalarLuiCommitted { .. } => MachineRspInstructionIdentity::Lui,
            Self::ScalarAddiCommitted { .. } => MachineRspInstructionIdentity::Addi,
            Self::ScalarJCommitted { .. } => MachineRspInstructionIdentity::J,
            Self::ScalarBltzCommitted { .. } => MachineRspInstructionIdentity::Bltz,
            Self::ScalarBgezCommitted { .. } => MachineRspInstructionIdentity::Bgez,
            Self::ScalarBgezalCommitted { .. } => MachineRspInstructionIdentity::Bgezal,
            Self::ScalarBneCommitted { .. } => MachineRspInstructionIdentity::Bne,
            Self::ScalarOriCommitted { .. } => MachineRspInstructionIdentity::Ori,
            Self::ScalarXoriCommitted { .. } => MachineRspInstructionIdentity::Xori,
            Self::ScalarSllCommitted { .. } => MachineRspInstructionIdentity::Sll,
            Self::VectorLqvCommitted { .. } => MachineRspInstructionIdentity::Lqv,
            Self::VectorVsubCommitted { .. } => MachineRspInstructionIdentity::Vsub,
            Self::VectorVaddcCommitted { .. } => MachineRspInstructionIdentity::Vaddc,
            Self::ScalarLwCommitted { .. } => MachineRspInstructionIdentity::Lw,
            Self::NopCommitted { .. } => MachineRspInstructionIdentity::Nop,
            Self::BreakCommitted { .. } => MachineRspInstructionIdentity::Break,
        }
    }

    pub const fn instruction_pc(self) -> u16 {
        match self {
            Self::ScalarMfc0Committed { instruction_pc, .. }
            | Self::ScalarMtc0Committed { instruction_pc, .. }
            | Self::ScalarLuiCommitted { instruction_pc, .. }
            | Self::ScalarAddiCommitted { instruction_pc, .. }
            | Self::ScalarJCommitted { instruction_pc, .. }
            | Self::ScalarBltzCommitted { instruction_pc, .. }
            | Self::ScalarBgezCommitted { instruction_pc, .. }
            | Self::ScalarBgezalCommitted { instruction_pc, .. }
            | Self::ScalarBneCommitted { instruction_pc, .. }
            | Self::ScalarOriCommitted { instruction_pc, .. }
            | Self::ScalarXoriCommitted { instruction_pc, .. }
            | Self::ScalarSllCommitted { instruction_pc, .. }
            | Self::VectorLqvCommitted { instruction_pc, .. }
            | Self::VectorVsubCommitted { instruction_pc, .. }
            | Self::VectorVaddcCommitted { instruction_pc, .. }
            | Self::ScalarLwCommitted { instruction_pc, .. }
            | Self::NopCommitted { instruction_pc }
            | Self::BreakCommitted { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn destination_gpr(self) -> Option<u8> {
        match self {
            Self::ScalarMfc0Committed {
                destination_gpr, ..
            }
            | Self::ScalarLwCommitted {
                destination_gpr, ..
            }
            | Self::ScalarLuiCommitted {
                destination_gpr, ..
            }
            | Self::ScalarAddiCommitted {
                destination_gpr, ..
            }
            | Self::ScalarOriCommitted {
                destination_gpr, ..
            }
            | Self::ScalarXoriCommitted {
                destination_gpr, ..
            }
            | Self::ScalarSllCommitted {
                destination_gpr, ..
            } => Some(destination_gpr),
            Self::ScalarMtc0Committed { .. }
            | Self::ScalarJCommitted { .. }
            | Self::ScalarBltzCommitted { .. }
            | Self::ScalarBgezCommitted { .. }
            | Self::ScalarBgezalCommitted {
                link_value: None, ..
            }
            | Self::ScalarBneCommitted { .. }
            | Self::VectorLqvCommitted { .. }
            | Self::VectorVsubCommitted { .. }
            | Self::VectorVaddcCommitted { .. }
            | Self::NopCommitted { .. }
            | Self::BreakCommitted { .. } => None,
            Self::ScalarBgezalCommitted {
                link_value: Some(_),
                ..
            } => Some(31),
        }
    }

    pub const fn control_register(self) -> Option<MachineRspControlRegister> {
        match self {
            Self::ScalarMfc0Committed {
                control_register, ..
            } => Some(control_register),
            Self::ScalarMtc0Committed {
                control_register, ..
            } => Some(control_register),
            Self::VectorLqvCommitted { .. }
            | Self::VectorVsubCommitted { .. }
            | Self::VectorVaddcCommitted { .. }
            | Self::ScalarLuiCommitted { .. }
            | Self::ScalarAddiCommitted { .. }
            | Self::ScalarJCommitted { .. }
            | Self::ScalarBltzCommitted { .. }
            | Self::ScalarBgezCommitted { .. }
            | Self::ScalarBgezalCommitted { .. }
            | Self::ScalarBneCommitted { .. }
            | Self::ScalarOriCommitted { .. }
            | Self::ScalarXoriCommitted { .. }
            | Self::ScalarSllCommitted { .. }
            | Self::ScalarLwCommitted { .. }
            | Self::NopCommitted { .. }
            | Self::BreakCommitted { .. } => None,
        }
    }

    pub const fn result_value(self) -> Option<u32> {
        match self {
            Self::ScalarMfc0Committed { result_value, .. }
            | Self::ScalarLwCommitted { result_value, .. }
            | Self::ScalarLuiCommitted { result_value, .. }
            | Self::ScalarAddiCommitted { result_value, .. }
            | Self::ScalarOriCommitted { result_value, .. }
            | Self::ScalarXoriCommitted { result_value, .. } => Some(result_value),
            Self::ScalarSllCommitted { result_value, .. } => Some(result_value),
            Self::ScalarMtc0Committed { .. }
            | Self::ScalarJCommitted { .. }
            | Self::ScalarBltzCommitted { .. }
            | Self::ScalarBgezCommitted { .. }
            | Self::ScalarBgezalCommitted {
                link_value: None, ..
            }
            | Self::ScalarBneCommitted { .. }
            | Self::VectorLqvCommitted { .. }
            | Self::VectorVsubCommitted { .. }
            | Self::VectorVaddcCommitted { .. }
            | Self::NopCommitted { .. }
            | Self::BreakCommitted { .. } => None,
            Self::ScalarBgezalCommitted {
                link_value: Some(link_value),
                ..
            } => Some(link_value),
        }
    }

    pub const fn destination_vector(self) -> Option<u8> {
        match self {
            Self::ScalarMfc0Committed { .. }
            | Self::ScalarMtc0Committed { .. }
            | Self::ScalarLuiCommitted { .. }
            | Self::ScalarAddiCommitted { .. }
            | Self::ScalarJCommitted { .. }
            | Self::ScalarBltzCommitted { .. }
            | Self::ScalarBgezCommitted { .. }
            | Self::ScalarBgezalCommitted { .. }
            | Self::ScalarBneCommitted { .. }
            | Self::ScalarOriCommitted { .. }
            | Self::ScalarXoriCommitted { .. }
            | Self::ScalarSllCommitted { .. }
            | Self::ScalarLwCommitted { .. }
            | Self::NopCommitted { .. }
            | Self::BreakCommitted { .. } => None,
            Self::VectorLqvCommitted {
                destination_vector, ..
            }
            | Self::VectorVsubCommitted {
                destination_vector, ..
            }
            | Self::VectorVaddcCommitted {
                destination_vector, ..
            } => Some(destination_vector),
        }
    }

    pub const fn local_dmem_address(self) -> Option<u16> {
        match self {
            Self::ScalarMfc0Committed { .. }
            | Self::ScalarMtc0Committed { .. }
            | Self::ScalarLuiCommitted { .. }
            | Self::ScalarAddiCommitted { .. }
            | Self::ScalarJCommitted { .. }
            | Self::ScalarBltzCommitted { .. }
            | Self::ScalarBgezCommitted { .. }
            | Self::ScalarBgezalCommitted { .. }
            | Self::ScalarBneCommitted { .. }
            | Self::ScalarOriCommitted { .. }
            | Self::ScalarXoriCommitted { .. }
            | Self::ScalarSllCommitted { .. }
            | Self::VectorVsubCommitted { .. }
            | Self::VectorVaddcCommitted { .. }
            | Self::NopCommitted { .. }
            | Self::BreakCommitted { .. } => None,
            Self::ScalarLwCommitted {
                local_dmem_address, ..
            } => Some(local_dmem_address),
            Self::VectorLqvCommitted {
                local_dmem_address, ..
            } => Some(local_dmem_address),
        }
    }

    pub const fn vector_result_available(self) -> Option<bool> {
        match self {
            Self::ScalarMfc0Committed { .. }
            | Self::ScalarMtc0Committed { .. }
            | Self::ScalarLuiCommitted { .. }
            | Self::ScalarAddiCommitted { .. }
            | Self::ScalarJCommitted { .. }
            | Self::ScalarBltzCommitted { .. }
            | Self::ScalarBgezCommitted { .. }
            | Self::ScalarBgezalCommitted { .. }
            | Self::ScalarBneCommitted { .. }
            | Self::ScalarOriCommitted { .. }
            | Self::ScalarXoriCommitted { .. } => None,
            Self::ScalarSllCommitted { .. } => None,
            Self::ScalarLwCommitted { .. }
            | Self::NopCommitted { .. }
            | Self::BreakCommitted { .. } => None,
            Self::VectorLqvCommitted {
                result_available, ..
            }
            | Self::VectorVsubCommitted {
                result_available, ..
            }
            | Self::VectorVaddcCommitted {
                result_available, ..
            } => Some(result_available),
        }
    }

    pub const fn interrupt_on_break(self) -> Option<bool> {
        match self {
            Self::BreakCommitted {
                interrupt_on_break, ..
            } => Some(interrupt_on_break),
            _ => None,
        }
    }

    pub const fn interrupt_signaled(self) -> Option<bool> {
        match self {
            Self::BreakCommitted {
                interrupt_signaled, ..
            } => Some(interrupt_signaled),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspFetchRejection {
    PcUnavailable,
    UnalignedPc { pc: u16 },
    OutOfRangePc { pc: u16 },
    UnknownImemWord { pc: u16 },
    OpaqueImemWord { pc: u16 },
    InconsistentImemKnowledge { pc: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspUnrepresentedInstructionClass {
    Scalar,
    Cop0Transfer,
    Vector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspStepRejectionReason {
    SingleStepUnsupported,
    Fetch(MachineRspFetchRejection),
    MalformedMfc0Encoding,
    UnsupportedCop0Register {
        register_index: u8,
    },
    MalformedMtc0Encoding,
    Mtc0SourceUnavailable {
        source_gpr: u8,
    },
    UnsupportedMtc0ControlRegister {
        register_index: u8,
    },
    Mtc0SpStatusCommandMalformed,
    Mtc0SpStatusInterruptCommandUnsupported,
    Mtc0DmaRecordCapacityExhausted,
    Mtc0DmaAddressUnavailable,
    Mtc0DmaRdramRangeRejected {
        physical_address: u32,
    },
    Mtc0WriteDmaRecordCapacityExhausted,
    Mtc0WriteDmaAddressUnavailable,
    Mtc0WriteDmaSourceRangeRejected {
        local_address: u16,
    },
    Mtc0WriteDmaSourceUnavailable {
        local_address: u16,
    },
    Mtc0WriteDmaSourceOpaque {
        local_address: u16,
    },
    Mtc0WriteDmaSourceKnowledgeInconsistent {
        local_address: u16,
    },
    Mtc0WriteDmaRdramRangeRejected {
        physical_address: u32,
    },
    DpcStatusCommandUnsupported {
        raw_command_word: u32,
    },
    DpcCounterInvariantMalformed {
        counter: MachineDpcCounterIdentity,
        value: u32,
    },
    BreakCodeUnsupported {
        code: u32,
    },
    BreakInDelaySlotUnsupported {
        owner_pc: u16,
    },
    XoriSourceUnavailable {
        source_gpr: u8,
    },
    OriSourceUnavailable {
        source_gpr: u8,
    },
    MalformedLuiEncoding,
    AddiSourceUnavailable {
        source_gpr: u8,
    },
    UnsupportedRegimmSelector {
        selector: u8,
    },
    BltzSourceUnavailable {
        source_gpr: u8,
    },
    BgezSourceUnavailable {
        source_gpr: u8,
    },
    BgezalSourceUnavailable {
        source_gpr: u8,
    },
    BneSourceAUnavailable {
        source_gpr: u8,
    },
    BneSourceBUnavailable {
        source_gpr: u8,
    },
    ControlFlowInDelaySlot {
        owner_pc: u16,
    },
    LqvScalarBaseUnavailable {
        base_gpr: u8,
    },
    LqvElementUnsupported {
        element: u8,
    },
    LqvAddressMisaligned {
        local_dmem_address: u16,
    },
    LqvDmemKnowledgeMalformed {
        local_dmem_address: u16,
    },
    VectorLoadUnsupported {
        subopcode: u8,
    },
    VectorStoreUnsupported,
    ScalarLwBaseUnavailable {
        base_gpr: u8,
    },
    ScalarLwAddressMisaligned {
        local_dmem_address: u16,
    },
    ScalarLwDmemByteUnavailable {
        local_dmem_address: u16,
        first_unavailable_offset: u16,
    },
    ScalarLwDmemKnowledgeMalformed {
        local_dmem_address: u16,
    },
    ScalarLoadUnsupported {
        opcode: u8,
    },
    ScalarStoreUnsupported {
        opcode: u8,
    },
    MalformedSllEncoding,
    SllSourceUnavailable {
        source_gpr: u8,
    },
    VsubElementUnsupported {
        element: u8,
    },
    VaddcElementUnsupported {
        element: u8,
    },
    UnrepresentedInstruction {
        class: MachineRspUnrepresentedInstructionClass,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRspStepRejection {
    reason: MachineRspStepRejectionReason,
}

impl MachineRspStepRejection {
    pub(crate) const fn new(reason: MachineRspStepRejectionReason) -> Self {
        Self { reason }
    }

    pub const fn reason(self) -> MachineRspStepRejectionReason {
        self.reason
    }
}

impl fmt::Display for MachineRspStepRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reason {
            MachineRspStepRejectionReason::SingleStepUnsupported => {
                write!(f, "RSP single-step execution is not represented")
            }
            MachineRspStepRejectionReason::Fetch(MachineRspFetchRejection::PcUnavailable) => {
                write!(f, "RSP fetch rejected because SP PC is unavailable")
            }
            MachineRspStepRejectionReason::Fetch(MachineRspFetchRejection::UnalignedPc { pc }) => {
                write!(f, "RSP fetch rejected for unaligned local PC 0x{pc:03x}")
            }
            MachineRspStepRejectionReason::Fetch(MachineRspFetchRejection::OutOfRangePc { pc }) => {
                write!(f, "RSP fetch rejected for out-of-range local PC 0x{pc:04x}")
            }
            MachineRspStepRejectionReason::Fetch(MachineRspFetchRejection::UnknownImemWord {
                pc,
            }) => write!(
                f,
                "RSP fetch rejected because IMEM knowledge is unavailable at local PC 0x{pc:03x}"
            ),
            MachineRspStepRejectionReason::Fetch(MachineRspFetchRejection::OpaqueImemWord {
                pc,
            }) => write!(
                f,
                "RSP fetch rejected because IMEM word truth is opaque at local PC 0x{pc:03x}"
            ),
            MachineRspStepRejectionReason::Fetch(
                MachineRspFetchRejection::InconsistentImemKnowledge { pc },
            ) => write!(
                f,
                "RSP fetch rejected because IMEM knowledge is inconsistent at local PC 0x{pc:03x}"
            ),
            MachineRspStepRejectionReason::MalformedMfc0Encoding => {
                write!(f, "RSP Mfc0 encoding is malformed")
            }
            MachineRspStepRejectionReason::UnsupportedCop0Register { register_index } => write!(
                f,
                "RSP Mfc0 control-register index {register_index} is not represented"
            ),
            MachineRspStepRejectionReason::MalformedMtc0Encoding => {
                write!(f, "RSP Mtc0 encoding is malformed")
            }
            MachineRspStepRejectionReason::Mtc0SourceUnavailable { source_gpr } => {
                write!(f, "RSP Mtc0 scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::UnsupportedMtc0ControlRegister {
                register_index,
            } => write!(
                f,
                "RSP Mtc0 control-register index {register_index} is not represented"
            ),
            MachineRspStepRejectionReason::Mtc0SpStatusCommandMalformed => {
                write!(f, "RSP Mtc0 SP_STATUS command is malformed")
            }
            MachineRspStepRejectionReason::Mtc0SpStatusInterruptCommandUnsupported => {
                write!(f, "RSP Mtc0 SP_STATUS interrupt command is not represented")
            }
            MachineRspStepRejectionReason::Mtc0DmaRecordCapacityExhausted => {
                write!(f, "RSP Mtc0 SP_RD_LEN rejected because SP DMA record capacity is exhausted")
            }
            MachineRspStepRejectionReason::Mtc0DmaAddressUnavailable => {
                write!(f, "RSP Mtc0 SP_RD_LEN rejected because one programmed SP DMA address is unavailable")
            }
            MachineRspStepRejectionReason::Mtc0DmaRdramRangeRejected {
                physical_address,
            } => write!(
                f,
                "RSP Mtc0 SP_RD_LEN rejected because RDRAM source address 0x{physical_address:08x} is outside represented memory"
            ),
            MachineRspStepRejectionReason::Mtc0WriteDmaRecordCapacityExhausted => {
                write!(f, "RSP Mtc0 SP_WR_LEN rejected because SP DMA record capacity is exhausted")
            }
            MachineRspStepRejectionReason::Mtc0WriteDmaAddressUnavailable => {
                write!(f, "RSP Mtc0 SP_WR_LEN rejected because one programmed SP DMA address is unavailable")
            }
            MachineRspStepRejectionReason::Mtc0WriteDmaSourceRangeRejected {
                local_address,
            } => write!(
                f,
                "RSP Mtc0 SP_WR_LEN rejected because selected SP source address 0x{local_address:04x} would require unsupported wrapping or lies outside represented memory"
            ),
            MachineRspStepRejectionReason::Mtc0WriteDmaSourceUnavailable {
                local_address,
            } => write!(
                f,
                "RSP Mtc0 SP_WR_LEN rejected because selected SP source byte 0x{local_address:04x} is unavailable"
            ),
            MachineRspStepRejectionReason::Mtc0WriteDmaSourceOpaque {
                local_address,
            } => write!(
                f,
                "RSP Mtc0 SP_WR_LEN rejected because selected SP source word at 0x{local_address:04x} is opaque"
            ),
            MachineRspStepRejectionReason::Mtc0WriteDmaSourceKnowledgeInconsistent {
                local_address,
            } => write!(
                f,
                "RSP Mtc0 SP_WR_LEN rejected because selected SP source knowledge at 0x{local_address:04x} is inconsistent"
            ),
            MachineRspStepRejectionReason::Mtc0WriteDmaRdramRangeRejected {
                physical_address,
            } => write!(
                f,
                "RSP Mtc0 SP_WR_LEN rejected because RDRAM destination address 0x{physical_address:08x} is outside represented memory"
            ),
            MachineRspStepRejectionReason::DpcStatusCommandUnsupported {
                raw_command_word,
            } => write!(
                f,
                "RSP Mtc0 DPC_STATUS command 0x{raw_command_word:08x} is outside the exact represented counter-clear command"
            ),
            MachineRspStepRejectionReason::DpcCounterInvariantMalformed {
                counter,
                value,
            } => write!(
                f,
                "RSP Mtc0 DPC_STATUS rejected malformed {counter:?} counter value 0x{value:08x}"
            ),
            MachineRspStepRejectionReason::BreakCodeUnsupported { code } => write!(
                f,
                "RSP Break code field 0x{code:05x} is outside the exact represented zero-code identity"
            ),
            MachineRspStepRejectionReason::BreakInDelaySlotUnsupported { owner_pc } => write!(
                f,
                "RSP Break in the active delay slot owned by local PC 0x{owner_pc:03x} is not represented"
            ),
            MachineRspStepRejectionReason::XoriSourceUnavailable { source_gpr } => {
                write!(f, "RSP Xori scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::OriSourceUnavailable { source_gpr } => {
                write!(f, "RSP Ori scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::MalformedLuiEncoding => {
                write!(f, "RSP scalar Lui encoding is malformed")
            }
            MachineRspStepRejectionReason::AddiSourceUnavailable { source_gpr } => {
                write!(f, "RSP Addi scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::UnsupportedRegimmSelector { selector } => write!(
                f,
                "RSP REGIMM selector {selector} is not represented beyond exact Bltz and Bgez"
            ),
            MachineRspStepRejectionReason::BltzSourceUnavailable { source_gpr } => {
                write!(f, "RSP Bltz scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::BgezSourceUnavailable { source_gpr } => {
                write!(f, "RSP Bgez scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::BgezalSourceUnavailable { source_gpr } => {
                write!(f, "RSP Bgezal scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::BneSourceAUnavailable { source_gpr } => {
                write!(f, "RSP Bne scalar source A r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::BneSourceBUnavailable { source_gpr } => {
                write!(f, "RSP Bne scalar source B r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::ControlFlowInDelaySlot { owner_pc } => write!(
                f,
                "RSP control flow in the active delay slot owned by local PC 0x{owner_pc:03x} is not represented"
            ),
            MachineRspStepRejectionReason::LqvScalarBaseUnavailable { base_gpr } => write!(
                f,
                "RSP Lqv scalar base r{base_gpr} is unavailable"
            ),
            MachineRspStepRejectionReason::LqvElementUnsupported { element } => write!(
                f,
                "RSP Lqv byte element {element} is outside the element-zero boundary"
            ),
            MachineRspStepRejectionReason::LqvAddressMisaligned {
                local_dmem_address,
            } => write!(
                f,
                "RSP Lqv local DMEM address 0x{local_dmem_address:03x} is outside the aligned full-register boundary"
            ),
            MachineRspStepRejectionReason::LqvDmemKnowledgeMalformed {
                local_dmem_address,
            } => write!(
                f,
                "RSP Lqv DMEM knowledge is malformed at local address 0x{local_dmem_address:03x}"
            ),
            MachineRspStepRejectionReason::VectorLoadUnsupported { subopcode } => write!(
                f,
                "RSP vector-load sub-operation {subopcode} is not represented"
            ),
            MachineRspStepRejectionReason::VectorStoreUnsupported => {
                write!(f, "RSP vector stores are not represented")
            }
            MachineRspStepRejectionReason::ScalarLwBaseUnavailable { base_gpr } => {
                write!(f, "RSP scalar Lw base r{base_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::ScalarLwAddressMisaligned {
                local_dmem_address,
            } => write!(
                f,
                "RSP scalar Lw local DMEM address 0x{local_dmem_address:03x} is outside the aligned word boundary"
            ),
            MachineRspStepRejectionReason::ScalarLwDmemByteUnavailable {
                local_dmem_address,
                first_unavailable_offset,
            } => write!(
                f,
                "RSP scalar Lw DMEM value is unavailable at local offset 0x{first_unavailable_offset:03x} in word 0x{local_dmem_address:03x}"
            ),
            MachineRspStepRejectionReason::ScalarLwDmemKnowledgeMalformed {
                local_dmem_address,
            } => write!(
                f,
                "RSP scalar Lw DMEM knowledge is malformed at local address 0x{local_dmem_address:03x}"
            ),
            MachineRspStepRejectionReason::ScalarLoadUnsupported { opcode } => {
                write!(f, "RSP scalar-load opcode 0x{opcode:02x} is not represented")
            }
            MachineRspStepRejectionReason::ScalarStoreUnsupported { opcode } => {
                write!(f, "RSP scalar-store opcode 0x{opcode:02x} is not represented")
            }
            MachineRspStepRejectionReason::MalformedSllEncoding => {
                write!(f, "RSP scalar Sll encoding is malformed")
            }
            MachineRspStepRejectionReason::SllSourceUnavailable { source_gpr } => {
                write!(f, "RSP scalar Sll source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::VsubElementUnsupported { element } => write!(
                f,
                "RSP Vsub element {element} is outside the element-zero lane-aligned boundary"
            ),
            MachineRspStepRejectionReason::VaddcElementUnsupported { element } => write!(
                f,
                "RSP Vaddc element {element} is outside the element-zero lane-aligned boundary"
            ),
            MachineRspStepRejectionReason::UnrepresentedInstruction { class } => {
                write!(f, "RSP {class:?} instruction identity is not represented")
            }
        }
    }
}

impl std::error::Error for MachineRspStepRejection {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineRspMfc0Plan {
    instruction_pc: u16,
    old_next_pc: u16,
    destination_gpr: u8,
    control_source: MachineRspMfc0ControlSource,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspMfc0Plan {
    pub(crate) const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(self) -> u16 {
        self.old_next_pc
    }

    pub(crate) const fn control_source(self) -> MachineRspMfc0ControlSource {
        self.control_source
    }

    pub(crate) fn instruction_source(self) -> MachineRspInstructionSource {
        classify_instruction_source(self.byte_provenance)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspMtc0Plan {
    instruction_pc: u16,
    old_next_pc: u16,
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    control_register: MachineRspControlRegister,
    byte_provenance: [SpImemByteProvenance; 4],
    source_index: usize,
}

impl MachineRspMtc0Plan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.old_next_pc
    }

    pub(crate) const fn source_value(&self) -> u32 {
        self.source_value
    }

    pub(crate) const fn source_gpr(&self) -> u8 {
        self.source_gpr
    }

    pub(crate) fn source(&self) -> MachineRspScalarRegisterSource {
        self.source.clone()
    }

    pub(crate) const fn control_register(&self) -> MachineRspControlRegister {
        self.control_register
    }

    pub(crate) const fn instruction_provenance(&self) -> [SpImemByteProvenance; 4] {
        self.byte_provenance
    }

    pub(crate) fn instruction_source(&self) -> MachineRspInstructionSource {
        classify_instruction_source(self.byte_provenance)
    }

    pub(crate) const fn source_index(&self) -> usize {
        self.source_index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineRspLuiPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    destination_gpr: u8,
    immediate: u16,
    result_value: u32,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspLuiPlan {
    pub(crate) const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(self) -> u16 {
        self.old_next_pc
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspAddiPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    destination_gpr: u8,
    signed_immediate: i16,
    result_value: u32,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspAddiPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.old_next_pc
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspBranchPlan {
    source: MachineRspBranchSource,
    selected_next_pc: u16,
}

impl MachineRspBranchPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.source.instruction_pc()
    }

    pub(crate) const fn delay_slot_pc(&self) -> u16 {
        self.source.delay_slot_pc()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspXoriPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    destination_gpr: u8,
    immediate: u16,
    result_value: u32,
    byte_provenance: [SpImemByteProvenance; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspOriPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    destination_gpr: u8,
    immediate: u16,
    result_value: u32,
    byte_provenance: [SpImemByteProvenance; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspSllPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    source_gpr: u8,
    source_value: u32,
    source: MachineRspScalarRegisterSource,
    destination_gpr: u8,
    shift_amount: u8,
    result_value: u32,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspSllPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.old_next_pc
    }
}

impl MachineRspOriPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.old_next_pc
    }
}

impl MachineRspXoriPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.old_next_pc
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspLqvAddressPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    destination_vector: u8,
    element: u8,
    signed_offset: i8,
    base_gpr: u8,
    base_value: u32,
    base_source: MachineRspScalarRegisterSource,
    local_dmem_address: u16,
}

impl MachineRspLqvAddressPlan {
    pub(crate) const fn local_dmem_address(&self) -> u16 {
        self.local_dmem_address
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspLqvPlan {
    address: MachineRspLqvAddressPlan,
    byte_provenance: [SpImemByteProvenance; 4],
    destination_state: MachineRspVectorRegisterState,
}

impl MachineRspLqvPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.address.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.address.old_next_pc
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspScalarLwAddressPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    base_gpr: u8,
    base_value: u32,
    base_source: MachineRspScalarRegisterSource,
    destination_gpr: u8,
    signed_offset: i16,
    local_dmem_address: u16,
}

impl MachineRspScalarLwAddressPlan {
    pub(crate) const fn local_dmem_address(&self) -> u16 {
        self.local_dmem_address
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspScalarLwPlan {
    address: MachineRspScalarLwAddressPlan,
    byte_provenance: [SpImemByteProvenance; RSP_SCALAR_LW_BYTE_COUNT],
    dmem_knowledge: [MachineSpDmemByteKnowledgeDescriptor; RSP_SCALAR_LW_BYTE_COUNT],
    result_value: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspVectorArithmeticPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    identity: MachineRspInstructionIdentity,
    destination_vector: u8,
    old_destination_state: MachineRspVectorRegisterState,
    byte_provenance: [SpImemByteProvenance; 4],
    destination_state: MachineRspVectorRegisterState,
    old_accumulator_and_flags: MachineRspAccumulatorAndFlagsState,
    accumulator_and_flags: MachineRspAccumulatorAndFlagsState,
}

impl MachineRspVectorArithmeticPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.old_next_pc
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineRspScalarLwDmemObservation {
    descriptor: MachineSpDmemByteKnowledgeDescriptor,
    value: Option<u8>,
}

impl MachineRspScalarLwDmemObservation {
    pub(crate) const fn from_knowledge(
        offset: SpDmemOffset,
        knowledge: MachineSpDmemByteKnowledge,
    ) -> Self {
        Self {
            descriptor: MachineSpDmemByteKnowledgeDescriptor::new(offset, knowledge.source()),
            value: knowledge.value(),
        }
    }

    #[cfg(test)]
    pub(crate) const fn from_parts(
        descriptor: MachineSpDmemByteKnowledgeDescriptor,
        value: Option<u8>,
    ) -> Self {
        Self { descriptor, value }
    }
}

impl MachineRspScalarLwPlan {
    pub(crate) const fn instruction_pc(&self) -> u16 {
        self.address.instruction_pc
    }

    pub(crate) const fn old_next_pc(&self) -> u16 {
        self.address.old_next_pc
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineRspNopPlan {
    instruction_pc: u16,
    old_next_pc: u16,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspNopPlan {
    pub(crate) const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub(crate) const fn old_next_pc(self) -> u16 {
        self.old_next_pc
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineRspBreakPlan {
    source: MachineRspBreakSource,
}

impl MachineRspBreakPlan {
    pub(crate) const fn source(self) -> MachineRspBreakSource {
        self.source
    }

    pub(crate) const fn instruction_pc(self) -> u16 {
        self.source.instruction_pc()
    }

    pub(crate) const fn old_next_pc(self) -> u16 {
        self.source.prior_next_pc()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineRspDecodedInstruction {
    Mfc0 {
        destination_gpr: u8,
        control_register: MachineRspControlRegister,
    },
    Mtc0 {
        source_gpr: u8,
        control_register: MachineRspControlRegister,
    },
    Lui {
        destination_gpr: u8,
        immediate: u16,
    },
    Addi {
        source_gpr: u8,
        destination_gpr: u8,
        signed_immediate: i16,
    },
    J {
        target_pc: u16,
    },
    Bltz {
        source_gpr: u8,
        signed_offset: i16,
    },
    Bgez {
        source_gpr: u8,
        signed_offset: i16,
    },
    Bgezal {
        source_gpr: u8,
        signed_offset: i16,
    },
    Bne {
        source_gpr_a: u8,
        source_gpr_b: u8,
        signed_offset: i16,
    },
    Xori {
        source_gpr: u8,
        destination_gpr: u8,
        immediate: u16,
    },
    Ori {
        source_gpr: u8,
        destination_gpr: u8,
        immediate: u16,
    },
    Sll {
        source_gpr: u8,
        destination_gpr: u8,
        shift_amount: u8,
    },
    Lqv {
        base_gpr: u8,
        destination_vector: u8,
        element: u8,
        signed_offset: i8,
    },
    Vsub {
        destination_vector: u8,
        source_vector_a: u8,
        source_vector_b: u8,
        element: u8,
    },
    Vaddc {
        destination_vector: u8,
        source_vector_a: u8,
        source_vector_b: u8,
        element: u8,
    },
    Lw {
        base_gpr: u8,
        destination_gpr: u8,
        signed_offset: i16,
    },
    Nop,
    Break,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MachineRspExecutionState {
    scalar_registers: [MachineRspScalarRegisterState; RSP_SCALAR_REGISTER_COUNT],
    next_pc: Option<u16>,
    delay_slot_context: Option<MachineRspDelaySlotContext>,
    committed_instruction_count: u64,
    last_instruction: Option<MachineRspLastInstructionState>,
    vector_unit: MachineRspVectorUnitState,
    accumulator_and_flags: MachineRspAccumulatorAndFlagsState,
    mtc0_sources: Vec<MachineRspMtc0Source>,
}

impl MachineRspExecutionState {
    pub(crate) fn clean_room_ntsc_x105_post_boot() -> Self {
        let mut state = Self::default();
        state.scalar_registers[RSP_NTSC_X105_POST_BOOT_GPR_11_INDEX] =
            MachineRspScalarRegisterState::Available {
                value: RSP_NTSC_X105_POST_BOOT_GPR_11_VALUE,
                source: MachineRspScalarRegisterSource::CleanRoomHleNtscX105PinnedPostBoot,
            };
        state
    }

    pub(crate) fn scalar_register(&self, index: usize) -> Option<MachineRspScalarRegisterState> {
        self.scalar_registers.get(index).cloned()
    }

    pub(crate) const fn next_pc(&self) -> Option<u16> {
        self.next_pc
    }

    pub(crate) fn delay_slot_context(&self) -> Option<MachineRspDelaySlotContext> {
        self.delay_slot_context.clone()
    }

    pub(crate) const fn committed_instruction_count(&self) -> u64 {
        self.committed_instruction_count
    }

    pub(crate) const fn last_instruction(&self) -> Option<MachineRspLastInstructionState> {
        self.last_instruction
    }

    pub(crate) const fn vector_unit(&self) -> &MachineRspVectorUnitState {
        &self.vector_unit
    }

    pub(crate) fn vector_register(&self, index: usize) -> Option<&MachineRspVectorRegisterState> {
        self.vector_unit.register(index)
    }

    pub(crate) fn accumulator_and_flags(&self) -> MachineRspAccumulatorAndFlagsState {
        self.accumulator_and_flags.clone()
    }

    pub(crate) fn mtc0_source(&self, index: usize) -> Option<&MachineRspMtc0Source> {
        self.mtc0_sources.get(index)
    }

    pub(crate) fn synchronize_pc_write(&mut self, current_pc: u16) {
        self.next_pc = Some(sequential_local_pc(current_pc));
        self.delay_slot_context = None;
    }

    pub(crate) fn decode(
        &self,
        raw_word: u32,
    ) -> Result<MachineRspDecodedInstruction, MachineRspStepRejection> {
        if raw_word == 0 {
            return Ok(MachineRspDecodedInstruction::Nop);
        }
        if raw_word >> 26 == 0 && raw_word & 0x3f == u32::from(RSP_SCALAR_BREAK_FUNCTION) {
            let code = (raw_word >> 6) & RSP_SCALAR_BREAK_CODE_MASK;
            if raw_word == RSP_SCALAR_BREAK_WORD {
                return Ok(MachineRspDecodedInstruction::Break);
            }
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::BreakCodeUnsupported { code },
            ));
        }
        let opcode = (raw_word >> 26) as u8;
        if opcode == RSP_SCALAR_J_OPCODE {
            return Ok(MachineRspDecodedInstruction::J {
                target_pc: jump_target_local_pc(raw_word & 0x03ff_ffff),
            });
        }
        if opcode == RSP_COP0_OPCODE {
            let transfer_selector = ((raw_word >> 21) & 0x1f) as u8;
            if transfer_selector == RSP_COP0_MTC0_TRANSFER_SELECTOR {
                if raw_word & 0x7ff != 0 {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::MalformedMtc0Encoding,
                    ));
                }
                let register_index = ((raw_word >> 11) & 0x1f) as u8;
                let control_register = match register_index {
                    RSP_COP0_SP_MEMORY_ADDRESS_INDEX => MachineRspControlRegister::SpMemoryAddress,
                    RSP_COP0_SP_DRAM_ADDRESS_INDEX => MachineRspControlRegister::SpDramAddress,
                    RSP_COP0_SP_READ_LENGTH_INDEX => MachineRspControlRegister::SpReadLength,
                    RSP_COP0_SP_WRITE_LENGTH_INDEX => MachineRspControlRegister::SpWriteLength,
                    RSP_COP0_SP_STATUS_INDEX => MachineRspControlRegister::SpStatus,
                    RSP_COP0_SP_SEMAPHORE_INDEX => MachineRspControlRegister::SpSemaphore,
                    RSP_COP0_DPC_STATUS_INDEX => MachineRspControlRegister::DpcStatus,
                    _ => {
                        return Err(MachineRspStepRejection::new(
                            MachineRspStepRejectionReason::UnsupportedMtc0ControlRegister {
                                register_index,
                            },
                        ))
                    }
                };
                return Ok(MachineRspDecodedInstruction::Mtc0 {
                    source_gpr: ((raw_word >> 16) & 0x1f) as u8,
                    control_register,
                });
            }
            if transfer_selector != RSP_COP0_MFC0_TRANSFER_SELECTOR {
                return Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::UnrepresentedInstruction {
                        class: MachineRspUnrepresentedInstructionClass::Cop0Transfer,
                    },
                ));
            }
            if raw_word & 0x7ff != 0 {
                return Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::MalformedMfc0Encoding,
                ));
            }
            let register_index = ((raw_word >> 11) & 0x1f) as u8;
            let control_register = match register_index {
                RSP_COP0_SP_DRAM_ADDRESS_INDEX => MachineRspControlRegister::SpDramAddress,
                RSP_COP0_SP_DMA_FULL_INDEX => MachineRspControlRegister::SpDmaFull,
                RSP_COP0_SP_DMA_BUSY_INDEX => MachineRspControlRegister::SpDmaBusy,
                RSP_COP0_SP_SEMAPHORE_INDEX => MachineRspControlRegister::SpSemaphore,
                _ => {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::UnsupportedCop0Register { register_index },
                    ))
                }
            };
            return Ok(MachineRspDecodedInstruction::Mfc0 {
                destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                control_register,
            });
        }
        if opcode == RSP_SCALAR_REGIMM_OPCODE {
            let selector = ((raw_word >> 16) & 0x1f) as u8;
            let source_gpr = ((raw_word >> 21) & 0x1f) as u8;
            let signed_offset = raw_word as u16 as i16;
            return match selector {
                RSP_SCALAR_BLTZ_SELECTOR => Ok(MachineRspDecodedInstruction::Bltz {
                    source_gpr,
                    signed_offset,
                }),
                RSP_SCALAR_BGEZ_SELECTOR => Ok(MachineRspDecodedInstruction::Bgez {
                    source_gpr,
                    signed_offset,
                }),
                RSP_SCALAR_BGEZAL_SELECTOR => Ok(MachineRspDecodedInstruction::Bgezal {
                    source_gpr,
                    signed_offset,
                }),
                _ => Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::UnsupportedRegimmSelector { selector },
                )),
            };
        }
        if opcode == RSP_SCALAR_BNE_OPCODE {
            return Ok(MachineRspDecodedInstruction::Bne {
                source_gpr_a: ((raw_word >> 21) & 0x1f) as u8,
                source_gpr_b: ((raw_word >> 16) & 0x1f) as u8,
                signed_offset: raw_word as u16 as i16,
            });
        }
        if opcode == RSP_SCALAR_ADDI_OPCODE {
            return Ok(MachineRspDecodedInstruction::Addi {
                source_gpr: ((raw_word >> 21) & 0x1f) as u8,
                destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                signed_immediate: raw_word as u16 as i16,
            });
        }
        if opcode == RSP_SCALAR_ORI_OPCODE {
            return Ok(MachineRspDecodedInstruction::Ori {
                source_gpr: ((raw_word >> 21) & 0x1f) as u8,
                destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                immediate: raw_word as u16,
            });
        }
        if opcode == RSP_SCALAR_XORI_OPCODE {
            return Ok(MachineRspDecodedInstruction::Xori {
                source_gpr: ((raw_word >> 21) & 0x1f) as u8,
                destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                immediate: raw_word as u16,
            });
        }
        if opcode == RSP_SCALAR_LUI_OPCODE {
            if (raw_word >> 21) & 0x1f != 0 {
                return Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::MalformedLuiEncoding,
                ));
            }
            return Ok(MachineRspDecodedInstruction::Lui {
                destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                immediate: raw_word as u16,
            });
        }
        if opcode == RSP_VECTOR_LOAD_OPCODE {
            let subopcode = ((raw_word >> 11) & 0x1f) as u8;
            if subopcode != RSP_VECTOR_LQV_SUBOPCODE {
                return Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::VectorLoadUnsupported { subopcode },
                ));
            }
            let raw_offset = (raw_word & 0x7f) as u8;
            let signed_offset = ((raw_offset << 1) as i8) >> 1;
            return Ok(MachineRspDecodedInstruction::Lqv {
                base_gpr: ((raw_word >> 21) & 0x1f) as u8,
                destination_vector: ((raw_word >> 16) & 0x1f) as u8,
                element: ((raw_word >> 7) & 0x0f) as u8,
                signed_offset,
            });
        }
        if opcode == RSP_VECTOR_COMPUTE_OPCODE && ((raw_word >> 21) & 0x10) != 0 {
            let destination_vector = ((raw_word >> 6) & 0x1f) as u8;
            let source_vector_a = ((raw_word >> 11) & 0x1f) as u8;
            let source_vector_b = ((raw_word >> 16) & 0x1f) as u8;
            let element = ((raw_word >> 21) & 0x0f) as u8;
            return match (raw_word & 0x3f) as u8 {
                RSP_VECTOR_VSUB_FUNCTION => Ok(MachineRspDecodedInstruction::Vsub {
                    destination_vector,
                    source_vector_a,
                    source_vector_b,
                    element,
                }),
                RSP_VECTOR_VADDC_FUNCTION => Ok(MachineRspDecodedInstruction::Vaddc {
                    destination_vector,
                    source_vector_a,
                    source_vector_b,
                    element,
                }),
                _ => Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::UnrepresentedInstruction {
                        class: MachineRspUnrepresentedInstructionClass::Vector,
                    },
                )),
            };
        }
        if opcode == 0x3a {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::VectorStoreUnsupported,
            ));
        }
        if opcode == RSP_SCALAR_LW_OPCODE {
            return Ok(MachineRspDecodedInstruction::Lw {
                base_gpr: ((raw_word >> 21) & 0x1f) as u8,
                destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                signed_offset: raw_word as u16 as i16,
            });
        }
        if (0x20..=0x26).contains(&opcode) {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ScalarLoadUnsupported { opcode },
            ));
        }
        if (0x28..=0x2e).contains(&opcode) {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ScalarStoreUnsupported { opcode },
            ));
        }
        if opcode == 0 && raw_word & 0x3f == 0 {
            if (raw_word >> 21) & 0x1f != 0 {
                return Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::MalformedSllEncoding,
                ));
            }
            return Ok(MachineRspDecodedInstruction::Sll {
                source_gpr: ((raw_word >> 16) & 0x1f) as u8,
                destination_gpr: ((raw_word >> 11) & 0x1f) as u8,
                shift_amount: ((raw_word >> 6) & 0x1f) as u8,
            });
        }
        let class = if opcode == RSP_VECTOR_COMPUTE_OPCODE {
            MachineRspUnrepresentedInstructionClass::Vector
        } else {
            MachineRspUnrepresentedInstructionClass::Scalar
        };
        Err(MachineRspStepRejection::new(
            MachineRspStepRejectionReason::UnrepresentedInstruction { class },
        ))
    }

    pub(crate) fn plan_mfc0(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        control_source: MachineRspMfc0ControlSource,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> MachineRspMfc0Plan {
        let MachineRspDecodedInstruction::Mfc0 {
            destination_gpr,
            control_register,
        } = decoded
        else {
            unreachable!("Mfc0 planner receives only decoded Mfc0")
        };
        debug_assert_eq!(control_register, control_source.register());
        MachineRspMfc0Plan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            destination_gpr,
            control_source,
            byte_provenance,
        }
    }

    pub(crate) fn plan_break(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
        status: MachineSpStatusState,
        mi_interrupts: MachineMiInterruptState,
    ) -> Result<MachineRspBreakPlan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Break = decoded else {
            unreachable!("Break planner receives only decoded Break")
        };
        if let Some(delay) = &self.delay_slot_context {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::BreakInDelaySlotUnsupported {
                    owner_pc: delay.owner_pc(),
                },
            ));
        }
        let prior_next_pc = self
            .next_pc
            .unwrap_or_else(|| sequential_local_pc(instruction_pc));
        let pre_break_mi_sp_pending = mi_interrupts.pending(MachineMiInterruptSource::Sp);
        Ok(MachineRspBreakPlan {
            source: MachineRspBreakSource {
                instruction_pc,
                prior_next_pc,
                instruction_provenance: byte_provenance,
                raw_word: RSP_SCALAR_BREAK_WORD,
                pre_break_status: status,
                pre_break_mi_sp_pending,
                pre_break_mi_sp_pending_source: mi_interrupts
                    .pending_set_provenance(MachineMiInterruptSource::Sp),
                interrupt_signaled: status.interrupt_on_break(),
            },
        })
    }

    pub(crate) fn apply_mfc0(&mut self, plan: MachineRspMfc0Plan) -> MachineRspStepOutcome {
        let result_value = plan.control_source.result_value();
        if plan.destination_gpr != 0 {
            self.scalar_registers[usize::from(plan.destination_gpr)] =
                MachineRspScalarRegisterState::Available {
                    value: result_value,
                    source: MachineRspScalarRegisterSource::Mfc0(MachineRspMfc0ResultSource {
                        instruction_pc: plan.instruction_pc,
                        control_source: plan.control_source,
                        byte_provenance: plan.byte_provenance,
                    }),
                };
        }
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Mfc0,
            destination: MachineRspLastInstructionDestination::ScalarMfc0 {
                destination_gpr: plan.destination_gpr,
                control_register: plan.control_source.register(),
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarMfc0Committed {
            instruction_pc: plan.instruction_pc,
            destination_gpr: plan.destination_gpr,
            control_register: plan.control_source.register(),
            result_value,
        }
    }

    pub(crate) fn plan_mtc0(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> Result<MachineRspMtc0Plan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Mtc0 {
            source_gpr,
            control_register,
        } = decoded
        else {
            unreachable!("Mtc0 planner receives only decoded Mtc0")
        };
        let MachineRspScalarRegisterState::Available {
            value: source_value,
            source,
        } = &self.scalar_registers[usize::from(source_gpr)]
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::Mtc0SourceUnavailable { source_gpr },
            ));
        };
        Ok(MachineRspMtc0Plan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            source_gpr,
            source_value: *source_value,
            source: source.clone(),
            control_register,
            byte_provenance,
            source_index: self.mtc0_sources.len(),
        })
    }

    pub(crate) fn apply_mtc0(&mut self, plan: MachineRspMtc0Plan) -> MachineRspStepOutcome {
        debug_assert_eq!(plan.source_index, self.mtc0_sources.len());
        self.mtc0_sources.push(MachineRspMtc0Source {
            instruction_pc: plan.instruction_pc,
            instruction_provenance: plan.byte_provenance,
            source_gpr: plan.source_gpr,
            source_value: plan.source_value,
            source: plan.source.clone(),
            control_register: plan.control_register,
        });
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Mtc0,
            destination: MachineRspLastInstructionDestination::ScalarMtc0 {
                source_gpr: plan.source_gpr,
                control_register: plan.control_register,
                source_index: plan.source_index,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarMtc0Committed {
            instruction_pc: plan.instruction_pc,
            source_gpr: plan.source_gpr,
            source_value: plan.source_value,
            control_register: plan.control_register,
            source_index: plan.source_index,
        }
    }

    pub(crate) fn plan_lui(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> MachineRspLuiPlan {
        let MachineRspDecodedInstruction::Lui {
            destination_gpr,
            immediate,
        } = decoded
        else {
            unreachable!("Lui planner receives only decoded Lui")
        };
        MachineRspLuiPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            destination_gpr,
            immediate,
            result_value: u32::from(immediate) << 16,
            byte_provenance,
        }
    }

    pub(crate) fn apply_lui(&mut self, plan: MachineRspLuiPlan) -> MachineRspStepOutcome {
        if plan.destination_gpr != 0 {
            self.scalar_registers[usize::from(plan.destination_gpr)] =
                MachineRspScalarRegisterState::Available {
                    value: plan.result_value,
                    source: MachineRspScalarRegisterSource::Lui(Box::new(MachineRspLuiSource {
                        instruction_pc: plan.instruction_pc,
                        instruction_provenance: plan.byte_provenance,
                        immediate: plan.immediate,
                    })),
                };
        }
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Lui,
            destination: MachineRspLastInstructionDestination::ScalarLui {
                destination_gpr: plan.destination_gpr,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarLuiCommitted {
            instruction_pc: plan.instruction_pc,
            destination_gpr: plan.destination_gpr,
            result_value: plan.result_value,
        }
    }

    pub(crate) fn plan_addi(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> Result<MachineRspAddiPlan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Addi {
            source_gpr,
            destination_gpr,
            signed_immediate,
        } = decoded
        else {
            unreachable!("Addi planner receives only decoded Addi")
        };
        let MachineRspScalarRegisterState::Available {
            value: source_value,
            source,
        } = &self.scalar_registers[usize::from(source_gpr)]
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::AddiSourceUnavailable { source_gpr },
            ));
        };
        Ok(MachineRspAddiPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            source_gpr,
            source_value: *source_value,
            source: source.clone(),
            destination_gpr,
            signed_immediate,
            result_value: source_value.wrapping_add_signed(i32::from(signed_immediate)),
            byte_provenance,
        })
    }

    pub(crate) fn apply_addi(&mut self, plan: MachineRspAddiPlan) -> MachineRspStepOutcome {
        if plan.destination_gpr != 0 {
            self.scalar_registers[usize::from(plan.destination_gpr)] =
                MachineRspScalarRegisterState::Available {
                    value: plan.result_value,
                    source: MachineRspScalarRegisterSource::Addi(Box::new(MachineRspAddiSource {
                        instruction_pc: plan.instruction_pc,
                        instruction_provenance: plan.byte_provenance,
                        source_gpr: plan.source_gpr,
                        source_value: plan.source_value,
                        source: plan.source.clone(),
                        signed_immediate: plan.signed_immediate,
                    })),
                };
        }
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Addi,
            destination: MachineRspLastInstructionDestination::ScalarAddi {
                destination_gpr: plan.destination_gpr,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarAddiCommitted {
            instruction_pc: plan.instruction_pc,
            destination_gpr: plan.destination_gpr,
            result_value: plan.result_value,
        }
    }

    pub(crate) fn plan_branch(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> Result<MachineRspBranchPlan, MachineRspStepRejection> {
        if let Some(delay) = &self.delay_slot_context {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ControlFlowInDelaySlot {
                    owner_pc: delay.owner_pc(),
                },
            ));
        }
        let delay_slot_pc = self
            .next_pc
            .unwrap_or_else(|| sequential_local_pc(instruction_pc));
        let source = match decoded {
            MachineRspDecodedInstruction::J { target_pc } => MachineRspBranchSource::J {
                instruction_pc,
                instruction_provenance: byte_provenance,
                delay_slot_pc,
                target_pc,
            },
            MachineRspDecodedInstruction::Bltz {
                source_gpr,
                signed_offset,
            } => {
                let MachineRspScalarRegisterState::Available {
                    value: source_value,
                    source,
                } = &self.scalar_registers[usize::from(source_gpr)]
                else {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::BltzSourceUnavailable { source_gpr },
                    ));
                };
                let target_pc = branch_target_local_pc(delay_slot_pc, signed_offset);
                MachineRspBranchSource::Bltz {
                    instruction_pc,
                    instruction_provenance: byte_provenance,
                    source_gpr,
                    source_value: *source_value,
                    source: source.clone(),
                    signed_offset,
                    delay_slot_pc,
                    target_pc,
                    taken: source_value & 0x8000_0000 != 0,
                }
            }
            MachineRspDecodedInstruction::Bgez {
                source_gpr,
                signed_offset,
            } => {
                let MachineRspScalarRegisterState::Available {
                    value: source_value,
                    source,
                } = &self.scalar_registers[usize::from(source_gpr)]
                else {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::BgezSourceUnavailable { source_gpr },
                    ));
                };
                let target_pc = branch_target_local_pc(delay_slot_pc, signed_offset);
                MachineRspBranchSource::Bgez {
                    instruction_pc,
                    instruction_provenance: byte_provenance,
                    source_gpr,
                    source_value: *source_value,
                    source: source.clone(),
                    signed_offset,
                    delay_slot_pc,
                    target_pc,
                    taken: source_value & 0x8000_0000 == 0,
                }
            }
            MachineRspDecodedInstruction::Bgezal {
                source_gpr,
                signed_offset,
            } => {
                let MachineRspScalarRegisterState::Available {
                    value: source_value,
                    source,
                } = &self.scalar_registers[usize::from(source_gpr)]
                else {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::BgezalSourceUnavailable { source_gpr },
                    ));
                };
                let target_pc = branch_target_local_pc(delay_slot_pc, signed_offset);
                MachineRspBranchSource::Bgezal {
                    instruction_pc,
                    instruction_provenance: byte_provenance,
                    source_gpr,
                    source_value: *source_value,
                    source: source.clone(),
                    signed_offset,
                    delay_slot_pc,
                    target_pc,
                    taken: source_value & 0x8000_0000 == 0,
                    link_value: sequential_local_pc(delay_slot_pc).into(),
                }
            }
            MachineRspDecodedInstruction::Bne {
                source_gpr_a,
                source_gpr_b,
                signed_offset,
            } => {
                let MachineRspScalarRegisterState::Available {
                    value: source_value_a,
                    source: source_a,
                } = &self.scalar_registers[usize::from(source_gpr_a)]
                else {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::BneSourceAUnavailable {
                            source_gpr: source_gpr_a,
                        },
                    ));
                };
                let MachineRspScalarRegisterState::Available {
                    value: source_value_b,
                    source: source_b,
                } = &self.scalar_registers[usize::from(source_gpr_b)]
                else {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::BneSourceBUnavailable {
                            source_gpr: source_gpr_b,
                        },
                    ));
                };
                let target_pc = branch_target_local_pc(delay_slot_pc, signed_offset);
                MachineRspBranchSource::Bne {
                    instruction_pc,
                    instruction_provenance: byte_provenance,
                    source_gpr_a,
                    source_value_a: *source_value_a,
                    source_a: source_a.clone(),
                    source_gpr_b,
                    source_value_b: *source_value_b,
                    source_b: source_b.clone(),
                    signed_offset,
                    delay_slot_pc,
                    target_pc,
                    taken: source_value_a != source_value_b,
                }
            }
            _ => unreachable!("branch planner receives only decoded J, Bltz, Bgez, Bgezal, or Bne"),
        };
        let selected_next_pc = if source.taken() {
            source.target_pc()
        } else {
            sequential_local_pc(delay_slot_pc)
        };
        Ok(MachineRspBranchPlan {
            source,
            selected_next_pc,
        })
    }

    pub(crate) fn apply_branch(&mut self, plan: MachineRspBranchPlan) -> MachineRspStepOutcome {
        let instruction_pc = plan.source.instruction_pc();
        let identity = plan.source.identity();
        let delay_slot_pc = plan.source.delay_slot_pc();
        let target_pc = plan.source.target_pc();
        let taken = plan.source.taken();
        let byte_provenance = plan.source.instruction_provenance();
        let bgezal_link = plan.source.bgezal_link();
        let link_value = bgezal_link.as_ref().map(|(value, _)| *value);
        self.next_pc = Some(plan.selected_next_pc);
        self.delay_slot_context = Some(MachineRspDelaySlotContext::new(plan.source));
        if let Some((link_value, link_source)) = bgezal_link {
            self.scalar_registers[31] = MachineRspScalarRegisterState::Available {
                value: link_value,
                source: MachineRspScalarRegisterSource::Bgezal(Box::new(link_source)),
            };
        }
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc,
            identity,
            destination: if link_value.is_some() {
                MachineRspLastInstructionDestination::BranchAndLink {
                    destination_gpr: 31,
                }
            } else {
                MachineRspLastInstructionDestination::Branch
            },
            byte_provenance,
        });
        match identity {
            MachineRspInstructionIdentity::J => MachineRspStepOutcome::ScalarJCommitted {
                instruction_pc,
                delay_slot_pc,
                target_pc,
            },
            MachineRspInstructionIdentity::Bltz => MachineRspStepOutcome::ScalarBltzCommitted {
                instruction_pc,
                delay_slot_pc,
                target_pc,
                taken,
            },
            MachineRspInstructionIdentity::Bgez => MachineRspStepOutcome::ScalarBgezCommitted {
                instruction_pc,
                delay_slot_pc,
                target_pc,
                taken,
            },
            MachineRspInstructionIdentity::Bgezal => MachineRspStepOutcome::ScalarBgezalCommitted {
                instruction_pc,
                delay_slot_pc,
                target_pc,
                taken,
                link_value,
            },
            MachineRspInstructionIdentity::Bne => MachineRspStepOutcome::ScalarBneCommitted {
                instruction_pc,
                delay_slot_pc,
                target_pc,
                taken,
            },
            _ => unreachable!("branch plan identity is exactly J, Bltz, Bgez, Bgezal, or Bne"),
        }
    }

    pub(crate) fn plan_xori(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> Result<MachineRspXoriPlan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Xori {
            source_gpr,
            destination_gpr,
            immediate,
        } = decoded
        else {
            unreachable!("Xori planner receives only decoded Xori")
        };
        let MachineRspScalarRegisterState::Available {
            value: source_value,
            source,
        } = &self.scalar_registers[usize::from(source_gpr)]
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::XoriSourceUnavailable { source_gpr },
            ));
        };
        Ok(MachineRspXoriPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            source_gpr,
            source_value: *source_value,
            source: source.clone(),
            destination_gpr,
            immediate,
            result_value: *source_value ^ u32::from(immediate),
            byte_provenance,
        })
    }

    pub(crate) fn plan_ori(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> Result<MachineRspOriPlan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Ori {
            source_gpr,
            destination_gpr,
            immediate,
        } = decoded
        else {
            unreachable!("Ori planner receives only decoded Ori")
        };
        let MachineRspScalarRegisterState::Available {
            value: source_value,
            source,
        } = &self.scalar_registers[usize::from(source_gpr)]
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::OriSourceUnavailable { source_gpr },
            ));
        };
        Ok(MachineRspOriPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            source_gpr,
            source_value: *source_value,
            source: source.clone(),
            destination_gpr,
            immediate,
            result_value: *source_value | u32::from(immediate),
            byte_provenance,
        })
    }

    pub(crate) fn plan_sll(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> Result<MachineRspSllPlan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Sll {
            source_gpr,
            destination_gpr,
            shift_amount,
        } = decoded
        else {
            unreachable!("Sll planner receives only decoded Sll")
        };
        let MachineRspScalarRegisterState::Available {
            value: source_value,
            source,
        } = &self.scalar_registers[usize::from(source_gpr)]
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::SllSourceUnavailable { source_gpr },
            ));
        };
        Ok(MachineRspSllPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            source_gpr,
            source_value: *source_value,
            source: source.clone(),
            destination_gpr,
            shift_amount,
            result_value: source_value.wrapping_shl(u32::from(shift_amount)),
            byte_provenance,
        })
    }

    pub(crate) fn apply_ori(&mut self, plan: MachineRspOriPlan) -> MachineRspStepOutcome {
        if plan.destination_gpr != 0 {
            self.scalar_registers[usize::from(plan.destination_gpr)] =
                MachineRspScalarRegisterState::Available {
                    value: plan.result_value,
                    source: MachineRspScalarRegisterSource::Ori(Box::new(MachineRspOriSource {
                        instruction_pc: plan.instruction_pc,
                        instruction_provenance: plan.byte_provenance,
                        source_gpr: plan.source_gpr,
                        source_value: plan.source_value,
                        source: plan.source.clone(),
                        immediate: plan.immediate,
                    })),
                };
        }
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Ori,
            destination: MachineRspLastInstructionDestination::ScalarOri {
                destination_gpr: plan.destination_gpr,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarOriCommitted {
            instruction_pc: plan.instruction_pc,
            destination_gpr: plan.destination_gpr,
            result_value: plan.result_value,
        }
    }

    pub(crate) fn apply_sll(&mut self, plan: MachineRspSllPlan) -> MachineRspStepOutcome {
        if plan.destination_gpr != 0 {
            self.scalar_registers[usize::from(plan.destination_gpr)] =
                MachineRspScalarRegisterState::Available {
                    value: plan.result_value,
                    source: MachineRspScalarRegisterSource::Sll(Box::new(MachineRspSllSource {
                        instruction_pc: plan.instruction_pc,
                        instruction_provenance: plan.byte_provenance,
                        source_gpr: plan.source_gpr,
                        source_value: plan.source_value,
                        source: plan.source.clone(),
                        shift_amount: plan.shift_amount,
                    })),
                };
        }
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Sll,
            destination: MachineRspLastInstructionDestination::ScalarSll {
                destination_gpr: plan.destination_gpr,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarSllCommitted {
            instruction_pc: plan.instruction_pc,
            destination_gpr: plan.destination_gpr,
            result_value: plan.result_value,
        }
    }

    pub(crate) fn apply_xori(&mut self, plan: MachineRspXoriPlan) -> MachineRspStepOutcome {
        if plan.destination_gpr != 0 {
            self.scalar_registers[usize::from(plan.destination_gpr)] =
                MachineRspScalarRegisterState::Available {
                    value: plan.result_value,
                    source: MachineRspScalarRegisterSource::Xori(Box::new(MachineRspXoriSource {
                        instruction_pc: plan.instruction_pc,
                        instruction_provenance: plan.byte_provenance,
                        source_gpr: plan.source_gpr,
                        source_value: plan.source_value,
                        source: plan.source.clone(),
                        immediate: plan.immediate,
                    })),
                };
        }
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Xori,
            destination: MachineRspLastInstructionDestination::ScalarXori {
                destination_gpr: plan.destination_gpr,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarXoriCommitted {
            instruction_pc: plan.instruction_pc,
            destination_gpr: plan.destination_gpr,
            result_value: plan.result_value,
        }
    }

    pub(crate) fn plan_lqv_address(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
    ) -> Result<MachineRspLqvAddressPlan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Lqv {
            base_gpr,
            destination_vector,
            element,
            signed_offset,
        } = decoded
        else {
            unreachable!("Lqv address planner receives only decoded Lqv")
        };
        let base_state = &self.scalar_registers[usize::from(base_gpr)];
        let MachineRspScalarRegisterState::Available {
            value: base_value,
            source: base_source,
        } = base_state
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::LqvScalarBaseUnavailable { base_gpr },
            ));
        };
        let base_value = *base_value;
        let base_source = base_source.clone();
        if element != 0 {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::LqvElementUnsupported { element },
            ));
        }
        let scaled_offset = i32::from(signed_offset) << 4;
        let local_dmem_address = ((base_value & u32::from(RSP_LOCAL_ADDRESS_MASK))
            .wrapping_add_signed(scaled_offset)) as u16
            & RSP_LOCAL_ADDRESS_MASK;
        if local_dmem_address & 0x0f != 0 {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::LqvAddressMisaligned { local_dmem_address },
            ));
        }
        Ok(MachineRspLqvAddressPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            destination_vector,
            element,
            signed_offset,
            base_gpr,
            base_value,
            base_source,
            local_dmem_address,
        })
    }

    pub(crate) fn plan_lqv(
        &self,
        address: MachineRspLqvAddressPlan,
        byte_provenance: [SpImemByteProvenance; 4],
        observations: [MachineSpDmemByteKnowledge; RSP_VECTOR_REGISTER_BYTE_COUNT],
    ) -> Result<MachineRspLqvPlan, MachineRspStepRejection> {
        let local_dmem_address = address.local_dmem_address;
        let mut descriptors = [MachineSpDmemByteKnowledgeDescriptor::new(
            SpDmemOffset::new(0),
            MachineSpDmemByteKnowledgeSource::Unavailable {
                source: crate::sp_dmem::MachineSpDmemUnavailableSource::ConstructionOrReset,
            },
        ); RSP_VECTOR_REGISTER_BYTE_COUNT];
        let mut bytes = [0_u8; RSP_VECTOR_REGISTER_BYTE_COUNT];
        let mut all_available = true;
        for index in 0..RSP_VECTOR_REGISTER_BYTE_COUNT {
            descriptors[index] = MachineSpDmemByteKnowledgeDescriptor::new(
                SpDmemOffset::new(u32::from(local_dmem_address) + index as u32),
                observations[index].source(),
            );
            match observations[index] {
                MachineSpDmemByteKnowledge::Available { value, .. } => bytes[index] = value,
                MachineSpDmemByteKnowledge::Unavailable { .. } => all_available = false,
            }
        }
        if descriptors.iter().enumerate().any(|(index, descriptor)| {
            descriptor.offset().value() != u32::from(local_dmem_address) + index as u32
        }) {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::LqvDmemKnowledgeMalformed { local_dmem_address },
            ));
        }
        let source = MachineRspLqvSource {
            instruction_pc: address.instruction_pc,
            instruction_provenance: byte_provenance,
            base_gpr: address.base_gpr,
            base_value: address.base_value,
            base_source: address.base_source.clone(),
            element: address.element,
            signed_offset: address.signed_offset,
            local_dmem_address,
            dmem_knowledge: descriptors,
        };
        let destination_state = if all_available {
            MachineRspVectorRegisterState::Available {
                bytes,
                source: MachineRspVectorRegisterSource::Lqv(Box::new(source)),
            }
        } else {
            MachineRspVectorRegisterState::Unavailable {
                source: MachineRspVectorUnavailableSource::Lqv(Box::new(source)),
            }
        };
        Ok(MachineRspLqvPlan {
            address,
            byte_provenance,
            destination_state,
        })
    }

    pub(crate) fn apply_lqv(&mut self, plan: MachineRspLqvPlan) -> MachineRspStepOutcome {
        let result_available = plan.destination_state.is_available();
        self.vector_unit.registers[usize::from(plan.address.destination_vector)] =
            plan.destination_state;
        self.next_pc = Some(sequential_local_pc(plan.address.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.address.instruction_pc,
            identity: MachineRspInstructionIdentity::Lqv,
            destination: MachineRspLastInstructionDestination::VectorLqv {
                destination_vector: plan.address.destination_vector,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::VectorLqvCommitted {
            instruction_pc: plan.address.instruction_pc,
            destination_vector: plan.address.destination_vector,
            local_dmem_address: plan.address.local_dmem_address,
            result_available,
        }
    }

    pub(crate) fn plan_vector_arithmetic(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> Result<MachineRspVectorArithmeticPlan, MachineRspStepRejection> {
        let (identity, destination_vector, source_vector_a, source_vector_b, element) =
            match decoded {
                MachineRspDecodedInstruction::Vsub {
                    destination_vector,
                    source_vector_a,
                    source_vector_b,
                    element,
                } => (
                    MachineRspInstructionIdentity::Vsub,
                    destination_vector,
                    source_vector_a,
                    source_vector_b,
                    element,
                ),
                MachineRspDecodedInstruction::Vaddc {
                    destination_vector,
                    source_vector_a,
                    source_vector_b,
                    element,
                } => (
                    MachineRspInstructionIdentity::Vaddc,
                    destination_vector,
                    source_vector_a,
                    source_vector_b,
                    element,
                ),
                _ => unreachable!("vector arithmetic planner receives only decoded Vsub or Vaddc"),
            };
        if element != 0 {
            return Err(MachineRspStepRejection::new(match identity {
                MachineRspInstructionIdentity::Vsub => {
                    MachineRspStepRejectionReason::VsubElementUnsupported { element }
                }
                MachineRspInstructionIdentity::Vaddc => {
                    MachineRspStepRejectionReason::VaddcElementUnsupported { element }
                }
                _ => unreachable!("vector arithmetic plan identity is Vsub or Vaddc"),
            }));
        }

        let source_a = self.vector_unit.registers[usize::from(source_vector_a)].clone();
        let source_b = if source_vector_a == source_vector_b {
            None
        } else {
            Some(self.vector_unit.registers[usize::from(source_vector_b)].clone())
        };
        let old_destination_state =
            self.vector_unit.registers[usize::from(destination_vector)].clone();
        let old_accumulator_and_flags = self.accumulator_and_flags.clone();

        let (available_bytes, available_accumulator_low, available_carry) = match identity {
            MachineRspInstructionIdentity::Vsub => {
                let borrow = old_accumulator_and_flags.vco.carry_or_borrow.value();
                let computed = borrow.and_then(|borrow| {
                    if source_vector_a == source_vector_b {
                        Some(compute_vsub_self_alias(borrow))
                    } else {
                        compute_vsub_available(&source_a, source_b.as_ref()?, borrow)
                    }
                });
                match computed {
                    Some((bytes, accumulator_low)) => (Some(bytes), Some(accumulator_low), None),
                    None => (None, None, None),
                }
            }
            MachineRspInstructionIdentity::Vaddc => {
                let computed = if source_vector_a == source_vector_b {
                    compute_vaddc_available(&source_a, &source_a)
                } else {
                    compute_vaddc_available(&source_a, source_b.as_ref().expect("non-alias source"))
                };
                match computed {
                    Some((bytes, accumulator_low, carry)) => {
                        (Some(bytes), Some(accumulator_low), Some(carry))
                    }
                    None => (None, None, None),
                }
            }
            _ => unreachable!("vector arithmetic plan identity is Vsub or Vaddc"),
        };
        let result_available = available_bytes.is_some();
        let arithmetic_source = Arc::new(MachineRspVectorArithmeticSource {
            instruction_pc,
            instruction_provenance: byte_provenance,
            identity,
            destination_vector,
            source_vector_a,
            source_a,
            source_vector_b,
            source_b,
            element,
            vsub_borrow_input: (identity == MachineRspInstructionIdentity::Vsub)
                .then(|| old_accumulator_and_flags.vco.carry_or_borrow.clone()),
            result_available,
        });

        let destination_state = match (identity, available_bytes) {
            (MachineRspInstructionIdentity::Vsub, Some(bytes)) => {
                MachineRspVectorRegisterState::Available {
                    bytes,
                    source: MachineRspVectorRegisterSource::Vsub(arithmetic_source.clone()),
                }
            }
            (MachineRspInstructionIdentity::Vsub, None) => {
                MachineRspVectorRegisterState::Unavailable {
                    source: MachineRspVectorUnavailableSource::Vsub(arithmetic_source.clone()),
                }
            }
            (MachineRspInstructionIdentity::Vaddc, Some(bytes)) => {
                MachineRspVectorRegisterState::Available {
                    bytes,
                    source: MachineRspVectorRegisterSource::Vaddc(arithmetic_source.clone()),
                }
            }
            (MachineRspInstructionIdentity::Vaddc, None) => {
                MachineRspVectorRegisterState::Unavailable {
                    source: MachineRspVectorUnavailableSource::Vaddc(arithmetic_source.clone()),
                }
            }
            _ => unreachable!("vector arithmetic plan identity is Vsub or Vaddc"),
        };

        let mut accumulator_and_flags = old_accumulator_and_flags.clone();
        for lane_index in 0..RSP_VECTOR_LANE_COUNT {
            accumulator_and_flags.accumulator.lanes[lane_index].low =
                match (identity, available_accumulator_low) {
                    (MachineRspInstructionIdentity::Vsub, Some(values)) => {
                        MachineRspAccumulatorSliceState::Available {
                            value: values[lane_index],
                            source: MachineRspAccumulatorSliceSource::Vsub(
                                arithmetic_source.clone(),
                            ),
                        }
                    }
                    (MachineRspInstructionIdentity::Vsub, None) => {
                        MachineRspAccumulatorSliceState::Unavailable {
                            source: MachineRspAccumulatorSliceUnavailableSource::Vsub(
                                arithmetic_source.clone(),
                            ),
                        }
                    }
                    (MachineRspInstructionIdentity::Vaddc, Some(values)) => {
                        MachineRspAccumulatorSliceState::Available {
                            value: values[lane_index],
                            source: MachineRspAccumulatorSliceSource::Vaddc(
                                arithmetic_source.clone(),
                            ),
                        }
                    }
                    (MachineRspInstructionIdentity::Vaddc, None) => {
                        MachineRspAccumulatorSliceState::Unavailable {
                            source: MachineRspAccumulatorSliceUnavailableSource::Vaddc(
                                arithmetic_source.clone(),
                            ),
                        }
                    }
                    _ => unreachable!("vector arithmetic plan identity is Vsub or Vaddc"),
                };
        }
        match identity {
            MachineRspInstructionIdentity::Vsub => {
                accumulator_and_flags.vco = MachineRspVcoState {
                    carry_or_borrow: MachineRspVcoHalfState::Available {
                        value: 0,
                        source: MachineRspVcoHalfSource::VsubClear(arithmetic_source.clone()),
                    },
                    not_equal: MachineRspVcoHalfState::Available {
                        value: 0,
                        source: MachineRspVcoHalfSource::VsubClear(arithmetic_source.clone()),
                    },
                };
            }
            MachineRspInstructionIdentity::Vaddc => {
                accumulator_and_flags.vco = MachineRspVcoState {
                    carry_or_borrow: match available_carry {
                        Some(value) => MachineRspVcoHalfState::Available {
                            value,
                            source: MachineRspVcoHalfSource::VaddcCarry(arithmetic_source.clone()),
                        },
                        None => MachineRspVcoHalfState::Unavailable {
                            source: MachineRspVcoHalfUnavailableSource::VaddcCarry(
                                arithmetic_source.clone(),
                            ),
                        },
                    },
                    not_equal: MachineRspVcoHalfState::Available {
                        value: 0,
                        source: MachineRspVcoHalfSource::VaddcNotEqualClear(
                            arithmetic_source.clone(),
                        ),
                    },
                };
            }
            _ => unreachable!("vector arithmetic plan identity is Vsub or Vaddc"),
        }

        Ok(MachineRspVectorArithmeticPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            identity,
            destination_vector,
            old_destination_state,
            byte_provenance,
            destination_state,
            old_accumulator_and_flags,
            accumulator_and_flags,
        })
    }

    pub(crate) fn apply_vector_arithmetic(
        &mut self,
        plan: MachineRspVectorArithmeticPlan,
    ) -> MachineRspStepOutcome {
        debug_assert_eq!(
            self.vector_unit.registers[usize::from(plan.destination_vector)],
            plan.old_destination_state
        );
        debug_assert_eq!(self.accumulator_and_flags, plan.old_accumulator_and_flags);
        let result_available = plan.destination_state.is_available();
        self.vector_unit.registers[usize::from(plan.destination_vector)] = plan.destination_state;
        self.accumulator_and_flags = plan.accumulator_and_flags;
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: plan.identity,
            destination: MachineRspLastInstructionDestination::VectorArithmetic {
                destination_vector: plan.destination_vector,
            },
            byte_provenance: plan.byte_provenance,
        });
        match plan.identity {
            MachineRspInstructionIdentity::Vsub => MachineRspStepOutcome::VectorVsubCommitted {
                instruction_pc: plan.instruction_pc,
                destination_vector: plan.destination_vector,
                result_available,
            },
            MachineRspInstructionIdentity::Vaddc => MachineRspStepOutcome::VectorVaddcCommitted {
                instruction_pc: plan.instruction_pc,
                destination_vector: plan.destination_vector,
                result_available,
            },
            _ => unreachable!("vector arithmetic plan identity is Vsub or Vaddc"),
        }
    }

    pub(crate) fn plan_lw_address(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
    ) -> Result<MachineRspScalarLwAddressPlan, MachineRspStepRejection> {
        let MachineRspDecodedInstruction::Lw {
            base_gpr,
            destination_gpr,
            signed_offset,
        } = decoded
        else {
            unreachable!("scalar Lw address planner receives only decoded Lw")
        };
        let MachineRspScalarRegisterState::Available {
            value: base_value,
            source: base_source,
        } = &self.scalar_registers[usize::from(base_gpr)]
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ScalarLwBaseUnavailable { base_gpr },
            ));
        };
        let local_dmem_address = base_value.wrapping_add_signed(i32::from(signed_offset)) as u16
            & RSP_LOCAL_ADDRESS_MASK;
        if local_dmem_address & RSP_INSTRUCTION_ALIGNMENT_MASK != 0 {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ScalarLwAddressMisaligned { local_dmem_address },
            ));
        }
        Ok(MachineRspScalarLwAddressPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            base_gpr,
            base_value: *base_value,
            base_source: base_source.clone(),
            destination_gpr,
            signed_offset,
            local_dmem_address,
        })
    }

    pub(crate) fn plan_lw(
        &self,
        address: MachineRspScalarLwAddressPlan,
        byte_provenance: [SpImemByteProvenance; RSP_SCALAR_LW_BYTE_COUNT],
        observations: [MachineRspScalarLwDmemObservation; RSP_SCALAR_LW_BYTE_COUNT],
    ) -> Result<MachineRspScalarLwPlan, MachineRspStepRejection> {
        let local_dmem_address = address.local_dmem_address;
        let mut descriptors = [observations[0].descriptor; RSP_SCALAR_LW_BYTE_COUNT];
        let mut bytes = [0_u8; RSP_SCALAR_LW_BYTE_COUNT];
        for (index, observation) in observations.into_iter().enumerate() {
            let offset = u32::from(local_dmem_address) + index as u32;
            if observation.descriptor.offset().value() != offset {
                return Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::ScalarLwDmemKnowledgeMalformed {
                        local_dmem_address,
                    },
                ));
            }
            descriptors[index] = observation.descriptor;
            match (observation.descriptor.is_available(), observation.value) {
                (true, Some(value)) => bytes[index] = value,
                (false, None) => {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::ScalarLwDmemByteUnavailable {
                            local_dmem_address,
                            first_unavailable_offset: offset as u16,
                        },
                    ));
                }
                _ => {
                    return Err(MachineRspStepRejection::new(
                        MachineRspStepRejectionReason::ScalarLwDmemKnowledgeMalformed {
                            local_dmem_address,
                        },
                    ));
                }
            }
        }
        Ok(MachineRspScalarLwPlan {
            address,
            byte_provenance,
            dmem_knowledge: descriptors,
            result_value: u32::from_be_bytes(bytes),
        })
    }

    pub(crate) fn apply_lw(&mut self, plan: MachineRspScalarLwPlan) -> MachineRspStepOutcome {
        let source = MachineRspScalarLwSource {
            instruction_pc: plan.address.instruction_pc,
            instruction_provenance: plan.byte_provenance,
            base_gpr: plan.address.base_gpr,
            base_value: plan.address.base_value,
            base_source: plan.address.base_source.clone(),
            signed_offset: plan.address.signed_offset,
            local_dmem_address: plan.address.local_dmem_address,
            dmem_knowledge: plan.dmem_knowledge,
        };
        if plan.address.destination_gpr != 0 {
            self.scalar_registers[usize::from(plan.address.destination_gpr)] =
                MachineRspScalarRegisterState::Available {
                    value: plan.result_value,
                    source: MachineRspScalarRegisterSource::Lw(Box::new(source)),
                };
        }
        self.next_pc = Some(sequential_local_pc(plan.address.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.address.instruction_pc,
            identity: MachineRspInstructionIdentity::Lw,
            destination: MachineRspLastInstructionDestination::ScalarLw {
                destination_gpr: plan.address.destination_gpr,
            },
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarLwCommitted {
            instruction_pc: plan.address.instruction_pc,
            destination_gpr: plan.address.destination_gpr,
            local_dmem_address: plan.address.local_dmem_address,
            result_value: plan.result_value,
        }
    }

    pub(crate) fn plan_nop(
        &self,
        instruction_pc: u16,
        decoded: MachineRspDecodedInstruction,
        byte_provenance: [SpImemByteProvenance; 4],
    ) -> MachineRspNopPlan {
        let MachineRspDecodedInstruction::Nop = decoded else {
            unreachable!("Nop planner receives only decoded Nop")
        };
        MachineRspNopPlan {
            instruction_pc,
            old_next_pc: self
                .next_pc
                .unwrap_or_else(|| sequential_local_pc(instruction_pc)),
            byte_provenance,
        }
    }

    pub(crate) fn apply_nop(&mut self, plan: MachineRspNopPlan) -> MachineRspStepOutcome {
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc,
            identity: MachineRspInstructionIdentity::Nop,
            destination: MachineRspLastInstructionDestination::None,
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::NopCommitted {
            instruction_pc: plan.instruction_pc,
        }
    }

    pub(crate) fn apply_break(&mut self, plan: MachineRspBreakPlan) -> MachineRspStepOutcome {
        let source = plan.source();
        self.next_pc = Some(sequential_local_pc(plan.old_next_pc()));
        self.delay_slot_context = None;
        self.committed_instruction_count = self.committed_instruction_count.wrapping_add(1);
        self.last_instruction = Some(MachineRspLastInstructionState {
            instruction_pc: plan.instruction_pc(),
            identity: MachineRspInstructionIdentity::Break,
            destination: MachineRspLastInstructionDestination::None,
            byte_provenance: source.instruction_provenance(),
        });
        MachineRspStepOutcome::BreakCommitted {
            instruction_pc: plan.instruction_pc(),
            interrupt_on_break: source.interrupt_on_break(),
            interrupt_signaled: source.interrupt_signaled(),
        }
    }

    #[cfg(test)]
    pub(crate) fn stage_delay_for_test(&mut self, owner_pc: u16) {
        let delay_slot_pc = sequential_local_pc(owner_pc);
        self.delay_slot_context = Some(MachineRspDelaySlotContext::new(
            MachineRspBranchSource::Bltz {
                instruction_pc: owner_pc,
                instruction_provenance: [SpImemByteProvenance::GeneratedMachineTestStaging; 4],
                source_gpr: 0,
                source_value: 0,
                source: MachineRspScalarRegisterSource::ArchitecturalZero,
                signed_offset: 0,
                delay_slot_pc,
                target_pc: sequential_local_pc(delay_slot_pc),
                taken: false,
            },
        ));
    }
}

impl Default for MachineRspExecutionState {
    fn default() -> Self {
        let unavailable = MachineRspScalarRegisterState::Unavailable {
            source: MachineRspUnavailableSource::ConstructionOrReset,
        };
        let mut scalar_registers = core::array::from_fn(|_| unavailable.clone());
        scalar_registers[0] = MachineRspScalarRegisterState::Available {
            value: 0,
            source: MachineRspScalarRegisterSource::ArchitecturalZero,
        };
        Self {
            scalar_registers,
            next_pc: None,
            delay_slot_context: None,
            committed_instruction_count: 0,
            last_instruction: None,
            vector_unit: MachineRspVectorUnitState {
                registers: core::array::from_fn(|_| MachineRspVectorRegisterState::Unavailable {
                    source: MachineRspVectorUnavailableSource::ConstructionOrReset,
                }),
            },
            accumulator_and_flags: MachineRspAccumulatorAndFlagsState {
                accumulator: MachineRspAccumulatorState {
                    lanes: core::array::from_fn(|_| MachineRspAccumulatorLaneState {
                        high: MachineRspAccumulatorSliceState::Unavailable {
                            source:
                                MachineRspAccumulatorSliceUnavailableSource::ConstructionOrReset,
                        },
                        middle: MachineRspAccumulatorSliceState::Unavailable {
                            source:
                                MachineRspAccumulatorSliceUnavailableSource::ConstructionOrReset,
                        },
                        low: MachineRspAccumulatorSliceState::Unavailable {
                            source:
                                MachineRspAccumulatorSliceUnavailableSource::ConstructionOrReset,
                        },
                    }),
                },
                vco: MachineRspVcoState {
                    carry_or_borrow: MachineRspVcoHalfState::Unavailable {
                        source: MachineRspVcoHalfUnavailableSource::ConstructionOrReset,
                    },
                    not_equal: MachineRspVcoHalfState::Unavailable {
                        source: MachineRspVcoHalfUnavailableSource::ConstructionOrReset,
                    },
                },
                vcc: MachineRspVccState::Unavailable {
                    source: MachineRspVccSource::ConstructionOrReset,
                },
                vce: MachineRspVceState::Unavailable {
                    source: MachineRspVceSource::ConstructionOrReset,
                },
            },
            mtc0_sources: Vec::new(),
        }
    }
}

fn compute_vsub_self_alias(borrow: u8) -> ([u8; RSP_VECTOR_REGISTER_BYTE_COUNT], [u16; 8]) {
    let mut bytes = [0_u8; RSP_VECTOR_REGISTER_BYTE_COUNT];
    let mut accumulator_low = [0_u16; RSP_VECTOR_LANE_COUNT];
    for (lane_index, accumulator_low) in accumulator_low.iter_mut().enumerate() {
        let result = -i32::from((borrow >> lane_index) & 1);
        *accumulator_low = result as u16;
        let lane_bytes = (result as i16).to_be_bytes();
        bytes[lane_index * 2] = lane_bytes[0];
        bytes[lane_index * 2 + 1] = lane_bytes[1];
    }
    (bytes, accumulator_low)
}

fn compute_vsub_available(
    source_a: &MachineRspVectorRegisterState,
    source_b: &MachineRspVectorRegisterState,
    borrow: u8,
) -> Option<([u8; RSP_VECTOR_REGISTER_BYTE_COUNT], [u16; 8])> {
    let source_a = source_a.bytes()?;
    let source_b = source_b.bytes()?;
    let mut bytes = [0_u8; RSP_VECTOR_REGISTER_BYTE_COUNT];
    let mut accumulator_low = [0_u16; RSP_VECTOR_LANE_COUNT];
    for (lane_index, accumulator_low) in accumulator_low.iter_mut().enumerate() {
        let byte_index = lane_index * 2;
        let lane_a = i32::from(i16::from_be_bytes([
            source_a[byte_index],
            source_a[byte_index + 1],
        ]));
        let lane_b = i32::from(i16::from_be_bytes([
            source_b[byte_index],
            source_b[byte_index + 1],
        ]));
        let wide_result = lane_a - lane_b - i32::from((borrow >> lane_index) & 1);
        *accumulator_low = wide_result as u16;
        let clamped = wide_result.clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16;
        let lane_bytes = clamped.to_be_bytes();
        bytes[byte_index] = lane_bytes[0];
        bytes[byte_index + 1] = lane_bytes[1];
    }
    Some((bytes, accumulator_low))
}

fn compute_vaddc_available(
    source_a: &MachineRspVectorRegisterState,
    source_b: &MachineRspVectorRegisterState,
) -> Option<([u8; RSP_VECTOR_REGISTER_BYTE_COUNT], [u16; 8], u8)> {
    let source_a = source_a.bytes()?;
    let source_b = source_b.bytes()?;
    let mut bytes = [0_u8; RSP_VECTOR_REGISTER_BYTE_COUNT];
    let mut accumulator_low = [0_u16; RSP_VECTOR_LANE_COUNT];
    let mut carry = 0_u8;
    for (lane_index, accumulator_low) in accumulator_low.iter_mut().enumerate() {
        let byte_index = lane_index * 2;
        let lane_a = u32::from(u16::from_be_bytes([
            source_a[byte_index],
            source_a[byte_index + 1],
        ]));
        let lane_b = u32::from(u16::from_be_bytes([
            source_b[byte_index],
            source_b[byte_index + 1],
        ]));
        let sum = lane_a + lane_b;
        let low = sum as u16;
        *accumulator_low = low;
        carry |= ((sum >> 16) as u8) << lane_index;
        let lane_bytes = low.to_be_bytes();
        bytes[byte_index] = lane_bytes[0];
        bytes[byte_index + 1] = lane_bytes[1];
    }
    Some((bytes, accumulator_low, carry))
}

pub(crate) const fn sequential_local_pc(pc: u16) -> u16 {
    pc.wrapping_add(4) & RSP_LOCAL_ADDRESS_MASK & !RSP_INSTRUCTION_ALIGNMENT_MASK
}

pub(crate) fn branch_target_local_pc(delay_slot_pc: u16, signed_offset: i16) -> u16 {
    ((delay_slot_pc as u32).wrapping_add_signed((signed_offset as i32) << 2) as u16)
        & RSP_LOCAL_ADDRESS_MASK
        & !RSP_INSTRUCTION_ALIGNMENT_MASK
}

pub(crate) const fn jump_target_local_pc(encoded_target: u32) -> u16 {
    ((encoded_target << 2) as u16) & RSP_LOCAL_ADDRESS_MASK & !RSP_INSTRUCTION_ALIGNMENT_MASK
}

pub(crate) fn classify_instruction_source(
    provenance: [SpImemByteProvenance; 4],
) -> MachineRspInstructionSource {
    let first = provenance[0];
    if provenance.iter().all(|candidate| *candidate == first) {
        return match first {
            SpImemByteProvenance::UserSuppliedPifFirmware { .. } => {
                MachineRspInstructionSource::UserSuppliedPifFirmware
            }
            SpImemByteProvenance::PublicSyntheticColdX105Bootstrap { .. } => {
                MachineRspInstructionSource::PublicSyntheticColdX105Bootstrap
            }
            SpImemByteProvenance::CpuStoreWord { .. } => MachineRspInstructionSource::CpuStoreWord,
            SpImemByteProvenance::CpuStoreByte { .. } => MachineRspInstructionSource::CpuStoreByte,
            SpImemByteProvenance::SpDma { record_index } => {
                MachineRspInstructionSource::SpDma { record_index }
            }
            SpImemByteProvenance::Unknown | SpImemByteProvenance::OpaqueCpuStoreWord { .. } => {
                unreachable!("committed RSP instruction provenance must be known")
            }
            #[cfg(test)]
            SpImemByteProvenance::GeneratedMachineTestStaging => {
                MachineRspInstructionSource::GeneratedMachineTestStaging
            }
        };
    }
    MachineRspInstructionSource::MixedKnown
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INSTRUCTION_PROVENANCE: [SpImemByteProvenance; 4] =
        [SpImemByteProvenance::GeneratedMachineTestStaging; 4];

    const fn lqv_word(base_gpr: u8, destination_vector: u8, element: u8, signed_offset: i8) -> u32 {
        ((RSP_VECTOR_LOAD_OPCODE as u32) << 26)
            | ((base_gpr as u32) << 21)
            | ((destination_vector as u32) << 16)
            | ((RSP_VECTOR_LQV_SUBOPCODE as u32) << 11)
            | ((element as u32) << 7)
            | ((signed_offset as u8 & 0x7f) as u32)
    }

    const fn lw_word(base_gpr: u8, destination_gpr: u8, signed_offset: i16) -> u32 {
        ((RSP_SCALAR_LW_OPCODE as u32) << 26)
            | ((base_gpr as u32) << 21)
            | ((destination_gpr as u32) << 16)
            | signed_offset as u16 as u32
    }

    const fn mtc0_word(source_gpr: u8, control_index: u8) -> u32 {
        ((RSP_COP0_OPCODE as u32) << 26)
            | ((RSP_COP0_MTC0_TRANSFER_SELECTOR as u32) << 21)
            | ((source_gpr as u32) << 16)
            | ((control_index as u32) << 11)
    }

    const fn mfc0_word(destination_gpr: u8, control_index: u8) -> u32 {
        ((RSP_COP0_OPCODE as u32) << 26)
            | ((RSP_COP0_MFC0_TRANSFER_SELECTOR as u32) << 21)
            | ((destination_gpr as u32) << 16)
            | ((control_index as u32) << 11)
    }

    const fn lui_word(destination_gpr: u8, immediate: u16) -> u32 {
        ((RSP_SCALAR_LUI_OPCODE as u32) << 26) | ((destination_gpr as u32) << 16) | immediate as u32
    }

    const fn addi_word(source_gpr: u8, destination_gpr: u8, signed_immediate: i16) -> u32 {
        ((RSP_SCALAR_ADDI_OPCODE as u32) << 26)
            | ((source_gpr as u32) << 21)
            | ((destination_gpr as u32) << 16)
            | signed_immediate as u16 as u32
    }

    const fn j_word(target_pc: u16) -> u32 {
        ((RSP_SCALAR_J_OPCODE as u32) << 26) | ((target_pc as u32) >> 2)
    }

    const fn bltz_word(source_gpr: u8, signed_offset: i16) -> u32 {
        ((RSP_SCALAR_REGIMM_OPCODE as u32) << 26)
            | ((source_gpr as u32) << 21)
            | ((RSP_SCALAR_BLTZ_SELECTOR as u32) << 16)
            | signed_offset as u16 as u32
    }

    const fn bgez_word(source_gpr: u8, signed_offset: i16) -> u32 {
        ((RSP_SCALAR_REGIMM_OPCODE as u32) << 26)
            | ((source_gpr as u32) << 21)
            | ((RSP_SCALAR_BGEZ_SELECTOR as u32) << 16)
            | signed_offset as u16 as u32
    }

    const fn bgezal_word(source_gpr: u8, signed_offset: i16) -> u32 {
        ((RSP_SCALAR_REGIMM_OPCODE as u32) << 26)
            | ((source_gpr as u32) << 21)
            | ((RSP_SCALAR_BGEZAL_SELECTOR as u32) << 16)
            | signed_offset as u16 as u32
    }

    const fn bne_word(source_gpr_a: u8, source_gpr_b: u8, signed_offset: i16) -> u32 {
        ((RSP_SCALAR_BNE_OPCODE as u32) << 26)
            | ((source_gpr_a as u32) << 21)
            | ((source_gpr_b as u32) << 16)
            | signed_offset as u16 as u32
    }

    const fn xori_word(source_gpr: u8, destination_gpr: u8, immediate: u16) -> u32 {
        ((RSP_SCALAR_XORI_OPCODE as u32) << 26)
            | ((source_gpr as u32) << 21)
            | ((destination_gpr as u32) << 16)
            | immediate as u32
    }

    const fn ori_word(source_gpr: u8, destination_gpr: u8, immediate: u16) -> u32 {
        ((RSP_SCALAR_ORI_OPCODE as u32) << 26)
            | ((source_gpr as u32) << 21)
            | ((destination_gpr as u32) << 16)
            | immediate as u32
    }

    const fn sll_word(source_gpr: u8, destination_gpr: u8, shift_amount: u8) -> u32 {
        ((source_gpr as u32) << 16)
            | ((destination_gpr as u32) << 11)
            | (((shift_amount & 0x1f) as u32) << 6)
    }

    const fn vector_compute_word(
        function: u8,
        destination_vector: u8,
        source_vector_a: u8,
        source_vector_b: u8,
        element: u8,
    ) -> u32 {
        ((RSP_VECTOR_COMPUTE_OPCODE as u32) << 26)
            | (1 << 25)
            | ((element as u32) << 21)
            | ((source_vector_b as u32) << 16)
            | ((source_vector_a as u32) << 11)
            | ((destination_vector as u32) << 6)
            | function as u32
    }

    fn stage_available_scalar(rsp: &mut MachineRspExecutionState, index: usize, value: u32) {
        rsp.scalar_registers[index] = MachineRspScalarRegisterState::Available {
            value,
            source: MachineRspScalarRegisterSource::Mfc0(MachineRspMfc0ResultSource {
                instruction_pc: 0x0f0,
                control_source: MachineRspMfc0ControlSource::SpDramAddress {
                    value,
                    source: MachineSpDramAddressSource::SourceDefinedReset,
                },
                byte_provenance: TEST_INSTRUCTION_PROVENANCE,
            }),
        };
    }

    fn stage_available_vector(
        rsp: &mut MachineRspExecutionState,
        index: usize,
        bytes: [u8; RSP_VECTOR_REGISTER_BYTE_COUNT],
    ) {
        let dmem_knowledge = core::array::from_fn(|byte_index| {
            MachineSpDmemByteKnowledgeDescriptor::new(
                SpDmemOffset::new(byte_index as u32),
                MachineSpDmemByteKnowledgeSource::Available {
                    source: crate::sp_dmem::MachineSpDmemByteSource::GeneratedMachineTestStaging,
                },
            )
        });
        rsp.vector_unit.registers[index] = MachineRspVectorRegisterState::Available {
            bytes,
            source: MachineRspVectorRegisterSource::Lqv(Box::new(MachineRspLqvSource {
                instruction_pc: 0,
                instruction_provenance: TEST_INSTRUCTION_PROVENANCE,
                base_gpr: 0,
                base_value: 0,
                base_source: MachineRspScalarRegisterSource::ArchitecturalZero,
                element: 0,
                signed_offset: 0,
                local_dmem_address: 0,
                dmem_knowledge,
            })),
        };
    }

    fn vector_bytes_from_lanes(lanes: [u16; RSP_VECTOR_LANE_COUNT]) -> [u8; 16] {
        let mut bytes = [0_u8; 16];
        for (lane_index, lane) in lanes.into_iter().enumerate() {
            let lane_bytes = lane.to_be_bytes();
            bytes[lane_index * 2] = lane_bytes[0];
            bytes[lane_index * 2 + 1] = lane_bytes[1];
        }
        bytes
    }

    fn stage_available_vector_control(rsp: &mut MachineRspExecutionState, carry_or_borrow: u8) {
        for lane_index in 0..RSP_VECTOR_LANE_COUNT {
            let lane = &mut rsp.accumulator_and_flags.accumulator.lanes[lane_index];
            lane.high = MachineRspAccumulatorSliceState::Available {
                value: 0x1000 + lane_index as u16,
                source: MachineRspAccumulatorSliceSource::GeneratedMachineTestStaging,
            };
            lane.middle = MachineRspAccumulatorSliceState::Available {
                value: 0x2000 + lane_index as u16,
                source: MachineRspAccumulatorSliceSource::GeneratedMachineTestStaging,
            };
        }
        rsp.accumulator_and_flags.vco = MachineRspVcoState {
            carry_or_borrow: MachineRspVcoHalfState::Available {
                value: carry_or_borrow,
                source: MachineRspVcoHalfSource::GeneratedMachineTestStaging,
            },
            not_equal: MachineRspVcoHalfState::Available {
                value: 0xa5,
                source: MachineRspVcoHalfSource::GeneratedMachineTestStaging,
            },
        };
        rsp.accumulator_and_flags.vcc = MachineRspVccState::Available {
            value: 0x5aa5,
            source: MachineRspVccSource::GeneratedMachineTestStaging,
        };
        rsp.accumulator_and_flags.vce = MachineRspVceState::Available {
            value: 0x5a,
            source: MachineRspVceSource::GeneratedMachineTestStaging,
        };
    }

    fn available_dmem_observations(
        bytes: [u8; RSP_VECTOR_REGISTER_BYTE_COUNT],
    ) -> [MachineSpDmemByteKnowledge; RSP_VECTOR_REGISTER_BYTE_COUNT] {
        bytes.map(|value| MachineSpDmemByteKnowledge::Available {
            value,
            source: crate::sp_dmem::MachineSpDmemByteSource::GeneratedMachineTestStaging,
        })
    }

    fn unavailable_dmem_observations(
    ) -> [MachineSpDmemByteKnowledge; RSP_VECTOR_REGISTER_BYTE_COUNT] {
        [MachineSpDmemByteKnowledge::Unavailable {
            source: crate::sp_dmem::MachineSpDmemUnavailableSource::ConstructionOrReset,
        }; RSP_VECTOR_REGISTER_BYTE_COUNT]
    }

    fn available_lw_observations(
        start: u16,
        bytes: [u8; RSP_SCALAR_LW_BYTE_COUNT],
    ) -> [MachineRspScalarLwDmemObservation; RSP_SCALAR_LW_BYTE_COUNT] {
        core::array::from_fn(|index| {
            MachineRspScalarLwDmemObservation::from_knowledge(
                SpDmemOffset::new(u32::from(start) + index as u32),
                MachineSpDmemByteKnowledge::Available {
                    value: bytes[index],
                    source: crate::sp_dmem::MachineSpDmemByteSource::GeneratedMachineTestStaging,
                },
            )
        })
    }

    #[test]
    fn rsp_foundation_rsp_vector_register_slots_start_with_only_scalar_zero_available() {
        let rsp = MachineRspExecutionState::default();
        assert_eq!(
            rsp.scalar_register(0),
            Some(MachineRspScalarRegisterState::Available {
                value: 0,
                source: MachineRspScalarRegisterSource::ArchitecturalZero,
            })
        );
        for index in 1..RSP_SCALAR_REGISTER_COUNT {
            assert_eq!(
                rsp.scalar_register(index),
                Some(MachineRspScalarRegisterState::Unavailable {
                    source: MachineRspUnavailableSource::ConstructionOrReset,
                })
            );
        }
        assert_eq!(rsp.next_pc(), None);
        assert_eq!(rsp.delay_slot_context(), None);
        assert_eq!(rsp.committed_instruction_count(), 0);
        assert_eq!(rsp.last_instruction(), None);
        assert_eq!(
            rsp.vector_unit().register_count(),
            RSP_VECTOR_REGISTER_COUNT
        );
        for index in 0..RSP_VECTOR_REGISTER_COUNT {
            assert!(matches!(
                rsp.vector_register(index),
                Some(MachineRspVectorRegisterState::Unavailable {
                    source: MachineRspVectorUnavailableSource::ConstructionOrReset,
                })
            ));
        }
        let accumulator_and_flags = rsp.accumulator_and_flags();
        assert_eq!(
            accumulator_and_flags.accumulator().lane_count(),
            RSP_VECTOR_LANE_COUNT
        );
        for lane_index in 0..RSP_VECTOR_LANE_COUNT {
            let lane = accumulator_and_flags
                .accumulator()
                .lane(lane_index)
                .unwrap();
            for slice in [lane.high(), lane.middle(), lane.low()] {
                assert_eq!(
                    slice.unavailable_source(),
                    Some(&MachineRspAccumulatorSliceUnavailableSource::ConstructionOrReset)
                );
            }
        }
        assert_eq!(
            accumulator_and_flags
                .vco()
                .carry_or_borrow()
                .unavailable_source(),
            Some(&MachineRspVcoHalfUnavailableSource::ConstructionOrReset)
        );
        assert_eq!(
            accumulator_and_flags.vco().not_equal().unavailable_source(),
            Some(&MachineRspVcoHalfUnavailableSource::ConstructionOrReset)
        );
        assert_eq!(
            accumulator_and_flags.vcc(),
            MachineRspVccState::Unavailable {
                source: MachineRspVccSource::ConstructionOrReset,
            }
        );
        assert_eq!(
            accumulator_and_flags.vce(),
            MachineRspVceState::Unavailable {
                source: MachineRspVceSource::ConstructionOrReset,
            }
        );
    }

    #[test]
    fn clean_room_ntsc_x105_post_boot_stages_only_public_gpr_11_truth() {
        let mut expected = MachineRspExecutionState::default();
        expected.scalar_registers[RSP_NTSC_X105_POST_BOOT_GPR_11_INDEX] =
            MachineRspScalarRegisterState::Available {
                value: RSP_NTSC_X105_POST_BOOT_GPR_11_VALUE,
                source: MachineRspScalarRegisterSource::CleanRoomHleNtscX105PinnedPostBoot,
            };
        let staged = MachineRspExecutionState::clean_room_ntsc_x105_post_boot();
        assert_eq!(staged, expected);
        assert_eq!(staged.committed_instruction_count(), 0);
        assert_eq!(staged.next_pc(), None);
        assert_eq!(staged.delay_slot_context(), None);
        assert_eq!(staged.last_instruction(), None);
    }

    #[test]
    fn rsp_mfc0_mtc0_xori_lqv_scalar_lw_and_raw_zero_nop_decode_boundary_is_exact() {
        let rsp = MachineRspExecutionState::default();
        assert_eq!(
            rsp.decode(0x4008_3800),
            Ok(MachineRspDecodedInstruction::Mfc0 {
                destination_gpr: 8,
                control_register: MachineRspControlRegister::SpSemaphore,
            })
        );
        assert_eq!(
            rsp.decode(0x400b_0800),
            Ok(MachineRspDecodedInstruction::Mfc0 {
                destination_gpr: 11,
                control_register: MachineRspControlRegister::SpDramAddress,
            })
        );
        assert_eq!(
            rsp.decode(0xc80c_2000),
            Ok(MachineRspDecodedInstruction::Lqv {
                base_gpr: 0,
                destination_vector: 12,
                element: 0,
                signed_offset: 0,
            })
        );
        assert_eq!(
            rsp.decode(0x8c04_0040),
            Ok(MachineRspDecodedInstruction::Lw {
                base_gpr: 0,
                destination_gpr: 4,
                signed_offset: 0x40,
            })
        );
        assert_eq!(
            rsp.decode(0x4080_0000),
            Ok(MachineRspDecodedInstruction::Mtc0 {
                source_gpr: 0,
                control_register: MachineRspControlRegister::SpMemoryAddress,
            })
        );
        assert_eq!(
            rsp.decode(0x4083_0800),
            Ok(MachineRspDecodedInstruction::Mtc0 {
                source_gpr: 3,
                control_register: MachineRspControlRegister::SpDramAddress,
            })
        );
        assert_eq!(
            rsp.decode(0x4080_1000),
            Ok(MachineRspDecodedInstruction::Mtc0 {
                source_gpr: 0,
                control_register: MachineRspControlRegister::SpReadLength,
            })
        );
        assert_eq!(
            rsp.decode(0x4083_1800),
            Ok(MachineRspDecodedInstruction::Mtc0 {
                source_gpr: 3,
                control_register: MachineRspControlRegister::SpWriteLength,
            })
        );
        assert_eq!(
            rsp.decode(mtc0_word(3, RSP_COP0_SP_STATUS_INDEX)),
            Ok(MachineRspDecodedInstruction::Mtc0 {
                source_gpr: 3,
                control_register: MachineRspControlRegister::SpStatus,
            })
        );
        assert_eq!(
            rsp.decode(0x4083_5800),
            Ok(MachineRspDecodedInstruction::Mtc0 {
                source_gpr: 3,
                control_register: MachineRspControlRegister::DpcStatus,
            })
        );
        assert_eq!(
            rsp.decode(0x3803_0180),
            Ok(MachineRspDecodedInstruction::Xori {
                source_gpr: 0,
                destination_gpr: 3,
                immediate: 0x0180,
            })
        );
        assert_eq!(
            rsp.decode(ori_word(3, 4, 0x8180)),
            Ok(MachineRspDecodedInstruction::Ori {
                source_gpr: 3,
                destination_gpr: 4,
                immediate: 0x8180,
            })
        );
        assert_eq!(rsp.decode(0), Ok(MachineRspDecodedInstruction::Nop));
        assert_eq!(
            rsp.decode(0x0000_0040),
            Ok(MachineRspDecodedInstruction::Sll {
                source_gpr: 0,
                destination_gpr: 0,
                shift_amount: 1,
            })
        );
        assert_eq!(
            rsp.decode(sll_word(3, 4, 17) | (1 << 21))
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::MalformedSllEncoding
        );
        assert_eq!(
            rsp.decode(mtc0_word(0, 10)).unwrap_err().reason(),
            MachineRspStepRejectionReason::UnsupportedMtc0ControlRegister { register_index: 10 }
        );
        assert_eq!(
            rsp.decode(mtc0_word(0, 0) | 1).unwrap_err().reason(),
            MachineRspStepRejectionReason::MalformedMtc0Encoding
        );
        assert_eq!(
            rsp.decode(0x3c05_0020),
            Ok(MachineRspDecodedInstruction::Lui {
                destination_gpr: 5,
                immediate: 0x0020,
            })
        );
        assert_eq!(
            rsp.decode(RSP_SCALAR_BREAK_WORD),
            Ok(MachineRspDecodedInstruction::Break)
        );
        for code in [1, 0x12345, RSP_SCALAR_BREAK_CODE_MASK] {
            assert_eq!(
                rsp.decode((code << 6) | u32::from(RSP_SCALAR_BREAK_FUNCTION))
                    .unwrap_err()
                    .reason(),
                MachineRspStepRejectionReason::BreakCodeUnsupported { code }
            );
        }
        assert_eq!(
            rsp.decode(0x0000_000c).unwrap_err().reason(),
            MachineRspStepRejectionReason::UnrepresentedInstruction {
                class: MachineRspUnrepresentedInstructionClass::Scalar,
            }
        );
        assert_eq!(
            rsp.decode(0x2400_000d).unwrap_err().reason(),
            MachineRspStepRejectionReason::UnrepresentedInstruction {
                class: MachineRspUnrepresentedInstructionClass::Scalar,
            }
        );
    }

    #[test]
    fn rsp_mtc0_consumes_available_source_records_exact_lineage_and_commits_once() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x018);
        stage_available_scalar(&mut rsp, 3, 0x0000_0187);
        let scalar_before = rsp.scalar_registers.clone();
        let vector_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();
        let decoded = rsp.decode(mtc0_word(3, 1)).unwrap();
        let plan = rsp
            .plan_mtc0(0x018, decoded, TEST_INSTRUCTION_PROVENANCE)
            .unwrap();
        assert_eq!(plan.source_gpr, 3);
        assert_eq!(plan.source_value(), 0x0000_0187);
        assert_eq!(
            plan.control_register,
            MachineRspControlRegister::SpDramAddress
        );
        assert_eq!(plan.source_index(), 0);

        assert_eq!(
            rsp.apply_mtc0(plan),
            MachineRspStepOutcome::ScalarMtc0Committed {
                instruction_pc: 0x018,
                source_gpr: 3,
                source_value: 0x0000_0187,
                control_register: MachineRspControlRegister::SpDramAddress,
                source_index: 0,
            }
        );
        assert_eq!(rsp.scalar_registers, scalar_before);
        assert_eq!(rsp.vector_unit, vector_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);
        assert_eq!(rsp.next_pc(), Some(0x020));
        assert_eq!(rsp.committed_instruction_count(), 1);
        let last = rsp.last_instruction().unwrap();
        assert_eq!(last.identity(), MachineRspInstructionIdentity::Mtc0);
        assert_eq!(last.source_gpr(), Some(3));
        assert_eq!(
            last.control_register(),
            Some(MachineRspControlRegister::SpDramAddress)
        );
        assert_eq!(last.mtc0_source_index(), Some(0));
        let source = rsp.mtc0_source(0).unwrap();
        assert_eq!(source.instruction_pc(), 0x018);
        assert_eq!(
            source.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );
        assert_eq!(source.instruction_provenance(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(source.source_gpr(), 3);
        assert_eq!(source.source_value(), 0x0000_0187);
        assert_eq!(source.source(), scalar_before[3].source().unwrap());
        assert_eq!(
            source.control_register(),
            MachineRspControlRegister::SpDramAddress
        );

        assert_eq!(
            rsp.plan_mtc0(
                0x020,
                rsp.decode(mtc0_word(2, 0)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::Mtc0SourceUnavailable { source_gpr: 2 }
        );
    }

    #[test]
    fn rsp_xori_is_32_bit_read_before_write_and_r0_discard_is_exact() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x01c);
        stage_available_scalar(&mut rsp, 3, 0xa5a5_00ff);
        let alias_plan = rsp
            .plan_xori(
                0x01c,
                rsp.decode(xori_word(3, 3, 0x0180)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_xori(alias_plan),
            MachineRspStepOutcome::ScalarXoriCommitted {
                instruction_pc: 0x01c,
                destination_gpr: 3,
                result_value: 0xa5a5_017f,
            }
        );
        let state = rsp.scalar_register(3).unwrap();
        assert_eq!(state.value(), Some(0xa5a5_017f));
        let source = match state.source().unwrap() {
            MachineRspScalarRegisterSource::Xori(source) => source,
            other => panic!("Xori result lacks Xori provenance: {other:?}"),
        };
        assert_eq!(source.instruction_pc(), 0x01c);
        assert_eq!(source.instruction_provenance(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(source.source_gpr(), 3);
        assert_eq!(source.source_value(), 0xa5a5_00ff);
        assert_eq!(source.immediate(), 0x0180);
        assert!(matches!(
            source.source(),
            MachineRspScalarRegisterSource::Mfc0(_)
        ));

        let zero_before = rsp.scalar_register(0).unwrap();
        let zero_plan = rsp
            .plan_xori(
                0x020,
                rsp.decode(xori_word(3, 0, 0xffff)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_xori(zero_plan),
            MachineRspStepOutcome::ScalarXoriCommitted {
                instruction_pc: 0x020,
                destination_gpr: 0,
                result_value: 0xa5a5_fe80,
            }
        );
        assert_eq!(rsp.scalar_register(0).unwrap(), zero_before);
        assert_eq!(rsp.committed_instruction_count(), 2);

        assert_eq!(
            rsp.plan_xori(
                0x024,
                rsp.decode(xori_word(2, 4, 1)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::XoriSourceUnavailable { source_gpr: 2 }
        );
    }

    #[test]
    fn rsp_ori_zero_extends_reads_before_write_and_preserves_non_scalar_truth() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x024);
        stage_available_scalar(&mut rsp, 3, 0xa5a5_00ff);
        let vector_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();
        let alias_plan = rsp
            .plan_ori(
                0x024,
                rsp.decode(ori_word(3, 3, 0x8180)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_ori(alias_plan),
            MachineRspStepOutcome::ScalarOriCommitted {
                instruction_pc: 0x024,
                destination_gpr: 3,
                result_value: 0xa5a5_81ff,
            }
        );
        let state = rsp.scalar_register(3).unwrap();
        assert_eq!(state.value(), Some(0xa5a5_81ff));
        let source = match state.source().unwrap() {
            MachineRspScalarRegisterSource::Ori(source) => source,
            other => panic!("Ori result lacks Ori provenance: {other:?}"),
        };
        assert_eq!(source.instruction_pc(), 0x024);
        assert_eq!(source.instruction_provenance(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(source.source_gpr(), 3);
        assert_eq!(source.source_value(), 0xa5a5_00ff);
        assert_eq!(source.immediate(), 0x8180);
        assert!(matches!(
            source.source(),
            MachineRspScalarRegisterSource::Mfc0(_)
        ));
        assert_eq!(rsp.vector_unit, vector_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);
        assert_eq!(rsp.next_pc(), Some(0x02c));
        assert_eq!(rsp.delay_slot_context(), None);
        assert_eq!(rsp.committed_instruction_count(), 1);
        assert_eq!(
            rsp.last_instruction().unwrap().identity(),
            MachineRspInstructionIdentity::Ori
        );
        assert_eq!(rsp.last_instruction().unwrap().destination_gpr(), Some(3));

        let zero_before = rsp.scalar_register(0).unwrap();
        let zero_plan = rsp
            .plan_ori(
                0x02c,
                rsp.decode(ori_word(3, 0, 0xffff)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_ori(zero_plan),
            MachineRspStepOutcome::ScalarOriCommitted {
                instruction_pc: 0x02c,
                destination_gpr: 0,
                result_value: 0xa5a5_ffff,
            }
        );
        assert_eq!(rsp.scalar_register(0).unwrap(), zero_before);
        assert_eq!(rsp.committed_instruction_count(), 2);

        let before_rejection = rsp.clone();
        assert_eq!(
            rsp.plan_ori(
                0x030,
                rsp.decode(ori_word(2, 4, 1)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::OriSourceUnavailable { source_gpr: 2 }
        );
        assert_eq!(rsp, before_rejection);
    }

    #[test]
    fn rsp_sll_reads_before_alias_write_preserves_r0_and_rejects_atomically() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x034);
        stage_available_scalar(&mut rsp, 3, 0x8000_0001);
        let vector_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();
        let alias_plan = rsp
            .plan_sll(
                0x034,
                rsp.decode(sll_word(3, 3, 1)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_sll(alias_plan),
            MachineRspStepOutcome::ScalarSllCommitted {
                instruction_pc: 0x034,
                destination_gpr: 3,
                result_value: 2,
            }
        );
        let state = rsp.scalar_register(3).unwrap();
        assert_eq!(state.value(), Some(2));
        let source = match state.source().unwrap() {
            MachineRspScalarRegisterSource::Sll(source) => source,
            other => panic!("Sll result lacks Sll provenance: {other:?}"),
        };
        assert_eq!(source.instruction_pc(), 0x034);
        assert_eq!(source.instruction_provenance(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(source.source_gpr(), 3);
        assert_eq!(source.source_value(), 0x8000_0001);
        assert_eq!(source.shift_amount(), 1);
        assert!(matches!(
            source.source(),
            MachineRspScalarRegisterSource::Mfc0(_)
        ));
        assert_eq!(rsp.vector_unit, vector_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);
        assert_eq!(rsp.next_pc(), Some(0x03c));
        assert_eq!(rsp.delay_slot_context(), None);
        assert_eq!(rsp.committed_instruction_count(), 1);
        assert_eq!(
            rsp.last_instruction().unwrap().identity(),
            MachineRspInstructionIdentity::Sll
        );
        assert_eq!(rsp.last_instruction().unwrap().destination_gpr(), Some(3));

        let zero_before = rsp.scalar_register(0).unwrap();
        let zero_plan = rsp
            .plan_sll(
                0x03c,
                rsp.decode(sll_word(3, 0, 31)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_sll(zero_plan),
            MachineRspStepOutcome::ScalarSllCommitted {
                instruction_pc: 0x03c,
                destination_gpr: 0,
                result_value: 0,
            }
        );
        assert_eq!(rsp.scalar_register(0).unwrap(), zero_before);
        assert_eq!(rsp.scalar_register(3).unwrap().value(), Some(2));
        assert_eq!(rsp.committed_instruction_count(), 2);

        let before_rejection = rsp.clone();
        assert_eq!(
            rsp.plan_sll(
                0x040,
                rsp.decode(sll_word(2, 4, 7)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::SllSourceUnavailable { source_gpr: 2 }
        );
        assert_eq!(rsp, before_rejection);
    }

    #[test]
    fn rsp_sll_shift_boundaries_and_non_alias_destinations_are_exact() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0);
        stage_available_scalar(&mut rsp, 1, 0x8000_0001);
        stage_available_scalar(&mut rsp, 2, 1);

        let shift_zero = rsp
            .plan_sll(
                0,
                rsp.decode(sll_word(1, 4, 0)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_sll(shift_zero),
            MachineRspStepOutcome::ScalarSllCommitted {
                instruction_pc: 0,
                destination_gpr: 4,
                result_value: 0x8000_0001,
            }
        );

        let shift_one = rsp
            .plan_sll(
                4,
                rsp.decode(sll_word(1, 5, 1)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_sll(shift_one),
            MachineRspStepOutcome::ScalarSllCommitted {
                instruction_pc: 4,
                destination_gpr: 5,
                result_value: 2,
            }
        );

        let shift_thirty_one = rsp
            .plan_sll(
                8,
                rsp.decode(sll_word(2, 6, 31)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_sll(shift_thirty_one),
            MachineRspStepOutcome::ScalarSllCommitted {
                instruction_pc: 8,
                destination_gpr: 6,
                result_value: 0x8000_0000,
            }
        );

        let zero_source = rsp
            .plan_sll(
                12,
                rsp.decode(sll_word(0, 7, 31)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_sll(zero_source),
            MachineRspStepOutcome::ScalarSllCommitted {
                instruction_pc: 12,
                destination_gpr: 7,
                result_value: 0,
            }
        );

        assert_eq!(rsp.scalar_register(1).unwrap().value(), Some(0x8000_0001));
        assert_eq!(rsp.scalar_register(2).unwrap().value(), Some(1));
        assert_eq!(rsp.scalar_register(4).unwrap().value(), Some(0x8000_0001));
        assert_eq!(rsp.scalar_register(5).unwrap().value(), Some(2));
        assert_eq!(rsp.scalar_register(6).unwrap().value(), Some(0x8000_0000));
        assert_eq!(rsp.scalar_register(7).unwrap().value(), Some(0));
        assert_eq!(rsp.next_pc(), Some(20));
        assert_eq!(rsp.delay_slot_context(), None);
        assert_eq!(rsp.committed_instruction_count(), 4);
    }

    #[test]
    fn rsp_lui_and_rsp_addi_exact_scalar_semantics_and_provenance_are_owned_once() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x028);
        assert_eq!(
            rsp.decode(0x3c05_0020),
            Ok(MachineRspDecodedInstruction::Lui {
                destination_gpr: 5,
                immediate: 0x0020,
            })
        );
        assert_eq!(
            rsp.decode(lui_word(5, 0x0020) | (1 << 21))
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::MalformedLuiEncoding
        );

        let vectors_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();
        assert_eq!(
            rsp.apply_lui(rsp.plan_lui(
                0x028,
                rsp.decode(lui_word(5, 0x0020)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )),
            MachineRspStepOutcome::ScalarLuiCommitted {
                instruction_pc: 0x028,
                destination_gpr: 5,
                result_value: 0x0020_0000,
            }
        );
        let r5 = rsp.scalar_register(5).unwrap();
        assert_eq!(r5.value(), Some(0x0020_0000));
        let lui_source = match r5.source().unwrap() {
            MachineRspScalarRegisterSource::Lui(source) => source,
            other => panic!("Lui result lacks exact provenance: {other:?}"),
        };
        assert_eq!(lui_source.instruction_pc(), 0x028);
        assert_eq!(
            lui_source.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );
        assert_eq!(lui_source.immediate(), 0x0020);
        assert_eq!(rsp.vector_unit, vectors_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);

        stage_available_scalar(&mut rsp, 6, 0xdead_beef);
        assert_eq!(
            rsp.apply_lui(rsp.plan_lui(
                0x02c,
                rsp.decode(lui_word(6, 0x1234)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )),
            MachineRspStepOutcome::ScalarLuiCommitted {
                instruction_pc: 0x02c,
                destination_gpr: 6,
                result_value: 0x1234_0000,
            }
        );
        assert_eq!(rsp.scalar_register(6).unwrap().value(), Some(0x1234_0000));

        let r0_before = rsp.scalar_register(0).unwrap();
        assert_eq!(
            rsp.apply_lui(rsp.plan_lui(
                0x030,
                rsp.decode(lui_word(0, 0xffff)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )),
            MachineRspStepOutcome::ScalarLuiCommitted {
                instruction_pc: 0x030,
                destination_gpr: 0,
                result_value: 0xffff_0000,
            }
        );
        assert_eq!(rsp.scalar_register(0).unwrap(), r0_before);

        stage_available_scalar(&mut rsp, 7, 0x7fff_ffff);
        assert_eq!(
            rsp.apply_addi(
                rsp.plan_addi(
                    0x034,
                    rsp.decode(addi_word(7, 7, 1)).unwrap(),
                    TEST_INSTRUCTION_PROVENANCE,
                )
                .unwrap(),
            ),
            MachineRspStepOutcome::ScalarAddiCommitted {
                instruction_pc: 0x034,
                destination_gpr: 7,
                result_value: 0x8000_0000,
            }
        );
        let r7 = rsp.scalar_register(7).unwrap();
        let addi_source = match r7.source().unwrap() {
            MachineRspScalarRegisterSource::Addi(source) => source,
            other => panic!("Addi result lacks exact provenance: {other:?}"),
        };
        assert_eq!(addi_source.instruction_pc(), 0x034);
        assert_eq!(
            addi_source.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );
        assert_eq!(addi_source.source_gpr(), 7);
        assert_eq!(addi_source.source_value(), 0x7fff_ffff);
        assert_eq!(addi_source.signed_immediate(), 1);
        assert!(matches!(
            addi_source.source(),
            MachineRspScalarRegisterSource::Mfc0(_)
        ));

        assert_eq!(
            rsp.apply_addi(
                rsp.plan_addi(
                    0x038,
                    rsp.decode(addi_word(7, 7, -1)).unwrap(),
                    TEST_INSTRUCTION_PROVENANCE,
                )
                .unwrap(),
            ),
            MachineRspStepOutcome::ScalarAddiCommitted {
                instruction_pc: 0x038,
                destination_gpr: 7,
                result_value: 0x7fff_ffff,
            }
        );
        assert_eq!(rsp.scalar_register(7).unwrap().value(), Some(0x7fff_ffff));

        let r0_before = rsp.scalar_register(0).unwrap();
        assert_eq!(
            rsp.apply_addi(
                rsp.plan_addi(
                    0x03c,
                    rsp.decode(addi_word(7, 0, -2)).unwrap(),
                    TEST_INSTRUCTION_PROVENANCE,
                )
                .unwrap(),
            ),
            MachineRspStepOutcome::ScalarAddiCommitted {
                instruction_pc: 0x03c,
                destination_gpr: 0,
                result_value: 0x7fff_fffd,
            }
        );
        assert_eq!(rsp.scalar_register(0).unwrap(), r0_before);
        assert_eq!(
            rsp.plan_addi(
                0x040,
                rsp.decode(addi_word(2, 3, 1)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::AddiSourceUnavailable { source_gpr: 2 }
        );
    }

    #[test]
    fn rsp_bltz_rsp_bne_branch_targets_and_delay_context_are_exact() {
        let mut not_taken = MachineRspExecutionState::default();
        not_taken.synchronize_pc_write(0x02c);
        stage_available_scalar(&mut not_taken, 5, 0x0020_0000);
        assert_eq!(
            not_taken.decode(0x04a0_001b),
            Ok(MachineRspDecodedInstruction::Bltz {
                source_gpr: 5,
                signed_offset: 0x001b,
            })
        );
        let scalar_before = not_taken.scalar_registers.clone();
        let plan = not_taken
            .plan_branch(
                0x02c,
                not_taken.decode(bltz_word(5, 0x001b)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            not_taken.apply_branch(plan),
            MachineRspStepOutcome::ScalarBltzCommitted {
                instruction_pc: 0x02c,
                delay_slot_pc: 0x030,
                target_pc: 0x09c,
                taken: false,
            }
        );
        assert_eq!(not_taken.scalar_registers, scalar_before);
        assert_eq!(not_taken.next_pc(), Some(0x034));
        assert_eq!(not_taken.committed_instruction_count(), 1);
        let context = not_taken.delay_slot_context().unwrap();
        assert_eq!(context.owner_pc(), 0x02c);
        assert_eq!(context.identity(), MachineRspInstructionIdentity::Bltz);
        assert_eq!(context.delay_slot_pc(), 0x030);
        assert_eq!(context.target_pc(), 0x09c);
        assert!(!context.taken());
        assert_eq!(context.source_gpr_a(), Some(5));
        assert_eq!(context.source_value_a(), Some(0x0020_0000));
        assert_eq!(context.source_gpr_b(), None);
        assert_eq!(context.signed_offset(), Some(0x001b));
        let context_before_cpu_interleave = context.clone();
        assert_eq!(
            not_taken.delay_slot_context(),
            Some(context_before_cpu_interleave),
            "an unrelated processor call has no RSP execution-owner mutation seam"
        );
        let slot = not_taken.plan_nop(
            0x030,
            not_taken.decode(0).unwrap(),
            TEST_INSTRUCTION_PROVENANCE,
        );
        assert_eq!(
            not_taken.apply_nop(slot),
            MachineRspStepOutcome::NopCommitted {
                instruction_pc: 0x030,
            }
        );
        assert_eq!(not_taken.delay_slot_context(), None);
        assert_eq!(not_taken.next_pc(), Some(0x038));
        assert_eq!(not_taken.committed_instruction_count(), 2);

        let mut taken = MachineRspExecutionState::default();
        taken.synchronize_pc_write(0x02c);
        stage_available_scalar(&mut taken, 5, 0xffff_ffff);
        let plan = taken
            .plan_branch(
                0x02c,
                taken.decode(bltz_word(5, 0x001b)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            taken.apply_branch(plan),
            MachineRspStepOutcome::ScalarBltzCommitted {
                instruction_pc: 0x02c,
                delay_slot_pc: 0x030,
                target_pc: 0x09c,
                taken: true,
            }
        );
        assert_eq!(taken.next_pc(), Some(0x09c));

        let mut bne = MachineRspExecutionState::default();
        bne.synchronize_pc_write(0x034);
        stage_available_scalar(&mut bne, 3, 1);
        stage_available_scalar(&mut bne, 4, 0x0001_0000);
        assert_eq!(
            bne.decode(0x1460_fffd),
            Ok(MachineRspDecodedInstruction::Bne {
                source_gpr_a: 3,
                source_gpr_b: 0,
                signed_offset: -3,
            })
        );
        let plan = bne
            .plan_branch(
                0x034,
                bne.decode(bne_word(3, 0, -3)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            bne.apply_branch(plan),
            MachineRspStepOutcome::ScalarBneCommitted {
                instruction_pc: 0x034,
                delay_slot_pc: 0x038,
                target_pc: 0x02c,
                taken: true,
            }
        );
        let context = bne.delay_slot_context().unwrap();
        assert_eq!(context.source_gpr_a(), Some(3));
        assert_eq!(context.source_value_a(), Some(1));
        assert_eq!(context.source_gpr_b(), Some(0));
        assert_eq!(context.source_value_b(), Some(0));
        assert_eq!(context.signed_offset(), Some(-3));

        let mut full_width = MachineRspExecutionState::default();
        full_width.synchronize_pc_write(0x058);
        stage_available_scalar(&mut full_width, 3, 0x0001_0000);
        stage_available_scalar(&mut full_width, 4, 0);
        let plan = full_width
            .plan_branch(
                0x058,
                full_width.decode(bne_word(3, 4, -2)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            full_width.apply_branch(plan),
            MachineRspStepOutcome::ScalarBneCommitted {
                instruction_pc: 0x058,
                delay_slot_pc: 0x05c,
                target_pc: 0x054,
                taken: true,
            },
            "Bne compares all 32 bits rather than only the low half"
        );
    }

    #[test]
    fn rsp_bgezal_links_only_when_taken_and_reads_alias_before_write() {
        let mut taken = MachineRspExecutionState::default();
        taken.synchronize_pc_write(0x02c);
        stage_available_scalar(&mut taken, 5, 0x7fff_ffff);
        stage_available_scalar(&mut taken, 31, 0xaaaa_5555);
        assert_eq!(
            taken.decode(bgezal_word(5, 0x001b)),
            Ok(MachineRspDecodedInstruction::Bgezal {
                source_gpr: 5,
                signed_offset: 0x001b,
            })
        );
        let plan = taken
            .plan_branch(
                0x02c,
                taken.decode(bgezal_word(5, 0x001b)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            taken.apply_branch(plan),
            MachineRspStepOutcome::ScalarBgezalCommitted {
                instruction_pc: 0x02c,
                delay_slot_pc: 0x030,
                target_pc: 0x09c,
                taken: true,
                link_value: Some(0x034),
            }
        );
        assert_eq!(taken.next_pc(), Some(0x09c));
        assert_eq!(taken.committed_instruction_count(), 1);
        assert_eq!(taken.scalar_register(31).unwrap().value(), Some(0x034));
        let link_source = match taken.scalar_register(31).unwrap().source() {
            Some(MachineRspScalarRegisterSource::Bgezal(source)) => source,
            other => panic!("Bgezal link lacks exact source: {other:?}"),
        };
        assert_eq!(link_source.instruction_pc(), 0x02c);
        assert_eq!(link_source.source_gpr(), 5);
        assert_eq!(link_source.source_value(), 0x7fff_ffff);
        assert_eq!(link_source.signed_offset(), 0x001b);
        assert_eq!(link_source.link_value(), 0x034);
        assert_eq!(
            link_source.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );
        assert_eq!(
            taken.last_instruction().unwrap().identity(),
            MachineRspInstructionIdentity::Bgezal
        );
        assert_eq!(
            taken.last_instruction().unwrap().destination_gpr(),
            Some(31)
        );
        let context = taken.delay_slot_context().unwrap();
        assert_eq!(context.identity(), MachineRspInstructionIdentity::Bgezal);
        assert_eq!(context.source_gpr_a(), Some(5));
        assert_eq!(context.source_value_a(), Some(0x7fff_ffff));
        assert_eq!(context.signed_offset(), Some(0x001b));
        assert!(context.taken());

        let mut not_taken = MachineRspExecutionState::default();
        not_taken.synchronize_pc_write(0x02c);
        stage_available_scalar(&mut not_taken, 5, 0x8000_0000);
        stage_available_scalar(&mut not_taken, 31, 0x1234_5678);
        let prior_link = not_taken.scalar_register(31).unwrap().clone();
        let plan = not_taken
            .plan_branch(
                0x02c,
                not_taken.decode(bgezal_word(5, -3)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            not_taken.apply_branch(plan),
            MachineRspStepOutcome::ScalarBgezalCommitted {
                instruction_pc: 0x02c,
                delay_slot_pc: 0x030,
                target_pc: 0x024,
                taken: false,
                link_value: None,
            }
        );
        assert_eq!(not_taken.next_pc(), Some(0x034));
        assert_eq!(not_taken.scalar_register(31), Some(prior_link));
        assert_eq!(
            not_taken.last_instruction().unwrap().destination_gpr(),
            None
        );

        let mut aliased = MachineRspExecutionState::default();
        aliased.synchronize_pc_write(0x0ff8);
        stage_available_scalar(&mut aliased, 31, 1);
        let plan = aliased
            .plan_branch(
                0x0ff8,
                aliased.decode(bgezal_word(31, 1)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            aliased.apply_branch(plan),
            MachineRspStepOutcome::ScalarBgezalCommitted {
                instruction_pc: 0x0ff8,
                delay_slot_pc: 0x0ffc,
                target_pc: 0,
                taken: true,
                link_value: Some(0),
            }
        );
        assert_eq!(aliased.scalar_register(31).unwrap().value(), Some(0));
        let source = match aliased.scalar_register(31).unwrap().source() {
            Some(MachineRspScalarRegisterSource::Bgezal(source)) => source,
            other => panic!("aliased Bgezal link lacks exact source: {other:?}"),
        };
        assert_eq!(source.source_gpr(), 31);
        assert_eq!(source.source_value(), 1);

        let mut unavailable = MachineRspExecutionState::default();
        unavailable.synchronize_pc_write(0x040);
        let before = unavailable.clone();
        assert_eq!(
            unavailable
                .plan_branch(
                    0x040,
                    unavailable.decode(bgezal_word(3, 1)).unwrap(),
                    TEST_INSTRUCTION_PROVENANCE,
                )
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::BgezalSourceUnavailable { source_gpr: 3 }
        );
        assert_eq!(unavailable, before);
    }

    #[test]
    fn rsp_j_uses_exact_local_target_and_one_delay_slot_without_register_inputs() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x120);
        assert_eq!(
            rsp.decode(j_word(0x3a0)),
            Ok(MachineRspDecodedInstruction::J { target_pc: 0x3a0 })
        );
        assert_eq!(
            jump_target_local_pc((0x0015_a3a0_u32 >> 2) & 0x03ff_ffff),
            0x3a0,
            "only the aligned low twelve local-PC bits survive"
        );

        let scalar_before = rsp.scalar_registers.clone();
        let vector_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();
        let plan = rsp
            .plan_branch(
                0x120,
                rsp.decode(j_word(0x3a0)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_branch(plan),
            MachineRspStepOutcome::ScalarJCommitted {
                instruction_pc: 0x120,
                delay_slot_pc: 0x124,
                target_pc: 0x3a0,
            }
        );
        assert_eq!(rsp.scalar_registers, scalar_before);
        assert_eq!(rsp.vector_unit, vector_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);
        assert_eq!(rsp.next_pc(), Some(0x3a0));
        assert_eq!(rsp.committed_instruction_count(), 1);
        assert_eq!(
            rsp.last_instruction().unwrap().identity(),
            MachineRspInstructionIdentity::J
        );

        let context = rsp.delay_slot_context().unwrap();
        assert_eq!(context.owner_pc(), 0x120);
        assert_eq!(context.identity(), MachineRspInstructionIdentity::J);
        assert_eq!(context.delay_slot_pc(), 0x124);
        assert_eq!(context.target_pc(), 0x3a0);
        assert!(context.taken());
        assert_eq!(context.source_gpr_a(), None);
        assert_eq!(context.source_value_a(), None);
        assert_eq!(context.source_a(), None);
        assert_eq!(context.source_gpr_b(), None);
        assert_eq!(context.source_value_b(), None);
        assert_eq!(context.source_b(), None);
        assert_eq!(context.signed_offset(), None);
        assert_eq!(
            context.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );

        let rejection_before = rsp.clone();
        assert_eq!(
            rsp.plan_branch(
                0x124,
                rsp.decode(j_word(0x200)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::ControlFlowInDelaySlot { owner_pc: 0x120 }
        );
        assert_eq!(rsp, rejection_before);

        let slot = rsp.plan_nop(0x124, rsp.decode(0).unwrap(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(
            rsp.apply_nop(slot),
            MachineRspStepOutcome::NopCommitted {
                instruction_pc: 0x124,
            }
        );
        assert_eq!(rsp.delay_slot_context(), None);
        assert_eq!(rsp.next_pc(), Some(0x3a4));
        assert_eq!(rsp.committed_instruction_count(), 2);
    }

    #[test]
    fn rsp_branch_rejections_preserve_delay_context_and_unknown_sources() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x02c);
        assert_eq!(
            rsp.plan_branch(
                0x02c,
                rsp.decode(bltz_word(5, 0x001b)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::BltzSourceUnavailable { source_gpr: 5 }
        );
        assert_eq!(
            rsp.plan_branch(
                0x034,
                rsp.decode(bne_word(3, 0, -3)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::BneSourceAUnavailable { source_gpr: 3 }
        );
        stage_available_scalar(&mut rsp, 3, 1);
        assert_eq!(
            rsp.plan_branch(
                0x034,
                rsp.decode(bne_word(3, 4, -3)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::BneSourceBUnavailable { source_gpr: 4 }
        );
        assert_eq!(
            rsp.decode(((RSP_SCALAR_REGIMM_OPCODE as u32) << 26) | (5 << 21) | (2 << 16))
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::UnsupportedRegimmSelector { selector: 2 }
        );

        let plan = rsp
            .plan_branch(
                0x034,
                rsp.decode(bne_word(3, 0, -3)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        rsp.apply_branch(plan);
        let post_branch = rsp.clone();
        assert_eq!(
            rsp.plan_branch(
                0x038,
                rsp.decode(bltz_word(3, 1)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::ControlFlowInDelaySlot { owner_pc: 0x034 }
        );
        assert_eq!(rsp, post_branch);
    }

    #[test]
    fn rsp_mfc0_sp_dma_busy_and_full_read_atomic_idle_truth_before_vsub_frontier() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x054);
        assert_eq!(
            rsp.decode(0x4003_3000),
            Ok(MachineRspDecodedInstruction::Mfc0 {
                destination_gpr: 3,
                control_register: MachineRspControlRegister::SpDmaBusy,
            })
        );
        let plan = rsp.plan_mfc0(
            0x054,
            rsp.decode(mfc0_word(3, RSP_COP0_SP_DMA_BUSY_INDEX))
                .unwrap(),
            MachineRspMfc0ControlSource::SpDmaBusy { busy: false },
            TEST_INSTRUCTION_PROVENANCE,
        );
        assert_eq!(
            rsp.apply_mfc0(plan),
            MachineRspStepOutcome::ScalarMfc0Committed {
                instruction_pc: 0x054,
                destination_gpr: 3,
                control_register: MachineRspControlRegister::SpDmaBusy,
                result_value: 0,
            }
        );
        let r3 = rsp.scalar_register(3).unwrap();
        assert_eq!(r3.value(), Some(0));
        let source = match r3.source().unwrap() {
            MachineRspScalarRegisterSource::Mfc0(source) => source,
            other => panic!("DMA_BUSY result lacks exact Mfc0 provenance: {other:?}"),
        };
        assert_eq!(source.instruction_pc(), 0x054);
        assert_eq!(
            source.control_source(),
            MachineRspMfc0ControlSource::SpDmaBusy { busy: false }
        );
        assert_eq!(
            rsp.decode(mfc0_word(4, RSP_COP0_SP_DMA_FULL_INDEX)),
            Ok(MachineRspDecodedInstruction::Mfc0 {
                destination_gpr: 4,
                control_register: MachineRspControlRegister::SpDmaFull,
            })
        );
        let plan = rsp.plan_mfc0(
            0x058,
            rsp.decode(mfc0_word(4, RSP_COP0_SP_DMA_FULL_INDEX))
                .unwrap(),
            MachineRspMfc0ControlSource::SpDmaFull { full: false },
            TEST_INSTRUCTION_PROVENANCE,
        );
        assert_eq!(
            rsp.apply_mfc0(plan),
            MachineRspStepOutcome::ScalarMfc0Committed {
                instruction_pc: 0x058,
                destination_gpr: 4,
                control_register: MachineRspControlRegister::SpDmaFull,
                result_value: 0,
            }
        );
        let source = match rsp.scalar_register(4).unwrap().source().unwrap() {
            MachineRspScalarRegisterSource::Mfc0(source) => source,
            other => panic!("DMA_FULL result lacks exact Mfc0 provenance: {other:?}"),
        };
        assert_eq!(
            source.control_source(),
            MachineRspMfc0ControlSource::SpDmaFull { full: false }
        );
        assert_eq!(
            rsp.decode(0x4a0d_6b51),
            Ok(MachineRspDecodedInstruction::Vsub {
                destination_vector: 13,
                source_vector_a: 13,
                source_vector_b: 13,
                element: 0,
            })
        );
    }

    #[test]
    fn rsp_vector_lqv_address_uses_low_twelve_signed_scaled_wrapping_truth() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0);

        let positive = rsp
            .plan_lqv_address(0, rsp.decode(lqv_word(0, 12, 0, 1)).unwrap())
            .unwrap();
        assert_eq!(positive.local_dmem_address(), 0x010);

        let negative = rsp
            .plan_lqv_address(0, rsp.decode(lqv_word(0, 12, 0, -1)).unwrap())
            .unwrap();
        assert_eq!(negative.local_dmem_address(), 0xff0);

        stage_available_scalar(&mut rsp, 3, 0xabcd_f020);
        let low_twelve = rsp
            .plan_lqv_address(0, rsp.decode(lqv_word(3, 12, 0, 0)).unwrap())
            .unwrap();
        assert_eq!(low_twelve.local_dmem_address(), 0x020);

        assert_eq!(
            rsp.plan_lqv_address(0, rsp.decode(lqv_word(1, 12, 0, 0)).unwrap())
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::LqvScalarBaseUnavailable { base_gpr: 1 }
        );
        assert_eq!(
            rsp.plan_lqv_address(0, rsp.decode(lqv_word(0, 12, 1, 0)).unwrap())
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::LqvElementUnsupported { element: 1 }
        );
        stage_available_scalar(&mut rsp, 4, 0x1234_5008);
        assert_eq!(
            rsp.plan_lqv_address(0, rsp.decode(lqv_word(4, 12, 0, 0)).unwrap())
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::LqvAddressMisaligned {
                local_dmem_address: 8,
            }
        );
    }

    #[test]
    fn rsp_vector_lqv_concrete_source_maps_exact_byte_order_and_commit_cadence() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0);
        let scalar_before = rsp.scalar_registers.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();
        let bytes = core::array::from_fn(|index| (index as u8).wrapping_mul(17).wrapping_add(3));
        let address = rsp
            .plan_lqv_address(0, rsp.decode(lqv_word(0, 12, 0, 0)).unwrap())
            .unwrap();
        let plan = rsp
            .plan_lqv(
                address,
                TEST_INSTRUCTION_PROVENANCE,
                available_dmem_observations(bytes),
            )
            .unwrap();
        let source = match &plan.destination_state {
            MachineRspVectorRegisterState::Available {
                bytes: planned,
                source: MachineRspVectorRegisterSource::Lqv(source),
            } => {
                assert_eq!(*planned, bytes);
                source.as_ref()
            }
            other => panic!("concrete Lqv did not plan an Available vector: {other:?}"),
        };
        assert_eq!(source.instruction_pc(), 0);
        assert_eq!(source.base_gpr(), 0);
        assert_eq!(source.base_value(), 0);
        assert_eq!(
            source.base_source(),
            MachineRspScalarRegisterSource::ArchitecturalZero
        );
        assert_eq!(source.element(), 0);
        assert_eq!(source.signed_offset(), 0);
        assert_eq!(source.local_dmem_address(), 0);
        assert!(source
            .dmem_knowledge()
            .iter()
            .enumerate()
            .all(|(index, descriptor)| {
                descriptor.offset() == SpDmemOffset::new(index as u32)
                    && matches!(
                        descriptor.source(),
                        MachineSpDmemByteKnowledgeSource::Available {
                            source: crate::sp_dmem::MachineSpDmemByteSource::GeneratedMachineTestStaging,
                        }
                    )
            }));

        let outcome = rsp.apply_lqv(plan);
        assert!(matches!(
            outcome,
            MachineRspStepOutcome::VectorLqvCommitted {
                instruction_pc: 0,
                destination_vector: 12,
                local_dmem_address: 0,
                result_available: true,
            }
        ));
        assert_eq!(rsp.vector_register(12).unwrap().bytes(), Some(&bytes));
        assert!((0..RSP_VECTOR_REGISTER_COUNT)
            .filter(|index| *index != 12)
            .all(|index| matches!(
                rsp.vector_register(index),
                Some(MachineRspVectorRegisterState::Unavailable {
                    source: MachineRspVectorUnavailableSource::ConstructionOrReset,
                })
            )));
        assert_eq!(rsp.scalar_registers, scalar_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);
        assert_eq!(rsp.next_pc(), Some(8));
        assert_eq!(rsp.delay_slot_context(), None);
        assert_eq!(rsp.committed_instruction_count(), 1);
        assert_eq!(
            rsp.last_instruction().unwrap().identity(),
            MachineRspInstructionIdentity::Lqv
        );
        assert_eq!(
            rsp.last_instruction().unwrap().destination_vector(),
            Some(12)
        );
    }

    #[test]
    fn rsp_vector_lqv_unavailable_and_mixed_sources_replace_whole_register_only() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0);
        let decoded = rsp.decode(lqv_word(0, 12, 0, 0)).unwrap();
        let unavailable_plan = rsp
            .plan_lqv(
                rsp.plan_lqv_address(0, decoded).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
                unavailable_dmem_observations(),
            )
            .unwrap();
        rsp.apply_lqv(unavailable_plan);
        let unavailable = rsp.vector_register(12).unwrap();
        assert_eq!(unavailable.bytes(), None);
        let unavailable_source = match unavailable.unavailable_source() {
            Some(MachineRspVectorUnavailableSource::Lqv(source)) => source,
            other => panic!("missing whole-register unavailable cause: {other:?}"),
        };
        assert!(unavailable_source
            .dmem_knowledge()
            .iter()
            .all(|descriptor| !descriptor.is_available()));

        let concrete_bytes = core::array::from_fn(|index| 0xf0_u8.wrapping_sub(index as u8 * 7));
        let concrete_plan = rsp
            .plan_lqv(
                rsp.plan_lqv_address(4, decoded).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
                available_dmem_observations(concrete_bytes),
            )
            .unwrap();
        rsp.apply_lqv(concrete_plan);
        assert_eq!(
            rsp.vector_register(12).unwrap().bytes(),
            Some(&concrete_bytes),
            "a complete concrete load replaces an unavailable destination"
        );

        let mut mixed = unavailable_dmem_observations();
        mixed[0] = MachineSpDmemByteKnowledge::Available {
            value: 0x5a,
            source: crate::sp_dmem::MachineSpDmemByteSource::GeneratedMachineTestStaging,
        };
        let mixed_plan = rsp
            .plan_lqv(
                rsp.plan_lqv_address(8, decoded).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
                mixed,
            )
            .unwrap();
        rsp.apply_lqv(mixed_plan);
        let mixed_result = rsp.vector_register(12).unwrap();
        assert_eq!(
            mixed_result.bytes(),
            None,
            "mixed knowledge stores no partial vector payload"
        );
        let mixed_source = match mixed_result.unavailable_source() {
            Some(MachineRspVectorUnavailableSource::Lqv(source)) => source,
            other => panic!("mixed Lqv lacks exact unavailable cause: {other:?}"),
        };
        assert!(mixed_source.dmem_knowledge()[0].is_available());
        assert!(mixed_source.dmem_knowledge()[1..]
            .iter()
            .all(|descriptor| !descriptor.is_available()));
        assert_eq!(rsp.committed_instruction_count(), 3);
        assert_eq!(rsp.next_pc(), Some(16));
    }

    #[test]
    fn rsp_scalar_lw_address_uses_signed_low_twelve_aligned_truth() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x00c);

        let public = rsp
            .plan_lw_address(0x00c, rsp.decode(0x8c04_0040).unwrap())
            .unwrap();
        assert_eq!(public.local_dmem_address(), 0x040);

        let negative = rsp
            .plan_lw_address(0x00c, rsp.decode(lw_word(0, 4, -4)).unwrap())
            .unwrap();
        assert_eq!(negative.local_dmem_address(), 0xffc);

        stage_available_scalar(&mut rsp, 3, 0xabcd_f040);
        let low_twelve = rsp
            .plan_lw_address(0x00c, rsp.decode(lw_word(3, 4, 0)).unwrap())
            .unwrap();
        assert_eq!(low_twelve.local_dmem_address(), 0x040);

        stage_available_scalar(&mut rsp, 5, 0xffff_0ffc);
        let wrapped = rsp
            .plan_lw_address(0x00c, rsp.decode(lw_word(5, 4, 4)).unwrap())
            .unwrap();
        assert_eq!(wrapped.local_dmem_address(), 0);

        assert_eq!(
            rsp.plan_lw_address(0x00c, rsp.decode(lw_word(1, 4, 0)).unwrap())
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::ScalarLwBaseUnavailable { base_gpr: 1 }
        );
        assert_eq!(
            rsp.plan_lw_address(0x00c, rsp.decode(lw_word(0, 4, 2)).unwrap())
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::ScalarLwAddressMisaligned {
                local_dmem_address: 2,
            }
        );
    }

    #[test]
    fn rsp_scalar_lw_available_word_commits_big_endian_truth_and_exact_provenance() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x00c);
        let vectors_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();
        let address = rsp
            .plan_lw_address(0x00c, rsp.decode(0x8c04_0040).unwrap())
            .unwrap();
        let plan = rsp
            .plan_lw(
                address,
                TEST_INSTRUCTION_PROVENANCE,
                available_lw_observations(0x040, [0x03, 0xa0, 0x48, 0x20]),
            )
            .unwrap();
        let outcome = rsp.apply_lw(plan);

        assert_eq!(
            outcome,
            MachineRspStepOutcome::ScalarLwCommitted {
                instruction_pc: 0x00c,
                destination_gpr: 4,
                local_dmem_address: 0x040,
                result_value: 0x03a0_4820,
            }
        );
        let r4 = rsp.scalar_register(4).unwrap();
        assert_eq!(r4.value(), Some(0x03a0_4820));
        let source = match r4.source() {
            Some(MachineRspScalarRegisterSource::Lw(source)) => source,
            other => panic!("scalar Lw result lacks exact provenance: {other:?}"),
        };
        assert_eq!(source.instruction_pc(), 0x00c);
        assert_eq!(
            source.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );
        assert_eq!(source.instruction_provenance(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(source.base_gpr(), 0);
        assert_eq!(source.base_value(), 0);
        assert_eq!(
            source.base_source(),
            MachineRspScalarRegisterSource::ArchitecturalZero
        );
        assert_eq!(source.signed_offset(), 0x40);
        assert_eq!(source.local_dmem_address(), 0x040);
        assert!(source
            .dmem_knowledge()
            .iter()
            .enumerate()
            .all(|(index, descriptor)| {
                descriptor.offset() == SpDmemOffset::new(0x040 + index as u32)
                    && matches!(
                        descriptor.source(),
                        MachineSpDmemByteKnowledgeSource::Available {
                            source: crate::sp_dmem::MachineSpDmemByteSource::GeneratedMachineTestStaging,
                        }
                    )
            }));
        assert_eq!(rsp.vector_unit, vectors_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);
        assert_eq!(rsp.next_pc(), Some(0x014));
        assert_eq!(rsp.delay_slot_context(), None);
        assert_eq!(rsp.committed_instruction_count(), 1);
        assert_eq!(
            rsp.last_instruction().unwrap().identity(),
            MachineRspInstructionIdentity::Lw
        );
        assert_eq!(rsp.last_instruction().unwrap().destination_gpr(), Some(4));
    }

    #[test]
    fn rsp_scalar_lw_replaces_destination_discards_r0_and_rejects_bad_knowledge() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0);
        stage_available_scalar(&mut rsp, 4, 0xdead_beef);
        let plan = rsp
            .plan_lw(
                rsp.plan_lw_address(0, rsp.decode(lw_word(0, 4, 0x40)).unwrap())
                    .unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
                available_lw_observations(0x040, [0x11, 0x22, 0x33, 0x44]),
            )
            .unwrap();
        rsp.apply_lw(plan);
        assert_eq!(rsp.scalar_register(4).unwrap().value(), Some(0x1122_3344));

        let r0_before = rsp.scalar_register(0).unwrap();
        let plan = rsp
            .plan_lw(
                rsp.plan_lw_address(4, rsp.decode(lw_word(0, 0, 0x40)).unwrap())
                    .unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
                available_lw_observations(0x040, [0xaa, 0xbb, 0xcc, 0xdd]),
            )
            .unwrap();
        assert_eq!(
            rsp.apply_lw(plan),
            MachineRspStepOutcome::ScalarLwCommitted {
                instruction_pc: 4,
                destination_gpr: 0,
                local_dmem_address: 0x040,
                result_value: 0xaabb_ccdd,
            }
        );
        assert_eq!(rsp.scalar_register(0).unwrap(), r0_before);

        let address = rsp
            .plan_lw_address(8, rsp.decode(lw_word(0, 6, 0x40)).unwrap())
            .unwrap();
        let mut unavailable = available_lw_observations(0x040, [1, 2, 3, 4]);
        unavailable[2] = MachineRspScalarLwDmemObservation::from_knowledge(
            SpDmemOffset::new(0x042),
            MachineSpDmemByteKnowledge::Unavailable {
                source: crate::sp_dmem::MachineSpDmemUnavailableSource::ConstructionOrReset,
            },
        );
        assert_eq!(
            rsp.plan_lw(address.clone(), TEST_INSTRUCTION_PROVENANCE, unavailable)
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::ScalarLwDmemByteUnavailable {
                local_dmem_address: 0x040,
                first_unavailable_offset: 0x042,
            }
        );
        assert!(!rsp.scalar_register(6).unwrap().is_available());

        let mut inconsistent = available_lw_observations(0x040, [1, 2, 3, 4]);
        inconsistent[1] = MachineRspScalarLwDmemObservation::from_parts(
            MachineSpDmemByteKnowledgeDescriptor::new(
                SpDmemOffset::new(0x047),
                MachineSpDmemByteKnowledgeSource::Available {
                    source: crate::sp_dmem::MachineSpDmemByteSource::GeneratedMachineTestStaging,
                },
            ),
            Some(2),
        );
        assert_eq!(
            rsp.plan_lw(address, TEST_INSTRUCTION_PROVENANCE, inconsistent)
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::ScalarLwDmemKnowledgeMalformed {
                local_dmem_address: 0x040,
            }
        );
        assert!(!rsp.scalar_register(6).unwrap().is_available());
    }

    #[test]
    fn rsp_nop_raw_zero_words_commit_independently_without_data_effects() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x010);
        stage_available_scalar(&mut rsp, 4, 0x03a0_4820);
        let scalars_before = rsp.scalar_registers.clone();
        let vectors_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags.clone();

        let first = rsp.plan_nop(0x010, rsp.decode(0).unwrap(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(
            rsp.apply_nop(first),
            MachineRspStepOutcome::NopCommitted {
                instruction_pc: 0x010,
            }
        );
        assert_eq!(rsp.next_pc(), Some(0x018));
        assert_eq!(rsp.committed_instruction_count(), 1);
        assert_eq!(
            rsp.last_instruction().unwrap().identity(),
            MachineRspInstructionIdentity::Nop
        );
        assert_eq!(rsp.scalar_registers, scalars_before);
        assert_eq!(rsp.vector_unit, vectors_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);

        let second = rsp.plan_nop(0x014, rsp.decode(0).unwrap(), TEST_INSTRUCTION_PROVENANCE);
        assert_eq!(
            rsp.apply_nop(second),
            MachineRspStepOutcome::NopCommitted {
                instruction_pc: 0x014,
            }
        );
        assert_eq!(rsp.next_pc(), Some(0x01c));
        assert_eq!(rsp.committed_instruction_count(), 2);
        assert_eq!(rsp.scalar_registers, scalars_before);
        assert_eq!(rsp.vector_unit, vectors_before);
        assert_eq!(rsp.accumulator_and_flags, accumulator_before);
    }

    #[test]
    fn rsp_vector_control_accumulator_slice_vco_vcc_vce_lifecycle_is_exact() {
        let first = MachineRspExecutionState::default();
        let second = MachineRspExecutionState::default();
        assert_eq!(first, second);
        let state = first.accumulator_and_flags();
        assert_eq!(state.accumulator().lane_count(), RSP_VECTOR_LANE_COUNT);
        for lane_index in 0..RSP_VECTOR_LANE_COUNT {
            let lane = state.accumulator().lane(lane_index).unwrap();
            for slice in [lane.high(), lane.middle(), lane.low()] {
                assert!(!slice.is_available());
                assert_eq!(
                    slice.unavailable_source(),
                    Some(&MachineRspAccumulatorSliceUnavailableSource::ConstructionOrReset)
                );
            }
        }
        for half in [state.vco().carry_or_borrow(), state.vco().not_equal()] {
            assert!(!half.is_available());
            assert_eq!(
                half.unavailable_source(),
                Some(&MachineRspVcoHalfUnavailableSource::ConstructionOrReset)
            );
        }
        assert_eq!(
            state.vcc(),
            MachineRspVccState::Unavailable {
                source: MachineRspVccSource::ConstructionOrReset,
            }
        );
        assert_eq!(
            state.vce(),
            MachineRspVceState::Unavailable {
                source: MachineRspVceSource::ConstructionOrReset,
            }
        );
    }

    #[test]
    fn rsp_vsub_self_alias_proves_all_borrow_patterns_without_consuming_vector_bytes() {
        let word = vector_compute_word(RSP_VECTOR_VSUB_FUNCTION, 13, 13, 13, 0);
        assert_eq!(word, 0x4a0d_6b51);
        for borrow in 0_u8..=u8::MAX {
            let mut rsp = MachineRspExecutionState::default();
            rsp.synchronize_pc_write(0x060);
            stage_available_vector_control(&mut rsp, borrow);
            let high_middle_before: Vec<_> = rsp
                .accumulator_and_flags
                .accumulator
                .lanes
                .iter()
                .map(|lane| (lane.high.clone(), lane.middle.clone()))
                .collect();
            let vcc_before = rsp.accumulator_and_flags.vcc;
            let vce_before = rsp.accumulator_and_flags.vce;
            let decoded = rsp.decode(word).unwrap();
            let plan = rsp
                .plan_vector_arithmetic(0x060, decoded, TEST_INSTRUCTION_PROVENANCE)
                .unwrap();
            assert_eq!(
                rsp.apply_vector_arithmetic(plan),
                MachineRspStepOutcome::VectorVsubCommitted {
                    instruction_pc: 0x060,
                    destination_vector: 13,
                    result_available: true,
                }
            );

            let result = rsp.vector_register(13).unwrap();
            let expected_lanes = core::array::from_fn(|lane| {
                if borrow & (1 << lane) == 0 {
                    0
                } else {
                    u16::MAX
                }
            });
            assert_eq!(
                result.bytes(),
                Some(&vector_bytes_from_lanes(expected_lanes))
            );
            let source = match result.available_source().unwrap() {
                MachineRspVectorRegisterSource::Vsub(source) => source,
                other => panic!("Vsub result lacks Vsub provenance: {other:?}"),
            };
            assert!(source.sources_alias());
            assert!(!source.source_a().is_available());
            assert_eq!(source.source_b(), None);
            assert_eq!(
                source
                    .vsub_borrow_input()
                    .and_then(MachineRspVcoHalfState::value),
                Some(borrow)
            );
            assert!(source.result_available());
            for lane_index in 0..RSP_VECTOR_LANE_COUNT {
                let lane = rsp
                    .accumulator_and_flags
                    .accumulator
                    .lane(lane_index)
                    .unwrap();
                assert_eq!(lane.low().value(), Some(expected_lanes[lane_index]));
                assert_eq!(
                    (&lane.high, &lane.middle),
                    (
                        &high_middle_before[lane_index].0,
                        &high_middle_before[lane_index].1
                    )
                );
            }
            assert_eq!(
                rsp.accumulator_and_flags.vco.carry_or_borrow.value(),
                Some(0)
            );
            assert_eq!(rsp.accumulator_and_flags.vco.not_equal.value(), Some(0));
            assert_eq!(rsp.accumulator_and_flags.vcc, vcc_before);
            assert_eq!(rsp.accumulator_and_flags.vce, vce_before);
            assert_eq!(rsp.next_pc(), Some(0x068));
            assert_eq!(rsp.committed_instruction_count(), 1);
        }
    }

    #[test]
    fn rsp_vsub_concrete_signed_clamp_accumulator_and_unavailable_policy_are_exact() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x060);
        stage_available_vector_control(&mut rsp, 0);
        stage_available_vector(
            &mut rsp,
            13,
            vector_bytes_from_lanes([
                0x7fff,
                0x8000,
                100,
                (-100_i16) as u16,
                0,
                1,
                (-1_i16) as u16,
                30_000,
            ]),
        );
        stage_available_vector(
            &mut rsp,
            14,
            vector_bytes_from_lanes([
                (-1_i16) as u16,
                1,
                (-50_i16) as u16,
                50,
                0,
                (-1_i16) as u16,
                1,
                (-30_000_i16) as u16,
            ]),
        );
        let high_middle_before: Vec<_> = rsp
            .accumulator_and_flags
            .accumulator
            .lanes
            .iter()
            .map(|lane| (lane.high.clone(), lane.middle.clone()))
            .collect();
        let plan = rsp
            .plan_vector_arithmetic(
                0x060,
                rsp.decode(vector_compute_word(RSP_VECTOR_VSUB_FUNCTION, 13, 13, 14, 0))
                    .unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert!(rsp.apply_vector_arithmetic(plan).vector_result_available() == Some(true));
        assert_eq!(
            rsp.vector_register(13).unwrap().bytes(),
            Some(&vector_bytes_from_lanes([
                0x7fff,
                0x8000,
                150,
                (-150_i16) as u16,
                0,
                2,
                (-2_i16) as u16,
                0x7fff,
            ]))
        );
        let expected_low = [
            0x8000,
            0x7fff,
            150,
            (-150_i16) as u16,
            0,
            2,
            (-2_i16) as u16,
            60_000,
        ];
        for lane_index in 0..RSP_VECTOR_LANE_COUNT {
            let lane = rsp
                .accumulator_and_flags
                .accumulator
                .lane(lane_index)
                .unwrap();
            assert_eq!(lane.low().value(), Some(expected_low[lane_index]));
            assert_eq!(
                (&lane.high, &lane.middle),
                (
                    &high_middle_before[lane_index].0,
                    &high_middle_before[lane_index].1
                )
            );
        }

        let mut unavailable_borrow = MachineRspExecutionState::default();
        unavailable_borrow.synchronize_pc_write(0x060);
        let plan = unavailable_borrow
            .plan_vector_arithmetic(
                0x060,
                unavailable_borrow
                    .decode(vector_compute_word(RSP_VECTOR_VSUB_FUNCTION, 13, 13, 13, 0))
                    .unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            unavailable_borrow
                .apply_vector_arithmetic(plan)
                .vector_result_available(),
            Some(false)
        );
        assert!(matches!(
            unavailable_borrow
                .vector_register(13)
                .unwrap()
                .unavailable_source(),
            Some(MachineRspVectorUnavailableSource::Vsub(_))
        ));
        for lane_index in 0..RSP_VECTOR_LANE_COUNT {
            assert!(matches!(
                unavailable_borrow
                    .accumulator_and_flags
                    .accumulator
                    .lane(lane_index)
                    .unwrap()
                    .low()
                    .unavailable_source(),
                Some(MachineRspAccumulatorSliceUnavailableSource::Vsub(_))
            ));
        }
        assert_eq!(
            unavailable_borrow
                .accumulator_and_flags
                .vco
                .carry_or_borrow
                .value(),
            Some(0)
        );

        let mut non_alias_unavailable = MachineRspExecutionState::default();
        non_alias_unavailable.synchronize_pc_write(0x060);
        stage_available_vector_control(&mut non_alias_unavailable, 0);
        stage_available_vector(&mut non_alias_unavailable, 14, [0; 16]);
        let plan = non_alias_unavailable
            .plan_vector_arithmetic(
                0x060,
                non_alias_unavailable
                    .decode(vector_compute_word(RSP_VECTOR_VSUB_FUNCTION, 13, 13, 14, 0))
                    .unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            non_alias_unavailable
                .apply_vector_arithmetic(plan)
                .vector_result_available(),
            Some(false)
        );
    }

    #[test]
    fn rsp_vaddc_proves_all_carry_patterns_and_ignores_old_vco() {
        let word = vector_compute_word(RSP_VECTOR_VADDC_FUNCTION, 13, 13, 14, 0);
        assert_eq!(word, 0x4a0e_6b54);
        for expected_carry in 0_u8..=u8::MAX {
            let mut rsp = MachineRspExecutionState::default();
            rsp.synchronize_pc_write(0x070);
            stage_available_vector_control(&mut rsp, !expected_carry);
            let source_a_lanes = core::array::from_fn(|lane| {
                if expected_carry & (1 << lane) == 0 {
                    1
                } else {
                    u16::MAX
                }
            });
            let source_b_lanes = core::array::from_fn(|lane| {
                if expected_carry & (1 << lane) == 0 {
                    2
                } else {
                    1
                }
            });
            stage_available_vector(&mut rsp, 13, vector_bytes_from_lanes(source_a_lanes));
            stage_available_vector(&mut rsp, 14, vector_bytes_from_lanes(source_b_lanes));
            let high_middle_before: Vec<_> = rsp
                .accumulator_and_flags
                .accumulator
                .lanes
                .iter()
                .map(|lane| (lane.high.clone(), lane.middle.clone()))
                .collect();
            let vcc_before = rsp.accumulator_and_flags.vcc;
            let vce_before = rsp.accumulator_and_flags.vce;
            let plan = rsp
                .plan_vector_arithmetic(
                    0x070,
                    rsp.decode(word).unwrap(),
                    TEST_INSTRUCTION_PROVENANCE,
                )
                .unwrap();
            assert_eq!(
                rsp.apply_vector_arithmetic(plan),
                MachineRspStepOutcome::VectorVaddcCommitted {
                    instruction_pc: 0x070,
                    destination_vector: 13,
                    result_available: true,
                }
            );
            let expected_low = core::array::from_fn(|lane| {
                if expected_carry & (1 << lane) == 0 {
                    3
                } else {
                    0
                }
            });
            assert_eq!(
                rsp.vector_register(13).unwrap().bytes(),
                Some(&vector_bytes_from_lanes(expected_low))
            );
            assert_eq!(
                rsp.accumulator_and_flags.vco.carry_or_borrow.value(),
                Some(expected_carry)
            );
            assert_eq!(rsp.accumulator_and_flags.vco.not_equal.value(), Some(0));
            for lane_index in 0..RSP_VECTOR_LANE_COUNT {
                let lane = rsp
                    .accumulator_and_flags
                    .accumulator
                    .lane(lane_index)
                    .unwrap();
                assert_eq!(lane.low().value(), Some(expected_low[lane_index]));
                assert_eq!(
                    (&lane.high, &lane.middle),
                    (
                        &high_middle_before[lane_index].0,
                        &high_middle_before[lane_index].1
                    )
                );
            }
            assert_eq!(rsp.accumulator_and_flags.vcc, vcc_before);
            assert_eq!(rsp.accumulator_and_flags.vce, vce_before);
        }
    }

    #[test]
    fn rsp_vaddc_unavailable_alias_and_control_replacement_are_exact() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x070);
        stage_available_vector_control(&mut rsp, 0xff);
        stage_available_vector(&mut rsp, 14, [0x5a; 16]);
        let old_vco = rsp.accumulator_and_flags.vco.clone();
        let vcc_before = rsp.accumulator_and_flags.vcc;
        let vce_before = rsp.accumulator_and_flags.vce;
        let plan = rsp
            .plan_vector_arithmetic(
                0x070,
                rsp.decode(vector_compute_word(
                    RSP_VECTOR_VADDC_FUNCTION,
                    13,
                    13,
                    14,
                    0,
                ))
                .unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        assert_eq!(
            rsp.apply_vector_arithmetic(plan),
            MachineRspStepOutcome::VectorVaddcCommitted {
                instruction_pc: 0x070,
                destination_vector: 13,
                result_available: false,
            }
        );
        let source = match rsp.vector_register(13).unwrap().unavailable_source() {
            Some(MachineRspVectorUnavailableSource::Vaddc(source)) => source,
            other => panic!("unavailable Vaddc lacks exact cause: {other:?}"),
        };
        assert_eq!(source.identity(), MachineRspInstructionIdentity::Vaddc);
        assert_eq!(source.vsub_borrow_input(), None);
        assert!(!source.result_available());
        assert_ne!(rsp.accumulator_and_flags.vco, old_vco);
        assert!(matches!(
            rsp.accumulator_and_flags.vco.carry_or_borrow,
            MachineRspVcoHalfState::Unavailable {
                source: MachineRspVcoHalfUnavailableSource::VaddcCarry(_),
            }
        ));
        assert_eq!(rsp.accumulator_and_flags.vco.not_equal.value(), Some(0));
        assert_eq!(rsp.accumulator_and_flags.vcc, vcc_before);
        assert_eq!(rsp.accumulator_and_flags.vce, vce_before);

        let mut alias = MachineRspExecutionState::default();
        alias.synchronize_pc_write(0x070);
        stage_available_vector(
            &mut alias,
            13,
            vector_bytes_from_lanes([1, 2, 3, 4, 5, 6, 7, 0xffff]),
        );
        let plan = alias
            .plan_vector_arithmetic(
                0x070,
                alias
                    .decode(vector_compute_word(
                        RSP_VECTOR_VADDC_FUNCTION,
                        13,
                        13,
                        13,
                        0,
                    ))
                    .unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap();
        alias.apply_vector_arithmetic(plan);
        assert_eq!(
            alias.vector_register(13).unwrap().bytes(),
            Some(&vector_bytes_from_lanes([2, 4, 6, 8, 10, 12, 14, 0xfffe]))
        );
        assert_eq!(
            alias.accumulator_and_flags.vco.carry_or_borrow.value(),
            Some(0x80)
        );
    }

    #[test]
    fn rsp_bgez_decode_condition_target_and_delay_context_are_exact() {
        assert_eq!(bgez_word(3, -3), 0x0461_fffd);
        for (source_value, taken) in [(1, true), (0, true), (0x8000_0000, false)] {
            let mut rsp = MachineRspExecutionState::default();
            rsp.synchronize_pc_write(0x06c);
            stage_available_scalar(&mut rsp, 3, source_value);
            let plan = rsp
                .plan_branch(
                    0x06c,
                    rsp.decode(bgez_word(3, -3)).unwrap(),
                    TEST_INSTRUCTION_PROVENANCE,
                )
                .unwrap();
            assert_eq!(
                rsp.apply_branch(plan),
                MachineRspStepOutcome::ScalarBgezCommitted {
                    instruction_pc: 0x06c,
                    delay_slot_pc: 0x070,
                    target_pc: 0x064,
                    taken,
                }
            );
            let delay = rsp.delay_slot_context().unwrap();
            assert_eq!(delay.identity(), MachineRspInstructionIdentity::Bgez);
            assert_eq!(delay.owner_pc(), 0x06c);
            assert_eq!(delay.delay_slot_pc(), 0x070);
            assert_eq!(delay.target_pc(), 0x064);
            assert_eq!(delay.taken(), taken);
            assert_eq!(rsp.next_pc(), Some(if taken { 0x064 } else { 0x074 }));
        }

        let unavailable = MachineRspExecutionState::default();
        assert_eq!(
            unavailable
                .plan_branch(
                    0x06c,
                    unavailable.decode(bgez_word(3, -3)).unwrap(),
                    TEST_INSTRUCTION_PROVENANCE,
                )
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::BgezSourceUnavailable { source_gpr: 3 }
        );
        assert_eq!(
            unavailable
                .decode(((RSP_SCALAR_REGIMM_OPCODE as u32) << 26) | (3 << 21) | (2 << 16) | 0xfffd)
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::UnsupportedRegimmSelector { selector: 2 }
        );
    }

    #[test]
    fn rsp_vsub_vaddc_element_and_delay_slot_rejections_are_atomic() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x060);
        let before = rsp.clone();
        let decoded = rsp
            .decode(vector_compute_word(RSP_VECTOR_VSUB_FUNCTION, 13, 13, 13, 1))
            .unwrap();
        assert_eq!(
            rsp.plan_vector_arithmetic(0x060, decoded, TEST_INSTRUCTION_PROVENANCE)
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::VsubElementUnsupported { element: 1 }
        );
        assert_eq!(rsp, before);

        let decoded = rsp
            .decode(vector_compute_word(
                RSP_VECTOR_VADDC_FUNCTION,
                13,
                13,
                14,
                15,
            ))
            .unwrap();
        assert_eq!(
            rsp.plan_vector_arithmetic(0x060, decoded, TEST_INSTRUCTION_PROVENANCE)
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::VaddcElementUnsupported { element: 15 }
        );
        assert_eq!(rsp, before);

        stage_available_scalar(&mut rsp, 3, 1);
        rsp.stage_delay_for_test(0x050);
        let committed_branch_state = rsp.clone();
        assert_eq!(
            rsp.plan_branch(
                0x06c,
                rsp.decode(bgez_word(3, -3)).unwrap(),
                TEST_INSTRUCTION_PROVENANCE,
            )
            .unwrap_err()
            .reason(),
            MachineRspStepRejectionReason::ControlFlowInDelaySlot { owner_pc: 0x050 }
        );
        assert_eq!(rsp, committed_branch_state);
        assert_eq!(
            rsp.decode(vector_compute_word(0x10, 13, 13, 14, 0))
                .unwrap_err()
                .reason(),
            MachineRspStepRejectionReason::UnrepresentedInstruction {
                class: MachineRspUnrepresentedInstructionClass::Vector,
            }
        );
    }

    #[test]
    fn rsp_vector_load_store_consumer_and_other_scalar_memory_frontiers_remain_closed() {
        let rsp = MachineRspExecutionState::default();
        let other_vector_load = ((RSP_VECTOR_LOAD_OPCODE as u32) << 26) | (3 << 11);
        assert_eq!(
            rsp.decode(other_vector_load).unwrap_err().reason(),
            MachineRspStepRejectionReason::VectorLoadUnsupported { subopcode: 3 }
        );
        assert_eq!(
            rsp.decode(0xe800_0000).unwrap_err().reason(),
            MachineRspStepRejectionReason::VectorStoreUnsupported
        );
        assert_eq!(
            rsp.decode(0x4a00_0000).unwrap_err().reason(),
            MachineRspStepRejectionReason::UnrepresentedInstruction {
                class: MachineRspUnrepresentedInstructionClass::Vector,
            }
        );
        assert_eq!(
            rsp.decode(0x8004_0040).unwrap_err().reason(),
            MachineRspStepRejectionReason::ScalarLoadUnsupported { opcode: 0x20 }
        );
        assert_eq!(
            rsp.decode(0xac04_0040).unwrap_err().reason(),
            MachineRspStepRejectionReason::ScalarStoreUnsupported { opcode: 0x2b }
        );
    }
}
