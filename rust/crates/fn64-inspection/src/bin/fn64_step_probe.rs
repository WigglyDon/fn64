use core::fmt;
use std::process::ExitCode;

use fn64_core::cpu::address::CpuAddress;
use fn64_core::{
    load_cartridge, Cartridge, CartridgeLoadError, CpuAddressErrorKind, CpuInstructionIdentity,
    Machine, MachineCartridgeBootstrapError, MachineCpuInstructionFetchError,
    MachineLoadWordRejectionReason, MachineLoadWordTarget, MachinePifIpl2HandoffBootMedium,
    MachinePifIpl2HandoffResetKind, MachinePifIpl3Family, MachinePifVersionBit,
    MachineRepresentedStepError, MachineRepresentedStepOutcome, MachineSpDmemLoadWordProvenance,
    MachineStepCadenceSource, MachineStepControlFlowAction, MachineStepCountAction,
    MachineStepNoEffectExecutedInstructionCategory, MachineStepStoppedInstructionCategory,
    MachineStepUnsupportedInstructionCategory, PifFirmwareValidationError, PifIpl2Profile,
    RdramAccessError, SpDmemOffset, PIF_BOOT_ROM_SIZE_BYTES,
};

const DIRECT_CPU_PC: u32 = 0x8000_0000;
const GENERAL_EXCEPTION_VECTOR_PC: u32 = 0x8000_0180;
const GENERAL_EXCEPTION_VECTOR_NEXT_PC: u32 = 0x8000_0184;

const STEP_PROBE_OUTPUT: &str = "fn64 rust step probe\
\ncase: cpu-local-committed ok\
\ncase: cpu-local-arithmetic-overflow ok\
\ncase: sync-no-effect ok\
\ncase: syscall-stopped ok\
\ncase: break-stopped ok\
\ncase: unsupported-rollback ok\
\ncase: instruction-fetch-adel ok\
\ncase: source-clear-rejection ok\
\ncase: sp-dmem-lw-committed ok\
\ncase: sp-dmem-lw-unknown-rejection ok\
\ncase: sp-dmem-lw-delay-slot-adel ok\
\ncase: generated-x105-next-frontier ok\
\ncase: control-flow-taken-delay-slot ok\
\ncase: control-flow-untaken-delay-slot ok\
\ncase: control-flow-jal-link ok\
\ncase: control-flow-jalr-alias ok\
\ncase: control-flow-delay-slot-exception ok\
\ncase: control-flow-branch-in-delay-slot-rejection ok\
\nno-window: ok\
\nresult: ok\n";

#[derive(Debug)]
enum StepProbeError {
    Rdram {
        case: &'static str,
        source: RdramAccessError,
    },
    Cartridge {
        case: &'static str,
        source: CartridgeLoadError,
    },
    PifFirmware {
        case: &'static str,
        source: PifFirmwareValidationError,
    },
    Bootstrap {
        case: &'static str,
        source: MachineCartridgeBootstrapError,
    },
    Step {
        case: &'static str,
        source: MachineRepresentedStepError,
    },
    Assertion {
        case: &'static str,
        check: &'static str,
    },
}

impl fmt::Display for StepProbeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rdram { case, source } => {
                write!(f, "{case}: synthetic RDRAM seed failed: {source}")
            }
            Self::Cartridge { case, source } => {
                write!(f, "{case}: generated cartridge setup failed: {source}")
            }
            Self::PifFirmware { case, source } => {
                write!(f, "{case}: generated PIF-shaped input failed: {source}")
            }
            Self::Bootstrap { case, source } => {
                write!(f, "{case}: generated bootstrap setup failed: {source}")
            }
            Self::Step { case, source } => write!(f, "{case}: Machine::step failed: {source}"),
            Self::Assertion { case, check } => write!(f, "{case}: assertion failed: {check}"),
        }
    }
}

impl std::error::Error for StepProbeError {}

