use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use fn64_core::{
    load_cartridge, CpuInstructionIdentity, Machine, MachineBootSource, MachineBootstrapGprSource,
    MachineCartridgeBootstrapError, MachineCleanRoomBootProfile, MachineCop0TlbOperationError,
    MachineCop1ControlTransferRejectionReason, MachineCpuInstructionFetchError,
    MachineCpuInstructionInspection, MachineLoadWordRejectionReason, MachinePiDomain,
    MachinePiDomainTimingField, MachinePiDomainTimingRegister, MachinePifIpl2HandoffBootMedium,
    MachinePifIpl2HandoffResetKind, MachinePifIpl3Family, MachinePifVersionBit,
    MachineRepresentedStepError, MachineRepresentedStepOutcome, MachineRspInstructionIdentity,
    MachineRspStepRejectionReason, MachineSpStatusState, MachineStepCpuLocalInvocationRejection,
    MachineStepProcessor, PifFirmwareClassification, PifIpl2Profile, RDRAM_SIZE_BYTES,
};

const DEFAULT_MAX_STEPS: u64 = 100_000_000;
const MAX_RUNTIME_FRONTIERS: usize = 128;
const REDACTED_USER_PIF_FIRMWARE: &str = "<REDACTED_USER_PIF_FIRMWARE>";

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserCartridgeProbeArguments {
    input_path: PathBuf,
    pif_rom_path: Option<PathBuf>,
    max_steps: u64,
    observe_rsp_pressure: bool,
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
    let mut rsp_committed_steps = 0_u64;
    let mut first_rsp_identity: Option<MachineRspInstructionIdentity> = None;
    let mut rsp_break_committed = false;
    let mut post_break_processor: Option<MachineStepProcessor> = None;

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

        let post_break_followup = rsp_break_committed;
        let outcome = match machine.step() {
            Ok(outcome) => outcome,
            Err(error) => {
                return Err(redacted_machine_step_error(
                    &machine,
                    attempted_steps.saturating_add(1),
                    committed_steps,
                    entry_executions,
                    rsp_committed_steps,
                    first_rsp_identity,
                    error,
                ));
            }
        };
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

        if let MachineRepresentedStepOutcome::RspCommitted { outcome } = outcome {
            rsp_committed_steps += 1;
            first_rsp_identity.get_or_insert(outcome.identity());
            if outcome.identity() == MachineRspInstructionIdentity::Break {
                rsp_break_committed = true;
            }
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
                if !arguments.observe_rsp_pressure {
                    break;
                }
            }
        }

        if post_break_followup {
            post_break_processor = Some(outcome.processor());
            break;
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
    let task_status_source = task_status
        .source()
        .cpu_store()
        .ok_or_else(|| "guest RSP task start lacks CPU-store provenance".to_owned())?;
    println!(
        "rsp_task.status_source_gpr: r{}",
        task_status_source.source_gpr()
    );
    println!(
        "rsp_task.status_source_lineage: {:?}",
        task_status_source.source_lineage()
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
    println!("rsp.instructions_executed: {rsp_committed_steps}");
    println!(
        "rsp.first_instruction_identity: {}",
        first_rsp_identity.map_or_else(
            || "not_attempted".to_owned(),
            |identity| format!("{identity:?}")
        )
    );
    println!("rsp.break_committed: {rsp_break_committed}");
    println!(
        "rsp.post_break_step_processor: {}",
        post_break_processor.map_or_else(
            || "not_attempted".to_owned(),
            |processor| format!("{processor:?}")
        )
    );
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
    let mut observe_rsp_pressure = false;

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
        } else if argument == "--rsp-pressure" {
            if observe_rsp_pressure {
                return Err(user_cartridge_probe_usage());
            }
            observe_rsp_pressure = true;
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
        observe_rsp_pressure,
    })
}

fn parse_max_steps(raw: &OsString) -> Result<u64, String> {
    raw.to_str()
        .ok_or_else(|| "max-steps must be UTF-8 decimal".to_owned())?
        .parse::<u64>()
        .map_err(|_| "max-steps must be a positive decimal integer".to_owned())
}

