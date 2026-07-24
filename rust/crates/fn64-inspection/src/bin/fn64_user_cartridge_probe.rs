use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use fn64_core::{
    load_cartridge, rom_source_layout_name, CpuInstructionIdentity, Machine,
    MachineCpuInstructionInspection, MachineRepresentedStepOutcome, MachineSpStatusState,
};

const DEFAULT_MAX_STEPS: u64 = 100_000_000;
const MAX_RUNTIME_FRONTIERS: usize = 128;

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
    let (input_path, max_steps) = parse_arguments(std::env::args_os().skip(1))?;
    let basename = redacted_basename(&input_path);
    let source_bytes = std::fs::read(&input_path)
        .map_err(|error| format!("input read failed for {basename}: {error}"))?;
    let source_size = source_bytes.len();
    let cartridge = load_cartridge(source_bytes)
        .map_err(|error| format!("input normalization failed for {basename}: {error}"))?;
    let source_layout = cartridge.source_layout();
    let normalized_size = cartridge.size_bytes();
    let entrypoint = cartridge.metadata().entry_point;

    let mut machine = Machine::from_cartridge(cartridge);
    machine.install_public_synthetic_cold_x105_bootstrap();
    machine
        .stage_cartridge_bootstrap()
        .map_err(|error| format!("cold x105 bootstrap staging failed: {error}"))?;

    let mut attempted_steps = 0_u64;
    let mut committed_steps = 0_u64;
    let mut cartridge_runtime_committed_steps = 0_u64;
    let mut entry_executions = 0_u64;
    let mut first_entry: Option<MachineCpuInstructionInspection> = None;
    let mut task_boundary: Option<(MachineCpuInstructionInspection, MachineSpStatusState, bool)> =
        None;
    let mut runtime_frontiers = Vec::new();
    let mut prior_sp_dma_records = machine.sp_dma_record_count();

    while attempted_steps < max_steps {
        let pc = machine.cpu().pc();
        let previous_halt = machine.sp_status_state().map(MachineSpStatusState::halt);
        let needs_inspection = pc == entrypoint
            || (entry_executions != 0
                && machine.sp_dma_record_count() != 0
                && previous_halt == Some(true));
        let inspection = needs_inspection
            .then(|| machine.inspect_current_cpu_instruction())
            .transpose()
            .map_err(|error| {
                format!("bounded instruction inspection failed at PC 0x{pc:08X}: {error}")
            })?;

        let outcome = machine.step().map_err(|error| {
            format!("Machine::step stopped before the first RSP task at PC 0x{pc:08X}: {error}")
        })?;
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
                && previous_halt == Some(true)
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

    let first_entry =
        first_entry.ok_or_else(|| "cartridge entrypoint was not executed".to_owned())?;
    let (task_instruction, task_status, halt_before) = task_boundary.ok_or_else(|| {
        format!("step ceiling {max_steps} reached before the first RSP task submission")
    })?;

    println!("fn64 user cartridge probe");
    println!("result: ok");
    println!("classification: USER_PROVIDED_CARTRIDGE_MACHINE_STEP_COMPOSITION");
    println!("input.basename: {basename}");
    println!("input.source_bytes: {source_size}");
    println!(
        "input.byte_order: {}",
        rom_source_layout_name(source_layout)
    );
    println!("input.normalized_bytes: {normalized_size}");
    println!("cartridge.entrypoint: 0x{entrypoint:08X}");
    println!(
        "cartridge.first_instruction.word: 0x{:08X}",
        first_entry.fields().raw().bits()
    );
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
        "rsp_task.start_instruction_word: 0x{:08X}",
        task_instruction.fields().raw().bits()
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
    println!(
        "rsp_task.status_command: 0x{:08X}",
        task_status.command_word()
    );
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
) -> Result<(PathBuf, u64), String> {
    let input_path = arguments.next().map(PathBuf::from).ok_or_else(|| {
        "usage: fn64_user_cartridge_probe <cartridge-path> [max-steps]".to_owned()
    })?;
    let max_steps = match arguments.next() {
        Some(raw) => raw
            .to_str()
            .ok_or_else(|| "max-steps must be UTF-8 decimal".to_owned())?
            .parse::<u64>()
            .map_err(|_| "max-steps must be a positive decimal integer".to_owned())?,
        None => DEFAULT_MAX_STEPS,
    };
    if max_steps == 0 {
        return Err("max-steps must be greater than zero".to_owned());
    }
    if arguments.next().is_some() {
        return Err("usage: fn64_user_cartridge_probe <cartridge-path> [max-steps]".to_owned());
    }
    Ok((input_path, max_steps))
}

fn redacted_basename(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<non-utf8-basename>")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_path_parser_owns_only_path_and_positive_step_ceiling() {
        assert_eq!(
            parse_arguments(
                [OsString::from("/private/input.z64"), OsString::from("1234"),].into_iter(),
            )
            .unwrap(),
            (PathBuf::from("/private/input.z64"), 1234)
        );
        assert_eq!(
            parse_arguments([OsString::from("/private/input.z64")].into_iter()).unwrap(),
            (PathBuf::from("/private/input.z64"), DEFAULT_MAX_STEPS)
        );
        assert!(parse_arguments(
            [OsString::from("/private/input.z64"), OsString::from("0")].into_iter()
        )
        .is_err());
    }

    #[test]
    fn redacted_input_name_never_exposes_parent_directories() {
        assert_eq!(
            redacted_basename(Path::new("/private/collection/input.z64")),
            "input.z64"
        );
    }
}
