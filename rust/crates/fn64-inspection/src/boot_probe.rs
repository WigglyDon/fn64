use core::fmt;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::PathBuf;

use fn64_core::{
    load_cartridge, rom_source_layout_name, CartridgeLoadError, CpuInstructionIdentity, Machine,
    MachineBootstrapControlFlowSource, MachineBootstrapCop0StatusSource, MachineBootstrapGprSource,
    MachineCartridgeBootstrapError, MachineCpuInstructionInspection, MachineCpuInstructionSource,
    MachineLoadWordRejection, MachineLoadWordRejectionReason, MachineLoadWordTarget,
    MachinePifFirmwareState, MachinePifIpl2HandoffBootMedium, MachinePifIpl2HandoffResetKind,
    MachinePifIpl3Family, MachinePifVersionBit, MachineRepresentedStepError,
    MachineRepresentedStepOutcome, MachineSpDmemInstructionProvenance, PifFirmwareValidationError,
    PifIpl2Profile, RomMetadata, RomSourceLayout, CPU_GPR_COUNT,
    MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX,
    MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, MACHINE_PIF_IPL2_HANDOFF_SP_VALUE,
    MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX,
};

pub const DEFAULT_BOOT_PROBE_MAX_STEPS: u64 = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BootCheckpoint {
    Boot0,
    Boot1,
    Boot2,
    Boot3,
    Boot4,
}

impl BootCheckpoint {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Boot0 => "BOOT-0",
            Self::Boot1 => "BOOT-1",
            Self::Boot2 => "BOOT-2",
            Self::Boot3 => "BOOT-3",
            Self::Boot4 => "BOOT-4",
        }
    }

    pub const fn definition(self) -> &'static str {
        match self {
            Self::Boot0 => "private cartridge input structurally validated",
            Self::Boot1 => "machine-owned bootstrap payload and CPU entry state staged",
            Self::Boot2 => {
                "ROM-derived instruction committed its complete known-operand architectural result"
            }
            Self::Boot3 => "machine behavior reached the cartridge-declared program entry",
            Self::Boot4 => "program instruction after bootstrap handoff executed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootProbeArguments {
    input_path: PathBuf,
    pif_rom_path: Option<PathBuf>,
    pif_profile: Option<PifIpl2Profile>,
    ipl3_family: Option<MachinePifIpl3Family>,
    reset_kind: Option<MachinePifIpl2HandoffResetKind>,
    boot_medium: Option<MachinePifIpl2HandoffBootMedium>,
    pif_version_bit: Option<MachinePifVersionBit>,
    max_steps: u64,
}

impl BootProbeArguments {
    pub fn input_path(&self) -> &PathBuf {
        &self.input_path
    }

    pub fn pif_rom_path(&self) -> Option<&PathBuf> {
        self.pif_rom_path.as_ref()
    }

    pub const fn pif_profile(&self) -> Option<PifIpl2Profile> {
        self.pif_profile
    }

    pub const fn ipl3_family(&self) -> Option<MachinePifIpl3Family> {
        self.ipl3_family
    }

    pub const fn reset_kind(&self) -> Option<MachinePifIpl2HandoffResetKind> {
        self.reset_kind
    }

    pub const fn boot_medium(&self) -> Option<MachinePifIpl2HandoffBootMedium> {
        self.boot_medium
    }

    pub const fn pif_version_bit(&self) -> Option<MachinePifVersionBit> {
        self.pif_version_bit
    }

    pub const fn max_steps(&self) -> u64 {
        self.max_steps
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootProbeArgumentError {
    Usage,
    InvalidMaxSteps,
    MissingPifRomPath,
    MissingPifProfile,
    PifProfileRequiresRom,
    UnsupportedPifProfile(String),
    MissingIpl3Family,
    UnsupportedIpl3Family(String),
    MissingResetKind,
    UnsupportedResetKind(String),
    MissingBootMedium,
    UnsupportedBootMedium(String),
    MissingPifVersionBit,
    UnsupportedPifVersionBit(String),
}

impl fmt::Display for BootProbeArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => write!(
                f,
                "usage: fn64_boot_probe <rom-path> [--pif-rom <path>] [--pif-profile <ntsc-pinned|pal-pinned|mpal-pinned>] [--ipl3-family <x105>] [--reset-kind <cold>] [--boot-medium <cartridge>] [--pif-version-bit <0|1>] [--max-steps <positive-integer>]"
            ),
            Self::InvalidMaxSteps => write!(f, "--max-steps requires a positive integer"),
            Self::MissingPifRomPath => write!(f, "--pif-rom requires an explicit path"),
            Self::MissingPifProfile => write!(f, "--pif-profile requires an explicit profile"),
            Self::PifProfileRequiresRom => {
                write!(f, "--pif-profile requires an explicit --pif-rom path")
            }
            Self::UnsupportedPifProfile(value) => write!(
                f,
                "unsupported PIF IPL2 profile: {value}; expected ntsc-pinned, pal-pinned, or mpal-pinned"
            ),
            Self::MissingIpl3Family => write!(f, "--ipl3-family requires an explicit family"),
            Self::UnsupportedIpl3Family(value) => {
                write!(f, "unsupported IPL3 family: {value}; expected x105")
            }
            Self::MissingResetKind => write!(f, "--reset-kind requires an explicit kind"),
            Self::UnsupportedResetKind(value) => {
                write!(f, "unsupported reset kind: {value}; expected cold")
            }
            Self::MissingBootMedium => write!(f, "--boot-medium requires an explicit medium"),
            Self::UnsupportedBootMedium(value) => {
                write!(f, "unsupported boot medium: {value}; expected cartridge")
            }
            Self::MissingPifVersionBit => {
                write!(f, "--pif-version-bit requires an explicit bit")
            }
            Self::UnsupportedPifVersionBit(value) => {
                write!(f, "unsupported PIF version bit: {value}; expected 0 or 1")
            }
        }
    }
}

impl std::error::Error for BootProbeArgumentError {}

fn parse_pif_ipl2_profile(value: &str) -> Option<PifIpl2Profile> {
    match value {
        "ntsc-pinned" => Some(PifIpl2Profile::NtscPinned),
        "pal-pinned" => Some(PifIpl2Profile::PalPinned),
        "mpal-pinned" => Some(PifIpl2Profile::MpalPinned),
        _ => None,
    }
}

fn is_boot_probe_flag(value: &OsString) -> bool {
    [
        "--pif-rom",
        "--pif-profile",
        "--ipl3-family",
        "--reset-kind",
        "--boot-medium",
        "--pif-version-bit",
        "--max-steps",
    ]
    .iter()
    .any(|flag| value == flag)
}

