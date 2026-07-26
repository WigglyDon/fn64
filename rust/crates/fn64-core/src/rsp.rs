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
pub const RSP_COP0_SP_DRAM_ADDRESS_INDEX: u8 = 1;
pub const RSP_COP0_SP_SEMAPHORE_INDEX: u8 = 7;
pub const RSP_VECTOR_LOAD_OPCODE: u8 = 0x32;
pub const RSP_VECTOR_LQV_SUBOPCODE: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspUnavailableSource {
    ConstructionOrReset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspControlRegister {
    SpDramAddress,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspScalarRegisterSource {
    ArchitecturalZero,
    Mfc0(MachineRspMfc0ResultSource),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub const fn value(self) -> Option<u32> {
        match self {
            Self::Available { value, .. } => Some(value),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn source(self) -> Option<MachineRspScalarRegisterSource> {
        match self {
            Self::Available { source, .. } => Some(source),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available { .. })
    }

    pub const fn unavailable_source(self) -> Option<MachineRspUnavailableSource> {
        match self {
            Self::Unavailable { source } => Some(source),
            Self::Available { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub fn instruction_source(self) -> MachineRspInstructionSource {
        classify_instruction_source(self.instruction_provenance)
    }

    pub const fn base_gpr(self) -> u8 {
        self.base_gpr
    }

    pub const fn base_value(self) -> u32 {
        self.base_value
    }

    pub const fn base_source(self) -> MachineRspScalarRegisterSource {
        self.base_source
    }

    pub const fn element(self) -> u8 {
        self.element
    }

    pub const fn signed_offset(self) -> i8 {
        self.signed_offset
    }

    pub const fn local_dmem_address(self) -> u16 {
        self.local_dmem_address
    }

    pub const fn dmem_knowledge(
        self,
    ) -> [MachineSpDmemByteKnowledgeDescriptor; RSP_VECTOR_REGISTER_BYTE_COUNT] {
        self.dmem_knowledge
    }

    #[cfg(test)]
    pub(crate) const fn instruction_provenance(self) -> [SpImemByteProvenance; 4] {
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
    Lqv,
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
            MachineRspLastInstructionDestination::VectorLqv { .. } => None,
        }
    }

    pub const fn control_register(self) -> Option<MachineRspControlRegister> {
        match self.destination {
            MachineRspLastInstructionDestination::ScalarMfc0 {
                control_register, ..
            } => Some(control_register),
            MachineRspLastInstructionDestination::VectorLqv { .. } => None,
        }
    }

    pub const fn destination_vector(self) -> Option<u8> {
        match self.destination {
            MachineRspLastInstructionDestination::ScalarMfc0 { .. } => None,
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
}

impl MachineRspStepOutcome {
    pub const fn identity(self) -> MachineRspInstructionIdentity {
        match self {
            Self::ScalarMfc0Committed { .. } => MachineRspInstructionIdentity::Mfc0,
            Self::VectorLqvCommitted { .. } => MachineRspInstructionIdentity::Lqv,
        }
    }

    pub const fn instruction_pc(self) -> u16 {
        match self {
            Self::ScalarMfc0Committed { instruction_pc, .. }
            | Self::VectorLqvCommitted { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn destination_gpr(self) -> Option<u8> {
        match self {
            Self::ScalarMfc0Committed {
                destination_gpr, ..
            } => Some(destination_gpr),
            Self::VectorLqvCommitted { .. } => None,
        }
    }

    pub const fn control_register(self) -> Option<MachineRspControlRegister> {
        match self {
            Self::ScalarMfc0Committed {
                control_register, ..
            } => Some(control_register),
            Self::VectorLqvCommitted { .. } => None,
        }
    }

    pub const fn result_value(self) -> Option<u32> {
        match self {
            Self::ScalarMfc0Committed { result_value, .. } => Some(result_value),
            Self::VectorLqvCommitted { .. } => None,
        }
    }

    pub const fn destination_vector(self) -> Option<u8> {
        match self {
            Self::ScalarMfc0Committed { .. } => None,
            Self::VectorLqvCommitted {
                destination_vector, ..
            } => Some(destination_vector),
        }
    }

    pub const fn local_dmem_address(self) -> Option<u16> {
        match self {
            Self::ScalarMfc0Committed { .. } => None,
            Self::VectorLqvCommitted {
                local_dmem_address, ..
            } => Some(local_dmem_address),
        }
    }

    pub const fn vector_result_available(self) -> Option<bool> {
        match self {
            Self::ScalarMfc0Committed { .. } => None,
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
pub struct MachineRspScalarLwFrontier {
    base_gpr: u8,
    destination_gpr: u8,
    signed_offset: i16,
}

impl MachineRspScalarLwFrontier {
    pub const fn base_gpr(self) -> u8 {
        self.base_gpr
    }

    pub const fn destination_gpr(self) -> u8 {
        self.destination_gpr
    }

    pub const fn signed_offset(self) -> i16 {
        self.signed_offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRspStepRejectionReason {
    SingleStepUnsupported,
    Fetch(MachineRspFetchRejection),
    MalformedMfc0Encoding,
    UnsupportedCop0Register {
        register_index: u8,
    },
    Mtc0Unsupported,
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
    ScalarLwUnrepresented {
        frontier: MachineRspScalarLwFrontier,
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
            MachineRspStepRejectionReason::Mtc0Unsupported => {
                write!(f, "RSP Mtc0 is not represented")
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
            MachineRspStepRejectionReason::ScalarLwUnrepresented { .. } => {
                write!(f, "RSP scalar Lw is identified but not represented")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub(crate) const fn local_dmem_address(self) -> u16 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineRspDecodedInstruction {
    Mfc0 {
        destination_gpr: u8,
        control_register: MachineRspControlRegister,
    },
    Lqv {
        base_gpr: u8,
        destination_vector: u8,
        element: u8,
        signed_offset: i8,
    },
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
}

impl MachineRspExecutionState {
    pub(crate) fn scalar_register(&self, index: usize) -> Option<MachineRspScalarRegisterState> {
        self.scalar_registers.get(index).copied()
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

    pub(crate) fn synchronize_pc_write(&mut self, current_pc: u16) {
        self.next_pc = Some(sequential_local_pc(current_pc));
        self.delay_slot_context = None;
    }

    pub(crate) fn decode(
        &self,
        raw_word: u32,
    ) -> Result<MachineRspDecodedInstruction, MachineRspStepRejection> {
        let opcode = (raw_word >> 26) as u8;
        if opcode == RSP_COP0_OPCODE {
            let transfer_selector = ((raw_word >> 21) & 0x1f) as u8;
            if transfer_selector == RSP_COP0_MTC0_TRANSFER_SELECTOR {
                return Err(MachineRspStepRejection::new(
                    MachineRspStepRejectionReason::Mtc0Unsupported,
                ));
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
        if opcode == 0x23 {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::ScalarLwUnrepresented {
                    frontier: MachineRspScalarLwFrontier {
                        base_gpr: ((raw_word >> 21) & 0x1f) as u8,
                        destination_gpr: ((raw_word >> 16) & 0x1f) as u8,
                        signed_offset: raw_word as u16 as i16,
                    },
                },
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
        let base_state = self.scalar_registers[usize::from(base_gpr)];
        let MachineRspScalarRegisterState::Available {
            value: base_value,
            source: base_source,
        } = base_state
        else {
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::LqvScalarBaseUnavailable { base_gpr },
            ));
        };
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
            base_source: address.base_source,
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
        let mut scalar_registers = [unavailable; RSP_SCALAR_REGISTER_COUNT];
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
    fn rsp_mfc0_and_lqv_decode_boundary_is_exact() {
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
        let scalar_before = rsp.scalar_registers;
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
                **source
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
    fn rsp_vector_load_store_consumer_and_scalar_lw_frontiers_remain_closed() {
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
            rsp.decode(0x8c04_0040).unwrap_err().reason(),
            MachineRspStepRejectionReason::ScalarLwUnrepresented {
                frontier: MachineRspScalarLwFrontier {
                    base_gpr: 0,
                    destination_gpr: 4,
                    signed_offset: 0x40,
                },
            }
        );
    }
}