fn main() -> ExitCode {
    if std::env::args_os().len() != 1 {
        eprintln!("usage: fn64_step_probe");
        return ExitCode::from(2);
    }

    match run_step_probe() {
        Ok(()) => {
            print!("{STEP_PROBE_OUTPUT}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("fn64 rust step probe");
            eprintln!("result: fail");
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run_step_probe() -> Result<(), StepProbeError> {
    probe_cpu_local_committed_success()?;
    probe_cpu_local_arithmetic_overflow()?;
    probe_sync_no_effect()?;
    probe_stopped_instruction(
        "syscall-stopped",
        0x0000_000c,
        CpuInstructionIdentity::SpecialSyscall,
        MachineStepStoppedInstructionCategory::Syscall,
    )?;
    probe_stopped_instruction(
        "break-stopped",
        0x0000_000d,
        CpuInstructionIdentity::SpecialBreak,
        MachineStepStoppedInstructionCategory::Break,
    )?;
    probe_unsupported_rollback()?;
    probe_instruction_fetch_adel()?;
    probe_source_clear_rejection()?;
    probe_generated_x105_sp_dmem_lw_frontier()?;
    probe_sp_dmem_lw_unknown_rejection()?;
    probe_sp_dmem_lw_delay_slot_adel()?;
    probe_control_flow_taken_delay_slot()?;
    probe_control_flow_untaken_delay_slot()?;
    probe_control_flow_jal_link()?;
    probe_control_flow_jalr_alias()?;
    probe_control_flow_delay_slot_exception()?;
    probe_control_flow_branch_in_delay_slot_rejection()?;
    Ok(())
}

fn probe_cpu_local_committed_success() -> Result<(), StepProbeError> {
    const CASE: &str = "cpu-local-committed";
    let mut machine = synthetic_direct_machine_with_instruction(CASE, 0x3c02_8000)?;

    let outcome = step(&mut machine, CASE)?;
    match outcome {
        MachineRepresentedStepOutcome::CpuLocalCommitted {
            identity,
            cadence_plan,
        } => {
            require(
                CASE,
                identity == CpuInstructionIdentity::Lui,
                "LUI identity",
            )?;
            require(
                CASE,
                cadence_plan.source() == MachineStepCadenceSource::CommittedInstruction,
                "committed cadence source",
            )?;
            require(
                CASE,
                cadence_plan.control_flow_action() == MachineStepControlFlowAction::CommitStaged,
                "committed control-flow action",
            )?;
            require(
                CASE,
                cadence_plan.count_action() == MachineStepCountAction::Advance,
                "committed Count action",
            )?;
        }
        _ => return assertion(CASE, "CPU-local committed outcome"),
    }

    require(
        CASE,
        machine.cpu().gpr(2) == Some(0xffff_ffff_8000_0000),
        "visible LUI writeback",
    )?;
    require(CASE, machine.cpu().pc() == 0x8000_0004, "committed pc")?;
    require(
        CASE,
        machine.cpu().next_pc() == 0x8000_0008,
        "committed next_pc",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 1,
        "Count advanced exactly once",
    )
}

fn probe_cpu_local_arithmetic_overflow() -> Result<(), StepProbeError> {
    const CASE: &str = "cpu-local-arithmetic-overflow";
    let mut machine = Machine::from_cartridge(Cartridge::default());
    for (offset, instruction) in [
        (0x00, 0x3c02_7fff),
        (0x04, 0x3442_ffff),
        (0x08, 0x3c03_1234),
        (0x0c, 0x2043_0001),
    ] {
        seed_instruction(&mut machine, CASE, offset, instruction)?;
    }
    machine.stage_cpu_pc(DIRECT_CPU_PC);

    for _ in 0..3 {
        require(
            CASE,
            matches!(
                step(&mut machine, CASE)?,
                MachineRepresentedStepOutcome::CpuLocalCommitted { .. }
            ),
            "synthetic register setup committed through Machine::step",
        )?;
    }

    let destination_before = machine.cpu().gpr(3);
    let count_before = machine.cpu().cop0_count();
    let normal_next_pc = machine.cpu().next_pc();
    let bad_vaddr_before = machine.cpu().cop0_bad_vaddr();
    let outcome = step(&mut machine, CASE)?;

    match outcome {
        MachineRepresentedStepOutcome::ArithmeticOverflowException { identity } => require(
            CASE,
            identity == CpuInstructionIdentity::Addi,
            "ADDI overflow identity",
        )?,
        _ => return assertion(CASE, "arithmetic-overflow exception outcome"),
    }

    require(
        CASE,
        machine.cpu().gpr(3) == destination_before,
        "no destination writeback",
    )?;
    require(
        CASE,
        machine.cpu().pc() != normal_next_pc,
        "normal cadence not committed",
    )?;
    require(
        CASE,
        machine.cpu().pc() == GENERAL_EXCEPTION_VECTOR_PC,
        "exception vector pc",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == GENERAL_EXCEPTION_VECTOR_NEXT_PC,
        "exception vector next_pc",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == count_before,
        "Count did not advance",
    )?;
    require(
        CASE,
        machine.cpu().cop0_exception_code() == 12,
        "arithmetic-overflow exception code",
    )?;
    require(
        CASE,
        machine.cpu().cop0_epc() == 0x8000_000c,
        "arithmetic-overflow EPC",
    )?;
    require(
        CASE,
        machine.cpu().cop0_status() == 0x0000_0002,
        "arithmetic-overflow EXL state",
    )?;
    require(
        CASE,
        machine.cpu().cop0_bad_vaddr() == bad_vaddr_before,
        "BadVAddr not written",
    )
}

fn probe_sync_no_effect() -> Result<(), StepProbeError> {
    const CASE: &str = "sync-no-effect";
    let mut machine = synthetic_direct_machine_with_instruction(CASE, 0x0000_000f)?;

    let outcome = step(&mut machine, CASE)?;
    match outcome {
        MachineRepresentedStepOutcome::NoEffectCommitted {
            instruction,
            cadence_plan,
        } => {
            require(
                CASE,
                instruction.category() == MachineStepNoEffectExecutedInstructionCategory::Sync,
                "SYNC no-effect category",
            )?;
            require(
                CASE,
                cadence_plan.source() == MachineStepCadenceSource::CommittedInstruction,
                "SYNC committed cadence",
            )?;
        }
        _ => return assertion(CASE, "SYNC no-effect committed outcome"),
    }

    assert_normal_single_step_cadence(&machine, CASE)
}

fn probe_stopped_instruction(
    case: &'static str,
    raw: u32,
    expected_identity: CpuInstructionIdentity,
    expected_category: MachineStepStoppedInstructionCategory,
) -> Result<(), StepProbeError> {
    let mut machine = synthetic_direct_machine_with_instruction(case, raw)?;

    let outcome = step(&mut machine, case)?;
    match outcome {
        MachineRepresentedStepOutcome::Stopped {
            instruction,
            cadence_plan,
        } => {
            require(
                case,
                instruction.identity() == expected_identity,
                "stopped identity",
            )?;
            require(
                case,
                instruction.category() == expected_category,
                "stopped category",
            )?;
            require(
                case,
                cadence_plan.source() == MachineStepCadenceSource::StoppedInstruction,
                "stopped cadence source",
            )?;
        }
        _ => return assertion(case, "stopped outcome"),
    }

    assert_normal_single_step_cadence(&machine, case)?;
    require(
        case,
        machine.cpu().cop0_status() == 0,
        "no exception status",
    )?;
    require(case, machine.cpu().cop0_epc() == 0, "no exception EPC")?;
    require(
        case,
        machine.cpu().cop0_exception_code() == 0,
        "no SYSCALL/BREAK exception entry",
    )
}

fn probe_unsupported_rollback() -> Result<(), StepProbeError> {
    const CASE: &str = "unsupported-rollback";
    let mut machine = synthetic_direct_machine_with_instruction(CASE, 0x7000_1234)?;
    let pc_before = machine.cpu().pc();
    let next_pc_before = machine.cpu().next_pc();

    let outcome = step(&mut machine, CASE)?;
    match outcome {
        MachineRepresentedStepOutcome::Unsupported {
            instruction,
            cadence_plan,
        } => {
            require(
                CASE,
                instruction.category() == MachineStepUnsupportedInstructionCategory::UnknownPrimary,
                "unsupported category",
            )?;
            require(
                CASE,
                instruction.raw().bits() == 0x7000_1234,
                "unsupported instruction word",
            )?;
            require(
                CASE,
                cadence_plan.control_flow_action() == MachineStepControlFlowAction::RestoreSnapshot,
                "unsupported control-flow rollback",
            )?;
            require(
                CASE,
                cadence_plan.count_action() == MachineStepCountAction::DoNotAdvance,
                "unsupported Count action",
            )?;
        }
        _ => return assertion(CASE, "unsupported outcome"),
    }

    assert_rejected_state_unchanged(&machine, CASE, pc_before, next_pc_before)
}

fn probe_instruction_fetch_adel() -> Result<(), StepProbeError> {
    const CASE: &str = "instruction-fetch-adel";
    const FAULT_PC: u32 = 0xa400_0042;
    let mut machine = Machine::from_cartridge(Cartridge::default());
    machine.stage_cpu_pc(FAULT_PC);
    let count_before = machine.cpu().cop0_count();
    let normal_next_pc = machine.cpu().next_pc();

    let outcome = step(&mut machine, CASE)?;
    match outcome {
        MachineRepresentedStepOutcome::InstructionFetchAddressError { plan, cadence_plan } => {
            require(
                CASE,
                plan.cpu_address() == CpuAddress::new(FAULT_PC),
                "AdEL fault address",
            )?;
            require(
                CASE,
                plan.bad_vaddr() == CpuAddress::new(FAULT_PC),
                "AdEL BadVAddr plan",
            )?;
            require(CASE, plan.cause_exception_code() == 4, "AdEL cause code")?;
            require(
                CASE,
                cadence_plan.source() == MachineStepCadenceSource::FetchAddressErrorException,
                "fetch AdEL cadence source",
            )?;
        }
        _ => return assertion(CASE, "instruction-fetch AdEL outcome"),
    }

    require(
        CASE,
        machine.cpu().pc() != normal_next_pc,
        "normal cadence not committed",
    )?;
    require(
        CASE,
        machine.cpu().pc() == GENERAL_EXCEPTION_VECTOR_PC,
        "AdEL exception vector pc",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == GENERAL_EXCEPTION_VECTOR_NEXT_PC,
        "AdEL exception vector next_pc",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == count_before,
        "AdEL Count did not advance",
    )?;
    require(
        CASE,
        machine.cpu().cop0_exception_code() == 4,
        "AdEL exception code",
    )?;
    require(CASE, machine.cpu().cop0_epc() == FAULT_PC, "AdEL EPC")?;
    require(
        CASE,
        machine.cpu().cop0_bad_vaddr() == FAULT_PC,
        "AdEL BadVAddr",
    )
}

fn probe_source_clear_rejection() -> Result<(), StepProbeError> {
    const CASE: &str = "source-clear-rejection";
    const REJECTED_PC: u32 = 0x4000_0000;
    let mut machine = Machine::from_cartridge(Cartridge::default());
    machine.stage_cpu_pc(REJECTED_PC);
    let pc_before = machine.cpu().pc();
    let next_pc_before = machine.cpu().next_pc();

    match machine.step() {
        Err(MachineRepresentedStepError::FetchRejected(
            MachineCpuInstructionFetchError::NonDirectUnsupported { cpu_address },
        )) => require(
            CASE,
            cpu_address == CpuAddress::new(REJECTED_PC),
            "source-clear rejected fetch address",
        )?,
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "source-clear fetch rejection"),
    }

    assert_rejected_state_unchanged(&machine, CASE, pc_before, next_pc_before)
}

fn probe_generated_x105_sp_dmem_lw_frontier() -> Result<(), StepProbeError> {
    const CASE: &str = "generated-x105-next-frontier";
    const GENERATED_SP_DMEM_WORD: u32 = 0x1357_9bdf;
    let words = [
        (0x40, special_word(29, 0, 9, 0x20)),
        (0x44, immediate_word(0x23, 9, 8, 0xf010)),
        (0x48, immediate_word(0x23, 11, 10, 0x0044)),
        (0x4c, special_word(10, 8, 10, 0x26)),
        (0x50, immediate_word(0x2b, 9, 10, 0xf010)),
        (0x84, GENERATED_SP_DMEM_WORD),
    ];
    let (mut machine, generated_sp_imem_word) = generated_cold_x105_machine(CASE, &words)?;
    let expected_sp_imem_value = sign_extend_word(generated_sp_imem_word);
    let expected_sp_dmem_value = sign_extend_word(GENERATED_SP_DMEM_WORD);

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::SpecialAdd,
    )?;
    require(
        CASE,
        machine.cpu().gpr(9) == Some(0xffff_ffff_a400_1ff0),
        "generated entry register copy",
    )?;

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::LoadWordCommitted {
            effective_address,
            target,
            destination_gpr,
            loaded_word,
            result_value,
            ..
        } => {
            require(
                CASE,
                effective_address == 0xffff_ffff_a400_1000,
                "generated SP-IMEM load address",
            )?;
            require(
                CASE,
                target == MachineLoadWordTarget::SpImem { offset: 0 },
                "generated SP-IMEM load target",
            )?;
            require(CASE, destination_gpr == 8, "generated SP-IMEM destination")?;
            require(
                CASE,
                loaded_word == generated_sp_imem_word,
                "generated SP-IMEM word",
            )?;
            require(
                CASE,
                result_value == expected_sp_imem_value,
                "generated SP-IMEM result",
            )?;
        }
        _ => return assertion(CASE, "generated SP-IMEM Lw outcome"),
    }

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::LoadWordCommitted {
            effective_address,
            target: MachineLoadWordTarget::SpDmem { offset, provenance },
            destination_gpr,
            loaded_word,
            result_value,
            ..
        } => {
            require(
                CASE,
                effective_address == 0xffff_ffff_a400_0084,
                "generated SP-DMEM load address",
            )?;
            require(
                CASE,
                offset == SpDmemOffset::new(0x84),
                "generated SP-DMEM offset",
            )?;
            require(
                CASE,
                provenance
                    == MachineSpDmemLoadWordProvenance::CartridgeBootstrap {
                        cartridge_offset: 0x84,
                    },
                "generated SP-DMEM cartridge provenance",
            )?;
            require(CASE, destination_gpr == 10, "generated SP-DMEM destination")?;
            require(
                CASE,
                loaded_word == GENERATED_SP_DMEM_WORD,
                "generated SP-DMEM word",
            )?;
            require(
                CASE,
                result_value == expected_sp_dmem_value,
                "generated SP-DMEM result",
            )?;
        }
        _ => return assertion(CASE, "generated SP-DMEM Lw outcome"),
    }

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::SpecialXor,
    )?;
    require(
        CASE,
        machine.cpu().gpr(10) == Some(expected_sp_dmem_value ^ expected_sp_imem_value),
        "generated transformed value",
    )?;
    require(CASE, machine.cpu().pc() == 0xa400_0050, "next frontier pc")?;
    require(
        CASE,
        machine.cpu().next_pc() == 0xa400_0054,
        "next frontier next_pc",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 4,
        "four committed steps",
    )?;

    let pc_before = machine.cpu().pc();
    let next_pc_before = machine.cpu().next_pc();
    let count_before = machine.cpu().cop0_count();
    let result_before = machine.cpu().gpr(10);
    match machine.step() {
        Err(MachineRepresentedStepError::UnrepresentedInstruction {
            identity: CpuInstructionIdentity::Sw,
            ..
        }) => {}
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "aligned Sw next frontier"),
    }
    require(
        CASE,
        machine.cpu().pc() == pc_before,
        "frontier pc rollback",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == next_pc_before,
        "frontier next_pc rollback",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == count_before,
        "frontier Count rollback",
    )?;
    require(
        CASE,
        machine.cpu().gpr(10) == result_before,
        "frontier GPR rollback",
    )
}