pub fn parse_boot_probe_arguments<I>(
    arguments: I,
) -> Result<BootProbeArguments, BootProbeArgumentError>
where
    I: IntoIterator<Item = OsString>,
{
    let mut arguments = arguments.into_iter();
    let input_path = arguments
        .next()
        .filter(|value| !is_boot_probe_flag(value))
        .ok_or(BootProbeArgumentError::Usage)?;
    let mut pif_rom_path = None;
    let mut pif_profile = None;
    let mut ipl3_family = None;
    let mut reset_kind = None;
    let mut boot_medium = None;
    let mut pif_version_bit = None;
    let mut max_steps = DEFAULT_BOOT_PROBE_MAX_STEPS;
    let mut max_steps_seen = false;

    while let Some(flag) = arguments.next() {
        if flag == "--pif-rom" {
            if pif_rom_path.is_some() {
                return Err(BootProbeArgumentError::Usage);
            }
            let value = arguments
                .next()
                .filter(|value| !value.is_empty() && !is_boot_probe_flag(value))
                .ok_or(BootProbeArgumentError::MissingPifRomPath)?;
            pif_rom_path = Some(PathBuf::from(value));
        } else if flag == "--pif-profile" {
            if pif_profile.is_some() {
                return Err(BootProbeArgumentError::Usage);
            }
            let value = arguments
                .next()
                .filter(|value| !value.is_empty() && !is_boot_probe_flag(value))
                .ok_or(BootProbeArgumentError::MissingPifProfile)?;
            let value = value
                .to_str()
                .ok_or_else(|| BootProbeArgumentError::UnsupportedPifProfile("non-UTF-8".into()))?;
            pif_profile =
                Some(parse_pif_ipl2_profile(value).ok_or_else(|| {
                    BootProbeArgumentError::UnsupportedPifProfile(value.to_owned())
                })?);
        } else if flag == "--ipl3-family" {
            if ipl3_family.is_some() {
                return Err(BootProbeArgumentError::Usage);
            }
            let value = arguments
                .next()
                .filter(|value| !value.is_empty() && !is_boot_probe_flag(value))
                .ok_or(BootProbeArgumentError::MissingIpl3Family)?;
            let value = value
                .to_str()
                .ok_or_else(|| BootProbeArgumentError::UnsupportedIpl3Family("non-UTF-8".into()))?;
            ipl3_family = Some(match value {
                "x105" => MachinePifIpl3Family::X105,
                _ => return Err(BootProbeArgumentError::UnsupportedIpl3Family(value.into())),
            });
        } else if flag == "--reset-kind" {
            if reset_kind.is_some() {
                return Err(BootProbeArgumentError::Usage);
            }
            let value = arguments
                .next()
                .filter(|value| !value.is_empty() && !is_boot_probe_flag(value))
                .ok_or(BootProbeArgumentError::MissingResetKind)?;
            let value = value
                .to_str()
                .ok_or_else(|| BootProbeArgumentError::UnsupportedResetKind("non-UTF-8".into()))?;
            reset_kind = Some(match value {
                "cold" => MachinePifIpl2HandoffResetKind::Cold,
                _ => return Err(BootProbeArgumentError::UnsupportedResetKind(value.into())),
            });
        } else if flag == "--boot-medium" {
            if boot_medium.is_some() {
                return Err(BootProbeArgumentError::Usage);
            }
            let value = arguments
                .next()
                .filter(|value| !value.is_empty() && !is_boot_probe_flag(value))
                .ok_or(BootProbeArgumentError::MissingBootMedium)?;
            let value = value
                .to_str()
                .ok_or_else(|| BootProbeArgumentError::UnsupportedBootMedium("non-UTF-8".into()))?;
            boot_medium = Some(match value {
                "cartridge" => MachinePifIpl2HandoffBootMedium::Cartridge,
                _ => return Err(BootProbeArgumentError::UnsupportedBootMedium(value.into())),
            });
        } else if flag == "--pif-version-bit" {
            if pif_version_bit.is_some() {
                return Err(BootProbeArgumentError::Usage);
            }
            let value = arguments
                .next()
                .filter(|value| !value.is_empty() && !is_boot_probe_flag(value))
                .ok_or(BootProbeArgumentError::MissingPifVersionBit)?;
            let value = value.to_str().ok_or_else(|| {
                BootProbeArgumentError::UnsupportedPifVersionBit("non-UTF-8".into())
            })?;
            pif_version_bit = Some(match value {
                "0" => MachinePifVersionBit::Zero,
                "1" => MachinePifVersionBit::One,
                _ => {
                    return Err(BootProbeArgumentError::UnsupportedPifVersionBit(
                        value.into(),
                    ))
                }
            });
        } else if flag == "--max-steps" {
            if max_steps_seen {
                return Err(BootProbeArgumentError::Usage);
            }
            let value = arguments
                .next()
                .ok_or(BootProbeArgumentError::InvalidMaxSteps)?;
            max_steps = value
                .to_str()
                .and_then(|value| value.parse::<u64>().ok())
                .filter(|value| *value > 0)
                .ok_or(BootProbeArgumentError::InvalidMaxSteps)?;
            max_steps_seen = true;
        } else {
            return Err(BootProbeArgumentError::Usage);
        }
    }

    if pif_rom_path.is_none() && pif_profile.is_some() {
        return Err(BootProbeArgumentError::PifProfileRequiresRom);
    }

    Ok(BootProbeArguments {
        input_path: PathBuf::from(input_path),
        pif_rom_path,
        pif_profile,
        ipl3_family,
        reset_kind,
        boot_medium,
        pif_version_bit,
        max_steps,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootProbeReport {
    output: String,
    highest_checkpoint: BootCheckpoint,
    attempted_steps: u64,
    committed_steps: u64,
    first_frontier: String,
}

impl BootProbeReport {
    pub fn output(&self) -> &str {
        &self.output
    }

    pub const fn highest_checkpoint(&self) -> BootCheckpoint {
        self.highest_checkpoint
    }

    pub const fn attempted_steps(&self) -> u64 {
        self.attempted_steps
    }

    pub const fn committed_steps(&self) -> u64 {
        self.committed_steps
    }

    pub fn first_frontier(&self) -> &str {
        &self.first_frontier
    }

    pub const fn expected_frontier_exit_status(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootProbeError {
    InvalidStepBudget,
    CartridgeLoad(CartridgeLoadError),
    PifFirmwareValidation(PifFirmwareValidationError),
    Bootstrap(MachineCartridgeBootstrapError),
    MachineInvariant {
        attempted_step: u64,
        source: MachineRepresentedStepError,
    },
}

impl fmt::Display for BootProbeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStepBudget => write!(f, "fixed step budget must be positive"),
            Self::CartridgeLoad(error) => write!(f, "structural cartridge input rejected: {error}"),
            Self::PifFirmwareValidation(error) => {
                write!(f, "PIF firmware input rejected: {error}")
            }
            Self::Bootstrap(error) => write!(f, "machine bootstrap staging rejected: {error}"),
            Self::MachineInvariant {
                attempted_step,
                source,
            } => write!(
                f,
                "machine invariant failed at attempted step {}: {}",
                attempted_step, source
            ),
        }
    }
}

impl std::error::Error for BootProbeError {}

impl BootProbeError {
    pub const fn exit_status(&self) -> u8 {
        1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CpuObservation {
    pc: u32,
    next_pc: u32,
    count: u32,
    status: u32,
    delay_slot_active: bool,
    hi: u64,
    lo: u64,
    gprs: [u64; CPU_GPR_COUNT],
    gpr_sources: [MachineBootstrapGprSource; CPU_GPR_COUNT],
}

impl CpuObservation {
    fn capture(machine: &Machine) -> Self {
        let mut gprs = [0; CPU_GPR_COUNT];
        for (index, destination) in gprs.iter_mut().enumerate() {
            *destination = machine
                .cpu()
                .gpr(index)
                .expect("fixed GPR observation index must be represented");
        }
        let gpr_sources = match machine.cartridge_bootstrap_state() {
            Some(state) => core::array::from_fn(|index| {
                state
                    .gpr_source(index)
                    .expect("fixed GPR source observation index must be represented")
            }),
            None => [MachineBootstrapGprSource::UnknownPifProduced; CPU_GPR_COUNT],
        };
        Self {
            pc: machine.cpu().pc(),
            next_pc: machine.cpu().next_pc(),
            count: machine.cpu().cop0_count(),
            status: machine.cpu().cop0_status(),
            delay_slot_active: machine.cpu_delay_slot_context().is_some(),
            hi: machine.cpu().hi(),
            lo: machine.cpu().lo(),
            gprs,
            gpr_sources,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LastCommittedStep {
    inspection: MachineCpuInstructionInspection,
    outcome: &'static str,
    effect: String,
    gpr_effect: Option<CommittedGprEffect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CommittedGprEffect {
    index: usize,
    old_value: u64,
    new_value: u64,
    old_source: MachineBootstrapGprSource,
    new_source: MachineBootstrapGprSource,
}

pub fn run_boot_probe(
    owned_input_bytes: Vec<u8>,
    input_path: &str,
    max_steps: u64,
) -> Result<BootProbeReport, BootProbeError> {
    run_boot_probe_with_pif_firmware_and_handoff(
        owned_input_bytes,
        input_path,
        None,
        None,
        None,
        None,
        None,
        None,
        max_steps,
    )
}

pub fn run_boot_probe_with_pif_firmware(
    owned_input_bytes: Vec<u8>,
    input_path: &str,
    owned_pif_firmware: Option<Vec<u8>>,
    pif_profile: Option<PifIpl2Profile>,
    max_steps: u64,
) -> Result<BootProbeReport, BootProbeError> {
    run_boot_probe_with_pif_firmware_and_handoff(
        owned_input_bytes,
        input_path,
        owned_pif_firmware,
        pif_profile,
        None,
        None,
        None,
        None,
        max_steps,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn run_boot_probe_with_pif_firmware_and_handoff(
    owned_input_bytes: Vec<u8>,
    input_path: &str,
    owned_pif_firmware: Option<Vec<u8>>,
    pif_profile: Option<PifIpl2Profile>,
    ipl3_family: Option<MachinePifIpl3Family>,
    reset_kind: Option<MachinePifIpl2HandoffResetKind>,
    boot_medium: Option<MachinePifIpl2HandoffBootMedium>,
    pif_version_bit: Option<MachinePifVersionBit>,
    max_steps: u64,
) -> Result<BootProbeReport, BootProbeError> {
    if max_steps == 0 {
        return Err(BootProbeError::InvalidStepBudget);
    }

    let cartridge = load_cartridge(owned_input_bytes).map_err(BootProbeError::CartridgeLoad)?;
    let source_layout = cartridge.source_layout();
    let metadata = cartridge.metadata().clone();
    let input_size_bytes = cartridge.size_bytes();
    let mut machine = Machine::from_cartridge(cartridge);
    if let Some(owned_bytes) = owned_pif_firmware {
        machine
            .install_pif_firmware(owned_bytes)
            .map_err(BootProbeError::PifFirmwareValidation)?;
    }
    if let Some(profile) = pif_profile {
        machine.install_pif_ipl2_profile(profile);
    }
    if let Some(family) = ipl3_family {
        machine.install_pif_ipl3_family(family);
    }
    if let Some(kind) = reset_kind {
        machine.install_pif_ipl2_handoff_reset_kind(kind);
    }
    if let Some(medium) = boot_medium {
        machine.install_pif_ipl2_handoff_boot_medium(medium);
    }
    if let Some(bit) = pif_version_bit {
        machine.install_pif_version_bit(bit);
    }
    let staging = machine
        .stage_cartridge_bootstrap()
        .map_err(BootProbeError::Bootstrap)?;
    let staging_observation = CpuObservation::capture(&machine);
    let mut highest_checkpoint = BootCheckpoint::Boot1;

    let mut attempted_steps = 0;
    let mut committed_steps = 0;
    let mut last_committed_step = None;
    let mut last_represented_outcome = "none";
    let mut first_frontier = None;
    let mut cartridge_entry_reached = false;
    let mut game_program_instruction_ran = false;

    while attempted_steps < max_steps {
        let inspection = machine.inspect_current_cpu_instruction().ok();
        let before = CpuObservation::capture(&machine);
        let entered_game_program = cartridge_entry_reached;
        attempted_steps += 1;

        match machine.step() {
            Ok(outcome) => {
                let after = CpuObservation::capture(&machine);
                last_represented_outcome = represented_outcome_name(outcome);

                if is_committed_instruction(outcome) {
                    committed_steps += 1;
                    if let Some(inspection) = inspection {
                        if instruction_is_cartridge_derived(inspection) {
                            highest_checkpoint = highest_checkpoint.max(BootCheckpoint::Boot2);
                        }
                        last_committed_step = Some(LastCommittedStep {
                            inspection,
                            outcome: represented_outcome_name(outcome),
                            effect: format_cpu_effect(&before, &after),
                            gpr_effect: committed_gpr_effect(&before, &after),
                        });
                    }

                    if entered_game_program {
                        game_program_instruction_ran = true;
                        highest_checkpoint = highest_checkpoint.max(BootCheckpoint::Boot4);
                    }
                    if after.pc == metadata.entry_point {
                        cartridge_entry_reached = true;
                        highest_checkpoint = highest_checkpoint.max(BootCheckpoint::Boot3);
                    }
                }

                match outcome {
                    MachineRepresentedStepOutcome::CpuLocalCommitted { .. }
                    | MachineRepresentedStepOutcome::LoadWordCommitted { .. }
                    | MachineRepresentedStepOutcome::NoEffectCommitted { .. } => {}
                    MachineRepresentedStepOutcome::DataAddressError { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-data-address-error",
                            inspection,
                            outcome.identity(),
                            &before,
                        ));
                        break;
                    }
                    MachineRepresentedStepOutcome::Stopped { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-stop",
                            inspection,
                            outcome.identity(),
                            &before,
                        ));
                        break;
                    }
                    MachineRepresentedStepOutcome::Unsupported { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-unsupported-instruction",
                            inspection,
                            outcome.identity(),
                            &before,
                        ));
                        break;
                    }
                    MachineRepresentedStepOutcome::ArithmeticOverflowException { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-arithmetic-overflow-exception",
                            inspection,
                            outcome.identity(),
                            &before,
                        ));
                        break;
                    }
                    MachineRepresentedStepOutcome::InstructionFetchAddressError { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-instruction-fetch-address-error",
                            inspection,
                            None,
                            &before,
                        ));
                        break;
                    }
                }
            }
            Err(error @ MachineRepresentedStepError::UnrepresentedInstruction { .. }) => {
                first_frontier = Some(format_frontier(
                    "unrepresented-instruction",
                    inspection,
                    error.identity(),
                    &before,
                ));
                break;
            }
            Err(error @ MachineRepresentedStepError::BootstrapCpuStateUnavailable(_)) => {
                last_represented_outcome = "bootstrap-cpu-state-unavailable";
                first_frontier = Some(format_bootstrap_cpu_state_frontier(
                    inspection,
                    error
                        .bootstrap_cpu_state_unavailable()
                        .expect("matched bootstrap state rejection must retain its detail"),
                    &before,
                ));
                break;
            }
            Err(error @ MachineRepresentedStepError::LoadWordRejected(_)) => {
                last_represented_outcome = "load-word-rejected";
                first_frontier = Some(format_load_word_rejection_frontier(
                    inspection,
                    error
                        .load_word_rejection()
                        .expect("matched Lw rejection must retain its detail"),
                    &before,
                ));
                break;
            }
            Err(error @ MachineRepresentedStepError::FetchRejected(_)) => {
                first_frontier = Some(format!(
                    "fetch-rejected address=0x{:08X} detail={}",
                    before.pc, error
                ));
                break;
            }
            Err(source) => {
                return Err(BootProbeError::MachineInvariant {
                    attempted_step: attempted_steps,
                    source,
                });
            }
        }
    }

    let first_frontier = first_frontier.unwrap_or_else(|| {
        format!(
            "explicit-step-budget address=0x{:08X} budget={}",
            machine.cpu().pc(),
            max_steps
        )
    });
    let output = format_report(ReportFacts {
        input_path,
        input_size_bytes,
        source_layout,
        metadata: &metadata,
        max_steps,
        attempted_steps,
        committed_steps,
        highest_checkpoint,
        staging,
        pif_firmware_state: staging.pif_firmware_state(),
        staging_observation,
        last_committed_step: last_committed_step.as_ref(),
        last_represented_outcome,
        first_frontier: &first_frontier,
        final_observation: CpuObservation::capture(&machine),
        cartridge_entry_reached,
        game_program_instruction_ran,
    });

    Ok(BootProbeReport {
        output,
        highest_checkpoint,
        attempted_steps,
        committed_steps,
        first_frontier,
    })
}