fn is_user_cartridge_probe_flag(value: &OsString) -> bool {
    matches!(
        value.to_str(),
        Some("--pif-rom" | "--max-steps" | "--rsp-pressure")
    )
}

fn user_cartridge_probe_usage() -> String {
    "usage: fn64_user_cartridge_probe <cartridge-path> [max-steps | --max-steps <positive-integer>] [--pif-rom <path>] [--rsp-pressure]".to_owned()
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
    machine: &Machine,
    attempted_step: u64,
    committed_steps: u64,
    entry_executions: u64,
    rsp_committed_steps: u64,
    first_rsp_identity: Option<MachineRspInstructionIdentity>,
    error: MachineRepresentedStepError,
) -> String {
    let progress = format!(
        "attempt={attempted_step} committed_steps={committed_steps} entry_commits={entry_executions} rsp_committed={rsp_committed_steps} rsp_first_identity={}",
        first_rsp_identity.map_or_else(
            || "not_committed".to_owned(),
            |identity| format!("{identity:?}")
        )
    );
    match error {
        MachineRepresentedStepError::RspRejected(rejection) => {
            format!(
                "Machine::step stopped at the first RSP pressure: {progress} selected_processor=RSP local_pc=redacted category={}",
                redacted_rsp_rejection_category(rejection.reason())
            )
        }
        MachineRepresentedStepError::LoadWordRejected(rejection) => {
            let direct_context = match rejection.reason() {
                MachineLoadWordRejectionReason::DirectTargetMiss => {
                    let cpu_address = rejection.cpu_address().value();
                    format!(
                        " owner_region={} direct_segment={} rdram_capacity_relation={} base_source={}",
                        redacted_direct_cpu_owner_region(cpu_address),
                        redacted_direct_cpu_segment(cpu_address),
                        redacted_rdram_capacity_relation(cpu_address),
                        redacted_load_base_source(machine, rejection.fields().rs())
                    )
                }
                _ => String::new(),
            };
            format!(
                "Machine::step stopped before the first RSP task: {progress} selected_processor=CPU identity={:?} category=load-word-{}{direct_context}",
                rejection.identity(),
                redacted_load_word_rejection_category(rejection.reason())
            )
        }
        MachineRepresentedStepError::Cop1ControlTransferRejected(rejection) => format!(
            "Machine::step stopped before the first RSP task: {progress} selected_processor=CPU identity={:?} category=cop1-control-{}",
            rejection.kind().identity(),
            redacted_cop1_control_rejection_category(rejection.reason())
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
                MachineRepresentedStepError::Cop1ControlTransferRejected(_) => unreachable!(
                    "COP1 control-transfer rejection was structurally classified above"
                ),
                MachineRepresentedStepError::CacheRejected(_) => "cache-rejected",
                MachineRepresentedStepError::CpuLocalInvocationRejected(rejection) => {
                    redacted_cpu_local_invocation_rejection_category(machine, rejection)
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
            format!(
                "Machine::step stopped before the first RSP task: {progress} selected_processor=CPU identity={identity} category={category}"
            )
        }
    }
}

fn redacted_cpu_local_invocation_rejection_category(
    machine: &Machine,
    rejection: MachineStepCpuLocalInvocationRejection,
) -> &'static str {
    match rejection.cop0_tlb_error() {
        Some(MachineCop0TlbOperationError::IndexUnavailable) => {
            "cpu-local-cop0-tlb-index-unavailable"
        }
        Some(MachineCop0TlbOperationError::IndexOutOfRange { .. }) => {
            "cpu-local-cop0-tlb-index-out-of-range"
        }
        Some(MachineCop0TlbOperationError::EntryUnavailable { .. }) => {
            "cpu-local-cop0-tlb-entry-unavailable"
        }
        Some(MachineCop0TlbOperationError::WorkingRegistersUnavailable)
            if machine.cpu().cop0_page_mask().is_none() =>
        {
            "cpu-local-cop0-tlb-page-mask-unavailable"
        }
        Some(MachineCop0TlbOperationError::WorkingRegistersUnavailable)
            if machine.cpu().cop0_entry_lo0().is_none() =>
        {
            "cpu-local-cop0-tlb-entry-lo0-unavailable"
        }
        Some(MachineCop0TlbOperationError::WorkingRegistersUnavailable)
            if machine.cpu().cop0_entry_lo1().is_none() =>
        {
            "cpu-local-cop0-tlb-entry-lo1-unavailable"
        }
        Some(MachineCop0TlbOperationError::WorkingRegistersUnavailable) => {
            "cpu-local-cop0-tlb-working-registers-unavailable"
        }
        None => "cpu-local-invocation-rejected",
    }
}