fn probe_sp_dmem_lw_unknown_rejection() -> Result<(), StepProbeError> {
    const CASE: &str = "sp-dmem-lw-unknown-rejection";
    let words = [
        (0x40, special_word(29, 0, 9, 0x20)),
        (0x44, immediate_word(0x23, 9, 8, 0xe010)),
    ];
    let cartridge = generated_cartridge(CASE, &words)?;
    let mut machine = Machine::from_cartridge(cartridge);
    machine
        .stage_cartridge_bootstrap()
        .map_err(|source| StepProbeError::Bootstrap { case: CASE, source })?;
    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::SpecialAdd,
    )?;
    let pc_before = machine.cpu().pc();
    let next_pc_before = machine.cpu().next_pc();
    let count_before = machine.cpu().cop0_count();

    match machine.step() {
        Err(MachineRepresentedStepError::LoadWordRejected(rejection)) => {
            require(
                CASE,
                rejection.effective_address() == 0xffff_ffff_a400_0000,
                "unknown SP-DMEM effective address",
            )?;
            require(
                CASE,
                rejection.target()
                    == Some(MachineLoadWordTarget::SpDmem {
                        offset: SpDmemOffset::new(0),
                        provenance: MachineSpDmemLoadWordProvenance::UnclassifiedMachineStorage,
                    }),
                "unknown SP-DMEM target",
            )?;
            require(
                CASE,
                rejection.reason()
                    == MachineLoadWordRejectionReason::SpDmemUnknown {
                        first_unknown_offset: 0,
                    },
                "unknown SP-DMEM rejection reason",
            )?;
        }
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "unknown SP-DMEM rejection"),
    }

    require(
        CASE,
        machine.cpu().pc() == pc_before,
        "unknown rejection pc",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == next_pc_before,
        "unknown rejection next_pc",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == count_before,
        "unknown rejection Count",
    )
}

