use core::fmt;

use crate::cartridge::Cartridge;
use crate::cpu::address::{
    check_cpu_data_alignment, classify_direct_rdram_address, select_cpu_data_address_error,
    select_cpu_data_address_error_for_access, translate_direct_cpu_physical_address, CpuAddress,
    CpuAddressErrorExceptionEntryError, CpuAddressErrorKind, CpuAddressTarget, CpuDataAccessKind,
    CpuDataAddressError, CpuDataWidth, RdramOffset,
};
use crate::cpu::{
    decode_cpu_instruction_word, identify_cpu_instruction, select_cpu_local_executed_helper, Cpu,
    CpuArithmeticOverflowExceptionEntryError, CpuControlFlowSnapshot, CpuDelaySlotContext,
    CpuInstructionFields, CpuInstructionIdentity, CpuInstructionWord,
    CpuLocalExecutedHelperArithmeticOverflow, CpuLocalExecutedHelperExecutedInstruction,
    CpuLocalExecutedHelperInvocationError, CpuLocalExecutedHelperInvocationOutcome,
    CpuRegisterIndexError, NON_BOOT_RESET_VECTOR_PC,
};
use crate::pif_firmware::{
    MachinePifFirmwareState, PifFirmware, PifFirmwareValidationError, PifIpl2Profile,
};
use crate::rdram::{Rdram, RdramAccessError};
use crate::sp_dmem::{SpDmem, SpDmemOffset, SpDmemReadError, SP_DMEM_SIZE_BYTES};
#[cfg(test)]
use crate::sp_imem::SpImemReadError;
use crate::sp_imem::{SpImem, SpImemOffset, SP_IMEM_SIZE_BYTES};

mod cartridge_bootstrap;
mod rdram_reservation;

pub use cartridge_bootstrap::{
    MachineBootstrapCpuStateKind, MachineBootstrapCpuStateUnavailable, MachineBootstrapGprSource,
    MachineCartridgeBootstrapError, MachineCartridgeBootstrapState,
    MachineCpuInstructionInspection, MachineCpuInstructionSource,
    MachineSpDmemInstructionProvenance, MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC,
    MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC, MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE,
    MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET, MACHINE_GENERAL_PIF_RESET_GPR29_VALUE,
    MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX,
};

use rdram_reservation::CpuRdramReservation;

const CPU_INSTRUCTION_FETCH_WIDTH: usize = 4;
const CPU_DATA_WORD_WIDTH: usize = 4;
const COP0_BAD_VADDR_REGISTER_INDEX: u8 = 8;
const COP0_COUNT_REGISTER_INDEX: u8 = 9;
const COP0_COMPARE_REGISTER_INDEX: u8 = 11;
const COP0_STATUS_REGISTER_INDEX: u8 = 12;
const COP0_CAUSE_REGISTER_INDEX: u8 = 13;
const COP0_EPC_REGISTER_INDEX: u8 = 14;
const SP_DMEM_PHYSICAL_BASE: u32 = 0x0400_0000;
const SP_IMEM_PHYSICAL_BASE: u32 = 0x0400_1000;
const UNAVAILABLE_PIF_ROM_RESET_PHYSICAL_ADDRESS: u32 = 0x1fc0_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectRdramAccessError {
    cpu_address: CpuAddress,
    width: usize,
}

impl DirectRdramAccessError {
    pub const fn cpu_address(&self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn width(&self) -> usize {
        self.width
    }
}

impl fmt::Display for DirectRdramAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "direct RDRAM access unsupported: address={} width={}",
            self.cpu_address.value(),
            self.width
        )
    }
}

impl std::error::Error for DirectRdramAccessError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineLoadWordTarget {
    DirectRdram { offset: RdramOffset },
    SpImem { offset: u32 },
}

impl MachineLoadWordTarget {
    pub const fn direct_rdram_offset(self) -> Option<RdramOffset> {
        match self {
            Self::DirectRdram { offset } => Some(offset),
            Self::SpImem { .. } => None,
        }
    }

    pub const fn sp_imem_offset(self) -> Option<u32> {
        match self {
            Self::SpImem { offset } => Some(offset),
            Self::DirectRdram { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineLoadWordRejectionReason {
    NonDirectUnsupported,
    DirectTargetMiss,
    DirectRdramReadRejected,
    SpImemUnknown { first_unknown_offset: u32 },
    SpImemReadRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineLoadWordRejection {
    fields: CpuInstructionFields,
    effective_address: u64,
    cpu_address: CpuAddress,
    target: Option<MachineLoadWordTarget>,
    reason: MachineLoadWordRejectionReason,
}

impl MachineLoadWordRejection {
    const fn new(
        fields: CpuInstructionFields,
        effective_address: u64,
        cpu_address: CpuAddress,
        target: Option<MachineLoadWordTarget>,
        reason: MachineLoadWordRejectionReason,
    ) -> Self {
        Self {
            fields,
            effective_address,
            cpu_address,
            target,
            reason,
        }
    }

    pub const fn identity(self) -> CpuInstructionIdentity {
        CpuInstructionIdentity::Lw
    }

    pub const fn fields(self) -> CpuInstructionFields {
        self.fields
    }

    pub const fn effective_address(self) -> u64 {
        self.effective_address
    }

    pub const fn cpu_address(self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn target(self) -> Option<MachineLoadWordTarget> {
        self.target
    }

    pub const fn reason(self) -> MachineLoadWordRejectionReason {
        self.reason
    }
}

impl fmt::Display for MachineLoadWordRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lw rejected before mutation: effective_address=0x{:016X} cpu_address=0x{:08X} target={:?} reason={:?}",
            self.effective_address,
            self.cpu_address.value(),
            self.target,
            self.reason
        )
    }
}

impl std::error::Error for MachineLoadWordRejection {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineCpuDataWordTargetError {
    Unaligned { cpu_address: CpuAddress },
    NonDirectUnsupported { cpu_address: CpuAddress },
    DirectTargetMiss { cpu_address: CpuAddress },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineDirectRdramCpuDataAccessError {
    AddressErrorEntered(CpuDataAddressError),
    AddressErrorEntryBlocked {
        address_error: CpuDataAddressError,
        entry_error: CpuAddressErrorExceptionEntryError,
    },
    DirectRdram {
        access_kind: CpuDataAccessKind,
        width: CpuDataWidth,
        source: DirectRdramAccessError,
    },
}

impl MachineDirectRdramCpuDataAccessError {
    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::AddressErrorEntered(address_error) => address_error.address(),
            Self::AddressErrorEntryBlocked { address_error, .. } => address_error.address(),
            Self::DirectRdram { source, .. } => source.cpu_address(),
        }
    }

    pub const fn access_kind(self) -> CpuDataAccessKind {
        match self {
            Self::AddressErrorEntered(address_error) => address_error.access_kind(),
            Self::AddressErrorEntryBlocked { address_error, .. } => address_error.access_kind(),
            Self::DirectRdram { access_kind, .. } => access_kind,
        }
    }

    pub const fn width(self) -> CpuDataWidth {
        match self {
            Self::AddressErrorEntered(address_error) => address_error.width(),
            Self::AddressErrorEntryBlocked { address_error, .. } => address_error.width(),
            Self::DirectRdram { width, .. } => width,
        }
    }

    pub const fn exception_kind(self) -> Option<CpuAddressErrorKind> {
        match self {
            Self::AddressErrorEntered(address_error) => Some(address_error.exception_kind()),
            Self::AddressErrorEntryBlocked { address_error, .. } => {
                Some(address_error.exception_kind())
            }
            Self::DirectRdram { .. } => None,
        }
    }

    pub const fn address_error(self) -> Option<CpuDataAddressError> {
        match self {
            Self::AddressErrorEntered(address_error) => Some(address_error),
            Self::AddressErrorEntryBlocked { address_error, .. } => Some(address_error),
            Self::DirectRdram { .. } => None,
        }
    }

    pub const fn entry_error(self) -> Option<CpuAddressErrorExceptionEntryError> {
        match self {
            Self::AddressErrorEntryBlocked { entry_error, .. } => Some(entry_error),
            Self::AddressErrorEntered(_) | Self::DirectRdram { .. } => None,
        }
    }

    pub const fn direct_rdram_error(self) -> Option<DirectRdramAccessError> {
        match self {
            Self::DirectRdram { source, .. } => Some(source),
            Self::AddressErrorEntered(_) | Self::AddressErrorEntryBlocked { .. } => None,
        }
    }
}

impl fmt::Display for MachineDirectRdramCpuDataAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::AddressErrorEntered(address_error) => {
                write!(f, "direct RDRAM CPU data access entered {address_error}")
            }
            Self::AddressErrorEntryBlocked {
                address_error,
                entry_error,
            } => write!(
                f,
                "direct RDRAM CPU data access blocked {address_error}: {entry_error}"
            ),
            Self::DirectRdram {
                access_kind,
                width,
                source,
            } => write!(
                f,
                "direct RDRAM CPU data {:?} {}-byte access rejected: {}",
                access_kind,
                width.bytes(),
                source
            ),
        }
    }
}

impl std::error::Error for MachineDirectRdramCpuDataAccessError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineDirectRdramCpuInstructionFetchError {
    Unaligned { cpu_address: CpuAddress },
    DirectRdram { source: DirectRdramAccessError },
}

impl MachineDirectRdramCpuInstructionFetchError {
    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::Unaligned { cpu_address } => cpu_address,
            Self::DirectRdram { source } => source.cpu_address(),
        }
    }

    pub const fn width(self) -> usize {
        4
    }

    pub const fn is_unaligned(self) -> bool {
        matches!(self, Self::Unaligned { .. })
    }

    pub const fn direct_rdram_error(self) -> Option<DirectRdramAccessError> {
        match self {
            Self::DirectRdram { source } => Some(source),
            Self::Unaligned { .. } => None,
        }
    }
}

impl fmt::Display for MachineDirectRdramCpuInstructionFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unaligned { cpu_address } => write!(
                f,
                "direct RDRAM CPU instruction fetch requires 4-byte aligned PC: {}",
                cpu_address.value()
            ),
            Self::DirectRdram { source } => {
                write!(f, "direct RDRAM CPU instruction fetch rejected: {source}")
            }
        }
    }
}

impl std::error::Error for MachineDirectRdramCpuInstructionFetchError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDmemCpuInstructionFetchError {
    source: SpDmemReadError,
}

impl MachineSpDmemCpuInstructionFetchError {
    pub const fn source(self) -> SpDmemReadError {
        self.source
    }

    pub const fn offset(self) -> SpDmemOffset {
        self.source.offset()
    }

    pub const fn width(self) -> usize {
        self.source.width()
    }
}

impl fmt::Display for MachineSpDmemCpuInstructionFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SP DMEM CPU instruction fetch rejected: {}", self.source)
    }
}

impl std::error::Error for MachineSpDmemCpuInstructionFetchError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCpuInstructionFetchTarget {
    DirectRdram { offset: RdramOffset },
    SpDmem { offset: SpDmemOffset },
    PifResetUnavailable,
}

impl MachineCpuInstructionFetchTarget {
    pub const fn width(self) -> usize {
        CPU_INSTRUCTION_FETCH_WIDTH
    }

    pub const fn direct_rdram_offset(self) -> Option<RdramOffset> {
        match self {
            Self::DirectRdram { offset } => Some(offset),
            Self::SpDmem { .. } | Self::PifResetUnavailable => None,
        }
    }

    pub const fn sp_dmem_offset(self) -> Option<SpDmemOffset> {
        match self {
            Self::SpDmem { offset } => Some(offset),
            Self::DirectRdram { .. } | Self::PifResetUnavailable => None,
        }
    }

    pub const fn is_pif_reset_unavailable(self) -> bool {
        matches!(self, Self::PifResetUnavailable)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCpuInstructionFetchTargetError {
    Unaligned { cpu_address: CpuAddress },
    NonDirectUnsupported { cpu_address: CpuAddress },
    DirectTargetMiss { cpu_address: CpuAddress },
}

impl MachineCpuInstructionFetchTargetError {
    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::Unaligned { cpu_address }
            | Self::NonDirectUnsupported { cpu_address }
            | Self::DirectTargetMiss { cpu_address } => cpu_address,
        }
    }

    pub const fn width(self) -> usize {
        CPU_INSTRUCTION_FETCH_WIDTH
    }

    pub const fn is_unaligned(self) -> bool {
        matches!(self, Self::Unaligned { .. })
    }

    pub const fn is_non_direct_unsupported(self) -> bool {
        matches!(self, Self::NonDirectUnsupported { .. })
    }

    pub const fn is_direct_target_miss(self) -> bool {
        matches!(self, Self::DirectTargetMiss { .. })
    }
}

impl fmt::Display for MachineCpuInstructionFetchTargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unaligned { cpu_address } => write!(
                f,
                "CPU instruction fetch target requires 4-byte aligned PC: {}",
                cpu_address.value()
            ),
            Self::NonDirectUnsupported { cpu_address } => write!(
                f,
                "CPU instruction fetch target unsupported for non-direct address: {}",
                cpu_address.value()
            ),
            Self::DirectTargetMiss { cpu_address } => write!(
                f,
                "CPU instruction fetch direct target miss: address={} width={}",
                cpu_address.value(),
                CPU_INSTRUCTION_FETCH_WIDTH
            ),
        }
    }
}

impl std::error::Error for MachineCpuInstructionFetchTargetError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCpuInstructionFetchError {
    Unaligned {
        cpu_address: CpuAddress,
    },
    NonDirectUnsupported {
        cpu_address: CpuAddress,
    },
    DirectTargetMiss {
        cpu_address: CpuAddress,
    },
    PifResetUnavailable {
        cpu_address: CpuAddress,
    },
    DirectRdram {
        cpu_address: CpuAddress,
        source: MachineDirectRdramCpuInstructionFetchError,
    },
    SpDmem {
        cpu_address: CpuAddress,
        offset: SpDmemOffset,
        source: MachineSpDmemCpuInstructionFetchError,
    },
}

impl MachineCpuInstructionFetchError {
    pub const fn cpu_address(self) -> CpuAddress {
        match self {
            Self::Unaligned { cpu_address }
            | Self::NonDirectUnsupported { cpu_address }
            | Self::DirectTargetMiss { cpu_address }
            | Self::PifResetUnavailable { cpu_address }
            | Self::DirectRdram { cpu_address, .. }
            | Self::SpDmem { cpu_address, .. } => cpu_address,
        }
    }

    pub const fn width(self) -> usize {
        CPU_INSTRUCTION_FETCH_WIDTH
    }

    pub const fn is_unaligned(self) -> bool {
        matches!(self, Self::Unaligned { .. })
    }

    pub const fn is_non_direct_unsupported(self) -> bool {
        matches!(self, Self::NonDirectUnsupported { .. })
    }

    pub const fn is_direct_target_miss(self) -> bool {
        matches!(self, Self::DirectTargetMiss { .. })
    }

    pub const fn is_pif_reset_unavailable(self) -> bool {
        matches!(self, Self::PifResetUnavailable { .. })
    }

    pub const fn direct_rdram_error(self) -> Option<MachineDirectRdramCpuInstructionFetchError> {
        match self {
            Self::DirectRdram { source, .. } => Some(source),
            Self::Unaligned { .. }
            | Self::NonDirectUnsupported { .. }
            | Self::DirectTargetMiss { .. }
            | Self::PifResetUnavailable { .. }
            | Self::SpDmem { .. } => None,
        }
    }

    pub const fn sp_dmem_error(self) -> Option<MachineSpDmemCpuInstructionFetchError> {
        match self {
            Self::SpDmem { source, .. } => Some(source),
            Self::Unaligned { .. }
            | Self::NonDirectUnsupported { .. }
            | Self::DirectTargetMiss { .. }
            | Self::PifResetUnavailable { .. }
            | Self::DirectRdram { .. } => None,
        }
    }

    pub const fn sp_dmem_offset(self) -> Option<SpDmemOffset> {
        match self {
            Self::SpDmem { offset, .. } => Some(offset),
            Self::Unaligned { .. }
            | Self::NonDirectUnsupported { .. }
            | Self::DirectTargetMiss { .. }
            | Self::PifResetUnavailable { .. }
            | Self::DirectRdram { .. } => None,
        }
    }

    const fn from_target_error(error: MachineCpuInstructionFetchTargetError) -> Self {
        match error {
            MachineCpuInstructionFetchTargetError::Unaligned { cpu_address } => {
                Self::Unaligned { cpu_address }
            }
            MachineCpuInstructionFetchTargetError::NonDirectUnsupported { cpu_address } => {
                Self::NonDirectUnsupported { cpu_address }
            }
            MachineCpuInstructionFetchTargetError::DirectTargetMiss { cpu_address } => {
                Self::DirectTargetMiss { cpu_address }
            }
        }
    }
}

impl fmt::Display for MachineCpuInstructionFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unaligned { cpu_address } => write!(
                f,
                "CPU instruction fetch requires 4-byte aligned PC: {}",
                cpu_address.value()
            ),
            Self::NonDirectUnsupported { cpu_address } => write!(
                f,
                "CPU instruction fetch unsupported for non-direct address: {}",
                cpu_address.value()
            ),
            Self::DirectTargetMiss { cpu_address } => write!(
                f,
                "CPU instruction fetch direct target miss: address={} width={}",
                cpu_address.value(),
                CPU_INSTRUCTION_FETCH_WIDTH
            ),
            Self::PifResetUnavailable { cpu_address } => write!(
                f,
                "CPU instruction fetch unavailable PIF reset target: {}",
                cpu_address.value()
            ),
            Self::DirectRdram { source, .. } => {
                write!(
                    f,
                    "CPU instruction fetch direct RDRAM source rejected: {source}"
                )
            }
            Self::SpDmem { source, .. } => {
                write!(f, "CPU instruction fetch SP DMEM source rejected: {source}")
            }
        }
    }
}

impl std::error::Error for MachineCpuInstructionFetchError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineInstructionFetchAddressErrorSource {
    Unaligned,
    DirectTargetMiss,
    PifResetUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineInstructionFetchAddressErrorPlan {
    fetch_error: MachineCpuInstructionFetchError,
    source: MachineInstructionFetchAddressErrorSource,
    exception_kind: CpuAddressErrorKind,
}

impl MachineInstructionFetchAddressErrorPlan {
    const fn new(
        fetch_error: MachineCpuInstructionFetchError,
        source: MachineInstructionFetchAddressErrorSource,
    ) -> Self {
        Self {
            fetch_error,
            source,
            exception_kind: CpuAddressErrorKind::AddressErrorLoad,
        }
    }

    pub const fn fetch_error(self) -> MachineCpuInstructionFetchError {
        self.fetch_error
    }

    pub const fn source(self) -> MachineInstructionFetchAddressErrorSource {
        self.source
    }

    pub const fn cpu_address(self) -> CpuAddress {
        self.fetch_error.cpu_address()
    }

    pub const fn bad_vaddr(self) -> CpuAddress {
        self.cpu_address()
    }

    pub const fn width(self) -> usize {
        self.fetch_error.width()
    }

    pub const fn exception_kind(self) -> CpuAddressErrorKind {
        self.exception_kind
    }

    pub const fn cause_exception_code(self) -> u8 {
        self.exception_kind.cause_exception_code()
    }
}

impl fmt::Display for MachineInstructionFetchAddressErrorPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU instruction fetch {:?} selected {} for {}-byte address error at {}",
            self.source,
            self.exception_kind.short_name(),
            self.width(),
            self.cpu_address().value()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineInstructionFetchAddressErrorPlanError {
    fetch_error: MachineCpuInstructionFetchError,
}

impl MachineInstructionFetchAddressErrorPlanError {
    pub const fn fetch_error(self) -> MachineCpuInstructionFetchError {
        self.fetch_error
    }

    pub const fn cpu_address(self) -> CpuAddress {
        self.fetch_error.cpu_address()
    }

    pub const fn width(self) -> usize {
        self.fetch_error.width()
    }
}

impl fmt::Display for MachineInstructionFetchAddressErrorPlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU instruction fetch fault does not select local address-error entry: {}",
            self.fetch_error
        )
    }
}

impl std::error::Error for MachineInstructionFetchAddressErrorPlanError {}

pub fn select_cpu_instruction_fetch_address_error(
    fetch_error: MachineCpuInstructionFetchError,
) -> Result<MachineInstructionFetchAddressErrorPlan, MachineInstructionFetchAddressErrorPlanError> {
    match fetch_error {
        MachineCpuInstructionFetchError::Unaligned { .. } => {
            Ok(MachineInstructionFetchAddressErrorPlan::new(
                fetch_error,
                MachineInstructionFetchAddressErrorSource::Unaligned,
            ))
        }
        MachineCpuInstructionFetchError::DirectTargetMiss { .. } => {
            Ok(MachineInstructionFetchAddressErrorPlan::new(
                fetch_error,
                MachineInstructionFetchAddressErrorSource::DirectTargetMiss,
            ))
        }
        MachineCpuInstructionFetchError::PifResetUnavailable { .. } => {
            Ok(MachineInstructionFetchAddressErrorPlan::new(
                fetch_error,
                MachineInstructionFetchAddressErrorSource::PifResetUnavailable,
            ))
        }
        MachineCpuInstructionFetchError::NonDirectUnsupported { .. }
        | MachineCpuInstructionFetchError::DirectRdram { .. }
        | MachineCpuInstructionFetchError::SpDmem { .. } => {
            Err(MachineInstructionFetchAddressErrorPlanError { fetch_error })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineStepFetchFaultAction {
    EnterAddressError(MachineInstructionFetchAddressErrorPlan),
    Rethrow(MachineCpuInstructionFetchError),
}

#[cfg(test)]
impl MachineStepFetchFaultAction {
    pub(crate) const fn fetch_error(self) -> MachineCpuInstructionFetchError {
        match self {
            Self::EnterAddressError(plan) => plan.fetch_error(),
            Self::Rethrow(fetch_error) => fetch_error,
        }
    }

    pub(crate) const fn cpu_address(self) -> CpuAddress {
        self.fetch_error().cpu_address()
    }

    pub(crate) const fn width(self) -> usize {
        self.fetch_error().width()
    }

    pub(crate) const fn address_error_plan(
        self,
    ) -> Option<MachineInstructionFetchAddressErrorPlan> {
        match self {
            Self::EnterAddressError(plan) => Some(plan),
            Self::Rethrow(_) => None,
        }
    }

    pub(crate) const fn is_enter_address_error(self) -> bool {
        match self {
            Self::EnterAddressError(_) => true,
            Self::Rethrow(_) => false,
        }
    }

    pub(crate) const fn is_rethrow(self) -> bool {
        match self {
            Self::EnterAddressError(_) => false,
            Self::Rethrow(_) => true,
        }
    }
}

impl fmt::Display for MachineStepFetchFaultAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EnterAddressError(plan) => {
                write!(f, "CPU step fetch fault will enter {plan}")
            }
            Self::Rethrow(fetch_error) => {
                write!(f, "CPU step fetch fault will rethrow: {fetch_error}")
            }
        }
    }
}

pub(crate) fn classify_step_fetch_fault_action(
    fetch_error: MachineCpuInstructionFetchError,
) -> MachineStepFetchFaultAction {
    match select_cpu_instruction_fetch_address_error(fetch_error) {
        Ok(plan) => MachineStepFetchFaultAction::EnterAddressError(plan),
        Err(error) => MachineStepFetchFaultAction::Rethrow(error.fetch_error()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineStepUnsupportedInstructionCategory {
    UnknownPrimary,
    SpecialUnknown,
    RegimmUnknown,
    ControlFlowInDelaySlot,
    Cop0Unimplemented,
    Cop0RegisterUnsupported,
    CoprocessorUnimplemented,
    CacheUnimplemented,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineStepUnsupportedInstruction {
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
    category: MachineStepUnsupportedInstructionCategory,
}

impl MachineStepUnsupportedInstruction {
    const fn new(
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
        category: MachineStepUnsupportedInstructionCategory,
    ) -> Self {
        Self {
            fields,
            identity,
            category,
        }
    }

    pub const fn fields(self) -> CpuInstructionFields {
        self.fields
    }

    pub const fn raw(self) -> CpuInstructionWord {
        self.fields.raw()
    }

    pub const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    pub const fn category(self) -> MachineStepUnsupportedInstructionCategory {
        self.category
    }
}

impl fmt::Display for MachineStepUnsupportedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU step unsupported instruction {:?}: raw=0x{:08X} identity={:?}",
            self.category,
            self.raw().bits(),
            self.identity
        )
    }
}

pub(crate) const fn classify_step_unsupported_instruction(
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
) -> Option<MachineStepUnsupportedInstruction> {
    let category = match identity {
        CpuInstructionIdentity::UnknownPrimary => {
            Some(MachineStepUnsupportedInstructionCategory::UnknownPrimary)
        }
        CpuInstructionIdentity::SpecialUnknown => {
            Some(MachineStepUnsupportedInstructionCategory::SpecialUnknown)
        }
        CpuInstructionIdentity::RegimmUnknown => {
            Some(MachineStepUnsupportedInstructionCategory::RegimmUnknown)
        }
        CpuInstructionIdentity::Cop0 => {
            Some(MachineStepUnsupportedInstructionCategory::Cop0Unimplemented)
        }
        CpuInstructionIdentity::Cop0Mfc0 => {
            if is_supported_cop0_mfc0_register(fields.rd()) {
                None
            } else {
                Some(MachineStepUnsupportedInstructionCategory::Cop0RegisterUnsupported)
            }
        }
        CpuInstructionIdentity::Cop0Mtc0 => {
            if is_supported_cop0_mtc0_register(fields.rd()) {
                None
            } else {
                Some(MachineStepUnsupportedInstructionCategory::Cop0RegisterUnsupported)
            }
        }
        CpuInstructionIdentity::Cop1
        | CpuInstructionIdentity::Cop2
        | CpuInstructionIdentity::Cop3
        | CpuInstructionIdentity::Lwc1
        | CpuInstructionIdentity::Lwc2
        | CpuInstructionIdentity::Ldc1
        | CpuInstructionIdentity::Ldc2
        | CpuInstructionIdentity::Swc1
        | CpuInstructionIdentity::Swc2
        | CpuInstructionIdentity::Sdc1
        | CpuInstructionIdentity::Sdc2 => {
            Some(MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented)
        }
        CpuInstructionIdentity::Cache => {
            Some(MachineStepUnsupportedInstructionCategory::CacheUnimplemented)
        }
        _ => None,
    };

    match category {
        Some(category) => Some(MachineStepUnsupportedInstruction::new(
            fields, identity, category,
        )),
        None => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineStepStoppedInstructionCategory {
    Syscall,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineStepStoppedInstruction {
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
    category: MachineStepStoppedInstructionCategory,
}

impl MachineStepStoppedInstruction {
    const fn new(
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
        category: MachineStepStoppedInstructionCategory,
    ) -> Self {
        Self {
            fields,
            identity,
            category,
        }
    }

    pub const fn fields(self) -> CpuInstructionFields {
        self.fields
    }

    pub const fn raw(self) -> CpuInstructionWord {
        self.fields.raw()
    }

    pub const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    pub const fn category(self) -> MachineStepStoppedInstructionCategory {
        self.category
    }
}

impl fmt::Display for MachineStepStoppedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU step stopped instruction {:?}: raw=0x{:08X} identity={:?}",
            self.category,
            self.raw().bits(),
            self.identity
        )
    }
}

pub(crate) const fn classify_step_stopped_instruction(
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
) -> Option<MachineStepStoppedInstruction> {
    let category = match identity {
        CpuInstructionIdentity::SpecialSyscall => {
            Some(MachineStepStoppedInstructionCategory::Syscall)
        }
        CpuInstructionIdentity::SpecialBreak => Some(MachineStepStoppedInstructionCategory::Break),
        _ => None,
    };

    match category {
        Some(category) => Some(MachineStepStoppedInstruction::new(
            fields, identity, category,
        )),
        None => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineStepNoEffectExecutedInstructionCategory {
    Sync,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineStepNoEffectExecutedInstruction {
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
    category: MachineStepNoEffectExecutedInstructionCategory,
}

impl MachineStepNoEffectExecutedInstruction {
    const fn new(
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
        category: MachineStepNoEffectExecutedInstructionCategory,
    ) -> Self {
        Self {
            fields,
            identity,
            category,
        }
    }

    pub const fn fields(self) -> CpuInstructionFields {
        self.fields
    }

    pub const fn raw(self) -> CpuInstructionWord {
        self.fields.raw()
    }

    pub const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    pub const fn category(self) -> MachineStepNoEffectExecutedInstructionCategory {
        self.category
    }
}

impl fmt::Display for MachineStepNoEffectExecutedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU step no-effect executed instruction {:?}: raw=0x{:08X} identity={:?}",
            self.category,
            self.raw().bits(),
            self.identity
        )
    }
}

pub(crate) const fn classify_step_no_effect_executed_instruction(
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
) -> Option<MachineStepNoEffectExecutedInstruction> {
    let category = match identity {
        CpuInstructionIdentity::SpecialSync => {
            Some(MachineStepNoEffectExecutedInstructionCategory::Sync)
        }
        _ => None,
    };

    match category {
        Some(category) => Some(MachineStepNoEffectExecutedInstruction::new(
            fields, identity, category,
        )),
        None => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineOrdinaryControlFlowOperand {
    register_index: u8,
    value: u64,
}

impl MachineOrdinaryControlFlowOperand {
    const fn new(register_index: u8, value: u64) -> Self {
        Self {
            register_index,
            value,
        }
    }

    #[cfg(test)]
    pub(crate) const fn register_index(self) -> u8 {
        self.register_index
    }

    #[cfg(test)]
    pub(crate) const fn value(self) -> u64 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineOrdinaryControlFlowLink {
    destination_gpr: u8,
    value: u64,
}

impl MachineOrdinaryControlFlowLink {
    const fn new(destination_gpr: u8, value: u64) -> Self {
        Self {
            destination_gpr,
            value,
        }
    }

    pub(crate) const fn destination_gpr(self) -> u8 {
        self.destination_gpr
    }

    pub(crate) const fn value(self) -> u64 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineOrdinaryControlFlowResult {
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
    instruction_pc: CpuAddress,
    delay_slot_pc: CpuAddress,
    source_a: Option<MachineOrdinaryControlFlowOperand>,
    source_b: Option<MachineOrdinaryControlFlowOperand>,
    condition_taken: Option<bool>,
    target_pc: CpuAddress,
    selected_next_pc: CpuAddress,
    link: Option<MachineOrdinaryControlFlowLink>,
}

impl MachineOrdinaryControlFlowResult {
    pub(crate) const fn fields(self) -> CpuInstructionFields {
        self.fields
    }

    pub(crate) const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    pub(crate) const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    #[cfg(test)]
    pub(crate) const fn delay_slot_pc(self) -> CpuAddress {
        self.delay_slot_pc
    }

    #[cfg(test)]
    pub(crate) const fn source_a(self) -> Option<MachineOrdinaryControlFlowOperand> {
        self.source_a
    }

    #[cfg(test)]
    pub(crate) const fn source_b(self) -> Option<MachineOrdinaryControlFlowOperand> {
        self.source_b
    }

    #[cfg(test)]
    pub(crate) const fn condition_taken(self) -> Option<bool> {
        self.condition_taken
    }

    #[cfg(test)]
    pub(crate) const fn target_pc(self) -> CpuAddress {
        self.target_pc
    }

    pub(crate) const fn selected_next_pc(self) -> CpuAddress {
        self.selected_next_pc
    }

    pub(crate) const fn link(self) -> Option<MachineOrdinaryControlFlowLink> {
        self.link
    }
}

const fn is_supported_cop0_mfc0_register(rd: u8) -> bool {
    matches!(
        rd,
        COP0_BAD_VADDR_REGISTER_INDEX
            | COP0_COUNT_REGISTER_INDEX
            | COP0_COMPARE_REGISTER_INDEX
            | COP0_STATUS_REGISTER_INDEX
            | COP0_CAUSE_REGISTER_INDEX
            | COP0_EPC_REGISTER_INDEX
    )
}

const fn is_supported_cop0_mtc0_register(rd: u8) -> bool {
    matches!(
        rd,
        COP0_COUNT_REGISTER_INDEX
            | COP0_COMPARE_REGISTER_INDEX
            | COP0_STATUS_REGISTER_INDEX
            | COP0_CAUSE_REGISTER_INDEX
            | COP0_EPC_REGISTER_INDEX
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineStepCadenceSource {
    CommittedInstruction,
    StoppedInstruction,
    UnsupportedInstruction,
    InterruptedBeforeFetch,
    EnteredException,
    FetchAddressErrorException,
    SuccessfulEret,
    BranchLikelyAnnul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineStepControlFlowAction {
    CommitStaged,
    RestoreSnapshot,
    PreserveExceptionVector,
    ReturnBeforeCadence,
    BlockedByEretReturn,
    BlockedByBranchLikelyAnnul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineStepCountAction {
    Advance,
    DoNotAdvance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineStepCadencePlan {
    source: MachineStepCadenceSource,
    control_flow_action: MachineStepControlFlowAction,
    count_action: MachineStepCountAction,
}

impl MachineStepCadencePlan {
    const fn new(
        source: MachineStepCadenceSource,
        control_flow_action: MachineStepControlFlowAction,
        count_action: MachineStepCountAction,
    ) -> Self {
        Self {
            source,
            control_flow_action,
            count_action,
        }
    }

    pub const fn source(self) -> MachineStepCadenceSource {
        self.source
    }

    pub const fn control_flow_action(self) -> MachineStepControlFlowAction {
        self.control_flow_action
    }

    pub const fn count_action(self) -> MachineStepCountAction {
        self.count_action
    }

    pub const fn advances_count(self) -> bool {
        matches!(self.count_action, MachineStepCountAction::Advance)
    }

    pub const fn mutates_state(self) -> bool {
        false
    }
}

impl fmt::Display for MachineStepCadencePlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU step cadence {:?}: control_flow={:?} count={:?}",
            self.source, self.control_flow_action, self.count_action
        )
    }
}

pub(crate) const fn classify_machine_step_cadence(
    source: MachineStepCadenceSource,
) -> MachineStepCadencePlan {
    match source {
        MachineStepCadenceSource::CommittedInstruction => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::CommitStaged,
            MachineStepCountAction::Advance,
        ),
        MachineStepCadenceSource::StoppedInstruction => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::CommitStaged,
            MachineStepCountAction::Advance,
        ),
        MachineStepCadenceSource::UnsupportedInstruction => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::RestoreSnapshot,
            MachineStepCountAction::DoNotAdvance,
        ),
        MachineStepCadenceSource::InterruptedBeforeFetch => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::ReturnBeforeCadence,
            MachineStepCountAction::DoNotAdvance,
        ),
        MachineStepCadenceSource::EnteredException => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::PreserveExceptionVector,
            MachineStepCountAction::DoNotAdvance,
        ),
        MachineStepCadenceSource::FetchAddressErrorException => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::PreserveExceptionVector,
            MachineStepCountAction::DoNotAdvance,
        ),
        MachineStepCadenceSource::SuccessfulEret => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::BlockedByEretReturn,
            MachineStepCountAction::Advance,
        ),
        MachineStepCadenceSource::BranchLikelyAnnul => MachineStepCadencePlan::new(
            source,
            MachineStepControlFlowAction::BlockedByBranchLikelyAnnul,
            MachineStepCountAction::Advance,
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineCpuLocalInvocationStepAction {
    CommitControlFlowAndAdvanceCount,
    EnterArithmeticOverflowException,
    RejectInvocationError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineCpuLocalInvocationStepActionPlan {
    CommitControlFlowAndAdvanceCount {
        executed: CpuLocalExecutedHelperExecutedInstruction,
        cadence_plan: MachineStepCadencePlan,
    },
    EnterArithmeticOverflowException {
        overflow: CpuLocalExecutedHelperArithmeticOverflow,
    },
    RejectInvocationError {
        error: CpuLocalExecutedHelperInvocationError,
    },
}

#[allow(dead_code)]
impl MachineCpuLocalInvocationStepActionPlan {
    pub(crate) const fn action(self) -> MachineCpuLocalInvocationStepAction {
        match self {
            Self::CommitControlFlowAndAdvanceCount { .. } => {
                MachineCpuLocalInvocationStepAction::CommitControlFlowAndAdvanceCount
            }
            Self::EnterArithmeticOverflowException { .. } => {
                MachineCpuLocalInvocationStepAction::EnterArithmeticOverflowException
            }
            Self::RejectInvocationError { .. } => {
                MachineCpuLocalInvocationStepAction::RejectInvocationError
            }
        }
    }

    pub(crate) const fn cadence_plan(self) -> Option<MachineStepCadencePlan> {
        match self {
            Self::CommitControlFlowAndAdvanceCount { cadence_plan, .. } => Some(cadence_plan),
            Self::EnterArithmeticOverflowException { .. } | Self::RejectInvocationError { .. } => {
                None
            }
        }
    }

    pub(crate) const fn executed(self) -> Option<CpuLocalExecutedHelperExecutedInstruction> {
        match self {
            Self::CommitControlFlowAndAdvanceCount { executed, .. } => Some(executed),
            Self::EnterArithmeticOverflowException { .. } | Self::RejectInvocationError { .. } => {
                None
            }
        }
    }

    pub(crate) const fn overflow(self) -> Option<CpuLocalExecutedHelperArithmeticOverflow> {
        match self {
            Self::EnterArithmeticOverflowException { overflow } => Some(overflow),
            Self::CommitControlFlowAndAdvanceCount { .. } | Self::RejectInvocationError { .. } => {
                None
            }
        }
    }

    pub(crate) const fn invocation_error(self) -> Option<CpuLocalExecutedHelperInvocationError> {
        match self {
            Self::RejectInvocationError { error } => Some(error),
            Self::CommitControlFlowAndAdvanceCount { .. }
            | Self::EnterArithmeticOverflowException { .. } => None,
        }
    }

    pub(crate) const fn mutates_state(self) -> bool {
        false
    }
}

impl fmt::Display for MachineCpuLocalInvocationStepActionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommitControlFlowAndAdvanceCount {
                executed,
                cadence_plan,
            } => write!(
                f,
                "CPU-local invocation {:?}/{:?}: future cadence {}",
                executed.identity(),
                executed.family(),
                cadence_plan
            ),
            Self::EnterArithmeticOverflowException { overflow } => write!(
                f,
                "CPU-local invocation {:?}/{:?}: future arithmetic-overflow exception entry",
                overflow.identity(),
                overflow.family()
            ),
            Self::RejectInvocationError { error } => {
                write!(
                    f,
                    "CPU-local invocation rejected before step action: {error}"
                )
            }
        }
    }
}

#[allow(dead_code)]
pub(crate) const fn classify_cpu_local_invocation_step_action(
    result: Result<CpuLocalExecutedHelperInvocationOutcome, CpuLocalExecutedHelperInvocationError>,
) -> MachineCpuLocalInvocationStepActionPlan {
    match result {
        Ok(CpuLocalExecutedHelperInvocationOutcome::Executed(executed)) => {
            MachineCpuLocalInvocationStepActionPlan::CommitControlFlowAndAdvanceCount {
                executed,
                cadence_plan: classify_machine_step_cadence(
                    MachineStepCadenceSource::CommittedInstruction,
                ),
            }
        }
        Ok(CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow(overflow)) => {
            MachineCpuLocalInvocationStepActionPlan::EnterArithmeticOverflowException { overflow }
        }
        Err(error) => MachineCpuLocalInvocationStepActionPlan::RejectInvocationError { error },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct MachineCpuLocalCommittedSuccessCadence {
    executed: CpuLocalExecutedHelperExecutedInstruction,
    cadence_plan: MachineStepCadencePlan,
}

#[allow(dead_code)]
impl MachineCpuLocalCommittedSuccessCadence {
    pub(crate) const fn executed(self) -> CpuLocalExecutedHelperExecutedInstruction {
        self.executed
    }

    pub(crate) const fn cadence_plan(self) -> MachineStepCadencePlan {
        self.cadence_plan
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineCpuLocalCommittedSuccessCadenceError {
    NonSuccessAction(MachineCpuLocalInvocationStepActionPlan),
}

impl fmt::Display for MachineCpuLocalCommittedSuccessCadenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonSuccessAction(plan) => write!(
                f,
                "CPU-local committed success cadence requires a successful action plan, got {}",
                plan
            ),
        }
    }
}

impl std::error::Error for MachineCpuLocalCommittedSuccessCadenceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct MachineCpuLocalArithmeticOverflowException {
    overflow: CpuLocalExecutedHelperArithmeticOverflow,
}

#[allow(dead_code)]
impl MachineCpuLocalArithmeticOverflowException {
    pub(crate) const fn overflow(self) -> CpuLocalExecutedHelperArithmeticOverflow {
        self.overflow
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineCpuLocalArithmeticOverflowExceptionError {
    NonOverflowAction(MachineCpuLocalInvocationStepActionPlan),
    Entry(CpuArithmeticOverflowExceptionEntryError),
}

impl fmt::Display for MachineCpuLocalArithmeticOverflowExceptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonOverflowAction(plan) => write!(
                f,
                "CPU-local arithmetic-overflow exception application requires an overflow action plan, got {}",
                plan
            ),
            Self::Entry(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for MachineCpuLocalArithmeticOverflowExceptionError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineCpuLocalStepActionApplication {
    CommittedSuccess(MachineCpuLocalCommittedSuccessCadence),
    ArithmeticOverflowException(MachineCpuLocalArithmeticOverflowException),
}

#[allow(dead_code)]
impl MachineCpuLocalStepActionApplication {
    pub(crate) const fn committed_success(self) -> Option<MachineCpuLocalCommittedSuccessCadence> {
        match self {
            Self::CommittedSuccess(cadence) => Some(cadence),
            Self::ArithmeticOverflowException(_) => None,
        }
    }

    pub(crate) const fn arithmetic_overflow_exception(
        self,
    ) -> Option<MachineCpuLocalArithmeticOverflowException> {
        match self {
            Self::ArithmeticOverflowException(exception) => Some(exception),
            Self::CommittedSuccess(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineCpuLocalStepActionApplicationError {
    RejectedInvocation(CpuLocalExecutedHelperInvocationError),
    CommittedSuccess(MachineCpuLocalCommittedSuccessCadenceError),
    ArithmeticOverflowException(MachineCpuLocalArithmeticOverflowExceptionError),
}

impl fmt::Display for MachineCpuLocalStepActionApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RejectedInvocation(error) => write!(
                f,
                "CPU-local step action application rejected invocation error: {error}"
            ),
            Self::CommittedSuccess(error) => error.fmt(f),
            Self::ArithmeticOverflowException(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for MachineCpuLocalStepActionApplicationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineLoadWordCommitPlan {
    fields: CpuInstructionFields,
    execution_address: CpuAddress,
    effective_address: u64,
    target: MachineLoadWordTarget,
    loaded_word: u32,
    result_value: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineLoadWordAddressErrorPlan {
    fields: CpuInstructionFields,
    effective_address: u64,
    address_error: CpuDataAddressError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineLoadWordStepAction {
    Commit(MachineLoadWordCommitPlan),
    EnterDataAddressError(MachineLoadWordAddressErrorPlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineLoadWordStepApplication {
    Committed {
        plan: MachineLoadWordCommitPlan,
        cadence_plan: MachineStepCadencePlan,
    },
    DataAddressError {
        plan: MachineLoadWordAddressErrorPlan,
        cadence_plan: MachineStepCadencePlan,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineLoadWordStepApplicationError {
    RegisterIndex(CpuRegisterIndexError),
    DataAddressErrorEntry(CpuAddressErrorExceptionEntryError),
}

impl fmt::Display for MachineLoadWordStepApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegisterIndex(error) => error.fmt(f),
            Self::DataAddressErrorEntry(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for MachineLoadWordStepApplicationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineOrdinaryControlFlowStepAction {
    Beq(MachineOrdinaryControlFlowResult),
    Bne(MachineOrdinaryControlFlowResult),
    J(MachineOrdinaryControlFlowResult),
    Jal(MachineOrdinaryControlFlowResult),
    Jr(MachineOrdinaryControlFlowResult),
    Jalr(MachineOrdinaryControlFlowResult),
}

impl MachineOrdinaryControlFlowStepAction {
    const fn result(self) -> MachineOrdinaryControlFlowResult {
        match self {
            Self::Beq(result)
            | Self::Bne(result)
            | Self::J(result)
            | Self::Jal(result)
            | Self::Jr(result)
            | Self::Jalr(result) => result,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineOrdinaryControlFlowStepApplication {
    result: MachineOrdinaryControlFlowResult,
    cadence_plan: MachineStepCadencePlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineNonCpuLocalStepFrontierAction {
    NoEffectExecuted(MachineStepNoEffectExecutedInstruction),
    Stopped(MachineStepStoppedInstruction),
    Unsupported(MachineStepUnsupportedInstruction),
    FetchFault(MachineStepFetchFaultAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineNonCpuLocalStepFrontierApplication {
    NoEffectExecuted {
        instruction: MachineStepNoEffectExecutedInstruction,
        cadence_plan: MachineStepCadencePlan,
    },
    Stopped {
        instruction: MachineStepStoppedInstruction,
        cadence_plan: MachineStepCadencePlan,
    },
    Unsupported {
        instruction: MachineStepUnsupportedInstruction,
        cadence_plan: MachineStepCadencePlan,
    },
    FetchAddressErrorException {
        plan: MachineInstructionFetchAddressErrorPlan,
        cadence_plan: MachineStepCadencePlan,
    },
}

#[allow(dead_code)]
impl MachineNonCpuLocalStepFrontierApplication {
    pub(crate) const fn cadence_plan(self) -> MachineStepCadencePlan {
        match self {
            Self::NoEffectExecuted { cadence_plan, .. }
            | Self::Stopped { cadence_plan, .. }
            | Self::Unsupported { cadence_plan, .. }
            | Self::FetchAddressErrorException { cadence_plan, .. } => cadence_plan,
        }
    }

    pub(crate) const fn no_effect_executed_instruction(
        self,
    ) -> Option<MachineStepNoEffectExecutedInstruction> {
        match self {
            Self::NoEffectExecuted { instruction, .. } => Some(instruction),
            Self::Stopped { .. }
            | Self::Unsupported { .. }
            | Self::FetchAddressErrorException { .. } => None,
        }
    }

    pub(crate) const fn stopped_instruction(self) -> Option<MachineStepStoppedInstruction> {
        match self {
            Self::Stopped { instruction, .. } => Some(instruction),
            Self::NoEffectExecuted { .. }
            | Self::Unsupported { .. }
            | Self::FetchAddressErrorException { .. } => None,
        }
    }

    pub(crate) const fn unsupported_instruction(self) -> Option<MachineStepUnsupportedInstruction> {
        match self {
            Self::Unsupported { instruction, .. } => Some(instruction),
            Self::NoEffectExecuted { .. }
            | Self::Stopped { .. }
            | Self::FetchAddressErrorException { .. } => None,
        }
    }

    pub(crate) const fn fetch_address_error_plan(
        self,
    ) -> Option<MachineInstructionFetchAddressErrorPlan> {
        match self {
            Self::FetchAddressErrorException { plan, .. } => Some(plan),
            Self::NoEffectExecuted { .. } | Self::Stopped { .. } | Self::Unsupported { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineNonCpuLocalStepFrontierApplicationError {
    FetchFaultRethrow(MachineCpuInstructionFetchError),
    FetchAddressErrorEntry(CpuAddressErrorExceptionEntryError),
}

impl fmt::Display for MachineNonCpuLocalStepFrontierApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchFaultRethrow(fetch_error) => {
                write!(
                    f,
                    "non-CPU-local fetch-fault action rethrows: {fetch_error}"
                )
            }
            Self::FetchAddressErrorEntry(entry_error) => entry_error.fmt(f),
        }
    }
}

impl std::error::Error for MachineNonCpuLocalStepFrontierApplicationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineClassifiedStepAction {
    CpuLocal(MachineCpuLocalInvocationStepActionPlan),
    OrdinaryControlFlow(MachineOrdinaryControlFlowStepAction),
    LoadWord(MachineLoadWordStepAction),
    NonCpuLocalFrontier(MachineNonCpuLocalStepFrontierAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineClassifiedStepActionApplication {
    CpuLocal(MachineCpuLocalStepActionApplication),
    OrdinaryControlFlow(MachineOrdinaryControlFlowStepApplication),
    LoadWord(MachineLoadWordStepApplication),
    NonCpuLocalFrontier(MachineNonCpuLocalStepFrontierApplication),
}

#[allow(dead_code)]
impl MachineClassifiedStepActionApplication {
    pub(crate) const fn cpu_local(self) -> Option<MachineCpuLocalStepActionApplication> {
        match self {
            Self::CpuLocal(application) => Some(application),
            Self::OrdinaryControlFlow(_) | Self::LoadWord(_) | Self::NonCpuLocalFrontier(_) => None,
        }
    }

    pub(crate) const fn non_cpu_local_frontier(
        self,
    ) -> Option<MachineNonCpuLocalStepFrontierApplication> {
        match self {
            Self::NonCpuLocalFrontier(application) => Some(application),
            Self::CpuLocal(_) | Self::OrdinaryControlFlow(_) | Self::LoadWord(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineClassifiedStepActionApplicationError {
    CpuLocal(MachineCpuLocalStepActionApplicationError),
    LoadWord(MachineLoadWordStepApplicationError),
    NonCpuLocalFrontier(MachineNonCpuLocalStepFrontierApplicationError),
}

impl fmt::Display for MachineClassifiedStepActionApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CpuLocal(error) => error.fmt(f),
            Self::LoadWord(error) => error.fmt(f),
            Self::NonCpuLocalFrontier(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for MachineClassifiedStepActionApplicationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct MachineCurrentPcClassifiedStepAction {
    control_flow_snapshot: CpuControlFlowSnapshot,
    action: MachineClassifiedStepAction,
}

#[allow(dead_code)]
impl MachineCurrentPcClassifiedStepAction {
    pub(crate) const fn control_flow_snapshot(self) -> CpuControlFlowSnapshot {
        self.control_flow_snapshot
    }

    pub(crate) const fn action(self) -> MachineClassifiedStepAction {
        self.action
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum MachineCurrentPcClassifiedStepActionError {
    FetchFaultRethrow(MachineCpuInstructionFetchError),
    BootstrapCpuStateUnavailable(MachineBootstrapCpuStateUnavailable),
    OrdinaryControlFlowRejected(MachineOrdinaryControlFlowRejection),
    LoadWordRejected(MachineLoadWordRejection),
    CpuLocalInvocation(CpuLocalExecutedHelperInvocationError),
    UnrepresentedInstruction {
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
    },
}

#[allow(dead_code)]
impl MachineCurrentPcClassifiedStepActionError {
    pub(crate) const fn fetch_error(self) -> Option<MachineCpuInstructionFetchError> {
        match self {
            Self::FetchFaultRethrow(fetch_error) => Some(fetch_error),
            Self::BootstrapCpuStateUnavailable(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::LoadWordRejected(_)
            | Self::CpuLocalInvocation(_)
            | Self::UnrepresentedInstruction { .. } => None,
        }
    }

    pub(crate) const fn bootstrap_cpu_state_unavailable(
        self,
    ) -> Option<MachineBootstrapCpuStateUnavailable> {
        match self {
            Self::BootstrapCpuStateUnavailable(error) => Some(error),
            Self::FetchFaultRethrow(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::LoadWordRejected(_)
            | Self::CpuLocalInvocation(_)
            | Self::UnrepresentedInstruction { .. } => None,
        }
    }

    pub(crate) const fn invocation_error(self) -> Option<CpuLocalExecutedHelperInvocationError> {
        match self {
            Self::CpuLocalInvocation(error) => Some(error),
            Self::FetchFaultRethrow(_)
            | Self::BootstrapCpuStateUnavailable(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::LoadWordRejected(_)
            | Self::UnrepresentedInstruction { .. } => None,
        }
    }

    pub(crate) const fn fields(self) -> Option<CpuInstructionFields> {
        match self {
            Self::UnrepresentedInstruction { fields, .. } => Some(fields),
            Self::FetchFaultRethrow(_)
            | Self::BootstrapCpuStateUnavailable(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::LoadWordRejected(_)
            | Self::CpuLocalInvocation(_) => None,
        }
    }

    pub(crate) const fn identity(self) -> Option<CpuInstructionIdentity> {
        match self {
            Self::UnrepresentedInstruction { identity, .. } => Some(identity),
            Self::BootstrapCpuStateUnavailable(error) => Some(error.identity()),
            Self::OrdinaryControlFlowRejected(error) => Some(error.identity()),
            Self::LoadWordRejected(error) => Some(error.identity()),
            Self::FetchFaultRethrow(_) | Self::CpuLocalInvocation(_) => None,
        }
    }

    pub(crate) const fn load_word_rejection(self) -> Option<MachineLoadWordRejection> {
        match self {
            Self::LoadWordRejected(rejection) => Some(rejection),
            Self::FetchFaultRethrow(_)
            | Self::BootstrapCpuStateUnavailable(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::CpuLocalInvocation(_)
            | Self::UnrepresentedInstruction { .. } => None,
        }
    }
}

impl fmt::Display for MachineCurrentPcClassifiedStepActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchFaultRethrow(fetch_error) => {
                write!(
                    f,
                    "current-PC classified step action production rethrows fetch fault: {fetch_error}"
                )
            }
            Self::CpuLocalInvocation(error) => {
                write!(
                    f,
                    "current-PC classified step action production rejected CPU-local invocation: {error}"
                )
            }
            Self::BootstrapCpuStateUnavailable(error) => {
                write!(
                    f,
                    "current-PC classified step action production rejected unknown bootstrap operand: {error}"
                )
            }
            Self::OrdinaryControlFlowRejected(error) => error.fmt(f),
            Self::LoadWordRejected(error) => {
                write!(f, "current-PC classified step action production {error}")
            }
            Self::UnrepresentedInstruction { fields, identity } => {
                write!(
                    f,
                    "current-PC classified step action production has no sealed category for raw=0x{:08X} identity={:?}",
                    fields.raw().bits(),
                    identity
                )
            }
        }
    }
}

impl std::error::Error for MachineCurrentPcClassifiedStepActionError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineOrdinaryControlFlowRejectionReason {
    BootstrapSourceUnavailable {
        register_index: u8,
        source: MachineBootstrapGprSource,
    },
    BootstrapLinkLineageUnavailable {
        destination_gpr: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineOrdinaryControlFlowRejection {
    instruction_pc: CpuAddress,
    identity: CpuInstructionIdentity,
    reason: MachineOrdinaryControlFlowRejectionReason,
}

impl MachineOrdinaryControlFlowRejection {
    const fn new(
        result: MachineOrdinaryControlFlowResult,
        reason: MachineOrdinaryControlFlowRejectionReason,
    ) -> Self {
        Self {
            instruction_pc: result.instruction_pc(),
            identity: result.identity(),
            reason,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    pub const fn reason(self) -> MachineOrdinaryControlFlowRejectionReason {
        self.reason
    }
}

impl fmt::Display for MachineOrdinaryControlFlowRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ordinary control flow {:?} rejected before mutation at 0x{:08X}: {:?}",
            self.identity,
            self.instruction_pc.value(),
            self.reason
        )
    }
}

impl std::error::Error for MachineOrdinaryControlFlowRejection {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineArithmeticOverflowExceptionEntryRejection {
    pc: CpuAddress,
    next_pc: CpuAddress,
    status: u32,
}

impl MachineArithmeticOverflowExceptionEntryRejection {
    const fn from_cpu_error(error: CpuArithmeticOverflowExceptionEntryError) -> Self {
        Self {
            pc: error.pc(),
            next_pc: error.next_pc(),
            status: error.status(),
        }
    }

    pub const fn pc(self) -> CpuAddress {
        self.pc
    }

    pub const fn next_pc(self) -> CpuAddress {
        self.next_pc
    }

    pub const fn status(self) -> u32 {
        self.status
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineStepCpuLocalInvocationRejection {
    HelperRejectedSelection { identity: CpuInstructionIdentity },
    RegisterIndex(CpuRegisterIndexError),
}

impl MachineStepCpuLocalInvocationRejection {
    const fn from_invocation_error(error: CpuLocalExecutedHelperInvocationError) -> Self {
        match error {
            CpuLocalExecutedHelperInvocationError::HelperRejectedSelection(selection) => {
                Self::HelperRejectedSelection {
                    identity: selection.identity(),
                }
            }
            CpuLocalExecutedHelperInvocationError::RegisterIndex(error) => {
                Self::RegisterIndex(error)
            }
        }
    }

    pub const fn identity(self) -> Option<CpuInstructionIdentity> {
        match self {
            Self::HelperRejectedSelection { identity } => Some(identity),
            Self::RegisterIndex(_) => None,
        }
    }

    pub const fn register_index_error(self) -> Option<CpuRegisterIndexError> {
        match self {
            Self::RegisterIndex(error) => Some(error),
            Self::HelperRejectedSelection { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRepresentedStepOutcome {
    CpuLocalCommitted {
        identity: CpuInstructionIdentity,
        cadence_plan: MachineStepCadencePlan,
    },
    LoadWordCommitted {
        effective_address: u64,
        target: MachineLoadWordTarget,
        destination_gpr: u8,
        loaded_word: u32,
        result_value: u64,
        cadence_plan: MachineStepCadencePlan,
    },
    DataAddressError {
        identity: CpuInstructionIdentity,
        effective_address: u64,
        address_error: CpuDataAddressError,
        cadence_plan: MachineStepCadencePlan,
    },
    ArithmeticOverflowException {
        identity: CpuInstructionIdentity,
    },
    NoEffectCommitted {
        instruction: MachineStepNoEffectExecutedInstruction,
        cadence_plan: MachineStepCadencePlan,
    },
    Stopped {
        instruction: MachineStepStoppedInstruction,
        cadence_plan: MachineStepCadencePlan,
    },
    Unsupported {
        instruction: MachineStepUnsupportedInstruction,
        cadence_plan: MachineStepCadencePlan,
    },
    InstructionFetchAddressError {
        plan: MachineInstructionFetchAddressErrorPlan,
        cadence_plan: MachineStepCadencePlan,
    },
}

impl MachineRepresentedStepOutcome {
    fn from_application(application: MachineClassifiedStepActionApplication) -> Self {
        match application {
            MachineClassifiedStepActionApplication::CpuLocal(
                MachineCpuLocalStepActionApplication::CommittedSuccess(committed),
            ) => Self::CpuLocalCommitted {
                identity: committed.executed().identity(),
                cadence_plan: committed.cadence_plan(),
            },
            MachineClassifiedStepActionApplication::CpuLocal(
                MachineCpuLocalStepActionApplication::ArithmeticOverflowException(exception),
            ) => Self::ArithmeticOverflowException {
                identity: exception.overflow().identity(),
            },
            MachineClassifiedStepActionApplication::OrdinaryControlFlow(application) => {
                Self::CpuLocalCommitted {
                    identity: application.result.identity(),
                    cadence_plan: application.cadence_plan,
                }
            }
            MachineClassifiedStepActionApplication::LoadWord(
                MachineLoadWordStepApplication::Committed { plan, cadence_plan },
            ) => Self::LoadWordCommitted {
                effective_address: plan.effective_address,
                target: plan.target,
                destination_gpr: plan.fields.rt(),
                loaded_word: plan.loaded_word,
                result_value: plan.result_value,
                cadence_plan,
            },
            MachineClassifiedStepActionApplication::LoadWord(
                MachineLoadWordStepApplication::DataAddressError { plan, cadence_plan },
            ) => Self::DataAddressError {
                identity: CpuInstructionIdentity::Lw,
                effective_address: plan.effective_address,
                address_error: plan.address_error,
                cadence_plan,
            },
            MachineClassifiedStepActionApplication::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierApplication::NoEffectExecuted {
                    instruction,
                    cadence_plan,
                },
            ) => Self::NoEffectCommitted {
                instruction,
                cadence_plan,
            },
            MachineClassifiedStepActionApplication::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierApplication::Stopped {
                    instruction,
                    cadence_plan,
                },
            ) => Self::Stopped {
                instruction,
                cadence_plan,
            },
            MachineClassifiedStepActionApplication::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierApplication::Unsupported {
                    instruction,
                    cadence_plan,
                },
            ) => Self::Unsupported {
                instruction,
                cadence_plan,
            },
            MachineClassifiedStepActionApplication::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierApplication::FetchAddressErrorException {
                    plan,
                    cadence_plan,
                },
            ) => Self::InstructionFetchAddressError { plan, cadence_plan },
        }
    }

    pub const fn identity(self) -> Option<CpuInstructionIdentity> {
        match self {
            Self::CpuLocalCommitted { identity, .. }
            | Self::DataAddressError { identity, .. }
            | Self::ArithmeticOverflowException { identity } => Some(identity),
            Self::LoadWordCommitted { .. } => Some(CpuInstructionIdentity::Lw),
            Self::NoEffectCommitted { instruction, .. } => Some(instruction.identity()),
            Self::Stopped { instruction, .. } => Some(instruction.identity()),
            Self::Unsupported { instruction, .. } => Some(instruction.identity()),
            Self::InstructionFetchAddressError { .. } => None,
        }
    }

    pub const fn cadence_plan(self) -> Option<MachineStepCadencePlan> {
        match self {
            Self::CpuLocalCommitted { cadence_plan, .. }
            | Self::LoadWordCommitted { cadence_plan, .. }
            | Self::DataAddressError { cadence_plan, .. }
            | Self::NoEffectCommitted { cadence_plan, .. }
            | Self::Stopped { cadence_plan, .. }
            | Self::Unsupported { cadence_plan, .. }
            | Self::InstructionFetchAddressError { cadence_plan, .. } => Some(cadence_plan),
            Self::ArithmeticOverflowException { .. } => None,
        }
    }

    pub const fn stopped_instruction(self) -> Option<MachineStepStoppedInstruction> {
        match self {
            Self::Stopped { instruction, .. } => Some(instruction),
            Self::CpuLocalCommitted { .. }
            | Self::LoadWordCommitted { .. }
            | Self::DataAddressError { .. }
            | Self::ArithmeticOverflowException { .. }
            | Self::NoEffectCommitted { .. }
            | Self::Unsupported { .. }
            | Self::InstructionFetchAddressError { .. } => None,
        }
    }

    pub const fn unsupported_instruction(self) -> Option<MachineStepUnsupportedInstruction> {
        match self {
            Self::Unsupported { instruction, .. } => Some(instruction),
            Self::CpuLocalCommitted { .. }
            | Self::LoadWordCommitted { .. }
            | Self::DataAddressError { .. }
            | Self::ArithmeticOverflowException { .. }
            | Self::NoEffectCommitted { .. }
            | Self::Stopped { .. }
            | Self::InstructionFetchAddressError { .. } => None,
        }
    }

    pub const fn no_effect_instruction(self) -> Option<MachineStepNoEffectExecutedInstruction> {
        match self {
            Self::NoEffectCommitted { instruction, .. } => Some(instruction),
            Self::CpuLocalCommitted { .. }
            | Self::LoadWordCommitted { .. }
            | Self::DataAddressError { .. }
            | Self::ArithmeticOverflowException { .. }
            | Self::Stopped { .. }
            | Self::Unsupported { .. }
            | Self::InstructionFetchAddressError { .. } => None,
        }
    }

    pub const fn fetch_address_error_plan(self) -> Option<MachineInstructionFetchAddressErrorPlan> {
        match self {
            Self::InstructionFetchAddressError { plan, .. } => Some(plan),
            Self::CpuLocalCommitted { .. }
            | Self::LoadWordCommitted { .. }
            | Self::DataAddressError { .. }
            | Self::ArithmeticOverflowException { .. }
            | Self::NoEffectCommitted { .. }
            | Self::Stopped { .. }
            | Self::Unsupported { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRepresentedStepError {
    FetchRejected(MachineCpuInstructionFetchError),
    BootstrapCpuStateUnavailable(MachineBootstrapCpuStateUnavailable),
    OrdinaryControlFlowRejected(MachineOrdinaryControlFlowRejection),
    LoadWordRejected(MachineLoadWordRejection),
    CpuLocalInvocationRejected(MachineStepCpuLocalInvocationRejection),
    UnrepresentedInstruction {
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
    },
    ArithmeticOverflowExceptionEntryRejected(MachineArithmeticOverflowExceptionEntryRejection),
    DataAddressErrorExceptionEntryRejected(CpuAddressErrorExceptionEntryError),
    InstructionFetchAddressErrorEntryRejected(CpuAddressErrorExceptionEntryError),
    CompositionInvariantRejected,
}

impl MachineRepresentedStepError {
    fn from_production_error(error: MachineCurrentPcClassifiedStepActionError) -> Self {
        match error {
            MachineCurrentPcClassifiedStepActionError::FetchFaultRethrow(fetch_error) => {
                Self::FetchRejected(fetch_error)
            }
            MachineCurrentPcClassifiedStepActionError::BootstrapCpuStateUnavailable(error) => {
                Self::BootstrapCpuStateUnavailable(error)
            }
            MachineCurrentPcClassifiedStepActionError::OrdinaryControlFlowRejected(error) => {
                Self::OrdinaryControlFlowRejected(error)
            }
            MachineCurrentPcClassifiedStepActionError::LoadWordRejected(error) => {
                Self::LoadWordRejected(error)
            }
            MachineCurrentPcClassifiedStepActionError::CpuLocalInvocation(error) => {
                Self::CpuLocalInvocationRejected(
                    MachineStepCpuLocalInvocationRejection::from_invocation_error(error),
                )
            }
            MachineCurrentPcClassifiedStepActionError::UnrepresentedInstruction {
                fields,
                identity,
            } => Self::UnrepresentedInstruction { fields, identity },
        }
    }

    fn from_application_error(error: MachineClassifiedStepActionApplicationError) -> Self {
        match error {
            MachineClassifiedStepActionApplicationError::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierApplicationError::FetchFaultRethrow(fetch_error),
            ) => Self::FetchRejected(fetch_error),
            MachineClassifiedStepActionApplicationError::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierApplicationError::FetchAddressErrorEntry(error),
            ) => Self::InstructionFetchAddressErrorEntryRejected(error),
            MachineClassifiedStepActionApplicationError::CpuLocal(
                MachineCpuLocalStepActionApplicationError::RejectedInvocation(error),
            ) => Self::CpuLocalInvocationRejected(
                MachineStepCpuLocalInvocationRejection::from_invocation_error(error),
            ),
            MachineClassifiedStepActionApplicationError::CpuLocal(
                MachineCpuLocalStepActionApplicationError::ArithmeticOverflowException(
                    MachineCpuLocalArithmeticOverflowExceptionError::Entry(error),
                ),
            ) => Self::ArithmeticOverflowExceptionEntryRejected(
                MachineArithmeticOverflowExceptionEntryRejection::from_cpu_error(error),
            ),
            MachineClassifiedStepActionApplicationError::LoadWord(
                MachineLoadWordStepApplicationError::DataAddressErrorEntry(error),
            ) => Self::DataAddressErrorExceptionEntryRejected(error),
            MachineClassifiedStepActionApplicationError::CpuLocal(
                MachineCpuLocalStepActionApplicationError::CommittedSuccess(_),
            )
            | MachineClassifiedStepActionApplicationError::CpuLocal(
                MachineCpuLocalStepActionApplicationError::ArithmeticOverflowException(
                    MachineCpuLocalArithmeticOverflowExceptionError::NonOverflowAction(_),
                ),
            )
            | MachineClassifiedStepActionApplicationError::LoadWord(
                MachineLoadWordStepApplicationError::RegisterIndex(_),
            ) => Self::CompositionInvariantRejected,
        }
    }

    pub const fn fetch_error(self) -> Option<MachineCpuInstructionFetchError> {
        match self {
            Self::FetchRejected(fetch_error) => Some(fetch_error),
            Self::BootstrapCpuStateUnavailable(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::LoadWordRejected(_)
            | Self::CpuLocalInvocationRejected(_)
            | Self::UnrepresentedInstruction { .. }
            | Self::ArithmeticOverflowExceptionEntryRejected(_)
            | Self::DataAddressErrorExceptionEntryRejected(_)
            | Self::InstructionFetchAddressErrorEntryRejected(_)
            | Self::CompositionInvariantRejected => None,
        }
    }

    pub const fn identity(self) -> Option<CpuInstructionIdentity> {
        match self {
            Self::BootstrapCpuStateUnavailable(error) => Some(error.identity()),
            Self::OrdinaryControlFlowRejected(error) => Some(error.identity()),
            Self::LoadWordRejected(error) => Some(error.identity()),
            Self::CpuLocalInvocationRejected(rejection) => rejection.identity(),
            Self::UnrepresentedInstruction { identity, .. } => Some(identity),
            Self::FetchRejected(_)
            | Self::ArithmeticOverflowExceptionEntryRejected(_)
            | Self::DataAddressErrorExceptionEntryRejected(_)
            | Self::InstructionFetchAddressErrorEntryRejected(_)
            | Self::CompositionInvariantRejected => None,
        }
    }

    pub const fn bootstrap_cpu_state_unavailable(
        self,
    ) -> Option<MachineBootstrapCpuStateUnavailable> {
        match self {
            Self::BootstrapCpuStateUnavailable(error) => Some(error),
            Self::FetchRejected(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::LoadWordRejected(_)
            | Self::CpuLocalInvocationRejected(_)
            | Self::UnrepresentedInstruction { .. }
            | Self::ArithmeticOverflowExceptionEntryRejected(_)
            | Self::DataAddressErrorExceptionEntryRejected(_)
            | Self::InstructionFetchAddressErrorEntryRejected(_)
            | Self::CompositionInvariantRejected => None,
        }
    }

    pub const fn load_word_rejection(self) -> Option<MachineLoadWordRejection> {
        match self {
            Self::LoadWordRejected(rejection) => Some(rejection),
            Self::FetchRejected(_)
            | Self::BootstrapCpuStateUnavailable(_)
            | Self::OrdinaryControlFlowRejected(_)
            | Self::CpuLocalInvocationRejected(_)
            | Self::UnrepresentedInstruction { .. }
            | Self::ArithmeticOverflowExceptionEntryRejected(_)
            | Self::DataAddressErrorExceptionEntryRejected(_)
            | Self::InstructionFetchAddressErrorEntryRejected(_)
            | Self::CompositionInvariantRejected => None,
        }
    }

    pub const fn ordinary_control_flow_rejection(
        self,
    ) -> Option<MachineOrdinaryControlFlowRejection> {
        match self {
            Self::OrdinaryControlFlowRejected(rejection) => Some(rejection),
            Self::FetchRejected(_)
            | Self::BootstrapCpuStateUnavailable(_)
            | Self::LoadWordRejected(_)
            | Self::CpuLocalInvocationRejected(_)
            | Self::UnrepresentedInstruction { .. }
            | Self::ArithmeticOverflowExceptionEntryRejected(_)
            | Self::DataAddressErrorExceptionEntryRejected(_)
            | Self::InstructionFetchAddressErrorEntryRejected(_)
            | Self::CompositionInvariantRejected => None,
        }
    }
}

impl fmt::Display for MachineRepresentedStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchRejected(fetch_error) => {
                write!(f, "represented Machine::step rejected fetch: {fetch_error}")
            }
            Self::BootstrapCpuStateUnavailable(error) => write!(
                f,
                "represented Machine::step rejected unknown bootstrap CPU operand: {error}"
            ),
            Self::OrdinaryControlFlowRejected(error) => {
                write!(f, "represented Machine::step {error}")
            }
            Self::LoadWordRejected(error) => {
                write!(f, "represented Machine::step {error}")
            }
            Self::CpuLocalInvocationRejected(rejection) => write!(
                f,
                "represented Machine::step rejected CPU-local invocation: {rejection:?}"
            ),
            Self::UnrepresentedInstruction { fields, identity } => write!(
                f,
                "represented Machine::step has no sealed category for raw=0x{:08X} identity={:?}",
                fields.raw().bits(),
                identity
            ),
            Self::ArithmeticOverflowExceptionEntryRejected(error) => write!(
                f,
                "represented Machine::step arithmetic-overflow entry rejected: pc={} next_pc={} status={}",
                error.pc().value(),
                error.next_pc().value(),
                error.status()
            ),
            Self::DataAddressErrorExceptionEntryRejected(error) => error.fmt(f),
            Self::InstructionFetchAddressErrorEntryRejected(error) => error.fmt(f),
            Self::CompositionInvariantRejected => write!(
                f,
                "represented Machine::step composition rejected an internal invariant"
            ),
        }
    }
}

impl std::error::Error for MachineRepresentedStepError {}

pub struct Machine {
    cartridge: Cartridge,
    pif_firmware: Option<PifFirmware>,
    cpu: Cpu,
    rdram: Rdram,
    sp_dmem: SpDmem,
    sp_imem: SpImem,
    cpu_rdram_reservation: CpuRdramReservation,
    cartridge_bootstrap: Option<MachineCartridgeBootstrapState>,
    powered_on: bool,
}

impl Machine {
    pub fn from_cartridge(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            pif_firmware: None,
            cpu: Cpu::new(),
            rdram: Rdram::default(),
            sp_dmem: SpDmem::default(),
            sp_imem: SpImem::default(),
            cpu_rdram_reservation: CpuRdramReservation::new(),
            cartridge_bootstrap: None,
            powered_on: true,
        }
    }

    pub fn reset(&mut self) {
        self.cpu = Cpu::new();
        self.rdram = Rdram::default();
        self.sp_dmem = SpDmem::default();
        self.sp_imem = SpImem::default();
        self.cpu_rdram_reservation = CpuRdramReservation::new();
        self.cartridge_bootstrap = None;
        self.powered_on = true;
    }

    /// Validates and transfers one explicitly profiled raw PIF Boot ROM into
    /// this Machine.
    ///
    /// Validation completes before the accepted immutable firmware owner is
    /// replaced. This input-only boundary does not execute firmware or produce
    /// SP IMEM state.
    pub fn install_pif_firmware(
        &mut self,
        profile: PifIpl2Profile,
        owned_bytes: Vec<u8>,
    ) -> Result<MachinePifFirmwareState, PifFirmwareValidationError> {
        let firmware = PifFirmware::from_owned_bytes(profile, owned_bytes)?;
        let state = firmware.state();
        self.pif_firmware = Some(firmware);
        Ok(state)
    }

    pub fn pif_firmware_state(&self) -> MachinePifFirmwareState {
        self.pif_firmware
            .as_ref()
            .map_or(MachinePifFirmwareState::Absent, PifFirmware::state)
    }

    /// Stages the represented CPU control-flow pair for a selected PC.
    ///
    /// This sets `pc` to `value` and establishes the sequential invariant
    /// `next_pc = value.wrapping_add(4)`, clearing any stale delay-slot
    /// context without exposing mutable CPU or COP0 state and without fetching
    /// or executing an instruction.
    pub fn stage_cpu_pc(&mut self, value: u32) {
        self.cpu.stage_pc(value);
    }

    /// Returns the explicit owner of the currently represented CPU delay slot.
    pub fn cpu_delay_slot_context(&self) -> Option<CpuDelaySlotContext> {
        self.cpu.delay_slot_context()
    }

    pub fn step(&mut self) -> Result<MachineRepresentedStepOutcome, MachineRepresentedStepError> {
        let produced = self
            .produce_current_pc_classified_step_action()
            .map_err(MachineRepresentedStepError::from_production_error)?;
        let applied = self
            .apply_classified_step_action(produced.action(), produced.control_flow_snapshot())
            .map_err(MachineRepresentedStepError::from_application_error)?;

        Ok(MachineRepresentedStepOutcome::from_application(applied))
    }

    fn produce_ordinary_control_flow_step_action(
        &self,
        control_flow_snapshot: CpuControlFlowSnapshot,
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
    ) -> Option<MachineOrdinaryControlFlowStepAction> {
        let instruction_pc = CpuAddress::new(control_flow_snapshot.pc());
        let delay_slot_pc = CpuAddress::new(control_flow_snapshot.next_pc());
        let link_value = sign_extend_cpu_address(control_flow_snapshot.pc().wrapping_add(8));

        let action = match identity {
            CpuInstructionIdentity::Beq | CpuInstructionIdentity::Bne => {
                let source_a_value = self
                    .cpu
                    .gpr(usize::from(fields.rs()))
                    .expect("decoded CPU register index is five bits");
                let source_b_value = self
                    .cpu
                    .gpr(usize::from(fields.rt()))
                    .expect("decoded CPU register index is five bits");
                let condition_taken = if identity == CpuInstructionIdentity::Beq {
                    source_a_value == source_b_value
                } else {
                    source_a_value != source_b_value
                };
                let target_pc = CpuAddress::new(conditional_branch_target(
                    control_flow_snapshot.pc(),
                    fields.immediate_u16(),
                ));
                let selected_next_pc = if condition_taken {
                    target_pc
                } else {
                    CpuAddress::new(control_flow_snapshot.pc().wrapping_add(8))
                };
                let result = MachineOrdinaryControlFlowResult {
                    fields,
                    identity,
                    instruction_pc,
                    delay_slot_pc,
                    source_a: Some(MachineOrdinaryControlFlowOperand::new(
                        fields.rs(),
                        source_a_value,
                    )),
                    source_b: Some(MachineOrdinaryControlFlowOperand::new(
                        fields.rt(),
                        source_b_value,
                    )),
                    condition_taken: Some(condition_taken),
                    target_pc,
                    selected_next_pc,
                    link: None,
                };
                if identity == CpuInstructionIdentity::Beq {
                    MachineOrdinaryControlFlowStepAction::Beq(result)
                } else {
                    MachineOrdinaryControlFlowStepAction::Bne(result)
                }
            }
            CpuInstructionIdentity::J | CpuInstructionIdentity::Jal => {
                let target_pc = CpuAddress::new(jump_target(
                    control_flow_snapshot.pc(),
                    fields.jump_target(),
                ));
                let link = if identity == CpuInstructionIdentity::Jal {
                    Some(MachineOrdinaryControlFlowLink::new(31, link_value))
                } else {
                    None
                };
                let result = MachineOrdinaryControlFlowResult {
                    fields,
                    identity,
                    instruction_pc,
                    delay_slot_pc,
                    source_a: None,
                    source_b: None,
                    condition_taken: None,
                    target_pc,
                    selected_next_pc: target_pc,
                    link,
                };
                if identity == CpuInstructionIdentity::J {
                    MachineOrdinaryControlFlowStepAction::J(result)
                } else {
                    MachineOrdinaryControlFlowStepAction::Jal(result)
                }
            }
            CpuInstructionIdentity::SpecialJr | CpuInstructionIdentity::SpecialJalr => {
                let source_value = self
                    .cpu
                    .gpr(usize::from(fields.rs()))
                    .expect("decoded CPU register index is five bits");
                let target_pc = CpuAddress::new(source_value as u32);
                let link = if identity == CpuInstructionIdentity::SpecialJalr {
                    Some(MachineOrdinaryControlFlowLink::new(fields.rd(), link_value))
                } else {
                    None
                };
                let result = MachineOrdinaryControlFlowResult {
                    fields,
                    identity,
                    instruction_pc,
                    delay_slot_pc,
                    source_a: Some(MachineOrdinaryControlFlowOperand::new(
                        fields.rs(),
                        source_value,
                    )),
                    source_b: None,
                    condition_taken: None,
                    target_pc,
                    selected_next_pc: target_pc,
                    link,
                };
                if identity == CpuInstructionIdentity::SpecialJr {
                    MachineOrdinaryControlFlowStepAction::Jr(result)
                } else {
                    MachineOrdinaryControlFlowStepAction::Jalr(result)
                }
            }
            _ => return None,
        };

        Some(action)
    }

    fn ordinary_control_flow_rejection(
        &self,
        result: MachineOrdinaryControlFlowResult,
    ) -> Option<MachineOrdinaryControlFlowRejection> {
        let state = self.cartridge_bootstrap?;
        for operand in [result.source_a, result.source_b].into_iter().flatten() {
            let source = state
                .gpr_source(usize::from(operand.register_index))
                .unwrap_or(MachineBootstrapGprSource::UnknownPifProduced);
            if !source.is_known() {
                return Some(MachineOrdinaryControlFlowRejection::new(
                    result,
                    MachineOrdinaryControlFlowRejectionReason::BootstrapSourceUnavailable {
                        register_index: operand.register_index,
                        source,
                    },
                ));
            }
        }

        if let Some(link) = result.link {
            if link.destination_gpr != 0 {
                return Some(MachineOrdinaryControlFlowRejection::new(
                    result,
                    MachineOrdinaryControlFlowRejectionReason::BootstrapLinkLineageUnavailable {
                        destination_gpr: link.destination_gpr,
                    },
                ));
            }
        }

        None
    }

    fn apply_ordinary_control_flow_step_action(
        &mut self,
        action: MachineOrdinaryControlFlowStepAction,
        control_flow_snapshot: CpuControlFlowSnapshot,
    ) -> MachineOrdinaryControlFlowStepApplication {
        let result = action.result();
        if let Some(link) = result.link() {
            self.cpu
                .set_gpr(usize::from(link.destination_gpr()), link.value())
                .expect("decoded CPU register index is five bits");
            self.record_known_bootstrap_gpr_destination(
                result.instruction_pc(),
                result.fields(),
                result.identity(),
            );
        }
        self.cpu
            .commit_ordinary_control_flow(control_flow_snapshot, result.selected_next_pc().value());
        self.cpu.advance_count_for_committed_step();

        MachineOrdinaryControlFlowStepApplication {
            result,
            cadence_plan: classify_machine_step_cadence(
                MachineStepCadenceSource::CommittedInstruction,
            ),
        }
    }

    fn produce_load_word_step_action(
        &self,
        execution_address: CpuAddress,
        fields: CpuInstructionFields,
    ) -> Result<MachineLoadWordStepAction, MachineLoadWordRejection> {
        let base_value = self
            .cpu
            .gpr(usize::from(fields.rs()))
            .expect("decoded CPU register index is five bits");
        let signed_immediate = i64::from(fields.immediate_u16() as i16);
        let effective_address = base_value.wrapping_add_signed(signed_immediate);
        let cpu_address = CpuAddress::new(effective_address as u32);

        let target = match classify_cpu_data_word_target(cpu_address) {
            Ok(target) => target,
            Err(MachineCpuDataWordTargetError::Unaligned { .. }) => {
                let address_error = select_cpu_data_address_error_for_access(
                    CpuDataAccessKind::Read,
                    cpu_address,
                    CpuDataWidth::Word,
                );
                return Ok(MachineLoadWordStepAction::EnterDataAddressError(
                    MachineLoadWordAddressErrorPlan {
                        fields,
                        effective_address,
                        address_error,
                    },
                ));
            }
            Err(MachineCpuDataWordTargetError::NonDirectUnsupported { .. }) => {
                return Err(MachineLoadWordRejection::new(
                    fields,
                    effective_address,
                    cpu_address,
                    None,
                    MachineLoadWordRejectionReason::NonDirectUnsupported,
                ));
            }
            Err(MachineCpuDataWordTargetError::DirectTargetMiss { .. }) => {
                return Err(MachineLoadWordRejection::new(
                    fields,
                    effective_address,
                    cpu_address,
                    None,
                    MachineLoadWordRejectionReason::DirectTargetMiss,
                ));
            }
        };

        let loaded_word = match target {
            MachineLoadWordTarget::DirectRdram { offset } => {
                self.rdram.read_u32_be(offset.as_usize()).map_err(|_| {
                    MachineLoadWordRejection::new(
                        fields,
                        effective_address,
                        cpu_address,
                        Some(target),
                        MachineLoadWordRejectionReason::DirectRdramReadRejected,
                    )
                })?
            }
            MachineLoadWordTarget::SpImem { offset } => {
                let known_word = self
                    .sp_imem
                    .read_known_u32_be(SpImemOffset::new(offset))
                    .map_err(|error| {
                        let reason = match error.unknown_offset() {
                            Some(unknown_offset) => MachineLoadWordRejectionReason::SpImemUnknown {
                                first_unknown_offset: unknown_offset.value(),
                            },
                            None => MachineLoadWordRejectionReason::SpImemReadRejected,
                        };
                        MachineLoadWordRejection::new(
                            fields,
                            effective_address,
                            cpu_address,
                            Some(target),
                            reason,
                        )
                    })?;
                known_word.value()
            }
        };
        let result_value = sign_extend_loaded_word(loaded_word);

        Ok(MachineLoadWordStepAction::Commit(
            MachineLoadWordCommitPlan {
                fields,
                execution_address,
                effective_address,
                target,
                loaded_word,
                result_value,
            },
        ))
    }

    fn apply_load_word_step_action(
        &mut self,
        action: MachineLoadWordStepAction,
        control_flow_snapshot: CpuControlFlowSnapshot,
    ) -> Result<MachineLoadWordStepApplication, MachineLoadWordStepApplicationError> {
        match action {
            MachineLoadWordStepAction::Commit(plan) => {
                self.cpu
                    .set_gpr(usize::from(plan.fields.rt()), plan.result_value)
                    .map_err(MachineLoadWordStepApplicationError::RegisterIndex)?;
                self.record_known_bootstrap_gpr_destination(
                    plan.execution_address,
                    plan.fields,
                    CpuInstructionIdentity::Lw,
                );
                self.cpu
                    .commit_staged_step_control_flow(control_flow_snapshot);
                self.cpu.advance_count_for_committed_step();
                let cadence_plan =
                    classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction);

                Ok(MachineLoadWordStepApplication::Committed { plan, cadence_plan })
            }
            MachineLoadWordStepAction::EnterDataAddressError(plan) => {
                self.cpu.restore_control_flow(control_flow_snapshot);
                self.cpu
                    .enter_data_address_error_exception(plan.address_error)
                    .map_err(MachineLoadWordStepApplicationError::DataAddressErrorEntry)?;
                let cadence_plan =
                    classify_machine_step_cadence(MachineStepCadenceSource::EnteredException);

                Ok(MachineLoadWordStepApplication::DataAddressError { plan, cadence_plan })
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn apply_cpu_local_committed_success_cadence(
        &mut self,
        action_plan: MachineCpuLocalInvocationStepActionPlan,
        control_flow_snapshot: CpuControlFlowSnapshot,
    ) -> Result<MachineCpuLocalCommittedSuccessCadence, MachineCpuLocalCommittedSuccessCadenceError>
    {
        match action_plan {
            MachineCpuLocalInvocationStepActionPlan::CommitControlFlowAndAdvanceCount {
                executed,
                cadence_plan,
            } => {
                self.cpu
                    .commit_staged_step_control_flow(control_flow_snapshot);
                self.cpu.advance_count_for_committed_step();
                Ok(MachineCpuLocalCommittedSuccessCadence {
                    executed,
                    cadence_plan,
                })
            }
            MachineCpuLocalInvocationStepActionPlan::EnterArithmeticOverflowException {
                ..
            }
            | MachineCpuLocalInvocationStepActionPlan::RejectInvocationError { .. } => {
                Err(MachineCpuLocalCommittedSuccessCadenceError::NonSuccessAction(action_plan))
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn apply_cpu_local_arithmetic_overflow_exception(
        &mut self,
        action_plan: MachineCpuLocalInvocationStepActionPlan,
        control_flow_snapshot: CpuControlFlowSnapshot,
    ) -> Result<
        MachineCpuLocalArithmeticOverflowException,
        MachineCpuLocalArithmeticOverflowExceptionError,
    > {
        match action_plan {
            MachineCpuLocalInvocationStepActionPlan::EnterArithmeticOverflowException {
                overflow,
            } => {
                self.cpu.restore_control_flow(control_flow_snapshot);
                self.cpu
                    .enter_arithmetic_overflow_exception()
                    .map_err(MachineCpuLocalArithmeticOverflowExceptionError::Entry)?;
                Ok(MachineCpuLocalArithmeticOverflowException { overflow })
            }
            MachineCpuLocalInvocationStepActionPlan::CommitControlFlowAndAdvanceCount {
                ..
            }
            | MachineCpuLocalInvocationStepActionPlan::RejectInvocationError { .. } => {
                Err(MachineCpuLocalArithmeticOverflowExceptionError::NonOverflowAction(action_plan))
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn apply_cpu_local_step_action(
        &mut self,
        action_plan: MachineCpuLocalInvocationStepActionPlan,
        control_flow_snapshot: CpuControlFlowSnapshot,
    ) -> Result<MachineCpuLocalStepActionApplication, MachineCpuLocalStepActionApplicationError>
    {
        match action_plan {
            MachineCpuLocalInvocationStepActionPlan::CommitControlFlowAndAdvanceCount {
                ..
            } => self
                .apply_cpu_local_committed_success_cadence(action_plan, control_flow_snapshot)
                .map(MachineCpuLocalStepActionApplication::CommittedSuccess)
                .map_err(MachineCpuLocalStepActionApplicationError::CommittedSuccess),
            MachineCpuLocalInvocationStepActionPlan::EnterArithmeticOverflowException {
                ..
            } => self
                .apply_cpu_local_arithmetic_overflow_exception(action_plan, control_flow_snapshot)
                .map(MachineCpuLocalStepActionApplication::ArithmeticOverflowException)
                .map_err(MachineCpuLocalStepActionApplicationError::ArithmeticOverflowException),
            MachineCpuLocalInvocationStepActionPlan::RejectInvocationError { error } => {
                Err(MachineCpuLocalStepActionApplicationError::RejectedInvocation(error))
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn apply_non_cpu_local_step_frontier_action(
        &mut self,
        action: MachineNonCpuLocalStepFrontierAction,
        control_flow_snapshot: CpuControlFlowSnapshot,
    ) -> Result<
        MachineNonCpuLocalStepFrontierApplication,
        MachineNonCpuLocalStepFrontierApplicationError,
    > {
        match action {
            MachineNonCpuLocalStepFrontierAction::NoEffectExecuted(instruction) => {
                let cadence_plan =
                    classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction);
                self.cpu
                    .commit_staged_step_control_flow(control_flow_snapshot);
                self.cpu.advance_count_for_committed_step();
                Ok(
                    MachineNonCpuLocalStepFrontierApplication::NoEffectExecuted {
                        instruction,
                        cadence_plan,
                    },
                )
            }
            MachineNonCpuLocalStepFrontierAction::Stopped(instruction) => {
                let cadence_plan =
                    classify_machine_step_cadence(MachineStepCadenceSource::StoppedInstruction);
                self.cpu
                    .commit_staged_step_control_flow(control_flow_snapshot);
                self.cpu.advance_count_for_committed_step();
                Ok(MachineNonCpuLocalStepFrontierApplication::Stopped {
                    instruction,
                    cadence_plan,
                })
            }
            MachineNonCpuLocalStepFrontierAction::Unsupported(instruction) => {
                let cadence_plan =
                    classify_machine_step_cadence(MachineStepCadenceSource::UnsupportedInstruction);
                self.cpu.restore_control_flow(control_flow_snapshot);
                Ok(MachineNonCpuLocalStepFrontierApplication::Unsupported {
                    instruction,
                    cadence_plan,
                })
            }
            MachineNonCpuLocalStepFrontierAction::FetchFault(fetch_action) => match fetch_action {
                MachineStepFetchFaultAction::EnterAddressError(plan) => {
                    let cadence_plan = classify_machine_step_cadence(
                        MachineStepCadenceSource::FetchAddressErrorException,
                    );
                    self.enter_instruction_fetch_address_error_exception(plan)
                        .map_err(
                            MachineNonCpuLocalStepFrontierApplicationError::FetchAddressErrorEntry,
                        )?;
                    Ok(
                        MachineNonCpuLocalStepFrontierApplication::FetchAddressErrorException {
                            plan,
                            cadence_plan,
                        },
                    )
                }
                MachineStepFetchFaultAction::Rethrow(fetch_error) => Err(
                    MachineNonCpuLocalStepFrontierApplicationError::FetchFaultRethrow(fetch_error),
                ),
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn apply_classified_step_action(
        &mut self,
        action: MachineClassifiedStepAction,
        control_flow_snapshot: CpuControlFlowSnapshot,
    ) -> Result<MachineClassifiedStepActionApplication, MachineClassifiedStepActionApplicationError>
    {
        match action {
            MachineClassifiedStepAction::CpuLocal(action_plan) => self
                .apply_cpu_local_step_action(action_plan, control_flow_snapshot)
                .map(MachineClassifiedStepActionApplication::CpuLocal)
                .map_err(MachineClassifiedStepActionApplicationError::CpuLocal),
            MachineClassifiedStepAction::OrdinaryControlFlow(action) => {
                Ok(MachineClassifiedStepActionApplication::OrdinaryControlFlow(
                    self.apply_ordinary_control_flow_step_action(action, control_flow_snapshot),
                ))
            }
            MachineClassifiedStepAction::LoadWord(action) => self
                .apply_load_word_step_action(action, control_flow_snapshot)
                .map(MachineClassifiedStepActionApplication::LoadWord)
                .map_err(MachineClassifiedStepActionApplicationError::LoadWord),
            MachineClassifiedStepAction::NonCpuLocalFrontier(frontier_action) => self
                .apply_non_cpu_local_step_frontier_action(frontier_action, control_flow_snapshot)
                .map(MachineClassifiedStepActionApplication::NonCpuLocalFrontier)
                .map_err(MachineClassifiedStepActionApplicationError::NonCpuLocalFrontier),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn produce_current_pc_classified_step_action(
        &mut self,
    ) -> Result<MachineCurrentPcClassifiedStepAction, MachineCurrentPcClassifiedStepActionError>
    {
        let control_flow_snapshot = self.cpu.capture_control_flow();

        let instruction_word = match self.fetch_current_cpu_instruction_word() {
            Ok(instruction_word) => instruction_word,
            Err(fetch_error) => {
                let fetch_action = classify_step_fetch_fault_action(fetch_error);
                self.cpu.restore_control_flow(control_flow_snapshot);

                return match fetch_action {
                    MachineStepFetchFaultAction::EnterAddressError(_) => {
                        Ok(MachineCurrentPcClassifiedStepAction {
                            control_flow_snapshot,
                            action: MachineClassifiedStepAction::NonCpuLocalFrontier(
                                MachineNonCpuLocalStepFrontierAction::FetchFault(fetch_action),
                            ),
                        })
                    }
                    MachineStepFetchFaultAction::Rethrow(fetch_error) => Err(
                        MachineCurrentPcClassifiedStepActionError::FetchFaultRethrow(fetch_error),
                    ),
                };
            }
        };

        let fields = decode_cpu_instruction_word(instruction_word);
        let identity = identify_cpu_instruction(fields);

        if control_flow_snapshot.delay_slot_context().is_some()
            && is_ordinary_control_flow_identity(identity)
        {
            return Ok(MachineCurrentPcClassifiedStepAction {
                control_flow_snapshot,
                action: MachineClassifiedStepAction::NonCpuLocalFrontier(
                    MachineNonCpuLocalStepFrontierAction::Unsupported(
                        MachineStepUnsupportedInstruction::new(
                            fields,
                            identity,
                            MachineStepUnsupportedInstructionCategory::ControlFlowInDelaySlot,
                        ),
                    ),
                ),
            });
        }

        if let Some(action) =
            self.produce_ordinary_control_flow_step_action(control_flow_snapshot, fields, identity)
        {
            if let Some(rejection) = self.ordinary_control_flow_rejection(action.result()) {
                return Err(
                    MachineCurrentPcClassifiedStepActionError::OrdinaryControlFlowRejected(
                        rejection,
                    ),
                );
            }
            return Ok(MachineCurrentPcClassifiedStepAction {
                control_flow_snapshot,
                action: MachineClassifiedStepAction::OrdinaryControlFlow(action),
            });
        }

        self.cpu.stage_next_sequential_pc_for_step();

        if let Some(instruction) = classify_step_no_effect_executed_instruction(fields, identity) {
            return Ok(MachineCurrentPcClassifiedStepAction {
                control_flow_snapshot,
                action: MachineClassifiedStepAction::NonCpuLocalFrontier(
                    MachineNonCpuLocalStepFrontierAction::NoEffectExecuted(instruction),
                ),
            });
        }

        if let Some(instruction) = classify_step_stopped_instruction(fields, identity) {
            return Ok(MachineCurrentPcClassifiedStepAction {
                control_flow_snapshot,
                action: MachineClassifiedStepAction::NonCpuLocalFrontier(
                    MachineNonCpuLocalStepFrontierAction::Stopped(instruction),
                ),
            });
        }

        if let Some(instruction) = classify_step_unsupported_instruction(fields, identity) {
            return Ok(MachineCurrentPcClassifiedStepAction {
                control_flow_snapshot,
                action: MachineClassifiedStepAction::NonCpuLocalFrontier(
                    MachineNonCpuLocalStepFrontierAction::Unsupported(instruction),
                ),
            });
        }

        if identity == CpuInstructionIdentity::Lw {
            let execution_address = CpuAddress::new(control_flow_snapshot.pc());
            if let Err(error) =
                self.require_known_bootstrap_gpr_sources(execution_address, fields, identity)
            {
                self.cpu.restore_control_flow(control_flow_snapshot);
                return Err(
                    MachineCurrentPcClassifiedStepActionError::BootstrapCpuStateUnavailable(error),
                );
            }

            return match self.produce_load_word_step_action(execution_address, fields) {
                Ok(action) => Ok(MachineCurrentPcClassifiedStepAction {
                    control_flow_snapshot,
                    action: MachineClassifiedStepAction::LoadWord(action),
                }),
                Err(rejection) => {
                    self.cpu.restore_control_flow(control_flow_snapshot);
                    Err(MachineCurrentPcClassifiedStepActionError::LoadWordRejected(
                        rejection,
                    ))
                }
            };
        }

        let Some(selection) = select_cpu_local_executed_helper(identity) else {
            self.cpu.restore_control_flow(control_flow_snapshot);
            return Err(
                MachineCurrentPcClassifiedStepActionError::UnrepresentedInstruction {
                    fields,
                    identity,
                },
            );
        };

        let execution_address = CpuAddress::new(control_flow_snapshot.pc());
        if let Err(error) =
            self.require_known_bootstrap_gpr_sources(execution_address, fields, identity)
        {
            self.cpu.restore_control_flow(control_flow_snapshot);
            return Err(
                MachineCurrentPcClassifiedStepActionError::BootstrapCpuStateUnavailable(error),
            );
        }

        match self.cpu.invoke_cpu_local_executed_helper(fields, selection) {
            Ok(outcome) => {
                if outcome.is_executed() {
                    self.record_known_bootstrap_gpr_destination(
                        execution_address,
                        fields,
                        identity,
                    );
                }
                Ok(MachineCurrentPcClassifiedStepAction {
                    control_flow_snapshot,
                    action: MachineClassifiedStepAction::CpuLocal(
                        classify_cpu_local_invocation_step_action(Ok(outcome)),
                    ),
                })
            }
            Err(error) => {
                self.cpu.restore_control_flow(control_flow_snapshot);
                Err(MachineCurrentPcClassifiedStepActionError::CpuLocalInvocation(error))
            }
        }
    }

    pub fn powered_on(&self) -> bool {
        self.powered_on
    }

    pub fn cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn rdram(&self) -> &Rdram {
        &self.rdram
    }

    pub fn sp_dmem(&self) -> &SpDmem {
        &self.sp_dmem
    }

    #[cfg(test)]
    pub(crate) fn stage_generated_sp_imem_word_for_test(
        &mut self,
        offset: u32,
        value: u32,
    ) -> Result<(), SpImemReadError> {
        self.sp_imem
            .stage_known_u32_be_for_test(SpImemOffset::new(offset), value)
    }

    #[cfg(test)]
    pub(crate) fn pif_firmware_bytes_for_test(&self) -> Option<&[u8]> {
        self.pif_firmware.as_ref().map(PifFirmware::bytes)
    }

    pub fn read_direct_rdram_u8(
        &self,
        cpu_address: CpuAddress,
    ) -> Result<u8, DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 1)?;
        self.rdram
            .read_u8(offset)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 1,
            })
    }

    pub fn read_direct_rdram_u16_be(
        &self,
        cpu_address: CpuAddress,
    ) -> Result<u16, DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 2)?;
        self.rdram
            .read_u16_be(offset)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 2,
            })
    }

    pub fn read_direct_rdram_u32_be(
        &self,
        cpu_address: CpuAddress,
    ) -> Result<u32, DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 4)?;
        self.rdram
            .read_u32_be(offset)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 4,
            })
    }

    pub fn read_direct_rdram_u64_be(
        &self,
        cpu_address: CpuAddress,
    ) -> Result<u64, DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 8)?;
        self.rdram
            .read_u64_be(offset)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 8,
            })
    }

    pub fn classify_cpu_instruction_fetch_target(
        cpu_address: CpuAddress,
    ) -> Result<MachineCpuInstructionFetchTarget, MachineCpuInstructionFetchTargetError> {
        if (cpu_address.value() & 0x3) != 0 {
            return Err(MachineCpuInstructionFetchTargetError::Unaligned { cpu_address });
        }

        let Some(physical_address) = translate_direct_cpu_physical_address(cpu_address) else {
            return Err(
                MachineCpuInstructionFetchTargetError::NonDirectUnsupported { cpu_address },
            );
        };

        if let CpuAddressTarget::DirectRdram(offset) =
            classify_direct_rdram_address(cpu_address, CPU_INSTRUCTION_FETCH_WIDTH)
        {
            return Ok(MachineCpuInstructionFetchTarget::DirectRdram { offset });
        }

        if let Some(offset) = translate_cpu_physical_sp_dmem_instruction_fetch_address(
            physical_address,
            CPU_INSTRUCTION_FETCH_WIDTH,
        ) {
            return Ok(MachineCpuInstructionFetchTarget::SpDmem {
                offset: SpDmemOffset::new(offset),
            });
        }

        if is_unavailable_pif_rom_reset_fetch(cpu_address, physical_address) {
            return Ok(MachineCpuInstructionFetchTarget::PifResetUnavailable);
        }

        Err(MachineCpuInstructionFetchTargetError::DirectTargetMiss { cpu_address })
    }

    pub fn fetch_direct_rdram_cpu_instruction_word(
        &self,
        cpu_address: CpuAddress,
    ) -> Result<CpuInstructionWord, MachineDirectRdramCpuInstructionFetchError> {
        if (cpu_address.value() & 0x3) != 0 {
            return Err(MachineDirectRdramCpuInstructionFetchError::Unaligned { cpu_address });
        }

        self.read_direct_rdram_u32_be(cpu_address)
            .map(CpuInstructionWord::new)
            .map_err(|source| MachineDirectRdramCpuInstructionFetchError::DirectRdram { source })
    }

    pub fn fetch_sp_dmem_cpu_instruction_word(
        &self,
        offset: SpDmemOffset,
    ) -> Result<CpuInstructionWord, MachineSpDmemCpuInstructionFetchError> {
        self.sp_dmem
            .read_u32_be(offset)
            .map(CpuInstructionWord::new)
            .map_err(|source| MachineSpDmemCpuInstructionFetchError { source })
    }

    pub fn fetch_cpu_instruction_word_at(
        &self,
        cpu_address: CpuAddress,
    ) -> Result<CpuInstructionWord, MachineCpuInstructionFetchError> {
        match Self::classify_cpu_instruction_fetch_target(cpu_address)
            .map_err(MachineCpuInstructionFetchError::from_target_error)?
        {
            MachineCpuInstructionFetchTarget::DirectRdram { .. } => self
                .fetch_direct_rdram_cpu_instruction_word(cpu_address)
                .map_err(|source| MachineCpuInstructionFetchError::DirectRdram {
                    cpu_address,
                    source,
                }),
            MachineCpuInstructionFetchTarget::SpDmem { offset } => self
                .fetch_sp_dmem_cpu_instruction_word(offset)
                .map_err(|source| MachineCpuInstructionFetchError::SpDmem {
                    cpu_address,
                    offset,
                    source,
                }),
            MachineCpuInstructionFetchTarget::PifResetUnavailable => {
                Err(MachineCpuInstructionFetchError::PifResetUnavailable { cpu_address })
            }
        }
    }

    pub fn fetch_current_cpu_instruction_word(
        &self,
    ) -> Result<CpuInstructionWord, MachineCpuInstructionFetchError> {
        self.fetch_cpu_instruction_word_at(CpuAddress::new(self.cpu.pc()))
    }

    pub fn enter_instruction_fetch_address_error_exception(
        &mut self,
        plan: MachineInstructionFetchAddressErrorPlan,
    ) -> Result<(), CpuAddressErrorExceptionEntryError> {
        self.cpu
            .enter_instruction_fetch_address_error_exception(plan.bad_vaddr())
    }

    pub fn write_direct_rdram_u8(
        &mut self,
        cpu_address: CpuAddress,
        value: u8,
    ) -> Result<(), DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 1)?;
        self.write_rdram_u8(offset, value)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 1,
            })
    }

    pub fn write_direct_rdram_u16_be(
        &mut self,
        cpu_address: CpuAddress,
        value: u16,
    ) -> Result<(), DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 2)?;
        self.write_rdram_u16_be(offset, value)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 2,
            })
    }

    pub fn write_direct_rdram_u32_be(
        &mut self,
        cpu_address: CpuAddress,
        value: u32,
    ) -> Result<(), DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 4)?;
        self.write_rdram_u32_be(offset, value)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 4,
            })
    }

    pub fn write_direct_rdram_u64_be(
        &mut self,
        cpu_address: CpuAddress,
        value: u64,
    ) -> Result<(), DirectRdramAccessError> {
        let offset = Self::direct_rdram_offset(cpu_address, 8)?;
        self.write_rdram_u64_be(offset, value)
            .map_err(|_| DirectRdramAccessError {
                cpu_address,
                width: 8,
            })
    }

    pub fn read_direct_rdram_cpu_data_u8(
        &mut self,
        cpu_address: CpuAddress,
    ) -> Result<u8, MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Read,
            cpu_address,
            CpuDataWidth::Byte,
        )?;
        match self.read_direct_rdram_u8(cpu_address) {
            Ok(value) => Ok(value),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Read,
                CpuDataWidth::Byte,
                source,
            )),
        }
    }

    pub fn read_direct_rdram_cpu_data_u16_be(
        &mut self,
        cpu_address: CpuAddress,
    ) -> Result<u16, MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Read,
            cpu_address,
            CpuDataWidth::Halfword,
        )?;
        match self.read_direct_rdram_u16_be(cpu_address) {
            Ok(value) => Ok(value),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Read,
                CpuDataWidth::Halfword,
                source,
            )),
        }
    }

    pub fn read_direct_rdram_cpu_data_u32_be(
        &mut self,
        cpu_address: CpuAddress,
    ) -> Result<u32, MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Read,
            cpu_address,
            CpuDataWidth::Word,
        )?;
        match self.read_direct_rdram_u32_be(cpu_address) {
            Ok(value) => Ok(value),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Read,
                CpuDataWidth::Word,
                source,
            )),
        }
    }

    pub fn read_direct_rdram_cpu_data_u64_be(
        &mut self,
        cpu_address: CpuAddress,
    ) -> Result<u64, MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Read,
            cpu_address,
            CpuDataWidth::Doubleword,
        )?;
        match self.read_direct_rdram_u64_be(cpu_address) {
            Ok(value) => Ok(value),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Read,
                CpuDataWidth::Doubleword,
                source,
            )),
        }
    }

    pub fn write_direct_rdram_cpu_data_u8(
        &mut self,
        cpu_address: CpuAddress,
        value: u8,
    ) -> Result<(), MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Write,
            cpu_address,
            CpuDataWidth::Byte,
        )?;
        match self.write_direct_rdram_u8(cpu_address, value) {
            Ok(()) => Ok(()),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Write,
                CpuDataWidth::Byte,
                source,
            )),
        }
    }

    pub fn write_direct_rdram_cpu_data_u16_be(
        &mut self,
        cpu_address: CpuAddress,
        value: u16,
    ) -> Result<(), MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Write,
            cpu_address,
            CpuDataWidth::Halfword,
        )?;
        match self.write_direct_rdram_u16_be(cpu_address, value) {
            Ok(()) => Ok(()),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Write,
                CpuDataWidth::Halfword,
                source,
            )),
        }
    }

    pub fn write_direct_rdram_cpu_data_u32_be(
        &mut self,
        cpu_address: CpuAddress,
        value: u32,
    ) -> Result<(), MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Write,
            cpu_address,
            CpuDataWidth::Word,
        )?;
        match self.write_direct_rdram_u32_be(cpu_address, value) {
            Ok(()) => Ok(()),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Write,
                CpuDataWidth::Word,
                source,
            )),
        }
    }

    pub fn write_direct_rdram_cpu_data_u64_be(
        &mut self,
        cpu_address: CpuAddress,
        value: u64,
    ) -> Result<(), MachineDirectRdramCpuDataAccessError> {
        self.preflight_direct_rdram_cpu_data_access(
            CpuDataAccessKind::Write,
            cpu_address,
            CpuDataWidth::Doubleword,
        )?;
        match self.write_direct_rdram_u64_be(cpu_address, value) {
            Ok(()) => Ok(()),
            Err(source) => Err(self.enter_direct_rdram_cpu_data_rejection_address_error(
                CpuDataAccessKind::Write,
                CpuDataWidth::Doubleword,
                source,
            )),
        }
    }

    pub fn write_rdram_u8(&mut self, offset: usize, value: u8) -> Result<(), RdramAccessError> {
        self.rdram.require_u8_offset(offset)?;
        self.cpu_rdram_reservation
            .invalidate_for_rdram_write(offset as u32, 1);
        self.rdram.write_u8_at_checked_offset(offset, value);
        Ok(())
    }

    pub fn write_rdram_u16_be(
        &mut self,
        offset: usize,
        value: u16,
    ) -> Result<(), RdramAccessError> {
        self.rdram.require_u16_be_offset(offset)?;
        self.cpu_rdram_reservation
            .invalidate_for_rdram_write(offset as u32, 2);
        self.rdram.write_u16_be_at_checked_offset(offset, value);
        Ok(())
    }

    pub fn write_rdram_u32_be(
        &mut self,
        offset: usize,
        value: u32,
    ) -> Result<(), RdramAccessError> {
        self.rdram.require_u32_be_offset(offset)?;
        self.cpu_rdram_reservation
            .invalidate_for_rdram_write(offset as u32, 4);
        self.rdram.write_u32_be_at_checked_offset(offset, value);
        Ok(())
    }

    pub fn write_rdram_u64_be(
        &mut self,
        offset: usize,
        value: u64,
    ) -> Result<(), RdramAccessError> {
        self.rdram.require_u64_be_offset(offset)?;
        self.cpu_rdram_reservation
            .invalidate_for_rdram_write(offset as u32, 8);
        self.rdram.write_u64_be_at_checked_offset(offset, value);
        Ok(())
    }

    fn direct_rdram_offset(
        cpu_address: CpuAddress,
        width: usize,
    ) -> Result<usize, DirectRdramAccessError> {
        match classify_direct_rdram_address(cpu_address, width) {
            CpuAddressTarget::DirectRdram(offset) => Ok(offset.as_usize()),
            CpuAddressTarget::Unsupported => Err(DirectRdramAccessError { cpu_address, width }),
        }
    }

    fn preflight_direct_rdram_cpu_data_access(
        &mut self,
        access_kind: CpuDataAccessKind,
        cpu_address: CpuAddress,
        width: CpuDataWidth,
    ) -> Result<(), MachineDirectRdramCpuDataAccessError> {
        let Err(alignment_error) = check_cpu_data_alignment(access_kind, cpu_address, width) else {
            return Ok(());
        };

        let address_error = select_cpu_data_address_error(alignment_error);
        Err(self.enter_direct_rdram_cpu_data_address_error(address_error))
    }

    fn enter_direct_rdram_cpu_data_rejection_address_error(
        &mut self,
        access_kind: CpuDataAccessKind,
        width: CpuDataWidth,
        source: DirectRdramAccessError,
    ) -> MachineDirectRdramCpuDataAccessError {
        let address_error =
            select_cpu_data_address_error_for_access(access_kind, source.cpu_address(), width);
        self.enter_direct_rdram_cpu_data_address_error(address_error)
    }

    fn enter_direct_rdram_cpu_data_address_error(
        &mut self,
        address_error: CpuDataAddressError,
    ) -> MachineDirectRdramCpuDataAccessError {
        match self.cpu.enter_data_address_error_exception(address_error) {
            Ok(()) => MachineDirectRdramCpuDataAccessError::AddressErrorEntered(address_error),
            Err(entry_error) => MachineDirectRdramCpuDataAccessError::AddressErrorEntryBlocked {
                address_error,
                entry_error,
            },
        }
    }
}

const fn is_ordinary_control_flow_identity(identity: CpuInstructionIdentity) -> bool {
    matches!(
        identity,
        CpuInstructionIdentity::Beq
            | CpuInstructionIdentity::Bne
            | CpuInstructionIdentity::J
            | CpuInstructionIdentity::Jal
            | CpuInstructionIdentity::SpecialJr
            | CpuInstructionIdentity::SpecialJalr
    )
}

const fn conditional_branch_target(instruction_pc: u32, immediate: u16) -> u32 {
    let displacement = (immediate as i16 as i32).wrapping_mul(4) as u32;
    instruction_pc.wrapping_add(4).wrapping_add(displacement)
}

const fn jump_target(instruction_pc: u32, instruction_index: u32) -> u32 {
    let region = instruction_pc.wrapping_add(4) & 0xf000_0000;
    region | ((instruction_index & 0x03ff_ffff) << 2)
}

const fn sign_extend_cpu_address(value: u32) -> u64 {
    (value as i32 as i64) as u64
}

fn classify_cpu_data_word_target(
    cpu_address: CpuAddress,
) -> Result<MachineLoadWordTarget, MachineCpuDataWordTargetError> {
    if (cpu_address.value() & 0x3) != 0 {
        return Err(MachineCpuDataWordTargetError::Unaligned { cpu_address });
    }

    let Some(physical_address) = translate_direct_cpu_physical_address(cpu_address) else {
        return Err(MachineCpuDataWordTargetError::NonDirectUnsupported { cpu_address });
    };

    if let CpuAddressTarget::DirectRdram(offset) =
        classify_direct_rdram_address(cpu_address, CPU_DATA_WORD_WIDTH)
    {
        return Ok(MachineLoadWordTarget::DirectRdram { offset });
    }

    if let Some(offset) =
        translate_cpu_physical_sp_imem_data_word_address(physical_address, CPU_DATA_WORD_WIDTH)
    {
        return Ok(MachineLoadWordTarget::SpImem { offset });
    }

    Err(MachineCpuDataWordTargetError::DirectTargetMiss { cpu_address })
}

fn translate_cpu_physical_sp_imem_data_word_address(
    physical_address: u32,
    width: usize,
) -> Option<u32> {
    if width == 0 || width > SP_IMEM_SIZE_BYTES || physical_address < SP_IMEM_PHYSICAL_BASE {
        return None;
    }

    let offset = physical_address - SP_IMEM_PHYSICAL_BASE;
    if (offset as usize) > SP_IMEM_SIZE_BYTES - width {
        return None;
    }

    Some(offset)
}

const fn sign_extend_loaded_word(value: u32) -> u64 {
    if (value & 0x8000_0000) == 0 {
        value as u64
    } else {
        0xffff_ffff_0000_0000 | value as u64
    }
}

fn translate_cpu_physical_sp_dmem_instruction_fetch_address(
    physical_address: u32,
    width: usize,
) -> Option<u32> {
    if width == 0 || width > SP_DMEM_SIZE_BYTES {
        return None;
    }

    if physical_address < SP_DMEM_PHYSICAL_BASE {
        return None;
    }

    let offset = physical_address - SP_DMEM_PHYSICAL_BASE;
    if (offset as usize) > SP_DMEM_SIZE_BYTES - width {
        return None;
    }

    Some(offset)
}

const fn is_unavailable_pif_rom_reset_fetch(
    cpu_address: CpuAddress,
    physical_address: u32,
) -> bool {
    cpu_address.value() == NON_BOOT_RESET_VECTOR_PC
        && physical_address == UNAVAILABLE_PIF_ROM_RESET_PHYSICAL_ADDRESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::{
        load_cartridge, RomMetadata, RomSourceLayout, CARTRIDGE_HEADER_ENTRY_WORD_OFFSET,
    };
    use crate::cpu::address::{
        classify_direct_rdram_address, CpuAddress, CpuAddressErrorKind, CpuAddressTarget,
        CpuDataAccessKind, CpuDataWidth, RdramOffset,
    };
    use crate::cpu::{
        decode_cpu_instruction_word, identify_cpu_instruction, select_cpu_local_executed_helper,
        CpuInstructionFields, CpuInstructionIdentity, CpuInstructionWord,
        CpuLocalExecutedHelperArithmeticOverflow, CpuLocalExecutedHelperExecutedInstruction,
        CpuLocalExecutedHelperFamily, CpuLocalExecutedHelperInvocationError,
        CpuLocalExecutedHelperInvocationOutcome, CPU_GPR_COUNT, NON_BOOT_RESET_VECTOR_NEXT_PC,
        NON_BOOT_RESET_VECTOR_PC,
    };
    use crate::pif_firmware::{
        PifFirmwareClassification, PifIpl2Profile, PIF_BOOT_ROM_SIZE_BYTES,
        PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
    };
    use crate::rdram::RDRAM_SIZE_BYTES;

    fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset] = ((value >> 24) & 0xff) as u8;
        bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
        bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
        bytes[offset + 3] = (value & 0xff) as u8;
    }

    fn kseg0(offset: usize) -> CpuAddress {
        CpuAddress::new(0x8000_0000 + offset as u32)
    }

    fn kseg1(offset: usize) -> CpuAddress {
        CpuAddress::new(0xa000_0000 + offset as u32)
    }

    fn generated_pif_firmware(seed: u8, size: usize) -> Vec<u8> {
        (0..size)
            .map(|index| seed.wrapping_add((index as u8).wrapping_mul(29)))
            .collect()
    }

    fn instruction_fields(bits: u32) -> CpuInstructionFields {
        decode_cpu_instruction_word(CpuInstructionWord::new(bits))
    }

    fn classify_step_unsupported_instruction(
        fields: CpuInstructionFields,
    ) -> Option<MachineStepUnsupportedInstruction> {
        super::classify_step_unsupported_instruction(fields, identify_cpu_instruction(fields))
    }

    fn classify_step_stopped_instruction(
        fields: CpuInstructionFields,
    ) -> Option<MachineStepStoppedInstruction> {
        super::classify_step_stopped_instruction(fields, identify_cpu_instruction(fields))
    }

    fn classify_step_no_effect_executed_instruction(
        fields: CpuInstructionFields,
    ) -> Option<MachineStepNoEffectExecutedInstruction> {
        super::classify_step_no_effect_executed_instruction(
            fields,
            identify_cpu_instruction(fields),
        )
    }

    fn special_shift_word(rs: u8, rt: u8, rd: u8, sa: u8, funct: u8) -> u32 {
        (u32::from(rs) << 21)
            | (u32::from(rt) << 16)
            | (u32::from(rd) << 11)
            | (u32::from(sa) << 6)
            | u32::from(funct)
    }

    fn immediate_word(opcode: u8, rs: u8, rt: u8, immediate: u16) -> u32 {
        (u32::from(opcode) << 26)
            | (u32::from(rs) << 21)
            | (u32::from(rt) << 16)
            | u32::from(immediate)
    }

    const COP0_STATUS_EXL: u32 = 0x0000_0002;
    const LOCAL_EXCEPTION_VECTOR_PC: u32 = 0x8000_0180;
    const LOCAL_EXCEPTION_VECTOR_NEXT_PC: u32 = 0x8000_0184;

    fn assert_default_cpu_exception_state(machine: &Machine) {
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu_delay_slot_context(), None);
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    fn assert_entered_data_address_error(
        error: MachineDirectRdramCpuDataAccessError,
        address: CpuAddress,
        access_kind: CpuDataAccessKind,
        width: CpuDataWidth,
        exception_kind: CpuAddressErrorKind,
    ) {
        assert_eq!(error.cpu_address(), address);
        assert_eq!(error.access_kind(), access_kind);
        assert_eq!(error.width(), width);
        assert_eq!(error.exception_kind(), Some(exception_kind));
        assert!(error.entry_error().is_none());
        assert!(error.direct_rdram_error().is_none());

        let address_error = error.address_error().unwrap();
        assert_eq!(address_error.address(), address);
        assert_eq!(address_error.bad_vaddr(), address);
        assert_eq!(address_error.access_kind(), access_kind);
        assert_eq!(address_error.width(), width);
        assert_eq!(address_error.exception_kind(), exception_kind);
        assert_eq!(
            address_error.cause_exception_code(),
            exception_kind.cause_exception_code()
        );
    }

    fn assert_unaligned_fetch_error(
        error: MachineDirectRdramCpuInstructionFetchError,
        address: CpuAddress,
    ) {
        assert_eq!(error.cpu_address(), address);
        assert_eq!(error.width(), 4);
        assert!(error.is_unaligned());
        assert!(error.direct_rdram_error().is_none());
        assert_eq!(
            error.to_string(),
            format!(
                "direct RDRAM CPU instruction fetch requires 4-byte aligned PC: {}",
                address.value()
            )
        );
    }

    fn assert_direct_fetch_error(
        error: MachineDirectRdramCpuInstructionFetchError,
        address: CpuAddress,
    ) {
        assert_eq!(error.cpu_address(), address);
        assert_eq!(error.width(), 4);
        assert!(!error.is_unaligned());
        assert_eq!(
            error.direct_rdram_error(),
            Some(DirectRdramAccessError {
                cpu_address: address,
                width: 4,
            })
        );
    }

    fn assert_fetch_target_unaligned_error(
        error: MachineCpuInstructionFetchTargetError,
        address: CpuAddress,
    ) {
        assert_eq!(error.cpu_address(), address);
        assert_eq!(error.width(), 4);
        assert!(error.is_unaligned());
        assert!(!error.is_non_direct_unsupported());
        assert!(!error.is_direct_target_miss());
    }

    fn assert_fetch_target_non_direct_error(
        error: MachineCpuInstructionFetchTargetError,
        address: CpuAddress,
    ) {
        assert_eq!(error.cpu_address(), address);
        assert_eq!(error.width(), 4);
        assert!(!error.is_unaligned());
        assert!(error.is_non_direct_unsupported());
        assert!(!error.is_direct_target_miss());
    }

    fn assert_fetch_target_direct_miss_error(
        error: MachineCpuInstructionFetchTargetError,
        address: CpuAddress,
    ) {
        assert_eq!(error.cpu_address(), address);
        assert_eq!(error.width(), 4);
        assert!(!error.is_unaligned());
        assert!(!error.is_non_direct_unsupported());
        assert!(error.is_direct_target_miss());
    }

    fn assert_instruction_fetch_address_error_plan(
        plan: MachineInstructionFetchAddressErrorPlan,
        fetch_error: MachineCpuInstructionFetchError,
        source: MachineInstructionFetchAddressErrorSource,
        address: CpuAddress,
    ) {
        assert_eq!(plan.fetch_error(), fetch_error);
        assert_eq!(plan.source(), source);
        assert_eq!(plan.cpu_address(), address);
        assert_eq!(plan.bad_vaddr(), address);
        assert_eq!(plan.width(), 4);
        assert_eq!(plan.exception_kind(), CpuAddressErrorKind::AddressErrorLoad);
        assert_eq!(plan.cause_exception_code(), 4);
    }

    fn assert_step_fetch_fault_enters_address_error(
        action: MachineStepFetchFaultAction,
        fetch_error: MachineCpuInstructionFetchError,
        source: MachineInstructionFetchAddressErrorSource,
        address: CpuAddress,
    ) {
        assert_eq!(action.fetch_error(), fetch_error);
        assert_eq!(action.cpu_address(), address);
        assert_eq!(action.width(), 4);
        assert!(action.is_enter_address_error());
        assert!(!action.is_rethrow());
        let plan = action.address_error_plan().unwrap();
        assert_instruction_fetch_address_error_plan(plan, fetch_error, source, address);
    }

    fn assert_step_fetch_fault_rethrows(
        action: MachineStepFetchFaultAction,
        fetch_error: MachineCpuInstructionFetchError,
        address: CpuAddress,
    ) {
        assert_eq!(action.fetch_error(), fetch_error);
        assert_eq!(action.cpu_address(), address);
        assert_eq!(action.width(), 4);
        assert_eq!(action.address_error_plan(), None);
        assert!(!action.is_enter_address_error());
        assert!(action.is_rethrow());
    }

    fn assert_step_unsupported_instruction(
        outcome: MachineStepUnsupportedInstruction,
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
        category: MachineStepUnsupportedInstructionCategory,
    ) {
        assert_eq!(outcome.fields(), fields);
        assert_eq!(outcome.raw(), fields.raw());
        assert_eq!(outcome.identity(), identity);
        assert_eq!(outcome.category(), category);
    }

    fn assert_step_stopped_instruction(
        outcome: MachineStepStoppedInstruction,
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
        category: MachineStepStoppedInstructionCategory,
    ) {
        assert_eq!(outcome.fields(), fields);
        assert_eq!(outcome.raw(), fields.raw());
        assert_eq!(outcome.identity(), identity);
        assert_eq!(outcome.category(), category);
    }

    fn assert_step_no_effect_executed_instruction(
        outcome: MachineStepNoEffectExecutedInstruction,
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
        category: MachineStepNoEffectExecutedInstructionCategory,
    ) {
        assert_eq!(outcome.fields(), fields);
        assert_eq!(outcome.raw(), fields.raw());
        assert_eq!(outcome.identity(), identity);
        assert_eq!(outcome.category(), category);
    }

    fn assert_machine_step_cadence_plan(
        source: MachineStepCadenceSource,
        control_flow_action: MachineStepControlFlowAction,
        count_action: MachineStepCountAction,
    ) {
        let plan = classify_machine_step_cadence(source);

        assert_eq!(plan.source(), source);
        assert_eq!(plan.control_flow_action(), control_flow_action);
        assert_eq!(plan.count_action(), count_action);
        assert_eq!(
            plan.advances_count(),
            matches!(count_action, MachineStepCountAction::Advance)
        );
        assert!(!plan.mutates_state());
    }

    fn invoke_cpu_local_executed_helper_for_step_action(
        machine: &mut Machine,
        bits: u32,
    ) -> Result<CpuLocalExecutedHelperInvocationOutcome, CpuLocalExecutedHelperInvocationError>
    {
        let fields = instruction_fields(bits);
        let identity = identify_cpu_instruction(fields);
        let selection = select_cpu_local_executed_helper(identity)
            .expect("identity should select a sealed CPU-local executed helper family");

        machine
            .cpu
            .invoke_cpu_local_executed_helper(fields, selection)
    }

    fn assert_committed_local_step_action(
        plan: MachineCpuLocalInvocationStepActionPlan,
        identity: CpuInstructionIdentity,
        family: CpuLocalExecutedHelperFamily,
    ) {
        assert_eq!(
            plan.action(),
            MachineCpuLocalInvocationStepAction::CommitControlFlowAndAdvanceCount
        );
        assert!(!plan.mutates_state());

        let executed = plan
            .executed()
            .expect("committed local step plan should preserve executed outcome");
        assert_eq!(executed.identity(), identity);
        assert_eq!(executed.family(), family);
        assert_eq!(plan.overflow(), None);
        assert_eq!(plan.invocation_error(), None);

        let cadence_plan = plan
            .cadence_plan()
            .expect("committed local step plan should reference cadence planning");
        assert_eq!(
            cadence_plan,
            classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction)
        );
        assert_eq!(
            cadence_plan.control_flow_action(),
            MachineStepControlFlowAction::CommitStaged
        );
        assert_eq!(cadence_plan.count_action(), MachineStepCountAction::Advance);
        assert!(cadence_plan.advances_count());
        assert!(!cadence_plan.mutates_state());
    }

    fn assert_overflow_local_step_action(
        plan: MachineCpuLocalInvocationStepActionPlan,
        identity: CpuInstructionIdentity,
        family: CpuLocalExecutedHelperFamily,
    ) {
        assert_eq!(
            plan.action(),
            MachineCpuLocalInvocationStepAction::EnterArithmeticOverflowException
        );
        assert!(!plan.mutates_state());
        assert_eq!(plan.executed(), None);
        assert_eq!(plan.cadence_plan(), None);
        assert_eq!(plan.invocation_error(), None);

        let overflow = plan
            .overflow()
            .expect("overflow local step plan should preserve overflow data");
        assert_eq!(overflow.identity(), identity);
        assert_eq!(overflow.family(), family);
    }

    fn assert_arithmetic_overflow_exception_entry(
        machine: &Machine,
        epc: u32,
        branch_delay: bool,
        bad_vaddr: u32,
        count: u32,
    ) {
        assert_eq!(machine.cpu().pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().cop0_exception_code(), 12);
        assert_eq!(machine.cpu().cop0_epc(), epc);
        assert_eq!(machine.cpu().cop0_exception_branch_delay(), branch_delay);
        assert_eq!(
            machine.cpu().cop0_status() & COP0_STATUS_EXL,
            COP0_STATUS_EXL
        );
        assert_eq!(machine.cpu().cop0_bad_vaddr(), bad_vaddr);
        assert_eq!(machine.cpu().cop0_count(), count);
    }

    fn committed_cpu_local_success_action_plan(
        identity: CpuInstructionIdentity,
        family: CpuLocalExecutedHelperFamily,
    ) -> MachineCpuLocalInvocationStepActionPlan {
        classify_cpu_local_invocation_step_action(Ok(
            CpuLocalExecutedHelperInvocationOutcome::Executed(
                CpuLocalExecutedHelperExecutedInstruction::new_for_test(identity, family),
            ),
        ))
    }

    fn special_trapping_overflow_action_plan() -> MachineCpuLocalInvocationStepActionPlan {
        classify_cpu_local_invocation_step_action(Ok(
            CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow(
                CpuLocalExecutedHelperArithmeticOverflow::special_trapping_integer_for_test(
                    CpuInstructionIdentity::SpecialAdd,
                    3,
                    0x0000_0000_7fff_ffff,
                    1,
                ),
            ),
        ))
    }

    fn immediate_trapping_overflow_action_plan() -> MachineCpuLocalInvocationStepActionPlan {
        classify_cpu_local_invocation_step_action(Ok(
            CpuLocalExecutedHelperInvocationOutcome::ArithmeticOverflow(
                CpuLocalExecutedHelperArithmeticOverflow::immediate_trapping_integer_for_test(
                    CpuInstructionIdentity::Addi,
                    2,
                    0x0000_0000_7fff_ffff,
                    1,
                    1,
                ),
            ),
        ))
    }

    fn assert_entered_instruction_fetch_address_error(
        machine: &Machine,
        bad_vaddr: CpuAddress,
        epc: CpuAddress,
        count_before: u32,
    ) {
        assert_eq!(machine.cpu().cop0_bad_vaddr(), bad_vaddr.value());
        assert_eq!(machine.cpu().cop0_exception_code(), 4);
        assert!(!machine.cpu().cop0_exception_branch_delay());
        assert_eq!(machine.cpu().cop0_epc(), epc.value());
        assert_eq!(
            machine.cpu().cop0_status() & COP0_STATUS_EXL,
            COP0_STATUS_EXL
        );
        assert_eq!(machine.cpu().pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().cop0_count(), count_before);
    }

    fn assert_machine_reset_owned_state(machine: &Machine) {
        assert!(machine.powered_on());
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu_delay_slot_context(), None);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        for index in 0..CPU_GPR_COUNT {
            assert_eq!(machine.cpu().gpr(index), Some(0));
        }
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
        assert_eq!(machine.rdram().read_u8(0x20), Ok(0));
        assert_eq!(machine.rdram().read_u8(RDRAM_SIZE_BYTES - 1), Ok(0));
        assert_eq!(machine.sp_dmem().size_bytes(), SP_DMEM_SIZE_BYTES);
        assert_eq!(machine.sp_dmem().read_u8(SpDmemOffset::new(0)), Ok(0));
        assert_eq!(
            machine
                .sp_dmem()
                .read_u8(SpDmemOffset::new((SP_DMEM_SIZE_BYTES - 1) as u32)),
            Ok(0)
        );
        assert_eq!(machine.sp_imem.size_bytes(), SP_IMEM_SIZE_BYTES);
        for offset in [0, SP_IMEM_SIZE_BYTES - 1] {
            let observed = machine
                .sp_imem
                .observe_byte(SpImemOffset::new(offset as u32))
                .unwrap();
            assert_eq!(observed.value(), 0);
            assert!(!observed.is_known());
        }
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
    }

    fn dirty_represented_machine_reset_state(machine: &mut Machine) {
        machine
            .write_rdram_u64_be(0x20, 0x0123_4567_89ab_cdef)
            .unwrap();
        machine
            .write_rdram_u32_be(RDRAM_SIZE_BYTES - 4, 0xaabb_ccdd)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine
            .stage_generated_sp_imem_word_for_test(0, 0x89ab_cdef)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x20, 8);
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_1004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(1, 0x0102_0304_0506_0708), Ok(()));
        assert_eq!(machine.cpu.set_gpr(31, 0xfedc_ba98_7654_3210), Ok(()));
        assert!(machine
            .read_direct_rdram_cpu_data_u32_be(kseg0(0x0000_0001))
            .is_err());
    }

    fn make_synthetic_normalized_rom_proof_image() -> Vec<u8> {
        let mut rom = vec![0; 0x60];
        write_be_u32(&mut rom, 0x00, 0x8037_1240);
        write_be_u32(&mut rom, 0x04, 0x1234_5678);
        write_be_u32(
            &mut rom,
            CARTRIDGE_HEADER_ENTRY_WORD_OFFSET as usize,
            0x8024_6000,
        );
        write_be_u32(&mut rom, 0x0C, 0x0040_0000);
        write_be_u32(&mut rom, 0x10, 0x89AB_CDEF);
        write_be_u32(&mut rom, 0x14, 0x0123_4567);

        for (index, ch) in b"FN64 MACHINE PROOF".iter().enumerate() {
            rom[0x20 + index] = *ch;
        }

        rom[0x3C] = b'M';
        rom[0x3D] = b'C';
        rom[0x3E] = 0x45;
        rom[0x3F] = 0x03;

        for (offset, byte) in rom.iter_mut().enumerate().skip(0x40) {
            *byte = ((offset * 7 + 0x19) & 0xff) as u8;
        }

        rom
    }

    #[test]
    fn machine_from_empty_cartridge_is_powered_on_and_owns_cartridge() {
        let machine = Machine::from_cartridge(Cartridge::default());

        assert!(machine.powered_on());
        assert_eq!(
            machine.cartridge().source_layout(),
            RomSourceLayout::BigEndian
        );
        assert_eq!(machine.cartridge().size_bytes(), 0);
        assert_eq!(machine.cartridge().metadata(), &RomMetadata::default());
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.sp_dmem().size_bytes(), SP_DMEM_SIZE_BYTES);
    }

    #[test]
    fn machine_from_loaded_cartridge_preserves_cartridge_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let cartridge = load_cartridge(normalized_bytes.clone()).unwrap();
        let machine = Machine::from_cartridge(cartridge);
        let machine_cartridge = machine.cartridge();

        assert!(machine.powered_on());
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.sp_dmem().size_bytes(), SP_DMEM_SIZE_BYTES);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(
            machine_cartridge.source_layout(),
            RomSourceLayout::BigEndian
        );
        assert_eq!(machine_cartridge.size_bytes(), normalized_bytes.len());
        assert_eq!(machine_cartridge.metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine_cartridge.metadata().image_name,
            "FN64 MACHINE PROOF"
        );
        assert_eq!(machine_cartridge.metadata().cartridge_id, "MC");
        assert_eq!(
            machine_cartridge.read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(
            machine_cartridge
                .read_u8((normalized_bytes.len() - 1) as u32)
                .unwrap(),
            *normalized_bytes.last().unwrap()
        );
    }

    #[test]
    fn machine_pif_firmware_is_absent_until_explicit_owned_installation() {
        let machine = Machine::from_cartridge(Cartridge::default());

        assert_eq!(
            machine.pif_firmware_state(),
            MachinePifFirmwareState::Absent
        );
        assert!(machine.pif_firmware_bytes_for_test().is_none());
    }

    #[test]
    fn machine_owns_accepted_pif_firmware_independently_of_caller_storage() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let mut caller_bytes = generated_pif_firmware(0x31, PIF_BOOT_ROM_SIZE_BYTES);
        let expected_bytes = caller_bytes.clone();

        let state = machine
            .install_pif_firmware(PifIpl2Profile::NtscPinned, caller_bytes.clone())
            .unwrap();
        caller_bytes.fill(0xff);

        assert_eq!(
            state,
            MachinePifFirmwareState::Accepted {
                classification: PifFirmwareClassification::RawBootRom,
                profile: PifIpl2Profile::NtscPinned,
                size_bytes: PIF_BOOT_ROM_SIZE_BYTES,
            }
        );
        assert_eq!(machine.pif_firmware_state(), state);
        assert_eq!(
            machine.pif_firmware_bytes_for_test(),
            Some(expected_bytes.as_slice())
        );
        assert_ne!(
            machine.pif_firmware_bytes_for_test(),
            Some(caller_bytes.as_slice())
        );
    }

    #[test]
    fn machine_reset_preserves_accepted_pif_firmware_like_cartridge_input() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let expected_bytes = generated_pif_firmware(0x42, PIF_BOOT_ROM_SIZE_BYTES);
        let accepted = machine
            .install_pif_firmware(PifIpl2Profile::PalPinned, expected_bytes.clone())
            .unwrap();
        machine.stage_cpu_pc(0x8000_2000);
        machine.write_rdram_u32_be(0x20, 0x1122_3344).unwrap();

        machine.reset();

        assert_eq!(machine.pif_firmware_state(), accepted);
        assert_eq!(
            machine.pif_firmware_bytes_for_test(),
            Some(expected_bytes.as_slice())
        );
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.rdram().read_u32_be(0x20), Ok(0));
    }

    #[test]
    fn rejected_pif_firmware_replacement_has_no_partial_machine_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let accepted_bytes = generated_pif_firmware(0x53, PIF_BOOT_ROM_SIZE_BYTES);
        machine
            .install_pif_firmware(PifIpl2Profile::MpalPinned, accepted_bytes.clone())
            .unwrap();
        machine.stage_cpu_pc(0x8000_3000);
        machine.write_rdram_u32_be(0x30, 0xaabb_ccdd).unwrap();
        let before = lw_snapshot(&machine);

        let malformed = machine
            .install_pif_firmware(
                PifIpl2Profile::NtscPinned,
                generated_pif_firmware(0x64, PIF_BOOT_ROM_SIZE_BYTES - 1),
            )
            .unwrap_err();
        assert!(malformed.is_malformed());
        assert_eq!(lw_snapshot(&machine), before);

        let unsupported = machine
            .install_pif_firmware(
                PifIpl2Profile::PalPinned,
                generated_pif_firmware(0x75, PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES),
            )
            .unwrap_err();
        assert!(unsupported.is_unsupported());
        assert_eq!(lw_snapshot(&machine), before);
        assert_eq!(
            machine.pif_firmware_bytes_for_test(),
            Some(accepted_bytes.as_slice())
        );
    }

    #[test]
    fn machine_from_cartridge_owns_cpu_construction_state() {
        let machine = Machine::from_cartridge(Cartridge::default());
        let cpu = machine.cpu();

        assert_eq!(cpu.pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(cpu.next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(cpu.hi(), 0);
        assert_eq!(cpu.lo(), 0);
        assert_eq!(cpu.gpr(0), Some(0));
        assert_eq!(cpu.gpr(1), Some(0));
        assert_eq!(cpu.gpr(8), Some(0));
        assert_eq!(cpu.gpr(31), Some(0));
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.cartridge().size_bytes(), 0);
    }

    #[test]
    fn machine_from_cartridge_owns_default_cpu_rdram_reservation_state() {
        let machine = Machine::from_cartridge(Cartridge::default());

        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), 0);
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn machine_reset_restores_represented_non_boot_power_on_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        dirty_represented_machine_reset_state(&mut machine);
        assert_ne!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_ne!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_ne!(machine.rdram().read_u8(0x20), Ok(0));
        assert!(machine.cpu_rdram_reservation.is_valid());

        machine.reset();

        assert_machine_reset_owned_state(&machine);
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().metadata().image_name,
            "FN64 MACHINE PROOF"
        );
        assert_eq!(machine.cartridge().metadata().cartridge_id, "MC");
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(
            machine
                .cartridge()
                .read_u8((normalized_bytes.len() - 1) as u32)
                .unwrap(),
            *normalized_bytes.last().unwrap()
        );
    }

    #[test]
    fn machine_reset_is_repeatable_and_construction_equivalent_for_represented_state() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        dirty_represented_machine_reset_state(&mut machine);
        machine.reset();
        machine.reset();

        assert_machine_reset_owned_state(&machine);
        assert_eq!(
            machine.cartridge().source_layout(),
            RomSourceLayout::BigEndian
        );
        assert_eq!(machine.cartridge().size_bytes(), 0);
        assert_eq!(machine.cartridge().metadata(), &RomMetadata::default());
    }

    #[test]
    fn machine_reset_after_direct_rdram_cpu_data_address_error_does_not_execute_or_write_back() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.write_direct_rdram_u8(kseg0(0x40), 0x5a).unwrap();
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        let error = machine
            .write_direct_rdram_cpu_data_u32_be(kseg0(0x0000_0041), 0xaabb_ccdd)
            .unwrap_err();
        assert_entered_data_address_error(
            error,
            kseg0(0x0000_0041),
            CpuDataAccessKind::Write,
            CpuDataWidth::Word,
            CpuAddressErrorKind::AddressErrorStore,
        );
        assert_eq!(machine.rdram().read_u8(0x40), Ok(0x5a));
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));

        machine.reset();

        assert_machine_reset_owned_state(&machine);
        assert_eq!(machine.cartridge().size_bytes(), 0);
    }

    #[test]
    fn machine_private_reservation_staging_preserves_earned_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.cpu_rdram_reservation.stage(0x0010_0200, 8);

        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0010_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
        assert_eq!(machine.rdram().read_u8(RDRAM_SIZE_BYTES - 1), Ok(0));
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn machine_private_reservation_invalidation_preserves_earned_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.cpu_rdram_reservation.stage(0x0010_0200, 8);
        machine
            .cpu_rdram_reservation
            .invalidate_for_rdram_write(0x0010_0204, 1);

        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
        assert_eq!(machine.rdram().read_u8(RDRAM_SIZE_BYTES - 1), Ok(0));
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn raw_rdram_byte_write_updates_first_and_last_storage_offsets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_offset = RDRAM_SIZE_BYTES - 1;

        machine.write_rdram_u8(0, 0xab).unwrap();
        machine.write_rdram_u8(last_offset, 0xcd).unwrap();

        assert_eq!(machine.rdram().read_u8(0), Ok(0xab));
        assert_eq!(machine.rdram().read_u8(1), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_offset - 1), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_offset), Ok(0xcd));
    }

    #[test]
    fn raw_rdram_byte_write_rejects_invalid_offsets_before_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        let exact_end_error = machine.write_rdram_u8(RDRAM_SIZE_BYTES, 0xab).unwrap_err();
        let past_end_error = machine
            .write_rdram_u8(RDRAM_SIZE_BYTES + 1, 0xcd)
            .unwrap_err();

        assert_eq!(exact_end_error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(exact_end_error.width(), 1);
        assert_eq!(
            exact_end_error.to_string(),
            "RDRAM access out of range: address=4194304 width=1"
        );
        assert_eq!(past_end_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_end_error.width(), 1);
        assert_eq!(
            past_end_error.to_string(),
            "RDRAM access out of range: address=4194305 width=1"
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
        assert_eq!(machine.rdram().read_u8(RDRAM_SIZE_BYTES - 1), Ok(0));
    }

    #[test]
    fn raw_rdram_byte_write_invalidates_only_overlapping_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        machine.write_rdram_u8(0x0000_00ff, 0x11).unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);

        machine.write_rdram_u8(0x0000_0104, 0x22).unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);

        machine.write_rdram_u8(0x0000_0100, 0x33).unwrap();
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_00ff), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0x33));
        assert_eq!(machine.rdram().read_u8(0x0000_0104), Ok(0x22));
    }

    #[test]
    fn raw_rdram_byte_write_uses_latest_staged_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        machine.cpu_rdram_reservation.stage(0x0000_0200, 4);
        machine.write_rdram_u8(0x0000_0100, 0x44).unwrap();

        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0x44));

        machine.write_rdram_u8(0x0000_0200, 0x55).unwrap();

        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_0200), Ok(0x55));
    }

    #[test]
    fn raw_rdram_byte_write_preserves_unrelated_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.cpu_rdram_reservation.stage(0x0010_0200, 8);
        machine.write_rdram_u8(0x0010_0204, 0xef).unwrap();

        assert_eq!(machine.rdram().read_u8(0x0010_0203), Ok(0));
        assert_eq!(machine.rdram().read_u8(0x0010_0204), Ok(0xef));
        assert_eq!(machine.rdram().read_u8(0x0010_0205), Ok(0));
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn direct_rdram_address_classification_preserves_machine_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.write_rdram_u8(0x0000_0020, 0x5a).unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);

        let target = classify_direct_rdram_address(CpuAddress::new(0x8000_0100), 4);

        assert_eq!(
            target,
            CpuAddressTarget::DirectRdram(RdramOffset::new(0x100))
        );
        assert_eq!(machine.rdram().read_u8(0x0000_001f), Ok(0));
        assert_eq!(machine.rdram().read_u8(0x0000_0020), Ok(0x5a));
        assert_eq!(machine.rdram().read_u8(0x0000_0021), Ok(0));
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn raw_rdram_u16_be_write_updates_first_and_last_valid_storage_offsets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 2;

        machine.write_rdram_u16_be(0, 0xabcd).unwrap();
        machine
            .write_rdram_u16_be(last_valid_offset, 0x1234)
            .unwrap();

        assert_eq!(machine.rdram().read_u8(0), Ok(0xab));
        assert_eq!(machine.rdram().read_u8(1), Ok(0xcd));
        assert_eq!(machine.rdram().read_u8(2), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset - 1), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x12));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x34));
    }

    #[test]
    fn raw_rdram_u16_be_write_accepts_odd_storage_offset_without_alignment_check() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.write_rdram_u16_be(3, 0xcafe).unwrap();

        assert_eq!(machine.rdram().read_u8(2), Ok(0));
        assert_eq!(machine.rdram().read_u8(3), Ok(0xca));
        assert_eq!(machine.rdram().read_u8(4), Ok(0xfe));
        assert_eq!(machine.rdram().read_u8(5), Ok(0));
    }

    #[test]
    fn raw_rdram_u16_be_write_rejects_invalid_offsets_before_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 2;
        let last_byte_offset = RDRAM_SIZE_BYTES - 1;

        machine
            .write_rdram_u16_be(last_valid_offset, 0x1122)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        let last_byte_error = machine
            .write_rdram_u16_be(last_byte_offset, 0xaabb)
            .unwrap_err();
        let exact_end_error = machine
            .write_rdram_u16_be(RDRAM_SIZE_BYTES, 0xccdd)
            .unwrap_err();
        let past_end_error = machine
            .write_rdram_u16_be(RDRAM_SIZE_BYTES + 1, 0xeeff)
            .unwrap_err();

        assert_eq!(last_byte_error.offset(), last_byte_offset);
        assert_eq!(last_byte_error.width(), 2);
        assert_eq!(
            last_byte_error.to_string(),
            "RDRAM access out of range: address=4194303 width=2"
        );
        assert_eq!(exact_end_error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(exact_end_error.width(), 2);
        assert_eq!(
            exact_end_error.to_string(),
            "RDRAM access out of range: address=4194304 width=2"
        );
        assert_eq!(past_end_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_end_error.width(), 2);
        assert_eq!(
            past_end_error.to_string(),
            "RDRAM access out of range: address=4194305 width=2"
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x22));
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
    }

    #[test]
    fn raw_rdram_u16_be_write_invalidates_only_overlapping_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        machine.write_rdram_u16_be(0x0000_00fe, 0x1122).unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);

        machine.write_rdram_u16_be(0x0000_0104, 0x3344).unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);

        machine.write_rdram_u16_be(0x0000_00ff, 0x5566).unwrap();
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_00fe), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(0x0000_00ff), Ok(0x55));
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0x66));
        assert_eq!(machine.rdram().read_u8(0x0000_0104), Ok(0x33));
        assert_eq!(machine.rdram().read_u8(0x0000_0105), Ok(0x44));
    }

    #[test]
    fn raw_rdram_u16_be_write_uses_latest_staged_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        machine.cpu_rdram_reservation.stage(0x0000_0200, 4);
        machine.write_rdram_u16_be(0x0000_0100, 0x7788).unwrap();

        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0x77));
        assert_eq!(machine.rdram().read_u8(0x0000_0101), Ok(0x88));

        machine.write_rdram_u16_be(0x0000_0201, 0x99aa).unwrap();

        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_0201), Ok(0x99));
        assert_eq!(machine.rdram().read_u8(0x0000_0202), Ok(0xaa));
    }

    #[test]
    fn raw_rdram_u16_be_write_preserves_unrelated_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.cpu_rdram_reservation.stage(0x0010_0200, 8);
        machine.write_rdram_u16_be(0x0010_0204, 0xbeef).unwrap();

        assert_eq!(machine.rdram().read_u8(0x0010_0203), Ok(0));
        assert_eq!(machine.rdram().read_u8(0x0010_0204), Ok(0xbe));
        assert_eq!(machine.rdram().read_u8(0x0010_0205), Ok(0xef));
        assert_eq!(machine.rdram().read_u8(0x0010_0206), Ok(0));
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn raw_rdram_u32_be_write_updates_first_and_last_valid_storage_offsets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 4;

        machine.write_rdram_u32_be(0, 0x89ab_cdef).unwrap();
        machine
            .write_rdram_u32_be(last_valid_offset, 0x1234_5678)
            .unwrap();

        assert_eq!(machine.rdram().read_u8(0), Ok(0x89));
        assert_eq!(machine.rdram().read_u8(1), Ok(0xab));
        assert_eq!(machine.rdram().read_u8(2), Ok(0xcd));
        assert_eq!(machine.rdram().read_u8(3), Ok(0xef));
        assert_eq!(machine.rdram().read_u8(4), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset - 1), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x12));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x34));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 2), Ok(0x56));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 3), Ok(0x78));
    }

    #[test]
    fn raw_rdram_u32_be_write_accepts_unaligned_storage_offset_without_alignment_check() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.write_rdram_u32_be(3, 0xcafe_babe).unwrap();

        assert_eq!(machine.rdram().read_u8(2), Ok(0));
        assert_eq!(machine.rdram().read_u8(3), Ok(0xca));
        assert_eq!(machine.rdram().read_u8(4), Ok(0xfe));
        assert_eq!(machine.rdram().read_u8(5), Ok(0xba));
        assert_eq!(machine.rdram().read_u8(6), Ok(0xbe));
        assert_eq!(machine.rdram().read_u8(7), Ok(0));
    }

    #[test]
    fn raw_rdram_u32_be_write_rejects_invalid_offsets_before_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 4;
        let third_to_last_byte_offset = RDRAM_SIZE_BYTES - 3;
        let second_to_last_byte_offset = RDRAM_SIZE_BYTES - 2;
        let last_byte_offset = RDRAM_SIZE_BYTES - 1;

        machine
            .write_rdram_u32_be(last_valid_offset, 0x1122_3344)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        let third_to_last_error = machine
            .write_rdram_u32_be(third_to_last_byte_offset, 0xaabb_ccdd)
            .unwrap_err();
        let second_to_last_error = machine
            .write_rdram_u32_be(second_to_last_byte_offset, 0xeeff_0011)
            .unwrap_err();
        let last_byte_error = machine
            .write_rdram_u32_be(last_byte_offset, 0x2233_4455)
            .unwrap_err();
        let exact_end_error = machine
            .write_rdram_u32_be(RDRAM_SIZE_BYTES, 0x6677_8899)
            .unwrap_err();
        let past_end_error = machine
            .write_rdram_u32_be(RDRAM_SIZE_BYTES + 1, 0xaabb_ccdd)
            .unwrap_err();

        assert_eq!(third_to_last_error.offset(), third_to_last_byte_offset);
        assert_eq!(third_to_last_error.width(), 4);
        assert_eq!(
            third_to_last_error.to_string(),
            "RDRAM access out of range: address=4194301 width=4"
        );
        assert_eq!(second_to_last_error.offset(), second_to_last_byte_offset);
        assert_eq!(second_to_last_error.width(), 4);
        assert_eq!(last_byte_error.offset(), last_byte_offset);
        assert_eq!(last_byte_error.width(), 4);
        assert_eq!(exact_end_error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(exact_end_error.width(), 4);
        assert_eq!(
            exact_end_error.to_string(),
            "RDRAM access out of range: address=4194304 width=4"
        );
        assert_eq!(past_end_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_end_error.width(), 4);
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x22));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 2), Ok(0x33));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 3), Ok(0x44));
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
    }

    #[test]
    fn raw_rdram_u32_be_write_invalidates_only_overlapping_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        machine
            .write_rdram_u32_be(0x0000_00fc, 0x1122_3344)
            .unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);

        machine
            .write_rdram_u32_be(0x0000_0104, 0x5566_7788)
            .unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);

        machine
            .write_rdram_u32_be(0x0000_0100, 0xaabb_ccdd)
            .unwrap();
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0xaa));
        assert_eq!(machine.rdram().read_u8(0x0000_0101), Ok(0xbb));
        assert_eq!(machine.rdram().read_u8(0x0000_0102), Ok(0xcc));
        assert_eq!(machine.rdram().read_u8(0x0000_0103), Ok(0xdd));
        assert_eq!(machine.rdram().read_u8(0x0000_0104), Ok(0x55));
        assert_eq!(machine.rdram().read_u8(0x0000_0107), Ok(0x88));

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        machine
            .write_rdram_u32_be(0x0000_00fd, 0x99aa_bbcc)
            .unwrap();
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_00fc), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(0x0000_00fd), Ok(0x99));
        assert_eq!(machine.rdram().read_u8(0x0000_00fe), Ok(0xaa));
        assert_eq!(machine.rdram().read_u8(0x0000_00ff), Ok(0xbb));
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0xcc));
        assert_eq!(machine.rdram().read_u8(0x0000_0104), Ok(0x55));
        assert_eq!(machine.rdram().read_u8(0x0000_0107), Ok(0x88));
    }

    #[test]
    fn raw_rdram_u32_be_write_uses_latest_staged_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        machine.cpu_rdram_reservation.stage(0x0000_0200, 4);
        machine
            .write_rdram_u32_be(0x0000_0100, 0x1234_5678)
            .unwrap();

        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0x12));
        assert_eq!(machine.rdram().read_u8(0x0000_0103), Ok(0x78));

        machine
            .write_rdram_u32_be(0x0000_0201, 0x9abc_def0)
            .unwrap();

        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_0201), Ok(0x9a));
        assert_eq!(machine.rdram().read_u8(0x0000_0204), Ok(0xf0));
    }

    #[test]
    fn raw_rdram_u32_be_write_preserves_unrelated_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.cpu_rdram_reservation.stage(0x0010_0200, 8);
        machine
            .write_rdram_u32_be(0x0010_0204, 0xdead_beef)
            .unwrap();

        assert_eq!(machine.rdram().read_u8(0x0010_0203), Ok(0));
        assert_eq!(machine.rdram().read_u8(0x0010_0204), Ok(0xde));
        assert_eq!(machine.rdram().read_u8(0x0010_0205), Ok(0xad));
        assert_eq!(machine.rdram().read_u8(0x0010_0206), Ok(0xbe));
        assert_eq!(machine.rdram().read_u8(0x0010_0207), Ok(0xef));
        assert_eq!(machine.rdram().read_u8(0x0010_0208), Ok(0));
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn raw_rdram_u64_be_write_updates_first_and_last_valid_storage_offsets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 8;

        machine
            .write_rdram_u64_be(0, 0x0123_4567_89ab_cdef)
            .unwrap();
        machine
            .write_rdram_u64_be(last_valid_offset, 0xfedc_ba98_7654_3210)
            .unwrap();

        assert_eq!(machine.rdram().read_u8(0), Ok(0x01));
        assert_eq!(machine.rdram().read_u8(1), Ok(0x23));
        assert_eq!(machine.rdram().read_u8(2), Ok(0x45));
        assert_eq!(machine.rdram().read_u8(3), Ok(0x67));
        assert_eq!(machine.rdram().read_u8(4), Ok(0x89));
        assert_eq!(machine.rdram().read_u8(5), Ok(0xab));
        assert_eq!(machine.rdram().read_u8(6), Ok(0xcd));
        assert_eq!(machine.rdram().read_u8(7), Ok(0xef));
        assert_eq!(machine.rdram().read_u8(8), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset - 1), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0xfe));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0xdc));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 2), Ok(0xba));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 3), Ok(0x98));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 4), Ok(0x76));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 5), Ok(0x54));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 6), Ok(0x32));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 7), Ok(0x10));
    }

    #[test]
    fn raw_rdram_u64_be_write_accepts_unaligned_storage_offset_without_alignment_check() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine
            .write_rdram_u64_be(3, 0x1122_3344_5566_7788)
            .unwrap();

        assert_eq!(machine.rdram().read_u8(2), Ok(0));
        assert_eq!(machine.rdram().read_u8(3), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(4), Ok(0x22));
        assert_eq!(machine.rdram().read_u8(5), Ok(0x33));
        assert_eq!(machine.rdram().read_u8(6), Ok(0x44));
        assert_eq!(machine.rdram().read_u8(7), Ok(0x55));
        assert_eq!(machine.rdram().read_u8(8), Ok(0x66));
        assert_eq!(machine.rdram().read_u8(9), Ok(0x77));
        assert_eq!(machine.rdram().read_u8(10), Ok(0x88));
        assert_eq!(machine.rdram().read_u8(11), Ok(0));
    }

    #[test]
    fn raw_rdram_u64_be_write_rejects_invalid_offsets_before_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 8;
        let seventh_to_last_byte_offset = RDRAM_SIZE_BYTES - 7;
        let sixth_to_last_byte_offset = RDRAM_SIZE_BYTES - 6;
        let fifth_to_last_byte_offset = RDRAM_SIZE_BYTES - 5;
        let fourth_to_last_byte_offset = RDRAM_SIZE_BYTES - 4;
        let third_to_last_byte_offset = RDRAM_SIZE_BYTES - 3;
        let second_to_last_byte_offset = RDRAM_SIZE_BYTES - 2;
        let last_byte_offset = RDRAM_SIZE_BYTES - 1;

        machine
            .write_rdram_u64_be(last_valid_offset, 0x0102_0304_0506_0708)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        let seventh_to_last_error = machine
            .write_rdram_u64_be(seventh_to_last_byte_offset, 0x1111_2222_3333_4444)
            .unwrap_err();
        let sixth_to_last_error = machine
            .write_rdram_u64_be(sixth_to_last_byte_offset, 0x2222_3333_4444_5555)
            .unwrap_err();
        let fifth_to_last_error = machine
            .write_rdram_u64_be(fifth_to_last_byte_offset, 0x3333_4444_5555_6666)
            .unwrap_err();
        let fourth_to_last_error = machine
            .write_rdram_u64_be(fourth_to_last_byte_offset, 0x4444_5555_6666_7777)
            .unwrap_err();
        let third_to_last_error = machine
            .write_rdram_u64_be(third_to_last_byte_offset, 0x5555_6666_7777_8888)
            .unwrap_err();
        let second_to_last_error = machine
            .write_rdram_u64_be(second_to_last_byte_offset, 0x6666_7777_8888_9999)
            .unwrap_err();
        let last_byte_error = machine
            .write_rdram_u64_be(last_byte_offset, 0x7777_8888_9999_aaaa)
            .unwrap_err();
        let exact_end_error = machine
            .write_rdram_u64_be(RDRAM_SIZE_BYTES, 0x8888_9999_aaaa_bbbb)
            .unwrap_err();
        let past_end_error = machine
            .write_rdram_u64_be(RDRAM_SIZE_BYTES + 1, 0x9999_aaaa_bbbb_cccc)
            .unwrap_err();

        assert_eq!(seventh_to_last_error.offset(), seventh_to_last_byte_offset);
        assert_eq!(seventh_to_last_error.width(), 8);
        assert_eq!(
            seventh_to_last_error.to_string(),
            "RDRAM access out of range: address=4194297 width=8"
        );
        assert_eq!(sixth_to_last_error.offset(), sixth_to_last_byte_offset);
        assert_eq!(sixth_to_last_error.width(), 8);
        assert_eq!(fifth_to_last_error.offset(), fifth_to_last_byte_offset);
        assert_eq!(fifth_to_last_error.width(), 8);
        assert_eq!(fourth_to_last_error.offset(), fourth_to_last_byte_offset);
        assert_eq!(fourth_to_last_error.width(), 8);
        assert_eq!(third_to_last_error.offset(), third_to_last_byte_offset);
        assert_eq!(third_to_last_error.width(), 8);
        assert_eq!(second_to_last_error.offset(), second_to_last_byte_offset);
        assert_eq!(second_to_last_error.width(), 8);
        assert_eq!(last_byte_error.offset(), last_byte_offset);
        assert_eq!(last_byte_error.width(), 8);
        assert_eq!(exact_end_error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(exact_end_error.width(), 8);
        assert_eq!(
            exact_end_error.to_string(),
            "RDRAM access out of range: address=4194304 width=8"
        );
        assert_eq!(past_end_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_end_error.width(), 8);
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x01));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x02));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 2), Ok(0x03));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 3), Ok(0x04));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 4), Ok(0x05));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 5), Ok(0x06));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 6), Ok(0x07));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 7), Ok(0x08));
        assert_eq!(machine.rdram().read_u8(0), Ok(0));
    }

    #[test]
    fn raw_rdram_u64_be_write_invalidates_only_overlapping_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine
            .write_rdram_u64_be(0x0000_00f8, 0x1111_2222_3333_4444)
            .unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);

        machine
            .write_rdram_u64_be(0x0000_0108, 0x5555_6666_7777_8888)
            .unwrap();
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);

        machine
            .write_rdram_u64_be(0x0000_0100, 0xaabb_ccdd_eeff_0011)
            .unwrap();
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0xaa));
        assert_eq!(machine.rdram().read_u8(0x0000_0107), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(0x0000_0108), Ok(0x55));
        assert_eq!(machine.rdram().read_u8(0x0000_010f), Ok(0x88));

        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine
            .write_rdram_u64_be(0x0000_00f9, 0x99aa_bbcc_ddee_ff00)
            .unwrap();
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_00f8), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(0x0000_00f9), Ok(0x99));
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0x00));
        assert_eq!(machine.rdram().read_u8(0x0000_0108), Ok(0x55));
        assert_eq!(machine.rdram().read_u8(0x0000_010f), Ok(0x88));
    }

    #[test]
    fn raw_rdram_u64_be_write_uses_latest_staged_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu_rdram_reservation.stage(0x0000_0200, 8);
        machine
            .write_rdram_u64_be(0x0000_0100, 0x1234_5678_9abc_def0)
            .unwrap();

        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        assert_eq!(machine.rdram().read_u8(0x0000_0100), Ok(0x12));
        assert_eq!(machine.rdram().read_u8(0x0000_0107), Ok(0xf0));

        machine
            .write_rdram_u64_be(0x0000_0201, 0x0fed_cba9_8765_4321)
            .unwrap();

        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_eq!(machine.rdram().read_u8(0x0000_0201), Ok(0x0f));
        assert_eq!(machine.rdram().read_u8(0x0000_0208), Ok(0x21));
    }

    #[test]
    fn raw_rdram_u64_be_write_preserves_unrelated_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.cpu_rdram_reservation.stage(0x0010_0200, 8);
        machine
            .write_rdram_u64_be(0x0010_0204, 0x0123_4567_89ab_cdef)
            .unwrap();

        assert_eq!(machine.rdram().read_u8(0x0010_0203), Ok(0));
        assert_eq!(machine.rdram().read_u8(0x0010_0204), Ok(0x01));
        assert_eq!(machine.rdram().read_u8(0x0010_0205), Ok(0x23));
        assert_eq!(machine.rdram().read_u8(0x0010_0206), Ok(0x45));
        assert_eq!(machine.rdram().read_u8(0x0010_0207), Ok(0x67));
        assert_eq!(machine.rdram().read_u8(0x0010_0208), Ok(0x89));
        assert_eq!(machine.rdram().read_u8(0x0010_0209), Ok(0xab));
        assert_eq!(machine.rdram().read_u8(0x0010_020a), Ok(0xcd));
        assert_eq!(machine.rdram().read_u8(0x0010_020b), Ok(0xef));
        assert_eq!(machine.rdram().read_u8(0x0010_020c), Ok(0));
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn raw_rdram_u16_be_read_observes_big_endian_storage_offsets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 2;

        machine.write_rdram_u16_be(0, 0x1234).unwrap();
        machine.write_rdram_u16_be(3, 0x5678).unwrap();
        machine
            .write_rdram_u16_be(last_valid_offset, 0x9abc)
            .unwrap();

        assert_eq!(machine.rdram().read_u16_be(0), Ok(0x1234));
        assert_eq!(machine.rdram().read_u16_be(3), Ok(0x5678));
        assert_eq!(machine.rdram().read_u16_be(last_valid_offset), Ok(0x9abc));
        assert_eq!(machine.rdram().read_u8(2), Ok(0));
        assert_eq!(machine.rdram().read_u8(5), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset - 1), Ok(0));
    }

    #[test]
    fn raw_rdram_u16_be_read_rejects_invalid_offsets_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 2;
        let last_byte_offset = RDRAM_SIZE_BYTES - 1;

        machine
            .write_rdram_u16_be(last_valid_offset, 0x1122)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        let last_byte_error = machine.rdram().read_u16_be(last_byte_offset).unwrap_err();
        let exact_end_error = machine.rdram().read_u16_be(RDRAM_SIZE_BYTES).unwrap_err();
        let past_end_error = machine
            .rdram()
            .read_u16_be(RDRAM_SIZE_BYTES + 1)
            .unwrap_err();

        assert_eq!(last_byte_error.offset(), last_byte_offset);
        assert_eq!(last_byte_error.width(), 2);
        assert_eq!(
            last_byte_error.to_string(),
            "RDRAM access out of range: address=4194303 width=2"
        );
        assert_eq!(exact_end_error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(exact_end_error.width(), 2);
        assert_eq!(past_end_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_end_error.width(), 2);
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x22));
    }

    #[test]
    fn raw_rdram_u32_be_read_observes_big_endian_storage_offsets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 4;

        machine.write_rdram_u32_be(0, 0x1234_5678).unwrap();
        machine.write_rdram_u32_be(3, 0x9abc_def0).unwrap();
        machine
            .write_rdram_u32_be(last_valid_offset, 0x0bad_cafe)
            .unwrap();

        assert_eq!(machine.rdram().read_u32_be(0), Ok(0x1234_569a));
        assert_eq!(machine.rdram().read_u32_be(3), Ok(0x9abc_def0));
        assert_eq!(
            machine.rdram().read_u32_be(last_valid_offset),
            Ok(0x0bad_cafe)
        );
        assert_eq!(machine.rdram().read_u8(2), Ok(0x56));
        assert_eq!(machine.rdram().read_u8(7), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset - 1), Ok(0));
    }

    #[test]
    fn raw_rdram_u32_be_read_rejects_invalid_offsets_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 4;
        let third_to_last_byte_offset = RDRAM_SIZE_BYTES - 3;
        let second_to_last_byte_offset = RDRAM_SIZE_BYTES - 2;
        let last_byte_offset = RDRAM_SIZE_BYTES - 1;

        machine
            .write_rdram_u32_be(last_valid_offset, 0x1122_3344)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 4);
        let third_to_last_error = machine
            .rdram()
            .read_u32_be(third_to_last_byte_offset)
            .unwrap_err();
        let second_to_last_error = machine
            .rdram()
            .read_u32_be(second_to_last_byte_offset)
            .unwrap_err();
        let last_byte_error = machine.rdram().read_u32_be(last_byte_offset).unwrap_err();
        let exact_end_error = machine.rdram().read_u32_be(RDRAM_SIZE_BYTES).unwrap_err();
        let past_end_error = machine
            .rdram()
            .read_u32_be(RDRAM_SIZE_BYTES + 1)
            .unwrap_err();

        assert_eq!(third_to_last_error.offset(), third_to_last_byte_offset);
        assert_eq!(third_to_last_error.width(), 4);
        assert_eq!(
            third_to_last_error.to_string(),
            "RDRAM access out of range: address=4194301 width=4"
        );
        assert_eq!(second_to_last_error.offset(), second_to_last_byte_offset);
        assert_eq!(second_to_last_error.width(), 4);
        assert_eq!(last_byte_error.offset(), last_byte_offset);
        assert_eq!(last_byte_error.width(), 4);
        assert_eq!(exact_end_error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(exact_end_error.width(), 4);
        assert_eq!(past_end_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_end_error.width(), 4);
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x22));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 2), Ok(0x33));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 3), Ok(0x44));
    }

    #[test]
    fn raw_rdram_u64_be_read_observes_big_endian_storage_offsets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 8;

        machine
            .write_rdram_u64_be(0, 0x0123_4567_89ab_cdef)
            .unwrap();
        machine
            .write_rdram_u64_be(3, 0x1020_3040_5060_7080)
            .unwrap();
        machine
            .write_rdram_u64_be(last_valid_offset, 0xfedc_ba98_7654_3210)
            .unwrap();

        assert_eq!(machine.rdram().read_u64_be(0), Ok(0x0123_4510_2030_4050));
        assert_eq!(machine.rdram().read_u64_be(3), Ok(0x1020_3040_5060_7080));
        assert_eq!(
            machine.rdram().read_u64_be(last_valid_offset),
            Ok(0xfedc_ba98_7654_3210)
        );
        assert_eq!(machine.rdram().read_u8(2), Ok(0x45));
        assert_eq!(machine.rdram().read_u8(11), Ok(0));
        assert_eq!(machine.rdram().read_u8(last_valid_offset - 1), Ok(0));
    }

    #[test]
    fn raw_rdram_u64_be_read_rejects_invalid_offsets_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 8;
        let seventh_to_last_byte_offset = RDRAM_SIZE_BYTES - 7;
        let sixth_to_last_byte_offset = RDRAM_SIZE_BYTES - 6;
        let fifth_to_last_byte_offset = RDRAM_SIZE_BYTES - 5;
        let fourth_to_last_byte_offset = RDRAM_SIZE_BYTES - 4;
        let third_to_last_byte_offset = RDRAM_SIZE_BYTES - 3;
        let second_to_last_byte_offset = RDRAM_SIZE_BYTES - 2;
        let last_byte_offset = RDRAM_SIZE_BYTES - 1;

        machine
            .write_rdram_u64_be(last_valid_offset, 0x0102_0304_0506_0708)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        let seventh_to_last_error = machine
            .rdram()
            .read_u64_be(seventh_to_last_byte_offset)
            .unwrap_err();
        let sixth_to_last_error = machine
            .rdram()
            .read_u64_be(sixth_to_last_byte_offset)
            .unwrap_err();
        let fifth_to_last_error = machine
            .rdram()
            .read_u64_be(fifth_to_last_byte_offset)
            .unwrap_err();
        let fourth_to_last_error = machine
            .rdram()
            .read_u64_be(fourth_to_last_byte_offset)
            .unwrap_err();
        let third_to_last_error = machine
            .rdram()
            .read_u64_be(third_to_last_byte_offset)
            .unwrap_err();
        let second_to_last_error = machine
            .rdram()
            .read_u64_be(second_to_last_byte_offset)
            .unwrap_err();
        let last_byte_error = machine.rdram().read_u64_be(last_byte_offset).unwrap_err();
        let exact_end_error = machine.rdram().read_u64_be(RDRAM_SIZE_BYTES).unwrap_err();
        let past_end_error = machine
            .rdram()
            .read_u64_be(RDRAM_SIZE_BYTES + 1)
            .unwrap_err();

        assert_eq!(seventh_to_last_error.offset(), seventh_to_last_byte_offset);
        assert_eq!(seventh_to_last_error.width(), 8);
        assert_eq!(
            seventh_to_last_error.to_string(),
            "RDRAM access out of range: address=4194297 width=8"
        );
        assert_eq!(sixth_to_last_error.offset(), sixth_to_last_byte_offset);
        assert_eq!(sixth_to_last_error.width(), 8);
        assert_eq!(fifth_to_last_error.offset(), fifth_to_last_byte_offset);
        assert_eq!(fifth_to_last_error.width(), 8);
        assert_eq!(fourth_to_last_error.offset(), fourth_to_last_byte_offset);
        assert_eq!(fourth_to_last_error.width(), 8);
        assert_eq!(third_to_last_error.offset(), third_to_last_byte_offset);
        assert_eq!(third_to_last_error.width(), 8);
        assert_eq!(second_to_last_error.offset(), second_to_last_byte_offset);
        assert_eq!(second_to_last_error.width(), 8);
        assert_eq!(last_byte_error.offset(), last_byte_offset);
        assert_eq!(last_byte_error.width(), 8);
        assert_eq!(exact_end_error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(exact_end_error.width(), 8);
        assert_eq!(past_end_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_end_error.width(), 8);
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        assert_eq!(machine.rdram().read_u8(last_valid_offset), Ok(0x01));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 1), Ok(0x02));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 2), Ok(0x03));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 3), Ok(0x04));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 4), Ok(0x05));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 5), Ok(0x06));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 6), Ok(0x07));
        assert_eq!(machine.rdram().read_u8(last_valid_offset + 7), Ok(0x08));
    }

    #[test]
    fn raw_rdram_read_widths_preserve_unrelated_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.write_rdram_u16_be(0x0010_0200, 0x1234).unwrap();
        machine
            .write_rdram_u32_be(0x0010_0204, 0x5678_9abc)
            .unwrap();
        machine
            .write_rdram_u64_be(0x0010_0208, 0xdef0_1122_3344_5566)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0010_0200, 16);

        assert_eq!(machine.rdram().read_u16_be(0x0010_0200), Ok(0x1234));
        assert_eq!(machine.rdram().read_u32_be(0x0010_0204), Ok(0x5678_9abc));
        assert_eq!(
            machine.rdram().read_u64_be(0x0010_0208),
            Ok(0xdef0_1122_3344_5566)
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0010_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 16);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.rdram().size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn direct_rdram_read_values_support_kseg0_and_kseg1_for_all_widths() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine
            .write_rdram_u64_be(0x0000_0020, 0x0123_4567_89ab_cdef)
            .unwrap();

        assert_eq!(machine.read_direct_rdram_u8(kseg0(0x0000_0020)), Ok(0x01));
        assert_eq!(machine.read_direct_rdram_u8(kseg1(0x0000_0027)), Ok(0xef));
        assert_eq!(
            machine.read_direct_rdram_u16_be(kseg0(0x0000_0020)),
            Ok(0x0123)
        );
        assert_eq!(
            machine.read_direct_rdram_u16_be(kseg1(0x0000_0026)),
            Ok(0xcdef)
        );
        assert_eq!(
            machine.read_direct_rdram_u32_be(kseg0(0x0000_0020)),
            Ok(0x0123_4567)
        );
        assert_eq!(
            machine.read_direct_rdram_u32_be(kseg1(0x0000_0024)),
            Ok(0x89ab_cdef)
        );
        assert_eq!(
            machine.read_direct_rdram_u64_be(kseg0(0x0000_0020)),
            Ok(0x0123_4567_89ab_cdef)
        );
        assert_eq!(
            machine.read_direct_rdram_u64_be(kseg1(0x0000_0020)),
            Ok(0x0123_4567_89ab_cdef)
        );
    }

    #[test]
    fn direct_rdram_read_values_accept_last_valid_address_per_width() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        let last_u8 = RDRAM_SIZE_BYTES - 1;
        machine.write_rdram_u8(last_u8, 0x11).unwrap();
        assert_eq!(machine.read_direct_rdram_u8(kseg0(last_u8)), Ok(0x11));
        assert_eq!(machine.read_direct_rdram_u8(kseg1(last_u8)), Ok(0x11));

        let last_u16 = RDRAM_SIZE_BYTES - 2;
        machine.write_rdram_u16_be(last_u16, 0x2233).unwrap();
        assert_eq!(
            machine.read_direct_rdram_u16_be(kseg0(last_u16)),
            Ok(0x2233)
        );
        assert_eq!(
            machine.read_direct_rdram_u16_be(kseg1(last_u16)),
            Ok(0x2233)
        );

        let last_u32 = RDRAM_SIZE_BYTES - 4;
        machine.write_rdram_u32_be(last_u32, 0x4455_6677).unwrap();
        assert_eq!(
            machine.read_direct_rdram_u32_be(kseg0(last_u32)),
            Ok(0x4455_6677)
        );
        assert_eq!(
            machine.read_direct_rdram_u32_be(kseg1(last_u32)),
            Ok(0x4455_6677)
        );

        let last_u64 = RDRAM_SIZE_BYTES - 8;
        machine
            .write_rdram_u64_be(last_u64, 0x8899_aabb_ccdd_eeff)
            .unwrap();
        assert_eq!(
            machine.read_direct_rdram_u64_be(kseg0(last_u64)),
            Ok(0x8899_aabb_ccdd_eeff)
        );
        assert_eq!(
            machine.read_direct_rdram_u64_be(kseg1(last_u64)),
            Ok(0x8899_aabb_ccdd_eeff)
        );
    }

    #[test]
    fn direct_rdram_read_values_reject_invalid_direct_spans_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine
            .write_rdram_u64_be(RDRAM_SIZE_BYTES - 8, 0x0102_0304_0506_0708)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);

        let exact_end = machine.read_direct_rdram_u8(kseg0(RDRAM_SIZE_BYTES));
        let last_byte_u16 = machine.read_direct_rdram_u16_be(kseg1(RDRAM_SIZE_BYTES - 1));
        let third_to_last_u32 = machine.read_direct_rdram_u32_be(kseg0(RDRAM_SIZE_BYTES - 3));
        let seventh_to_last_u64 = machine.read_direct_rdram_u64_be(kseg1(RDRAM_SIZE_BYTES - 7));
        let past_end = machine.read_direct_rdram_u8(kseg0(RDRAM_SIZE_BYTES + 1));

        assert_eq!(
            exact_end.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg0(RDRAM_SIZE_BYTES),
                width: 1
            }
        );
        assert_eq!(
            last_byte_u16.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg1(RDRAM_SIZE_BYTES - 1),
                width: 2
            }
        );
        assert_eq!(
            third_to_last_u32.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg0(RDRAM_SIZE_BYTES - 3),
                width: 4
            }
        );
        assert_eq!(
            seventh_to_last_u64.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg1(RDRAM_SIZE_BYTES - 7),
                width: 8
            }
        );
        let past_end_error = past_end.unwrap_err();
        assert_eq!(past_end_error.cpu_address(), kseg0(RDRAM_SIZE_BYTES + 1));
        assert_eq!(past_end_error.width(), 1);
        assert_eq!(
            past_end_error.to_string(),
            "direct RDRAM access unsupported: address=2151677953 width=1"
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        assert_eq!(
            machine.rdram().read_u64_be(RDRAM_SIZE_BYTES - 8),
            Ok(0x0102_0304_0506_0708)
        );
    }

    #[test]
    fn direct_rdram_read_values_reject_non_direct_addresses() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.write_rdram_u32_be(0, 0x1122_3344).unwrap();

        for (address, width) in [
            (CpuAddress::new(0x0000_0000), 1),
            (CpuAddress::new(0x6000_0000), 2),
            (CpuAddress::new(0xbfc0_0000), 4),
            (CpuAddress::new(0xc000_0000), 8),
        ] {
            let error = match width {
                1 => machine.read_direct_rdram_u8(address).unwrap_err(),
                2 => machine.read_direct_rdram_u16_be(address).unwrap_err(),
                4 => machine.read_direct_rdram_u32_be(address).unwrap_err(),
                8 => machine.read_direct_rdram_u64_be(address).unwrap_err(),
                _ => unreachable!(),
            };
            assert_eq!(error.cpu_address(), address);
            assert_eq!(error.width(), width);
        }
        assert_eq!(machine.rdram().read_u32_be(0), Ok(0x1122_3344));
    }

    #[test]
    fn direct_rdram_read_values_accept_unaligned_addresses_without_alignment_check() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine
            .write_rdram_u64_be(3, 0x1122_3344_5566_7788)
            .unwrap();

        assert_eq!(machine.read_direct_rdram_u8(kseg0(3)), Ok(0x11));
        assert_eq!(machine.read_direct_rdram_u16_be(kseg1(3)), Ok(0x1122));
        assert_eq!(machine.read_direct_rdram_u32_be(kseg0(5)), Ok(0x3344_5566));
        assert_eq!(
            machine.read_direct_rdram_u64_be(kseg1(3)),
            Ok(0x1122_3344_5566_7788)
        );
    }

    #[test]
    fn direct_rdram_read_values_preserve_unrelated_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine
            .write_rdram_u64_be(0x0010_0208, 0xdef0_1122_3344_5566)
            .unwrap();
        machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef).unwrap();
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        machine.cpu_rdram_reservation.stage(0x0010_0200, 16);

        assert_eq!(
            machine.read_direct_rdram_u64_be(kseg0(0x0010_0208)),
            Ok(0xdef0_1122_3344_5566)
        );
        assert_eq!(
            machine.read_direct_rdram_u32_be(kseg1(0x0010_0208)),
            Ok(0xdef0_1122)
        );
        assert_eq!(
            machine.read_direct_rdram_u16_be(kseg0(0x0010_020c)),
            Ok(0x3344)
        );
        assert_eq!(machine.read_direct_rdram_u8(kseg1(0x0010_020f)), Ok(0x66));

        assert_eq!(
            machine.rdram().read_u64_be(0x0010_0208),
            Ok(0xdef0_1122_3344_5566)
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0010_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 16);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.cpu().pc(), 0x8000_1000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2000);
        assert_eq!(machine.cpu().hi(), 0x1111_2222_3333_4444);
        assert_eq!(machine.cpu().lo(), 0x5555_6666_7777_8888);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn direct_rdram_write_values_support_kseg0_and_kseg1_for_all_widths() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        assert_eq!(
            machine.write_direct_rdram_u8(kseg0(0x0000_0040), 0x11),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u8(kseg1(0x0000_0041), 0x22),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u16_be(kseg0(0x0000_0044), 0x3344),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u16_be(kseg1(0x0000_0046), 0x5566),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u32_be(kseg0(0x0000_0050), 0x7788_99aa),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u32_be(kseg1(0x0000_0054), 0xbbcc_ddee),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u64_be(kseg0(0x0000_0060), 0x0102_0304_0506_0708),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u64_be(kseg1(0x0000_0068), 0x1112_1314_1516_1718),
            Ok(())
        );

        assert_eq!(machine.rdram().read_u8(0x0000_0040), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(0x0000_0041), Ok(0x22));
        assert_eq!(machine.rdram().read_u16_be(0x0000_0044), Ok(0x3344));
        assert_eq!(machine.rdram().read_u16_be(0x0000_0046), Ok(0x5566));
        assert_eq!(machine.rdram().read_u32_be(0x0000_0050), Ok(0x7788_99aa));
        assert_eq!(machine.rdram().read_u32_be(0x0000_0054), Ok(0xbbcc_ddee));
        assert_eq!(
            machine.rdram().read_u64_be(0x0000_0060),
            Ok(0x0102_0304_0506_0708)
        );
        assert_eq!(
            machine.rdram().read_u64_be(0x0000_0068),
            Ok(0x1112_1314_1516_1718)
        );
    }

    #[test]
    fn direct_rdram_write_values_accept_last_valid_address_per_width() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        let last_u8 = RDRAM_SIZE_BYTES - 1;
        assert_eq!(machine.write_direct_rdram_u8(kseg0(last_u8), 0x11), Ok(()));
        assert_eq!(machine.write_direct_rdram_u8(kseg1(last_u8), 0x22), Ok(()));
        assert_eq!(machine.rdram().read_u8(last_u8), Ok(0x22));

        let last_u16 = RDRAM_SIZE_BYTES - 2;
        assert_eq!(
            machine.write_direct_rdram_u16_be(kseg0(last_u16), 0x3344),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u16_be(kseg1(last_u16), 0x5566),
            Ok(())
        );
        assert_eq!(machine.rdram().read_u16_be(last_u16), Ok(0x5566));

        let last_u32 = RDRAM_SIZE_BYTES - 4;
        assert_eq!(
            machine.write_direct_rdram_u32_be(kseg0(last_u32), 0x7788_99aa),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u32_be(kseg1(last_u32), 0xbbcc_ddee),
            Ok(())
        );
        assert_eq!(machine.rdram().read_u32_be(last_u32), Ok(0xbbcc_ddee));

        let last_u64 = RDRAM_SIZE_BYTES - 8;
        assert_eq!(
            machine.write_direct_rdram_u64_be(kseg0(last_u64), 0x0102_0304_0506_0708),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_u64_be(kseg1(last_u64), 0x1112_1314_1516_1718),
            Ok(())
        );
        assert_eq!(
            machine.rdram().read_u64_be(last_u64),
            Ok(0x1112_1314_1516_1718)
        );
    }

    #[test]
    fn direct_rdram_write_values_reject_invalid_direct_spans_before_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine
            .write_rdram_u64_be(RDRAM_SIZE_BYTES - 8, 0x0102_0304_0506_0708)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);

        let exact_end = machine.write_direct_rdram_u8(kseg0(RDRAM_SIZE_BYTES), 0xaa);
        let last_byte_u16 = machine.write_direct_rdram_u16_be(kseg1(RDRAM_SIZE_BYTES - 1), 0xbbcc);
        let third_to_last_u32 =
            machine.write_direct_rdram_u32_be(kseg0(RDRAM_SIZE_BYTES - 3), 0xddee_ff00);
        let seventh_to_last_u64 =
            machine.write_direct_rdram_u64_be(kseg1(RDRAM_SIZE_BYTES - 7), 0x1122_3344_5566_7788);
        let past_end = machine.write_direct_rdram_u8(kseg0(RDRAM_SIZE_BYTES + 1), 0x99);

        assert_eq!(
            exact_end.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg0(RDRAM_SIZE_BYTES),
                width: 1
            }
        );
        assert_eq!(
            last_byte_u16.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg1(RDRAM_SIZE_BYTES - 1),
                width: 2
            }
        );
        assert_eq!(
            third_to_last_u32.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg0(RDRAM_SIZE_BYTES - 3),
                width: 4
            }
        );
        assert_eq!(
            seventh_to_last_u64.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg1(RDRAM_SIZE_BYTES - 7),
                width: 8
            }
        );
        assert_eq!(
            past_end.unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg0(RDRAM_SIZE_BYTES + 1),
                width: 1
            }
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        assert_eq!(
            machine.rdram().read_u64_be(RDRAM_SIZE_BYTES - 8),
            Ok(0x0102_0304_0506_0708)
        );
    }

    #[test]
    fn direct_rdram_write_values_reject_non_direct_addresses() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.write_rdram_u32_be(0, 0x1122_3344).unwrap();

        for (address, width) in [
            (CpuAddress::new(0x0000_0000), 1),
            (CpuAddress::new(0x6000_0000), 2),
            (CpuAddress::new(0xbfc0_0000), 4),
            (CpuAddress::new(0xc000_0000), 8),
        ] {
            let error = match width {
                1 => machine.write_direct_rdram_u8(address, 0x55).unwrap_err(),
                2 => machine
                    .write_direct_rdram_u16_be(address, 0x6677)
                    .unwrap_err(),
                4 => machine
                    .write_direct_rdram_u32_be(address, 0x8899_aabb)
                    .unwrap_err(),
                8 => machine
                    .write_direct_rdram_u64_be(address, 0xccdd_eeff_0011_2233)
                    .unwrap_err(),
                _ => unreachable!(),
            };
            assert_eq!(error.cpu_address(), address);
            assert_eq!(error.width(), width);
        }
        assert_eq!(machine.rdram().read_u32_be(0), Ok(0x1122_3344));
    }

    #[test]
    fn direct_rdram_write_values_invalidate_only_overlapping_reservation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        assert_eq!(
            machine.write_direct_rdram_u32_be(kseg0(0x0000_0108), 0x1122_3344),
            Ok(())
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);

        assert_eq!(
            machine.write_direct_rdram_u8(kseg1(0x0000_00ff), 0x55),
            Ok(())
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);

        assert_eq!(
            machine.write_direct_rdram_u16_be(kseg0(0x0000_0107), 0x6677),
            Ok(())
        );
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
    }

    #[test]
    fn direct_rdram_write_values_preserve_unrelated_machine_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef).unwrap();
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        machine.cpu_rdram_reservation.stage(0x0010_0200, 16);

        assert_eq!(
            machine.write_direct_rdram_u64_be(kseg1(0x0010_0300), 0xdef0_1122_3344_5566),
            Ok(())
        );

        assert_eq!(
            machine.rdram().read_u64_be(0x0010_0300),
            Ok(0xdef0_1122_3344_5566)
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0010_0200);
        assert_eq!(machine.cpu_rdram_reservation.width(), 16);
        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.cpu().pc(), 0x8000_1000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2000);
        assert_eq!(machine.cpu().hi(), 0x1111_2222_3333_4444);
        assert_eq!(machine.cpu().lo(), 0x5555_6666_7777_8888);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn direct_rdram_cpu_data_reads_support_kseg0_kseg1_and_big_endian_values() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine
            .write_rdram_u64_be(0x0000_0020, 0x0123_4567_89ab_cdef)
            .unwrap();

        assert_eq!(
            machine.read_direct_rdram_cpu_data_u8(kseg0(0x0000_0020)),
            Ok(0x01)
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u8(kseg1(0x0000_0027)),
            Ok(0xef)
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u16_be(kseg0(0x0000_0020)),
            Ok(0x0123)
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u16_be(kseg1(0x0000_0026)),
            Ok(0xcdef)
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u32_be(kseg0(0x0000_0020)),
            Ok(0x0123_4567)
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u32_be(kseg1(0x0000_0024)),
            Ok(0x89ab_cdef)
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u64_be(kseg0(0x0000_0020)),
            Ok(0x0123_4567_89ab_cdef)
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u64_be(kseg1(0x0000_0020)),
            Ok(0x0123_4567_89ab_cdef)
        );
        assert_default_cpu_exception_state(&machine);
    }

    #[test]
    fn direct_rdram_cpu_data_writes_support_kseg0_kseg1_and_big_endian_values() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u8(kseg0(0x0000_0040), 0x11),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u8(kseg1(0x0000_0041), 0x22),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u16_be(kseg0(0x0000_0044), 0x3344),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u16_be(kseg1(0x0000_0046), 0x5566),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u32_be(kseg0(0x0000_0050), 0x7788_99aa),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u32_be(kseg1(0x0000_0054), 0xbbcc_ddee),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u64_be(kseg0(0x0000_0060), 0x0102_0304_0506_0708),
            Ok(())
        );
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u64_be(kseg1(0x0000_0068), 0x1112_1314_1516_1718),
            Ok(())
        );

        assert_eq!(machine.rdram().read_u8(0x0000_0040), Ok(0x11));
        assert_eq!(machine.rdram().read_u8(0x0000_0041), Ok(0x22));
        assert_eq!(machine.rdram().read_u16_be(0x0000_0044), Ok(0x3344));
        assert_eq!(machine.rdram().read_u16_be(0x0000_0046), Ok(0x5566));
        assert_eq!(machine.rdram().read_u32_be(0x0000_0050), Ok(0x7788_99aa));
        assert_eq!(machine.rdram().read_u32_be(0x0000_0054), Ok(0xbbcc_ddee));
        assert_eq!(
            machine.rdram().read_u64_be(0x0000_0060),
            Ok(0x0102_0304_0506_0708)
        );
        assert_eq!(
            machine.rdram().read_u64_be(0x0000_0068),
            Ok(0x1112_1314_1516_1718)
        );
        assert_default_cpu_exception_state(&machine);
    }

    #[test]
    fn direct_rdram_cpu_data_writes_preserve_raw_reservation_invalidation_order() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        assert_eq!(
            machine.write_direct_rdram_cpu_data_u32_be(kseg0(0x0000_0108), 0x1122_3344),
            Ok(())
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u8(kseg1(0x0000_00ff), 0x55),
            Ok(())
        );
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0100);
        assert_eq!(machine.cpu_rdram_reservation.width(), 8);

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u16_be(kseg0(0x0000_0106), 0x6677),
            Ok(())
        );
        assert!(!machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 0);
        assert_default_cpu_exception_state(&machine);
    }

    #[test]
    fn direct_rdram_cpu_data_boundary_acceptance_uses_direct_width_rules() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u8(kseg0(RDRAM_SIZE_BYTES - 1), 0x11),
            Ok(())
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u8(kseg1(RDRAM_SIZE_BYTES - 1)),
            Ok(0x11)
        );

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u16_be(kseg0(RDRAM_SIZE_BYTES - 2), 0x2233),
            Ok(())
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u16_be(kseg1(RDRAM_SIZE_BYTES - 2)),
            Ok(0x2233)
        );

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u32_be(kseg0(RDRAM_SIZE_BYTES - 4), 0x4455_6677),
            Ok(())
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u32_be(kseg1(RDRAM_SIZE_BYTES - 4)),
            Ok(0x4455_6677)
        );

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u64_be(
                kseg0(RDRAM_SIZE_BYTES - 8),
                0x8899_aabb_ccdd_eeff
            ),
            Ok(())
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u64_be(kseg1(RDRAM_SIZE_BYTES - 8)),
            Ok(0x8899_aabb_ccdd_eeff)
        );

        assert_default_cpu_exception_state(&machine);

        let mut read_rejection = Machine::from_cartridge(Cartridge::default());
        let exact_end_error = read_rejection
            .read_direct_rdram_cpu_data_u8(kseg0(RDRAM_SIZE_BYTES))
            .unwrap_err();
        assert_entered_data_address_error(
            exact_end_error,
            kseg0(RDRAM_SIZE_BYTES),
            CpuDataAccessKind::Read,
            CpuDataWidth::Byte,
            CpuAddressErrorKind::AddressErrorLoad,
        );
        assert_eq!(
            read_rejection.cpu().cop0_bad_vaddr(),
            kseg0(RDRAM_SIZE_BYTES).value()
        );
        assert_eq!(read_rejection.cpu().cop0_exception_code(), 4);

        let mut write_rejection = Machine::from_cartridge(Cartridge::default());
        write_rejection
            .write_rdram_u64_be(RDRAM_SIZE_BYTES - 8, 0x0102_0304_0506_0708)
            .unwrap();
        write_rejection
            .cpu_rdram_reservation
            .stage((RDRAM_SIZE_BYTES - 8) as u32, 8);
        let exact_end_write_error = write_rejection
            .write_direct_rdram_cpu_data_u64_be(kseg1(RDRAM_SIZE_BYTES), 0x8899_aabb_ccdd_eeff)
            .unwrap_err();
        assert_entered_data_address_error(
            exact_end_write_error,
            kseg1(RDRAM_SIZE_BYTES),
            CpuDataAccessKind::Write,
            CpuDataWidth::Doubleword,
            CpuAddressErrorKind::AddressErrorStore,
        );
        assert_eq!(
            write_rejection.rdram().read_u64_be(RDRAM_SIZE_BYTES - 8),
            Ok(0x0102_0304_0506_0708)
        );
        assert!(write_rejection.cpu_rdram_reservation.is_valid());
        assert_eq!(
            write_rejection.cpu_rdram_reservation.rdram_offset(),
            (RDRAM_SIZE_BYTES - 8) as u32
        );
        assert_eq!(write_rejection.cpu_rdram_reservation.width(), 8);
    }

    #[test]
    fn direct_rdram_cpu_data_unaligned_reads_enter_adel_without_storage_or_gpr_mutation() {
        for (address, width) in [
            (kseg0(0x0000_0021), CpuDataWidth::Halfword),
            (kseg1(0x0000_0022), CpuDataWidth::Word),
            (kseg0(0x0000_0023), CpuDataWidth::Doubleword),
        ] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            machine
                .write_rdram_u64_be(0x0000_0020, 0x0123_4567_89ab_cdef)
                .unwrap();
            machine.cpu.set_gpr(8, 0xfeed_face_cafe_beef).unwrap();
            machine.cpu_rdram_reservation.stage(0x0000_0020, 8);

            let error = match width {
                CpuDataWidth::Halfword => machine
                    .read_direct_rdram_cpu_data_u16_be(address)
                    .map(|_| ())
                    .unwrap_err(),
                CpuDataWidth::Word => machine
                    .read_direct_rdram_cpu_data_u32_be(address)
                    .map(|_| ())
                    .unwrap_err(),
                CpuDataWidth::Doubleword => machine
                    .read_direct_rdram_cpu_data_u64_be(address)
                    .map(|_| ())
                    .unwrap_err(),
                CpuDataWidth::Byte => unreachable!(),
            };

            assert_entered_data_address_error(
                error,
                address,
                CpuDataAccessKind::Read,
                width,
                CpuAddressErrorKind::AddressErrorLoad,
            );
            assert_eq!(machine.cpu().cop0_bad_vaddr(), address.value());
            assert_eq!(machine.cpu().cop0_exception_code(), 4);
            assert_eq!(machine.cpu().cop0_epc(), NON_BOOT_RESET_VECTOR_PC);
            assert_eq!(
                machine.cpu().cop0_status() & COP0_STATUS_EXL,
                COP0_STATUS_EXL
            );
            assert!(!machine.cpu().cop0_exception_branch_delay());
            assert_eq!(machine.cpu().pc(), LOCAL_EXCEPTION_VECTOR_PC);
            assert_eq!(machine.cpu().next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
            assert_eq!(machine.cpu().gpr(8), Some(0xfeed_face_cafe_beef));
            assert_eq!(
                machine.rdram().read_u64_be(0x0000_0020),
                Ok(0x0123_4567_89ab_cdef)
            );
            assert!(machine.cpu_rdram_reservation.is_valid());
            assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0020);
            assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        }
    }

    #[test]
    fn direct_rdram_cpu_data_unaligned_writes_enter_ades_without_storage_or_reservation_mutation() {
        for (address, width) in [
            (kseg0(0x0000_0041), CpuDataWidth::Halfword),
            (kseg1(0x0000_0042), CpuDataWidth::Word),
            (kseg0(0x0000_0043), CpuDataWidth::Doubleword),
        ] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            machine
                .write_rdram_u64_be(0x0000_0040, 0x1122_3344_5566_7788)
                .unwrap();
            machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef).unwrap();
            machine.cpu_rdram_reservation.stage(0x0000_0040, 8);

            let error = match width {
                CpuDataWidth::Halfword => machine
                    .write_direct_rdram_cpu_data_u16_be(address, 0xaabb)
                    .unwrap_err(),
                CpuDataWidth::Word => machine
                    .write_direct_rdram_cpu_data_u32_be(address, 0xccdd_eeff)
                    .unwrap_err(),
                CpuDataWidth::Doubleword => machine
                    .write_direct_rdram_cpu_data_u64_be(address, 0x0102_0304_0506_0708)
                    .unwrap_err(),
                CpuDataWidth::Byte => unreachable!(),
            };

            assert_entered_data_address_error(
                error,
                address,
                CpuDataAccessKind::Write,
                width,
                CpuAddressErrorKind::AddressErrorStore,
            );
            assert_eq!(machine.cpu().cop0_bad_vaddr(), address.value());
            assert_eq!(machine.cpu().cop0_exception_code(), 5);
            assert_eq!(machine.cpu().cop0_epc(), NON_BOOT_RESET_VECTOR_PC);
            assert_eq!(
                machine.cpu().cop0_status() & COP0_STATUS_EXL,
                COP0_STATUS_EXL
            );
            assert!(!machine.cpu().cop0_exception_branch_delay());
            assert_eq!(machine.cpu().pc(), LOCAL_EXCEPTION_VECTOR_PC);
            assert_eq!(machine.cpu().next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
            assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
            assert_eq!(
                machine.rdram().read_u64_be(0x0000_0040),
                Ok(0x1122_3344_5566_7788)
            );
            assert!(machine.cpu_rdram_reservation.is_valid());
            assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0x0000_0040);
            assert_eq!(machine.cpu_rdram_reservation.width(), 8);
        }
    }

    #[test]
    fn direct_rdram_cpu_data_byte_access_never_enters_alignment_exception() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        assert_eq!(
            machine.write_direct_rdram_cpu_data_u8(kseg0(0x0000_0003), 0x5a),
            Ok(())
        );
        assert_eq!(
            machine.read_direct_rdram_cpu_data_u8(kseg1(0x0000_0003)),
            Ok(0x5a)
        );
        assert_eq!(machine.rdram().read_u8(0x0000_0003), Ok(0x5a));
        assert_default_cpu_exception_state(&machine);
    }

    #[test]
    fn direct_rdram_cpu_data_aligned_target_rejection_enters_adel_or_ades() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.write_rdram_u32_be(0, 0x1122_3344).unwrap();
        machine.cpu_rdram_reservation.stage(0, 4);

        let read_error = machine
            .read_direct_rdram_cpu_data_u32_be(CpuAddress::new(0x0000_0000))
            .unwrap_err();
        assert_entered_data_address_error(
            read_error,
            CpuAddress::new(0x0000_0000),
            CpuDataAccessKind::Read,
            CpuDataWidth::Word,
            CpuAddressErrorKind::AddressErrorLoad,
        );
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x0000_0000);
        assert_eq!(machine.cpu().cop0_exception_code(), 4);
        assert_eq!(machine.rdram().read_u32_be(0), Ok(0x1122_3344));
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);

        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.write_rdram_u32_be(0, 0x1122_3344).unwrap();
        machine.cpu_rdram_reservation.stage(0, 4);
        let write_error = machine
            .write_direct_rdram_cpu_data_u32_be(kseg1(RDRAM_SIZE_BYTES), 0xaabb_ccdd)
            .unwrap_err();
        assert_entered_data_address_error(
            write_error,
            kseg1(RDRAM_SIZE_BYTES),
            CpuDataAccessKind::Write,
            CpuDataWidth::Word,
            CpuAddressErrorKind::AddressErrorStore,
        );
        assert_eq!(
            machine.cpu().cop0_bad_vaddr(),
            kseg1(RDRAM_SIZE_BYTES).value()
        );
        assert_eq!(machine.cpu().cop0_exception_code(), 5);
        assert_eq!(machine.rdram().read_u32_be(0), Ok(0x1122_3344));
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
    }

    #[test]
    fn lower_level_direct_rdram_value_apis_keep_direct_rejection_errors() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.write_rdram_u32_be(0, 0x1122_3344).unwrap();
        machine.cpu_rdram_reservation.stage(0, 4);

        assert_eq!(
            machine
                .read_direct_rdram_u32_be(CpuAddress::new(0x0000_0000))
                .unwrap_err(),
            DirectRdramAccessError {
                cpu_address: CpuAddress::new(0x0000_0000),
                width: 4
            }
        );
        assert_eq!(
            machine
                .write_direct_rdram_u32_be(kseg1(RDRAM_SIZE_BYTES), 0xaabb_ccdd)
                .unwrap_err(),
            DirectRdramAccessError {
                cpu_address: kseg1(RDRAM_SIZE_BYTES),
                width: 4
            }
        );
        assert_default_cpu_exception_state(&machine);
        assert_eq!(machine.rdram().read_u32_be(0), Ok(0x1122_3344));
        assert!(machine.cpu_rdram_reservation.is_valid());
        assert_eq!(machine.cpu_rdram_reservation.rdram_offset(), 0);
        assert_eq!(machine.cpu_rdram_reservation.width(), 4);
    }

    #[test]
    fn direct_rdram_instruction_fetch_reads_kseg0_and_kseg1_big_endian_words() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        machine
            .write_rdram_u32_be(0x0000_0020, 0x3c01_1234)
            .unwrap();
        machine
            .write_rdram_u32_be(0x0000_0024, 0x3421_5678)
            .unwrap();

        assert_eq!(
            machine.fetch_direct_rdram_cpu_instruction_word(kseg0(0x0000_0020)),
            Ok(CpuInstructionWord::new(0x3c01_1234))
        );
        assert_eq!(
            machine.fetch_direct_rdram_cpu_instruction_word(kseg1(0x0000_0024)),
            Ok(CpuInstructionWord::new(0x3421_5678))
        );
    }

    #[test]
    fn direct_rdram_instruction_fetch_uses_last_valid_word_boundary() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = RDRAM_SIZE_BYTES - 4;

        machine
            .write_rdram_u32_be(last_valid_offset, 0xaabb_ccdd)
            .unwrap();

        assert_eq!(
            machine.fetch_direct_rdram_cpu_instruction_word(kseg0(last_valid_offset)),
            Ok(CpuInstructionWord::new(0xaabb_ccdd))
        );
        assert_direct_fetch_error(
            machine
                .fetch_direct_rdram_cpu_instruction_word(kseg1(RDRAM_SIZE_BYTES))
                .unwrap_err(),
            kseg1(RDRAM_SIZE_BYTES),
        );
        assert_direct_fetch_error(
            machine
                .fetch_direct_rdram_cpu_instruction_word(kseg0(RDRAM_SIZE_BYTES + 4))
                .unwrap_err(),
            kseg0(RDRAM_SIZE_BYTES + 4),
        );
    }

    #[test]
    fn direct_rdram_instruction_fetch_checks_alignment_before_target_rejection() {
        let machine = Machine::from_cartridge(Cartridge::default());

        assert_unaligned_fetch_error(
            machine
                .fetch_direct_rdram_cpu_instruction_word(kseg0(0x0000_0001))
                .unwrap_err(),
            kseg0(0x0000_0001),
        );
        assert_unaligned_fetch_error(
            machine
                .fetch_direct_rdram_cpu_instruction_word(CpuAddress::new(0x0000_0002))
                .unwrap_err(),
            CpuAddress::new(0x0000_0002),
        );
    }

    #[test]
    fn direct_rdram_instruction_fetch_rejects_non_direct_and_pif_reset_without_exception_entry() {
        let machine = Machine::from_cartridge(Cartridge::default());

        assert_direct_fetch_error(
            machine
                .fetch_direct_rdram_cpu_instruction_word(CpuAddress::new(0x0000_0000))
                .unwrap_err(),
            CpuAddress::new(0x0000_0000),
        );
        assert_direct_fetch_error(
            machine
                .fetch_direct_rdram_cpu_instruction_word(CpuAddress::new(NON_BOOT_RESET_VECTOR_PC))
                .unwrap_err(),
            CpuAddress::new(NON_BOOT_RESET_VECTOR_PC),
        );

        assert_default_cpu_exception_state(&machine);
    }

    #[test]
    fn fetched_direct_rdram_instruction_word_can_be_decoded_and_identified_by_sealed_cpu_layers() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0030, 0x3408_00ab)
            .unwrap();

        let word = machine
            .fetch_direct_rdram_cpu_instruction_word(kseg0(0x0000_0030))
            .unwrap();
        let fields = decode_cpu_instruction_word(word);
        let identity = identify_cpu_instruction(fields);

        assert_eq!(word.bits(), 0x3408_00ab);
        assert_eq!(fields.raw(), word);
        assert_eq!(identity, CpuInstructionIdentity::Ori);
    }

    #[test]
    fn direct_rdram_instruction_fetch_preserves_machine_cpu_rdram_and_reservation_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
        );
        let cartridge_size_before = machine.cartridge().size_bytes();

        assert_eq!(
            machine.fetch_direct_rdram_cpu_instruction_word(kseg1(0x0000_0040)),
            Ok(CpuInstructionWord::new(0x8cc5_0104))
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
            )
        );
        assert_eq!(cartridge_size_before, machine.cartridge().size_bytes());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
    }

    #[test]
    fn cpu_instruction_fetch_target_classifies_kseg0_and_kseg1_direct_rdram() {
        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(kseg0(0x0000_0100)),
            Ok(MachineCpuInstructionFetchTarget::DirectRdram {
                offset: RdramOffset::new(0x0000_0100)
            })
        );
        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(kseg1(0x0000_0200)),
            Ok(MachineCpuInstructionFetchTarget::DirectRdram {
                offset: RdramOffset::new(0x0000_0200)
            })
        );

        let target = Machine::classify_cpu_instruction_fetch_target(kseg0(0x0000_0100)).unwrap();
        assert_eq!(target.width(), 4);
        assert_eq!(
            target.direct_rdram_offset(),
            Some(RdramOffset::new(0x0000_0100))
        );
        assert_eq!(target.sp_dmem_offset(), None);
        assert!(!target.is_pif_reset_unavailable());
    }

    #[test]
    fn cpu_instruction_fetch_target_uses_width_four_rdram_span_boundaries() {
        let last_valid_offset = RDRAM_SIZE_BYTES - 4;

        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(kseg0(last_valid_offset)),
            Ok(MachineCpuInstructionFetchTarget::DirectRdram {
                offset: RdramOffset::new(last_valid_offset as u32)
            })
        );
        assert_fetch_target_direct_miss_error(
            Machine::classify_cpu_instruction_fetch_target(kseg0(RDRAM_SIZE_BYTES)).unwrap_err(),
            kseg0(RDRAM_SIZE_BYTES),
        );
        assert_fetch_target_direct_miss_error(
            Machine::classify_cpu_instruction_fetch_target(kseg1(RDRAM_SIZE_BYTES + 4))
                .unwrap_err(),
            kseg1(RDRAM_SIZE_BYTES + 4),
        );
    }

    #[test]
    fn cpu_instruction_fetch_target_classifies_sp_dmem_fetch_target() {
        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0x8400_0000)),
            Ok(MachineCpuInstructionFetchTarget::SpDmem {
                offset: SpDmemOffset::new(0)
            })
        );
        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0xa400_0040)),
            Ok(MachineCpuInstructionFetchTarget::SpDmem {
                offset: SpDmemOffset::new(0x40)
            })
        );
        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0x8400_0ffc)),
            Ok(MachineCpuInstructionFetchTarget::SpDmem {
                offset: SpDmemOffset::new(0xffc)
            })
        );
        assert_fetch_target_direct_miss_error(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0x8400_1000))
                .unwrap_err(),
            CpuAddress::new(0x8400_1000),
        );
    }

    #[test]
    fn cpu_instruction_fetch_target_names_unavailable_pif_reset_fetch() {
        let target = Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(
            NON_BOOT_RESET_VECTOR_PC,
        ))
        .unwrap();

        assert_eq!(
            target,
            MachineCpuInstructionFetchTarget::PifResetUnavailable
        );
        assert_eq!(target.width(), 4);
        assert!(target.is_pif_reset_unavailable());
        assert_eq!(target.direct_rdram_offset(), None);
        assert_eq!(target.sp_dmem_offset(), None);
    }

    #[test]
    fn cpu_instruction_fetch_target_distinguishes_fetch_rejection_kinds() {
        assert_fetch_target_unaligned_error(
            Machine::classify_cpu_instruction_fetch_target(kseg0(0x0000_0001)).unwrap_err(),
            kseg0(0x0000_0001),
        );
        assert_fetch_target_unaligned_error(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0x0000_0002))
                .unwrap_err(),
            CpuAddress::new(0x0000_0002),
        );
        assert_fetch_target_unaligned_error(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0xa400_0042))
                .unwrap_err(),
            CpuAddress::new(0xa400_0042),
        );
        assert_fetch_target_non_direct_error(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0x0000_0000))
                .unwrap_err(),
            CpuAddress::new(0x0000_0000),
        );
        assert_fetch_target_direct_miss_error(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0x8500_0000))
                .unwrap_err(),
            CpuAddress::new(0x8500_0000),
        );
    }

    #[test]
    fn cpu_instruction_fetch_target_classification_preserves_machine_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0060, 0x3c01_1234)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0200, 8);
        machine.cpu.stage_pc(0x8000_3000);
        machine.cpu.stage_next_pc(0x8000_3004);
        assert_eq!(machine.cpu.set_gpr(9, 0x1122_3344_5566_7788), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0060),
            machine.rdram().read_u8(0x0000_0061),
            machine.rdram().read_u8(0x0000_0062),
            machine.rdram().read_u8(0x0000_0063),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().gpr(9),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
        );
        let cartridge_size_before = machine.cartridge().size_bytes();

        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(kseg0(0x0000_0060)),
            Ok(MachineCpuInstructionFetchTarget::DirectRdram {
                offset: RdramOffset::new(0x0000_0060)
            })
        );
        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0xa400_0040)),
            Ok(MachineCpuInstructionFetchTarget::SpDmem {
                offset: SpDmemOffset::new(0x40)
            })
        );
        assert_eq!(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(
                NON_BOOT_RESET_VECTOR_PC
            )),
            Ok(MachineCpuInstructionFetchTarget::PifResetUnavailable)
        );
        assert_fetch_target_direct_miss_error(
            Machine::classify_cpu_instruction_fetch_target(CpuAddress::new(0x8500_0000))
                .unwrap_err(),
            CpuAddress::new(0x8500_0000),
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0060),
                machine.rdram().read_u8(0x0000_0061),
                machine.rdram().read_u8(0x0000_0062),
                machine.rdram().read_u8(0x0000_0063),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().gpr(9),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
            )
        );
        assert_eq!(cartridge_size_before, machine.cartridge().size_bytes());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
    }

    #[test]
    fn sp_dmem_instruction_fetch_reads_one_big_endian_word() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);

        let word = machine
            .fetch_sp_dmem_cpu_instruction_word(SpDmemOffset::new(0x40))
            .unwrap();

        assert_eq!(word, CpuInstructionWord::new(0x3c01_1234));
        assert_eq!(word.bits(), 0x3c01_1234);
        assert_eq!(machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)), Ok(0x3c));
        assert_eq!(machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)), Ok(0x01));
        assert_eq!(machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)), Ok(0x12));
        assert_eq!(machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)), Ok(0x34));
    }

    #[test]
    fn sp_dmem_instruction_fetch_composes_with_decode_and_identity_only_in_tests() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x80), 0x3c01_1234);

        let word = machine
            .fetch_sp_dmem_cpu_instruction_word(SpDmemOffset::new(0x80))
            .unwrap();
        let fields = decode_cpu_instruction_word(word);

        assert_eq!(fields.raw(), CpuInstructionWord::new(0x3c01_1234));
        assert_eq!(fields.opcode(), 0x0f);
        assert_eq!(fields.rt(), 1);
        assert_eq!(fields.immediate_u16(), 0x1234);
        assert_eq!(
            identify_cpu_instruction(fields),
            CpuInstructionIdentity::Lui
        );
    }

    #[test]
    fn sp_dmem_instruction_fetch_uses_width_four_span_boundary() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        let last_valid_offset = SpDmemOffset::new((SP_DMEM_SIZE_BYTES - 4) as u32);
        machine
            .sp_dmem
            .write_u32_be_for_test(last_valid_offset, 0x0123_4567);

        assert_eq!(
            machine.fetch_sp_dmem_cpu_instruction_word(last_valid_offset),
            Ok(CpuInstructionWord::new(0x0123_4567))
        );

        for offset in [
            SP_DMEM_SIZE_BYTES - 3,
            SP_DMEM_SIZE_BYTES - 2,
            SP_DMEM_SIZE_BYTES - 1,
            SP_DMEM_SIZE_BYTES,
        ] {
            let error = machine
                .fetch_sp_dmem_cpu_instruction_word(SpDmemOffset::new(offset as u32))
                .unwrap_err();
            assert_eq!(error.offset(), SpDmemOffset::new(offset as u32));
            assert_eq!(error.width(), 4);
            assert_eq!(error.source().offset(), SpDmemOffset::new(offset as u32));
            assert_eq!(error.source().width(), 4);
        }
    }

    #[test]
    fn sp_dmem_instruction_fetch_preserves_machine_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine
            .write_rdram_u32_be(0x0000_0060, 0x8c22_0004)
            .unwrap();
        machine.cpu_rdram_reservation.stage(0x0000_0200, 8);
        machine.cpu.stage_pc(0x8000_3000);
        machine.cpu.stage_next_pc(0x8000_3004);
        assert_eq!(machine.cpu.set_gpr(9, 0x1122_3344_5566_7788), Ok(()));

        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let rdram_before = [
            machine.rdram().read_u8(0x0000_0060),
            machine.rdram().read_u8(0x0000_0061),
            machine.rdram().read_u8(0x0000_0062),
            machine.rdram().read_u8(0x0000_0063),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().gpr(9),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
        );
        let cartridge_size_before = machine.cartridge().size_bytes();

        assert_eq!(
            machine.fetch_sp_dmem_cpu_instruction_word(SpDmemOffset::new(0x40)),
            Ok(CpuInstructionWord::new(0x3c01_1234))
        );

        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0060),
                machine.rdram().read_u8(0x0000_0061),
                machine.rdram().read_u8(0x0000_0062),
                machine.rdram().read_u8(0x0000_0063),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().gpr(9),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
            )
        );
        assert_eq!(cartridge_size_before, machine.cartridge().size_bytes());
    }

    #[test]
    fn explicit_address_instruction_fetch_reads_direct_rdram_and_sp_dmem_targets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0020, 0x3408_00ab)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);

        assert_eq!(
            machine.fetch_cpu_instruction_word_at(kseg0(0x0000_0020)),
            Ok(CpuInstructionWord::new(0x3408_00ab))
        );
        assert_eq!(
            machine.fetch_cpu_instruction_word_at(kseg1(0x0000_0020)),
            Ok(CpuInstructionWord::new(0x3408_00ab))
        );
        assert_eq!(
            machine.fetch_cpu_instruction_word_at(CpuAddress::new(0xa400_0040)),
            Ok(CpuInstructionWord::new(0x3c01_1234))
        );
    }

    #[test]
    fn explicit_address_instruction_fetch_composes_with_decode_and_identity_only_in_tests() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0030, 0x3408_00ab)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x80), 0x3c01_1234);

        let direct_word = machine
            .fetch_cpu_instruction_word_at(kseg0(0x0000_0030))
            .unwrap();
        let direct_fields = decode_cpu_instruction_word(direct_word);
        assert_eq!(direct_fields.raw(), CpuInstructionWord::new(0x3408_00ab));
        assert_eq!(
            identify_cpu_instruction(direct_fields),
            CpuInstructionIdentity::Ori
        );

        let sp_word = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0xa400_0080))
            .unwrap();
        let sp_fields = decode_cpu_instruction_word(sp_word);
        assert_eq!(sp_fields.raw(), CpuInstructionWord::new(0x3c01_1234));
        assert_eq!(
            identify_cpu_instruction(sp_fields),
            CpuInstructionIdentity::Lui
        );
    }

    #[test]
    fn explicit_address_instruction_fetch_reports_named_rejections() {
        let machine = Machine::from_cartridge(Cartridge::default());

        let unaligned = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0xa400_0042))
            .unwrap_err();
        assert_eq!(unaligned.cpu_address(), CpuAddress::new(0xa400_0042));
        assert_eq!(unaligned.width(), 4);
        assert!(unaligned.is_unaligned());
        assert!(!unaligned.is_non_direct_unsupported());
        assert!(!unaligned.is_direct_target_miss());
        assert!(!unaligned.is_pif_reset_unavailable());
        assert_eq!(unaligned.direct_rdram_error(), None);
        assert_eq!(unaligned.sp_dmem_error(), None);
        assert_eq!(unaligned.sp_dmem_offset(), None);

        let pif = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(NON_BOOT_RESET_VECTOR_PC))
            .unwrap_err();
        assert_eq!(pif.cpu_address(), CpuAddress::new(NON_BOOT_RESET_VECTOR_PC));
        assert!(pif.is_pif_reset_unavailable());
        assert_eq!(pif.direct_rdram_error(), None);
        assert_eq!(pif.sp_dmem_error(), None);

        let non_direct = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x0000_0000))
            .unwrap_err();
        assert_eq!(non_direct.cpu_address(), CpuAddress::new(0x0000_0000));
        assert!(non_direct.is_non_direct_unsupported());
        assert_eq!(non_direct.direct_rdram_error(), None);
        assert_eq!(non_direct.sp_dmem_error(), None);

        let direct_miss = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x8500_0000))
            .unwrap_err();
        assert_eq!(direct_miss.cpu_address(), CpuAddress::new(0x8500_0000));
        assert!(direct_miss.is_direct_target_miss());
        assert_eq!(direct_miss.direct_rdram_error(), None);
        assert_eq!(direct_miss.sp_dmem_error(), None);

        let rdram_end = machine
            .fetch_cpu_instruction_word_at(kseg0(RDRAM_SIZE_BYTES))
            .unwrap_err();
        assert_eq!(rdram_end.cpu_address(), kseg0(RDRAM_SIZE_BYTES));
        assert!(rdram_end.is_direct_target_miss());
        assert_eq!(rdram_end.direct_rdram_error(), None);
        assert_eq!(rdram_end.sp_dmem_error(), None);
    }

    #[test]
    fn explicit_address_instruction_fetch_preserves_machine_state_on_success_and_error() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        assert_eq!(
            machine.fetch_cpu_instruction_word_at(kseg1(0x0000_0040)),
            Ok(CpuInstructionWord::new(0x8cc5_0104))
        );
        assert_eq!(
            machine.fetch_cpu_instruction_word_at(CpuAddress::new(0xa400_0040)),
            Ok(CpuInstructionWord::new(0x3c01_1234))
        );
        assert!(machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0xa400_0042))
            .unwrap_err()
            .is_unaligned());
        assert!(machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(NON_BOOT_RESET_VECTOR_PC))
            .unwrap_err()
            .is_pif_reset_unavailable());
        assert!(machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x0000_0000))
            .unwrap_err()
            .is_non_direct_unsupported());
        assert!(machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x8500_0000))
            .unwrap_err()
            .is_direct_target_miss());

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn current_pc_instruction_fetch_uses_reset_pc_and_reports_pif_unavailable() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        let construction_error = machine.fetch_current_cpu_instruction_word().unwrap_err();
        assert_eq!(
            construction_error.cpu_address(),
            CpuAddress::new(NON_BOOT_RESET_VECTOR_PC)
        );
        assert!(construction_error.is_pif_reset_unavailable());
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);

        machine.cpu.stage_pc(kseg0(0x0000_0020).value());
        machine
            .write_rdram_u32_be(0x0000_0020, 0x3408_00ab)
            .unwrap();
        assert_eq!(
            machine.fetch_current_cpu_instruction_word(),
            Ok(CpuInstructionWord::new(0x3408_00ab))
        );

        machine.reset();

        let reset_error = machine.fetch_current_cpu_instruction_word().unwrap_err();
        assert_eq!(
            reset_error.cpu_address(),
            CpuAddress::new(NON_BOOT_RESET_VECTOR_PC)
        );
        assert!(reset_error.is_pif_reset_unavailable());
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
    }

    #[test]
    fn current_pc_instruction_fetch_reads_direct_rdram_and_sp_dmem_targets() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0020, 0x3408_00ab)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);

        machine.cpu.stage_pc(kseg0(0x0000_0020).value());
        assert_eq!(
            machine.fetch_current_cpu_instruction_word(),
            Ok(CpuInstructionWord::new(0x3408_00ab))
        );

        machine.cpu.stage_pc(kseg1(0x0000_0020).value());
        assert_eq!(
            machine.fetch_current_cpu_instruction_word(),
            Ok(CpuInstructionWord::new(0x3408_00ab))
        );

        machine.cpu.stage_pc(0xa400_0040);
        assert_eq!(
            machine.fetch_current_cpu_instruction_word(),
            Ok(CpuInstructionWord::new(0x3c01_1234))
        );
    }

    #[test]
    fn current_pc_instruction_fetch_returns_same_rejections_as_explicit_address_fetch() {
        let mut machine = Machine::from_cartridge(Cartridge::default());

        for address in [
            0xa400_0042,
            NON_BOOT_RESET_VECTOR_PC,
            0x0000_0000,
            0x8500_0000,
            kseg0(RDRAM_SIZE_BYTES).value(),
        ] {
            machine.cpu.stage_pc(address);

            assert_eq!(
                machine.fetch_current_cpu_instruction_word(),
                machine.fetch_cpu_instruction_word_at(CpuAddress::new(address))
            );
        }
    }

    #[test]
    fn current_pc_instruction_fetch_composes_with_decode_and_identity_only_in_tests() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0030, 0x3408_00ab)
            .unwrap();

        machine.cpu.stage_pc(kseg0(0x0000_0030).value());

        let word = machine.fetch_current_cpu_instruction_word().unwrap();
        let fields = decode_cpu_instruction_word(word);

        assert_eq!(fields.raw(), CpuInstructionWord::new(0x3408_00ab));
        assert_eq!(
            identify_cpu_instruction(fields),
            CpuInstructionIdentity::Ori
        );
    }

    #[test]
    fn current_pc_instruction_fetch_preserves_machine_state_on_success_and_error() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(kseg1(0x0000_0040).value());
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        assert_eq!(
            machine.fetch_current_cpu_instruction_word(),
            Ok(CpuInstructionWord::new(0x8cc5_0104))
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );

        machine.cpu.stage_pc(0xa400_0042);
        machine.cpu.stage_next_pc(0x8000_3004);
        let cpu_error_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
        );

        assert!(machine
            .fetch_current_cpu_instruction_word()
            .unwrap_err()
            .is_unaligned());

        assert_eq!(
            cpu_error_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
    }

    #[test]
    fn instruction_fetch_fault_selection_maps_source_clear_faults_to_adel() {
        let machine = Machine::from_cartridge(Cartridge::default());

        let unaligned = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0xa400_0042))
            .unwrap_err();
        let unaligned_plan = select_cpu_instruction_fetch_address_error(unaligned).unwrap();
        assert_instruction_fetch_address_error_plan(
            unaligned_plan,
            unaligned,
            MachineInstructionFetchAddressErrorSource::Unaligned,
            CpuAddress::new(0xa400_0042),
        );

        let direct_miss = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x8500_0000))
            .unwrap_err();
        let direct_miss_plan = select_cpu_instruction_fetch_address_error(direct_miss).unwrap();
        assert_instruction_fetch_address_error_plan(
            direct_miss_plan,
            direct_miss,
            MachineInstructionFetchAddressErrorSource::DirectTargetMiss,
            CpuAddress::new(0x8500_0000),
        );

        let pif = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(NON_BOOT_RESET_VECTOR_PC))
            .unwrap_err();
        let pif_plan = select_cpu_instruction_fetch_address_error(pif).unwrap();
        assert_instruction_fetch_address_error_plan(
            pif_plan,
            pif,
            MachineInstructionFetchAddressErrorSource::PifResetUnavailable,
            CpuAddress::new(NON_BOOT_RESET_VECTOR_PC),
        );
        assert_eq!(
            pif_plan.to_string(),
            format!(
                "CPU instruction fetch PifResetUnavailable selected AdEL for 4-byte address error at {}",
                NON_BOOT_RESET_VECTOR_PC
            )
        );
    }

    #[test]
    fn instruction_fetch_fault_selection_preserves_non_converting_faults() {
        let machine = Machine::from_cartridge(Cartridge::default());

        let non_direct = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x0000_0000))
            .unwrap_err();
        let non_direct_error = select_cpu_instruction_fetch_address_error(non_direct).unwrap_err();
        assert_eq!(non_direct_error.fetch_error(), non_direct);
        assert_eq!(non_direct_error.cpu_address(), CpuAddress::new(0x0000_0000));
        assert_eq!(non_direct_error.width(), 4);
        assert_eq!(
            non_direct_error.to_string(),
            "CPU instruction fetch fault does not select local address-error entry: CPU instruction fetch unsupported for non-direct address: 0"
        );

        let direct_rdram_source = machine
            .fetch_direct_rdram_cpu_instruction_word(kseg0(RDRAM_SIZE_BYTES))
            .unwrap_err();
        let direct_rdram_error = MachineCpuInstructionFetchError::DirectRdram {
            cpu_address: kseg0(RDRAM_SIZE_BYTES),
            source: direct_rdram_source,
        };
        assert_eq!(
            select_cpu_instruction_fetch_address_error(direct_rdram_error)
                .unwrap_err()
                .fetch_error(),
            direct_rdram_error
        );

        let sp_offset = SpDmemOffset::new(SP_DMEM_SIZE_BYTES as u32);
        let sp_source = machine
            .fetch_sp_dmem_cpu_instruction_word(sp_offset)
            .unwrap_err();
        let sp_error = MachineCpuInstructionFetchError::SpDmem {
            cpu_address: CpuAddress::new(0xa400_1000),
            offset: sp_offset,
            source: sp_source,
        };
        assert_eq!(
            select_cpu_instruction_fetch_address_error(sp_error)
                .unwrap_err()
                .fetch_error(),
            sp_error
        );
    }

    #[test]
    fn step_fetch_fault_action_classifies_convertible_faults_as_adel_entry() {
        let machine = Machine::from_cartridge(Cartridge::default());

        let unaligned = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0xa400_0042))
            .unwrap_err();
        let unaligned_action = classify_step_fetch_fault_action(unaligned);
        assert_step_fetch_fault_enters_address_error(
            unaligned_action,
            unaligned,
            MachineInstructionFetchAddressErrorSource::Unaligned,
            CpuAddress::new(0xa400_0042),
        );

        let direct_miss = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x8500_0000))
            .unwrap_err();
        let direct_miss_action = classify_step_fetch_fault_action(direct_miss);
        assert_step_fetch_fault_enters_address_error(
            direct_miss_action,
            direct_miss,
            MachineInstructionFetchAddressErrorSource::DirectTargetMiss,
            CpuAddress::new(0x8500_0000),
        );

        let pif = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(NON_BOOT_RESET_VECTOR_PC))
            .unwrap_err();
        let pif_action = classify_step_fetch_fault_action(pif);
        assert_step_fetch_fault_enters_address_error(
            pif_action,
            pif,
            MachineInstructionFetchAddressErrorSource::PifResetUnavailable,
            CpuAddress::new(NON_BOOT_RESET_VECTOR_PC),
        );
        assert_eq!(
            pif_action.to_string(),
            format!(
                "CPU step fetch fault will enter CPU instruction fetch \
                 PifResetUnavailable selected AdEL for 4-byte address error at {}",
                NON_BOOT_RESET_VECTOR_PC
            )
        );
    }

    #[test]
    fn step_fetch_fault_action_classifies_non_converting_faults_as_rethrow() {
        let machine = Machine::from_cartridge(Cartridge::default());

        let non_direct = machine
            .fetch_cpu_instruction_word_at(CpuAddress::new(0x0000_0000))
            .unwrap_err();
        let non_direct_action = classify_step_fetch_fault_action(non_direct);
        assert_step_fetch_fault_rethrows(
            non_direct_action,
            non_direct,
            CpuAddress::new(0x0000_0000),
        );
        assert_eq!(
            non_direct_action.to_string(),
            "CPU step fetch fault will rethrow: CPU instruction fetch unsupported for non-direct address: 0"
        );

        let direct_rdram_address = kseg0(RDRAM_SIZE_BYTES);
        let direct_rdram_source = machine
            .fetch_direct_rdram_cpu_instruction_word(direct_rdram_address)
            .unwrap_err();
        let direct_rdram_error = MachineCpuInstructionFetchError::DirectRdram {
            cpu_address: direct_rdram_address,
            source: direct_rdram_source,
        };
        assert_step_fetch_fault_rethrows(
            classify_step_fetch_fault_action(direct_rdram_error),
            direct_rdram_error,
            direct_rdram_address,
        );

        let sp_offset = SpDmemOffset::new(SP_DMEM_SIZE_BYTES as u32);
        let sp_source = machine
            .fetch_sp_dmem_cpu_instruction_word(sp_offset)
            .unwrap_err();
        let sp_error = MachineCpuInstructionFetchError::SpDmem {
            cpu_address: CpuAddress::new(0xa400_1000),
            offset: sp_offset,
            source: sp_source,
        };
        assert_step_fetch_fault_rethrows(
            classify_step_fetch_fault_action(sp_error),
            sp_error,
            CpuAddress::new(0xa400_1000),
        );
    }

    #[test]
    fn step_fetch_fault_action_performs_no_machine_mutation() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0xa400_0042);
        machine.cpu.stage_next_pc(0x8000_3004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        let fetch_error = machine.fetch_current_cpu_instruction_word().unwrap_err();
        let action = classify_step_fetch_fault_action(fetch_error);
        assert_step_fetch_fault_enters_address_error(
            action,
            fetch_error,
            MachineInstructionFetchAddressErrorSource::Unaligned,
            CpuAddress::new(0xa400_0042),
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn step_unsupported_instruction_classifies_source_clear_unknown_identities() {
        let unknown_primary = instruction_fields(0x7000_1234);
        assert_eq!(
            identify_cpu_instruction(unknown_primary),
            CpuInstructionIdentity::UnknownPrimary
        );
        let unknown_primary_outcome =
            classify_step_unsupported_instruction(unknown_primary).unwrap();
        assert_step_unsupported_instruction(
            unknown_primary_outcome,
            unknown_primary,
            CpuInstructionIdentity::UnknownPrimary,
            MachineStepUnsupportedInstructionCategory::UnknownPrimary,
        );

        let special_unknown = instruction_fields(0x00a6_3801);
        assert_eq!(
            identify_cpu_instruction(special_unknown),
            CpuInstructionIdentity::SpecialUnknown
        );
        let special_unknown_outcome =
            classify_step_unsupported_instruction(special_unknown).unwrap();
        assert_step_unsupported_instruction(
            special_unknown_outcome,
            special_unknown,
            CpuInstructionIdentity::SpecialUnknown,
            MachineStepUnsupportedInstructionCategory::SpecialUnknown,
        );

        let regimm_unknown = instruction_fields((0x01 << 26) | (0x05 << 21) | (0x04 << 16));
        assert_eq!(
            identify_cpu_instruction(regimm_unknown),
            CpuInstructionIdentity::RegimmUnknown
        );
        let regimm_unknown_outcome = classify_step_unsupported_instruction(regimm_unknown).unwrap();
        assert_step_unsupported_instruction(
            regimm_unknown_outcome,
            regimm_unknown,
            CpuInstructionIdentity::RegimmUnknown,
            MachineStepUnsupportedInstructionCategory::RegimmUnknown,
        );

        assert_eq!(
            regimm_unknown_outcome.to_string(),
            format!(
                "CPU step unsupported instruction RegimmUnknown: raw=0x{:08X} identity=RegimmUnknown",
                regimm_unknown.raw().bits()
            )
        );
    }

    #[test]
    fn step_unsupported_instruction_classifies_source_clear_known_unimplemented_identities() {
        let known_unimplemented_cases = [
            (
                instruction_fields(0x4200_0000),
                CpuInstructionIdentity::Cop0,
                MachineStepUnsupportedInstructionCategory::Cop0Unimplemented,
            ),
            (
                instruction_fields(0x4400_0000),
                CpuInstructionIdentity::Cop1,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0x4800_0000),
                CpuInstructionIdentity::Cop2,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0x4c00_0000),
                CpuInstructionIdentity::Cop3,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xbc00_0000),
                CpuInstructionIdentity::Cache,
                MachineStepUnsupportedInstructionCategory::CacheUnimplemented,
            ),
            (
                instruction_fields(0xc400_0000),
                CpuInstructionIdentity::Lwc1,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xc800_0000),
                CpuInstructionIdentity::Lwc2,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xd400_0000),
                CpuInstructionIdentity::Ldc1,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xd800_0000),
                CpuInstructionIdentity::Ldc2,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xe400_0000),
                CpuInstructionIdentity::Swc1,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xe800_0000),
                CpuInstructionIdentity::Swc2,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xf400_0000),
                CpuInstructionIdentity::Sdc1,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
            (
                instruction_fields(0xf800_0000),
                CpuInstructionIdentity::Sdc2,
                MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
            ),
        ];

        for (fields, identity, category) in known_unimplemented_cases {
            assert_eq!(identify_cpu_instruction(fields), identity);
            let outcome = classify_step_unsupported_instruction(fields).unwrap();
            assert_step_unsupported_instruction(outcome, fields, identity, category);
        }
    }

    #[test]
    fn step_unsupported_instruction_classifies_invalid_cop0_register_forms() {
        let invalid_mfc0 = instruction_fields(0x4008_7800);
        assert_eq!(
            identify_cpu_instruction(invalid_mfc0),
            CpuInstructionIdentity::Cop0Mfc0
        );
        assert_step_unsupported_instruction(
            classify_step_unsupported_instruction(invalid_mfc0).unwrap(),
            invalid_mfc0,
            CpuInstructionIdentity::Cop0Mfc0,
            MachineStepUnsupportedInstructionCategory::Cop0RegisterUnsupported,
        );

        let invalid_mtc0 = instruction_fields(0x4088_7800);
        assert_eq!(
            identify_cpu_instruction(invalid_mtc0),
            CpuInstructionIdentity::Cop0Mtc0
        );
        assert_step_unsupported_instruction(
            classify_step_unsupported_instruction(invalid_mtc0).unwrap(),
            invalid_mtc0,
            CpuInstructionIdentity::Cop0Mtc0,
            MachineStepUnsupportedInstructionCategory::Cop0RegisterUnsupported,
        );
    }

    #[test]
    fn step_unsupported_instruction_does_not_classify_implemented_or_contextual_identities() {
        let nop = instruction_fields(0x0000_0000);
        assert_eq!(
            identify_cpu_instruction(nop),
            CpuInstructionIdentity::SpecialSll
        );
        assert_eq!(classify_step_unsupported_instruction(nop), None);

        let implemented_addiu = instruction_fields(0x2408_0001);
        assert_eq!(
            identify_cpu_instruction(implemented_addiu),
            CpuInstructionIdentity::Addiu
        );
        assert_eq!(
            classify_step_unsupported_instruction(implemented_addiu),
            None
        );

        let valid_cop0_mfc0 = instruction_fields(0x4008_4800);
        assert_eq!(
            identify_cpu_instruction(valid_cop0_mfc0),
            CpuInstructionIdentity::Cop0Mfc0
        );
        assert_eq!(classify_step_unsupported_instruction(valid_cop0_mfc0), None);

        let valid_cop0_mtc0 = instruction_fields(0x4088_4800);
        assert_eq!(
            identify_cpu_instruction(valid_cop0_mtc0),
            CpuInstructionIdentity::Cop0Mtc0
        );
        assert_eq!(classify_step_unsupported_instruction(valid_cop0_mtc0), None);

        let contextual_eret = instruction_fields(0x4200_0018);
        assert_eq!(
            identify_cpu_instruction(contextual_eret),
            CpuInstructionIdentity::Cop0Eret
        );
        assert_eq!(classify_step_unsupported_instruction(contextual_eret), None);

        let syscall = instruction_fields(0x0000_000c);
        assert_eq!(
            identify_cpu_instruction(syscall),
            CpuInstructionIdentity::SpecialSyscall
        );
        assert_eq!(classify_step_unsupported_instruction(syscall), None);

        let break_instruction = instruction_fields(0x0000_000d);
        assert_eq!(
            identify_cpu_instruction(break_instruction),
            CpuInstructionIdentity::SpecialBreak
        );
        assert_eq!(
            classify_step_unsupported_instruction(break_instruction),
            None
        );

        let sync = instruction_fields(0x0000_000f);
        assert_eq!(
            identify_cpu_instruction(sync),
            CpuInstructionIdentity::SpecialSync
        );
        assert_eq!(classify_step_unsupported_instruction(sync), None);

        let ll = instruction_fields(0xc000_0000);
        assert_eq!(identify_cpu_instruction(ll), CpuInstructionIdentity::Ll);
        assert_eq!(classify_step_unsupported_instruction(ll), None);

        let sc = instruction_fields(0xe000_0000);
        assert_eq!(identify_cpu_instruction(sc), CpuInstructionIdentity::Sc);
        assert_eq!(classify_step_unsupported_instruction(sc), None);
    }

    #[test]
    fn step_unsupported_instruction_classification_performs_no_machine_mutation() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        let fields = instruction_fields(0x7000_1234);
        let outcome = classify_step_unsupported_instruction(fields).unwrap();
        assert_step_unsupported_instruction(
            outcome,
            fields,
            CpuInstructionIdentity::UnknownPrimary,
            MachineStepUnsupportedInstructionCategory::UnknownPrimary,
        );

        let known_fields = instruction_fields(0x4400_0000);
        let known_outcome = classify_step_unsupported_instruction(known_fields).unwrap();
        assert_step_unsupported_instruction(
            known_outcome,
            known_fields,
            CpuInstructionIdentity::Cop1,
            MachineStepUnsupportedInstructionCategory::CoprocessorUnimplemented,
        );

        let invalid_cop0_fields = instruction_fields(0x4008_7800);
        let invalid_cop0_outcome =
            classify_step_unsupported_instruction(invalid_cop0_fields).unwrap();
        assert_step_unsupported_instruction(
            invalid_cop0_outcome,
            invalid_cop0_fields,
            CpuInstructionIdentity::Cop0Mfc0,
            MachineStepUnsupportedInstructionCategory::Cop0RegisterUnsupported,
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn step_stopped_instruction_classifies_source_clear_stopped_identities() {
        let syscall = instruction_fields(0x0000_000c);
        assert_eq!(
            identify_cpu_instruction(syscall),
            CpuInstructionIdentity::SpecialSyscall
        );
        let syscall_outcome = classify_step_stopped_instruction(syscall).unwrap();
        assert_step_stopped_instruction(
            syscall_outcome,
            syscall,
            CpuInstructionIdentity::SpecialSyscall,
            MachineStepStoppedInstructionCategory::Syscall,
        );

        let break_instruction = instruction_fields(0x0000_000d);
        assert_eq!(
            identify_cpu_instruction(break_instruction),
            CpuInstructionIdentity::SpecialBreak
        );
        let break_outcome = classify_step_stopped_instruction(break_instruction).unwrap();
        assert_step_stopped_instruction(
            break_outcome,
            break_instruction,
            CpuInstructionIdentity::SpecialBreak,
            MachineStepStoppedInstructionCategory::Break,
        );

        assert_eq!(
            break_outcome.to_string(),
            "CPU step stopped instruction Break: raw=0x0000000D identity=SpecialBreak"
        );
    }

    #[test]
    fn step_stopped_instruction_does_not_classify_executed_unsupported_or_implemented_identities() {
        let sync = instruction_fields(0x0000_000f);
        assert_eq!(
            identify_cpu_instruction(sync),
            CpuInstructionIdentity::SpecialSync
        );
        assert_eq!(classify_step_stopped_instruction(sync), None);

        let unknown_primary = instruction_fields(0x7000_1234);
        assert_eq!(
            identify_cpu_instruction(unknown_primary),
            CpuInstructionIdentity::UnknownPrimary
        );
        assert!(classify_step_unsupported_instruction(unknown_primary).is_some());
        assert_eq!(classify_step_stopped_instruction(unknown_primary), None);

        let known_unimplemented = instruction_fields(0x4400_0000);
        assert_eq!(
            identify_cpu_instruction(known_unimplemented),
            CpuInstructionIdentity::Cop1
        );
        assert!(classify_step_unsupported_instruction(known_unimplemented).is_some());
        assert_eq!(classify_step_stopped_instruction(known_unimplemented), None);

        let nop = instruction_fields(0x0000_0000);
        assert_eq!(
            identify_cpu_instruction(nop),
            CpuInstructionIdentity::SpecialSll
        );
        assert_eq!(classify_step_stopped_instruction(nop), None);

        let implemented_addiu = instruction_fields(0x2408_0001);
        assert_eq!(
            identify_cpu_instruction(implemented_addiu),
            CpuInstructionIdentity::Addiu
        );
        assert_eq!(classify_step_stopped_instruction(implemented_addiu), None);
    }

    #[test]
    fn step_stopped_instruction_classification_performs_no_machine_mutation() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        let syscall_fields = instruction_fields(0x0000_000c);
        let syscall_outcome = classify_step_stopped_instruction(syscall_fields).unwrap();
        assert_step_stopped_instruction(
            syscall_outcome,
            syscall_fields,
            CpuInstructionIdentity::SpecialSyscall,
            MachineStepStoppedInstructionCategory::Syscall,
        );

        let break_fields = instruction_fields(0x0000_000d);
        let break_outcome = classify_step_stopped_instruction(break_fields).unwrap();
        assert_step_stopped_instruction(
            break_outcome,
            break_fields,
            CpuInstructionIdentity::SpecialBreak,
            MachineStepStoppedInstructionCategory::Break,
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn step_no_effect_executed_instruction_classifies_source_clear_sync_identity() {
        let sync = instruction_fields(0x0000_000f);
        assert_eq!(
            identify_cpu_instruction(sync),
            CpuInstructionIdentity::SpecialSync
        );

        let outcome = classify_step_no_effect_executed_instruction(sync).unwrap();
        assert_step_no_effect_executed_instruction(
            outcome,
            sync,
            CpuInstructionIdentity::SpecialSync,
            MachineStepNoEffectExecutedInstructionCategory::Sync,
        );
        assert_eq!(
            outcome.to_string(),
            "CPU step no-effect executed instruction Sync: raw=0x0000000F identity=SpecialSync"
        );
    }

    #[test]
    fn step_no_effect_executed_instruction_does_not_classify_stopped_unsupported_or_writeback_paths(
    ) {
        let syscall = instruction_fields(0x0000_000c);
        assert_eq!(
            identify_cpu_instruction(syscall),
            CpuInstructionIdentity::SpecialSyscall
        );
        assert!(classify_step_stopped_instruction(syscall).is_some());
        assert_eq!(classify_step_no_effect_executed_instruction(syscall), None);

        let break_instruction = instruction_fields(0x0000_000d);
        assert_eq!(
            identify_cpu_instruction(break_instruction),
            CpuInstructionIdentity::SpecialBreak
        );
        assert!(classify_step_stopped_instruction(break_instruction).is_some());
        assert_eq!(
            classify_step_no_effect_executed_instruction(break_instruction),
            None
        );

        let unknown_primary = instruction_fields(0x7000_1234);
        assert_eq!(
            identify_cpu_instruction(unknown_primary),
            CpuInstructionIdentity::UnknownPrimary
        );
        assert!(classify_step_unsupported_instruction(unknown_primary).is_some());
        assert_eq!(
            classify_step_no_effect_executed_instruction(unknown_primary),
            None
        );

        let known_unimplemented = instruction_fields(0x4400_0000);
        assert_eq!(
            identify_cpu_instruction(known_unimplemented),
            CpuInstructionIdentity::Cop1
        );
        assert!(classify_step_unsupported_instruction(known_unimplemented).is_some());
        assert_eq!(
            classify_step_no_effect_executed_instruction(known_unimplemented),
            None
        );

        let nop = instruction_fields(0x0000_0000);
        assert_eq!(
            identify_cpu_instruction(nop),
            CpuInstructionIdentity::SpecialSll
        );
        assert_eq!(classify_step_no_effect_executed_instruction(nop), None);

        let implemented_addiu = instruction_fields(0x2408_0001);
        assert_eq!(
            identify_cpu_instruction(implemented_addiu),
            CpuInstructionIdentity::Addiu
        );
        assert_eq!(
            classify_step_no_effect_executed_instruction(implemented_addiu),
            None
        );
    }

    #[test]
    fn step_no_effect_executed_instruction_classification_performs_no_machine_mutation() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        let sync_fields = instruction_fields(0x0000_000f);
        let sync_outcome = classify_step_no_effect_executed_instruction(sync_fields).unwrap();
        assert_step_no_effect_executed_instruction(
            sync_outcome,
            sync_fields,
            CpuInstructionIdentity::SpecialSync,
            MachineStepNoEffectExecutedInstructionCategory::Sync,
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn special_shift_execution_mutates_only_destination_gpr() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x0000_0000_4000_0001), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(6),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(special_shift_word(0, 4, 5, 1, 0x00));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::SpecialSll);

        let executed = machine
            .cpu
            .execute_special_shift_instruction(identity, fields)
            .expect("SLL should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialSll);
        assert_eq!(machine.cpu().gpr(5), Some(0xffff_ffff_8000_0002));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(6),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn special_64_bit_shift_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x0000_0000_0000_0003), Ok(()));
        assert_eq!(machine.cpu.set_gpr(5, 0x0000_0000_0000_0041), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(5),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(special_shift_word(5, 4, 6, 0, 0x14));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::SpecialDsllv);

        let executed = machine
            .cpu
            .execute_special_shift_instruction(identity, fields)
            .expect("DSLLV should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialDsllv);
        assert_eq!(machine.cpu().gpr(6), Some(0x0000_0000_0000_0006));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(5),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn special_bitwise_logical_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x0000_ffff_0000_ffff), Ok(()));
        assert_eq!(machine.cpu.set_gpr(5, 0x00ff_00ff_0000_0000), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(5),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(special_shift_word(4, 5, 6, 0, 0x27));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::SpecialNor);

        let executed = machine
            .cpu
            .execute_special_bitwise_logical_instruction(identity, fields)
            .expect("NOR should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialNor);
        assert_eq!(machine.cpu().gpr(6), Some(0xff00_0000_ffff_0000));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(5),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn special_hi_lo_transfer_execution_mutates_only_intended_cpu_scalar() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0xffff_0000_aaaa_5555), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(6),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(special_shift_word(4, 0, 0, 0, 0x11));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::SpecialMthi);

        let executed = machine
            .cpu
            .execute_special_hi_lo_transfer_instruction(identity, fields)
            .expect("MTHI should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialMthi);
        assert_eq!(machine.cpu().hi(), 0xffff_0000_aaaa_5555);
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(6),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn special_non_trapping_integer_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, u64::MAX), Ok(()));
        assert_eq!(machine.cpu.set_gpr(5, 2), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(5),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(special_shift_word(4, 5, 6, 0, 0x2d));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::SpecialDaddu);

        let executed = machine
            .cpu
            .execute_special_non_trapping_integer_instruction(identity, fields)
            .expect("DADDU should execute");

        assert_eq!(executed.identity(), CpuInstructionIdentity::SpecialDaddu);
        assert_eq!(machine.cpu().gpr(6), Some(1));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(5),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn special_trapping_integer_non_overflow_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x0000_0000_7fff_fffe), Ok(()));
        assert_eq!(machine.cpu.set_gpr(5, 1), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(5),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(special_shift_word(4, 5, 6, 0, 0x20));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::SpecialAdd);

        let outcome = machine
            .cpu
            .execute_special_trapping_integer_instruction(identity, fields)
            .expect("ADD should execute without overflow");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialAdd);
        assert!(outcome.is_executed());
        assert_eq!(machine.cpu().gpr(6), Some(0x0000_0000_7fff_ffff));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(5),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn special_trapping_integer_overflow_outcome_mutates_no_machine_state() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x7fff_ffff), Ok(()));
        assert_eq!(machine.cpu.set_gpr(5, 1), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(5),
            machine.cpu().gpr(6),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(special_shift_word(4, 5, 6, 0, 0x20));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::SpecialAdd);

        let outcome = machine
            .cpu
            .execute_special_trapping_integer_instruction(identity, fields)
            .expect("ADD overflow should return an outcome");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialAdd);
        assert!(outcome.is_overflow());
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(5),
                machine.cpu().gpr(6),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn immediate_trapping_integer_non_overflow_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x0000_0000_7fff_fffe), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(immediate_word(0x08, 4, 6, 1));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::Addi);

        let outcome = machine
            .cpu
            .execute_immediate_trapping_integer_instruction(identity, fields)
            .expect("ADDI should execute without overflow");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::Addi);
        assert!(outcome.is_executed());
        assert_eq!(machine.cpu().gpr(6), Some(0x0000_0000_7fff_ffff));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn immediate_trapping_integer_overflow_outcome_mutates_no_machine_state() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x7fff_ffff), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(6),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(immediate_word(0x08, 4, 6, 1));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::Addi);

        let outcome = machine
            .cpu
            .execute_immediate_trapping_integer_instruction(identity, fields)
            .expect("ADDI overflow should return an outcome");

        assert_eq!(outcome.identity(), CpuInstructionIdentity::Addi);
        assert!(outcome.is_overflow());
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(6),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn immediate_non_trapping_integer_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0xffff_ffff_ffff_ffff), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(immediate_word(0x19, 4, 6, 1));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::Daddiu);

        let executed = machine
            .cpu
            .execute_immediate_non_trapping_integer_instruction(identity, fields)
            .expect("DADDIU should execute with full-width wrapping");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Daddiu);
        assert_eq!(machine.cpu().gpr(6), Some(0));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn immediate_comparison_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0x0000_0000_0001_0000), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(immediate_word(0x0b, 4, 6, 0xffff));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::Sltiu);

        let executed = machine
            .cpu
            .execute_immediate_comparison_instruction(identity, fields)
            .expect("SLTIU should execute with sign-extended immediate comparison");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Sltiu);
        assert_eq!(machine.cpu().gpr(6), Some(1));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn immediate_bitwise_logical_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(4, 0xffff_0000_1234_ffff), Ok(()));
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(4),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(immediate_word(0x0e, 4, 6, 0xffff));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::Xori);

        let executed = machine
            .cpu
            .execute_immediate_bitwise_logical_instruction(identity, fields)
            .expect("XORI should execute with zero-extended immediate");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Xori);
        assert_eq!(machine.cpu().gpr(6), Some(0xffff_0000_1234_0000));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(4),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn upper_immediate_lui_execution_mutates_only_destination_gpr() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(6, 0x0123_4567_89ab_cdef), Ok(()));
        assert_eq!(machine.cpu.set_gpr(31, 0xfedc_ba98_7654_3210), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(31),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fields = instruction_fields(immediate_word(0x0f, 31, 6, 0x8000));
        let identity = identify_cpu_instruction(fields);
        assert_eq!(identity, CpuInstructionIdentity::Lui);

        let executed = machine
            .cpu
            .execute_upper_immediate_instruction(identity, fields)
            .expect("LUI should execute with sign-extended upper immediate");

        assert_eq!(executed.identity(), CpuInstructionIdentity::Lui);
        assert_eq!(machine.cpu().gpr(6), Some(0xffff_ffff_8000_0000));
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(31),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn cpu_local_executed_helper_selection_performs_no_machine_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let shift_selection = select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialSll)
            .expect("SLL should select the sealed shift family");
        assert_eq!(
            shift_selection.family(),
            CpuLocalExecutedHelperFamily::SpecialShift
        );
        let lui_selection = select_cpu_local_executed_helper(CpuInstructionIdentity::Lui)
            .expect("LUI should select the sealed upper-immediate family");
        assert_eq!(
            lui_selection.family(),
            CpuLocalExecutedHelperFamily::UpperImmediateLui
        );
        assert_eq!(
            select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialBreak),
            None
        );

        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn cpu_local_executed_helper_invocation_mutates_only_expected_cpu_local_state() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(2, 3), Ok(()));
        assert_eq!(machine.cpu.set_gpr(3, 0x9999), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(2),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let bits = special_shift_word(0, 2, 3, 2, 0x00);
        let fields = instruction_fields(bits);
        let selection = select_cpu_local_executed_helper(CpuInstructionIdentity::SpecialSll)
            .expect("SLL should select the sealed shift family");
        let outcome = machine
            .cpu
            .invoke_cpu_local_executed_helper(fields, selection)
            .expect("SLL should invoke through the selected local helper");

        assert!(outcome.is_executed());
        assert_eq!(outcome.identity(), CpuInstructionIdentity::SpecialSll);
        assert_eq!(outcome.family(), CpuLocalExecutedHelperFamily::SpecialShift);
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(2),
            )
        );
        assert_eq!(machine.cpu().gpr(3), Some(12));
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn cpu_local_invocation_step_action_maps_successes_to_committed_cadence() {
        let mut sync_machine = Machine::from_cartridge(Cartridge::default());
        let outcome =
            invoke_cpu_local_executed_helper_for_step_action(&mut sync_machine, 0x0000_000f)
                .expect("SYNC should invoke as a no-effect executed helper");
        assert_committed_local_step_action(
            classify_cpu_local_invocation_step_action(Ok(outcome)),
            CpuInstructionIdentity::SpecialSync,
            CpuLocalExecutedHelperFamily::NoEffectSync,
        );

        let mut shift_machine = Machine::from_cartridge(Cartridge::default());
        assert_eq!(shift_machine.cpu.set_gpr(2, 3), Ok(()));
        let outcome = invoke_cpu_local_executed_helper_for_step_action(
            &mut shift_machine,
            special_shift_word(0, 2, 3, 2, 0x00),
        )
        .expect("SLL should invoke as a local GPR writeback helper");
        assert_committed_local_step_action(
            classify_cpu_local_invocation_step_action(Ok(outcome)),
            CpuInstructionIdentity::SpecialSll,
            CpuLocalExecutedHelperFamily::SpecialShift,
        );

        let mut hi_lo_machine = Machine::from_cartridge(Cartridge::default());
        assert_eq!(hi_lo_machine.cpu.set_gpr(5, 0x1111_2222_3333_4444), Ok(()));
        let outcome = invoke_cpu_local_executed_helper_for_step_action(
            &mut hi_lo_machine,
            special_shift_word(5, 0, 0, 0, 0x11),
        )
        .expect("MTHI should invoke as a local HI/LO transfer helper");
        assert_committed_local_step_action(
            classify_cpu_local_invocation_step_action(Ok(outcome)),
            CpuInstructionIdentity::SpecialMthi,
            CpuLocalExecutedHelperFamily::SpecialHiLoTransfer,
        );

        let mut immediate_machine = Machine::from_cartridge(Cartridge::default());
        assert_eq!(immediate_machine.cpu.set_gpr(1, 7), Ok(()));
        let outcome = invoke_cpu_local_executed_helper_for_step_action(
            &mut immediate_machine,
            immediate_word(0x19, 1, 2, 2),
        )
        .expect("DADDIU should invoke as a local immediate helper");
        assert_committed_local_step_action(
            classify_cpu_local_invocation_step_action(Ok(outcome)),
            CpuInstructionIdentity::Daddiu,
            CpuLocalExecutedHelperFamily::ImmediateNonTrappingInteger,
        );

        let mut lui_machine = Machine::from_cartridge(Cartridge::default());
        let outcome = invoke_cpu_local_executed_helper_for_step_action(
            &mut lui_machine,
            immediate_word(0x0f, 0, 2, 0x8000),
        )
        .expect("LUI should invoke as a local upper-immediate helper");
        assert_committed_local_step_action(
            classify_cpu_local_invocation_step_action(Ok(outcome)),
            CpuInstructionIdentity::Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );
    }

    #[test]
    fn cpu_local_invocation_step_action_maps_overflow_to_exception_entry_plan() {
        let mut special_machine = Machine::from_cartridge(Cartridge::default());
        assert_eq!(
            special_machine.cpu.set_gpr(1, 0x0000_0000_7fff_ffff),
            Ok(())
        );
        assert_eq!(special_machine.cpu.set_gpr(2, 1), Ok(()));
        let special_outcome = invoke_cpu_local_executed_helper_for_step_action(
            &mut special_machine,
            special_shift_word(1, 2, 3, 0, 0x20),
        )
        .expect("ADD overflow should return a local overflow outcome");
        let special_plan = classify_cpu_local_invocation_step_action(Ok(special_outcome));
        assert_overflow_local_step_action(
            special_plan,
            CpuInstructionIdentity::SpecialAdd,
            CpuLocalExecutedHelperFamily::SpecialTrappingInteger,
        );
        match special_plan.overflow().unwrap() {
            CpuLocalExecutedHelperArithmeticOverflow::SpecialTrappingInteger(overflow) => {
                assert_eq!(overflow.rd(), 3);
                assert_eq!(overflow.rs_value(), 0x0000_0000_7fff_ffff);
                assert_eq!(overflow.rt_value(), 1);
            }
            CpuLocalExecutedHelperArithmeticOverflow::ImmediateTrappingInteger(_) => {
                panic!("expected SPECIAL overflow payload")
            }
        }

        let mut immediate_machine = Machine::from_cartridge(Cartridge::default());
        assert_eq!(
            immediate_machine.cpu.set_gpr(1, 0x0000_0000_7fff_ffff),
            Ok(())
        );
        let immediate_outcome = invoke_cpu_local_executed_helper_for_step_action(
            &mut immediate_machine,
            immediate_word(0x08, 1, 2, 1),
        )
        .expect("ADDI overflow should return a local overflow outcome");
        let immediate_plan = classify_cpu_local_invocation_step_action(Ok(immediate_outcome));
        assert_overflow_local_step_action(
            immediate_plan,
            CpuInstructionIdentity::Addi,
            CpuLocalExecutedHelperFamily::ImmediateTrappingInteger,
        );
        match immediate_plan.overflow().unwrap() {
            CpuLocalExecutedHelperArithmeticOverflow::SpecialTrappingInteger(_) => {
                panic!("expected immediate overflow payload")
            }
            CpuLocalExecutedHelperArithmeticOverflow::ImmediateTrappingInteger(overflow) => {
                assert_eq!(overflow.rt(), 2);
                assert_eq!(overflow.rs_value(), 0x0000_0000_7fff_ffff);
                assert_eq!(overflow.immediate_u16(), 1);
                assert_eq!(overflow.immediate_value(), 1);
            }
        }
    }

    #[test]
    fn cpu_local_invocation_step_action_excludes_non_local_step_identities() {
        for identity in [
            CpuInstructionIdentity::SpecialSyscall,
            CpuInstructionIdentity::SpecialBreak,
            CpuInstructionIdentity::UnknownPrimary,
            CpuInstructionIdentity::SpecialUnknown,
            CpuInstructionIdentity::RegimmUnknown,
            CpuInstructionIdentity::J,
            CpuInstructionIdentity::Beq,
            CpuInstructionIdentity::Lw,
            CpuInstructionIdentity::Sw,
            CpuInstructionIdentity::Cop0Mfc0,
            CpuInstructionIdentity::Cop0Mtc0,
            CpuInstructionIdentity::Cop0Eret,
            CpuInstructionIdentity::Ll,
            CpuInstructionIdentity::Sc,
        ] {
            assert_eq!(select_cpu_local_executed_helper(identity), None);
        }
    }

    #[test]
    fn cpu_local_invocation_step_action_planning_performs_no_machine_mutation() {
        let mut outcome_machine = Machine::from_cartridge(Cartridge::default());
        let outcome = invoke_cpu_local_executed_helper_for_step_action(
            &mut outcome_machine,
            immediate_word(0x0f, 0, 2, 0x1234),
        )
        .expect("LUI should produce a local executed outcome");

        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let plan = classify_cpu_local_invocation_step_action(Ok(outcome));
        assert_committed_local_step_action(
            plan,
            CpuInstructionIdentity::Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
    }

    #[test]
    fn machine_step_cadence_plan_maps_source_clear_outcomes() {
        assert_machine_step_cadence_plan(
            MachineStepCadenceSource::CommittedInstruction,
            MachineStepControlFlowAction::CommitStaged,
            MachineStepCountAction::Advance,
        );
        assert_machine_step_cadence_plan(
            MachineStepCadenceSource::StoppedInstruction,
            MachineStepControlFlowAction::CommitStaged,
            MachineStepCountAction::Advance,
        );
        assert_machine_step_cadence_plan(
            MachineStepCadenceSource::UnsupportedInstruction,
            MachineStepControlFlowAction::RestoreSnapshot,
            MachineStepCountAction::DoNotAdvance,
        );
        assert_machine_step_cadence_plan(
            MachineStepCadenceSource::InterruptedBeforeFetch,
            MachineStepControlFlowAction::ReturnBeforeCadence,
            MachineStepCountAction::DoNotAdvance,
        );
        assert_machine_step_cadence_plan(
            MachineStepCadenceSource::EnteredException,
            MachineStepControlFlowAction::PreserveExceptionVector,
            MachineStepCountAction::DoNotAdvance,
        );
        assert_machine_step_cadence_plan(
            MachineStepCadenceSource::FetchAddressErrorException,
            MachineStepControlFlowAction::PreserveExceptionVector,
            MachineStepCountAction::DoNotAdvance,
        );
    }

    #[test]
    fn machine_step_cadence_plan_keeps_eret_and_branch_likely_control_flow_blocked() {
        let eret_plan = classify_machine_step_cadence(MachineStepCadenceSource::SuccessfulEret);
        assert_eq!(eret_plan.source(), MachineStepCadenceSource::SuccessfulEret);
        assert_eq!(
            eret_plan.control_flow_action(),
            MachineStepControlFlowAction::BlockedByEretReturn
        );
        assert_eq!(eret_plan.count_action(), MachineStepCountAction::Advance);
        assert!(eret_plan.advances_count());
        assert!(!eret_plan.mutates_state());

        let branch_likely_plan =
            classify_machine_step_cadence(MachineStepCadenceSource::BranchLikelyAnnul);
        assert_eq!(
            branch_likely_plan.source(),
            MachineStepCadenceSource::BranchLikelyAnnul
        );
        assert_eq!(
            branch_likely_plan.control_flow_action(),
            MachineStepControlFlowAction::BlockedByBranchLikelyAnnul
        );
        assert_eq!(
            branch_likely_plan.count_action(),
            MachineStepCountAction::Advance
        );
        assert!(branch_likely_plan.advances_count());
        assert!(!branch_likely_plan.mutates_state());
        assert_eq!(
            branch_likely_plan.to_string(),
            "CPU step cadence BranchLikelyAnnul: \
             control_flow=BlockedByBranchLikelyAnnul count=Advance"
        );
    }

    #[test]
    fn machine_step_cadence_plan_performs_no_machine_mutation() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_2000);
        machine.cpu.stage_next_pc(0x8000_2004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        for source in [
            MachineStepCadenceSource::CommittedInstruction,
            MachineStepCadenceSource::StoppedInstruction,
            MachineStepCadenceSource::UnsupportedInstruction,
            MachineStepCadenceSource::InterruptedBeforeFetch,
            MachineStepCadenceSource::EnteredException,
            MachineStepCadenceSource::FetchAddressErrorException,
            MachineStepCadenceSource::SuccessfulEret,
            MachineStepCadenceSource::BranchLikelyAnnul,
        ] {
            let plan = classify_machine_step_cadence(source);
            assert_eq!(plan.source(), source);
            assert!(!plan.mutates_state());
        }

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn cpu_control_flow_restore_preserves_non_control_flow_machine_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);

        let snapshot = machine.cpu.capture_control_flow();
        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_non_control_flow_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        machine.cpu.stage_pc(0x8000_3000);
        machine.cpu.stage_next_pc(0x8000_4000);
        machine.cpu.restore_control_flow(snapshot);

        assert_eq!(machine.cpu().pc(), 0x8000_1000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2000);
        assert_eq!(
            cpu_non_control_flow_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn cpu_step_next_pc_staging_preserves_non_control_flow_machine_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_non_control_flow_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        machine.cpu.stage_next_sequential_pc_for_step();

        assert_eq!(machine.cpu().pc(), 0x8000_1000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(
            cpu_non_control_flow_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn committed_step_control_flow_preserves_non_control_flow_machine_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .enter_instruction_fetch_address_error_exception(CpuAddress::new(0x8000_1234))
            .unwrap();
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        let snapshot = machine.cpu.capture_control_flow();

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_non_control_flow_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        machine.cpu.stage_next_sequential_pc_for_step();
        machine.cpu.commit_staged_step_control_flow(snapshot);

        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
        assert_eq!(
            cpu_non_control_flow_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
    }

    #[test]
    fn committed_step_count_advance_preserves_non_cop0_count_machine_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_1004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        machine.cpu.advance_count_for_committed_step();

        assert_eq!(machine.cpu().cop0_count(), 1);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
    }

    #[test]
    fn cpu_local_committed_success_cadence_commits_control_flow_then_advances_count_once() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = committed_cpu_local_success_action_plan(
            CpuInstructionIdentity::SpecialSync,
            CpuLocalExecutedHelperFamily::NoEffectSync,
        );
        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let machine_before = (
            machine.powered_on(),
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );
        let non_cadence_cpu_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let applied = machine
            .apply_cpu_local_committed_success_cadence(action_plan, snapshot)
            .expect("successful CPU-local action should apply committed cadence");

        assert_eq!(
            applied.executed().identity(),
            CpuInstructionIdentity::SpecialSync
        );
        assert_eq!(
            applied.executed().family(),
            CpuLocalExecutedHelperFamily::NoEffectSync
        );
        assert_eq!(
            applied.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(
            machine_before,
            (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
        assert_eq!(
            non_cadence_cpu_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
    }

    #[test]
    fn cpu_local_committed_success_cadence_uses_existing_count_timer_latch() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = committed_cpu_local_success_action_plan(
            CpuInstructionIdentity::Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );

        machine
            .apply_cpu_local_committed_success_cadence(action_plan, snapshot)
            .expect("successful CPU-local action should apply committed cadence");

        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0011);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
    }

    #[test]
    fn cpu_local_committed_success_cadence_rejects_overflow_and_rejection_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0xaaaa_bbbb_cccc_dddd);
        machine.cpu.stage_lo(0x1111_2222_3333_4444);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();

        let scalar_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        for action_plan in [
            special_trapping_overflow_action_plan(),
            immediate_trapping_overflow_action_plan(),
            classify_cpu_local_invocation_step_action(Err(
                CpuLocalExecutedHelperInvocationError::from(
                    machine.cpu.set_gpr(CPU_GPR_COUNT, 0).unwrap_err(),
                ),
            )),
        ] {
            let error = machine
                .apply_cpu_local_committed_success_cadence(action_plan, snapshot)
                .expect_err("non-success action should not apply committed cadence");
            assert_eq!(
                error,
                MachineCpuLocalCommittedSuccessCadenceError::NonSuccessAction(action_plan)
            );
            assert_eq!(
                scalar_before,
                (
                    machine.cpu().pc(),
                    machine.cpu().next_pc(),
                    machine.cpu().hi(),
                    machine.cpu().lo(),
                    machine.cpu().gpr(0),
                    machine.cpu().gpr(8),
                )
            );
            assert_eq!(
                cop0_before,
                (
                    machine.cpu().cop0_count(),
                    machine.cpu().cop0_compare(),
                    machine.cpu().cop0_timer_interrupt_pending(),
                    machine.cpu().cop0_status(),
                    machine.cpu().cop0_software_interrupt_pending(),
                    machine.cpu().cop0_epc(),
                    machine.cpu().cop0_bad_vaddr(),
                    machine.cpu().cop0_exception_code(),
                    machine.cpu().cop0_exception_branch_delay(),
                )
            );
        }
    }

    #[test]
    fn cpu_local_arithmetic_overflow_exception_application_enters_ordinary_exception_state() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_1004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0020, 0x0000_0021, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = special_trapping_overflow_action_plan();
        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let non_exception_cpu_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_software_interrupt_pending(),
        );

        let applied = machine
            .apply_cpu_local_arithmetic_overflow_exception(action_plan, snapshot)
            .expect("overflow action should enter arithmetic-overflow exception state");

        assert_eq!(
            applied.overflow().identity(),
            CpuInstructionIdentity::SpecialAdd
        );
        assert_eq!(
            applied.overflow().family(),
            CpuLocalExecutedHelperFamily::SpecialTrappingInteger
        );
        assert_arithmetic_overflow_exception_entry(
            &machine,
            0x8000_1000,
            false,
            0x1357_9bdf,
            0x0000_0020,
        );
        assert_ne!(machine.cpu().pc(), 0x8000_1004);
        assert_ne!(machine.cpu().next_pc(), 0x8000_1008);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(
            non_exception_cpu_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_software_interrupt_pending(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
    }

    #[test]
    fn cpu_local_arithmetic_overflow_exception_application_preserves_delay_slot_semantics() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1004);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_delay_slot_context_for_test(0x8000_1000);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0040, 0x0000_0041, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = immediate_trapping_overflow_action_plan();

        let applied = machine
            .apply_cpu_local_arithmetic_overflow_exception(action_plan, snapshot)
            .expect("delay-slot overflow action should enter arithmetic-overflow exception state");

        assert_eq!(applied.overflow().identity(), CpuInstructionIdentity::Addi);
        assert_eq!(
            applied.overflow().family(),
            CpuLocalExecutedHelperFamily::ImmediateTrappingInteger
        );
        assert_arithmetic_overflow_exception_entry(
            &machine,
            0x8000_1000,
            true,
            0x2468_ace0,
            0x0000_0040,
        );
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
    }

    #[test]
    fn cpu_local_arithmetic_overflow_exception_application_rejects_non_overflow_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0xaaaa_bbbb_cccc_dddd);
        machine.cpu.stage_lo(0x1111_2222_3333_4444);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);
        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();

        let scalar_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        for action_plan in [
            committed_cpu_local_success_action_plan(
                CpuInstructionIdentity::SpecialSync,
                CpuLocalExecutedHelperFamily::NoEffectSync,
            ),
            classify_cpu_local_invocation_step_action(Err(
                CpuLocalExecutedHelperInvocationError::from(
                    machine.cpu.set_gpr(CPU_GPR_COUNT, 0).unwrap_err(),
                ),
            )),
        ] {
            let error = machine
                .apply_cpu_local_arithmetic_overflow_exception(action_plan, snapshot)
                .expect_err("non-overflow action should not apply overflow exception");
            assert_eq!(
                error,
                MachineCpuLocalArithmeticOverflowExceptionError::NonOverflowAction(action_plan)
            );
            assert_eq!(
                scalar_before,
                (
                    machine.cpu().pc(),
                    machine.cpu().next_pc(),
                    machine.cpu().hi(),
                    machine.cpu().lo(),
                    machine.cpu().gpr(0),
                    machine.cpu().gpr(8),
                )
            );
            assert_eq!(
                cop0_before,
                (
                    machine.cpu().cop0_count(),
                    machine.cpu().cop0_compare(),
                    machine.cpu().cop0_timer_interrupt_pending(),
                    machine.cpu().cop0_status(),
                    machine.cpu().cop0_software_interrupt_pending(),
                    machine.cpu().cop0_epc(),
                    machine.cpu().cop0_bad_vaddr(),
                    machine.cpu().cop0_exception_code(),
                    machine.cpu().cop0_exception_branch_delay(),
                )
            );
        }
    }

    #[test]
    fn cpu_local_step_action_application_delegates_success_to_committed_cadence() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = committed_cpu_local_success_action_plan(
            CpuInstructionIdentity::Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );
        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let non_cadence_cpu_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let applied = machine
            .apply_cpu_local_step_action(action_plan, snapshot)
            .expect("successful CPU-local action should apply committed cadence");

        let committed = applied
            .committed_success()
            .expect("success action should return committed success application");
        assert_eq!(applied.arithmetic_overflow_exception(), None);
        assert_eq!(committed.executed().identity(), CpuInstructionIdentity::Lui);
        assert_eq!(
            committed.executed().family(),
            CpuLocalExecutedHelperFamily::UpperImmediateLui
        );
        assert_eq!(
            committed.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0011);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(
            non_cadence_cpu_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
    }

    #[test]
    fn cpu_local_step_action_application_delegates_overflow_to_exception_entry() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1004);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_delay_slot_context_for_test(0x8000_1000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = immediate_trapping_overflow_action_plan();

        let applied = machine
            .apply_cpu_local_step_action(action_plan, snapshot)
            .expect("overflow CPU-local action should apply arithmetic-overflow exception entry");

        let exception = applied
            .arithmetic_overflow_exception()
            .expect("overflow action should return arithmetic-overflow application");
        assert_eq!(applied.committed_success(), None);
        assert_eq!(
            exception.overflow().identity(),
            CpuInstructionIdentity::Addi
        );
        assert_eq!(
            exception.overflow().family(),
            CpuLocalExecutedHelperFamily::ImmediateTrappingInteger
        );
        assert_arithmetic_overflow_exception_entry(
            &machine,
            0x8000_1000,
            true,
            0x2468_ace0,
            0x0000_0010,
        );
        assert_ne!(machine.cpu().pc(), 0x8000_2000);
        assert_ne!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn cpu_local_step_action_application_rejects_invocation_error_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0xaaaa_bbbb_cccc_dddd);
        machine.cpu.stage_lo(0x1111_2222_3333_4444);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);
        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();

        let scalar_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let invocation_error = CpuLocalExecutedHelperInvocationError::from(
            machine.cpu.set_gpr(CPU_GPR_COUNT, 0).unwrap_err(),
        );
        let action_plan = classify_cpu_local_invocation_step_action(Err(invocation_error));

        let error = machine
            .apply_cpu_local_step_action(action_plan, snapshot)
            .expect_err("invocation rejection should not apply a CPU-local step action");

        assert_eq!(
            error,
            MachineCpuLocalStepActionApplicationError::RejectedInvocation(invocation_error)
        );
        assert_eq!(
            scalar_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
    }

    #[test]
    fn non_cpu_local_frontier_no_effect_applies_committed_cadence() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let instruction =
            classify_step_no_effect_executed_instruction(instruction_fields(0x0000_000f))
                .expect("SYNC should classify as no-effect executed");
        let action = MachineNonCpuLocalStepFrontierAction::NoEffectExecuted(instruction);
        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let non_cadence_cpu_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let applied = machine
            .apply_non_cpu_local_step_frontier_action(action, snapshot)
            .expect("no-effect executed frontier action should apply committed cadence");

        assert_eq!(applied.no_effect_executed_instruction(), Some(instruction));
        assert_eq!(applied.stopped_instruction(), None);
        assert_eq!(applied.unsupported_instruction(), None);
        assert_eq!(applied.fetch_address_error_plan(), None);
        assert_eq!(
            applied.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0011);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(
            non_cadence_cpu_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
    }

    #[test]
    fn non_cpu_local_frontier_stopped_applies_stopped_cadence_without_exception() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0xaaaa_bbbb_cccc_dddd);
        machine.cpu.stage_lo(0x1111_2222_3333_4444);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0020, 0x0000_0022, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let instruction = classify_step_stopped_instruction(instruction_fields(0x0000_000c))
            .expect("SYSCALL should classify as stopped");
        let action = MachineNonCpuLocalStepFrontierAction::Stopped(instruction);
        let non_cadence_cpu_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let applied = machine
            .apply_non_cpu_local_step_frontier_action(action, snapshot)
            .expect("stopped frontier action should apply stopped cadence");

        assert_eq!(applied.no_effect_executed_instruction(), None);
        assert_eq!(applied.stopped_instruction(), Some(instruction));
        assert_eq!(applied.unsupported_instruction(), None);
        assert_eq!(applied.fetch_address_error_plan(), None);
        assert_eq!(
            applied.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::StoppedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0021);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0022);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(
            non_cadence_cpu_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
    }

    #[test]
    fn non_cpu_local_frontier_unsupported_restores_snapshot_without_count() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0xaaaa_bbbb_cccc_dddd);
        machine.cpu.stage_lo(0x1111_2222_3333_4444);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let instruction = classify_step_unsupported_instruction(instruction_fields(0x7000_1234))
            .expect("unknown primary should classify as unsupported");
        let action = MachineNonCpuLocalStepFrontierAction::Unsupported(instruction);
        let non_control_flow_scalar_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let applied = machine
            .apply_non_cpu_local_step_frontier_action(action, snapshot)
            .expect("unsupported frontier action should restore snapshot");

        assert_eq!(applied.no_effect_executed_instruction(), None);
        assert_eq!(applied.stopped_instruction(), None);
        assert_eq!(applied.unsupported_instruction(), Some(instruction));
        assert_eq!(applied.fetch_address_error_plan(), None);
        assert_eq!(
            applied.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::UnsupportedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_1000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2000);
        assert_eq!(
            non_control_flow_scalar_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
    }

    #[test]
    fn non_cpu_local_frontier_fetch_fault_enters_adel_without_count_or_commit() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0xa400_0042);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let snapshot = machine.cpu.capture_control_flow();
        let fetch_error = MachineCpuInstructionFetchError::Unaligned {
            cpu_address: CpuAddress::new(0xa400_0042),
        };
        let fetch_action = classify_step_fetch_fault_action(fetch_error);
        let action = MachineNonCpuLocalStepFrontierAction::FetchFault(fetch_action);
        let non_exception_cpu_before = (
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_software_interrupt_pending(),
        );

        let applied = machine
            .apply_non_cpu_local_step_frontier_action(action, snapshot)
            .expect("selected fetch-fault frontier action should enter fetch AdEL");

        let plan = fetch_action
            .address_error_plan()
            .expect("selected fetch action should expose AdEL plan");
        assert_eq!(applied.no_effect_executed_instruction(), None);
        assert_eq!(applied.stopped_instruction(), None);
        assert_eq!(applied.unsupported_instruction(), None);
        assert_eq!(applied.fetch_address_error_plan(), Some(plan));
        assert_eq!(
            applied.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::FetchAddressErrorException)
        );
        assert_entered_instruction_fetch_address_error(
            &machine,
            CpuAddress::new(0xa400_0042),
            CpuAddress::new(0xa400_0042),
            0x0000_0010,
        );
        assert_ne!(machine.cpu().pc(), snapshot.next_pc());
        assert_ne!(machine.cpu().next_pc(), snapshot.next_pc().wrapping_add(4));
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(
            non_exception_cpu_before,
            (
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_software_interrupt_pending(),
            )
        );
    }

    #[test]
    fn non_cpu_local_frontier_fetch_fault_rejections_do_not_mutate() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0xaaaa_bbbb_cccc_dddd);
        machine.cpu.stage_lo(0x1111_2222_3333_4444);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);
        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();

        let scalar_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let fetch_error = MachineCpuInstructionFetchError::NonDirectUnsupported {
            cpu_address: CpuAddress::new(0x0000_0000),
        };
        let action = MachineNonCpuLocalStepFrontierAction::FetchFault(
            classify_step_fetch_fault_action(fetch_error),
        );

        let error = machine
            .apply_non_cpu_local_step_frontier_action(action, snapshot)
            .expect_err("non-converting fetch fault should not apply a frontier action");

        assert_eq!(
            error,
            MachineNonCpuLocalStepFrontierApplicationError::FetchFaultRethrow(fetch_error)
        );
        assert_eq!(
            scalar_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );

        let selected_fetch_error = MachineCpuInstructionFetchError::DirectTargetMiss {
            cpu_address: CpuAddress::new(0x8500_0000),
        };
        let selected_action = MachineNonCpuLocalStepFrontierAction::FetchFault(
            classify_step_fetch_fault_action(selected_fetch_error),
        );
        let blocked = machine
            .apply_non_cpu_local_step_frontier_action(selected_action, snapshot)
            .expect_err("selected fetch fault should reject step-coupled context");
        assert_eq!(
            blocked,
            MachineNonCpuLocalStepFrontierApplicationError::FetchAddressErrorEntry(
                CpuAddressErrorExceptionEntryError::new(
                    CpuAddress::new(0x8000_1000),
                    CpuAddress::new(0x8000_2004),
                    0,
                )
            )
        );
        assert_eq!(
            scalar_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
    }

    #[test]
    fn classified_step_action_delegates_cpu_local_success_application() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = committed_cpu_local_success_action_plan(
            CpuInstructionIdentity::Lui,
            CpuLocalExecutedHelperFamily::UpperImmediateLui,
        );

        let applied = machine
            .apply_classified_step_action(
                MachineClassifiedStepAction::CpuLocal(action_plan),
                snapshot,
            )
            .expect("classified CPU-local success should delegate to CPU-local applicator");

        let cpu_local = applied
            .cpu_local()
            .expect("classified CPU-local action should return CPU-local application");
        assert_eq!(applied.non_cpu_local_frontier(), None);
        let committed = cpu_local
            .committed_success()
            .expect("CPU-local success should commit cadence");
        assert_eq!(
            committed.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction)
        );
        assert_eq!(committed.executed().identity(), CpuInstructionIdentity::Lui);
        assert_eq!(
            committed.executed().family(),
            CpuLocalExecutedHelperFamily::UpperImmediateLui
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0011);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1357_9bdf);
    }

    #[test]
    fn classified_step_action_delegates_cpu_local_overflow_application() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1004);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_delay_slot_context_for_test(0x8000_1000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let action_plan = immediate_trapping_overflow_action_plan();

        let applied = machine
            .apply_classified_step_action(
                MachineClassifiedStepAction::CpuLocal(action_plan),
                snapshot,
            )
            .expect("classified CPU-local overflow should delegate to overflow applicator");

        let cpu_local = applied
            .cpu_local()
            .expect("classified CPU-local action should return CPU-local application");
        assert_eq!(applied.non_cpu_local_frontier(), None);
        let exception = cpu_local
            .arithmetic_overflow_exception()
            .expect("CPU-local overflow should enter arithmetic-overflow exception");
        assert_eq!(
            exception.overflow().identity(),
            CpuInstructionIdentity::Addi
        );
        assert_eq!(
            exception.overflow().family(),
            CpuLocalExecutedHelperFamily::ImmediateTrappingInteger
        );
        assert_arithmetic_overflow_exception_entry(
            &machine,
            0x8000_1000,
            true,
            0x2468_ace0,
            0x0000_0010,
        );
        assert_ne!(machine.cpu().pc(), 0x8000_2000);
        assert_ne!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
    }

    #[test]
    fn classified_step_action_delegates_no_effect_frontier_application() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let instruction =
            classify_step_no_effect_executed_instruction(instruction_fields(0x0000_000f))
                .expect("SYNC should classify as no-effect executed");
        let action = MachineClassifiedStepAction::NonCpuLocalFrontier(
            MachineNonCpuLocalStepFrontierAction::NoEffectExecuted(instruction),
        );

        let applied = machine
            .apply_classified_step_action(action, snapshot)
            .expect("classified no-effect action should delegate to frontier applicator");

        let frontier = applied
            .non_cpu_local_frontier()
            .expect("classified frontier action should return frontier application");
        assert_eq!(applied.cpu_local(), None);
        assert_eq!(frontier.no_effect_executed_instruction(), Some(instruction));
        assert_eq!(
            frontier.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0011);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1357_9bdf);
    }

    #[test]
    fn classified_step_action_delegates_stopped_frontier_application() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0020, 0x0000_0022, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let instruction = classify_step_stopped_instruction(instruction_fields(0x0000_000d))
            .expect("BREAK should classify as stopped");
        let action = MachineClassifiedStepAction::NonCpuLocalFrontier(
            MachineNonCpuLocalStepFrontierAction::Stopped(instruction),
        );

        let applied = machine
            .apply_classified_step_action(action, snapshot)
            .expect("classified stopped action should delegate to frontier applicator");

        let frontier = applied
            .non_cpu_local_frontier()
            .expect("classified frontier action should return frontier application");
        assert_eq!(applied.cpu_local(), None);
        assert_eq!(frontier.stopped_instruction(), Some(instruction));
        assert_eq!(
            frontier.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::StoppedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0021);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0022);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x2468_ace0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn classified_step_action_delegates_unsupported_frontier_application() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let instruction = classify_step_unsupported_instruction(instruction_fields(0x7000_1234))
            .expect("unknown primary should classify as unsupported");
        let action = MachineClassifiedStepAction::NonCpuLocalFrontier(
            MachineNonCpuLocalStepFrontierAction::Unsupported(instruction),
        );

        let applied = machine
            .apply_classified_step_action(action, snapshot)
            .expect("classified unsupported action should delegate to frontier applicator");

        let frontier = applied
            .non_cpu_local_frontier()
            .expect("classified frontier action should return frontier application");
        assert_eq!(applied.cpu_local(), None);
        assert_eq!(frontier.unsupported_instruction(), Some(instruction));
        assert_eq!(
            frontier.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::UnsupportedInstruction)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_1000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2000);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1020_3040);
    }

    #[test]
    fn classified_step_action_delegates_fetch_fault_frontier_application() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0xa400_0042);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let snapshot = machine.cpu.capture_control_flow();
        let fetch_error = MachineCpuInstructionFetchError::Unaligned {
            cpu_address: CpuAddress::new(0xa400_0042),
        };
        let fetch_action = classify_step_fetch_fault_action(fetch_error);
        let action = MachineClassifiedStepAction::NonCpuLocalFrontier(
            MachineNonCpuLocalStepFrontierAction::FetchFault(fetch_action),
        );

        let applied = machine
            .apply_classified_step_action(action, snapshot)
            .expect("classified selected fetch fault should delegate to frontier applicator");

        let plan = fetch_action
            .address_error_plan()
            .expect("selected fetch fault should carry an AdEL plan");
        let frontier = applied
            .non_cpu_local_frontier()
            .expect("classified frontier action should return frontier application");
        assert_eq!(applied.cpu_local(), None);
        assert_eq!(frontier.fetch_address_error_plan(), Some(plan));
        assert_eq!(
            frontier.cadence_plan(),
            classify_machine_step_cadence(MachineStepCadenceSource::FetchAddressErrorException)
        );
        assert_entered_instruction_fetch_address_error(
            &machine,
            CpuAddress::new(0xa400_0042),
            CpuAddress::new(0xa400_0042),
            0x0000_0010,
        );
        assert_ne!(machine.cpu().pc(), snapshot.next_pc());
        assert_ne!(machine.cpu().next_pc(), snapshot.next_pc().wrapping_add(4));
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
    }

    #[test]
    fn classified_step_action_rejects_delegated_errors_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8000_1000);
        machine.cpu.stage_next_pc(0x8000_2000);
        machine.cpu.stage_hi(0xaaaa_bbbb_cccc_dddd);
        machine.cpu.stage_lo(0x1111_2222_3333_4444);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let snapshot = machine.cpu.capture_control_flow();
        machine.cpu.stage_next_sequential_pc_for_step();
        let scalar_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(0),
            machine.cpu().gpr(8),
        );
        let cop0_before = (
            machine.cpu().cop0_count(),
            machine.cpu().cop0_compare(),
            machine.cpu().cop0_timer_interrupt_pending(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_software_interrupt_pending(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );

        let invocation_error = CpuLocalExecutedHelperInvocationError::from(
            machine.cpu.set_gpr(CPU_GPR_COUNT, 0).unwrap_err(),
        );
        let cpu_local_action = MachineClassifiedStepAction::CpuLocal(
            classify_cpu_local_invocation_step_action(Err(invocation_error)),
        );
        let cpu_local_error = machine
            .apply_classified_step_action(cpu_local_action, snapshot)
            .expect_err("invocation rejection should be delegated as rejection");
        assert_eq!(
            cpu_local_error,
            MachineClassifiedStepActionApplicationError::CpuLocal(
                MachineCpuLocalStepActionApplicationError::RejectedInvocation(invocation_error)
            )
        );
        assert_eq!(
            scalar_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );

        let fetch_error = MachineCpuInstructionFetchError::NonDirectUnsupported {
            cpu_address: CpuAddress::new(0x0000_0000),
        };
        let fetch_action = MachineClassifiedStepAction::NonCpuLocalFrontier(
            MachineNonCpuLocalStepFrontierAction::FetchFault(classify_step_fetch_fault_action(
                fetch_error,
            )),
        );
        let fetch_rejection = machine
            .apply_classified_step_action(fetch_action, snapshot)
            .expect_err("non-converting fetch fault should be delegated as rejection");
        assert_eq!(
            fetch_rejection,
            MachineClassifiedStepActionApplicationError::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierApplicationError::FetchFaultRethrow(fetch_error)
            )
        );
        assert_eq!(
            scalar_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            )
        );
        assert_eq!(
            cop0_before,
            (
                machine.cpu().cop0_count(),
                machine.cpu().cop0_compare(),
                machine.cpu().cop0_timer_interrupt_pending(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_software_interrupt_pending(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
    }

    #[test]
    fn current_pc_classified_step_action_captures_and_stages_before_no_effect_action() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x0000_000f)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let produced = machine
            .produce_current_pc_classified_step_action()
            .expect("SYNC should produce no-effect classified action");

        assert_eq!(produced.control_flow_snapshot().pc(), 0x8000_0000);
        assert_eq!(produced.control_flow_snapshot().next_pc(), 0x8000_2000);
        match produced.action() {
            MachineClassifiedStepAction::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierAction::NoEffectExecuted(instruction),
            ) => {
                assert_eq!(instruction.identity(), CpuInstructionIdentity::SpecialSync);
                assert_eq!(instruction.raw().bits(), 0x0000_000f);
            }
            other => panic!("expected no-effect frontier action, got {other:?}"),
        }
        assert_eq!(machine.cpu().pc(), 0x8000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1357_9bdf);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    #[test]
    fn current_pc_classified_step_action_fetch_fault_produces_selected_action_without_entry() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0xa400_0042);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let produced = machine
            .produce_current_pc_classified_step_action()
            .expect("unaligned current PC should produce selected fetch-fault action");

        assert_eq!(produced.control_flow_snapshot().pc(), 0xa400_0042);
        assert_eq!(produced.control_flow_snapshot().next_pc(), 0xa400_0046);
        match produced.action() {
            MachineClassifiedStepAction::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierAction::FetchFault(
                    MachineStepFetchFaultAction::EnterAddressError(plan),
                ),
            ) => {
                assert_eq!(plan.cpu_address(), CpuAddress::new(0xa400_0042));
                assert_eq!(plan.bad_vaddr(), CpuAddress::new(0xa400_0042));
                assert_eq!(plan.cause_exception_code(), 4);
            }
            other => panic!("expected selected fetch-fault action, got {other:?}"),
        }
        assert_eq!(machine.cpu().pc(), 0xa400_0042);
        assert_eq!(machine.cpu().next_pc(), 0xa400_0046);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1357_9bdf);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn current_pc_classified_step_action_fetch_rejection_restores_snapshot_without_count() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x4000_0000);
        machine.cpu.stage_next_pc(0x4000_0004);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let error = machine
            .produce_current_pc_classified_step_action()
            .expect_err("non-direct current PC should reject without action");

        let fetch_error = MachineCpuInstructionFetchError::NonDirectUnsupported {
            cpu_address: CpuAddress::new(0x4000_0000),
        };
        assert_eq!(
            error,
            MachineCurrentPcClassifiedStepActionError::FetchFaultRethrow(fetch_error)
        );
        assert_eq!(error.fetch_error(), Some(fetch_error));
        assert_eq!(machine.cpu().pc(), 0x4000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x4000_0004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1020_3040);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn current_pc_classified_step_action_stopped_actions_do_not_enter_exception() {
        for (raw, identity) in [
            (0x0000_000c, CpuInstructionIdentity::SpecialSyscall),
            (0x0000_000d, CpuInstructionIdentity::SpecialBreak),
        ] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            machine.write_rdram_u32_be(0x0000_0000, raw).unwrap();
            machine.cpu.stage_pc(0x8000_0000);
            machine.cpu.stage_next_pc(0x8000_2000);
            machine
                .cpu
                .stage_cop0_count_compare_timer_for_test(0x0000_0020, 0x0000_0022, false);
            machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

            let produced = machine
                .produce_current_pc_classified_step_action()
                .expect("stopped instruction should produce stopped classified action");

            assert_eq!(produced.control_flow_snapshot().pc(), 0x8000_0000);
            assert_eq!(produced.control_flow_snapshot().next_pc(), 0x8000_2000);
            match produced.action() {
                MachineClassifiedStepAction::NonCpuLocalFrontier(
                    MachineNonCpuLocalStepFrontierAction::Stopped(instruction),
                ) => {
                    assert_eq!(instruction.identity(), identity);
                    assert_eq!(instruction.raw().bits(), raw);
                }
                other => panic!("expected stopped frontier action, got {other:?}"),
            }
            assert_eq!(machine.cpu().pc(), 0x8000_0000);
            assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
            assert_eq!(machine.cpu().cop0_count(), 0x0000_0020);
            assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x2468_ace0);
            assert_eq!(machine.cpu().cop0_exception_code(), 0);
            assert!(!machine.cpu().cop0_exception_branch_delay());
        }
    }

    #[test]
    fn current_pc_classified_step_action_unsupported_identity_stages_without_invoking_helpers() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x7000_1234)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let produced = machine
            .produce_current_pc_classified_step_action()
            .expect("unknown primary should produce unsupported classified action");

        match produced.action() {
            MachineClassifiedStepAction::NonCpuLocalFrontier(
                MachineNonCpuLocalStepFrontierAction::Unsupported(instruction),
            ) => {
                assert_eq!(
                    instruction.identity(),
                    CpuInstructionIdentity::UnknownPrimary
                );
                assert_eq!(instruction.raw().bits(), 0x7000_1234);
            }
            other => panic!("expected unsupported frontier action, got {other:?}"),
        }
        assert_eq!(machine.cpu().pc(), 0x8000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1020_3040);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn current_pc_classified_step_action_cpu_local_success_writes_back_without_cadence() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x3c02_8000)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(2, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let produced = machine
            .produce_current_pc_classified_step_action()
            .expect("LUI should invoke sealed CPU-local helper and produce success action");

        assert_eq!(produced.control_flow_snapshot().pc(), 0x8000_0000);
        assert_eq!(produced.control_flow_snapshot().next_pc(), 0x8000_2000);
        match produced.action() {
            MachineClassifiedStepAction::CpuLocal(action_plan) => {
                assert_committed_local_step_action(
                    action_plan,
                    CpuInstructionIdentity::Lui,
                    CpuLocalExecutedHelperFamily::UpperImmediateLui,
                );
            }
            other => panic!("expected CPU-local action, got {other:?}"),
        }
        assert_eq!(machine.cpu().pc(), 0x8000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().gpr(2), Some(0xffff_ffff_8000_0000));
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1357_9bdf);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn current_pc_classified_step_action_cpu_local_overflow_produces_action_without_entry() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x2042_0001)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(2, 0x0000_0000_7fff_ffff), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

        let produced = machine
            .produce_current_pc_classified_step_action()
            .expect("ADDI overflow should produce CPU-local overflow action");

        match produced.action() {
            MachineClassifiedStepAction::CpuLocal(action_plan) => {
                assert_overflow_local_step_action(
                    action_plan,
                    CpuInstructionIdentity::Addi,
                    CpuLocalExecutedHelperFamily::ImmediateTrappingInteger,
                );
            }
            other => panic!("expected CPU-local overflow action, got {other:?}"),
        }
        assert_eq!(machine.cpu().pc(), 0x8000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().gpr(2), Some(0x0000_0000_7fff_ffff));
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x2468_ace0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
    }

    #[test]
    fn current_pc_classified_step_action_unrepresented_identity_rejects_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x1880_0001)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(4, 0x1357_9bdf), Ok(()));
        assert_eq!(machine.cpu.set_gpr(5, 0x1357_9bdf), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let error = machine
            .produce_current_pc_classified_step_action()
            .expect_err("unassigned branch identity should remain outside represented categories");

        assert_eq!(error.identity(), Some(CpuInstructionIdentity::Blez));
        assert_eq!(
            error.fields().map(|fields| fields.raw().bits()),
            Some(0x1880_0001)
        );
        assert_eq!(machine.cpu().pc(), 0x8000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2000);
        assert_eq!(machine.cpu().gpr(4), Some(0x1357_9bdf));
        assert_eq!(machine.cpu().gpr(5), Some(0x1357_9bdf));
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1020_3040);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn machine_step_cpu_local_success_composes_producer_and_applicator() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x3c02_8000)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(2, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let outcome = machine
            .step()
            .expect("represented LUI step should commit CPU-local success");

        assert_eq!(outcome.identity(), Some(CpuInstructionIdentity::Lui));
        assert_eq!(
            outcome.cadence_plan(),
            Some(classify_machine_step_cadence(
                MachineStepCadenceSource::CommittedInstruction
            ))
        );
        match outcome {
            MachineRepresentedStepOutcome::CpuLocalCommitted {
                identity,
                cadence_plan,
            } => {
                assert_eq!(identity, CpuInstructionIdentity::Lui);
                assert_eq!(
                    cadence_plan.control_flow_action(),
                    MachineStepControlFlowAction::CommitStaged
                );
                assert_eq!(cadence_plan.count_action(), MachineStepCountAction::Advance);
            }
            other => panic!("expected CPU-local committed outcome, got {other:?}"),
        }
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().gpr(2), Some(0xffff_ffff_8000_0000));
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0011);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1357_9bdf);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn machine_step_cpu_local_arithmetic_overflow_enters_exception_without_count() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x2042_0001)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_0004);
        assert_eq!(machine.cpu.set_gpr(2, 0x0000_0000_7fff_ffff), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

        let outcome = machine
            .step()
            .expect("represented ADDI overflow step should enter overflow exception");

        assert_eq!(outcome.identity(), Some(CpuInstructionIdentity::Addi));
        assert_eq!(outcome.cadence_plan(), None);
        match outcome {
            MachineRepresentedStepOutcome::ArithmeticOverflowException { identity } => {
                assert_eq!(identity, CpuInstructionIdentity::Addi);
            }
            other => panic!("expected arithmetic-overflow outcome, got {other:?}"),
        }
        assert_eq!(machine.cpu().gpr(2), Some(0x0000_0000_7fff_ffff));
        assert_arithmetic_overflow_exception_entry(
            &machine,
            0x8000_0000,
            false,
            0x2468_ace0,
            0x0000_0010,
        );
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
    }

    #[test]
    fn machine_step_sync_commits_no_effect_cadence_without_cpu_local_helper() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x0000_000f)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let outcome = machine
            .step()
            .expect("represented SYNC step should commit no-effect cadence");

        match outcome {
            MachineRepresentedStepOutcome::NoEffectCommitted {
                instruction,
                cadence_plan,
            } => {
                assert_eq!(instruction.identity(), CpuInstructionIdentity::SpecialSync);
                assert_eq!(
                    cadence_plan,
                    classify_machine_step_cadence(MachineStepCadenceSource::CommittedInstruction)
                );
            }
            other => panic!("expected no-effect committed outcome, got {other:?}"),
        }
        assert_eq!(
            outcome.no_effect_instruction().unwrap().raw().bits(),
            0x0000_000f
        );
        assert_eq!(machine.cpu().pc(), 0x8000_2000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0011);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1357_9bdf);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn machine_step_syscall_and_break_commit_stopped_cadence_without_exception() {
        for (raw, identity, category) in [
            (
                0x0000_000c,
                CpuInstructionIdentity::SpecialSyscall,
                MachineStepStoppedInstructionCategory::Syscall,
            ),
            (
                0x0000_000d,
                CpuInstructionIdentity::SpecialBreak,
                MachineStepStoppedInstructionCategory::Break,
            ),
        ] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            machine.write_rdram_u32_be(0x0000_0000, raw).unwrap();
            machine.cpu.stage_pc(0x8000_0000);
            machine.cpu.stage_next_pc(0x8000_2000);
            machine
                .cpu
                .stage_cop0_count_compare_timer_for_test(0x0000_0020, 0x0000_0022, false);
            machine.cpu.stage_cop0_bad_vaddr_for_test(0x2468_ace0);

            let outcome = machine
                .step()
                .expect("represented stopped step should commit stopped cadence");

            match outcome {
                MachineRepresentedStepOutcome::Stopped {
                    instruction,
                    cadence_plan,
                } => {
                    assert_eq!(instruction.identity(), identity);
                    assert_eq!(instruction.category(), category);
                    assert_eq!(
                        cadence_plan,
                        classify_machine_step_cadence(MachineStepCadenceSource::StoppedInstruction)
                    );
                }
                other => panic!("expected stopped outcome, got {other:?}"),
            }
            assert_eq!(outcome.stopped_instruction().unwrap().raw().bits(), raw);
            assert_eq!(machine.cpu().pc(), 0x8000_2000);
            assert_eq!(machine.cpu().next_pc(), 0x8000_2004);
            assert_eq!(machine.cpu().cop0_count(), 0x0000_0021);
            assert_eq!(machine.cpu().cop0_compare(), 0x0000_0022);
            assert!(!machine.cpu().cop0_timer_interrupt_pending());
            assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x2468_ace0);
            assert_eq!(machine.cpu().cop0_exception_code(), 0);
            assert!(!machine.cpu().cop0_exception_branch_delay());
        }
    }

    #[test]
    fn machine_step_unsupported_restores_snapshot_without_count() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0x0000_0000, 0x7000_1234)
            .unwrap();
        machine.cpu.stage_pc(0x8000_0000);
        machine.cpu.stage_next_pc(0x8000_2000);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let outcome = machine
            .step()
            .expect("represented unsupported step should return unsupported outcome");

        match outcome {
            MachineRepresentedStepOutcome::Unsupported {
                instruction,
                cadence_plan,
            } => {
                assert_eq!(
                    instruction.identity(),
                    CpuInstructionIdentity::UnknownPrimary
                );
                assert_eq!(
                    cadence_plan,
                    classify_machine_step_cadence(MachineStepCadenceSource::UnsupportedInstruction)
                );
            }
            other => panic!("expected unsupported outcome, got {other:?}"),
        }
        assert_eq!(
            outcome.unsupported_instruction().unwrap().raw().bits(),
            0x7000_1234
        );
        assert_eq!(machine.cpu().pc(), 0x8000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_2000);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1020_3040);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn machine_step_selected_fetch_fault_enters_adel_without_count() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0xa400_0042);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1357_9bdf);

        let outcome = machine
            .step()
            .expect("selected fetch fault should enter instruction-fetch AdEL");

        match outcome {
            MachineRepresentedStepOutcome::InstructionFetchAddressError { plan, cadence_plan } => {
                assert_eq!(plan.cpu_address(), CpuAddress::new(0xa400_0042));
                assert_eq!(plan.bad_vaddr(), CpuAddress::new(0xa400_0042));
                assert_eq!(plan.cause_exception_code(), 4);
                assert_eq!(
                    cadence_plan,
                    classify_machine_step_cadence(
                        MachineStepCadenceSource::FetchAddressErrorException
                    )
                );
            }
            other => panic!("expected fetch AdEL outcome, got {other:?}"),
        }
        assert_eq!(
            outcome.fetch_address_error_plan().unwrap().fetch_error(),
            MachineCpuInstructionFetchError::Unaligned {
                cpu_address: CpuAddress::new(0xa400_0042)
            }
        );
        assert_entered_instruction_fetch_address_error(
            &machine,
            CpuAddress::new(0xa400_0042),
            CpuAddress::new(0xa400_0042),
            0x0000_0010,
        );
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
    }

    #[test]
    fn machine_step_non_converting_fetch_rejection_restores_snapshot_without_count() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x4000_0000);
        machine.cpu.stage_next_pc(0x4000_0004);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0x0000_0010, 0x0000_0011, false);
        machine.cpu.stage_cop0_bad_vaddr_for_test(0x1020_3040);

        let error = machine
            .step()
            .expect_err("non-converting fetch rejection should return step error");

        let fetch_error = MachineCpuInstructionFetchError::NonDirectUnsupported {
            cpu_address: CpuAddress::new(0x4000_0000),
        };
        assert_eq!(
            error,
            MachineRepresentedStepError::FetchRejected(fetch_error)
        );
        assert_eq!(error.fetch_error(), Some(fetch_error));
        assert_eq!(machine.cpu().pc(), 0x4000_0000);
        assert_eq!(machine.cpu().next_pc(), 0x4000_0004);
        assert_eq!(machine.cpu().cop0_count(), 0x0000_0010);
        assert_eq!(machine.cpu().cop0_compare(), 0x0000_0011);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().gpr(8), Some(0x0123_4567_89ab_cdef));
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x1020_3040);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
    }

    #[test]
    fn instruction_fetch_fault_selection_performs_no_machine_mutation() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let mut machine =
            Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
        machine
            .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
            .unwrap();
        machine
            .sp_dmem
            .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
        machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
        machine.cpu.stage_pc(0xa400_0042);
        machine.cpu.stage_next_pc(0x8000_3004);
        machine.cpu.stage_hi(0x1111_2222_3333_4444);
        machine.cpu.stage_lo(0x5555_6666_7777_8888);
        assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

        let rdram_before = [
            machine.rdram().read_u8(0x0000_0040),
            machine.rdram().read_u8(0x0000_0041),
            machine.rdram().read_u8(0x0000_0042),
            machine.rdram().read_u8(0x0000_0043),
        ];
        let sp_bytes_before = [
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
            machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
        ];
        let reservation_before = (
            machine.cpu_rdram_reservation.is_valid(),
            machine.cpu_rdram_reservation.rdram_offset(),
            machine.cpu_rdram_reservation.width(),
        );
        let cpu_before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().hi(),
            machine.cpu().lo(),
            machine.cpu().gpr(8),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
        );
        let cartridge_before = (
            machine.cartridge().size_bytes(),
            machine.cartridge().metadata().entry_point,
        );

        let fetch_error = machine.fetch_current_cpu_instruction_word().unwrap_err();
        assert!(fetch_error.is_unaligned());

        let plan = select_cpu_instruction_fetch_address_error(fetch_error).unwrap();
        assert_instruction_fetch_address_error_plan(
            plan,
            fetch_error,
            MachineInstructionFetchAddressErrorSource::Unaligned,
            CpuAddress::new(0xa400_0042),
        );

        assert_eq!(
            rdram_before,
            [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ]
        );
        assert_eq!(
            sp_bytes_before,
            [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ]
        );
        assert_eq!(
            reservation_before,
            (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            )
        );
        assert_eq!(
            cpu_before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(8),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            )
        );
        assert_eq!(
            cartridge_before,
            (
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
            )
        );
    }

    #[test]
    fn instruction_fetch_address_error_entry_mutates_narrow_cop0_control_flow_state() {
        for (pc, expected_source) in [
            (
                0xa400_0042,
                MachineInstructionFetchAddressErrorSource::Unaligned,
            ),
            (
                0x8500_0000,
                MachineInstructionFetchAddressErrorSource::DirectTargetMiss,
            ),
            (
                NON_BOOT_RESET_VECTOR_PC,
                MachineInstructionFetchAddressErrorSource::PifResetUnavailable,
            ),
        ] {
            let normalized_bytes = make_synthetic_normalized_rom_proof_image();
            let mut machine =
                Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());
            machine
                .write_rdram_u32_be(0x0000_0040, 0x8cc5_0104)
                .unwrap();
            machine
                .sp_dmem
                .write_u32_be_for_test(SpDmemOffset::new(0x40), 0x3c01_1234);
            machine.cpu_rdram_reservation.stage(0x0000_0100, 8);
            machine.cpu.stage_pc(pc);
            machine.cpu.stage_hi(0x1111_2222_3333_4444);
            machine.cpu.stage_lo(0x5555_6666_7777_8888);
            assert_eq!(machine.cpu.set_gpr(8, 0x0123_4567_89ab_cdef), Ok(()));

            let rdram_before = [
                machine.rdram().read_u8(0x0000_0040),
                machine.rdram().read_u8(0x0000_0041),
                machine.rdram().read_u8(0x0000_0042),
                machine.rdram().read_u8(0x0000_0043),
            ];
            let sp_bytes_before = [
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
            ];
            let reservation_before = (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            );
            let count_before = machine.cpu().cop0_count();
            let machine_state_before = (
                machine.powered_on(),
                machine.cartridge().size_bytes(),
                machine.cartridge().metadata().entry_point,
                machine.cpu().hi(),
                machine.cpu().lo(),
                machine.cpu().gpr(0),
                machine.cpu().gpr(8),
            );

            let fetch_error = machine.fetch_current_cpu_instruction_word().unwrap_err();
            let plan = select_cpu_instruction_fetch_address_error(fetch_error).unwrap();
            assert_instruction_fetch_address_error_plan(
                plan,
                fetch_error,
                expected_source,
                CpuAddress::new(pc),
            );

            assert_eq!(
                machine.enter_instruction_fetch_address_error_exception(plan),
                Ok(())
            );

            assert_entered_instruction_fetch_address_error(
                &machine,
                CpuAddress::new(pc),
                CpuAddress::new(pc),
                count_before,
            );
            assert_eq!(
                machine_state_before,
                (
                    machine.powered_on(),
                    machine.cartridge().size_bytes(),
                    machine.cartridge().metadata().entry_point,
                    machine.cpu().hi(),
                    machine.cpu().lo(),
                    machine.cpu().gpr(0),
                    machine.cpu().gpr(8),
                )
            );
            assert_eq!(
                rdram_before,
                [
                    machine.rdram().read_u8(0x0000_0040),
                    machine.rdram().read_u8(0x0000_0041),
                    machine.rdram().read_u8(0x0000_0042),
                    machine.rdram().read_u8(0x0000_0043),
                ]
            );
            assert_eq!(
                sp_bytes_before,
                [
                    machine.sp_dmem().read_u8(SpDmemOffset::new(0x40)),
                    machine.sp_dmem().read_u8(SpDmemOffset::new(0x41)),
                    machine.sp_dmem().read_u8(SpDmemOffset::new(0x42)),
                    machine.sp_dmem().read_u8(SpDmemOffset::new(0x43)),
                ]
            );
            assert_eq!(
                reservation_before,
                (
                    machine.cpu_rdram_reservation.is_valid(),
                    machine.cpu_rdram_reservation.rdram_offset(),
                    machine.cpu_rdram_reservation.width(),
                )
            );
        }
    }

    #[test]
    fn instruction_fetch_address_error_entry_blocks_step_coupled_contexts_without_mutation() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(0x8500_0000);
        machine.cpu.stage_next_pc(0x8000_1000);

        let fetch_error = machine.fetch_current_cpu_instruction_word().unwrap_err();
        let plan = select_cpu_instruction_fetch_address_error(fetch_error).unwrap();
        let before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu().cop0_status(),
            machine.cpu().cop0_epc(),
            machine.cpu().cop0_bad_vaddr(),
            machine.cpu().cop0_exception_code(),
            machine.cpu().cop0_exception_branch_delay(),
            machine.cpu().cop0_count(),
        );

        let blocked = machine
            .enter_instruction_fetch_address_error_exception(plan)
            .unwrap_err();
        assert_eq!(blocked.pc(), CpuAddress::new(0x8500_0000));
        assert_eq!(blocked.next_pc(), CpuAddress::new(0x8000_1000));
        assert_eq!(blocked.status(), 0);
        assert_eq!(
            before,
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
                machine.cpu().cop0_count(),
            )
        );

        let mut exl_machine = Machine::from_cartridge(Cartridge::default());
        exl_machine.cpu.stage_pc(NON_BOOT_RESET_VECTOR_PC);
        let first_error = exl_machine
            .fetch_current_cpu_instruction_word()
            .unwrap_err();
        let first_plan = select_cpu_instruction_fetch_address_error(first_error).unwrap();
        assert_eq!(
            exl_machine.enter_instruction_fetch_address_error_exception(first_plan),
            Ok(())
        );

        let second_plan = select_cpu_instruction_fetch_address_error(
            MachineCpuInstructionFetchError::DirectTargetMiss {
                cpu_address: CpuAddress::new(0x8500_0000),
            },
        )
        .unwrap();
        let before = (
            exl_machine.cpu().pc(),
            exl_machine.cpu().next_pc(),
            exl_machine.cpu().cop0_status(),
            exl_machine.cpu().cop0_epc(),
            exl_machine.cpu().cop0_bad_vaddr(),
            exl_machine.cpu().cop0_exception_code(),
            exl_machine.cpu().cop0_exception_branch_delay(),
            exl_machine.cpu().cop0_count(),
        );

        let blocked = exl_machine
            .enter_instruction_fetch_address_error_exception(second_plan)
            .unwrap_err();
        assert_eq!(blocked.pc(), CpuAddress::new(LOCAL_EXCEPTION_VECTOR_PC));
        assert_eq!(
            blocked.next_pc(),
            CpuAddress::new(LOCAL_EXCEPTION_VECTOR_NEXT_PC)
        );
        assert_eq!(blocked.status() & COP0_STATUS_EXL, COP0_STATUS_EXL);
        assert_eq!(
            before,
            (
                exl_machine.cpu().pc(),
                exl_machine.cpu().next_pc(),
                exl_machine.cpu().cop0_status(),
                exl_machine.cpu().cop0_epc(),
                exl_machine.cpu().cop0_bad_vaddr(),
                exl_machine.cpu().cop0_exception_code(),
                exl_machine.cpu().cop0_exception_branch_delay(),
                exl_machine.cpu().cop0_count(),
            )
        );
    }

    #[test]
    fn instruction_fetch_apis_still_return_errors_without_entering_exceptions() {
        for address in [0xa400_0042, 0x8500_0000, NON_BOOT_RESET_VECTOR_PC] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            machine.cpu.stage_pc(address);
            let before = (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            );

            assert!(machine.fetch_current_cpu_instruction_word().is_err());
            assert!(machine
                .fetch_cpu_instruction_word_at(CpuAddress::new(address))
                .is_err());

            assert_eq!(
                before,
                (
                    machine.cpu().pc(),
                    machine.cpu().next_pc(),
                    machine.cpu().cop0_status(),
                    machine.cpu().cop0_epc(),
                    machine.cpu().cop0_bad_vaddr(),
                    machine.cpu().cop0_exception_code(),
                    machine.cpu().cop0_exception_branch_delay(),
                )
            );
        }
    }

    #[test]
    fn rdram_byte_read_does_not_change_machine_owned_facts() {
        let normalized_bytes = make_synthetic_normalized_rom_proof_image();
        let machine = Machine::from_cartridge(load_cartridge(normalized_bytes.clone()).unwrap());

        assert_eq!(machine.rdram().read_u8(0), Ok(0));
        assert_eq!(machine.rdram().read_u8(RDRAM_SIZE_BYTES - 1), Ok(0));

        assert!(machine.powered_on());
        assert_eq!(machine.cartridge().size_bytes(), normalized_bytes.len());
        assert_eq!(machine.cartridge().metadata().entry_point, 0x8024_6000);
        assert_eq!(
            machine.cartridge().read_u8(0x40).unwrap(),
            normalized_bytes[0x40]
        );
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
        assert_eq!(machine.cpu().hi(), 0);
        assert_eq!(machine.cpu().lo(), 0);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(8), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert!(!machine.cpu().cop0_timer_interrupt_pending());
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu().cop0_software_interrupt_pending(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
    }

    const fn lw_word(base: u8, rt: u8, immediate: u16) -> u32 {
        (0x23 << 26) | ((base as u32) << 21) | ((rt as u32) << 16) | immediate as u32
    }

    const fn special_add_word(rs: u8, rt: u8, rd: u8) -> u32 {
        ((rs as u32) << 21) | ((rt as u32) << 16) | ((rd as u32) << 11) | 0x20
    }

    fn make_generated_lw_bootstrap_cartridge(first: u32, second: u32) -> Vec<u8> {
        let mut bytes = vec![0; 0x1000];
        write_be_u32(&mut bytes, 0x00, 0x8037_1240);
        write_be_u32(&mut bytes, 0x04, 0x0102_0304);
        write_be_u32(&mut bytes, 0x08, 0x8000_1000);
        write_be_u32(&mut bytes, 0x0c, 0x0506_0708);
        write_be_u32(&mut bytes, 0x10, 0x1112_1314);
        write_be_u32(&mut bytes, 0x14, 0x1516_1718);
        bytes[0x20..0x31].copy_from_slice(b"FN64 LW GENERATED");
        bytes[0x3c] = b'L';
        bytes[0x3d] = b'W';
        bytes[0x3e] = 0x45;
        bytes[0x3f] = 1;
        write_be_u32(&mut bytes, 0x40, first);
        write_be_u32(&mut bytes, 0x44, second);
        bytes
    }

    fn staged_lw_bootstrap_machine(first: u32, second: u32) -> Machine {
        let cartridge = load_cartridge(make_generated_lw_bootstrap_cartridge(first, second))
            .expect("generated cartridge should normalize");
        let mut machine = Machine::from_cartridge(cartridge);
        machine
            .stage_cartridge_bootstrap()
            .expect("generated cartridge should stage");
        machine
    }

    #[derive(Debug, PartialEq, Eq)]
    struct MachineLwSnapshot {
        cartridge: Vec<u8>,
        pif_firmware_state: MachinePifFirmwareState,
        pif_firmware_bytes: Option<Vec<u8>>,
        pc: u32,
        next_pc: u32,
        gprs: [u64; CPU_GPR_COUNT],
        hi: u64,
        lo: u64,
        count: u32,
        compare: u32,
        timer_interrupt_pending: bool,
        status: u32,
        software_interrupt_pending: u32,
        epc: u32,
        bad_vaddr: u32,
        exception_code: u8,
        exception_branch_delay: bool,
        rdram: Vec<u8>,
        sp_dmem: Vec<u8>,
        sp_imem: Vec<crate::sp_imem::SpImemByteObservation>,
        bootstrap: Option<MachineCartridgeBootstrapState>,
        reservation: (bool, u32, usize),
        powered_on: bool,
    }

    fn lw_snapshot(machine: &Machine) -> MachineLwSnapshot {
        MachineLwSnapshot {
            cartridge: (0..machine.cartridge().size_bytes())
                .map(|offset| machine.cartridge().read_u8(offset as u32).unwrap())
                .collect(),
            pif_firmware_state: machine.pif_firmware_state(),
            pif_firmware_bytes: machine.pif_firmware_bytes_for_test().map(<[u8]>::to_vec),
            pc: machine.cpu().pc(),
            next_pc: machine.cpu().next_pc(),
            gprs: core::array::from_fn(|index| machine.cpu().gpr(index).unwrap()),
            hi: machine.cpu().hi(),
            lo: machine.cpu().lo(),
            count: machine.cpu().cop0_count(),
            compare: machine.cpu().cop0_compare(),
            timer_interrupt_pending: machine.cpu().cop0_timer_interrupt_pending(),
            status: machine.cpu().cop0_status(),
            software_interrupt_pending: machine.cpu().cop0_software_interrupt_pending(),
            epc: machine.cpu().cop0_epc(),
            bad_vaddr: machine.cpu().cop0_bad_vaddr(),
            exception_code: machine.cpu().cop0_exception_code(),
            exception_branch_delay: machine.cpu().cop0_exception_branch_delay(),
            rdram: (0..machine.rdram().size_bytes())
                .map(|offset| machine.rdram().read_u8(offset).unwrap())
                .collect(),
            sp_dmem: (0..machine.sp_dmem().size_bytes())
                .map(|offset| {
                    machine
                        .sp_dmem()
                        .read_u8(SpDmemOffset::new(offset as u32))
                        .unwrap()
                })
                .collect(),
            sp_imem: (0..machine.sp_imem.size_bytes())
                .map(|offset| {
                    machine
                        .sp_imem
                        .observe_byte(SpImemOffset::new(offset as u32))
                        .unwrap()
                })
                .collect(),
            bootstrap: machine.cartridge_bootstrap_state(),
            reservation: (
                machine.cpu_rdram_reservation.is_valid(),
                machine.cpu_rdram_reservation.rdram_offset(),
                machine.cpu_rdram_reservation.width(),
            ),
            powered_on: machine.powered_on(),
        }
    }

    #[test]
    fn cpu_data_word_routing_is_narrow_for_direct_rdram_and_sp_imem() {
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0xa400_1000)),
            Ok(MachineLoadWordTarget::SpImem { offset: 0 })
        );
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0x8400_1000)),
            Ok(MachineLoadWordTarget::SpImem { offset: 0 })
        );
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0xa400_1ffc)),
            Ok(MachineLoadWordTarget::SpImem { offset: 0x0ffc })
        );
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0x8000_0100)),
            Ok(MachineLoadWordTarget::DirectRdram {
                offset: RdramOffset::new(0x100),
            })
        );
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0xa400_1001)),
            Err(MachineCpuDataWordTargetError::Unaligned {
                cpu_address: CpuAddress::new(0xa400_1001),
            })
        );
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0xa400_2000)),
            Err(MachineCpuDataWordTargetError::DirectTargetMiss {
                cpu_address: CpuAddress::new(0xa400_2000),
            })
        );
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0xa404_0000)),
            Err(MachineCpuDataWordTargetError::DirectTargetMiss {
                cpu_address: CpuAddress::new(0xa404_0000),
            })
        );
        assert_eq!(
            classify_cpu_data_word_target(CpuAddress::new(0x0400_1000)),
            Err(MachineCpuDataWordTargetError::NonDirectUnsupported {
                cpu_address: CpuAddress::new(0x0400_1000),
            })
        );
    }

    #[test]
    fn load_word_direct_rdram_sign_extends_and_commits_exactly_once() {
        for (loaded_word, expected) in [
            (0x1234_5678, 0x0000_0000_1234_5678),
            (0x8000_0001, 0xffff_ffff_8000_0001),
        ] {
            let mut machine = staged_lw_bootstrap_machine(
                immediate_word(0x0f, 0, 1, 0x8000),
                lw_word(1, 2, 0x0100),
            );
            machine.write_rdram_u32_be(0x100, loaded_word).unwrap();

            assert!(matches!(
                machine.step(),
                Ok(MachineRepresentedStepOutcome::CpuLocalCommitted {
                    identity: CpuInstructionIdentity::Lui,
                    ..
                })
            ));
            let outcome = machine.step().unwrap();

            assert!(matches!(
                outcome,
                MachineRepresentedStepOutcome::LoadWordCommitted {
                    effective_address: 0xffff_ffff_8000_0100,
                    target: MachineLoadWordTarget::DirectRdram { offset },
                    destination_gpr: 2,
                    loaded_word: observed,
                    result_value,
                    cadence_plan,
                } if offset == RdramOffset::new(0x100)
                    && observed == loaded_word
                    && result_value == expected
                    && cadence_plan.advances_count()
            ));
            assert_eq!(machine.cpu().gpr(2), Some(expected));
            assert_eq!(machine.cpu().pc(), 0xa400_0048);
            assert_eq!(machine.cpu().next_pc(), 0xa400_004c);
            assert_eq!(machine.cpu().cop0_count(), 2);
            assert_eq!(
                machine.cartridge_bootstrap_state().unwrap().gpr_source(2),
                Some(MachineBootstrapGprSource::KnownInstructionResult {
                    execution_address: CpuAddress::new(0xa400_0044),
                    identity: CpuInstructionIdentity::Lw,
                    source_gpr_a: Some(1),
                    source_gpr_b: None,
                })
            );
        }
    }

    #[test]
    fn load_word_uses_prewrite_base_for_alias_and_discards_gpr_zero_destination() {
        let mut alias =
            staged_lw_bootstrap_machine(immediate_word(0x0f, 0, 8, 0x8000), lw_word(8, 8, 0x0120));
        alias.write_rdram_u32_be(0x120, 0x89ab_cdef).unwrap();
        alias.step().unwrap();
        alias.step().unwrap();
        assert_eq!(alias.cpu().gpr(8), Some(0xffff_ffff_89ab_cdef));
        assert_eq!(alias.cpu().cop0_count(), 2);

        let mut zero =
            staged_lw_bootstrap_machine(immediate_word(0x0f, 0, 1, 0x8000), lw_word(1, 0, 0x0140));
        zero.write_rdram_u32_be(0x140, 0xffff_ffff).unwrap();
        zero.step().unwrap();
        let outcome = zero.step().unwrap();
        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::LoadWordCommitted {
                destination_gpr: 0,
                result_value: u64::MAX,
                ..
            }
        ));
        assert_eq!(zero.cpu().gpr(0), Some(0));
        assert_eq!(
            zero.cartridge_bootstrap_state().unwrap().gpr_source(0),
            Some(MachineBootstrapGprSource::ArchitecturalZero)
        );
        assert_eq!(zero.cpu().cop0_count(), 2);
    }

    #[test]
    fn generated_known_sp_imem_allows_real_frontier_shaped_load_word_commit() {
        let mut machine =
            staged_lw_bootstrap_machine(special_add_word(29, 0, 9), lw_word(9, 8, 0xf010));
        machine
            .stage_generated_sp_imem_word_for_test(0, 0x8000_0042)
            .unwrap();

        machine.step().unwrap();
        let outcome = machine.step().unwrap();

        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::LoadWordCommitted {
                effective_address: 0xffff_ffff_a400_1000,
                target: MachineLoadWordTarget::SpImem { offset: 0 },
                destination_gpr: 8,
                loaded_word: 0x8000_0042,
                result_value: 0xffff_ffff_8000_0042,
                ..
            }
        ));
        assert_eq!(machine.cpu().gpr(8), Some(0xffff_ffff_8000_0042));
        assert_eq!(machine.cpu().pc(), 0xa400_0048);
        assert_eq!(machine.cpu().next_pc(), 0xa400_004c);
        assert_eq!(machine.cpu().cop0_count(), 2);
        assert_eq!(
            machine.cartridge_bootstrap_state().unwrap().gpr_source(8),
            Some(MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(0xa400_0044),
                identity: CpuInstructionIdentity::Lw,
                source_gpr_a: Some(9),
                source_gpr_b: None,
            })
        );
    }

    #[test]
    fn load_word_unknown_base_sp_imem_and_target_miss_reject_without_partial_mutation() {
        let mut unknown_base =
            staged_lw_bootstrap_machine(lw_word(7, 8, 0), special_shift_word(0, 0, 0, 0, 0));
        let unknown_base_before = lw_snapshot(&unknown_base);
        assert!(matches!(
            unknown_base.step(),
            Err(MachineRepresentedStepError::BootstrapCpuStateUnavailable(error))
                if error.identity() == CpuInstructionIdentity::Lw
                    && error.register_index() == 7
        ));
        assert_eq!(lw_snapshot(&unknown_base), unknown_base_before);

        let mut unknown_imem =
            staged_lw_bootstrap_machine(special_add_word(29, 0, 9), lw_word(9, 8, 0xf010));
        unknown_imem.step().unwrap();
        let unknown_imem_before = lw_snapshot(&unknown_imem);
        let rejection = unknown_imem
            .step()
            .unwrap_err()
            .load_word_rejection()
            .unwrap();
        assert_eq!(rejection.effective_address(), 0xffff_ffff_a400_1000);
        assert_eq!(
            rejection.target(),
            Some(MachineLoadWordTarget::SpImem { offset: 0 })
        );
        assert_eq!(
            rejection.reason(),
            MachineLoadWordRejectionReason::SpImemUnknown {
                first_unknown_offset: 0,
            }
        );
        assert_eq!(lw_snapshot(&unknown_imem), unknown_imem_before);

        let mut target_miss =
            staged_lw_bootstrap_machine(special_add_word(29, 0, 9), lw_word(9, 8, 0x1010));
        target_miss.step().unwrap();
        let target_miss_before = lw_snapshot(&target_miss);
        let rejection = target_miss
            .step()
            .unwrap_err()
            .load_word_rejection()
            .unwrap();
        assert_eq!(rejection.effective_address(), 0xffff_ffff_a400_3000);
        assert_eq!(rejection.target(), None);
        assert_eq!(
            rejection.reason(),
            MachineLoadWordRejectionReason::DirectTargetMiss
        );
        assert_eq!(lw_snapshot(&target_miss), target_miss_before);
    }

    #[test]
    fn load_word_effective_address_uses_wrapping_u64_arithmetic() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine
            .write_rdram_u32_be(0, lw_word(4, 5, 0x0010))
            .unwrap();
        machine.cpu.set_gpr(4, 0xffff_ffff_ffff_fff0).unwrap();
        machine.stage_cpu_pc(0x8000_0000);
        let before = lw_snapshot(&machine);

        let rejection = machine.step().unwrap_err().load_word_rejection().unwrap();

        assert_eq!(rejection.effective_address(), 0);
        assert_eq!(rejection.cpu_address(), CpuAddress::new(0));
        assert_eq!(
            rejection.reason(),
            MachineLoadWordRejectionReason::NonDirectUnsupported
        );
        assert_eq!(lw_snapshot(&machine), before);
    }

    #[test]
    fn unaligned_load_word_enters_data_adel_without_writeback_or_normal_cadence() {
        let mut machine =
            staged_lw_bootstrap_machine(special_add_word(29, 0, 9), lw_word(9, 8, 0xf011));
        machine
            .stage_generated_sp_imem_word_for_test(0, 0x1122_3344)
            .unwrap();
        machine.step().unwrap();
        let before = lw_snapshot(&machine);

        let outcome = machine.step().unwrap();
        let after = lw_snapshot(&machine);

        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::DataAddressError {
                identity: CpuInstructionIdentity::Lw,
                effective_address: 0xffff_ffff_a400_1001,
                address_error,
                cadence_plan,
            } if address_error.bad_vaddr() == CpuAddress::new(0xa400_1001)
                && address_error.exception_kind() == CpuAddressErrorKind::AddressErrorLoad
                && !cadence_plan.advances_count()
        ));
        assert_eq!(after.gprs, before.gprs);
        assert_eq!(after.hi, before.hi);
        assert_eq!(after.lo, before.lo);
        assert_eq!(after.count, before.count);
        assert_eq!(after.compare, before.compare);
        assert_eq!(
            after.timer_interrupt_pending,
            before.timer_interrupt_pending
        );
        assert_eq!(
            after.software_interrupt_pending,
            before.software_interrupt_pending
        );
        assert_eq!(&after.rdram, &before.rdram);
        assert_eq!(&after.sp_dmem, &before.sp_dmem);
        assert_eq!(&after.sp_imem, &before.sp_imem);
        assert_eq!(after.bootstrap, before.bootstrap);
        assert_eq!(after.reservation, before.reservation);
        assert_eq!(after.powered_on, before.powered_on);
        assert_eq!(after.pc, LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(after.next_pc, LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_eq!(after.epc, 0xa400_0044);
        assert_eq!(after.bad_vaddr, 0xa400_1001);
        assert_eq!(after.exception_code, 4);
        assert!(!after.exception_branch_delay);
        assert_eq!(after.status & COP0_STATUS_EXL, COP0_STATUS_EXL);
    }

    #[test]
    fn load_word_blocked_data_adel_entry_restores_every_preinstruction_fact() {
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.write_rdram_u32_be(0, lw_word(4, 5, 0)).unwrap();
        machine.cpu.set_gpr(4, 0xffff_ffff_8000_0101).unwrap();
        let prior_alignment_error = check_cpu_data_alignment(
            CpuDataAccessKind::Read,
            CpuAddress::new(0x8000_0201),
            CpuDataWidth::Word,
        )
        .unwrap_err();
        machine
            .cpu
            .enter_data_address_error_exception(select_cpu_data_address_error(
                prior_alignment_error,
            ))
            .unwrap();
        machine.stage_cpu_pc(0x8000_0000);
        let before = lw_snapshot(&machine);

        assert!(matches!(
            machine.step(),
            Err(MachineRepresentedStepError::DataAddressErrorExceptionEntryRejected(_))
        ));

        assert_eq!(lw_snapshot(&machine), before);
    }

    fn control_flow_branch_word(opcode: u8, rs: u8, rt: u8, immediate: i16) -> u32 {
        (u32::from(opcode) << 26)
            | (u32::from(rs) << 21)
            | (u32::from(rt) << 16)
            | u32::from(immediate as u16)
    }

    fn control_flow_jump_word(opcode: u8, target: u32) -> u32 {
        (u32::from(opcode) << 26) | ((target >> 2) & 0x03ff_ffff)
    }

    fn control_flow_register_jump_word(rs: u8, rd: u8, funct: u8) -> u32 {
        (u32::from(rs) << 21) | (u32::from(rd) << 11) | u32::from(funct)
    }

    fn seed_control_flow_word(machine: &mut Machine, pc: u32, word: u32) {
        machine
            .write_rdram_u32_be((pc & 0x1fff_ffff) as usize, word)
            .unwrap();
    }

    fn assert_control_flow_commit(
        outcome: MachineRepresentedStepOutcome,
        identity: CpuInstructionIdentity,
    ) {
        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::CpuLocalCommitted {
                identity: actual_identity,
                cadence_plan,
            } if actual_identity == identity
                && cadence_plan.source() == MachineStepCadenceSource::CommittedInstruction
                && cadence_plan.count_action() == MachineStepCountAction::Advance
        ));
    }

    fn assert_scheduled_delay_slot(machine: &Machine, owner_pc: u32, slot_pc: u32, next_pc: u32) {
        assert_eq!(machine.cpu().pc(), slot_pc);
        assert_eq!(machine.cpu().next_pc(), next_pc);
        assert_eq!(
            machine
                .cpu_delay_slot_context()
                .map(CpuDelaySlotContext::branch_or_jump_pc),
            Some(owner_pc)
        );
    }

    #[test]
    fn control_flow_planning_captures_all_six_identities_without_mutation() {
        const PC: u32 = 0x8000_1000;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.stage_cpu_pc(PC);
        machine.cpu.set_gpr(4, 0xffff_ffff_8000_2000).unwrap();
        machine.cpu.set_gpr(5, 0xffff_ffff_8000_2000).unwrap();
        machine.cpu.set_gpr(31, 0xaaaa_bbbb_cccc_dddd).unwrap();
        let snapshot = machine.cpu.capture_control_flow();
        let before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu_delay_slot_context(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(5),
            machine.cpu().gpr(31),
            machine.cpu().cop0_count(),
        );

        for (word, identity) in [
            (
                control_flow_branch_word(0x04, 4, 5, 2),
                CpuInstructionIdentity::Beq,
            ),
            (
                control_flow_branch_word(0x05, 4, 5, -2),
                CpuInstructionIdentity::Bne,
            ),
            (
                control_flow_jump_word(0x02, 0x8000_2000),
                CpuInstructionIdentity::J,
            ),
            (
                control_flow_jump_word(0x03, 0x8000_2000),
                CpuInstructionIdentity::Jal,
            ),
            (
                control_flow_register_jump_word(4, 0, 0x08),
                CpuInstructionIdentity::SpecialJr,
            ),
            (
                control_flow_register_jump_word(4, 4, 0x09),
                CpuInstructionIdentity::SpecialJalr,
            ),
        ] {
            let fields = instruction_fields(word);
            let action = machine
                .produce_ordinary_control_flow_step_action(snapshot, fields, identity)
                .expect("assigned control-flow identity should produce a bounded plan");
            let result = action.result();

            assert_eq!(result.identity(), identity);
            assert_eq!(result.instruction_pc(), CpuAddress::new(PC));
            assert_eq!(result.delay_slot_pc(), CpuAddress::new(PC + 4));
            assert_eq!(result.fields(), fields);

            match identity {
                CpuInstructionIdentity::Beq | CpuInstructionIdentity::Bne => {
                    let source_a = result.source_a().unwrap();
                    let source_b = result.source_b().unwrap();
                    assert_eq!(source_a.register_index(), 4);
                    assert_eq!(source_a.value(), 0xffff_ffff_8000_2000);
                    assert_eq!(source_b.register_index(), 5);
                    assert_eq!(source_b.value(), 0xffff_ffff_8000_2000);
                    assert!(result.condition_taken().is_some());
                }
                CpuInstructionIdentity::SpecialJr | CpuInstructionIdentity::SpecialJalr => {
                    let source = result.source_a().unwrap();
                    assert_eq!(source.register_index(), 4);
                    assert_eq!(source.value(), 0xffff_ffff_8000_2000);
                    assert_eq!(result.target_pc(), CpuAddress::new(0x8000_2000));
                }
                CpuInstructionIdentity::J | CpuInstructionIdentity::Jal => {
                    assert_eq!(result.source_a(), None);
                    assert_eq!(result.source_b(), None);
                    assert_eq!(result.target_pc(), CpuAddress::new(0x8000_2000));
                }
                _ => unreachable!(),
            }
        }

        assert_eq!(
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu_delay_slot_context(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(5),
                machine.cpu().gpr(31),
                machine.cpu().cop0_count(),
            ),
            before
        );
    }

    #[test]
    fn control_flow_target_and_link_arithmetic_wraps_explicitly() {
        assert_eq!(conditional_branch_target(0xffff_fffc, 1), 0x0000_0004);
        assert_eq!(
            conditional_branch_target(0xffff_fffc, (-1_i16) as u16),
            0xffff_fffc
        );
        assert_eq!(jump_target(0xffff_fffc, 0x0000_0003), 0x0000_000c);
        assert_eq!(sign_extend_cpu_address(0xffff_fffc_u32.wrapping_add(8)), 4);
    }

    #[test]
    fn current_pc_control_flow_producer_keeps_every_planned_fact_pre_mutation() {
        const PC: u32 = 0x8000_0000;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        seed_control_flow_word(
            &mut machine,
            PC,
            control_flow_register_jump_word(4, 4, 0x09),
        );
        machine
            .cpu
            .set_gpr(4, sign_extend_cpu_address(0x8000_0020))
            .unwrap();
        machine.cpu.set_gpr(31, 0xaaaa_bbbb_cccc_dddd).unwrap();
        machine.stage_cpu_pc(PC);
        let before = (
            machine.cpu().pc(),
            machine.cpu().next_pc(),
            machine.cpu_delay_slot_context(),
            machine.cpu().gpr(4),
            machine.cpu().gpr(31),
            machine.cpu().cop0_count(),
            machine.cpu().cop0_status(),
        );

        let produced = machine
            .produce_current_pc_classified_step_action()
            .expect("JALR should produce a bounded ordinary-control-flow action");

        assert!(matches!(
            produced.action(),
            MachineClassifiedStepAction::OrdinaryControlFlow(
                MachineOrdinaryControlFlowStepAction::Jalr(_)
            )
        ));
        assert_eq!(
            (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu_delay_slot_context(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(31),
                machine.cpu().cop0_count(),
                machine.cpu().cop0_status(),
            ),
            before
        );
    }

    #[test]
    fn control_flow_bootstrap_unknown_sources_and_link_lineage_reject_before_mutation() {
        let mut unknown_source =
            staged_lw_bootstrap_machine(control_flow_branch_word(0x04, 4, 0, 1), 0);
        let unknown_before = lw_snapshot(&unknown_source);

        let unknown_error = unknown_source
            .step()
            .unwrap_err()
            .ordinary_control_flow_rejection()
            .expect("unknown bootstrap branch source should retain exact rejection");

        assert_eq!(unknown_error.instruction_pc(), CpuAddress::new(0xa400_0040));
        assert_eq!(unknown_error.identity(), CpuInstructionIdentity::Beq);
        assert_eq!(
            unknown_error.reason(),
            MachineOrdinaryControlFlowRejectionReason::BootstrapSourceUnavailable {
                register_index: 4,
                source: MachineBootstrapGprSource::UnknownPifProduced,
            }
        );
        assert_eq!(lw_snapshot(&unknown_source), unknown_before);

        let mut link_lineage =
            staged_lw_bootstrap_machine(control_flow_jump_word(0x03, 0xa400_0050), 0);
        let link_before = lw_snapshot(&link_lineage);

        let link_error = link_lineage
            .step()
            .unwrap_err()
            .ordinary_control_flow_rejection()
            .expect("bootstrap link without a writable lineage owner should reject");

        assert_eq!(link_error.identity(), CpuInstructionIdentity::Jal);
        assert_eq!(
            link_error.reason(),
            MachineOrdinaryControlFlowRejectionReason::BootstrapLinkLineageUnavailable {
                destination_gpr: 31,
            }
        );
        assert_eq!(lw_snapshot(&link_lineage), link_before);
    }

    #[test]
    fn control_flow_beq_taken_and_untaken_each_execute_one_delay_slot() {
        const PC: u32 = 0x8000_0000;
        for (rhs, expected_taken, expected_next_pc) in
            [(7_u64, true, PC + 12), (8_u64, false, PC + 8)]
        {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            seed_control_flow_word(&mut machine, PC, control_flow_branch_word(0x04, 4, 5, 2));
            seed_control_flow_word(&mut machine, PC + 4, immediate_word(0x09, 6, 6, 1));
            seed_control_flow_word(
                &mut machine,
                expected_next_pc,
                immediate_word(0x09, 7, 7, 1),
            );
            machine.cpu.set_gpr(4, 7).unwrap();
            machine.cpu.set_gpr(5, rhs).unwrap();
            machine.stage_cpu_pc(PC);
            let fields = instruction_fields(control_flow_branch_word(0x04, 4, 5, 2));
            let plan = machine
                .produce_ordinary_control_flow_step_action(
                    machine.cpu.capture_control_flow(),
                    fields,
                    CpuInstructionIdentity::Beq,
                )
                .unwrap()
                .result();
            assert_eq!(plan.condition_taken(), Some(expected_taken));

            let branch = machine.step().unwrap();
            assert_control_flow_commit(branch, CpuInstructionIdentity::Beq);
            assert_scheduled_delay_slot(&machine, PC, PC + 4, expected_next_pc);
            assert_eq!(machine.cpu().cop0_count(), 1);
            assert_eq!(machine.cpu().gpr(6), Some(0));

            let slot = machine.step().unwrap();
            assert_control_flow_commit(slot, CpuInstructionIdentity::Addiu);
            assert_eq!(machine.cpu().gpr(6), Some(1));
            assert_eq!(machine.cpu().pc(), expected_next_pc);
            assert_eq!(machine.cpu().next_pc(), expected_next_pc + 4);
            assert_eq!(machine.cpu_delay_slot_context(), None);
            assert_eq!(machine.cpu().cop0_count(), 2);
            assert_eq!(machine.cpu().gpr(7), Some(0));

            assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Addiu);
            assert_eq!(machine.cpu().gpr(6), Some(1));
            assert_eq!(machine.cpu().gpr(7), Some(1));
            assert_eq!(machine.cpu().cop0_count(), 3);
        }
    }

    #[test]
    fn control_flow_beq_zero_same_register_and_negative_target_rules_are_explicit() {
        const PC: u32 = 0x8000_0020;
        for (rs, rt) in [(0, 0), (4, 4)] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            seed_control_flow_word(&mut machine, PC, control_flow_branch_word(0x04, rs, rt, -2));
            seed_control_flow_word(&mut machine, PC + 4, immediate_word(0x09, 6, 6, 1));
            machine.cpu.set_gpr(4, 0x1234_5678).unwrap();
            machine.stage_cpu_pc(PC);

            assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Beq);
            assert_scheduled_delay_slot(&machine, PC, PC + 4, PC - 4);
            assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Addiu);
            assert_eq!(machine.cpu().pc(), PC - 4);
            assert_eq!(machine.cpu().cop0_count(), 2);
        }
    }

    #[test]
    fn control_flow_bne_taken_untaken_positive_and_negative_targets() {
        const PC: u32 = 0x8000_0020;
        for (rhs, immediate, expected_next_pc) in [
            (2_u64, 2_i16, PC + 12),
            (1_u64, 2_i16, PC + 8),
            (2_u64, -2_i16, PC - 4),
            (1_u64, -2_i16, PC + 8),
        ] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            seed_control_flow_word(
                &mut machine,
                PC,
                control_flow_branch_word(0x05, 4, 5, immediate),
            );
            seed_control_flow_word(&mut machine, PC + 4, 0);
            machine.cpu.set_gpr(4, 1).unwrap();
            machine.cpu.set_gpr(5, rhs).unwrap();
            machine.stage_cpu_pc(PC);

            assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Bne);
            assert_scheduled_delay_slot(&machine, PC, PC + 4, expected_next_pc);
            assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::SpecialSll);
            assert_eq!(machine.cpu().pc(), expected_next_pc);
            assert_eq!(machine.cpu_delay_slot_context(), None);
            assert_eq!(machine.cpu().cop0_count(), 2);
        }
    }

    #[test]
    fn control_flow_jump_uses_pc_plus_four_region_and_executes_slot_once() {
        const PC: u32 = 0x803f_fff8;
        const TARGET: u32 = 0x8000_0100;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        seed_control_flow_word(&mut machine, PC, control_flow_jump_word(0x02, TARGET));
        seed_control_flow_word(&mut machine, PC + 4, immediate_word(0x09, 0, 6, 1));
        machine.stage_cpu_pc(PC);

        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::J);
        assert_scheduled_delay_slot(&machine, PC, PC + 4, TARGET);
        assert_eq!(machine.cpu().gpr(6), Some(0));

        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Addiu);
        assert_eq!(machine.cpu().gpr(6), Some(1));
        assert_eq!(machine.cpu().pc(), TARGET);
        assert_eq!(machine.cpu().next_pc(), TARGET + 4);
        assert_eq!(machine.cpu_delay_slot_context(), None);
        assert_eq!(machine.cpu().cop0_count(), 2);
    }

    #[test]
    fn control_flow_jal_writes_link_before_delay_slot_execution() {
        const PC: u32 = 0x8000_0000;
        const TARGET: u32 = 0x8000_0010;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        seed_control_flow_word(&mut machine, PC, control_flow_jump_word(0x03, TARGET));
        seed_control_flow_word(&mut machine, PC + 4, special_shift_word(31, 0, 5, 0, 0x21));
        machine.stage_cpu_pc(PC);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(0, 1, false);
        let expected_link = sign_extend_cpu_address(PC + 8);

        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Jal);
        assert_eq!(machine.cpu().gpr(31), Some(expected_link));
        assert_scheduled_delay_slot(&machine, PC, PC + 4, TARGET);
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert!(machine.cpu().cop0_timer_interrupt_pending());

        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::SpecialAddu);
        assert_eq!(machine.cpu().gpr(5), Some(expected_link));
        assert_eq!(machine.cpu().pc(), TARGET);
        assert_eq!(machine.cpu().cop0_count(), 2);
        assert!(machine.cpu().cop0_timer_interrupt_pending());
    }

    #[test]
    fn control_flow_jr_and_jalr_capture_old_source_before_link_write() {
        const PC: u32 = 0x8000_0000;
        const TARGET: u32 = 0x8000_0020;
        for (word, identity, link_destination) in [
            (
                control_flow_register_jump_word(4, 0, 0x08),
                CpuInstructionIdentity::SpecialJr,
                None,
            ),
            (
                control_flow_register_jump_word(4, 5, 0x09),
                CpuInstructionIdentity::SpecialJalr,
                Some(5),
            ),
            (
                control_flow_register_jump_word(4, 4, 0x09),
                CpuInstructionIdentity::SpecialJalr,
                Some(4),
            ),
            (
                control_flow_register_jump_word(4, 0, 0x09),
                CpuInstructionIdentity::SpecialJalr,
                Some(0),
            ),
        ] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            seed_control_flow_word(&mut machine, PC, word);
            seed_control_flow_word(&mut machine, PC + 4, immediate_word(0x09, 0, 6, 1));
            machine
                .cpu
                .set_gpr(4, sign_extend_cpu_address(TARGET))
                .unwrap();
            machine.cpu.set_gpr(5, 0xaaaa_bbbb_cccc_dddd).unwrap();
            machine.stage_cpu_pc(PC);

            assert_control_flow_commit(machine.step().unwrap(), identity);
            assert_scheduled_delay_slot(&machine, PC, PC + 4, TARGET);
            match link_destination {
                Some(0) => assert_eq!(machine.cpu().gpr(0), Some(0)),
                Some(destination) => assert_eq!(
                    machine.cpu().gpr(destination),
                    Some(sign_extend_cpu_address(PC + 8))
                ),
                None => assert_eq!(machine.cpu().gpr(4), Some(sign_extend_cpu_address(TARGET))),
            }

            assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Addiu);
            assert_eq!(machine.cpu().gpr(6), Some(1));
            assert_eq!(machine.cpu().pc(), TARGET);
            assert_eq!(machine.cpu_delay_slot_context(), None);
            assert_eq!(machine.cpu().cop0_count(), 2);
        }
    }

    #[test]
    fn branch_in_delay_slot_rejects_all_six_identities_without_mutation() {
        const PC: u32 = 0x8000_0000;
        const TARGET: u32 = 0x8000_0020;
        for (inner_word, inner_identity) in [
            (
                control_flow_branch_word(0x04, 0, 0, 1),
                CpuInstructionIdentity::Beq,
            ),
            (
                control_flow_branch_word(0x05, 0, 0, 1),
                CpuInstructionIdentity::Bne,
            ),
            (
                control_flow_jump_word(0x02, 0x8000_0040),
                CpuInstructionIdentity::J,
            ),
            (
                control_flow_jump_word(0x03, 0x8000_0040),
                CpuInstructionIdentity::Jal,
            ),
            (
                control_flow_register_jump_word(4, 0, 0x08),
                CpuInstructionIdentity::SpecialJr,
            ),
            (
                control_flow_register_jump_word(4, 5, 0x09),
                CpuInstructionIdentity::SpecialJalr,
            ),
        ] {
            let mut machine = Machine::from_cartridge(Cartridge::default());
            seed_control_flow_word(&mut machine, PC, control_flow_jump_word(0x02, TARGET));
            seed_control_flow_word(&mut machine, PC + 4, inner_word);
            machine
                .cpu
                .set_gpr(4, sign_extend_cpu_address(0x8000_0040))
                .unwrap();
            machine.cpu.set_gpr(5, 0x1111_2222_3333_4444).unwrap();
            machine.cpu.set_gpr(31, 0x5555_6666_7777_8888).unwrap();
            machine.stage_cpu_pc(PC);
            assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::J);
            let before = (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu_delay_slot_context(),
                machine.cpu().cop0_count(),
                machine.cpu().gpr(4),
                machine.cpu().gpr(5),
                machine.cpu().gpr(31),
                machine.cpu().cop0_status(),
                machine.cpu().cop0_epc(),
                machine.cpu().cop0_bad_vaddr(),
                machine.cpu().cop0_exception_code(),
                machine.cpu().cop0_exception_branch_delay(),
            );

            let outcome = machine.step().unwrap();
            assert!(matches!(
                outcome,
                MachineRepresentedStepOutcome::Unsupported {
                    instruction,
                    cadence_plan,
                } if instruction.identity() == inner_identity
                    && instruction.category()
                        == MachineStepUnsupportedInstructionCategory::ControlFlowInDelaySlot
                    && cadence_plan.count_action() == MachineStepCountAction::DoNotAdvance
                    && cadence_plan.control_flow_action()
                        == MachineStepControlFlowAction::RestoreSnapshot
            ));
            assert_eq!(
                (
                    machine.cpu().pc(),
                    machine.cpu().next_pc(),
                    machine.cpu_delay_slot_context(),
                    machine.cpu().cop0_count(),
                    machine.cpu().gpr(4),
                    machine.cpu().gpr(5),
                    machine.cpu().gpr(31),
                    machine.cpu().cop0_status(),
                    machine.cpu().cop0_epc(),
                    machine.cpu().cop0_bad_vaddr(),
                    machine.cpu().cop0_exception_code(),
                    machine.cpu().cop0_exception_branch_delay(),
                ),
                before
            );
        }
    }

    #[test]
    fn branch_delay_exception_arithmetic_overflow_uses_owner_epc_and_no_slot_count() {
        const PC: u32 = 0x8000_0000;
        const TARGET: u32 = 0x8000_0020;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        seed_control_flow_word(&mut machine, PC, control_flow_jump_word(0x02, TARGET));
        seed_control_flow_word(&mut machine, PC + 4, immediate_word(0x08, 2, 3, 1));
        machine.cpu.set_gpr(2, 0x0000_0000_7fff_ffff).unwrap();
        machine.cpu.set_gpr(3, 0xaaaa_bbbb_cccc_dddd).unwrap();
        machine.stage_cpu_pc(PC);

        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::J);
        assert_eq!(machine.cpu().cop0_count(), 1);
        let destination_before = machine.cpu().gpr(3);

        assert!(matches!(
            machine.step().unwrap(),
            MachineRepresentedStepOutcome::ArithmeticOverflowException {
                identity: CpuInstructionIdentity::Addi,
            }
        ));
        assert_eq!(machine.cpu().cop0_epc(), PC);
        assert!(machine.cpu().cop0_exception_branch_delay());
        assert_eq!(machine.cpu().cop0_exception_code(), 12);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert_eq!(machine.cpu().gpr(3), destination_before);
        assert_eq!(machine.cpu().pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), LOCAL_EXCEPTION_VECTOR_NEXT_PC);
        assert_ne!(machine.cpu().pc(), TARGET);
        assert_eq!(machine.cpu_delay_slot_context(), None);
    }

    #[test]
    fn branch_delay_exception_data_adel_handles_untaken_branch_context() {
        const PC: u32 = 0x8000_0000;
        const FALL_THROUGH: u32 = PC + 8;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        seed_control_flow_word(&mut machine, PC, control_flow_branch_word(0x04, 0, 1, 2));
        seed_control_flow_word(&mut machine, PC + 4, lw_word(4, 5, 0));
        machine.cpu.set_gpr(1, 1).unwrap();
        machine.cpu.set_gpr(4, 0xffff_ffff_8000_0101).unwrap();
        machine.cpu.set_gpr(5, 0xaaaa_bbbb_cccc_dddd).unwrap();
        machine.stage_cpu_pc(PC);

        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::Beq);
        assert_scheduled_delay_slot(&machine, PC, PC + 4, FALL_THROUGH);
        let destination_before = machine.cpu().gpr(5);

        let outcome = machine.step().unwrap();
        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::DataAddressError {
                identity: CpuInstructionIdentity::Lw,
                address_error,
                cadence_plan,
                ..
            } if address_error.bad_vaddr() == CpuAddress::new(0x8000_0101)
                && address_error.exception_kind() == CpuAddressErrorKind::AddressErrorLoad
                && !cadence_plan.advances_count()
        ));
        assert_eq!(machine.cpu().cop0_epc(), PC);
        assert!(machine.cpu().cop0_exception_branch_delay());
        assert_eq!(machine.cpu().cop0_exception_code(), 4);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0x8000_0101);
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert_eq!(machine.cpu().gpr(5), destination_before);
        assert_eq!(machine.cpu().pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_ne!(machine.cpu().pc(), FALL_THROUGH);
        assert_eq!(machine.cpu_delay_slot_context(), None);
    }

    #[test]
    fn branch_delay_exception_instruction_fetch_adel_uses_explicit_test_staging() {
        const OWNER_PC: u32 = 0x8000_0000;
        const SLOT_PC: u32 = 0x8000_0006;
        const TARGET: u32 = 0x8000_0020;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        machine.cpu.stage_pc(SLOT_PC);
        machine.cpu.stage_next_pc(TARGET);
        machine.cpu.stage_delay_slot_context_for_test(OWNER_PC);
        machine
            .cpu
            .stage_cop0_count_compare_timer_for_test(1, 0x20, false);

        let outcome = machine.step().unwrap();
        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::InstructionFetchAddressError {
                plan,
                cadence_plan,
            } if plan.bad_vaddr() == CpuAddress::new(SLOT_PC)
                && plan.cause_exception_code() == 4
                && !cadence_plan.advances_count()
        ));
        assert_eq!(machine.cpu().cop0_epc(), OWNER_PC);
        assert!(machine.cpu().cop0_exception_branch_delay());
        assert_eq!(machine.cpu().cop0_bad_vaddr(), SLOT_PC);
        assert_eq!(machine.cpu().cop0_exception_code(), 4);
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert_eq!(machine.cpu().pc(), LOCAL_EXCEPTION_VECTOR_PC);
        assert_ne!(machine.cpu().pc(), TARGET);
        assert_eq!(machine.cpu_delay_slot_context(), None);
    }

    #[test]
    fn delay_slot_context_reset_and_direct_pc_staging_clear_stale_state() {
        const PC: u32 = 0x8000_0000;
        const TARGET: u32 = 0x8000_0020;
        let mut machine = Machine::from_cartridge(Cartridge::default());
        seed_control_flow_word(&mut machine, PC, control_flow_jump_word(0x02, TARGET));
        machine.stage_cpu_pc(PC);

        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::J);
        assert!(machine.cpu_delay_slot_context().is_some());
        machine.stage_cpu_pc(0x8000_0100);
        assert_eq!(machine.cpu_delay_slot_context(), None);

        machine.stage_cpu_pc(PC);
        assert_control_flow_commit(machine.step().unwrap(), CpuInstructionIdentity::J);
        assert!(machine.cpu_delay_slot_context().is_some());
        machine.reset();
        assert_eq!(machine.cpu_delay_slot_context(), None);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().next_pc(), NON_BOOT_RESET_VECTOR_NEXT_PC);
    }
}