fn redacted_load_word_rejection_category(reason: MachineLoadWordRejectionReason) -> String {
    match reason {
        MachineLoadWordRejectionReason::NonDirectUnsupported => "non-direct-unsupported".to_owned(),
        MachineLoadWordRejectionReason::DirectTargetMiss => "direct-target-miss".to_owned(),
        MachineLoadWordRejectionReason::DirectRdramReadRejected => "rdram-read-rejected".to_owned(),
        MachineLoadWordRejectionReason::CartridgeReadRejected => {
            "cartridge-read-rejected".to_owned()
        }
        MachineLoadWordRejectionReason::SpDmemUnknown { .. } => "sp-dmem-unknown".to_owned(),
        MachineLoadWordRejectionReason::SpDmemReadRejected => "sp-dmem-read-rejected".to_owned(),
        MachineLoadWordRejectionReason::SpImemUnknown { .. } => "sp-imem-unknown".to_owned(),
        MachineLoadWordRejectionReason::SpImemWordOpaque { .. } => "sp-imem-opaque".to_owned(),
        MachineLoadWordRejectionReason::SpImemReadRejected => "sp-imem-read-rejected".to_owned(),
        MachineLoadWordRejectionReason::RiSelectUnavailable => "ri-select-unavailable".to_owned(),
        MachineLoadWordRejectionReason::RdramRegisterModeDisabled => {
            "rdram-register-mode-disabled".to_owned()
        }
        MachineLoadWordRejectionReason::RdramModuleRegisterUnavailable => {
            "rdram-module-register-unavailable".to_owned()
        }
        MachineLoadWordRejectionReason::RiRefreshUnavailable => "ri-refresh-unavailable".to_owned(),
        MachineLoadWordRejectionReason::PiDomainTimingUnavailable { register } => {
            redacted_pi_timing_register_category(register)
        }
        MachineLoadWordRejectionReason::PrimaryDataCacheStateUnavailable => {
            "primary-data-cache-state-unavailable".to_owned()
        }
    }
}

fn redacted_pi_timing_register_category(register: MachinePiDomainTimingRegister) -> String {
    let domain = match register.domain() {
        MachinePiDomain::One => "one",
        MachinePiDomain::Two => "two",
    };
    let field = match register.field() {
        MachinePiDomainTimingField::Latency => "latency",
        MachinePiDomainTimingField::PulseWidth => "pulse-width",
        MachinePiDomainTimingField::PageSize => "page-size",
        MachinePiDomainTimingField::Release => "release",
    };
    format!("pi-domain-{domain}-{field}-unavailable")
}

fn redacted_cop1_control_rejection_category(
    reason: MachineCop1ControlTransferRejectionReason,
) -> &'static str {
    match reason {
        MachineCop1ControlTransferRejectionReason::CoprocessorUnusable { .. } => {
            "coprocessor-unusable"
        }
        MachineCop1ControlTransferRejectionReason::MalformedEncoding { .. } => "malformed-encoding",
        MachineCop1ControlTransferRejectionReason::UnsupportedControlRegister { .. } => {
            "unsupported-control-register"
        }
        MachineCop1ControlTransferRejectionReason::StateUnavailable => "state-unavailable",
        MachineCop1ControlTransferRejectionReason::SourceUnavailable { .. } => "source-unavailable",
    }
}

