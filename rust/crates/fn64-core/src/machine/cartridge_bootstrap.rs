use core::fmt;

use crate::cartridge::{
    CartridgeReadError, RomSourceLayout, CARTRIDGE_CANDIDATE_IPL3_BYTE_COUNT,
    CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE, CARTRIDGE_CANDIDATE_IPL3_START_OFFSET,
};
use crate::cpu::address::{CpuAddress, RdramOffset};
use crate::cpu::{
    decode_cpu_instruction_word, identify_cpu_instruction, Cpu, CpuInstructionFields,
    CpuInstructionIdentity, CpuRegisterIndexError, CPU_GPR_COUNT,
};
use crate::machine::{Machine, MachineCpuInstructionFetchError, MachineCpuInstructionFetchTarget};
use crate::pif_firmware::{MachinePifFirmwareState, PifIpl2CopyLayout, PifIpl2Profile};
use crate::rdram::Rdram;
use crate::sp_dmem::{SpDmem, SpDmemOffset, SpDmemWriteError};
use crate::sp_imem::{SpImem, SpImemPifIpl2CopyError};

use super::rdram_reservation::CpuRdramReservation;

pub const MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC: u32 = 0xa400_0040;
pub const MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC: u32 = 0xa400_0044;
pub const MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET: u32 =
    CARTRIDGE_CANDIDATE_IPL3_START_OFFSET;
pub const MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE: u32 =
    CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE;