fn probe_sp_dmem_lw_delay_slot_adel() -> Result<(), StepProbeError> {
    const CASE: &str = "sp-dmem-lw-delay-slot-adel";
    let words = [
        (0x40, branch_word(0x04, 0, 0, 1)),
        (0x44, immediate_word(0x23, 11, 10, 0x0045)),
        (0x48, special_word(0, 0, 0, 0x00)),
    ];
    let (mut machine, _) = generated_cold_x105_machine(CASE, &words)?;

    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::Beq)?;
    require(CASE, machine.cpu().cop0_count() == 1, "branch Count")?;

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::DataAddressError {
            identity,
            effective_address,
            address_error,
            cadence_plan,
        } => {
            require(
                CASE,
                identity == CpuInstructionIdentity::Lw,
                "slot Lw identity",
            )?;
            require(
                CASE,
                effective_address == 0xffff_ffff_a400_0085,
                "slot AdEL effective address",
            )?;
            require(
                CASE,
                address_error.exception_kind() == CpuAddressErrorKind::AddressErrorLoad,
                "slot AdEL kind",
            )?;
            require(
                CASE,
                address_error.cause_exception_code() == 4,
                "slot AdEL code",
            )?;
            require(
                CASE,
                cadence_plan.source() == MachineStepCadenceSource::EnteredException,
                "slot AdEL cadence",
            )?;
        }
        _ => return assertion(CASE, "delay-slot AdEL outcome"),
    }

    require(
        CASE,
        machine.cpu().cop0_epc() == 0xa400_0040,
        "slot AdEL EPC",
    )?;
    require(
        CASE,
        machine.cpu().cop0_exception_branch_delay(),
        "slot AdEL BD",
    )?;
    require(
        CASE,
        machine.cpu().cop0_bad_vaddr() == 0xa400_0085,
        "slot AdEL BadVAddr",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 1,
        "slot AdEL Count unchanged",
    )?;
    require(
        CASE,
        machine.cpu().pc() == GENERAL_EXCEPTION_VECTOR_PC,
        "slot AdEL vector pc",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == GENERAL_EXCEPTION_VECTOR_NEXT_PC,
        "slot AdEL vector next_pc",
    )?;
    require(
        CASE,
        machine.cpu_delay_slot_context().is_none(),
        "slot AdEL context cleared",
    )
}