fn redacted_direct_cpu_owner_region(cpu_address: u32) -> &'static str {
    let physical_address = cpu_address & 0x1fff_ffff;
    match physical_address {
        0x0000_0000..=0x03ef_ffff => "rdram",
        0x03f0_0000..=0x03ff_ffff => "rdram-registers",
        0x0400_0000..=0x040f_ffff => "sp",
        0x0410_0000..=0x041f_ffff => "dpc",
        0x0420_0000..=0x042f_ffff => "dps",
        0x0430_0000..=0x043f_ffff => "mi",
        0x0440_0000..=0x044f_ffff => "vi",
        0x0450_0000..=0x045f_ffff => "ai",
        0x0460_0000..=0x046f_ffff => "pi",
        0x0470_0000..=0x047f_ffff => "ri",
        0x0480_0000..=0x048f_ffff => "si",
        0x0490_0000..=0x04ff_ffff => "unassigned-rcp",
        0x0500_0000..=0x1fbf_ffff => "cartridge",
        0x1fc0_0000..=0x1fcf_ffff => "pif",
        _ => "unassigned",
    }
}

fn redacted_direct_cpu_segment(cpu_address: u32) -> &'static str {
    match cpu_address & 0xe000_0000 {
        0x8000_0000 => "kseg0",
        0xa000_0000 => "kseg1",
        _ => "non-direct",
    }
}

fn redacted_rdram_capacity_relation(cpu_address: u32) -> &'static str {
    if (cpu_address & 0x1fff_ffff) < RDRAM_SIZE_BYTES as u32 {
        "within-represented-capacity"
    } else {
        "outside-represented-capacity"
    }
}

fn redacted_load_base_source(machine: &Machine, register_index: u8) -> &'static str {
    match machine
        .clean_room_hle_state()
        .and_then(|state| state.gpr_source(usize::from(register_index)))
    {
        Some(MachineBootstrapGprSource::UnknownPifProduced) => "unavailable",
        Some(MachineBootstrapGprSource::ArchitecturalZero) => "architectural-zero",
        Some(MachineBootstrapGprSource::CleanRoomHlePublicProfile) => "clean-room-public-profile",
        Some(MachineBootstrapGprSource::CleanRoomHleCartridgeEntry) => "cartridge-entry-derived",
        Some(MachineBootstrapGprSource::CleanRoomHleCartridgePayload) => {
            "cartridge-payload-derived"
        }
        Some(MachineBootstrapGprSource::KnownInstructionResult { .. }) => "instruction-result",
        Some(
            MachineBootstrapGprSource::PifIpl2HandoffEntryPointer
            | MachineBootstrapGprSource::PifIpl2RestoredStackPointer
            | MachineBootstrapGprSource::PifIpl2RetainedLink { .. }
            | MachineBootstrapGprSource::CartridgeBootMedium
            | MachineBootstrapGprSource::PifProfileTvType { .. }
            | MachineBootstrapGprSource::ColdResetKind
            | MachineBootstrapGprSource::X105Seed
            | MachineBootstrapGprSource::PifVersionRegionalState { .. },
        ) => "low-level-boot-source",
        None => "unavailable",
    }
}