fn is_committed_instruction(outcome: MachineRepresentedStepOutcome) -> bool {
    matches!(
        outcome,
        MachineRepresentedStepOutcome::CpuLocalCommitted { .. }
            | MachineRepresentedStepOutcome::LoadWordCommitted { .. }
            | MachineRepresentedStepOutcome::NoEffectCommitted { .. }
    )
}

fn instruction_is_cartridge_derived(inspection: MachineCpuInstructionInspection) -> bool {
    matches!(
        inspection.source(),
        MachineCpuInstructionSource::SpDmem {
            provenance: MachineSpDmemInstructionProvenance::CartridgeBootstrap { .. },
            ..
        }
    )
}

fn represented_outcome_name(outcome: MachineRepresentedStepOutcome) -> &'static str {
    match outcome {
        MachineRepresentedStepOutcome::CpuLocalCommitted { .. } => "cpu-local-committed",
        MachineRepresentedStepOutcome::LoadWordCommitted { .. } => "load-word-committed",
        MachineRepresentedStepOutcome::DataAddressError { .. } => "data-address-error",
        MachineRepresentedStepOutcome::ArithmeticOverflowException { .. } => {
            "arithmetic-overflow-exception"
        }
        MachineRepresentedStepOutcome::NoEffectCommitted { .. } => "no-effect-committed",
        MachineRepresentedStepOutcome::Stopped { .. } => "stopped",
        MachineRepresentedStepOutcome::Unsupported { .. } => "unsupported",
        MachineRepresentedStepOutcome::InstructionFetchAddressError { .. } => {
            "instruction-fetch-address-error"
        }
    }
}

fn format_cpu_effect(before: &CpuObservation, after: &CpuObservation) -> String {
    let mut gpr_changes = Vec::new();
    for index in 0..CPU_GPR_COUNT {
        if before.gprs[index] != after.gprs[index]
            || before.gpr_sources[index] != after.gpr_sources[index]
        {
            gpr_changes.push(format!(
                "r{}=0x{:016X}->0x{:016X} known={}->{} source={}->{}",
                index,
                before.gprs[index],
                after.gprs[index],
                yes_no(before.gpr_sources[index].is_known()),
                yes_no(after.gpr_sources[index].is_known()),
                format_gpr_source(before.gpr_sources[index]),
                format_gpr_source(after.gpr_sources[index])
            ));
        }
    }
    let gpr_changes = if gpr_changes.is_empty() {
        "none".to_owned()
    } else {
        gpr_changes.join(",")
    };

    format!(
        "pc=0x{:08X}->0x{:08X} next_pc=0x{:08X}->0x{:08X} count={}->{} hi=0x{:016X}->0x{:016X} lo=0x{:016X}->0x{:016X} gpr_mutations={}",
        before.pc,
        after.pc,
        before.next_pc,
        after.next_pc,
        before.count,
        after.count,
        before.hi,
        after.hi,
        before.lo,
        after.lo,
        gpr_changes
    )
}

fn committed_gpr_effect(
    before: &CpuObservation,
    after: &CpuObservation,
) -> Option<CommittedGprEffect> {
    (0..CPU_GPR_COUNT)
        .find(|index| {
            before.gprs[*index] != after.gprs[*index]
                || before.gpr_sources[*index] != after.gpr_sources[*index]
        })
        .map(|index| CommittedGprEffect {
            index,
            old_value: before.gprs[index],
            new_value: after.gprs[index],
            old_source: before.gpr_sources[index],
            new_source: after.gpr_sources[index],
        })
}