fn probe_control_flow_taken_delay_slot() -> Result<(), StepProbeError> {
    const CASE: &str = "control-flow-taken-delay-slot";
    let mut machine = Machine::from_cartridge(Cartridge::default());
    seed_instruction(&mut machine, CASE, 0x00, branch_word(0x04, 0, 0, 2))?;
    seed_instruction(&mut machine, CASE, 0x04, immediate_word(0x09, 2, 2, 1))?;
    machine.stage_cpu_pc(DIRECT_CPU_PC);

    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::Beq)?;
    require(CASE, machine.cpu().pc() == 0x8000_0004, "taken slot pc")?;
    require(
        CASE,
        machine.cpu().next_pc() == 0x8000_000c,
        "taken branch target",
    )?;
    require(
        CASE,
        machine
            .cpu_delay_slot_context()
            .map(|context| context.branch_or_jump_pc())
            == Some(DIRECT_CPU_PC),
        "taken explicit delay-slot owner",
    )?;
    require(
        CASE,
        machine.cpu().gpr(2) == Some(0),
        "slot not executed early",
    )?;
    require(CASE, machine.cpu().cop0_count() == 1, "branch Count")?;

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::Addiu,
    )?;
    require(CASE, machine.cpu().gpr(2) == Some(1), "slot executed once")?;
    require(CASE, machine.cpu().pc() == 0x8000_000c, "target after slot")?;
    require(
        CASE,
        machine.cpu_delay_slot_context().is_none(),
        "taken slot context cleared",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 2,
        "branch plus slot Count",
    )
}