fn redacted_rsp_rejection_category(reason: MachineRspStepRejectionReason) -> String {
    match reason {
        MachineRspStepRejectionReason::SingleStepUnsupported => {
            "single-step-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::Fetch(_) => "fetch-rejected".to_owned(),
        MachineRspStepRejectionReason::MalformedMfc0Encoding => "mfc0-malformed".to_owned(),
        MachineRspStepRejectionReason::UnsupportedCop0Register { register_index } => {
            let control = match register_index {
                0 => "sp-mem-addr",
                1 => "sp-dram-addr",
                2 => "sp-rd-len",
                3 => "sp-wr-len",
                4 => "sp-status",
                5 => "sp-dma-full",
                6 => "sp-dma-busy",
                7 => "sp-semaphore",
                8 => "dpc-start",
                9 => "dpc-end",
                10 => "dpc-current",
                11 => "dpc-status",
                12 => "dpc-clock",
                13 => "dpc-bufbusy",
                14 => "dpc-pipebusy",
                15 => "dpc-tmem",
                _ => "cop0-reserved",
            };
            format!("mfc0-{control}-unsupported")
        }
        MachineRspStepRejectionReason::MalformedMtc0Encoding => "mtc0-malformed".to_owned(),
        MachineRspStepRejectionReason::Mtc0SourceUnavailable { .. } => {
            "mtc0-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::UnsupportedMtc0ControlRegister { .. } => {
            "mtc0-control-destination-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0SpStatusCommandMalformed => {
            "mtc0-sp-status-command-malformed".to_owned()
        }
        MachineRspStepRejectionReason::Mtc0SpStatusInterruptCommandUnsupported => {
            "mtc0-sp-status-interrupt-command-unsupported".to_owned()
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
        MachineRspStepRejectionReason::OriSourceUnavailable { .. } => {
            "ori-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::AndiSourceUnavailable { .. } => {
            "andi-source-unavailable".to_owned()
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
        MachineRspStepRejectionReason::BgezalSourceUnavailable { .. } => {
            "bgezal-source-unavailable".to_owned()
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
        MachineRspStepRejectionReason::ScalarSwBaseUnavailable { .. } => {
            "sw-base-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::ScalarSwSourceUnavailable { source_gpr } => {
            format!("sw-source-r{source_gpr:02}-unavailable")
        }
        MachineRspStepRejectionReason::ScalarSwAddressMisaligned { .. } => {
            "sw-address-misaligned".to_owned()
        }
        MachineRspStepRejectionReason::ScalarSwDmemRangeMalformed { .. } => {
            "sw-dmem-range-malformed".to_owned()
        }
        MachineRspStepRejectionReason::ScalarLoadUnsupported { .. } => {
            "scalar-load-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::ScalarStoreUnsupported { opcode } => {
            format!("scalar-store-opcode-{opcode:02x}-unsupported")
        }
        MachineRspStepRejectionReason::MalformedSllEncoding => "sll-malformed".to_owned(),
        MachineRspStepRejectionReason::SllSourceUnavailable { .. } => {
            "sll-source-unavailable".to_owned()
        }
        MachineRspStepRejectionReason::ScalarFunctionUnsupported { function } => {
            let identity = match function {
                0x02 => "srl",
                0x03 => "sra",
                0x04 => "sllv",
                0x06 => "srlv",
                0x07 => "srav",
                0x08 => "jr",
                0x09 => "jalr",
                0x20 => "add",
                0x21 => "addu",
                0x22 => "sub",
                0x23 => "subu",
                0x24 => "and",
                0x25 => "or",
                0x26 => "xor",
                0x27 => "nor",
                0x2a => "slt",
                0x2b => "sltu",
                _ => "reserved",
            };
            format!("scalar-function-{identity}-unsupported")
        }
        MachineRspStepRejectionReason::ScalarOpcodeUnsupported { opcode } => {
            let identity = match opcode {
                0x03 => "jal",
                0x04 => "beq",
                0x06 => "blez",
                0x07 => "bgtz",
                0x09 => "addiu",
                0x0a => "slti",
                0x0b => "sltiu",
                0x0c => "andi",
                _ => "reserved",
            };
            format!("scalar-opcode-{identity}-unsupported")
        }
        MachineRspStepRejectionReason::VsubElementUnsupported { .. } => {
            "vsub-element-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::VaddcElementUnsupported { .. } => {
            "vaddc-element-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::VxorElementUnsupported { .. } => {
            "vxor-element-unsupported".to_owned()
        }
        MachineRspStepRejectionReason::UnrepresentedInstruction { class } => {
            format!("unrepresented-{class:?}-instruction")
        }
        MachineRspStepRejectionReason::VectorFunctionUnsupported { function, element } => {
            format!("vector-function-{function:02x}-element-{element:02x}-unsupported")
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
                observe_rsp_pressure: false,
            }
        );
        assert_eq!(
            parse_arguments([OsString::from("/private/input.z64")].into_iter()).unwrap(),
            UserCartridgeProbeArguments {
                input_path: PathBuf::from("/private/input.z64"),
                pif_rom_path: None,
                max_steps: DEFAULT_MAX_STEPS,
                observe_rsp_pressure: false,
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
                    OsString::from("--rsp-pressure"),
                ]
                .into_iter()
            )
            .unwrap(),
            UserCartridgeProbeArguments {
                input_path: PathBuf::from("/private/input.z64"),
                pif_rom_path: Some(PathBuf::from("/private/pif.bin")),
                max_steps: 4321,
                observe_rsp_pressure: true,
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
    fn rsp_sw_source_rejection_names_only_the_architectural_register() {
        assert_eq!(
            redacted_rsp_rejection_category(
                MachineRspStepRejectionReason::ScalarSwSourceUnavailable { source_gpr: 4 }
            ),
            "sw-source-r04-unavailable"
        );
    }

    #[test]
    fn rsp_mfc0_rejection_names_only_the_architectural_control_source() {
        assert_eq!(
            redacted_rsp_rejection_category(
                MachineRspStepRejectionReason::UnsupportedCop0Register { register_index: 4 }
            ),
            "mfc0-sp-status-unsupported"
        );
        assert_eq!(
            redacted_rsp_rejection_category(
                MachineRspStepRejectionReason::UnsupportedCop0Register { register_index: 10 }
            ),
            "mfc0-dpc-current-unsupported"
        );
    }

    #[test]
    fn rsp_scalar_rejections_name_only_public_instruction_identities() {
        assert_eq!(
            redacted_rsp_rejection_category(
                MachineRspStepRejectionReason::ScalarFunctionUnsupported { function: 0x24 }
            ),
            "scalar-function-and-unsupported"
        );
        assert_eq!(
            redacted_rsp_rejection_category(
                MachineRspStepRejectionReason::ScalarOpcodeUnsupported { opcode: 0x09 }
            ),
            "scalar-opcode-addiu-unsupported"
        );
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
        assert!(source.contains(" owner_region={}"));
        assert!(source.contains(" direct_segment={}"));
        assert!(source.contains(" rdram_capacity_relation={}"));
        assert!(source.contains(" base_source={}"));
        assert!(source.contains("category=cop1-control-{}"));
        let forbidden_load_address_field = ["load-word-", " cpu_address="].concat();
        assert!(!source.contains(&forbidden_load_address_field));
        let forbidden_synthetic_install =
            ["install_public_synthetic_cold_", "x105_bootstrap"].concat();
        assert!(!source.contains(&forbidden_synthetic_install));
    }

    #[test]
    fn direct_load_owner_region_is_value_free_and_architecture_bounded() {
        assert_eq!(redacted_direct_cpu_owner_region(0x8000_0400), "rdram");
        assert_eq!(
            redacted_direct_cpu_owner_region(0xa3f0_0000),
            "rdram-registers"
        );
        assert_eq!(redacted_direct_cpu_owner_region(0xa400_0000), "sp");
        assert_eq!(redacted_direct_cpu_owner_region(0xa410_0000), "dpc");
        assert_eq!(redacted_direct_cpu_owner_region(0xa450_0000), "ai");
        assert_eq!(redacted_direct_cpu_owner_region(0xb000_0000), "cartridge");
        assert_eq!(redacted_direct_cpu_owner_region(0xbfc0_0000), "pif");
        assert_eq!(redacted_direct_cpu_segment(0x8000_1000), "kseg0");
        assert_eq!(redacted_direct_cpu_segment(0xa000_1000), "kseg1");
        assert_eq!(
            redacted_rdram_capacity_relation(0x8000_1000),
            "within-represented-capacity"
        );
        assert_eq!(
            redacted_rdram_capacity_relation(0x8040_0000),
            "outside-represented-capacity"
        );
    }

    #[test]
    fn clean_room_load_base_source_reports_lineage_without_register_values() {
        let mut machine = generated_hle_machine();
        machine
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();

        assert_eq!(redacted_load_base_source(&machine, 0), "architectural-zero");
        assert_eq!(
            redacted_load_base_source(&machine, 1),
            "clean-room-public-profile"
        );
        assert_eq!(
            redacted_load_base_source(&machine, 2),
            "cartridge-payload-derived"
        );
        assert_eq!(
            redacted_load_base_source(&machine, 9),
            "cartridge-entry-derived"
        );

        machine.step().unwrap();
        assert_eq!(redacted_load_base_source(&machine, 2), "instruction-result");
    }

    #[test]
    fn cop1_control_rejections_are_value_free_categories() {
        assert_eq!(
            redacted_cop1_control_rejection_category(
                MachineCop1ControlTransferRejectionReason::CoprocessorUnusable { status: u32::MAX }
            ),
            "coprocessor-unusable"
        );
        assert_eq!(
            redacted_cop1_control_rejection_category(
                MachineCop1ControlTransferRejectionReason::MalformedEncoding { low_bits: u16::MAX }
            ),
            "malformed-encoding"
        );
        assert_eq!(
            redacted_cop1_control_rejection_category(
                MachineCop1ControlTransferRejectionReason::UnsupportedControlRegister {
                    register_index: 31,
                }
            ),
            "unsupported-control-register"
        );
        assert_eq!(
            redacted_cop1_control_rejection_category(
                MachineCop1ControlTransferRejectionReason::StateUnavailable
            ),
            "state-unavailable"
        );
        assert_eq!(
            redacted_cop1_control_rejection_category(
                MachineCop1ControlTransferRejectionReason::SourceUnavailable {
                    register_index: 1,
                    source: MachineBootstrapGprSource::ArchitecturalZero,
                }
            ),
            "source-unavailable"
        );
    }

    #[test]
    fn cop0_tlb_rejections_are_value_free_categories() {
        let machine = generated_hle_machine();
        for (error, expected) in [
            (
                MachineCop0TlbOperationError::IndexUnavailable,
                "cpu-local-cop0-tlb-index-unavailable",
            ),
            (
                MachineCop0TlbOperationError::IndexOutOfRange { index: u8::MAX },
                "cpu-local-cop0-tlb-index-out-of-range",
            ),
            (
                MachineCop0TlbOperationError::EntryUnavailable { index: u8::MAX },
                "cpu-local-cop0-tlb-entry-unavailable",
            ),
            (
                MachineCop0TlbOperationError::WorkingRegistersUnavailable,
                "cpu-local-cop0-tlb-page-mask-unavailable",
            ),
        ] {
            assert_eq!(
                redacted_cpu_local_invocation_rejection_category(
                    &machine,
                    MachineStepCpuLocalInvocationRejection::Cop0Tlb {
                        identity: CpuInstructionIdentity::Cop0Tlbwi,
                        error,
                    }
                ),
                expected
            );
            assert!(!expected.contains("255"));
        }
    }

    #[test]
    fn pi_timing_pressure_names_only_public_domain_and_field() {
        for (domain, field, expected) in [
            (
                MachinePiDomain::One,
                MachinePiDomainTimingField::Latency,
                "pi-domain-one-latency-unavailable",
            ),
            (
                MachinePiDomain::One,
                MachinePiDomainTimingField::PulseWidth,
                "pi-domain-one-pulse-width-unavailable",
            ),
            (
                MachinePiDomain::Two,
                MachinePiDomainTimingField::PageSize,
                "pi-domain-two-page-size-unavailable",
            ),
            (
                MachinePiDomain::Two,
                MachinePiDomainTimingField::Release,
                "pi-domain-two-release-unavailable",
            ),
        ] {
            assert_eq!(
                redacted_pi_timing_register_category(MachinePiDomainTimingRegister::new(
                    domain, field
                )),
                expected
            );
        }
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
