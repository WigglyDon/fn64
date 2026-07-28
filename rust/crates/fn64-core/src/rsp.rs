use core::fmt;

use crate::sp::{MachineSpDramAddressSource, MachineSpSemaphoreSource};
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
pub const RSP_COP0_SP_SEMAPHORE_INDEX: u8 = 7;
pub const RSP_SCALAR_XORI_OPCODE: u8 = 0x0e;
pub const RSP_SCALAR_LUI_OPCODE: u8 = 0x0f;
pub const RSP_SCALAR_LW_OPCODE: u8 = 0x23;
pub const RSP_SCALAR_LW_BYTE_COUNT: usize = 4;
pub const RSP_VECTOR_LOAD_OPCODE: u8 = 0x32;
pub const RSP_VECTOR_LQV_SUBOPCODE: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspUnavailableSource {
    ConstructionOrReset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspControlRegister {
    SpMemoryAddress,
    SpDramAddress,
    SpReadLength,
    SpSemaphore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspMfc0ControlSource {
    SpDramAddress {
        value: u32,
        source: MachineSpDramAddressSource,
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
            Self::SpSemaphore { .. } => MachineRspControlRegister::SpSemaphore,
        }
    }

    pub const fn result_value(self) -> u32 {
        match self {
            Self::SpDramAddress { value, .. } => value,
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
    Mfc0(MachineRspMfc0ResultSource),
    Lw(Box<MachineRspScalarLwSource>),
    Xori(Box<MachineRspXoriSource>),
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineRspVectorUnavailableSource {
    ConstructionOrReset,
    Lqv(Box<MachineRspLqvSource>),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspAccumulatorAndFlagsState {
    Unavailable { source: MachineRspUnavailableSource },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRspDelaySlotContext {
    owner_pc: u16,
}

impl MachineRspDelaySlotContext {
    #[cfg(test)]
    pub(crate) const fn new(owner_pc: u16) -> Self {
        Self { owner_pc }
    }

    pub const fn owner_pc(self) -> u16 {
        self.owner_pc
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspInstructionIdentity {
    Mfc0,
    Mtc0,
    Xori,
    Lqv,
    Lw,
    Nop,
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
enum MachineRspLastInstructionDestination {
    ScalarMfc0 {
        destination_gpr: u8,
        control_register: MachineRspControlRegister,
    },
    VectorLqv {
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
    ScalarXori {
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
            MachineRspLastInstructionDestination::ScalarXori { destination_gpr } => {
                Some(destination_gpr)
            }
            MachineRspLastInstructionDestination::VectorLqv { .. }
            | MachineRspLastInstructionDestination::ScalarMtc0 { .. }
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
            | MachineRspLastInstructionDestination::ScalarLw { .. }
            | MachineRspLastInstructionDestination::ScalarXori { .. }
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
            | MachineRspLastInstructionDestination::ScalarXori { .. }
            | MachineRspLastInstructionDestination::None => None,
            MachineRspLastInstructionDestination::VectorLqv { destination_vector } => {
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
    ScalarXoriCommitted {
        instruction_pc: u16,
        destination_gpr: u8,
        result_value: u32,
    },
    NopCommitted {
        instruction_pc: u16,
    },
}

impl MachineRspStepOutcome {
    pub const fn identity(self) -> MachineRspInstructionIdentity {
        match self {
            Self::ScalarMfc0Committed { .. } => MachineRspInstructionIdentity::Mfc0,
            Self::ScalarMtc0Committed { .. } => MachineRspInstructionIdentity::Mtc0,
            Self::ScalarXoriCommitted { .. } => MachineRspInstructionIdentity::Xori,
            Self::VectorLqvCommitted { .. } => MachineRspInstructionIdentity::Lqv,
            Self::ScalarLwCommitted { .. } => MachineRspInstructionIdentity::Lw,
            Self::NopCommitted { .. } => MachineRspInstructionIdentity::Nop,
        }
    }

    pub const fn instruction_pc(self) -> u16 {
        match self {
            Self::ScalarMfc0Committed { instruction_pc, .. }
            | Self::ScalarMtc0Committed { instruction_pc, .. }
            | Self::ScalarXoriCommitted { instruction_pc, .. }
            | Self::VectorLqvCommitted { instruction_pc, .. }
            | Self::ScalarLwCommitted { instruction_pc, .. }
            | Self::NopCommitted { instruction_pc } => instruction_pc,
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
            | Self::ScalarXoriCommitted {
                destination_gpr, ..
            } => Some(destination_gpr),
            Self::ScalarMtc0Committed { .. }
            | Self::VectorLqvCommitted { .. }
            | Self::NopCommitted { .. } => None,
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
            | Self::ScalarXoriCommitted { .. }
            | Self::ScalarLwCommitted { .. }
            | Self::NopCommitted { .. } => None,
        }
    }

    pub const fn result_value(self) -> Option<u32> {
        match self {
            Self::ScalarMfc0Committed { result_value, .. }
            | Self::ScalarLwCommitted { result_value, .. }
            | Self::ScalarXoriCommitted { result_value, .. } => Some(result_value),
            Self::ScalarMtc0Committed { .. }
            | Self::VectorLqvCommitted { .. }
            | Self::NopCommitted { .. } => None,
        }
    }

    pub const fn destination_vector(self) -> Option<u8> {
        match self {
            Self::ScalarMfc0Committed { .. }
            | Self::ScalarMtc0Committed { .. }
            | Self::ScalarXoriCommitted { .. }
            | Self::ScalarLwCommitted { .. }
            | Self::NopCommitted { .. } => None,
            Self::VectorLqvCommitted {
                destination_vector, ..
            } => Some(destination_vector),
        }
    }

    pub const fn local_dmem_address(self) -> Option<u16> {
        match self {
            Self::ScalarMfc0Committed { .. }
            | Self::ScalarMtc0Committed { .. }
            | Self::ScalarXoriCommitted { .. }
            | Self::NopCommitted { .. } => None,
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
            | Self::ScalarXoriCommitted { .. } => None,
            Self::ScalarLwCommitted { .. } | Self::NopCommitted { .. } => None,
            Self::VectorLqvCommitted {
                result_available, ..
            } => Some(result_available),
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
    Mtc0DmaRecordCapacityExhausted,
    Mtc0DmaAddressUnavailable,
    Mtc0DmaRdramRangeRejected {
        physical_address: u32,
    },
    XoriSourceUnavailable {
        source_gpr: u8,
    },
    ScalarLuiUnsupported,
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
    ScalarSllUnsupported,
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
            MachineRspStepRejectionReason::XoriSourceUnavailable { source_gpr } => {
                write!(f, "RSP Xori scalar source r{source_gpr} is unavailable")
            }
            MachineRspStepRejectionReason::ScalarLuiUnsupported => {
                write!(f, "RSP scalar Lui is not represented")
            }
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
            MachineRspStepRejectionReason::ScalarSllUnsupported => {
                write!(f, "RSP scalar Sll is not represented beyond the exact raw-zero Nop")
            }
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

    pub(crate) const fn source_index(&self) -> usize {
        self.source_index
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
pub(crate) enum MachineRspDecodedInstruction {
    Mfc0 {
        destination_gpr: u8,
        control_register: MachineRspControlRegister,
    },
    Mtc0 {
        source_gpr: u8,
        control_register: MachineRspControlRegister,
    },
    Xori {
        source_gpr: u8,
        destination_gpr: u8,
        immediate: u16,
    },
    Lqv {
        base_gpr: u8,
        destination_vector: u8,
        element: u8,
        signed_offset: i8,
    },
    Lw {
        base_gpr: u8,
        destination_gpr: u8,
        signed_offset: i16,
    },
    Nop,
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
    pub(crate) fn scalar_register(&self, index: usize) -> Option<MachineRspScalarRegisterState> {
        self.scalar_registers.get(index).cloned()
    }

    pub(crate) const fn next_pc(&self) -> Option<u16> {
        self.next_pc
    }

    pub(crate) const fn delay_slot_context(&self) -> Option<MachineRspDelaySlotContext> {
        self.delay_slot_context
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

    pub(crate) const fn accumulator_and_flags(&self) -> MachineRspAccumulatorAndFlagsState {
        self.accumulator_and_flags
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
        let opcode = (raw_word >> 26) as u8;
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
        if opcode == RSP_SCALAR_XORI_OPCODE {
            return Ok(MachineRspDecodedInstruction::Xori {
                source_gpr: ((raw_word >> 21) & 0x1f) as u8,
                destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                immediate: raw_word as u16,
            });
        }
        if opcode == RSP_SCALAR_LUI_OPCODE {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ScalarLuiUnsupported,
            ));
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
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ScalarSllUnsupported,
            ));
        }
        let class = if opcode == 0x12 {
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

    #[cfg(test)]
    pub(crate) fn stage_delay_for_test(&mut self, owner_pc: u16) {
        self.delay_slot_context = Some(MachineRspDelaySlotContext::new(owner_pc));
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
            accumulator_and_flags: MachineRspAccumulatorAndFlagsState::Unavailable {
                source: MachineRspUnavailableSource::ConstructionOrReset,
            },
            mtc0_sources: Vec::new(),
        }
    }
}

pub(crate) const fn sequential_local_pc(pc: u16) -> u16 {
    pc.wrapping_add(4) & RSP_LOCAL_ADDRESS_MASK & !RSP_INSTRUCTION_ALIGNMENT_MASK
}

fn classify_instruction_source(
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

    const fn xori_word(source_gpr: u8, destination_gpr: u8, immediate: u16) -> u32 {
        ((RSP_SCALAR_XORI_OPCODE as u32) << 26)
            | ((source_gpr as u32) << 21)
            | ((destination_gpr as u32) << 16)
            | immediate as u32
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
        assert!(matches!(
            rsp.accumulator_and_flags(),
            MachineRspAccumulatorAndFlagsState::Unavailable { .. }
        ));
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
            rsp.decode(0x3803_0180),
            Ok(MachineRspDecodedInstruction::Xori {
                source_gpr: 0,
                destination_gpr: 3,
                immediate: 0x0180,
            })
        );
        assert_eq!(rsp.decode(0), Ok(MachineRspDecodedInstruction::Nop));
        assert_eq!(
            rsp.decode(0x0000_0040).unwrap_err().reason(),
            MachineRspStepRejectionReason::ScalarSllUnsupported
        );
        assert_eq!(
            rsp.decode(mtc0_word(0, 3)).unwrap_err().reason(),
            MachineRspStepRejectionReason::UnsupportedMtc0ControlRegister { register_index: 3 }
        );
        assert_eq!(
            rsp.decode(mtc0_word(0, 0) | 1).unwrap_err().reason(),
            MachineRspStepRejectionReason::MalformedMtc0Encoding
        );
        assert_eq!(
            rsp.decode(0x3c05_0020).unwrap_err().reason(),
            MachineRspStepRejectionReason::ScalarLuiUnsupported
        );
    }

    #[test]
    fn rsp_mtc0_consumes_available_source_records_exact_lineage_and_commits_once() {
        let mut rsp = MachineRspExecutionState::default();
        rsp.synchronize_pc_write(0x018);
        stage_available_scalar(&mut rsp, 3, 0x0000_0187);
        let scalar_before = rsp.scalar_registers.clone();
        let vector_before = rsp.vector_unit.clone();
        let accumulator_before = rsp.accumulator_and_flags;
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
        let accumulator_before = rsp.accumulator_and_flags;
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
        let accumulator_before = rsp.accumulator_and_flags;
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
        let accumulator_before = rsp.accumulator_and_flags;

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