pub const MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX: u8 = 11;
pub const MACHINE_PIF_IPL2_HANDOFF_T3_VALUE: u64 = 0xffff_ffff_a400_0040;
pub const MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX: u8 = 29;
pub const MACHINE_PIF_IPL2_HANDOFF_SP_VALUE: u64 = 0xffff_ffff_a400_1ff0;
pub const MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX: u8 = 31;
pub const MACHINE_PIF_IPL2_HANDOFF_NTSC_RA_VALUE: u64 = 0xffff_ffff_a400_1550;
pub const MACHINE_PIF_IPL2_HANDOFF_NTSC_LINK_INSTRUCTION_ADDRESS: u32 = 0xa400_1548;
pub const MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX: u8 = 19;
pub const MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX: u8 = 20;
pub const MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX: u8 = 21;
pub const MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX: u8 = 22;
pub const MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX: u8 = 23;
pub const MACHINE_PIF_IPL2_HANDOFF_X105_SEED: u64 = 0x91;
pub const MACHINE_PIF_IPL1_STATUS: u32 = 0x3400_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePifIpl3Family {
    X105,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePifIpl2HandoffResetKind {
    Cold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePifIpl2HandoffBootMedium {
    Cartridge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePifVersionBit {
    Zero,
    One,
}

impl MachinePifVersionBit {
    pub const fn value(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePifIpl2HandoffInputs {
    ipl3_family: MachinePifIpl3Family,
    reset_kind: MachinePifIpl2HandoffResetKind,
    boot_medium: MachinePifIpl2HandoffBootMedium,
    pif_version_bit: MachinePifVersionBit,
}

impl MachinePifIpl2HandoffInputs {
    pub(crate) const fn new(
        ipl3_family: MachinePifIpl3Family,
        reset_kind: MachinePifIpl2HandoffResetKind,
        boot_medium: MachinePifIpl2HandoffBootMedium,
        pif_version_bit: MachinePifVersionBit,
    ) -> Self {
        Self {
            ipl3_family,
            reset_kind,
            boot_medium,
            pif_version_bit,
        }
    }

    pub const fn ipl3_family(self) -> MachinePifIpl3Family {
        self.ipl3_family
    }

    pub const fn reset_kind(self) -> MachinePifIpl2HandoffResetKind {
        self.reset_kind
    }

    pub const fn boot_medium(self) -> MachinePifIpl2HandoffBootMedium {
        self.boot_medium
    }

    pub const fn pif_version_bit(self) -> MachinePifVersionBit {
        self.pif_version_bit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineBootstrapCpuStateKind {
    /// The current represented reset subset plus the IPL3 `pc / next_pc` pair.
    ///
    /// Architectural GPR zero and the restored PIF IPL2 stack pointer are
    /// source-backed. Other PIF/CIC-produced GPR, COP0, or device state remains
    /// explicitly unknown rather than being inferred from zeroed storage.
    RepresentedResetSubset,
    /// The complete source-backed cold cartridge x105 handoff for the pinned
    /// NTSC PIF layout. PAL and MPAL remain explicitly unsupported.
    CoupledColdX105NtscPinned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineBootstrapGprSource {
    UnknownPifProduced,
    ArchitecturalZero,
    PifIpl2HandoffEntryPointer,
    PifIpl2RestoredStackPointer,
    PifIpl2RetainedLink {
        profile: PifIpl2Profile,
        link_instruction_address: CpuAddress,
    },
    CartridgeBootMedium,
    PifProfileTvType {
        profile: PifIpl2Profile,
    },
    ColdResetKind,
    X105Seed,
    PifVersionRegionalState {
        profile: PifIpl2Profile,
        pif_version_bit: MachinePifVersionBit,
    },
    KnownInstructionResult {
        execution_address: CpuAddress,
        identity: CpuInstructionIdentity,
        source_gpr_a: Option<u8>,
        source_gpr_b: Option<u8>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineBootstrapCop0StatusSource {
    UnknownPifProduced,
    PifIpl1ColdBootStatus,
}

impl MachineBootstrapCop0StatusSource {
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::UnknownPifProduced)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineBootstrapControlFlowSource {
    DirectCartridgeBootstrapStaging,
    PifIpl2CompletedX105Transfer { profile: PifIpl2Profile },
}

impl MachineBootstrapGprSource {
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::UnknownPifProduced)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCartridgeBootstrapState {
    source_layout: RomSourceLayout,
    cartridge_start_offset: u32,
    cartridge_end_offset_exclusive: u32,
    sp_dmem_start_offset: u32,
    sp_dmem_end_offset_exclusive: u32,
    execution_pc: CpuAddress,
    next_pc: CpuAddress,
    cpu_state_kind: MachineBootstrapCpuStateKind,
    pif_firmware_state: MachinePifFirmwareState,
    pif_ipl2_profile: Option<PifIpl2Profile>,
    pif_ipl2_copy_layout: Option<PifIpl2CopyLayout>,
    pif_ipl2_handoff_inputs: Option<MachinePifIpl2HandoffInputs>,
    cop0_status_source: MachineBootstrapCop0StatusSource,
    control_flow_source: MachineBootstrapControlFlowSource,
    gpr_sources: [MachineBootstrapGprSource; CPU_GPR_COUNT],
}

impl MachineCartridgeBootstrapState {
    pub const fn source_layout(self) -> RomSourceLayout {
        self.source_layout
    }

    pub const fn cartridge_start_offset(self) -> u32 {
        self.cartridge_start_offset
    }

    pub const fn cartridge_end_offset_exclusive(self) -> u32 {
        self.cartridge_end_offset_exclusive
    }

    pub const fn sp_dmem_start_offset(self) -> u32 {
        self.sp_dmem_start_offset
    }

    pub const fn sp_dmem_end_offset_exclusive(self) -> u32 {
        self.sp_dmem_end_offset_exclusive
    }

    pub const fn execution_pc(self) -> CpuAddress {
        self.execution_pc
    }

    pub const fn next_pc(self) -> CpuAddress {
        self.next_pc
    }

    pub const fn cpu_state_kind(self) -> MachineBootstrapCpuStateKind {
        self.cpu_state_kind
    }

    pub const fn pif_firmware_state(self) -> MachinePifFirmwareState {
        self.pif_firmware_state
    }

    pub const fn pif_ipl2_profile(self) -> Option<PifIpl2Profile> {
        self.pif_ipl2_profile
    }

    pub const fn pif_ipl2_copy_layout(self) -> Option<PifIpl2CopyLayout> {
        self.pif_ipl2_copy_layout
    }

    pub const fn pif_ipl2_handoff_inputs(self) -> Option<MachinePifIpl2HandoffInputs> {
        self.pif_ipl2_handoff_inputs
    }

    pub const fn cop0_status_source(self) -> MachineBootstrapCop0StatusSource {
        self.cop0_status_source
    }

    pub const fn control_flow_source(self) -> MachineBootstrapControlFlowSource {
        self.control_flow_source
    }

    pub const fn has_unrepresented_pif_cpu_state(self) -> bool {
        true
    }

    pub fn gpr_source(self, index: usize) -> Option<MachineBootstrapGprSource> {
        self.gpr_sources.get(index).copied()
    }

    pub fn gpr_is_known(self, index: usize) -> Option<bool> {
        self.gpr_source(index)
            .map(MachineBootstrapGprSource::is_known)
    }

    pub(super) fn contains_sp_dmem_word(self, offset: SpDmemOffset) -> bool {
        let start = self.sp_dmem_start_offset;
        let Some(end) = offset.value().checked_add(4) else {
            return false;
        };

        offset.value() >= start && end <= self.sp_dmem_end_offset_exclusive
    }

    pub(super) fn cartridge_offset_for_sp_dmem(self, offset: SpDmemOffset) -> u32 {
        self.cartridge_start_offset + (offset.value() - self.sp_dmem_start_offset)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineBootstrapCpuStateUnavailable {
    cpu_address: CpuAddress,
    identity: CpuInstructionIdentity,
    register_index: u8,
    source: MachineBootstrapGprSource,
}

impl MachineBootstrapCpuStateUnavailable {
    pub const fn cpu_address(self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }

    pub const fn register_index(self) -> u8 {
        self.register_index
    }

    pub const fn source(self) -> MachineBootstrapGprSource {
        self.source
    }
}

impl fmt::Display for MachineBootstrapCpuStateUnavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "bootstrap CPU state unavailable: address={} identity={:?} gpr={} source={:?}",
            self.cpu_address.value(),
            self.identity,
            self.register_index,
            self.source
        )
    }
}

impl std::error::Error for MachineBootstrapCpuStateUnavailable {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePifIpl2HandoffInputKind {
    PifIpl2Profile,
    Ipl3Family,
    ResetKind,
    BootMedium,
    PifVersionBit,
}

impl MachinePifIpl2HandoffInputKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::PifIpl2Profile => "pif-ipl2-profile",
            Self::Ipl3Family => "ipl3-family",
            Self::ResetKind => "reset-kind",
            Self::BootMedium => "boot-medium",
            Self::PifVersionBit => "pif-version-bit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineCartridgeBootstrapError {
    CartridgeSourceRangeUnavailable {
        required_end_offset_exclusive: u32,
        actual_size_bytes: usize,
    },
    CartridgeRead(CartridgeReadError),
    SpDmemDestinationRangeUnavailable {
        start_offset: u32,
        byte_count: usize,
    },
    SpImemPifIpl2CopyDestinationRangeUnavailable {
        start_offset: u32,
        byte_count: usize,
    },
    PifIpl2ProfileRequiresFirmware {
        profile: PifIpl2Profile,
    },
    MissingPifIpl2HandoffInput {
        input: MachinePifIpl2HandoffInputKind,
    },
    UnsupportedPifIpl2HandoffProfile {
        profile: PifIpl2Profile,
    },
    CpuRegister(CpuRegisterIndexError),
}

impl fmt::Display for MachineCartridgeBootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CartridgeSourceRangeUnavailable {
                required_end_offset_exclusive,
                actual_size_bytes,
            } => write!(
                f,
                "cartridge bootstrap source range unavailable: required_end={} actual_size={}",
                required_end_offset_exclusive, actual_size_bytes
            ),
            Self::CartridgeRead(error) => {
                write!(f, "cartridge bootstrap source read rejected: {error}")
            }
            Self::SpDmemDestinationRangeUnavailable {
                start_offset,
                byte_count,
            } => write!(
                f,
                "cartridge bootstrap SP DMEM destination unavailable: start={} width={}",
                start_offset, byte_count
            ),
            Self::SpImemPifIpl2CopyDestinationRangeUnavailable {
                start_offset,
                byte_count,
            } => write!(
                f,
                "cartridge bootstrap profiled PIF IPL2 SP IMEM destination unavailable: start={} width={}",
                start_offset, byte_count
            ),
            Self::PifIpl2ProfileRequiresFirmware { profile } => write!(
                f,
                "cartridge bootstrap PIF IPL2 profile {} requires accepted PIF firmware",
                profile.name()
            ),
            Self::MissingPifIpl2HandoffInput { input } => write!(
                f,
                "cold x105 coupled handoff requires explicit {} input",
                input.name()
            ),
            Self::UnsupportedPifIpl2HandoffProfile { profile } => write!(
                f,
                "cold x105 coupled handoff unsupported for PIF IPL2 profile {}",
                profile.name()
            ),
            Self::CpuRegister(error) => {
                write!(
                    f,
                    "cartridge bootstrap CPU register staging rejected: {error}"
                )
            }
        }
    }
}

impl std::error::Error for MachineCartridgeBootstrapError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColdX105CoupledHandoffPlan {
    inputs: MachinePifIpl2HandoffInputs,
    profile: PifIpl2Profile,
    t3: u64,
    sp: u64,
    ra: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    cop0_status: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmemInstructionProvenance {
    CartridgeBootstrap { cartridge_offset: u32 },
    UnclassifiedMachineStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCpuInstructionSource {
    DirectRdram {
        offset: RdramOffset,
    },
    SpDmem {
        offset: SpDmemOffset,
        provenance: MachineSpDmemInstructionProvenance,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCpuInstructionInspection {
    cpu_address: CpuAddress,
    source: MachineCpuInstructionSource,
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
}

impl MachineCpuInstructionInspection {
    pub const fn cpu_address(self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn source(self) -> MachineCpuInstructionSource {
        self.source
    }

    pub const fn fields(self) -> CpuInstructionFields {
        self.fields
    }

    pub const fn identity(self) -> CpuInstructionIdentity {
        self.identity
    }
}

impl Machine {
    fn plan_cold_x105_coupled_handoff(
        &self,
    ) -> Result<Option<ColdX105CoupledHandoffPlan>, MachineCartridgeBootstrapError> {
        let handoff_requested = self.pif_ipl3_family.is_some()
            || self.pif_ipl2_handoff_reset_kind.is_some()
            || self.pif_ipl2_handoff_boot_medium.is_some()
            || self.pif_version_bit.is_some();
        if !handoff_requested {
            return Ok(None);
        }

        let profile = self.pif_ipl2_profile.ok_or(
            MachineCartridgeBootstrapError::MissingPifIpl2HandoffInput {
                input: MachinePifIpl2HandoffInputKind::PifIpl2Profile,
            },
        )?;
        if self.pif_firmware.is_none() {
            return Err(MachineCartridgeBootstrapError::PifIpl2ProfileRequiresFirmware { profile });
        }
        let ipl3_family = self.pif_ipl3_family.ok_or(
            MachineCartridgeBootstrapError::MissingPifIpl2HandoffInput {
                input: MachinePifIpl2HandoffInputKind::Ipl3Family,
            },
        )?;
        let reset_kind = self.pif_ipl2_handoff_reset_kind.ok_or(
            MachineCartridgeBootstrapError::MissingPifIpl2HandoffInput {
                input: MachinePifIpl2HandoffInputKind::ResetKind,
            },
        )?;
        let boot_medium = self.pif_ipl2_handoff_boot_medium.ok_or(
            MachineCartridgeBootstrapError::MissingPifIpl2HandoffInput {
                input: MachinePifIpl2HandoffInputKind::BootMedium,
            },
        )?;
        let pif_version_bit = self.pif_version_bit.ok_or(
            MachineCartridgeBootstrapError::MissingPifIpl2HandoffInput {
                input: MachinePifIpl2HandoffInputKind::PifVersionBit,
            },
        )?;

        if profile != PifIpl2Profile::NtscPinned {
            return Err(
                MachineCartridgeBootstrapError::UnsupportedPifIpl2HandoffProfile { profile },
            );
        }

        let inputs =
            MachinePifIpl2HandoffInputs::new(ipl3_family, reset_kind, boot_medium, pif_version_bit);
        Ok(Some(ColdX105CoupledHandoffPlan {
            inputs,
            profile,
            t3: MACHINE_PIF_IPL2_HANDOFF_T3_VALUE,
            sp: MACHINE_PIF_IPL2_HANDOFF_SP_VALUE,
            ra: MACHINE_PIF_IPL2_HANDOFF_NTSC_RA_VALUE,
            s3: 0,
            s4: 1,
            s5: 0,
            s6: MACHINE_PIF_IPL2_HANDOFF_X105_SEED,
            s7: u64::from(pif_version_bit.value()),
            cop0_status: MACHINE_PIF_IPL1_STATUS,
        }))
    }

    /// Creates fn64's current machine-owned cartridge bootstrap state.
    ///
    /// This operation consumes only the already-normalized Cartridge owned by
    /// this Machine. It preflights and materializes the complete IPL3 source
    /// span before replacing represented CPU, RDRAM, SP DMEM, and reservation
    /// state. The execution PC is staged last in the replacement state. The
    /// represented reset subset stages only architectural zero and the restored
    /// PIF IPL2 stack pointer. Other PIF/CIC-produced register or device state
    /// remains explicitly unknown.
    pub fn stage_cartridge_bootstrap(
        &mut self,
    ) -> Result<MachineCartridgeBootstrapState, MachineCartridgeBootstrapError> {
        let handoff_plan = self.plan_cold_x105_coupled_handoff()?;
        if let Some(profile) = self.pif_ipl2_profile {
            if self.pif_firmware.is_none() {
                return Err(
                    MachineCartridgeBootstrapError::PifIpl2ProfileRequiresFirmware { profile },
                );
            }
        }

        let required_end = CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE;
        if self.cartridge.size_bytes() < required_end as usize {
            return Err(
                MachineCartridgeBootstrapError::CartridgeSourceRangeUnavailable {
                    required_end_offset_exclusive: required_end,
                    actual_size_bytes: self.cartridge.size_bytes(),
                },
            );
        }

        let mut bootstrap_bytes = vec![0; CARTRIDGE_CANDIDATE_IPL3_BYTE_COUNT as usize];
        for (index, destination) in bootstrap_bytes.iter_mut().enumerate() {
            let cartridge_offset = CARTRIDGE_CANDIDATE_IPL3_START_OFFSET + index as u32;
            *destination = self
                .cartridge
                .read_u8(cartridge_offset)
                .map_err(MachineCartridgeBootstrapError::CartridgeRead)?;
        }

        let mut replacement_sp_dmem = SpDmem::default();
        replacement_sp_dmem
            .write_bytes(
                SpDmemOffset::new(MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET),
                &bootstrap_bytes,
            )
            .map_err(map_sp_dmem_write_error)?;
        let (replacement_sp_imem, pif_ipl2_copy_layout) =
            match (self.pif_firmware.as_ref(), self.pif_ipl2_profile) {
                (Some(firmware), Some(profile)) => {
                    let copy = firmware.ipl2_copy(profile);
                    let layout = copy.layout();
                    (
                        SpImem::from_pif_ipl2_copy(copy).map_err(map_sp_imem_pif_copy_error)?,
                        Some(layout),
                    )
                }
                (Some(_), None) | (None, None) => (SpImem::default(), None),
                (None, Some(_)) => {
                    unreachable!("profile-without-firmware rejected before bootstrap planning")
                }
            };

        let mut replacement_cpu = Cpu::new();
        replacement_cpu
            .set_gpr(
                usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX),
                handoff_plan.map_or(MACHINE_PIF_IPL2_HANDOFF_SP_VALUE, |plan| plan.sp),
            )
            .map_err(MachineCartridgeBootstrapError::CpuRegister)?;
        if let Some(plan) = handoff_plan {
            for (index, value) in [
                (MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX, plan.t3),
                (MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX, plan.ra),
                (MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX, plan.s3),
                (MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX, plan.s4),
                (MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX, plan.s5),
                (MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX, plan.s6),
                (MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX, plan.s7),
            ] {
                replacement_cpu
                    .set_gpr(usize::from(index), value)
                    .map_err(MachineCartridgeBootstrapError::CpuRegister)?;
            }
            replacement_cpu.stage_cop0_status_for_bootstrap(plan.cop0_status);
        }
        replacement_cpu.stage_pc(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC);

        let mut gpr_sources = [MachineBootstrapGprSource::UnknownPifProduced; CPU_GPR_COUNT];
        gpr_sources[0] = MachineBootstrapGprSource::ArchitecturalZero;
        gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)] =
            MachineBootstrapGprSource::PifIpl2RestoredStackPointer;
        if let Some(plan) = handoff_plan {
            gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX)] =
                MachineBootstrapGprSource::PifIpl2HandoffEntryPointer;
            gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX)] =
                MachineBootstrapGprSource::PifIpl2RetainedLink {
                    profile: plan.profile,
                    link_instruction_address: CpuAddress::new(
                        MACHINE_PIF_IPL2_HANDOFF_NTSC_LINK_INSTRUCTION_ADDRESS,
                    ),
                };
            gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX)] =
                MachineBootstrapGprSource::CartridgeBootMedium;
            gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX)] =
                MachineBootstrapGprSource::PifProfileTvType {
                    profile: plan.profile,
                };
            gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX)] =
                MachineBootstrapGprSource::ColdResetKind;
            gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX)] =
                MachineBootstrapGprSource::X105Seed;
            gpr_sources[usize::from(MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX)] =
                MachineBootstrapGprSource::PifVersionRegionalState {
                    profile: plan.profile,
                    pif_version_bit: plan.inputs.pif_version_bit(),
                };
        }

        let state = MachineCartridgeBootstrapState {
            source_layout: self.cartridge.source_layout(),
            cartridge_start_offset: CARTRIDGE_CANDIDATE_IPL3_START_OFFSET,
            cartridge_end_offset_exclusive: CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE,
            sp_dmem_start_offset: MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET,
            sp_dmem_end_offset_exclusive: MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE,
            execution_pc: CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC),
            next_pc: CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC),
            cpu_state_kind: if handoff_plan.is_some() {
                MachineBootstrapCpuStateKind::CoupledColdX105NtscPinned
            } else {
                MachineBootstrapCpuStateKind::RepresentedResetSubset
            },
            pif_firmware_state: self.pif_firmware_state(),
            pif_ipl2_profile: self.pif_ipl2_profile(),
            pif_ipl2_copy_layout,
            pif_ipl2_handoff_inputs: handoff_plan.map(|plan| plan.inputs),
            cop0_status_source: if handoff_plan.is_some() {
                MachineBootstrapCop0StatusSource::PifIpl1ColdBootStatus
            } else {
                MachineBootstrapCop0StatusSource::UnknownPifProduced
            },
            control_flow_source: handoff_plan.map_or(
                MachineBootstrapControlFlowSource::DirectCartridgeBootstrapStaging,
                |plan| MachineBootstrapControlFlowSource::PifIpl2CompletedX105Transfer {
                    profile: plan.profile,
                },
            ),
            gpr_sources,
        };
        let replacement_ri = if handoff_plan.is_some() {
            crate::ri::Ri::cold_x105_entry()
        } else {
            crate::ri::Ri::default()
        };

        self.cpu = replacement_cpu;
        self.rdram = Rdram::default();
        self.sp_dmem = replacement_sp_dmem;
        self.sp_imem = replacement_sp_imem;
        self.ri = replacement_ri;
        self.cpu_rdram_reservation = CpuRdramReservation::new();
        self.powered_on = true;
        self.cartridge_bootstrap = Some(state);

        Ok(state)
    }

    pub fn cartridge_bootstrap_state(&self) -> Option<MachineCartridgeBootstrapState> {
        self.cartridge_bootstrap
    }

    pub(crate) fn require_known_bootstrap_gpr_sources(
        &self,
        cpu_address: CpuAddress,
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
    ) -> Result<(), MachineBootstrapCpuStateUnavailable> {
        let Some(state) = self.cartridge_bootstrap else {
            return Ok(());
        };
        let access = bootstrap_gpr_access(fields, identity);

        for register_index in access.sources().into_iter().flatten() {
            let source = state
                .gpr_source(usize::from(register_index))
                .unwrap_or(MachineBootstrapGprSource::UnknownPifProduced);
            if !source.is_known() {
                return Err(MachineBootstrapCpuStateUnavailable {
                    cpu_address,
                    identity,
                    register_index,
                    source,
                });
            }
        }

        Ok(())
    }

    pub(crate) fn record_known_bootstrap_gpr_destination(
        &mut self,
        cpu_address: CpuAddress,
        fields: CpuInstructionFields,
        identity: CpuInstructionIdentity,
    ) {
        let Some(state) = self.cartridge_bootstrap.as_mut() else {
            return;
        };
        let access = bootstrap_gpr_access(fields, identity);
        let Some(destination) = access.destination else {
            return;
        };
        if destination == 0 {
            state.gpr_sources[0] = MachineBootstrapGprSource::ArchitecturalZero;
            return;
        }

        state.gpr_sources[usize::from(destination)] =
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: cpu_address,
                identity,
                source_gpr_a: access.source_a,
                source_gpr_b: access.source_b,
            };
    }

    /// Reads the current instruction through Machine-owned address routing,
    /// decode, identity, and source-provenance classification without mutation.
    pub fn inspect_current_cpu_instruction(
        &self,
    ) -> Result<MachineCpuInstructionInspection, MachineCpuInstructionFetchError> {
        let cpu_address = CpuAddress::new(self.cpu.pc());
        let target = Self::classify_cpu_instruction_fetch_target(cpu_address)
            .map_err(MachineCpuInstructionFetchError::from_target_error)?;
        let instruction_word = self.fetch_cpu_instruction_word_at(cpu_address)?;
        let source = self.classify_cpu_instruction_source(target);
        let fields = decode_cpu_instruction_word(instruction_word);

        Ok(MachineCpuInstructionInspection {
            cpu_address,
            source,
            fields,
            identity: identify_cpu_instruction(fields),
        })
    }

    fn classify_cpu_instruction_source(
        &self,
        target: MachineCpuInstructionFetchTarget,
    ) -> MachineCpuInstructionSource {
        match target {
            MachineCpuInstructionFetchTarget::DirectRdram { offset } => {
                MachineCpuInstructionSource::DirectRdram { offset }
            }
            MachineCpuInstructionFetchTarget::SpDmem { offset } => {
                let provenance = match self.cartridge_bootstrap {
                    Some(state) if state.contains_sp_dmem_word(offset) => {
                        MachineSpDmemInstructionProvenance::CartridgeBootstrap {
                            cartridge_offset: state.cartridge_offset_for_sp_dmem(offset),
                        }
                    }
                    Some(_) | None => {
                        MachineSpDmemInstructionProvenance::UnclassifiedMachineStorage
                    }
                };
                MachineCpuInstructionSource::SpDmem { offset, provenance }
            }
            MachineCpuInstructionFetchTarget::PifResetUnavailable => {
                unreachable!("unavailable PIF reset target cannot yield an instruction")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BootstrapGprAccess {
    source_a: Option<u8>,
    source_b: Option<u8>,
    destination: Option<u8>,
}

impl BootstrapGprAccess {
    const fn sources(self) -> [Option<u8>; 2] {
        [self.source_a, self.source_b]
    }
}

const fn bootstrap_gpr_access(
    fields: CpuInstructionFields,
    identity: CpuInstructionIdentity,
) -> BootstrapGprAccess {
    use CpuInstructionIdentity::*;

    match identity {
        SpecialSll | SpecialSrl | SpecialSra | SpecialDsll | SpecialDsrl | SpecialDsra
        | SpecialDsll32 | SpecialDsrl32 | SpecialDsra32 => BootstrapGprAccess {
            source_a: Some(fields.rt()),
            source_b: None,
            destination: Some(fields.rd()),
        },
        SpecialSllv | SpecialSrlv | SpecialSrav | SpecialDsllv | SpecialDsrlv | SpecialDsrav
        | SpecialAnd | SpecialOr | SpecialXor | SpecialNor | SpecialAddu | SpecialSubu
        | SpecialDaddu | SpecialDsubu | SpecialSlt | SpecialSltu | SpecialAdd | SpecialSub
        | SpecialDadd | SpecialDsub => BootstrapGprAccess {
            source_a: Some(fields.rs()),
            source_b: Some(fields.rt()),
            destination: Some(fields.rd()),
        },
        SpecialMthi | SpecialMtlo => BootstrapGprAccess {
            source_a: Some(fields.rs()),
            source_b: None,
            destination: None,
        },
        SpecialMfhi | SpecialMflo => BootstrapGprAccess {
            source_a: None,
            source_b: None,
            destination: Some(fields.rd()),
        },
        Addi | Daddi | Addiu | Daddiu | Slti | Sltiu | Andi | Ori | Xori => BootstrapGprAccess {
            source_a: Some(fields.rs()),
            source_b: None,
            destination: Some(fields.rt()),
        },
        Lw => BootstrapGprAccess {
            source_a: Some(fields.rs()),
            source_b: None,
            destination: Some(fields.rt()),
        },
        Lui => BootstrapGprAccess {
            source_a: None,
            source_b: None,
            destination: Some(fields.rt()),
        },
        _ => BootstrapGprAccess {
            source_a: None,
            source_b: None,
            destination: None,
        },
    }
}

fn map_sp_dmem_write_error(error: SpDmemWriteError) -> MachineCartridgeBootstrapError {
    MachineCartridgeBootstrapError::SpDmemDestinationRangeUnavailable {
        start_offset: error.offset().value(),
        byte_count: error.width(),
    }
}

fn map_sp_imem_pif_copy_error(error: SpImemPifIpl2CopyError) -> MachineCartridgeBootstrapError {
    MachineCartridgeBootstrapError::SpImemPifIpl2CopyDestinationRangeUnavailable {
        start_offset: error.start_offset(),
        byte_count: error.byte_count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::load_cartridge;
    use crate::cpu::{CpuInstructionIdentity, CPU_GPR_COUNT, NON_BOOT_RESET_VECTOR_PC};
    use crate::machine::{MachineRepresentedStepError, MachineRepresentedStepOutcome};
    use crate::pif_firmware::{PifFirmwareClassification, PifIpl2Profile, PIF_BOOT_ROM_SIZE_BYTES};
    use crate::sp_imem::{SpImemByteObservation, SpImemByteProvenance, SpImemOffset};

    #[derive(Debug, PartialEq, Eq)]
    struct MachineArchitecturalSnapshot {
        pif_firmware_state: MachinePifFirmwareState,
        pif_firmware_bytes: Option<Vec<u8>>,
        pif_ipl2_profile: Option<PifIpl2Profile>,
        pif_ipl3_family: Option<MachinePifIpl3Family>,
        pif_ipl2_handoff_reset_kind: Option<MachinePifIpl2HandoffResetKind>,
        pif_ipl2_handoff_boot_medium: Option<MachinePifIpl2HandoffBootMedium>,
        pif_version_bit: Option<MachinePifVersionBit>,
        pc: u32,
        next_pc: u32,
        gprs: [u64; CPU_GPR_COUNT],
        hi: u64,
        lo: u64,
        cop0_count: u32,
        cop0_compare: u32,
        cop0_timer_interrupt_pending: bool,
        cop0_status: u32,
        cop0_software_interrupt_pending: u32,
        cop0_software_interrupt_pending_known: bool,
        cop0_epc: u32,
        cop0_bad_vaddr: u32,
        cop0_exception_code: u8,
        cop0_exception_branch_delay: bool,
        rdram: Vec<u8>,
        sp_dmem: Vec<u8>,
        sp_imem: Vec<SpImemByteObservation>,
        ri_select: Option<crate::ri::MachineRiSelectState>,
        bootstrap: Option<MachineCartridgeBootstrapState>,
    }

    fn architectural_snapshot(machine: &Machine) -> MachineArchitecturalSnapshot {
        MachineArchitecturalSnapshot {
            pif_firmware_state: machine.pif_firmware_state(),
            pif_firmware_bytes: machine.pif_firmware_bytes_for_test().map(<[u8]>::to_vec),
            pif_ipl2_profile: machine.pif_ipl2_profile(),
            pif_ipl3_family: machine.pif_ipl3_family(),
            pif_ipl2_handoff_reset_kind: machine.pif_ipl2_handoff_reset_kind(),
            pif_ipl2_handoff_boot_medium: machine.pif_ipl2_handoff_boot_medium(),
            pif_version_bit: machine.pif_version_bit(),
            pc: machine.cpu().pc(),
            next_pc: machine.cpu().next_pc(),
            gprs: core::array::from_fn(|index| machine.cpu().gpr(index).unwrap()),
            hi: machine.cpu().hi(),
            lo: machine.cpu().lo(),
            cop0_count: machine.cpu().cop0_count(),
            cop0_compare: machine.cpu().cop0_compare(),
            cop0_timer_interrupt_pending: machine.cpu().cop0_timer_interrupt_pending(),
            cop0_status: machine.cpu().cop0_status(),
            cop0_software_interrupt_pending: machine.cpu().cop0_software_interrupt_pending(),
            cop0_software_interrupt_pending_known: machine
                .cpu()
                .cop0_software_interrupt_pending_known(),
            cop0_epc: machine.cpu().cop0_epc(),
            cop0_bad_vaddr: machine.cpu().cop0_bad_vaddr(),
            cop0_exception_code: machine.cpu().cop0_exception_code(),
            cop0_exception_branch_delay: machine.cpu().cop0_exception_branch_delay(),
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
            ri_select: machine.ri_select_state(),
            bootstrap: machine.cartridge_bootstrap_state(),
        }
    }

    fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset] = ((value >> 24) & 0xff) as u8;
        bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
        bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
        bytes[offset + 3] = (value & 0xff) as u8;
    }

    const fn special_add_word(rs: u8, rt: u8, rd: u8) -> u32 {
        ((rs as u32) << 21) | ((rt as u32) << 16) | ((rd as u32) << 11) | 0x20
    }

    const fn lw_word(base: u8, rt: u8, immediate: u16) -> u32 {
        (0x23 << 26) | ((base as u32) << 21) | ((rt as u32) << 16) | immediate as u32
    }

    fn make_generated_normalized_boot_cartridge() -> Vec<u8> {
        let mut bytes = vec![0; CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE as usize];
        write_be_u32(&mut bytes, 0x00, 0x8037_1240);
        write_be_u32(&mut bytes, 0x04, 0x0102_0304);
        write_be_u32(&mut bytes, 0x08, 0x8000_1000);
        write_be_u32(&mut bytes, 0x0c, 0x0506_0708);
        write_be_u32(&mut bytes, 0x10, 0x1112_1314);
        write_be_u32(&mut bytes, 0x14, 0x1516_1718);
        bytes[0x20..0x33].copy_from_slice(b"FN64 GENERATED BOOT");
        bytes[0x3c] = b'G';
        bytes[0x3d] = b'B';
        bytes[0x3e] = 0x45;
        bytes[0x3f] = 1;

        for (offset, byte) in bytes
            .iter_mut()
            .enumerate()
            .skip(CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize)
        {
            *byte = ((offset * 11 + 0x2d) & 0xff) as u8;
        }
        write_be_u32(
            &mut bytes,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize,
            0x3c08_1234,
        );
        write_be_u32(
            &mut bytes,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize + 4,
            0x0000_000d,
        );
        write_be_u32(
            &mut bytes,
            CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE as usize - 4,
            0x3c09_5678,
        );
        bytes
    }

    fn generated_pif_firmware(seed: u8) -> Vec<u8> {
        (0..PIF_BOOT_ROM_SIZE_BYTES)
            .map(|index| seed.wrapping_add((index as u8).wrapping_mul(43)))
            .collect()
    }

    fn install_complete_cold_x105_inputs(
        machine: &mut Machine,
        profile: PifIpl2Profile,
        pif_version_bit: MachinePifVersionBit,
        firmware_seed: u8,
    ) {
        machine
            .install_pif_firmware(generated_pif_firmware(firmware_seed))
            .unwrap();
        machine.install_pif_ipl2_profile(profile);
        machine.install_pif_ipl3_family(MachinePifIpl3Family::X105);
        machine.install_pif_ipl2_handoff_reset_kind(MachinePifIpl2HandoffResetKind::Cold);
        machine.install_pif_ipl2_handoff_boot_medium(MachinePifIpl2HandoffBootMedium::Cartridge);
        machine.install_pif_version_bit(pif_version_bit);
    }

    fn encode_source_layout(mut normalized_bytes: Vec<u8>, layout: RomSourceLayout) -> Vec<u8> {
        match layout {
            RomSourceLayout::BigEndian => normalized_bytes,
            RomSourceLayout::ByteSwapped16 => {
                for chunk in normalized_bytes.chunks_exact_mut(2) {
                    chunk.swap(0, 1);
                }
                normalized_bytes
            }
            RomSourceLayout::LittleEndian32 => {
                for chunk in normalized_bytes.chunks_exact_mut(4) {
                    chunk.swap(0, 3);
                    chunk.swap(1, 2);
                }
                normalized_bytes
            }
        }
    }

    #[test]
    fn machine_cartridge_bootstrap_normalizes_and_stages_all_source_layouts() {
        let normalized_bytes = make_generated_normalized_boot_cartridge();

        for layout in [
            RomSourceLayout::BigEndian,
            RomSourceLayout::ByteSwapped16,
            RomSourceLayout::LittleEndian32,
        ] {
            let source_bytes = encode_source_layout(normalized_bytes.clone(), layout);
            let cartridge = load_cartridge(source_bytes).unwrap();
            let mut machine = Machine::from_cartridge(cartridge);
            machine.stage_cpu_pc(0x8000_2000);
            machine.write_rdram_u32_be(0x20, 0xfeed_beef).unwrap();
            machine.cpu.set_gpr(7, 0x1122_3344_5566_7788).unwrap();

            let state = machine.stage_cartridge_bootstrap().unwrap();

            assert_eq!(state.source_layout(), layout);
            assert_eq!(
                state.cartridge_start_offset(),
                CARTRIDGE_CANDIDATE_IPL3_START_OFFSET
            );
            assert_eq!(
                state.cartridge_end_offset_exclusive(),
                CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE
            );
            assert_eq!(
                state.sp_dmem_start_offset(),
                MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET
            );
            assert_eq!(
                state.sp_dmem_end_offset_exclusive(),
                MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE
            );
            assert_eq!(
                state.cpu_state_kind(),
                MachineBootstrapCpuStateKind::RepresentedResetSubset
            );
            assert!(state.has_unrepresented_pif_cpu_state());
            assert_eq!(
                state.gpr_source(0),
                Some(MachineBootstrapGprSource::ArchitecturalZero)
            );
            assert_eq!(state.gpr_is_known(0), Some(true));
            assert_eq!(
                state.gpr_source(usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)),
                Some(MachineBootstrapGprSource::PifIpl2RestoredStackPointer)
            );
            assert_eq!(
                state.gpr_is_known(usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)),
                Some(true)
            );
            assert_eq!(
                state.gpr_source(7),
                Some(MachineBootstrapGprSource::UnknownPifProduced)
            );
            assert_eq!(state.gpr_is_known(7), Some(false));
            assert_eq!(machine.cartridge_bootstrap_state(), Some(state));
            assert_eq!(machine.cpu().pc(), MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC);
            assert_eq!(machine.cpu().next_pc(), MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC);
            assert_eq!(machine.cpu().cop0_count(), 0);
            assert_eq!(machine.cpu().gpr(0), Some(0));
            assert_eq!(
                machine
                    .cpu()
                    .gpr(usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)),
                Some(MACHINE_PIF_IPL2_HANDOFF_SP_VALUE)
            );
            assert_eq!(machine.cpu().gpr(7), Some(0));
            assert_eq!(machine.rdram().read_u32_be(0x20), Ok(0));
            assert_eq!(machine.sp_dmem().read_u8(SpDmemOffset::new(0x3f)), Ok(0));

            for offset in
                CARTRIDGE_CANDIDATE_IPL3_START_OFFSET..CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE
            {
                assert_eq!(
                    machine.sp_dmem().read_u8(SpDmemOffset::new(offset)),
                    Ok(normalized_bytes[offset as usize])
                );
                assert_eq!(
                    machine.cartridge().read_u8(offset),
                    Ok(normalized_bytes[offset as usize])
                );
            }
        }
    }

    #[test]
    fn machine_cartridge_bootstrap_rejects_short_source_without_partial_mutation() {
        let mut short_bytes = make_generated_normalized_boot_cartridge();
        short_bytes.truncate(0x100);
        let cartridge = load_cartridge(short_bytes).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine.stage_cpu_pc(0x8000_3000);
        machine.write_rdram_u32_be(0x30, 0x1020_3040).unwrap();
        machine
            .sp_dmem
            .write_bytes(SpDmemOffset::new(0x40), &[0xaa, 0xbb, 0xcc, 0xdd])
            .unwrap();
        machine.cpu.set_gpr(5, 0xa5a5_5a5a).unwrap();
        machine
            .cpu
            .set_gpr(
                usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX),
                0x1357_9bdf_2468_ace0,
            )
            .unwrap();
        machine
            .install_pif_firmware(generated_pif_firmware(0x4f))
            .unwrap();
        machine.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
        machine
            .stage_generated_sp_imem_word_for_test(0, 0x5566_7788)
            .unwrap();
        let before = architectural_snapshot(&machine);

        assert_eq!(
            machine.stage_cartridge_bootstrap(),
            Err(
                MachineCartridgeBootstrapError::CartridgeSourceRangeUnavailable {
                    required_end_offset_exclusive: CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE,
                    actual_size_bytes: 0x100,
                }
            )
        );
        assert_eq!(machine.cpu().pc(), 0x8000_3000);
        assert_eq!(machine.cpu().next_pc(), 0x8000_3004);
        assert_eq!(machine.cpu().gpr(5), Some(0xa5a5_5a5a));
        assert_eq!(
            machine
                .cpu()
                .gpr(usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)),
            Some(0x1357_9bdf_2468_ace0)
        );
        assert_eq!(machine.rdram().read_u32_be(0x30), Ok(0x1020_3040));
        assert_eq!(
            machine.sp_dmem().read_u32_be(SpDmemOffset::new(0x40)),
            Ok(0xaabb_ccdd)
        );
        assert_eq!(machine.cartridge_bootstrap_state(), None);
        assert_eq!(machine.ri_select_state(), None);
        assert_eq!(architectural_snapshot(&machine), before);
    }

    #[test]
    fn machine_bootstrap_reset_subset_stages_only_zero_and_restored_pif_stack_pointer() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);

        let state = machine.stage_cartridge_bootstrap().unwrap();

        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(
            state.gpr_source(0),
            Some(MachineBootstrapGprSource::ArchitecturalZero)
        );
        assert_eq!(state.gpr_is_known(0), Some(true));
        assert_eq!(
            machine
                .cpu()
                .gpr(usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)),
            Some(MACHINE_PIF_IPL2_HANDOFF_SP_VALUE)
        );
        assert_eq!(
            state.gpr_source(usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)),
            Some(MachineBootstrapGprSource::PifIpl2RestoredStackPointer)
        );
        assert_eq!(
            state.gpr_is_known(usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX)),
            Some(true)
        );

        for index in 1..CPU_GPR_COUNT {
            if index == usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX) {
                continue;
            }
            assert_eq!(machine.cpu().gpr(index), Some(0));
            assert_eq!(
                state.gpr_source(index),
                Some(MachineBootstrapGprSource::UnknownPifProduced)
            );
            assert_eq!(state.gpr_is_known(index), Some(false));
        }
        assert_eq!(state.gpr_source(CPU_GPR_COUNT), None);
        assert_eq!(state.gpr_is_known(CPU_GPR_COUNT), None);
    }

    #[test]
    fn cold_x105_ntsc_handoff_materializes_exact_coupled_state_and_lineage() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        install_complete_cold_x105_inputs(
            &mut machine,
            PifIpl2Profile::NtscPinned,
            MachinePifVersionBit::One,
            0x34,
        );

        let state = machine.stage_cartridge_bootstrap().unwrap();

        assert_eq!(
            state.cpu_state_kind(),
            MachineBootstrapCpuStateKind::CoupledColdX105NtscPinned
        );
        assert_eq!(
            state.pif_ipl2_handoff_inputs(),
            Some(MachinePifIpl2HandoffInputs::new(
                MachinePifIpl3Family::X105,
                MachinePifIpl2HandoffResetKind::Cold,
                MachinePifIpl2HandoffBootMedium::Cartridge,
                MachinePifVersionBit::One,
            ))
        );
        assert_eq!(
            state.cop0_status_source(),
            MachineBootstrapCop0StatusSource::PifIpl1ColdBootStatus
        );
        assert_eq!(
            state.control_flow_source(),
            MachineBootstrapControlFlowSource::PifIpl2CompletedX105Transfer {
                profile: PifIpl2Profile::NtscPinned,
            }
        );
        assert_eq!(machine.cpu().cop0_status(), MACHINE_PIF_IPL1_STATUS);
        assert_eq!(machine.cpu().cop0_status() & 0x2, 0);
        assert_eq!(machine.cpu().cop0_status() & 0x1, 0);
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert_eq!(machine.cpu().cop0_epc(), 0);
        assert_eq!(machine.cpu().cop0_bad_vaddr(), 0);
        assert_eq!(machine.cpu().cop0_exception_code(), 0);
        assert!(!machine.cpu().cop0_exception_branch_delay());
        assert_eq!(machine.cpu().pc(), MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC);
        assert_eq!(machine.cpu().next_pc(), MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC);
        assert_eq!(machine.cpu_delay_slot_context(), None);

        let expected = [
            (
                MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX,
                MACHINE_PIF_IPL2_HANDOFF_T3_VALUE,
                MachineBootstrapGprSource::PifIpl2HandoffEntryPointer,
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX,
                MACHINE_PIF_IPL2_HANDOFF_SP_VALUE,
                MachineBootstrapGprSource::PifIpl2RestoredStackPointer,
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX,
                MACHINE_PIF_IPL2_HANDOFF_NTSC_RA_VALUE,
                MachineBootstrapGprSource::PifIpl2RetainedLink {
                    profile: PifIpl2Profile::NtscPinned,
                    link_instruction_address: CpuAddress::new(
                        MACHINE_PIF_IPL2_HANDOFF_NTSC_LINK_INSTRUCTION_ADDRESS,
                    ),
                },
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX,
                0,
                MachineBootstrapGprSource::CartridgeBootMedium,
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX,
                1,
                MachineBootstrapGprSource::PifProfileTvType {
                    profile: PifIpl2Profile::NtscPinned,
                },
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX,
                0,
                MachineBootstrapGprSource::ColdResetKind,
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX,
                MACHINE_PIF_IPL2_HANDOFF_X105_SEED,
                MachineBootstrapGprSource::X105Seed,
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX,
                1,
                MachineBootstrapGprSource::PifVersionRegionalState {
                    profile: PifIpl2Profile::NtscPinned,
                    pif_version_bit: MachinePifVersionBit::One,
                },
            ),
        ];
        for (index, value, source) in expected {
            assert_eq!(machine.cpu().gpr(usize::from(index)), Some(value));
            assert_eq!(state.gpr_source(usize::from(index)), Some(source));
            assert_eq!(state.gpr_is_known(usize::from(index)), Some(true));
        }

        for index in 1..CPU_GPR_COUNT {
            if expected
                .iter()
                .any(|(known_index, _, _)| usize::from(*known_index) == index)
            {
                continue;
            }
            assert_eq!(machine.cpu().gpr(index), Some(0));
            assert_eq!(
                state.gpr_source(index),
                Some(MachineBootstrapGprSource::UnknownPifProduced)
            );
            assert_eq!(state.gpr_is_known(index), Some(false));
        }
    }

    #[test]
    fn cold_x105_ntsc_pif_version_bit_is_explicit_and_changes_only_s7() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();

        for (bit, expected_s7) in [
            (MachinePifVersionBit::Zero, 0),
            (MachinePifVersionBit::One, 1),
        ] {
            let mut machine = Machine::from_cartridge(cartridge.clone());
            install_complete_cold_x105_inputs(&mut machine, PifIpl2Profile::NtscPinned, bit, 0x45);
            let state = machine.stage_cartridge_bootstrap().unwrap();

            assert_eq!(
                machine
                    .cpu()
                    .gpr(usize::from(MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX)),
                Some(expected_s7)
            );
            assert_eq!(
                state.gpr_source(usize::from(MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX)),
                Some(MachineBootstrapGprSource::PifVersionRegionalState {
                    profile: PifIpl2Profile::NtscPinned,
                    pif_version_bit: bit,
                })
            );
            assert_eq!(
                machine
                    .cpu()
                    .gpr(usize::from(MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX)),
                Some(MACHINE_PIF_IPL2_HANDOFF_NTSC_RA_VALUE)
            );
        }
    }

    #[test]
    fn cold_x105_handoff_missing_inputs_fail_before_any_runtime_mutation() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();

        for missing in [
            MachinePifIpl2HandoffInputKind::PifIpl2Profile,
            MachinePifIpl2HandoffInputKind::Ipl3Family,
            MachinePifIpl2HandoffInputKind::ResetKind,
            MachinePifIpl2HandoffInputKind::BootMedium,
            MachinePifIpl2HandoffInputKind::PifVersionBit,
        ] {
            let mut machine = Machine::from_cartridge(cartridge.clone());
            machine
                .install_pif_firmware(generated_pif_firmware(0x56))
                .unwrap();
            if missing != MachinePifIpl2HandoffInputKind::PifIpl2Profile {
                machine.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
            }
            if missing != MachinePifIpl2HandoffInputKind::Ipl3Family {
                machine.install_pif_ipl3_family(MachinePifIpl3Family::X105);
            }
            if missing != MachinePifIpl2HandoffInputKind::ResetKind {
                machine.install_pif_ipl2_handoff_reset_kind(MachinePifIpl2HandoffResetKind::Cold);
            }
            if missing != MachinePifIpl2HandoffInputKind::BootMedium {
                machine.install_pif_ipl2_handoff_boot_medium(
                    MachinePifIpl2HandoffBootMedium::Cartridge,
                );
            }
            if missing != MachinePifIpl2HandoffInputKind::PifVersionBit {
                machine.install_pif_version_bit(MachinePifVersionBit::Zero);
            }
            machine.stage_cpu_pc(0x8000_6000);
            machine.write_rdram_u32_be(0x68, 0x1020_3040).unwrap();
            machine
                .stage_generated_sp_imem_word_for_test(0, 0x5060_7080)
                .unwrap();
            let before = architectural_snapshot(&machine);

            assert_eq!(
                machine.stage_cartridge_bootstrap(),
                Err(MachineCartridgeBootstrapError::MissingPifIpl2HandoffInput { input: missing })
            );
            assert_eq!(architectural_snapshot(&machine), before);
        }
    }

    #[test]
    fn cold_x105_handoff_without_firmware_fails_before_any_runtime_mutation() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
        machine.install_pif_ipl3_family(MachinePifIpl3Family::X105);
        machine.install_pif_ipl2_handoff_reset_kind(MachinePifIpl2HandoffResetKind::Cold);
        machine.install_pif_ipl2_handoff_boot_medium(MachinePifIpl2HandoffBootMedium::Cartridge);
        machine.install_pif_version_bit(MachinePifVersionBit::Zero);
        machine.stage_cpu_pc(0x8000_7000);
        let before = architectural_snapshot(&machine);

        assert_eq!(
            machine.stage_cartridge_bootstrap(),
            Err(
                MachineCartridgeBootstrapError::PifIpl2ProfileRequiresFirmware {
                    profile: PifIpl2Profile::NtscPinned,
                }
            )
        );
        assert_eq!(architectural_snapshot(&machine), before);
    }

    #[test]
    fn cold_x105_handoff_rejects_unproved_pal_and_mpal_links_atomically() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();

        for profile in [PifIpl2Profile::PalPinned, PifIpl2Profile::MpalPinned] {
            let mut machine = Machine::from_cartridge(cartridge.clone());
            install_complete_cold_x105_inputs(
                &mut machine,
                PifIpl2Profile::NtscPinned,
                MachinePifVersionBit::One,
                0x67,
            );
            machine.stage_cartridge_bootstrap().unwrap();
            machine.install_pif_ipl2_profile(profile);
            machine.write_rdram_u32_be(0x78, 0x90a0_b0c0).unwrap();
            let before = architectural_snapshot(&machine);

            assert_eq!(
                machine.stage_cartridge_bootstrap(),
                Err(MachineCartridgeBootstrapError::UnsupportedPifIpl2HandoffProfile { profile })
            );
            assert_eq!(architectural_snapshot(&machine), before);
        }
    }

    #[test]
    fn cold_x105_handoff_inputs_persist_across_reset_and_repeated_bootstrap() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        install_complete_cold_x105_inputs(
            &mut machine,
            PifIpl2Profile::NtscPinned,
            MachinePifVersionBit::One,
            0x78,
        );

        let first = machine.stage_cartridge_bootstrap().unwrap();
        let first_gprs =
            core::array::from_fn::<_, CPU_GPR_COUNT, _>(|index| machine.cpu().gpr(index).unwrap());
        machine.reset();

        assert_eq!(machine.pif_ipl2_profile(), Some(PifIpl2Profile::NtscPinned));
        assert_eq!(machine.pif_ipl3_family(), Some(MachinePifIpl3Family::X105));
        assert_eq!(
            machine.pif_ipl2_handoff_reset_kind(),
            Some(MachinePifIpl2HandoffResetKind::Cold)
        );
        assert_eq!(
            machine.pif_ipl2_handoff_boot_medium(),
            Some(MachinePifIpl2HandoffBootMedium::Cartridge)
        );
        assert_eq!(machine.pif_version_bit(), Some(MachinePifVersionBit::One));
        assert_eq!(machine.cartridge_bootstrap_state(), None);
        assert_eq!(machine.ri_select_state(), None);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().cop0_status(), 0);
        assert_eq!(machine.cpu_delay_slot_context(), None);
        assert!(!machine
            .sp_imem
            .observe_byte(SpImemOffset::new(0))
            .unwrap()
            .is_known());

        let second = machine.stage_cartridge_bootstrap().unwrap();
        let second_gprs =
            core::array::from_fn::<_, CPU_GPR_COUNT, _>(|index| machine.cpu().gpr(index).unwrap());
        assert_eq!(second, first);
        assert_eq!(second_gprs, first_gprs);
        assert_eq!(machine.cpu().cop0_status(), MACHINE_PIF_IPL1_STATUS);
        assert_eq!(
            machine.ri_select_state(),
            Some(crate::ri::MachineRiSelectState::cold_x105_entry())
        );

        let third = machine.stage_cartridge_bootstrap().unwrap();
        assert_eq!(third, second);
        assert_eq!(
            core::array::from_fn::<_, CPU_GPR_COUNT, _>(|index| {
                machine.cpu().gpr(index).unwrap()
            }),
            second_gprs
        );
        assert_eq!(
            machine.ri_select_state(),
            Some(crate::ri::MachineRiSelectState::cold_x105_entry())
        );
    }

    #[test]
    fn cold_x105_handoff_remains_isolated_between_machine_instances() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut zero = Machine::from_cartridge(cartridge.clone());
        let mut one = Machine::from_cartridge(cartridge);
        install_complete_cold_x105_inputs(
            &mut zero,
            PifIpl2Profile::NtscPinned,
            MachinePifVersionBit::Zero,
            0x89,
        );
        install_complete_cold_x105_inputs(
            &mut one,
            PifIpl2Profile::NtscPinned,
            MachinePifVersionBit::One,
            0x9a,
        );

        zero.stage_cartridge_bootstrap().unwrap();
        one.stage_cartridge_bootstrap().unwrap();

        assert_eq!(
            zero.ri_select_state(),
            Some(crate::ri::MachineRiSelectState::cold_x105_entry())
        );
        assert_eq!(
            one.ri_select_state(),
            Some(crate::ri::MachineRiSelectState::cold_x105_entry())
        );

        assert_eq!(
            zero.cpu()
                .gpr(usize::from(MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX)),
            Some(0)
        );
        assert_eq!(
            one.cpu()
                .gpr(usize::from(MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX)),
            Some(1)
        );
        let one_before = architectural_snapshot(&one);
        zero.reset();
        assert_eq!(architectural_snapshot(&one), one_before);
    }

    #[test]
    fn machine_step_consumes_generated_cold_x105_t3_with_known_lineage() {
        let mut bytes = make_generated_normalized_boot_cartridge();
        write_be_u32(
            &mut bytes,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize,
            special_add_word(MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX, 0, 8),
        );
        let cartridge = load_cartridge(bytes).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        install_complete_cold_x105_inputs(
            &mut machine,
            PifIpl2Profile::NtscPinned,
            MachinePifVersionBit::Zero,
            0xab,
        );
        machine.stage_cartridge_bootstrap().unwrap();

        let outcome = machine.step().unwrap();

        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::CpuLocalCommitted {
                identity: CpuInstructionIdentity::SpecialAdd,
                ..
            }
        ));
        assert_eq!(
            machine.cpu().gpr(8),
            Some(MACHINE_PIF_IPL2_HANDOFF_T3_VALUE)
        );
        assert_eq!(
            machine.cartridge_bootstrap_state().unwrap().gpr_source(8),
            Some(MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC),
                identity: CpuInstructionIdentity::SpecialAdd,
                source_gpr_a: Some(MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX),
                source_gpr_b: Some(0),
            })
        );
        assert_eq!(machine.cpu().pc(), MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC);
        assert_eq!(
            machine.cpu().next_pc(),
            MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC + 4
        );
        assert_eq!(machine.cpu().cop0_count(), 1);
    }

    #[test]
    fn machine_bootstrap_unknown_gpr_source_rejection_has_no_partial_mutation() {
        let mut bytes = make_generated_normalized_boot_cartridge();
        write_be_u32(
            &mut bytes,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize,
            special_add_word(7, 0, 8),
        );
        let cartridge = load_cartridge(bytes).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine.stage_cartridge_bootstrap().unwrap();
        let before = architectural_snapshot(&machine);

        let error = machine.step().unwrap_err();

        let unavailable = error.bootstrap_cpu_state_unavailable().unwrap();
        assert_eq!(
            unavailable.cpu_address(),
            CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC)
        );
        assert_eq!(unavailable.identity(), CpuInstructionIdentity::SpecialAdd);
        assert_eq!(unavailable.register_index(), 7);
        assert_eq!(
            unavailable.source(),
            MachineBootstrapGprSource::UnknownPifProduced
        );
        assert!(matches!(
            error,
            MachineRepresentedStepError::BootstrapCpuStateUnavailable(_)
        ));
        assert_eq!(architectural_snapshot(&machine), before);
    }

    #[test]
    fn machine_bootstrap_known_special_add_commit_preserves_value_and_known_lineage() {
        let cases = [
            (
                MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX,
                0,
                9,
                Some(MACHINE_PIF_IPL2_HANDOFF_SP_VALUE),
            ),
            (
                MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX,
                0,
                MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX,
                Some(MACHINE_PIF_IPL2_HANDOFF_SP_VALUE),
            ),
            (MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, 0, 0, Some(0)),
        ];

        for (rs, rt, rd, expected_destination) in cases {
            let mut bytes = make_generated_normalized_boot_cartridge();
            write_be_u32(
                &mut bytes,
                CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize,
                special_add_word(rs, rt, rd),
            );
            let cartridge = load_cartridge(bytes).unwrap();
            let mut machine = Machine::from_cartridge(cartridge);
            machine.stage_cartridge_bootstrap().unwrap();
            let inspection = machine.inspect_current_cpu_instruction().unwrap();

            assert_eq!(inspection.identity(), CpuInstructionIdentity::SpecialAdd);
            assert_eq!(inspection.fields().rs(), rs);
            assert_eq!(inspection.fields().rt(), rt);
            assert_eq!(inspection.fields().rd(), rd);
            assert_eq!(
                inspection.source(),
                MachineCpuInstructionSource::SpDmem {
                    offset: SpDmemOffset::new(CARTRIDGE_CANDIDATE_IPL3_START_OFFSET),
                    provenance: MachineSpDmemInstructionProvenance::CartridgeBootstrap {
                        cartridge_offset: CARTRIDGE_CANDIDATE_IPL3_START_OFFSET,
                    },
                }
            );
            let before = machine.cartridge_bootstrap_state().unwrap();
            assert_eq!(before.gpr_is_known(usize::from(rs)), Some(true));
            assert_eq!(before.gpr_is_known(usize::from(rt)), Some(true));

            let outcome = machine.step().unwrap();

            assert!(matches!(
                outcome,
                MachineRepresentedStepOutcome::CpuLocalCommitted {
                    identity: CpuInstructionIdentity::SpecialAdd,
                    ..
                }
            ));
            assert_eq!(machine.cpu().gpr(usize::from(rd)), expected_destination);
            assert_eq!(machine.cpu().pc(), MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC);
            assert_eq!(
                machine.cpu().next_pc(),
                MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC + 4
            );
            assert_eq!(machine.cpu().cop0_count(), 1);

            let after = machine.cartridge_bootstrap_state().unwrap();
            if rd == 0 {
                assert_eq!(
                    after.gpr_source(0),
                    Some(MachineBootstrapGprSource::ArchitecturalZero)
                );
            } else {
                assert_eq!(
                    after.gpr_source(usize::from(rd)),
                    Some(MachineBootstrapGprSource::KnownInstructionResult {
                        execution_address: CpuAddress::new(
                            MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC
                        ),
                        identity: CpuInstructionIdentity::SpecialAdd,
                        source_gpr_a: Some(rs),
                        source_gpr_b: Some(rt),
                    })
                );
            }
            assert_eq!(after.gpr_is_known(usize::from(rd)), Some(true));
            assert_eq!(machine.cpu().gpr(0), Some(0));
        }
    }

    #[test]
    fn machine_cartridge_bootstrap_source_provenance_covers_exact_boundaries() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine.stage_cartridge_bootstrap().unwrap();

        let first = machine.inspect_current_cpu_instruction().unwrap();
        assert_eq!(
            first.cpu_address(),
            CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC)
        );
        assert_eq!(first.identity(), CpuInstructionIdentity::Lui);
        assert_eq!(first.fields().rt(), 8);
        assert_eq!(
            first.source(),
            MachineCpuInstructionSource::SpDmem {
                offset: SpDmemOffset::new(0x40),
                provenance: MachineSpDmemInstructionProvenance::CartridgeBootstrap {
                    cartridge_offset: 0x40,
                },
            }
        );

        machine.stage_cpu_pc(0xa400_0ffc);
        let last = machine.inspect_current_cpu_instruction().unwrap();
        assert_eq!(last.identity(), CpuInstructionIdentity::Lui);
        assert_eq!(
            last.source(),
            MachineCpuInstructionSource::SpDmem {
                offset: SpDmemOffset::new(0x0ffc),
                provenance: MachineSpDmemInstructionProvenance::CartridgeBootstrap {
                    cartridge_offset: 0x0ffc,
                },
            }
        );
    }

    #[test]
    fn machine_cartridge_bootstrap_rom_derived_step_commits_cpu_effect() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine.stage_cartridge_bootstrap().unwrap();
        let inspection = machine.inspect_current_cpu_instruction().unwrap();

        let outcome = machine.step().unwrap();

        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::CpuLocalCommitted {
                identity: CpuInstructionIdentity::Lui,
                ..
            }
        ));
        assert_eq!(
            inspection.source(),
            MachineCpuInstructionSource::SpDmem {
                offset: SpDmemOffset::new(0x40),
                provenance: MachineSpDmemInstructionProvenance::CartridgeBootstrap {
                    cartridge_offset: 0x40,
                },
            }
        );
        assert_eq!(machine.cpu().gpr(8), Some(0x0000_0000_1234_0000));
        assert_eq!(
            machine.cartridge_bootstrap_state().unwrap().gpr_source(8),
            Some(MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC),
                identity: CpuInstructionIdentity::Lui,
                source_gpr_a: None,
                source_gpr_b: None,
            })
        );
        assert_eq!(machine.cpu().pc(), 0xa400_0044);
        assert_eq!(machine.cpu().next_pc(), 0xa400_0048);
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert!(matches!(
            machine.step(),
            Ok(MachineRepresentedStepOutcome::Stopped { .. })
        ));
    }

    #[test]
    fn machine_cartridge_bootstrap_reset_clears_payload_and_provenance() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine.stage_cartridge_bootstrap().unwrap();

        machine.reset();

        assert_eq!(machine.cartridge_bootstrap_state(), None);
        assert_eq!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(
            machine.sp_dmem().read_u32_be(SpDmemOffset::new(0x40)),
            Ok(0)
        );
        let sp_imem_word_zero = machine
            .sp_imem
            .read_known_u32_be(SpImemOffset::new(0))
            .unwrap_err();
        assert_eq!(
            sp_imem_word_zero.unknown_offset(),
            Some(SpImemOffset::new(0))
        );
        machine.stage_cpu_pc(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC);
        let inspection = machine.inspect_current_cpu_instruction().unwrap();
        assert_eq!(inspection.identity(), CpuInstructionIdentity::SpecialSll);
        assert_eq!(
            inspection.source(),
            MachineCpuInstructionSource::SpDmem {
                offset: SpDmemOffset::new(0x40),
                provenance: MachineSpDmemInstructionProvenance::UnclassifiedMachineStorage,
            }
        );
    }

    #[test]
    fn machine_cartridge_bootstrap_keeps_sp_imem_unknown_without_a_source_fact() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine
            .stage_generated_sp_imem_word_for_test(0, 0x1122_3344)
            .unwrap();
        assert_eq!(
            machine
                .sp_imem
                .read_known_u32_be(SpImemOffset::new(0))
                .unwrap()
                .value(),
            0x1122_3344
        );

        machine.stage_cartridge_bootstrap().unwrap();

        assert_eq!(
            machine
                .sp_imem
                .read_known_u32_be(SpImemOffset::new(0))
                .unwrap_err()
                .unknown_offset(),
            Some(SpImemOffset::new(0))
        );
        for offset in 0..4 {
            let observed = machine
                .sp_imem
                .observe_byte(SpImemOffset::new(offset))
                .unwrap();
            assert_eq!(observed.value(), 0);
            assert_eq!(observed.provenance(), SpImemByteProvenance::Unknown);
        }
    }

    #[test]
    fn accepted_pif_firmware_without_profile_is_owned_and_non_materializing() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        let firmware_bytes = generated_pif_firmware(0x50);
        let expected_state = MachinePifFirmwareState::Accepted {
            classification: PifFirmwareClassification::RawBootRom,
            size_bytes: PIF_BOOT_ROM_SIZE_BYTES,
        };

        assert_eq!(
            machine
                .install_pif_firmware(firmware_bytes.clone())
                .unwrap(),
            expected_state
        );
        assert_eq!(machine.pif_ipl2_profile(), None);

        let state = machine.stage_cartridge_bootstrap().unwrap();

        assert_eq!(state.pif_firmware_state(), expected_state);
        assert_eq!(state.pif_ipl2_profile(), None);
        assert_eq!(state.pif_ipl2_copy_layout(), None);
        assert_eq!(
            machine.pif_firmware_bytes_for_test(),
            Some(firmware_bytes.as_slice())
        );
        for destination_offset in 0..crate::sp_imem::SP_IMEM_SIZE_BYTES as u32 {
            let observed = machine
                .sp_imem
                .observe_byte(SpImemOffset::new(destination_offset))
                .unwrap();
            assert_eq!(observed.value(), 0);
            assert_eq!(observed.provenance(), SpImemByteProvenance::Unknown);
        }
    }

    #[test]
    fn pif_profile_without_firmware_rejects_bootstrap_before_mutation() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        let profile = PifIpl2Profile::PalPinned;
        machine.install_pif_ipl2_profile(profile);
        machine.stage_cpu_pc(0x8000_5000);
        machine.write_rdram_u32_be(0x50, 0x5566_7788).unwrap();
        machine
            .stage_generated_sp_imem_word_for_test(0, 0x99aa_bbcc)
            .unwrap();
        let before = architectural_snapshot(&machine);

        assert_eq!(
            machine.stage_cartridge_bootstrap(),
            Err(MachineCartridgeBootstrapError::PifIpl2ProfileRequiresFirmware { profile })
        );
        assert_eq!(architectural_snapshot(&machine), before);
    }

    #[test]
    fn pif_installation_orders_materialize_identically_across_reset_and_repeated_stage() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut firmware_first = Machine::from_cartridge(cartridge.clone());
        let mut profile_first = Machine::from_cartridge(cartridge);
        let profile = PifIpl2Profile::MpalPinned;
        let firmware_bytes = generated_pif_firmware(0x5b);

        firmware_first
            .install_pif_firmware(firmware_bytes.clone())
            .unwrap();
        assert_eq!(firmware_first.pif_ipl2_profile(), None);
        firmware_first.install_pif_ipl2_profile(profile);

        profile_first.install_pif_ipl2_profile(profile);
        assert_eq!(
            profile_first.pif_firmware_state(),
            MachinePifFirmwareState::Absent
        );
        profile_first
            .install_pif_firmware(firmware_bytes.clone())
            .unwrap();

        let layout = profile.copy_layout();
        assert_eq!(
            architectural_snapshot(&firmware_first),
            architectural_snapshot(&profile_first)
        );

        for _ in 0..2 {
            let firmware_first_state = firmware_first.stage_cartridge_bootstrap().unwrap();
            let profile_first_state = profile_first.stage_cartridge_bootstrap().unwrap();
            assert_eq!(firmware_first_state.pif_ipl2_profile(), Some(profile));
            assert_eq!(profile_first_state.pif_ipl2_profile(), Some(profile));
            assert_eq!(firmware_first_state.pif_ipl2_copy_layout(), Some(layout));
            assert_eq!(profile_first_state.pif_ipl2_copy_layout(), Some(layout));
            assert_eq!(
                architectural_snapshot(&firmware_first),
                architectural_snapshot(&profile_first)
            );
        }

        firmware_first.reset();
        profile_first.reset();
        for machine in [&firmware_first, &profile_first] {
            assert_eq!(machine.pif_ipl2_profile(), Some(profile));
            assert_eq!(
                machine.pif_firmware_bytes_for_test(),
                Some(firmware_bytes.as_slice())
            );
            assert_eq!(
                machine
                    .sp_imem
                    .read_known_u32_be(SpImemOffset::new(0))
                    .unwrap_err()
                    .unknown_offset(),
                Some(SpImemOffset::new(0))
            );
        }
        assert_eq!(
            architectural_snapshot(&firmware_first),
            architectural_snapshot(&profile_first)
        );

        firmware_first.stage_cartridge_bootstrap().unwrap();
        profile_first.stage_cartridge_bootstrap().unwrap();
        let observed = profile_first
            .sp_imem
            .observe_byte(SpImemOffset::new(0))
            .unwrap();
        assert_eq!(
            observed.value(),
            firmware_bytes[layout.source_start_offset() as usize]
        );
        assert_eq!(
            observed.provenance(),
            SpImemByteProvenance::UserSuppliedPifFirmware {
                profile,
                source_offset: layout.source_start_offset(),
            }
        );
        assert_eq!(
            architectural_snapshot(&firmware_first),
            architectural_snapshot(&profile_first)
        );
    }

    #[test]
    fn rejected_pif_firmware_replacement_preserves_full_coupled_handoff_state() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        let firmware_bytes = generated_pif_firmware(0x5d);
        machine
            .install_pif_firmware(firmware_bytes.clone())
            .unwrap();
        machine.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
        machine.install_pif_ipl3_family(MachinePifIpl3Family::X105);
        machine.install_pif_ipl2_handoff_reset_kind(MachinePifIpl2HandoffResetKind::Cold);
        machine.install_pif_ipl2_handoff_boot_medium(MachinePifIpl2HandoffBootMedium::Cartridge);
        machine.install_pif_version_bit(MachinePifVersionBit::One);
        machine.stage_cartridge_bootstrap().unwrap();
        machine.step().unwrap();
        machine.write_rdram_u32_be(0x60, 0xdead_beef).unwrap();
        let before = architectural_snapshot(&machine);

        let malformed = machine
            .install_pif_firmware(vec![0x6e; PIF_BOOT_ROM_SIZE_BYTES - 1])
            .unwrap_err();
        assert!(malformed.is_malformed());
        assert_eq!(architectural_snapshot(&machine), before);

        let unsupported = machine
            .install_pif_firmware(vec![
                0x7f;
                crate::pif_firmware::PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES
            ])
            .unwrap_err();
        assert!(unsupported.is_unsupported());
        assert_eq!(architectural_snapshot(&machine), before);
        assert_eq!(
            machine.pif_firmware_bytes_for_test(),
            Some(firmware_bytes.as_slice())
        );
    }

    #[test]
    fn profiled_pif_copy_is_byte_exact_complete_and_provenanced_for_every_profile() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let firmware_bytes = generated_pif_firmware(0x61);

        for profile in [
            PifIpl2Profile::NtscPinned,
            PifIpl2Profile::PalPinned,
            PifIpl2Profile::MpalPinned,
        ] {
            let mut machine = Machine::from_cartridge(cartridge.clone());
            let expected_state = MachinePifFirmwareState::Accepted {
                classification: PifFirmwareClassification::RawBootRom,
                size_bytes: PIF_BOOT_ROM_SIZE_BYTES,
            };
            assert_eq!(
                machine
                    .install_pif_firmware(firmware_bytes.clone())
                    .unwrap(),
                expected_state
            );
            assert_eq!(machine.install_pif_ipl2_profile(profile), profile);

            let state = machine.stage_cartridge_bootstrap().unwrap();
            let layout = profile.copy_layout();
            assert_eq!(state.pif_firmware_state(), expected_state);
            assert_eq!(state.pif_ipl2_profile(), Some(profile));
            assert_eq!(state.pif_ipl2_copy_layout(), Some(layout));
            assert_eq!(
                layout.byte_count(),
                layout.sp_imem_end_offset_exclusive() as usize
            );
            assert!(layout.byte_count() >= 0x02c);

            for destination_offset in 0..layout.byte_count() {
                let source_offset = layout.source_start_offset() + destination_offset as u32;
                let observed = machine
                    .sp_imem
                    .observe_byte(SpImemOffset::new(destination_offset as u32))
                    .unwrap();
                assert_eq!(observed.value(), firmware_bytes[source_offset as usize]);
                assert_eq!(
                    observed.provenance(),
                    SpImemByteProvenance::UserSuppliedPifFirmware {
                        profile,
                        source_offset,
                    }
                );
            }

            for destination_offset in
                layout.sp_imem_end_offset_exclusive()..crate::sp_imem::SP_IMEM_SIZE_BYTES as u32
            {
                let observed = machine
                    .sp_imem
                    .observe_byte(SpImemOffset::new(destination_offset))
                    .unwrap();
                assert_eq!(observed.value(), 0);
                assert_eq!(observed.provenance(), SpImemByteProvenance::Unknown);
            }
        }
    }

    #[test]
    fn profiled_pif_copy_survives_repeated_bootstrap_and_reset_rematerialization() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        let firmware_bytes = generated_pif_firmware(0x72);
        let profile = PifIpl2Profile::PalPinned;
        let expected_state = machine
            .install_pif_firmware(firmware_bytes.clone())
            .unwrap();
        machine.install_pif_ipl2_profile(profile);

        for _ in 0..2 {
            machine.stage_cartridge_bootstrap().unwrap();
            assert_eq!(machine.pif_firmware_state(), expected_state);
            assert_eq!(machine.pif_ipl2_profile(), Some(profile));
            assert_eq!(
                machine
                    .sp_imem
                    .observe_byte(SpImemOffset::new(0))
                    .unwrap()
                    .value(),
                firmware_bytes[profile.copy_layout().source_start_offset() as usize]
            );
        }

        machine.reset();

        assert_eq!(machine.pif_firmware_state(), expected_state);
        assert_eq!(machine.pif_ipl2_profile(), Some(profile));
        assert_eq!(
            machine.pif_firmware_bytes_for_test(),
            Some(firmware_bytes.as_slice())
        );
        assert_eq!(machine.cartridge_bootstrap_state(), None);
        assert_eq!(
            machine
                .sp_imem
                .read_known_u32_be(SpImemOffset::new(0))
                .unwrap_err()
                .unknown_offset(),
            Some(SpImemOffset::new(0))
        );

        let state = machine.stage_cartridge_bootstrap().unwrap();
        assert_eq!(state.pif_ipl2_copy_layout(), Some(profile.copy_layout()));
        assert_eq!(
            machine
                .sp_imem
                .observe_byte(SpImemOffset::new(0))
                .unwrap()
                .provenance(),
            SpImemByteProvenance::UserSuppliedPifFirmware {
                profile,
                source_offset: profile.copy_layout().source_start_offset(),
            }
        );
    }

    #[test]
    fn shorter_pif_profile_replacement_clears_stale_bytes_and_provenance() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        for longer_profile in [PifIpl2Profile::PalPinned, PifIpl2Profile::MpalPinned] {
            let mut machine = Machine::from_cartridge(cartridge.clone());
            machine
                .install_pif_firmware(generated_pif_firmware(0x83))
                .unwrap();
            machine.install_pif_ipl2_profile(longer_profile);
            machine.stage_cartridge_bootstrap().unwrap();
            let first_ntsc_untouched = PifIpl2Profile::NtscPinned
                .copy_layout()
                .sp_imem_end_offset_exclusive();
            assert!(machine
                .sp_imem
                .observe_byte(SpImemOffset::new(first_ntsc_untouched))
                .unwrap()
                .is_known());

            machine.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
            machine.stage_cartridge_bootstrap().unwrap();

            let observed = machine
                .sp_imem
                .observe_byte(SpImemOffset::new(first_ntsc_untouched))
                .unwrap();
            assert_eq!(observed.value(), 0);
            assert_eq!(observed.provenance(), SpImemByteProvenance::Unknown);
        }
    }

    #[test]
    fn profiled_pif_copy_remains_isolated_between_machine_instances() {
        let cartridge = load_cartridge(make_generated_normalized_boot_cartridge()).unwrap();
        let mut first = Machine::from_cartridge(cartridge.clone());
        let mut second = Machine::from_cartridge(cartridge);
        let first_bytes = generated_pif_firmware(0x15);
        let second_bytes = generated_pif_firmware(0xa6);
        first.install_pif_firmware(first_bytes.clone()).unwrap();
        first.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
        second.install_pif_firmware(second_bytes.clone()).unwrap();
        second.install_pif_ipl2_profile(PifIpl2Profile::PalPinned);
        first.stage_cartridge_bootstrap().unwrap();
        second.stage_cartridge_bootstrap().unwrap();

        let first_observed = first.sp_imem.observe_byte(SpImemOffset::new(0)).unwrap();
        let second_observed = second.sp_imem.observe_byte(SpImemOffset::new(0)).unwrap();
        assert_eq!(first_observed.value(), first_bytes[0x0d4]);
        assert_eq!(second_observed.value(), second_bytes[0x0d4]);
        assert_ne!(first_observed, second_observed);

        first.reset();

        assert!(!first
            .sp_imem
            .observe_byte(SpImemOffset::new(0))
            .unwrap()
            .is_known());
        assert_eq!(
            second.sp_imem.observe_byte(SpImemOffset::new(0)).unwrap(),
            second_observed
        );
    }

    #[test]
    fn machine_step_consumes_a_generated_profiled_pif_word_through_sp_imem() {
        let mut cartridge_bytes = make_generated_normalized_boot_cartridge();
        write_be_u32(
            &mut cartridge_bytes,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize,
            special_add_word(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, 0, 9),
        );
        write_be_u32(
            &mut cartridge_bytes,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize + 4,
            lw_word(9, 8, 0xf010),
        );
        let cartridge = load_cartridge(cartridge_bytes).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        let firmware_bytes = generated_pif_firmware(0xa5);
        let profile = PifIpl2Profile::NtscPinned;
        let source_start = profile.copy_layout().source_start_offset() as usize;
        assert_eq!(source_start, 0x0d4);
        assert_eq!(
            &firmware_bytes[source_start..source_start + 4],
            &[0x41, 0x6c, 0x97, 0xc2]
        );
        let loaded_word = u32::from_be_bytes(
            firmware_bytes[source_start..source_start + 4]
                .try_into()
                .unwrap(),
        );
        let result_value = (loaded_word as i32 as i64) as u64;
        assert_eq!(loaded_word, 0x416c_97c2);
        assert_eq!(result_value, 0x0000_0000_416c_97c2);
        machine.install_pif_firmware(firmware_bytes).unwrap();
        machine.install_pif_ipl2_profile(profile);
        machine.install_pif_ipl3_family(MachinePifIpl3Family::X105);
        machine.install_pif_ipl2_handoff_reset_kind(MachinePifIpl2HandoffResetKind::Cold);
        machine.install_pif_ipl2_handoff_boot_medium(MachinePifIpl2HandoffBootMedium::Cartridge);
        machine.install_pif_version_bit(MachinePifVersionBit::Zero);
        machine.stage_cartridge_bootstrap().unwrap();

        assert_eq!(
            machine
                .cartridge_bootstrap_state()
                .unwrap()
                .cpu_state_kind(),
            MachineBootstrapCpuStateKind::CoupledColdX105NtscPinned
        );

        assert_eq!(machine.cpu().pc(), 0xa400_0040);
        assert_eq!(machine.cpu().next_pc(), 0xa400_0044);
        assert_eq!(machine.cpu().cop0_count(), 0);
        machine.step().unwrap();
        assert_eq!(machine.cpu().pc(), 0xa400_0044);
        assert_eq!(machine.cpu().next_pc(), 0xa400_0048);
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert_eq!(
            machine.cartridge_bootstrap_state().unwrap().gpr_source(8),
            Some(MachineBootstrapGprSource::UnknownPifProduced)
        );
        let outcome = machine.step().unwrap();

        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::LoadWordCommitted {
                effective_address: 0xffff_ffff_a400_1000,
                target: crate::machine::MachineLoadWordTarget::SpImem { offset: 0 },
                destination_gpr: 8,
                loaded_word: actual_word,
                result_value: actual_value,
                ..
            } if actual_word == loaded_word && actual_value == result_value
        ));
        assert_eq!(machine.cpu().gpr(8), Some(result_value));
        assert_eq!(
            machine.cartridge_bootstrap_state().unwrap().gpr_source(8),
            Some(MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(0xa400_0044),
                identity: CpuInstructionIdentity::Lw,
                source_gpr_a: Some(9),
                source_gpr_b: None,
            })
        );
        assert_eq!(machine.cpu().pc(), 0xa400_0048);
        assert_eq!(machine.cpu().next_pc(), 0xa400_004c);
        assert_eq!(machine.cpu().cop0_count(), 2);
    }

    #[test]
    fn machine_cartridge_bootstrap_instruction_inspection_keeps_pif_reset_unavailable() {
        let machine = Machine::from_cartridge(crate::Cartridge::default());

        assert!(matches!(
            machine.inspect_current_cpu_instruction(),
            Err(MachineCpuInstructionFetchError::PifResetUnavailable {
                cpu_address
            }) if cpu_address == CpuAddress::new(NON_BOOT_RESET_VECTOR_PC)
        ));
    }

    #[test]
    fn machine_cartridge_bootstrap_unknown_sp_imem_frontier_preserves_all_state() {
        let mut bytes = make_generated_normalized_boot_cartridge();
        write_be_u32(
            &mut bytes,
            CARTRIDGE_CANDIDATE_IPL3_START_OFFSET as usize,
            0x8fa8_0000,
        );
        let cartridge = load_cartridge(bytes).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        machine.stage_cartridge_bootstrap().unwrap();
        let inspection = machine.inspect_current_cpu_instruction().unwrap();

        assert_eq!(inspection.identity(), CpuInstructionIdentity::Lw);
        let before = architectural_snapshot(&machine);
        let rejection = machine
            .step()
            .unwrap_err()
            .load_word_rejection()
            .expect("unknown SP IMEM should be an explicit Lw rejection");

        assert_eq!(
            rejection.target(),
            Some(crate::machine::MachineLoadWordTarget::SpImem { offset: 0xff0 })
        );
        assert_eq!(architectural_snapshot(&machine), before);
    }
}