fn format_frontier(
    classification: &str,
    inspection: Option<MachineCpuInstructionInspection>,
    identity: Option<CpuInstructionIdentity>,
    before: &CpuObservation,
) -> String {
    match inspection {
        Some(inspection) => {
            let identity = identity.unwrap_or_else(|| inspection.identity());
            let mut result = format!(
                "{} address=0x{:08X} identity={:?} rs={} rt={} rd={} immediate=0x{:04X} source={}",
                classification,
                inspection.cpu_address().value(),
                identity,
                inspection.fields().rs(),
                inspection.fields().rt(),
                inspection.fields().rd(),
                inspection.fields().immediate_u16(),
                format_instruction_source(inspection.source())
            );
            if identity == CpuInstructionIdentity::Lw {
                let base_index = usize::from(inspection.fields().rs());
                let base_source = before.gpr_sources[base_index];
                if base_source.is_known() {
                    let base_value = before.gprs[base_index];
                    let signed_immediate = i64::from(inspection.fields().immediate_u16() as i16);
                    let effective_address = base_value.wrapping_add_signed(signed_immediate);
                    write!(
                        result,
                        " base_known=yes base_value=0x{:016X} base_source={} effective_address=0x{:016X} effective_cpu_address=0x{:08X}",
                        base_value,
                        format_gpr_source(base_source),
                        effective_address,
                        effective_address as u32
                    )
                    .unwrap();
                } else {
                    write!(
                        result,
                        " base_known=no base_source={} effective_address=unavailable",
                        format_gpr_source(base_source)
                    )
                    .unwrap();
                }
            }
            result
        }
        None => format!(
            "{} identity={:?} source=unavailable",
            classification, identity
        ),
    }
}

fn format_load_word_rejection_frontier(
    inspection: Option<MachineCpuInstructionInspection>,
    rejection: MachineLoadWordRejection,
    before: &CpuObservation,
) -> String {
    let target = match rejection.target() {
        Some(MachineLoadWordTarget::DirectRdram { offset }) => {
            format!("direct-rdram offset=0x{:08X}", offset.value())
        }
        Some(MachineLoadWordTarget::SpImem { offset }) => {
            format!("sp-imem offset=0x{offset:08X}")
        }
        None => "unclassified".to_owned(),
    };
    let reason = match rejection.reason() {
        MachineLoadWordRejectionReason::NonDirectUnsupported => "non-direct-unsupported".to_owned(),
        MachineLoadWordRejectionReason::DirectTargetMiss => "direct-target-miss".to_owned(),
        MachineLoadWordRejectionReason::DirectRdramReadRejected => {
            "direct-rdram-read-rejected".to_owned()
        }
        MachineLoadWordRejectionReason::SpImemUnknown {
            first_unknown_offset,
        } => format!("sp-imem-unknown first_unknown_offset=0x{first_unknown_offset:08X}"),
        MachineLoadWordRejectionReason::SpImemReadRejected => "sp-imem-read-rejected".to_owned(),
    };

    format!(
        "{} target={} reason={} rejected_before_mutation=yes",
        format_frontier(
            "load-word-rejected",
            inspection,
            Some(CpuInstructionIdentity::Lw),
            before,
        ),
        target,
        reason
    )
}

fn format_bootstrap_cpu_state_frontier(
    inspection: Option<MachineCpuInstructionInspection>,
    unavailable: fn64_core::MachineBootstrapCpuStateUnavailable,
    before: &CpuObservation,
) -> String {
    format!(
        "{} unknown_gpr={} unknown_source={} rejected_before_mutation=yes",
        format_frontier(
            "bootstrap-cpu-state-unavailable",
            inspection,
            Some(unavailable.identity()),
            before,
        ),
        unavailable.register_index(),
        format_gpr_source(unavailable.source())
    )
}

fn format_gpr_source(source: MachineBootstrapGprSource) -> String {
    match source {
        MachineBootstrapGprSource::UnknownPifProduced => "unknown-pif-produced".to_owned(),
        MachineBootstrapGprSource::ArchitecturalZero => "architectural-zero".to_owned(),
        MachineBootstrapGprSource::PifIpl2HandoffEntryPointer => {
            "pif-ipl2-handoff-entry-pointer".to_owned()
        }
        MachineBootstrapGprSource::PifIpl2RestoredStackPointer => {
            "pif-ipl2-restored-stack-pointer".to_owned()
        }
        MachineBootstrapGprSource::PifIpl2RetainedLink {
            profile,
            link_instruction_address,
        } => format!(
            "pif-ipl2-retained-link(profile={},instruction=0x{:08X})",
            profile.name(),
            link_instruction_address.value()
        ),
        MachineBootstrapGprSource::CartridgeBootMedium => "cartridge-boot-medium".to_owned(),
        MachineBootstrapGprSource::PifProfileTvType { profile } => {
            format!("pif-profile-tv-type(profile={})", profile.name())
        }
        MachineBootstrapGprSource::ColdResetKind => "cold-reset-kind".to_owned(),
        MachineBootstrapGprSource::X105Seed => "x105-seed".to_owned(),
        MachineBootstrapGprSource::PifVersionRegionalState {
            profile,
            pif_version_bit,
        } => format!(
            "pif-version-regional-state(profile={},version-bit={})",
            profile.name(),
            pif_version_bit.value()
        ),
        MachineBootstrapGprSource::KnownInstructionResult {
            execution_address,
            identity,
            source_gpr_a,
            source_gpr_b,
        } => format!(
            "known-instruction-result(address=0x{:08X},identity={:?},source_a={},source_b={})",
            execution_address.value(),
            identity,
            format_optional_gpr(source_gpr_a),
            format_optional_gpr(source_gpr_b)
        ),
    }
}

fn format_optional_gpr(index: Option<u8>) -> String {
    match index {
        Some(index) => format!("r{index}"),
        None => "none".to_owned(),
    }
}

fn format_instruction_source(source: MachineCpuInstructionSource) -> String {
    match source {
        MachineCpuInstructionSource::DirectRdram { offset } => {
            format!("direct-rdram offset=0x{:08X}", offset.value())
        }
        MachineCpuInstructionSource::SpDmem {
            offset,
            provenance: MachineSpDmemInstructionProvenance::CartridgeBootstrap { cartridge_offset },
        } => format!(
            "cartridge-bootstrap cartridge_offset=0x{:08X} sp_dmem_offset=0x{:08X}",
            cartridge_offset,
            offset.value()
        ),
        MachineCpuInstructionSource::SpDmem {
            offset,
            provenance: MachineSpDmemInstructionProvenance::UnclassifiedMachineStorage,
        } => format!("unclassified-sp-dmem offset=0x{:08X}", offset.value()),
    }
}

struct ReportFacts<'a> {
    input_path: &'a str,
    input_size_bytes: usize,
    source_layout: RomSourceLayout,
    metadata: &'a RomMetadata,
    max_steps: u64,
    attempted_steps: u64,
    committed_steps: u64,
    highest_checkpoint: BootCheckpoint,
    staging: fn64_core::MachineCartridgeBootstrapState,
    pif_firmware_state: MachinePifFirmwareState,
    staging_observation: CpuObservation,
    last_committed_step: Option<&'a LastCommittedStep>,
    last_represented_outcome: &'static str,
    first_frontier: &'a str,
    final_observation: CpuObservation,
    cartridge_entry_reached: bool,
    game_program_instruction_ran: bool,
}

