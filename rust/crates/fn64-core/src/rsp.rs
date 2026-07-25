use core::fmt;

use crate::sp::{MachineSpDramAddressSource, MachineSpSemaphoreSource};
use crate::sp_imem::SpImemByteProvenance;

pub const RSP_SCALAR_REGISTER_COUNT: usize = 32;
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
pub enum MachineRspVectorUnitState {
    Unavailable { source: MachineRspUnavailableSource },
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
pub struct MachineRspLastInstructionState {
    instruction_pc: u16,
    identity: MachineRspInstructionIdentity,
    destination_gpr: u8,
    control_register: MachineRspControlRegister,
    byte_provenance: [SpImemByteProvenance; 4],
}

impl MachineRspLastInstructionState {
    pub const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub const fn identity(self) -> MachineRspInstructionIdentity {
        self.identity
    }

    pub const fn destination_gpr(self) -> u8 {
        self.destination_gpr
    }

    pub const fn control_register(self) -> MachineRspControlRegister {
        self.control_register
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
}

impl MachineRspStepOutcome {
    pub const fn identity(self) -> MachineRspInstructionIdentity {
        match self {
            Self::ScalarMfc0Committed { .. } => MachineRspInstructionIdentity::Mfc0,
        }
    }

    pub const fn instruction_pc(self) -> u16 {
        match self {
            Self::ScalarMfc0Committed { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn destination_gpr(self) -> u8 {
        match self {
            Self::ScalarMfc0Committed {
                destination_gpr, ..
            } => destination_gpr,
        }
    }

    pub const fn control_register(self) -> MachineRspControlRegister {
        match self {
            Self::ScalarMfc0Committed {
                control_register, ..
            } => control_register,
        }
    }

    pub const fn result_value(self) -> u32 {
        match self {
            Self::ScalarMfc0Committed { result_value, .. } => result_value,
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
pub struct MachineRspLqvFrontier {
    base_gpr: u8,
    destination_vector: u8,
    element: u8,
    signed_offset: i8,
}

impl MachineRspLqvFrontier {
    #[cfg(test)]
    pub(crate) const fn new(
        base_gpr: u8,
        destination_vector: u8,
        element: u8,
        signed_offset: i8,
    ) -> Self {
        Self {
            base_gpr,
            destination_vector,
            element,
            signed_offset,
        }
    }

    pub const fn base_gpr(self) -> u8 {
        self.base_gpr
    }

    pub const fn destination_vector(self) -> u8 {
        self.destination_vector
    }

    pub const fn element(self) -> u8 {
        self.element
    }

    pub const fn signed_offset(self) -> i8 {
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
    VectorLqvUnrepresented {
        frontier: MachineRspLqvFrontier,
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
            MachineRspStepRejectionReason::VectorLqvUnrepresented { .. } => {
                write!(f, "RSP vector Lqv is identified but not represented")
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
pub(crate) enum MachineRspDecodedInstruction {
    Mfc0 {
        destination_gpr: u8,
        control_register: MachineRspControlRegister,
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

    pub(crate) const fn vector_unit(&self) -> MachineRspVectorUnitState {
        self.vector_unit
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
        if opcode == RSP_VECTOR_LOAD_OPCODE
            && ((raw_word >> 11) & 0x1f) as u8 == RSP_VECTOR_LQV_SUBOPCODE
        {
            let raw_offset = (raw_word & 0x7f) as u8;
            let signed_offset = ((raw_offset << 1) as i8) >> 1;
            return Err(MachineRspStepRejection::new(
                MachineRspStepRejectionReason::VectorLqvUnrepresented {
                    frontier: MachineRspLqvFrontier {
                        base_gpr: ((raw_word >> 21) & 0x1f) as u8,
                        destination_vector: ((raw_word >> 16) & 0x1f) as u8,
                        element: ((raw_word >> 7) & 0x0f) as u8,
                        signed_offset,
                    },
                },
            ));
        }
        let class = if opcode == RSP_VECTOR_LOAD_OPCODE {
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
        } = decoded;
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
            destination_gpr: plan.destination_gpr,
            control_register: plan.control_source.register(),
            byte_provenance: plan.byte_provenance,
        });
        MachineRspStepOutcome::ScalarMfc0Committed {
            instruction_pc: plan.instruction_pc,
            destination_gpr: plan.destination_gpr,
            control_register: plan.control_source.register(),
            result_value,
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
            vector_unit: MachineRspVectorUnitState::Unavailable {
                source: MachineRspUnavailableSource::ConstructionOrReset,
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

    #[test]
    fn rsp_foundation_starts_with_only_scalar_zero_available() {
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
        assert!(matches!(
            rsp.vector_unit(),
            MachineRspVectorUnitState::Unavailable { .. }
        ));
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
            rsp.decode(0xc80c_2000).unwrap_err().reason(),
            MachineRspStepRejectionReason::VectorLqvUnrepresented {
                frontier: MachineRspLqvFrontier {
                    base_gpr: 0,
                    destination_vector: 12,
                    element: 0,
                    signed_offset: 0,
                },
            }
        );
    }
}
