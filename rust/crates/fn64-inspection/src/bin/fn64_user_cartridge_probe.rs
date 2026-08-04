use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use fn64_core::{
    load_cartridge, CpuInstructionIdentity, Machine, MachineBootSource,
    MachineCartridgeBootstrapError, MachineCleanRoomBootProfile, MachineCpuInstructionFetchError,
    MachineCpuInstructionInspection, MachineLoadWordRejectionReason,
    MachinePifIpl2HandoffBootMedium, MachinePifIpl2HandoffResetKind, MachinePifIpl3Family,
    MachinePifVersionBit, MachineRepresentedStepError, MachineRepresentedStepOutcome,
    MachineRspStepRejectionReason, MachineSpStatusState, PifFirmwareClassification, PifIpl2Profile,
};

const DEFAULT_MAX_STEPS: u64 = 100_000_000;
const MAX_RUNTIME_FRONTIERS: usize = 128;
const REDACTED_USER_PIF_FIRMWARE: &str = "<REDACTED_USER_PIF_FIRMWARE>";

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserCartridgeProbeArguments {
    input_path: PathBuf,
    pif_rom_path: Option<PathBuf>,
    max_steps: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UserCartridgeBootMode {
    CleanRoomHle {
        profile: MachineCleanRoomBootProfile,
    },
    ExplicitPifFirmware {
        classification: PifFirmwareClassification,
    },
}

impl UserCartridgeBootMode {
    const fn name(self) -> &'static str {
        match self {
            Self::CleanRoomHle { .. } => "clean_room_hle",
            Self::ExplicitPifFirmware { .. } => "explicit_pif_firmware",
        }
    }

    const fn pif_identity(self) -> &'static str {
        match self {
            Self::CleanRoomHle { .. } => "not_used",
            Self::ExplicitPifFirmware { .. } => REDACTED_USER_PIF_FIRMWARE,
        }
    }

    const fn pif_material(self) -> &'static str {
        match self {
            Self::CleanRoomHle { .. } => "not_used",
            Self::ExplicitPifFirmware { .. } => "explicit-user-provided",
        }
    }

    fn pif_classification(self) -> &'static str {
        match self {
            Self::CleanRoomHle { .. } => "not_used",
            Self::ExplicitPifFirmware { classification } => classification.name(),
        }
    }

    const fn pif_execution(self) -> &'static str {
        "not_performed"
    }

    const fn x105_boot_rsp_execution(self) -> &'static str {
        match self {
            Self::CleanRoomHle { .. } => "not_performed",
            Self::ExplicitPifFirmware { .. } => "machine_step",
        }
    }

    const fn cartridge_entry_staged(self) -> &'static str {
        match self {
            Self::CleanRoomHle { .. } => "yes",
            Self::ExplicitPifFirmware { .. } => "no",
        }
    }

    const fn provenance(self) -> &'static str {
        match self {
            Self::CleanRoomHle { .. } => "clean_room_hle",
            Self::ExplicitPifFirmware { .. } => "explicit_pif_firmware",
        }
    }
}

struct RuntimeFrontier {
    class: String,
    owner: &'static str,
    pc: u32,
    identity: Option<CpuInstructionIdentity>,
}