fn format_report(facts: ReportFacts<'_>) -> String {
    let mut output = String::new();
    writeln!(output, "fn64 boot probe").unwrap();
    writeln!(output, "input_path: {}", facts.input_path).unwrap();
    writeln!(output, "input_size_bytes: {}", facts.input_size_bytes).unwrap();
    writeln!(
        output,
        "source_byte_order: {}",
        rom_source_layout_name(facts.source_layout)
    )
    .unwrap();
    writeln!(output, "structural_header: valid").unwrap();
    writeln!(output, "structurally_usable: yes").unwrap();
    match facts.pif_firmware_state {
        MachinePifFirmwareState::Absent => {
            writeln!(output, "pif_firmware_input: absent").unwrap();
            writeln!(output, "pif_firmware_classification: unavailable").unwrap();
            writeln!(output, "pif_firmware_size_bytes: unavailable").unwrap();
        }
        MachinePifFirmwareState::Accepted {
            classification,
            size_bytes,
        } => {
            writeln!(output, "pif_firmware_input: accepted").unwrap();
            writeln!(
                output,
                "pif_firmware_classification: {}",
                classification.name()
            )
            .unwrap();
            writeln!(output, "pif_firmware_size_bytes: {size_bytes}").unwrap();
        }
    }
    match facts.staging.pif_ipl2_profile() {
        Some(profile) => writeln!(output, "pif_ipl2_profile: {}", profile.name()).unwrap(),
        None => writeln!(output, "pif_ipl2_profile: unavailable").unwrap(),
    }
    writeln!(output, "pif_firmware_search: none").unwrap();
    writeln!(output, "pif_firmware_default_path: none").unwrap();
    writeln!(output, "pif_firmware_bytes_logged: no").unwrap();
    match facts.staging.pif_ipl2_copy_layout() {
        Some(layout) => {
            writeln!(output, "pif_firmware_sp_imem_production: materialized").unwrap();
            writeln!(
                output,
                "pif_ipl2_copy_source: raw [0x{:03X},0x{:03X})",
                layout.source_start_offset(),
                layout.source_end_offset_exclusive()
            )
            .unwrap();
            writeln!(
                output,
                "pif_ipl2_copy_destination: sp-imem [0x{:03X},0x{:03X})",
                layout.sp_imem_start_offset(),
                layout.sp_imem_end_offset_exclusive()
            )
            .unwrap();
            writeln!(output, "pif_ipl2_copy_size_bytes: {}", layout.byte_count()).unwrap();
            writeln!(
                output,
                "pif_ipl2_copy_provenance: user-supplied-pif-firmware"
            )
            .unwrap();
        }
        None => {
            let production = match (
                facts.pif_firmware_state.is_accepted(),
                facts.staging.pif_ipl2_profile(),
            ) {
                (true, None) => "not-materialized-no-profile",
                (false, None) => "unavailable",
                (_, Some(_)) => {
                    unreachable!("profiled bootstrap success must report a copy layout")
                }
            };
            writeln!(output, "pif_firmware_sp_imem_production: {production}").unwrap();
            writeln!(output, "pif_ipl2_copy_source: unavailable").unwrap();
            writeln!(output, "pif_ipl2_copy_destination: unavailable").unwrap();
            writeln!(output, "pif_ipl2_copy_size_bytes: unavailable").unwrap();
            writeln!(output, "pif_ipl2_copy_provenance: unavailable").unwrap();
        }
    }
    writeln!(output, "pif_ipl1_execution: no").unwrap();
    writeln!(output, "pif_ipl2_execution: no").unwrap();
    writeln!(
        output,
        "header_magic: 0x{:08X}",
        facts.metadata.header_magic
    )
    .unwrap();
    writeln!(output, "clock_rate: 0x{:08X}", facts.metadata.clock_rate).unwrap();
    writeln!(
        output,
        "cartridge_declared_entry: 0x{:08X}",
        facts.metadata.entry_point
    )
    .unwrap();
    writeln!(
        output,
        "release_address: 0x{:08X}",
        facts.metadata.release_address
    )
    .unwrap();
    writeln!(output, "crc1: 0x{:08X}", facts.metadata.crc1).unwrap();
    writeln!(output, "crc2: 0x{:08X}", facts.metadata.crc2).unwrap();
    writeln!(output, "internal_title: {}", facts.metadata.image_name).unwrap();
    writeln!(output, "cartridge_id: {}", facts.metadata.cartridge_id).unwrap();
    writeln!(
        output,
        "country_code: 0x{:02X}",
        facts.metadata.country_code
    )
    .unwrap();
    writeln!(output, "version: 0x{:02X}", facts.metadata.revision).unwrap();
    writeln!(
        output,
        "bootstrap_source_range: cartridge[0x{:08X}..0x{:08X})",
        facts.staging.cartridge_start_offset(),
        facts.staging.cartridge_end_offset_exclusive()
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_destination_range: sp-dmem[0x{:08X}..0x{:08X})",
        facts.staging.sp_dmem_start_offset(),
        facts.staging.sp_dmem_end_offset_exclusive()
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_cpu_state: {:?}",
        facts.staging.cpu_state_kind()
    )
    .unwrap();
    match facts.staging.pif_ipl2_handoff_inputs() {
        Some(inputs) => {
            writeln!(output, "pif_ipl2_coupled_handoff: materialized").unwrap();
            writeln!(output, "pif_ipl3_family: {:?}", inputs.ipl3_family()).unwrap();
            writeln!(
                output,
                "pif_ipl2_handoff_reset_kind: {:?}",
                inputs.reset_kind()
            )
            .unwrap();
            writeln!(
                output,
                "pif_ipl2_handoff_boot_medium: {:?}",
                inputs.boot_medium()
            )
            .unwrap();
            writeln!(
                output,
                "pif_version_bit: {}",
                inputs.pif_version_bit().value()
            )
            .unwrap();
        }
        None => {
            writeln!(output, "pif_ipl2_coupled_handoff: unavailable").unwrap();
            writeln!(output, "pif_ipl3_family: unavailable").unwrap();
            writeln!(output, "pif_ipl2_handoff_reset_kind: unavailable").unwrap();
            writeln!(output, "pif_ipl2_handoff_boot_medium: unavailable").unwrap();
            writeln!(output, "pif_version_bit: unavailable").unwrap();
        }
    }
    writeln!(
        output,
        "bootstrap_cop0_status: 0x{:08X}",
        facts.staging_observation.status
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_cop0_status_source_backed: {}",
        yes_no(facts.staging.cop0_status_source().is_known())
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_cop0_status_source: {}",
        match facts.staging.cop0_status_source() {
            MachineBootstrapCop0StatusSource::UnknownPifProduced => "unknown-pif-produced",
            MachineBootstrapCop0StatusSource::PifIpl1ColdBootStatus => {
                "pif-ipl1-cold-boot-status"
            }
        }
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_control_flow_source: {}",
        match facts.staging.control_flow_source() {
            MachineBootstrapControlFlowSource::DirectCartridgeBootstrapStaging => {
                "direct-cartridge-bootstrap-staging".to_owned()
            }
            MachineBootstrapControlFlowSource::PifIpl2CompletedX105Transfer { profile } => {
                format!(
                    "pif-ipl2-completed-x105-transfer(profile={})",
                    profile.name()
                )
            }
        }
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_delay_slot_active: {}",
        yes_no(facts.staging_observation.delay_slot_active)
    )
    .unwrap();
    writeln!(
        output,
        "unrepresented_pif_cpu_state: {}",
        yes_no(facts.staging.has_unrepresented_pif_cpu_state())
    )
    .unwrap();
    let stack_pointer_index = usize::from(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX);
    writeln!(
        output,
        "bootstrap_gpr0_value: 0x{:016X}",
        facts.staging_observation.gprs[0]
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_gpr0_known: {}",
        yes_no(facts.staging_observation.gpr_sources[0].is_known())
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_gpr0_source: {}",
        format_gpr_source(facts.staging_observation.gpr_sources[0])
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_gpr29_value: 0x{:016X}",
        facts.staging_observation.gprs[stack_pointer_index]
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_gpr29_expected_value: 0x{:016X}",
        MACHINE_PIF_IPL2_HANDOFF_SP_VALUE
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_gpr29_known: {}",
        yes_no(facts.staging_observation.gpr_sources[stack_pointer_index].is_known())
    )
    .unwrap();
    writeln!(
        output,
        "bootstrap_gpr29_source: {}",
        format_gpr_source(facts.staging_observation.gpr_sources[stack_pointer_index])
    )
    .unwrap();
    for (alias, index) in [
        ("t3", MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX),
        ("s3", MACHINE_PIF_IPL2_HANDOFF_S3_GPR_INDEX),
        ("s4", MACHINE_PIF_IPL2_HANDOFF_S4_GPR_INDEX),
        ("s5", MACHINE_PIF_IPL2_HANDOFF_S5_GPR_INDEX),
        ("s6", MACHINE_PIF_IPL2_HANDOFF_S6_GPR_INDEX),
        ("s7", MACHINE_PIF_IPL2_HANDOFF_S7_GPR_INDEX),
        ("sp", MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX),
        ("ra", MACHINE_PIF_IPL2_HANDOFF_RA_GPR_INDEX),
    ] {
        let index = usize::from(index);
        writeln!(
            output,
            "bootstrap_{alias}_value: 0x{:016X}",
            facts.staging_observation.gprs[index]
        )
        .unwrap();
        writeln!(
            output,
            "bootstrap_{alias}_known: {}",
            yes_no(facts.staging_observation.gpr_sources[index].is_known())
        )
        .unwrap();
        writeln!(
            output,
            "bootstrap_{alias}_source: {}",
            format_gpr_source(facts.staging_observation.gpr_sources[index])
        )
        .unwrap();
    }
    writeln!(output, "fixed_step_budget: {}", facts.max_steps).unwrap();
    writeln!(output, "attempted_steps: {}", facts.attempted_steps).unwrap();
    writeln!(output, "committed_steps: {}", facts.committed_steps).unwrap();
    writeln!(
        output,
        "highest_checkpoint: {}",
        facts.highest_checkpoint.name()
    )
    .unwrap();
    writeln!(
        output,
        "highest_checkpoint_definition: {}",
        facts.highest_checkpoint.definition()
    )
    .unwrap();
    match facts.last_committed_step {
        Some(last) => {
            writeln!(
                output,
                "last_committed_address: 0x{:08X}",
                last.inspection.cpu_address().value()
            )
            .unwrap();
            writeln!(
                output,
                "last_committed_identity: {:?}",
                last.inspection.identity()
            )
            .unwrap();
            writeln!(
                output,
                "last_committed_source: {}",
                format_instruction_source(last.inspection.source())
            )
            .unwrap();
            writeln!(output, "last_committed_outcome: {}", last.outcome).unwrap();
            writeln!(output, "last_committed_effect: {}", last.effect).unwrap();
            match last.gpr_effect {
                Some(effect) => {
                    writeln!(output, "last_committed_destination_gpr: {}", effect.index).unwrap();
                    writeln!(
                        output,
                        "last_committed_destination_old_value: 0x{:016X}",
                        effect.old_value
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "last_committed_destination_new_value: 0x{:016X}",
                        effect.new_value
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "last_committed_destination_value_changed: {}",
                        yes_no(effect.old_value != effect.new_value)
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "last_committed_destination_known: {}->{}",
                        yes_no(effect.old_source.is_known()),
                        yes_no(effect.new_source.is_known())
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "last_committed_destination_source: {}->{}",
                        format_gpr_source(effect.old_source),
                        format_gpr_source(effect.new_source)
                    )
                    .unwrap();
                }
                None => {
                    writeln!(output, "last_committed_destination_gpr: none").unwrap();
                    writeln!(output, "last_committed_destination_old_value: unavailable").unwrap();
                    writeln!(output, "last_committed_destination_new_value: unavailable").unwrap();
                    writeln!(output, "last_committed_destination_value_changed: no").unwrap();
                    writeln!(output, "last_committed_destination_known: unavailable").unwrap();
                    writeln!(output, "last_committed_destination_source: unavailable").unwrap();
                }
            }
        }
        None => {
            writeln!(output, "last_committed_address: unavailable").unwrap();
            writeln!(output, "last_committed_identity: unavailable").unwrap();
            writeln!(output, "last_committed_source: unavailable").unwrap();
            writeln!(output, "last_committed_outcome: none").unwrap();
            writeln!(output, "last_committed_effect: none").unwrap();
            writeln!(output, "last_committed_destination_gpr: unavailable").unwrap();
            writeln!(output, "last_committed_destination_old_value: unavailable").unwrap();
            writeln!(output, "last_committed_destination_new_value: unavailable").unwrap();
            writeln!(output, "last_committed_destination_value_changed: no").unwrap();
            writeln!(output, "last_committed_destination_known: unavailable").unwrap();
            writeln!(output, "last_committed_destination_source: unavailable").unwrap();
        }
    }
    writeln!(
        output,
        "last_represented_outcome: {}",
        facts.last_represented_outcome
    )
    .unwrap();
    writeln!(output, "pc: 0x{:08X}", facts.final_observation.pc).unwrap();
    writeln!(output, "next_pc: 0x{:08X}", facts.final_observation.next_pc).unwrap();
    writeln!(output, "count: {}", facts.final_observation.count).unwrap();
    writeln!(
        output,
        "first_unsupported_frontier: {}",
        facts.first_frontier
    )
    .unwrap();
    writeln!(
        output,
        "cartridge_declared_entry_reached: {}",
        yes_no(facts.cartridge_entry_reached)
    )
    .unwrap();
    writeln!(
        output,
        "game_program_instruction_after_handoff: {}",
        yes_no(facts.game_program_instruction_ran)
    )
    .unwrap();
    writeln!(output, "graphics_or_external_output_reached: no").unwrap();
    writeln!(
        output,
        "probe_stop_classification: expected-machine-frontier"
    )
    .unwrap();
    writeln!(output, "expected_frontier_exit_policy: success").unwrap();
    writeln!(output, "probe_exit_status: 0").unwrap();
    writeln!(output, "compatibility_claim: none").unwrap();
    writeln!(output, "no_window: yes").unwrap();
    writeln!(output, "result: represented-stop").unwrap();
    output
}

const fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fn64_core::{PIF_BOOT_ROM_SIZE_BYTES, PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES};

    fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset] = ((value >> 24) & 0xff) as u8;
        bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
        bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
        bytes[offset + 3] = value as u8;
    }

    fn make_generated_boot_fixture(first: u32, second: u32) -> Vec<u8> {
        let mut bytes = vec![0; 0x1000];
        write_be_u32(&mut bytes, 0x00, 0x8037_1240);
        write_be_u32(&mut bytes, 0x04, 0x0102_0304);
        write_be_u32(&mut bytes, 0x08, 0x8000_1000);
        write_be_u32(&mut bytes, 0x0c, 0x0506_0708);
        write_be_u32(&mut bytes, 0x10, 0x1112_1314);
        write_be_u32(&mut bytes, 0x14, 0x1516_1718);
        bytes[0x20..0x33].copy_from_slice(b"FN64 GENERATED BOOT");
        bytes[0x3c] = b'G';
        bytes[0x3d] = b'P';
        bytes[0x3e] = 0x45;
        bytes[0x3f] = 2;
        write_be_u32(&mut bytes, 0x40, first);
        write_be_u32(&mut bytes, 0x44, second);
        bytes
    }

    fn make_generated_pif_firmware(seed: u8, size: usize) -> Vec<u8> {
        (0..size)
            .map(|index| seed.wrapping_add((index as u8).wrapping_mul(47)))
            .collect()
    }

    const fn special_add_word(rs: u8, rt: u8, rd: u8) -> u32 {
        ((rs as u32) << 21) | ((rt as u32) << 16) | ((rd as u32) << 11) | 0x20
    }

    const fn lw_word(base: u8, rt: u8, immediate: u16) -> u32 {
        (0x23 << 26) | ((base as u32) << 21) | ((rt as u32) << 16) | immediate as u32
    }

    #[test]
    fn boot_probe_reports_known_special_add_and_unknown_sp_imem_load_frontier() {
        let report = run_boot_probe(
            make_generated_boot_fixture(
                special_add_word(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, 0, 9),
                lw_word(9, 8, 0xf010),
            ),
            "generated-fixture.z64",
            100,
        )
        .unwrap();

        assert_eq!(report.highest_checkpoint(), BootCheckpoint::Boot2);
        assert_eq!(report.attempted_steps(), 2);
        assert_eq!(report.committed_steps(), 1);
        assert_eq!(report.expected_frontier_exit_status(), 0);
        assert!(report
            .first_frontier()
            .contains("load-word-rejected address=0xA4000044 identity=Lw rs=9 rt=8"));
        assert!(report
            .first_frontier()
            .contains("base_known=yes base_value=0xFFFFFFFFA4001FF0"));
        assert!(report
            .first_frontier()
            .contains("effective_address=0xFFFFFFFFA4001000 effective_cpu_address=0xA4001000"));
        assert!(report
            .first_frontier()
            .contains("target=sp-imem offset=0x00000000"));
        assert!(report
            .first_frontier()
            .contains("reason=sp-imem-unknown first_unknown_offset=0x00000000"));
        assert!(report
            .output()
            .contains("last_committed_identity: SpecialAdd"));
        assert!(report
            .output()
            .contains("last_committed_source: cartridge-bootstrap cartridge_offset=0x00000040"));
        assert!(report.output().contains(
            "last_committed_effect: pc=0xA4000040->0xA4000044 next_pc=0xA4000044->0xA4000048 count=0->1"
        ));
        assert!(report
            .output()
            .contains("gpr_mutations=r9=0x0000000000000000->0xFFFFFFFFA4001FF0 known=no->yes"));
        assert!(report
            .output()
            .contains("last_committed_destination_gpr: 9"));
        assert!(report
            .output()
            .contains("last_committed_destination_old_value: 0x0000000000000000"));
        assert!(report
            .output()
            .contains("last_committed_destination_new_value: 0xFFFFFFFFA4001FF0"));
        assert!(report
            .output()
            .contains("last_committed_destination_value_changed: yes"));
        assert!(report
            .output()
            .contains("last_committed_destination_known: no->yes"));
        assert!(report
            .output()
            .contains("bootstrap_gpr29_value: 0xFFFFFFFFA4001FF0"));
        assert!(report.output().contains("bootstrap_gpr29_known: yes"));
        assert!(report
            .output()
            .contains("bootstrap_gpr29_source: pif-ipl2-restored-stack-pointer"));
        assert!(report.output().contains("highest_checkpoint: BOOT-2"));
        assert!(report.output().contains("probe_exit_status: 0"));
        assert!(report.output().contains("compatibility_claim: none"));
        assert!(!report.output().contains("raw="));
    }

    #[test]
    fn boot_probe_unknown_reset_state_rejection_is_distinct_and_uncommitted() {
        let report = run_boot_probe(
            make_generated_boot_fixture(special_add_word(7, 0, 8), lw_word(8, 9, 0)),
            "generated-fixture.z64",
            100,
        )
        .unwrap();

        assert_eq!(report.highest_checkpoint(), BootCheckpoint::Boot1);
        assert_eq!(report.attempted_steps(), 1);
        assert_eq!(report.committed_steps(), 0);
        assert!(report.first_frontier().contains(
            "bootstrap-cpu-state-unavailable address=0xA4000040 identity=SpecialAdd rs=7 rt=0 rd=8"
        ));
        assert!(report.first_frontier().contains(
            "unknown_gpr=7 unknown_source=unknown-pif-produced rejected_before_mutation=yes"
        ));
        assert!(report
            .output()
            .contains("last_represented_outcome: bootstrap-cpu-state-unavailable"));
        assert!(report.output().contains("pc: 0xA4000040"));
        assert!(report.output().contains("next_pc: 0xA4000044"));
        assert!(report.output().contains("count: 0"));
        assert!(report.output().contains("last_committed_effect: none"));
        assert!(report.output().contains("probe_exit_status: 0"));
    }

    #[test]
    fn boot_probe_fixed_budget_is_explicit_and_deterministic() {
        let bytes = make_generated_boot_fixture(0x3c08_1234, 0x3c09_5678);
        let first = run_boot_probe(bytes.clone(), "generated-fixture.z64", 1).unwrap();
        let second = run_boot_probe(bytes, "generated-fixture.z64", 1).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.attempted_steps(), 1);
        assert_eq!(first.committed_steps(), 1);
        assert!(first.first_frontier().contains("explicit-step-budget"));
        assert!(first.output().contains("fixed_step_budget: 1"));
        assert!(!first.output().contains("timestamp"));
        assert!(!first.output().contains("SDL"));
        assert!(!first.output().contains("audio"));
        assert!(!first.output().contains("raw="));
    }

    #[test]
    fn boot_probe_unknown_sp_imem_first_instruction_stops_at_boot1_without_mutation() {
        let report = run_boot_probe(
            make_generated_boot_fixture(lw_word(29, 8, 0xf010), 0),
            "generated-fixture.z64",
            100,
        )
        .unwrap();

        assert_eq!(report.highest_checkpoint(), BootCheckpoint::Boot1);
        assert_eq!(report.attempted_steps(), 1);
        assert_eq!(report.committed_steps(), 0);
        assert!(report.first_frontier().contains("reason=sp-imem-unknown"));
        assert!(report.output().contains("pc: 0xA4000040"));
        assert!(report.output().contains("next_pc: 0xA4000044"));
        assert!(report.output().contains("last_committed_effect: none"));
    }

    #[test]
    fn boot_probe_accepts_unprofiled_pif_input_without_materializing_sp_imem() {
        let cartridge = make_generated_boot_fixture(
            special_add_word(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, 0, 9),
            lw_word(9, 8, 0xf010),
        );
        let first = run_boot_probe_with_pif_firmware(
            cartridge.clone(),
            "generated-fixture.z64",
            Some(make_generated_pif_firmware(0x31, PIF_BOOT_ROM_SIZE_BYTES)),
            None,
            100,
        )
        .unwrap();
        let second = run_boot_probe_with_pif_firmware(
            cartridge,
            "generated-fixture.z64",
            Some(make_generated_pif_firmware(0xa7, PIF_BOOT_ROM_SIZE_BYTES)),
            None,
            100,
        )
        .unwrap();

        assert_eq!(first, second);
        assert_eq!(first.highest_checkpoint(), BootCheckpoint::Boot2);
        assert_eq!(first.attempted_steps(), 2);
        assert_eq!(first.committed_steps(), 1);
        assert!(first.output().contains("pif_firmware_input: accepted"));
        assert!(first
            .output()
            .contains("pif_firmware_classification: raw-boot-rom"));
        assert!(first.output().contains("pif_ipl2_profile: unavailable"));
        assert!(first.output().contains("pif_firmware_size_bytes: 1984"));
        assert!(first
            .output()
            .contains("pif_firmware_sp_imem_production: not-materialized-no-profile"));
        assert!(first.output().contains("pif_ipl2_copy_source: unavailable"));
        assert!(first.first_frontier().contains("reason=sp-imem-unknown"));
        assert!(first.output().contains("compatibility_claim: none"));
    }

    #[test]
    fn boot_probe_materializes_profiled_pif_copy_deterministically() {
        let cartridge = make_generated_boot_fixture(
            special_add_word(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, 0, 9),
            lw_word(9, 8, 0xf010),
        );
        let firmware = make_generated_pif_firmware(0x31, PIF_BOOT_ROM_SIZE_BYTES);
        let first = run_boot_probe_with_pif_firmware(
            cartridge.clone(),
            "generated-fixture.z64",
            Some(firmware.clone()),
            Some(PifIpl2Profile::NtscPinned),
            2,
        )
        .unwrap();
        let second = run_boot_probe_with_pif_firmware(
            cartridge,
            "generated-fixture.z64",
            Some(firmware),
            Some(PifIpl2Profile::NtscPinned),
            2,
        )
        .unwrap();

        assert_eq!(first, second);
        assert_eq!(first.highest_checkpoint(), BootCheckpoint::Boot2);
        assert_eq!(first.attempted_steps(), 2);
        assert_eq!(first.committed_steps(), 2);
        assert!(first.output().contains("pif_firmware_input: accepted"));
        assert!(first
            .output()
            .contains("pif_firmware_classification: raw-boot-rom"));
        assert!(first.output().contains("pif_ipl2_profile: NTSC_PINNED"));
        assert!(first.output().contains("pif_firmware_size_bytes: 1984"));
        assert!(first.output().contains("pif_firmware_search: none"));
        assert!(first.output().contains("pif_firmware_default_path: none"));
        assert!(first.output().contains("pif_firmware_bytes_logged: no"));
        assert!(first
            .output()
            .contains("pif_firmware_sp_imem_production: materialized"));
        assert!(first
            .output()
            .contains("pif_ipl2_copy_source: raw [0x0D4,0x71C)"));
        assert!(first
            .output()
            .contains("pif_ipl2_copy_destination: sp-imem [0x000,0x648)"));
        assert!(first.output().contains("pif_ipl2_copy_size_bytes: 1608"));
        assert!(first
            .output()
            .contains("pif_ipl2_copy_provenance: user-supplied-pif-firmware"));
        assert!(first.output().contains("last_committed_identity: Lw"));
        assert!(first.first_frontier().contains("explicit-step-budget"));
    }

    #[test]
    fn boot_probe_distinguishes_malformed_and_unsupported_pif_input() {
        let malformed = run_boot_probe_with_pif_firmware(
            make_generated_boot_fixture(0, 0),
            "generated-fixture.z64",
            Some(make_generated_pif_firmware(
                0x42,
                PIF_BOOT_ROM_SIZE_BYTES - 1,
            )),
            None,
            100,
        )
        .unwrap_err();
        assert!(matches!(
            malformed,
            BootProbeError::PifFirmwareValidation(error) if error.is_malformed()
        ));

        let unsupported = run_boot_probe_with_pif_firmware(
            make_generated_boot_fixture(0, 0),
            "generated-fixture.z64",
            Some(make_generated_pif_firmware(
                0x53,
                PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
            )),
            Some(PifIpl2Profile::MpalPinned),
            100,
        )
        .unwrap_err();
        assert!(matches!(
            unsupported,
            BootProbeError::PifFirmwareValidation(error) if error.is_unsupported()
        ));
    }

    #[test]
    fn boot_probe_rejects_structural_and_bootstrap_bounds_failures() {
        assert_eq!(
            run_boot_probe(
                make_generated_boot_fixture(0, 0),
                "generated-zero-budget",
                0
            ),
            Err(BootProbeError::InvalidStepBudget)
        );
        assert!(matches!(
            run_boot_probe(Vec::new(), "generated-empty", 100),
            Err(BootProbeError::CartridgeLoad(_))
        ));

        let mut short = make_generated_boot_fixture(0, 0);
        short.truncate(0x100);
        assert!(matches!(
            run_boot_probe(short, "generated-short", 100),
            Err(BootProbeError::Bootstrap(
                MachineCartridgeBootstrapError::CartridgeSourceRangeUnavailable { .. }
            ))
        ));
    }

    #[test]
    fn boot_probe_internal_machine_invariant_exit_policy_is_nonzero() {
        let error = BootProbeError::MachineInvariant {
            attempted_step: 1,
            source: MachineRepresentedStepError::CompositionInvariantRejected,
        };

        assert_eq!(error.exit_status(), 1);
        assert!(error.to_string().contains("machine invariant failed"));
    }

    #[test]
    fn boot_probe_argument_parser_owns_fixed_budget_policy() {
        let default = parse_boot_probe_arguments([OsString::from("fixture.z64")]).unwrap();
        assert_eq!(default.input_path(), &PathBuf::from("fixture.z64"));
        assert_eq!(default.pif_rom_path(), None);
        assert_eq!(default.pif_profile(), None);
        assert_eq!(default.ipl3_family(), None);
        assert_eq!(default.reset_kind(), None);
        assert_eq!(default.boot_medium(), None);
        assert_eq!(default.pif_version_bit(), None);
        assert_eq!(default.max_steps(), DEFAULT_BOOT_PROBE_MAX_STEPS);

        let explicit = parse_boot_probe_arguments([
            OsString::from("fixture.any"),
            OsString::from("--max-steps"),
            OsString::from("17"),
        ])
        .unwrap();
        assert_eq!(explicit.max_steps(), 17);
        assert_eq!(explicit.pif_rom_path(), None);
        assert_eq!(explicit.pif_profile(), None);
        assert_eq!(explicit.ipl3_family(), None);
        assert_eq!(explicit.reset_kind(), None);
        assert_eq!(explicit.boot_medium(), None);
        assert_eq!(explicit.pif_version_bit(), None);
        let with_pif = parse_boot_probe_arguments([
            OsString::from("fixture.any"),
            OsString::from("--max-steps"),
            OsString::from("23"),
            OsString::from("--pif-rom"),
            OsString::from("generated-pif.fixture"),
            OsString::from("--pif-profile"),
            OsString::from("pal-pinned"),
        ])
        .unwrap();
        assert_eq!(with_pif.max_steps(), 23);
        assert_eq!(
            with_pif.pif_rom_path(),
            Some(&PathBuf::from("generated-pif.fixture"))
        );
        assert_eq!(with_pif.pif_profile(), Some(PifIpl2Profile::PalPinned));
        assert_eq!(
            parse_boot_probe_arguments([
                OsString::from("fixture"),
                OsString::from("--max-steps"),
                OsString::from("0"),
            ]),
            Err(BootProbeArgumentError::InvalidMaxSteps)
        );
        assert_eq!(
            parse_boot_probe_arguments([OsString::from("fixture"), OsString::from("--pif-rom"),]),
            Err(BootProbeArgumentError::MissingPifRomPath)
        );
        assert_eq!(
            parse_boot_probe_arguments([
                OsString::from("fixture"),
                OsString::from("--pif-profile"),
            ]),
            Err(BootProbeArgumentError::MissingPifProfile)
        );
        let unprofiled_pif = parse_boot_probe_arguments([
            OsString::from("fixture"),
            OsString::from("--pif-rom"),
            OsString::from("generated-pif.fixture"),
        ])
        .unwrap();
        assert_eq!(
            unprofiled_pif.pif_rom_path(),
            Some(&PathBuf::from("generated-pif.fixture"))
        );
        assert_eq!(unprofiled_pif.pif_profile(), None);
        assert_eq!(
            parse_boot_probe_arguments([
                OsString::from("fixture"),
                OsString::from("--pif-profile"),
                OsString::from("ntsc-pinned"),
            ]),
            Err(BootProbeArgumentError::PifProfileRequiresRom)
        );
        assert_eq!(
            parse_boot_probe_arguments([
                OsString::from("fixture"),
                OsString::from("--pif-rom"),
                OsString::from("generated-pif.fixture"),
                OsString::from("--pif-profile"),
                OsString::from("auto"),
            ]),
            Err(BootProbeArgumentError::UnsupportedPifProfile(
                "auto".to_owned()
            ))
        );
        assert_eq!(
            parse_boot_probe_arguments(Vec::<OsString>::new()),
            Err(BootProbeArgumentError::Usage)
        );
    }

    #[test]
    fn boot_probe_argument_parser_owns_exact_pif_profile_spellings() {
        for (spelling, profile) in [
            ("ntsc-pinned", PifIpl2Profile::NtscPinned),
            ("pal-pinned", PifIpl2Profile::PalPinned),
            ("mpal-pinned", PifIpl2Profile::MpalPinned),
        ] {
            let parsed = parse_boot_probe_arguments([
                OsString::from("fixture"),
                OsString::from("--pif-rom"),
                OsString::from("generated-pif.fixture"),
                OsString::from("--pif-profile"),
                OsString::from(spelling),
            ])
            .unwrap();

            assert_eq!(parsed.pif_profile(), Some(profile));
        }

        for unsupported in ["auto", "ntsc", "pal", "mpal", "NTSC_PINNED", "ntsc_pinned"] {
            assert_eq!(
                parse_boot_probe_arguments([
                    OsString::from("fixture"),
                    OsString::from("--pif-rom"),
                    OsString::from("generated-pif.fixture"),
                    OsString::from("--pif-profile"),
                    OsString::from(unsupported),
                ]),
                Err(BootProbeArgumentError::UnsupportedPifProfile(
                    unsupported.to_owned()
                ))
            );
        }
    }

    #[test]
    fn boot_probe_argument_parser_owns_exact_cold_x105_handoff_spellings() {
        let parsed = parse_boot_probe_arguments([
            OsString::from("fixture"),
            OsString::from("--pif-rom"),
            OsString::from("generated-pif.fixture"),
            OsString::from("--pif-profile"),
            OsString::from("ntsc-pinned"),
            OsString::from("--ipl3-family"),
            OsString::from("x105"),
            OsString::from("--reset-kind"),
            OsString::from("cold"),
            OsString::from("--boot-medium"),
            OsString::from("cartridge"),
            OsString::from("--pif-version-bit"),
            OsString::from("1"),
        ])
        .unwrap();

        assert_eq!(parsed.pif_profile(), Some(PifIpl2Profile::NtscPinned));
        assert_eq!(parsed.ipl3_family(), Some(MachinePifIpl3Family::X105));
        assert_eq!(
            parsed.reset_kind(),
            Some(MachinePifIpl2HandoffResetKind::Cold)
        );
        assert_eq!(
            parsed.boot_medium(),
            Some(MachinePifIpl2HandoffBootMedium::Cartridge)
        );
        assert_eq!(parsed.pif_version_bit(), Some(MachinePifVersionBit::One));

        for (flag, value, expected) in [
            (
                "--ipl3-family",
                "unknown",
                BootProbeArgumentError::UnsupportedIpl3Family("unknown".into()),
            ),
            (
                "--reset-kind",
                "nmi",
                BootProbeArgumentError::UnsupportedResetKind("nmi".into()),
            ),
            (
                "--boot-medium",
                "dd",
                BootProbeArgumentError::UnsupportedBootMedium("dd".into()),
            ),
            (
                "--pif-version-bit",
                "2",
                BootProbeArgumentError::UnsupportedPifVersionBit("2".into()),
            ),
        ] {
            assert_eq!(
                parse_boot_probe_arguments([
                    OsString::from("fixture"),
                    OsString::from(flag),
                    OsString::from(value),
                ]),
                Err(expected)
            );
        }

        for (flag, expected) in [
            ("--ipl3-family", BootProbeArgumentError::MissingIpl3Family),
            ("--reset-kind", BootProbeArgumentError::MissingResetKind),
            ("--boot-medium", BootProbeArgumentError::MissingBootMedium),
            (
                "--pif-version-bit",
                BootProbeArgumentError::MissingPifVersionBit,
            ),
        ] {
            assert_eq!(
                parse_boot_probe_arguments([OsString::from("fixture"), OsString::from(flag),]),
                Err(expected)
            );
        }
    }

    #[test]
    fn boot_probe_reports_generated_ntsc_cold_x105_coupled_handoff() {
        let report = run_boot_probe_with_pif_firmware_and_handoff(
            make_generated_boot_fixture(
                special_add_word(MACHINE_PIF_IPL2_HANDOFF_T3_GPR_INDEX, 0, 8),
                lw_word(MACHINE_PIF_IPL2_HANDOFF_SP_GPR_INDEX, 9, 0xf010),
            ),
            "generated-fixture.z64",
            Some(make_generated_pif_firmware(0x26, PIF_BOOT_ROM_SIZE_BYTES)),
            Some(PifIpl2Profile::NtscPinned),
            Some(MachinePifIpl3Family::X105),
            Some(MachinePifIpl2HandoffResetKind::Cold),
            Some(MachinePifIpl2HandoffBootMedium::Cartridge),
            Some(MachinePifVersionBit::One),
            1,
        )
        .unwrap();

        assert_eq!(report.highest_checkpoint(), BootCheckpoint::Boot2);
        assert_eq!(report.committed_steps(), 1);
        assert!(report
            .output()
            .contains("bootstrap_cpu_state: CoupledColdX105NtscPinned"));
        assert!(report
            .output()
            .contains("pif_ipl2_coupled_handoff: materialized"));
        assert!(report.output().contains("pif_ipl3_family: X105"));
        assert!(report
            .output()
            .contains("pif_ipl2_handoff_reset_kind: Cold"));
        assert!(report
            .output()
            .contains("pif_ipl2_handoff_boot_medium: Cartridge"));
        assert!(report.output().contains("pif_version_bit: 1"));
        assert!(report
            .output()
            .contains("bootstrap_cop0_status: 0x34000000"));
        assert!(report
            .output()
            .contains("bootstrap_cop0_status_source_backed: yes"));
        assert!(report
            .output()
            .contains("bootstrap_cop0_status_source: pif-ipl1-cold-boot-status"));
        assert!(report.output().contains(
            "bootstrap_control_flow_source: pif-ipl2-completed-x105-transfer(profile=NTSC_PINNED)"
        ));
        assert!(report.output().contains("bootstrap_delay_slot_active: no"));
        assert!(report
            .output()
            .contains("bootstrap_t3_value: 0xFFFFFFFFA4000040"));
        assert!(report
            .output()
            .contains("bootstrap_ra_value: 0xFFFFFFFFA4001550"));
        assert!(report.output().contains(
            "bootstrap_ra_source: pif-ipl2-retained-link(profile=NTSC_PINNED,instruction=0xA4001548)"
        ));
        assert!(report
            .output()
            .contains("bootstrap_s3_value: 0x0000000000000000"));
        assert!(report
            .output()
            .contains("bootstrap_s4_value: 0x0000000000000001"));
        assert!(report
            .output()
            .contains("bootstrap_s5_value: 0x0000000000000000"));
        assert!(report
            .output()
            .contains("bootstrap_s6_value: 0x0000000000000091"));
        assert!(report
            .output()
            .contains("bootstrap_s7_value: 0x0000000000000001"));
        assert!(report.output().contains("pif_ipl1_execution: no"));
        assert!(report.output().contains("pif_ipl2_execution: no"));
        assert!(report.output().contains("compatibility_claim: none"));
    }
}