fn probe_control_flow_untaken_delay_slot() -> Result<(), StepProbeError> {
    const CASE: &str = "control-flow-untaken-delay-slot";
    let mut machine = Machine::from_cartridge(Cartridge::default());
    seed_instruction(&mut machine, CASE, 0x00, branch_word(0x05, 0, 0, 2))?;
    seed_instruction(&mut machine, CASE, 0x04, immediate_word(0x09, 3, 3, 1))?;
    machine.stage_cpu_pc(DIRECT_CPU_PC);

    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::Bne)?;
    require(CASE, machine.cpu().pc() == 0x8000_0004, "untaken slot pc")?;
    require(
        CASE,
        machine.cpu().next_pc() == 0x8000_0008,
        "untaken fall-through",
    )?;
    require(
        CASE,
        machine.cpu_delay_slot_context().is_some(),
        "untaken explicit delay-slot context",
    )?;
    require(
        CASE,
        machine.cpu().gpr(3) == Some(0),
        "slot not executed early",
    )?;

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::Addiu,
    )?;
    require(
        CASE,
        machine.cpu().gpr(3) == Some(1),
        "untaken slot executed once",
    )?;
    require(
        CASE,
        machine.cpu().pc() == 0x8000_0008,
        "fall-through after slot",
    )?;
    require(
        CASE,
        machine.cpu_delay_slot_context().is_none(),
        "untaken slot context cleared",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 2,
        "untaken two-step Count",
    )
}

fn probe_control_flow_jal_link() -> Result<(), StepProbeError> {
    const CASE: &str = "control-flow-jal-link";
    const TARGET: u32 = 0x8000_0010;
    const LINK: u64 = 0xffff_ffff_8000_0008;
    let mut machine = Machine::from_cartridge(Cartridge::default());
    seed_instruction(&mut machine, CASE, 0x00, jump_word(0x03, TARGET))?;
    seed_instruction(&mut machine, CASE, 0x04, special_word(31, 0, 5, 0x21))?;
    machine.stage_cpu_pc(DIRECT_CPU_PC);

    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::Jal)?;
    require(CASE, machine.cpu().gpr(31) == Some(LINK), "JAL link value")?;
    require(CASE, machine.cpu().next_pc() == TARGET, "JAL target")?;
    require(
        CASE,
        machine.cpu().gpr(5) == Some(0),
        "slot not executed early",
    )?;

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::SpecialAddu,
    )?;
    require(
        CASE,
        machine.cpu().gpr(5) == Some(LINK),
        "slot observed link",
    )?;
    require(CASE, machine.cpu().pc() == TARGET, "JAL target after slot")?;
    require(CASE, machine.cpu().cop0_count() == 2, "JAL plus slot Count")
}

fn probe_control_flow_jalr_alias() -> Result<(), StepProbeError> {
    const CASE: &str = "control-flow-jalr-alias";
    const TARGET: u32 = 0x8000_0020;
    const LINK: u64 = 0xffff_ffff_8000_0010;
    let mut machine = Machine::from_cartridge(Cartridge::default());
    for (offset, instruction) in [
        (0x00, immediate_word(0x0f, 0, 4, 0x8000)),
        (0x04, immediate_word(0x0d, 4, 4, 0x0020)),
        (0x08, register_jump_word(4, 4, 0x09)),
        (0x0c, special_word(4, 0, 5, 0x21)),
    ] {
        seed_instruction(&mut machine, CASE, offset, instruction)?;
    }
    machine.stage_cpu_pc(DIRECT_CPU_PC);

    for expected in [CpuInstructionIdentity::Lui, CpuInstructionIdentity::Ori] {
        require_committed_identity(CASE, step(&mut machine, CASE)?, expected)?;
    }
    require(
        CASE,
        machine.cpu().gpr(4) == Some(0xffff_ffff_8000_0020),
        "JALR old source staged",
    )?;

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::SpecialJalr,
    )?;
    require(
        CASE,
        machine.cpu().gpr(4) == Some(LINK),
        "JALR alias link write",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == TARGET,
        "JALR used old aliased source target",
    )?;

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::SpecialAddu,
    )?;
    require(
        CASE,
        machine.cpu().gpr(5) == Some(LINK),
        "JALR slot observed link",
    )?;
    require(CASE, machine.cpu().pc() == TARGET, "JALR target after slot")?;
    require(
        CASE,
        machine.cpu().cop0_count() == 4,
        "setup JALR slot Count",
    )
}