fn record_runtime_frontier(
    frontiers: &mut Vec<RuntimeFrontier>,
    class: String,
    owner: &'static str,
    pc: u32,
    identity: Option<CpuInstructionIdentity>,
) {
    if frontiers.len() < MAX_RUNTIME_FRONTIERS
        && !frontiers.iter().any(|frontier| frontier.class == class)
    {
        frontiers.push(RuntimeFrontier {
            class,
            owner,
            pc,
            identity,
        });
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("fn64 user cartridge probe");
            eprintln!("result: fail");
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let arguments = parse_arguments(std::env::args_os().skip(1))?;
    let input_identity = redacted_input_identity(&arguments.input_path);
    let source_bytes = std::fs::read(&arguments.input_path)
        .map_err(|error| format!("input read failed for {input_identity}: {error}"))?;
    let cartridge = load_cartridge(source_bytes)
        .map_err(|error| format!("input normalization failed for {input_identity}: {error}"))?;
    let entrypoint = cartridge.metadata().entry_point;
    let owned_pif_firmware = read_explicit_pif_firmware(arguments.pif_rom_path.as_deref())?;

    let mut machine = Machine::from_cartridge(cartridge);
    let boot_mode = stage_user_cartridge_boot(&mut machine, owned_pif_firmware)?;

    let mut attempted_steps = 0_u64;
    let mut committed_steps = 0_u64;
    let mut cartridge_runtime_committed_steps = 0_u64;
    let mut entry_executions = 0_u64;
    let mut first_entry: Option<MachineCpuInstructionInspection> = None;
    let mut task_boundary: Option<(MachineCpuInstructionInspection, MachineSpStatusState, bool)> =
        None;
    let mut runtime_frontiers = Vec::new();
    let mut prior_sp_dma_records = machine.sp_dma_record_count();

    while attempted_steps < arguments.max_steps {
        let pc = machine.cpu().pc();
        let previous_halt = machine
            .sp_status_state()
            .is_none_or(MachineSpStatusState::halt);
        let needs_inspection = pc == entrypoint
            || (entry_executions != 0 && machine.sp_dma_record_count() != 0 && previous_halt);
        let inspection = needs_inspection
            .then(|| machine.inspect_current_cpu_instruction())
            .transpose()
            .map_err(|error| redacted_cpu_inspection_error(pc, error))?;

        let rsp_pc = machine.sp_pc_state().map(|state| state.raw_low_field());
        let outcome = machine
            .step()
            .map_err(|error| redacted_machine_step_error(pc, rsp_pc, error))?;
        attempted_steps += 1;
        let committed = outcome
            .cadence_plan()
            .is_some_and(|cadence| cadence.advances_count());
        if committed {
            committed_steps += 1;
        }

        if pc == entrypoint && committed {
            entry_executions += 1;
            if first_entry.is_none() {
                first_entry = inspection;
            }
        }
        if entry_executions != 0 && committed {
            cartridge_runtime_committed_steps += 1;
        }

        if entry_executions != 0 {
            let identity = outcome.identity();
            if let Some(
                identity @ (CpuInstructionIdentity::SpecialDiv
                | CpuInstructionIdentity::SpecialDmultu
                | CpuInstructionIdentity::SpecialDdivu
                | CpuInstructionIdentity::RegimmBgez
                | CpuInstructionIdentity::Blez
                | CpuInstructionIdentity::Lb
                | CpuInstructionIdentity::Lh
                | CpuInstructionIdentity::Lhu
                | CpuInstructionIdentity::Ld
                | CpuInstructionIdentity::Sh
                | CpuInstructionIdentity::Sd
                | CpuInstructionIdentity::Cop0Tlbr
                | CpuInstructionIdentity::Cop0Tlbwi
                | CpuInstructionIdentity::Cop0Tlbwr
                | CpuInstructionIdentity::Cop0Tlbp
                | CpuInstructionIdentity::Cop0Eret
                | CpuInstructionIdentity::Cop1Cfc1
                | CpuInstructionIdentity::Cop1Ctc1),
            ) = identity
            {
                record_runtime_frontier(
                    &mut runtime_frontiers,
                    format!("cpu:{identity:?}"),
                    "Cpu",
                    pc,
                    Some(identity),
                );
            }

            match outcome {
                MachineRepresentedStepOutcome::CacheIndexInvalidateCommitted { .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        "cache:index-invalidate".to_owned(),
                        "Cpu.primary_caches",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::CacheIndexWritebackInvalidateCommitted {
                    ..
                } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        "cache:index-writeback-invalidate".to_owned(),
                        "Cpu.primary_caches/Rdram",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::CacheHitWritebackCommitted { .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        "cache:hit-writeback".to_owned(),
                        "Cpu.primary_caches/Rdram",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::CacheHitInvalidateCommitted { .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        "cache:hit-invalidate".to_owned(),
                        "Cpu.primary_caches",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::InterruptExceptionEntered { .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        "interrupt:cpu-entry".to_owned(),
                        "Cpu.Cop0/Mi",
                        pc,
                        None,
                    );
                }
                MachineRepresentedStepOutcome::Cop1ControlTransferCommitted { kind, .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        format!("cop1:{kind:?}"),
                        "Cpu.Cop1",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::Mfc0Committed { source, .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        format!("cop0-read:{source:?}"),
                        "Cpu.Cop0",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::Mtc0Committed { destination, .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        format!("cop0-write:{destination:?}"),
                        "Cpu.Cop0",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::DeviceStoreWordCommitted { target, .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        format!("device-store:{target:?}"),
                        "Machine device owner",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::SpPcStoreCommitted { .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        "sp:pc".to_owned(),
                        "Sp",
                        pc,
                        identity,
                    );
                }
                MachineRepresentedStepOutcome::SpStatusStoreCommitted { .. } => {
                    record_runtime_frontier(
                        &mut runtime_frontiers,
                        "sp:status".to_owned(),
                        "Sp",
                        pc,
                        identity,
                    );
                }
                _ => {}
            }

            let sp_dma_records = machine.sp_dma_record_count();
            if sp_dma_records > prior_sp_dma_records {
                record_runtime_frontier(
                    &mut runtime_frontiers,
                    "sp:dma".to_owned(),
                    "Sp/Rdram/SpDmem/SpImem",
                    pc,
                    identity,
                );
            }
            prior_sp_dma_records = sp_dma_records;
        }

        if let MachineRepresentedStepOutcome::SpStatusStoreCommitted { state, .. } = outcome {
            if entry_executions != 0
                && previous_halt
                && !state.halt()
                && machine.sp_dma_record_count() != 0
            {
                let inspection = inspection.ok_or_else(|| {
                    "task start committed without one bounded pre-step inspection".to_owned()
                })?;
                task_boundary = Some((inspection, state, true));
                break;
            }
        }
    }

    let first_entry = first_entry.ok_or_else(|| {
        format!(
            "cartridge entrypoint was not executed: boot_source={}",
            boot_mode.name()
        )
    })?;
    let (task_instruction, task_status, halt_before) = task_boundary.ok_or_else(|| {
        format!(
            "step ceiling {} reached before the first RSP task submission: boot_source={}",
            arguments.max_steps,
            boot_mode.name()
        )
    })?;

    println!("fn64 user cartridge probe");
    println!("result: ok");
    println!("classification: USER_PROVIDED_CARTRIDGE_MACHINE_STEP_COMPOSITION");
    println!("input.identity: {input_identity}");
    println!("boot_source: {}", boot_mode.name());
    println!("pif_firmware.identity: {}", boot_mode.pif_identity());
    println!("pif_firmware.material: {}", boot_mode.pif_material());
    println!(
        "pif_firmware.classification: {}",
        boot_mode.pif_classification()
    );
    println!("pif_execution: {}", boot_mode.pif_execution());
    println!(
        "x105_boot_rsp_execution: {}",
        boot_mode.x105_boot_rsp_execution()
    );
    println!(
        "cartridge_entry_staged: {}",
        boot_mode.cartridge_entry_staged()
    );
    println!("cartridge_execution: machine_step");
    println!("boot_provenance: {}", boot_mode.provenance());
    println!("pif_firmware.synthetic_fallback: none");
    println!("cartridge.entrypoint: 0x{entrypoint:08X}");
    println!(
        "cartridge.first_instruction.identity: {:?}",
        first_entry.identity()
    );
    println!("cartridge.first_instruction.executions: {entry_executions}");
    println!("runtime.attempted_steps: {attempted_steps}");
    println!("runtime.committed_steps: {committed_steps}");
    println!("runtime.cartridge_committed_steps: {cartridge_runtime_committed_steps}");
    println!("runtime.frontiers: {}", runtime_frontiers.len());
    for (index, frontier) in runtime_frontiers.iter().enumerate() {
        println!("runtime.frontier[{index}].class: {}", frontier.class);
        println!("runtime.frontier[{index}].owner: {}", frontier.owner);
        println!("runtime.frontier[{index}].pc: 0x{:08X}", frontier.pc);
        println!(
            "runtime.frontier[{index}].identity: {}",
            frontier.identity.map_or_else(
                || "instruction-boundary".to_owned(),
                |identity| format!("{identity:?}")
            )
        );
    }
    println!(
        "rsp_task.start_instruction_pc: 0x{:08X}",
        task_instruction.cpu_address().value()
    );
    println!(
        "rsp_task.start_instruction_identity: {:?}",
        task_instruction.identity()
    );
    println!(
        "rsp_task.status_source_gpr: r{}",
        task_status.source().source_gpr()
    );
    println!(
        "rsp_task.status_source_lineage: {:?}",
        task_status.source().source_lineage()
    );
    println!("rsp_task.status_command_class: halt-cleared");
    println!("rsp_task.halt_before: {halt_before}");
    println!("rsp_task.halt_after: {}", task_status.halt());
    println!(
        "rsp_task.sp_pc: 0x{:03X}",
        machine
            .sp_pc_state()
            .map_or(0, |state| state.raw_low_field())
    );
    println!("rsp_task.sp_dma_records: {}", machine.sp_dma_record_count());
    for index in 0..machine.sp_dma_record_count() {
        let record = machine
            .sp_dma_record(index)
            .expect("SP DMA record count owns contiguous records");
        println!(
            "rsp_task.sp_dma[{index}].direction: {:?}",
            record.direction()
        );
        println!(
            "rsp_task.sp_dma[{index}].rdram: 0x{:08X}..0x{:08X}",
            record.initial_rdram_address(),
            record.final_rdram_address()
        );
        println!(
            "rsp_task.sp_dma[{index}].sp_local: 0x{:04X}..0x{:04X}",
            record.initial_local_address(),
            record.final_local_address()
        );
        println!(
            "rsp_task.sp_dma[{index}].bytes: {}",
            record.transferred_byte_count()
        );
        println!("rsp_task.sp_dma[{index}].blocks: {}", record.block_count());
        println!(
            "rsp_task.sp_dma[{index}].block_bytes: {}",
            record.block_length_bytes()
        );
        println!(
            "rsp_task.sp_dma[{index}].dram_skip: {}",
            record.dram_skip_bytes()
        );
    }
    println!(
        "rsp_task.mi_sp_pending: {}",
        machine
            .mi_interrupt_state()
            .pending(fn64_core::MachineMiInterruptSource::Sp)
    );
    println!("final.pc: 0x{:08X}", machine.cpu().pc());
    println!("final.next_pc: 0x{:08X}", machine.cpu().next_pc());
    println!("final.count: {}", machine.cpu().cop0_count());
    println!("rsp.instructions_executed: 0");
    println!("window: none");

    Ok(())
}

fn parse_arguments(
    mut arguments: impl Iterator<Item = OsString>,
) -> Result<UserCartridgeProbeArguments, String> {
    let input_path = arguments.next().map(PathBuf::from).ok_or_else(|| {
        "usage: fn64_user_cartridge_probe <cartridge-path> [max-steps | --max-steps <positive-integer>] [--pif-rom <path>]".to_owned()
    })?;
    let mut pif_rom_path = None;
    let mut max_steps = DEFAULT_MAX_STEPS;
    let mut max_steps_seen = false;

    while let Some(argument) = arguments.next() {
        if argument == "--pif-rom" {
            if pif_rom_path.is_some() {
                return Err(user_cartridge_probe_usage());
            }
            let path = arguments
                .next()
                .filter(|value| !value.is_empty() && !is_user_cartridge_probe_flag(value))
                .ok_or_else(|| "--pif-rom requires an explicit path".to_owned())?;
            pif_rom_path = Some(PathBuf::from(path));
        } else if argument == "--max-steps" {
            if max_steps_seen {
                return Err(user_cartridge_probe_usage());
            }
            let raw = arguments
                .next()
                .ok_or_else(|| "--max-steps requires a positive decimal integer".to_owned())?;
            max_steps = parse_max_steps(&raw)?;
            max_steps_seen = true;
        } else if !max_steps_seen {
            max_steps = parse_max_steps(&argument)?;
            max_steps_seen = true;
        } else {
            return Err(user_cartridge_probe_usage());
        }
    }

    if max_steps == 0 {
        return Err("max-steps must be greater than zero".to_owned());
    }
    Ok(UserCartridgeProbeArguments {
        input_path,
        pif_rom_path,
        max_steps,
    })
}

fn parse_max_steps(raw: &OsString) -> Result<u64, String> {
    raw.to_str()
        .ok_or_else(|| "max-steps must be UTF-8 decimal".to_owned())?
        .parse::<u64>()
        .map_err(|_| "max-steps must be a positive decimal integer".to_owned())
}

fn is_user_cartridge_probe_flag(value: &OsString) -> bool {
    matches!(value.to_str(), Some("--pif-rom" | "--max-steps"))
}

fn user_cartridge_probe_usage() -> String {
    "usage: fn64_user_cartridge_probe <cartridge-path> [max-steps | --max-steps <positive-integer>] [--pif-rom <path>]".to_owned()
}

fn read_explicit_pif_firmware(path: Option<&Path>) -> Result<Option<Vec<u8>>, String> {
    path.map(|path| {
        std::fs::read(path).map_err(|error| {
            format!(
                "explicit PIF firmware read failed for {REDACTED_USER_PIF_FIRMWARE}: kind={:?}",
                error.kind()
            )
        })
    })
    .transpose()
}

fn stage_user_cartridge_boot(
    machine: &mut Machine,
    owned_pif_firmware: Option<Vec<u8>>,
) -> Result<UserCartridgeBootMode, String> {
    match owned_pif_firmware {
        Some(owned_bytes) => {
            let classification = stage_explicit_pif_cold_x105_bootstrap(machine, owned_bytes)?;
            if machine.boot_source() != Some(MachineBootSource::ExplicitPifFirmware) {
                return Err(
                    "explicit PIF boot source was not retained by Machine ownership".to_owned(),
                );
            }
            Ok(UserCartridgeBootMode::ExplicitPifFirmware { classification })
        }
        None => {
            let profile = MachineCleanRoomBootProfile::NtscX105Pinned;
            machine
                .stage_clean_room_cartridge_entry(profile)
                .map_err(|error| format!("clean-room cartridge-entry staging failed: {error}"))?;
            if machine.boot_source() != Some(MachineBootSource::CleanRoomHle { profile }) {
                return Err(
                    "clean-room HLE boot source was not retained by Machine ownership".to_owned(),
                );
            }
            Ok(UserCartridgeBootMode::CleanRoomHle { profile })
        }
    }
}

fn stage_explicit_pif_cold_x105_bootstrap(
    machine: &mut Machine,
    owned_pif_firmware: Vec<u8>,
) -> Result<PifFirmwareClassification, String> {
    machine
        .install_pif_firmware(owned_pif_firmware)
        .map_err(|error| format!("explicit PIF firmware input rejected: {error}"))?;

    machine.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
    machine.install_pif_ipl3_family(MachinePifIpl3Family::X105);
    machine.install_pif_ipl2_handoff_reset_kind(MachinePifIpl2HandoffResetKind::Cold);
    machine.install_pif_ipl2_handoff_boot_medium(MachinePifIpl2HandoffBootMedium::Cartridge);
    machine.install_pif_version_bit(MachinePifVersionBit::Zero);
    machine
        .stage_cartridge_bootstrap()
        .map_err(explicit_pif_bootstrap_error)?;

    machine
        .pif_firmware_state()
        .classification()
        .ok_or_else(|| "explicit PIF firmware ownership missing after bootstrap".to_owned())
}

fn explicit_pif_bootstrap_error(error: MachineCartridgeBootstrapError) -> String {
    match error {
        MachineCartridgeBootstrapError::PifIpl2ProfileRequiresFirmware { .. } => {
            "PIF_FIRMWARE_REQUIRED_FOR_AUTHENTIC_BOOT: owner=PifFirmware material=unavailable synthetic_fallback=none".to_owned()
        }
        other => format!("explicit PIF cold x105 bootstrap staging failed: {other}"),
    }
}

fn redacted_cpu_inspection_error(pc: u32, error: MachineCpuInstructionFetchError) -> String {
    let category = match error {
        MachineCpuInstructionFetchError::Unaligned { .. } => "unaligned",
        MachineCpuInstructionFetchError::NonDirectUnsupported { .. } => "non-direct-unsupported",
        MachineCpuInstructionFetchError::DirectTargetMiss { .. } => "direct-target-miss",
        MachineCpuInstructionFetchError::PifResetUnavailable { .. } => "pif-reset-unavailable",
        MachineCpuInstructionFetchError::PrimaryInstructionCacheLineUnavailable { .. } => {
            "instruction-cache-line-unavailable"
        }
        MachineCpuInstructionFetchError::PrimaryInstructionCacheDataUnavailable { .. } => {
            "instruction-cache-data-unavailable"
        }
        MachineCpuInstructionFetchError::DirectRdram { .. } => "rdram-fetch-rejected",
        MachineCpuInstructionFetchError::SpDmem { .. } => "sp-dmem-fetch-rejected",
    };
    let _ = pc;
    format!("bounded instruction inspection failed: selected_processor=CPU category={category}")
}

fn redacted_machine_step_error(
    cpu_pc: u32,
    rsp_pc: Option<u32>,
    error: MachineRepresentedStepError,
) -> String {
    match error {
        MachineRepresentedStepError::RspRejected(rejection) => {
            let local_pc = rsp_pc
                .map(|pc| format!("0x{pc:03X}"))
                .unwrap_or_else(|| "unavailable".to_owned());
            format!(
                "Machine::step stopped before the first RSP task: selected_processor=RSP local_pc={local_pc} category={}",
                redacted_rsp_rejection_category(rejection.reason())
            )
        }
        MachineRepresentedStepError::LoadWordRejected(rejection) => format!(
            "Machine::step stopped before the first RSP task: selected_processor=CPU identity={:?} category=load-word-{}",
            rejection.identity(),
            redacted_load_word_rejection_category(rejection.reason())
        ),
        cpu_error => {
            let category = match cpu_error {
                MachineRepresentedStepError::FetchRejected(_) => "fetch-rejected",
                MachineRepresentedStepError::BootstrapCpuStateUnavailable(_) => {
                    "bootstrap-cpu-state-unavailable"
                }
                MachineRepresentedStepError::OrdinaryControlFlowRejected(_) => {
                    "ordinary-control-flow-rejected"
                }
                MachineRepresentedStepError::LoadWordRejected(_) => {
                    unreachable!("load-word rejection was structurally classified above")
                }
                MachineRepresentedStepError::StoreWordRejected(_) => "store-word-rejected",
                MachineRepresentedStepError::Mfc0Rejected(_) => "mfc0-rejected",
                MachineRepresentedStepError::Mtc0Rejected(_) => "mtc0-rejected",
                MachineRepresentedStepError::Cop1ControlTransferRejected(_) => {
                    "cop1-control-transfer-rejected"
                }
                MachineRepresentedStepError::CacheRejected(_) => "cache-rejected",
                MachineRepresentedStepError::CpuLocalInvocationRejected(_) => {
                    "cpu-local-invocation-rejected"
                }
                MachineRepresentedStepError::UnrepresentedInstruction { .. } => {
                    "unrepresented-instruction"
                }
                MachineRepresentedStepError::ArithmeticOverflowExceptionEntryRejected(_) => {
                    "arithmetic-overflow-entry-rejected"
                }
                MachineRepresentedStepError::DataAddressErrorExceptionEntryRejected(_) => {
                    "data-address-error-entry-rejected"
                }
                MachineRepresentedStepError::InstructionFetchAddressErrorEntryRejected(_) => {
                    "instruction-fetch-address-error-entry-rejected"
                }
                MachineRepresentedStepError::CompositionInvariantRejected => {
                    "composition-invariant-rejected"
                }
                MachineRepresentedStepError::RspRejected(_) => {
                    unreachable!("RSP rejection was handled before CPU structural classification")
                }
            };
            let identity = cpu_error
                .identity()
                .map(|identity| format!("{identity:?}"))
                .unwrap_or_else(|| "unavailable".to_owned());
            let _ = cpu_pc;
            format!(
                "Machine::step stopped before the first RSP task: selected_processor=CPU identity={identity} category={category}"
            )
        }
    }
}

fn redacted_load_word_rejection_category(reason: MachineLoadWordRejectionReason) -> &'static str {
    match reason {
        MachineLoadWordRejectionReason::NonDirectUnsupported => "non-direct-unsupported",
        MachineLoadWordRejectionReason::DirectTargetMiss => "direct-target-miss",
        MachineLoadWordRejectionReason::DirectRdramReadRejected => "rdram-read-rejected",
        MachineLoadWordRejectionReason::CartridgeReadRejected => "cartridge-read-rejected",
        MachineLoadWordRejectionReason::SpDmemUnknown { .. } => "sp-dmem-unknown",
        MachineLoadWordRejectionReason::SpDmemReadRejected => "sp-dmem-read-rejected",
        MachineLoadWordRejectionReason::SpImemUnknown { .. } => "sp-imem-unknown",
        MachineLoadWordRejectionReason::SpImemWordOpaque { .. } => "sp-imem-opaque",
        MachineLoadWordRejectionReason::SpImemReadRejected => "sp-imem-read-rejected",
        MachineLoadWordRejectionReason::RiSelectUnavailable => "ri-select-unavailable",
        MachineLoadWordRejectionReason::RdramRegisterModeDisabled => "rdram-register-mode-disabled",
        MachineLoadWordRejectionReason::RdramModuleRegisterUnavailable => {
            "rdram-module-register-unavailable"
        }
        MachineLoadWordRejectionReason::RiRefreshUnavailable => "ri-refresh-unavailable",
        MachineLoadWordRejectionReason::PiDomainTimingUnavailable { .. } => {
            "pi-domain-timing-unavailable"
        }
        MachineLoadWordRejectionReason::PrimaryDataCacheStateUnavailable => {
            "primary-data-cache-state-unavailable"
        }
    }
}

fn redacted_rsp_rejection_category(reason: MachineRspStepRejectionReason) -> String {
    match reason {
        MachineRspStepRejectionReason::SingleStepUnsupported => {
            "single-step-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::Fetch(_) => "fetch-rejected".to_owned(),
        MachineRspStepRejectionReason::MalformedMfc0Encoding => "mfc0-malformed".to_owned(),
        MachineRspStepRejectionReason::UnsupportedCop0Register { .. } => {
            "mfc0-control-source-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::SpDmaFullUnsupported => {
            "mfc0-sp-dma-full-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::MalformedMtc0Encoding => "mtc0-malformed".to_owned(),
        MachineRspStepRejectionReason::Mtc0SourceUnavailable { .. } => {
            "mtc0-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::UnsupportedMtc0ControlRegister { .. } => {
            "mtc0-control-destination-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0DmaRecordCapacityExhausted => {
            "mtc0-read-dma-capacity-exhausted".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0DmaAddressUnavailable => {
            "mtc0-read-dma-address-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0DmaRdramRangeRejected { .. } => {
            "mtc0-read-dma-rdram-range-rejected".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0WriteDmaRecordCapacityExhausted => {
            "mtc0-write-dma-capacity-exhausted".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0WriteDmaAddressUnavailable => {
            "mtc0-write-dma-address-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0WriteDmaSourceRangeRejected { .. } => {
            "mtc0-write-dma-source-range-rejected".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0WriteDmaSourceUnavailable { .. } => {
            "mtc0-write-dma-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0WriteDmaSourceOpaque { .. } => {
            "mtc0-write-dma-source-opaque".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0WriteDmaSourceKnowledgeInconsistent { .. } => {
            "mtc0-write-dma-source-knowledge-inconsistent".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0WriteDmaRdramRangeRejected { .. } => {
            "mtc0-write-dma-rdram-range-rejected".to_owned()
        }
        MachineRspStepRejectionReason::DpcStatusCommandUnsupported { .. } => {
            "mtc0-dpc-status-command-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::DpcCounterInvariantMalformed { .. } => {
            "mtc0-dpc-counter-invariant-malformed".to_owned()
        }
        MachineRspStepRejectionReason::BreakCodeUnsupported { .. } => {
            "break-code-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::BreakInDelaySlotUnsupported { .. } => {
            "break-in-delay-slot-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::XoriSourceUnavailable { .. } => {
            "xori-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::MalformedLuiEncoding => "lui-malformed".to_owned(),
        MachineRspStepRejectionReason::AddiSourceUnavailable { .. } => {
            "addi-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::UnsupportedRegimmSelector { .. } => {
            "regimm-selector-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::BltzSourceUnavailable { .. } => {
            "bltz-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::BgezSourceUnavailable { .. } => {
            "bgez-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::BneSourceAUnavailable { .. } => {
            "bne-source-a-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::BneSourceBUnavailable { .. } => {
            "bne-source-b-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::ControlFlowInDelaySlot { .. } => {
            "control-flow-in-delay-slot".to_owned()
        }
        MachineRspStepRejectionReason::LqvScalarBaseUnavailable { .. } => {
            "lqv-scalar-base-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::LqvElementUnsupported { .. } => {
            "lqv-element-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::LqvAddressMisaligned { .. } => {
            "lqv-address-misaligned".to_owned()
        }
        MachineRspStepRejectionReason::LqvDmemKnowledgeMalformed { .. } => {
            "lqv-dmem-knowledge-malformed".to_owned()
        }
        MachineRspStepRejectionReason::VectorLoadUnsupported { .. } => {
            "vector-load-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::VectorStoreUnsupported => {
            "vector-store-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::ScalarLwBaseUnavailable { .. } => {
            "lw-base-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::ScalarLwAddressMisaligned { .. } => {
            "lw-address-misaligned".to_owned()
        }
        MachineRspStepRejectionReason::ScalarLwDmemByteUnavailable { .. } => {
            "lw-dmem-byte-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::ScalarLwDmemKnowledgeMalformed { .. } => {
            "lw-dmem-knowledge-malformed".to_owned()
        }
        MachineRspStepRejectionReason::ScalarLoadUnsupported { .. } => {
            "scalar-load-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::ScalarStoreUnsupported { .. } => {
            "scalar-store-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::ScalarSllUnsupported => "sll-unsupported".to_owned(),
        MachineRspStepRejectionReason::VsubElementUnsupported { .. } => {
            "vsub-element-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::VaddcElementUnsupported { .. } => {
            "vaddc-element-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::UnrepresentedInstruction { class } => {
            format!("unrepresented-{class:?}-instruction")
        }
    }
}

fn redacted_input_identity(_path: &Path) -> &'static str {
    "<REDACTED_USER_CARTRIDGE>"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generated_hle_machine() -> Machine {
        let mut bytes =
            vec![0; fn64_core::MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_END_OFFSET_EXCLUSIVE as usize];
        bytes[0..4].copy_from_slice(&0x8037_1240_u32.to_be_bytes());
        bytes[8..12].copy_from_slice(&0x8000_1000_u32.to_be_bytes());
        bytes[0x1000..0x1004].copy_from_slice(&0x2402_0042_u32.to_be_bytes());
        Machine::from_cartridge(load_cartridge(bytes).unwrap())
    }

    #[test]
    fn explicit_path_parser_owns_only_path_and_positive_step_ceiling() {
        assert_eq!(
            parse_arguments(
                [OsString::from("/private/input.z64"), OsString::from("1234"),].into_iter(),
            )
            .unwrap(),
            UserCartridgeProbeArguments {
                input_path: PathBuf::from("/private/input.z64"),
                pif_rom_path: None,
                max_steps: 1234,
            }
        );
        assert_eq!(
            parse_arguments([OsString::from("/private/input.z64")].into_iter()).unwrap(),
            UserCartridgeProbeArguments {
                input_path: PathBuf::from("/private/input.z64"),
                pif_rom_path: None,
                max_steps: DEFAULT_MAX_STEPS,
            }
        );
        assert_eq!(
            parse_arguments(
                [
                    OsString::from("/private/input.z64"),
                    OsString::from("--pif-rom"),
                    OsString::from("/private/pif.bin"),
                    OsString::from("--max-steps"),
                    OsString::from("4321"),
                ]
                .into_iter()
            )
            .unwrap(),
            UserCartridgeProbeArguments {
                input_path: PathBuf::from("/private/input.z64"),
                pif_rom_path: Some(PathBuf::from("/private/pif.bin")),
                max_steps: 4321,
            }
        );
        assert!(parse_arguments(
            [OsString::from("/private/input.z64"), OsString::from("0")].into_iter()
        )
        .is_err());
        assert!(parse_arguments(
            [
                OsString::from("/private/input.z64"),
                OsString::from("--pif-rom"),
            ]
            .into_iter()
        )
        .is_err());
    }

    #[test]
    fn redacted_input_identity_never_exposes_path_components() {
        assert_eq!(
            redacted_input_identity(Path::new("/private/collection/input.z64")),
            "<REDACTED_USER_CARTRIDGE>"
        );
        assert_eq!(REDACTED_USER_PIF_FIRMWARE, "<REDACTED_USER_PIF_FIRMWARE>");
    }

    #[test]
    fn output_contract_omits_private_identity_shape_and_raw_instruction_fields() {
        let source = include_str!("fn64_user_cartridge_probe.rs");
        for forbidden_field in [
            ["input", ".basename: "].concat(),
            ["input", ".source_bytes: "].concat(),
            ["input", ".byte_order: "].concat(),
            ["input", ".normalized_bytes: "].concat(),
            ["cartridge.first_instruction", ".word: "].concat(),
            ["rsp_task.start_instruction", "_word: "].concat(),
        ] {
            assert!(!source.contains(&forbidden_field));
        }
        assert!(source.contains("input.identity: {input_identity}"));
        assert!(source.contains("boot_source: {}"));
        assert!(source.contains("pif_firmware.identity: {}"));
        assert!(source.contains("pif_execution: {}"));
        assert!(source.contains("x105_boot_rsp_execution: {}"));
        assert!(source.contains("cartridge_entry_staged: {}"));
        assert!(source.contains("cartridge_execution: machine_step"));
        assert!(source.contains("boot_provenance: {}"));
        assert!(source.contains("pif_firmware.synthetic_fallback: none"));
        assert!(source.contains("cartridge.first_instruction.identity: {:?}"));
        assert!(source.contains("rsp_task.start_instruction_identity: {:?}"));
        assert!(source.contains("rsp_task.status_command_class: halt-cleared"));
        let forbidden_status_command_field = ["rsp_task.status_", "command: 0x"].concat();
        assert!(!source.contains(&forbidden_status_command_field));
        let forbidden_raw_step_display =
            ["Machine::step stopped before the first RSP task ", "at PC"].concat();
        assert!(!source.contains(&forbidden_raw_step_display));
        let forbidden_cpu_pc_field = ["selected_processor=CPU", " pc="].concat();
        assert!(!source.contains(&forbidden_cpu_pc_field));
        assert!(source.contains("category=load-word-{}"));
        let forbidden_synthetic_install =
            ["install_public_synthetic_cold_", "x105_bootstrap"].concat();
        assert!(!source.contains(&forbidden_synthetic_install));
    }

    #[test]
    fn absent_pif_material_selects_clean_room_hle_without_synthetic_fallback() {
        let mut machine = generated_hle_machine();

        let mode = stage_user_cartridge_boot(&mut machine, None).unwrap();

        assert_eq!(
            mode,
            UserCartridgeBootMode::CleanRoomHle {
                profile: MachineCleanRoomBootProfile::NtscX105Pinned,
            }
        );
        assert_eq!(
            machine.boot_source(),
            Some(MachineBootSource::CleanRoomHle {
                profile: MachineCleanRoomBootProfile::NtscX105Pinned,
            })
        );
        assert!(machine.pif_firmware_state().is_absent());
        assert!(machine.cartridge_bootstrap_state().is_none());
        assert!(machine.clean_room_hle_state().is_some());
    }
}