fn probe_control_flow_delay_slot_exception() -> Result<(), StepProbeError> {
    const CASE: &str = "control-flow-delay-slot-exception";
    const BRANCH_PC: u32 = 0x8000_0008;
    const TARGET: u32 = 0x8000_0020;
    let mut machine = Machine::from_cartridge(Cartridge::default());
    for (offset, instruction) in [
        (0x00, immediate_word(0x0f, 0, 2, 0x7fff)),
        (0x04, immediate_word(0x0d, 2, 2, 0xffff)),
        (0x08, jump_word(0x02, TARGET)),
        (0x0c, immediate_word(0x08, 2, 3, 1)),
    ] {
        seed_instruction(&mut machine, CASE, offset, instruction)?;
    }
    machine.stage_cpu_pc(DIRECT_CPU_PC);
    for expected in [CpuInstructionIdentity::Lui, CpuInstructionIdentity::Ori] {
        require_committed_identity(CASE, step(&mut machine, CASE)?, expected)?;
    }
    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::J)?;
    let count_before_fault = machine.cpu().cop0_count();
    let destination_before = machine.cpu().gpr(3);

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::ArithmeticOverflowException { identity } => require(
            CASE,
            identity == CpuInstructionIdentity::Addi,
            "slot overflow identity",
        )?,
        _ => return assertion(CASE, "slot overflow outcome"),
    }
    require(
        CASE,
        machine.cpu().cop0_epc() == BRANCH_PC,
        "slot overflow EPC",
    )?;
    require(
        CASE,
        machine.cpu().cop0_exception_branch_delay(),
        "slot overflow BD",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == count_before_fault,
        "slot overflow Count unchanged",
    )?;
    require(
        CASE,
        machine.cpu().gpr(3) == destination_before,
        "slot overflow no destination",
    )?;
    require(
        CASE,
        machine.cpu().pc() == GENERAL_EXCEPTION_VECTOR_PC,
        "slot overflow vector",
    )?;
    require(
        CASE,
        machine.cpu().pc() != TARGET,
        "slot overflow no target commit",
    )?;
    require(
        CASE,
        machine.cpu_delay_slot_context().is_none(),
        "slot overflow context consumed",
    )
}

fn probe_control_flow_branch_in_delay_slot_rejection() -> Result<(), StepProbeError> {
    const CASE: &str = "control-flow-branch-in-delay-slot-rejection";
    const TARGET: u32 = 0x8000_0020;
    let mut machine = Machine::from_cartridge(Cartridge::default());
    seed_instruction(&mut machine, CASE, 0x00, jump_word(0x02, TARGET))?;
    seed_instruction(&mut machine, CASE, 0x04, jump_word(0x03, 0x8000_0040))?;
    machine.stage_cpu_pc(DIRECT_CPU_PC);

    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::J)?;
    let pc_before = machine.cpu().pc();
    let next_pc_before = machine.cpu().next_pc();
    let context_before = machine.cpu_delay_slot_context();
    let count_before = machine.cpu().cop0_count();
    let link_before = machine.cpu().gpr(31);

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::Unsupported {
            instruction,
            cadence_plan,
        } => {
            require(
                CASE,
                instruction.identity() == CpuInstructionIdentity::Jal,
                "inner linking identity",
            )?;
            require(
                CASE,
                instruction.category()
                    == MachineStepUnsupportedInstructionCategory::ControlFlowInDelaySlot,
                "inner control-flow rejection category",
            )?;
            require(
                CASE,
                cadence_plan.count_action() == MachineStepCountAction::DoNotAdvance,
                "inner rejection Count action",
            )?;
        }
        _ => return assertion(CASE, "inner control-flow unsupported outcome"),
    }

    require(CASE, machine.cpu().pc() == pc_before, "inner rejection pc")?;
    require(
        CASE,
        machine.cpu().next_pc() == next_pc_before,
        "inner rejection next_pc",
    )?;
    require(
        CASE,
        machine.cpu_delay_slot_context() == context_before,
        "inner rejection context preserved",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == count_before,
        "inner rejection Count unchanged",
    )?;
    require(
        CASE,
        machine.cpu().gpr(31) == link_before,
        "inner rejection no link write",
    )
}

fn require_committed_identity(
    case: &'static str,
    outcome: MachineRepresentedStepOutcome,
    expected_identity: CpuInstructionIdentity,
) -> Result<(), StepProbeError> {
    require(
        case,
        matches!(
            outcome,
            MachineRepresentedStepOutcome::CpuLocalCommitted {
                identity,
                cadence_plan,
            } if identity == expected_identity
                && cadence_plan.source() == MachineStepCadenceSource::CommittedInstruction
                && cadence_plan.count_action() == MachineStepCountAction::Advance
        ),
        "committed identity and cadence",
    )
}

fn branch_word(opcode: u8, rs: u8, rt: u8, immediate: i16) -> u32 {
    (u32::from(opcode) << 26)
        | (u32::from(rs) << 21)
        | (u32::from(rt) << 16)
        | u32::from(immediate as u16)
}

fn jump_word(opcode: u8, target: u32) -> u32 {
    (u32::from(opcode) << 26) | ((target >> 2) & 0x03ff_ffff)
}

fn register_jump_word(rs: u8, rd: u8, funct: u8) -> u32 {
    (u32::from(rs) << 21) | (u32::from(rd) << 11) | u32::from(funct)
}

fn immediate_word(opcode: u8, rs: u8, rt: u8, immediate: u16) -> u32 {
    (u32::from(opcode) << 26) | (u32::from(rs) << 21) | (u32::from(rt) << 16) | u32::from(immediate)
}

fn special_word(rs: u8, rt: u8, rd: u8, funct: u8) -> u32 {
    (u32::from(rs) << 21) | (u32::from(rt) << 16) | (u32::from(rd) << 11) | u32::from(funct)
}

fn generated_cartridge(
    case: &'static str,
    words: &[(usize, u32)],
) -> Result<Cartridge, StepProbeError> {
    let mut bytes = vec![0; 0x1000];
    write_be_u32(&mut bytes, 0x00, 0x8037_1240);
    write_be_u32(&mut bytes, 0x04, 0x0102_0304);
    write_be_u32(&mut bytes, 0x08, 0x8000_1000);
    write_be_u32(&mut bytes, 0x0c, 0x0506_0708);
    write_be_u32(&mut bytes, 0x10, 0x1112_1314);
    write_be_u32(&mut bytes, 0x14, 0x1516_1718);
    let title = b"FN64 X105 GENERATED";
    bytes[0x20..0x20 + title.len()].copy_from_slice(title);
    bytes[0x3c] = b'G';
    bytes[0x3d] = b'X';
    bytes[0x3e] = 0x45;
    bytes[0x3f] = 1;
    for &(offset, word) in words {
        write_be_u32(&mut bytes, offset, word);
    }

    load_cartridge(bytes).map_err(|source| StepProbeError::Cartridge { case, source })
}

fn generated_cold_x105_machine(
    case: &'static str,
    words: &[(usize, u32)],
) -> Result<(Machine, u32), StepProbeError> {
    let cartridge = generated_cartridge(case, words)?;
    let mut machine = Machine::from_cartridge(cartridge);
    let firmware: Vec<u8> = (0..PIF_BOOT_ROM_SIZE_BYTES)
        .map(|index| 0xa5_u8.wrapping_add((index as u8).wrapping_mul(29)))
        .collect();
    let source_start = PifIpl2Profile::NtscPinned
        .copy_layout()
        .source_start_offset() as usize;
    let generated_sp_imem_word = u32::from_be_bytes(
        firmware[source_start..source_start + 4]
            .try_into()
            .expect("generated PIF-shaped word span is exact"),
    );
    machine
        .install_pif_firmware(firmware)
        .map_err(|source| StepProbeError::PifFirmware { case, source })?;
    machine.install_pif_ipl2_profile(PifIpl2Profile::NtscPinned);
    machine.install_pif_ipl3_family(MachinePifIpl3Family::X105);
    machine.install_pif_ipl2_handoff_reset_kind(MachinePifIpl2HandoffResetKind::Cold);
    machine.install_pif_ipl2_handoff_boot_medium(MachinePifIpl2HandoffBootMedium::Cartridge);
    machine.install_pif_version_bit(MachinePifVersionBit::Zero);
    machine
        .stage_cartridge_bootstrap()
        .map_err(|source| StepProbeError::Bootstrap { case, source })?;

    Ok((machine, generated_sp_imem_word))
}

fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset] = (value >> 24) as u8;
    bytes[offset + 1] = (value >> 16) as u8;
    bytes[offset + 2] = (value >> 8) as u8;
    bytes[offset + 3] = value as u8;
}

fn sign_extend_word(value: u32) -> u64 {
    (value as i32 as i64) as u64
}

fn synthetic_direct_machine_with_instruction(
    case: &'static str,
    instruction: u32,
) -> Result<Machine, StepProbeError> {
    let mut machine = Machine::from_cartridge(Cartridge::default());
    seed_instruction(&mut machine, case, 0, instruction)?;
    machine.stage_cpu_pc(DIRECT_CPU_PC);
    Ok(machine)
}

fn seed_instruction(
    machine: &mut Machine,
    case: &'static str,
    offset: usize,
    instruction: u32,
) -> Result<(), StepProbeError> {
    machine
        .write_rdram_u32_be(offset, instruction)
        .map_err(|source| StepProbeError::Rdram { case, source })
}

fn step(
    machine: &mut Machine,
    case: &'static str,
) -> Result<MachineRepresentedStepOutcome, StepProbeError> {
    machine
        .step()
        .map_err(|source| StepProbeError::Step { case, source })
}

fn assert_normal_single_step_cadence(
    machine: &Machine,
    case: &'static str,
) -> Result<(), StepProbeError> {
    require(case, machine.cpu().pc() == 0x8000_0004, "committed pc")?;
    require(
        case,
        machine.cpu().next_pc() == 0x8000_0008,
        "committed next_pc",
    )?;
    require(
        case,
        machine.cpu().cop0_count() == 1,
        "Count advanced exactly once",
    )
}

fn assert_rejected_state_unchanged(
    machine: &Machine,
    case: &'static str,
    pc_before: u32,
    next_pc_before: u32,
) -> Result<(), StepProbeError> {
    require(case, machine.cpu().pc() == pc_before, "pc restored")?;
    require(
        case,
        machine.cpu().next_pc() == next_pc_before,
        "next_pc restored",
    )?;
    require(
        case,
        machine.cpu().cop0_count() == 0,
        "Count did not advance",
    )?;
    require(
        case,
        machine.cpu().cop0_status() == 0,
        "no exception status",
    )?;
    require(case, machine.cpu().cop0_epc() == 0, "no exception EPC")?;
    require(
        case,
        machine.cpu().cop0_bad_vaddr() == 0,
        "no exception BadVAddr",
    )?;
    require(
        case,
        machine.cpu().cop0_exception_code() == 0,
        "no exception code",
    )
}

fn require(case: &'static str, condition: bool, check: &'static str) -> Result<(), StepProbeError> {
    if condition {
        Ok(())
    } else {
        assertion(case, check)
    }
}

fn assertion<T>(case: &'static str, check: &'static str) -> Result<T, StepProbeError> {
    Err(StepProbeError::Assertion { case, check })
}
