use core::fmt;
use std::process::ExitCode;

use fn64_core::cpu::address::CpuAddress;
use fn64_core::{
    load_cartridge, Cartridge, CartridgeLoadError, CpuAddressErrorKind, CpuInstructionIdentity,
    Machine, MachineBootstrapGprSource, MachineCartridgeBootstrapError,
    MachineCpuInstructionFetchError, MachineLoadWordRejectionReason, MachineLoadWordTarget,
    MachineMtc0Destination, MachineMtc0RejectionReason, MachineOrdinaryControlFlowRejectionReason,
    MachinePifIpl2HandoffBootMedium, MachinePifIpl2HandoffResetKind, MachinePifIpl3Family,
    MachinePifVersionBit, MachineRepresentedStepError, MachineRepresentedStepOutcome,
    MachineRiSelectSource, MachineSpDmemLoadWordProvenance, MachineStepCadenceSource,
    MachineStepControlFlowAction, MachineStepCountAction,
    MachineStepNoEffectExecutedInstructionCategory, MachineStepStoppedInstructionCategory,
    MachineStepUnsupportedInstructionCategory, MachineStoreWordRejectionReason,
    MachineStoreWordTarget, MachineStoreWordUnsupportedTarget, PifFirmwareValidationError,
    PifIpl2Profile, RdramAccessError, SpDmemOffset, PIF_BOOT_ROM_SIZE_BYTES,
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
\ncase: sp-imem-sw-committed ok\
\ncase: sp-imem-sw-big-endian ok\
\ncase: sp-imem-sw-lw-round-trip ok\
\ncase: sp-imem-sw-zero-source ok\
\ncase: sp-imem-sw-rs-rt-alias ok\
\ncase: sp-imem-sw-ades ok\
\ncase: sp-imem-sw-delay-slot-ades ok\
\ncase: sp-imem-sw-unknown-base-rejection ok\
\ncase: sp-imem-sw-unknown-source-rejection ok\
\ncase: sp-imem-sw-unsupported-target-rejection ok\
\ncase: bltz-taken ok\
\ncase: bltz-untaken ok\
\ncase: bltz-zero-source ok\
\ncase: bltz-signed-width-discriminator ok\
\ncase: bltz-positive-offset ok\
\ncase: bltz-negative-offset ok\
\ncase: bltz-delay-slot-committed ok\
\ncase: bltz-delay-slot-exception ok\
\ncase: bltz-in-delay-slot-rejection ok\
\ncase: bltz-unknown-source-rejection ok\
\ncase: generated-x105-post-bltz-frontier ok\
\ncase: mtc0-cause-clear-software-pending ok\
\ncase: mtc0-cause-set-ip0 ok\
\ncase: mtc0-cause-set-ip1 ok\
\ncase: mtc0-cause-preserve-readonly-state ok\
\ncase: mtc0-cause-preserve-timer-pending ok\
\ncase: mtc0-count-write-before-cadence ok\
\ncase: mtc0-count-compare-match-after-cadence ok\
\ncase: mtc0-compare-clear-timer ok\
\ncase: mtc0-compare-relatch-after-cadence ok\
\ncase: mtc0-delay-slot-committed ok\
\ncase: mtc0-unknown-source-rejection ok\
\ncase: mtc0-unsupported-destination-rejection ok\
\ncase: generated-x105-post-mtc0-trio-frontier ok\
\ncase: ri-select-cold-read-committed ok\
\ncase: ri-select-direct-alias ok\
\ncase: ri-select-neighbor-target-miss ok\
\ncase: ri-select-unaligned-adel ok\
\ncase: ri-select-reset-or-entry-source ok\
\ncase: ri-select-independent-machines ok\
\ncase: ri-select-unavailable-rejection ok\
\ncase: ri-select-bne-cold-path ok\
\ncase: ri-select-cold-delay-slot ok\
\ncase: ri-select-stack-save ok\
\ncase: generated-x105-post-ri-select-frontier ok\
\ncase: ri-config-store-committed ok\
\ncase: ri-config-defined-fields ok\
\ncase: ri-config-source-provenance ok\
\ncase: ri-config-direct-alias ok\
\ncase: ri-config-neighbor-target-miss ok\
\ncase: ri-config-reserved-bits-rejection ok\
\ncase: ri-config-unaligned-ades ok\
\ncase: ri-config-delay-slot-committed ok\
\ncase: ri-config-reset-clears ok\
\ncase: ri-config-bootstrap-clears-stale-state ok\
\ncase: ri-config-independent-machines ok\
\ncase: generated-x105-ri-config-wait-loop ok\
\ncase: generated-x105-current-load-frontier ok\
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
    probe_sp_imem_sw_successes()?;
    probe_sp_imem_sw_ades_cases()?;
    probe_sp_imem_sw_rejections()?;
    probe_bltz_conditions_and_targets()?;
    probe_bltz_delay_slot_paths()?;
    probe_bltz_rejections()?;
    probe_mtc0_cause_and_timer()?;
    probe_mtc0_count_and_compare_ordering()?;
    probe_mtc0_delay_slot_and_rejections()?;
    probe_ri_select_routes_and_lifecycle()?;
    probe_ri_config_routes_and_lifecycle()?;
    probe_generated_x105_post_mtc0_trio_frontier()?;
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

fn probe_sp_imem_sw_successes() -> Result<(), StepProbeError> {
    const CASE: &str = "sp-imem-sw-committed";
    let words = [
        (0x40, immediate_word(0x2b, 29, 31, 0xf010)),
        (0x44, immediate_word(0x23, 29, 8, 0xf010)),
    ];
    let (mut machine, _) = generated_cold_x105_machine(CASE, &words)?;

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::StoreWordCommitted {
            effective_address,
            target,
            source_gpr,
            stored_word,
            stored_bytes,
            provenance,
            cadence_plan,
        } => {
            require(
                CASE,
                effective_address == 0xffff_ffff_a400_1000,
                "SP-IMEM Sw effective address",
            )?;
            require(
                CASE,
                target == MachineStoreWordTarget::SpImem { offset: 0 },
                "SP-IMEM Sw target",
            )?;
            require(CASE, source_gpr == 31, "SP-IMEM Sw source GPR")?;
            require(CASE, stored_word == 0xa400_1550, "Sw low-word value")?;
            require(
                CASE,
                stored_bytes == [0xa4, 0x00, 0x15, 0x50],
                "Sw big-endian bytes",
            )?;
            require(
                CASE,
                provenance.instruction_pc() == CpuAddress::new(0xa400_0040),
                "Sw provenance instruction PC",
            )?;
            require(CASE, provenance.source_gpr() == 31, "Sw provenance GPR")?;
            require(
                CASE,
                matches!(
                    provenance.source_lineage(),
                    MachineBootstrapGprSource::PifIpl2RetainedLink { .. }
                ),
                "Sw retained-link source lineage",
            )?;
            require(
                CASE,
                cadence_plan.source() == MachineStepCadenceSource::CommittedInstruction,
                "Sw committed cadence",
            )?;
        }
        _ => return assertion(CASE, "SP-IMEM Sw committed outcome"),
    }
    require(CASE, machine.cpu().pc() == 0xa400_0044, "Sw committed pc")?;
    require(
        CASE,
        machine.cpu().next_pc() == 0xa400_0048,
        "Sw committed next_pc",
    )?;
    require(CASE, machine.cpu().cop0_count() == 1, "Sw committed Count")?;

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::LoadWordCommitted {
            target: MachineLoadWordTarget::SpImem { offset: 0 },
            destination_gpr: 8,
            loaded_word: 0xa400_1550,
            result_value: 0xffff_ffff_a400_1550,
            ..
        } => {}
        _ => return assertion(CASE, "Sw followed by Lw round trip"),
    }

    let zero_words = [
        (0x40, immediate_word(0x2b, 29, 0, 0xf010)),
        (0x44, immediate_word(0x23, 29, 8, 0xf010)),
    ];
    let (mut zero, _) = generated_cold_x105_machine(CASE, &zero_words)?;
    require(
        CASE,
        matches!(
            step(&mut zero, CASE)?,
            MachineRepresentedStepOutcome::StoreWordCommitted {
                source_gpr: 0,
                stored_word: 0,
                stored_bytes: [0, 0, 0, 0],
                ..
            }
        ),
        "r0 writes one known zero word",
    )?;
    require(
        CASE,
        matches!(
            step(&mut zero, CASE)?,
            MachineRepresentedStepOutcome::LoadWordCommitted {
                loaded_word: 0,
                result_value: 0,
                ..
            }
        ),
        "r0 store round trip",
    )?;

    let alias_words = [(0x40, immediate_word(0x2b, 29, 29, 0xf010))];
    let (mut alias, _) = generated_cold_x105_machine(CASE, &alias_words)?;
    require(
        CASE,
        matches!(
            step(&mut alias, CASE)?,
            MachineRepresentedStepOutcome::StoreWordCommitted {
                effective_address: 0xffff_ffff_a400_1000,
                source_gpr: 29,
                stored_word: 0xa400_1ff0,
                stored_bytes: [0xa4, 0x00, 0x1f, 0xf0],
                ..
            }
        ),
        "rs equals rt uses old value for address and data",
    )
}

fn probe_sp_imem_sw_ades_cases() -> Result<(), StepProbeError> {
    const CASE: &str = "sp-imem-sw-ades";
    let words = [(0x40, immediate_word(0x2b, 29, 7, 0xf011))];
    let (mut machine, _) = generated_cold_x105_machine(CASE, &words)?;

    match step(&mut machine, CASE)? {
        MachineRepresentedStepOutcome::DataAddressError {
            identity,
            effective_address,
            address_error,
            cadence_plan,
        } => {
            require(
                CASE,
                identity == CpuInstructionIdentity::Sw,
                "AdES Sw identity",
            )?;
            require(
                CASE,
                effective_address == 0xffff_ffff_a400_1001,
                "AdES effective address",
            )?;
            require(
                CASE,
                address_error.exception_kind() == CpuAddressErrorKind::AddressErrorStore,
                "AdES exception kind",
            )?;
            require(CASE, address_error.cause_exception_code() == 5, "AdES code")?;
            require(
                CASE,
                cadence_plan.source() == MachineStepCadenceSource::EnteredException,
                "AdES exception cadence",
            )?;
        }
        _ => return assertion(CASE, "sequential Sw AdES outcome"),
    }
    require(
        CASE,
        machine.cpu().cop0_bad_vaddr() == 0xa400_1001,
        "AdES BadVAddr",
    )?;
    require(CASE, machine.cpu().cop0_epc() == 0xa400_0040, "AdES EPC")?;
    require(
        CASE,
        !machine.cpu().cop0_exception_branch_delay(),
        "sequential AdES BD clear",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 0,
        "AdES Count unchanged",
    )?;

    let delay_words = [
        (0x40, branch_word(0x04, 0, 0, 1)),
        (0x44, immediate_word(0x2b, 29, 7, 0xf011)),
        (0x48, special_word(0, 0, 0, 0)),
    ];
    let (mut delay, _) = generated_cold_x105_machine(CASE, &delay_words)?;
    require_committed_identity(CASE, step(&mut delay, CASE)?, CpuInstructionIdentity::Beq)?;
    require(
        CASE,
        matches!(
            step(&mut delay, CASE)?,
            MachineRepresentedStepOutcome::DataAddressError {
                identity: CpuInstructionIdentity::Sw,
                effective_address: 0xffff_ffff_a400_1001,
                ..
            }
        ),
        "delay-slot Sw AdES outcome",
    )?;
    require(CASE, delay.cpu().cop0_epc() == 0xa400_0040, "slot AdES EPC")?;
    require(
        CASE,
        delay.cpu().cop0_exception_branch_delay(),
        "slot AdES BD",
    )?;
    require(CASE, delay.cpu().cop0_count() == 1, "slot AdES Count")?;
    require(
        CASE,
        delay.cpu_delay_slot_context().is_none(),
        "slot AdES context cleared",
    )?;
    require(
        CASE,
        delay.cpu().pc() == GENERAL_EXCEPTION_VECTOR_PC,
        "slot AdES vector",
    )
}

fn probe_sp_imem_sw_rejections() -> Result<(), StepProbeError> {
    const CASE: &str = "sp-imem-sw-unknown-base-rejection";
    let mut unknown_base =
        synthetic_direct_machine_with_instruction(CASE, immediate_word(0x2b, 1, 0, 0))?;
    let pc_before = unknown_base.cpu().pc();
    let next_pc_before = unknown_base.cpu().next_pc();
    match unknown_base.step() {
        Err(MachineRepresentedStepError::StoreWordRejected(rejection)) => require(
            CASE,
            matches!(
                rejection.reason(),
                MachineStoreWordRejectionReason::BaseSourceUnavailable {
                    register_index: 1,
                    ..
                }
            ),
            "unknown base rejection",
        )?,
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "unknown base Sw rejection"),
    }
    assert_rejected_state_unchanged(&unknown_base, CASE, pc_before, next_pc_before)?;

    let unknown_source_words = [(0x40, immediate_word(0x2b, 29, 7, 0xf010))];
    let (mut unknown_source, _) = generated_cold_x105_machine(CASE, &unknown_source_words)?;
    match unknown_source.step() {
        Err(MachineRepresentedStepError::StoreWordRejected(rejection)) => require(
            CASE,
            matches!(
                rejection.reason(),
                MachineStoreWordRejectionReason::ValueSourceUnavailable {
                    register_index: 7,
                    ..
                }
            ),
            "unknown source rejection",
        )?,
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "unknown source Sw rejection"),
    }
    require(
        CASE,
        unknown_source.cpu().cop0_count() == 0,
        "unknown source Count unchanged",
    )?;

    let unsupported_words = [
        (0x40, immediate_word(0x0f, 0, 1, 0x8000)),
        (0x44, immediate_word(0x0d, 1, 1, 0x0100)),
        (0x48, immediate_word(0x2b, 1, 0, 0)),
    ];
    let (mut unsupported, _) = generated_cold_x105_machine(CASE, &unsupported_words)?;
    require_committed_identity(
        CASE,
        step(&mut unsupported, CASE)?,
        CpuInstructionIdentity::Lui,
    )?;
    require_committed_identity(
        CASE,
        step(&mut unsupported, CASE)?,
        CpuInstructionIdentity::Ori,
    )?;
    let pc_before = unsupported.cpu().pc();
    let next_pc_before = unsupported.cpu().next_pc();
    let count_before = unsupported.cpu().cop0_count();
    match unsupported.step() {
        Err(MachineRepresentedStepError::StoreWordRejected(rejection)) => require(
            CASE,
            matches!(
                rejection.reason(),
                MachineStoreWordRejectionReason::UnsupportedTarget {
                    target: MachineStoreWordUnsupportedTarget::DirectRdram { .. }
                }
            ),
            "unsupported RDRAM target rejection",
        )?,
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "unsupported target Sw rejection"),
    }
    require(
        CASE,
        unsupported.cpu().pc() == pc_before,
        "unsupported target pc",
    )?;
    require(
        CASE,
        unsupported.cpu().next_pc() == next_pc_before,
        "unsupported target next_pc",
    )?;
    require(
        CASE,
        unsupported.cpu().cop0_count() == count_before,
        "unsupported target Count",
    )
}

fn probe_bltz_conditions_and_targets() -> Result<(), StepProbeError> {
    const CASE: &str = "bltz-taken";
    let taken_words = [
        (0x40, branch_word(0x01, 31, 0, 2)),
        (0x44, special_word(0, 0, 0, 0)),
    ];
    let (mut taken, _) = generated_cold_x105_machine(CASE, &taken_words)?;
    let retained_ra = taken.cpu().gpr(31);
    let retained_source = taken.cartridge_bootstrap_state().unwrap().gpr_source(31);
    require_committed_identity(
        CASE,
        step(&mut taken, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(CASE, taken.cpu().pc() == 0xa400_0044, "taken slot pc")?;
    require(
        CASE,
        taken.cpu().next_pc() == 0xa400_004c,
        "positive-offset target",
    )?;
    require(CASE, taken.cpu().gpr(31) == retained_ra, "BLTZ no link")?;
    require(
        CASE,
        taken.cartridge_bootstrap_state().unwrap().gpr_source(31) == retained_source,
        "BLTZ source lineage preserved",
    )?;

    let zero_words = [
        (0x40, branch_word(0x01, 0, 0, 2)),
        (0x44, special_word(0, 0, 0, 0)),
    ];
    let (mut zero, _) = generated_cold_x105_machine(CASE, &zero_words)?;
    require_committed_identity(
        CASE,
        step(&mut zero, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(
        CASE,
        zero.cpu().next_pc() == 0xa400_0048,
        "zero source is untaken",
    )?;
    require(
        CASE,
        zero.cpu().gpr(0) == Some(0),
        "architectural r0 preserved",
    )?;

    let positive_words = [
        (0x40, immediate_word(0x0d, 0, 4, 1)),
        (0x44, special_shift_word(0, 4, 4, 31, 0x38)),
        (0x48, branch_word(0x01, 4, 0, 2)),
        (0x4c, special_word(0, 0, 0, 0)),
    ];
    let (mut positive, _) = generated_cold_x105_machine(CASE, &positive_words)?;
    require_committed_identity(
        CASE,
        step(&mut positive, CASE)?,
        CpuInstructionIdentity::Ori,
    )?;
    require_committed_identity(
        CASE,
        step(&mut positive, CASE)?,
        CpuInstructionIdentity::SpecialDsll,
    )?;
    require(
        CASE,
        positive.cpu().gpr(4) == Some(0x0000_0000_8000_0000),
        "positive full-width discriminator value",
    )?;
    require_committed_identity(
        CASE,
        step(&mut positive, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(
        CASE,
        positive.cpu().next_pc() == 0xa400_0050,
        "positive full-width discriminator untaken",
    )?;

    let negative_words = [
        (0x40, immediate_word(0x0f, 0, 4, 0xffff)),
        (0x44, special_shift_word(0, 4, 4, 16, 0x38)),
        (0x48, branch_word(0x01, 4, 0, 2)),
        (0x4c, special_word(0, 0, 0, 0)),
    ];
    let (mut negative, _) = generated_cold_x105_machine(CASE, &negative_words)?;
    require_committed_identity(
        CASE,
        step(&mut negative, CASE)?,
        CpuInstructionIdentity::Lui,
    )?;
    require_committed_identity(
        CASE,
        step(&mut negative, CASE)?,
        CpuInstructionIdentity::SpecialDsll,
    )?;
    require(
        CASE,
        negative.cpu().gpr(4) == Some(0xffff_ffff_0000_0000),
        "negative full-width discriminator value",
    )?;
    require_committed_identity(
        CASE,
        step(&mut negative, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(
        CASE,
        negative.cpu().next_pc() == 0xa400_0054,
        "negative full-width discriminator taken",
    )?;

    let negative_offset_words = [
        (0x40, branch_word(0x01, 31, 0, -2)),
        (0x44, special_word(0, 0, 0, 0)),
    ];
    let (mut negative_offset, _) = generated_cold_x105_machine(CASE, &negative_offset_words)?;
    require_committed_identity(
        CASE,
        step(&mut negative_offset, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(
        CASE,
        negative_offset.cpu().next_pc() == 0xa400_003c,
        "negative-offset target",
    )
}

fn probe_bltz_delay_slot_paths() -> Result<(), StepProbeError> {
    const CASE: &str = "bltz-delay-slot-committed";
    let success_words = [
        (0x40, branch_word(0x01, 31, 0, 1)),
        (0x44, immediate_word(0x2b, 29, 0, 0xf010)),
        (0x48, special_word(0, 0, 0, 0)),
    ];
    let (mut success, _) = generated_cold_x105_machine(CASE, &success_words)?;
    require_committed_identity(
        CASE,
        step(&mut success, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(
        CASE,
        matches!(
            step(&mut success, CASE)?,
            MachineRepresentedStepOutcome::StoreWordCommitted {
                target: MachineStoreWordTarget::SpImem { offset: 0 },
                source_gpr: 0,
                stored_word: 0,
                ..
            }
        ),
        "BLTZ delay-slot Sw committed",
    )?;
    require(CASE, success.cpu().pc() == 0xa400_0048, "slot target pc")?;
    require(
        CASE,
        success.cpu().cop0_count() == 2,
        "branch plus slot Count",
    )?;
    require(
        CASE,
        success.cpu_delay_slot_context().is_none(),
        "slot context cleared",
    )?;

    let fault_words = [
        (0x40, branch_word(0x01, 31, 0, 1)),
        (0x44, immediate_word(0x2b, 29, 0, 0xf011)),
        (0x48, special_word(0, 0, 0, 0)),
    ];
    let (mut fault, _) = generated_cold_x105_machine(CASE, &fault_words)?;
    require_committed_identity(
        CASE,
        step(&mut fault, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(
        CASE,
        matches!(
            step(&mut fault, CASE)?,
            MachineRepresentedStepOutcome::DataAddressError {
                identity: CpuInstructionIdentity::Sw,
                address_error,
                ..
            } if address_error.exception_kind() == CpuAddressErrorKind::AddressErrorStore
                && address_error.bad_vaddr() == CpuAddress::new(0xa400_1001)
        ),
        "BLTZ slot AdES",
    )?;
    require(
        CASE,
        fault.cpu().cop0_epc() == 0xa400_0040,
        "slot owner EPC",
    )?;
    require(CASE, fault.cpu().cop0_exception_branch_delay(), "slot BD")?;
    require(
        CASE,
        fault.cpu().cop0_count() == 1,
        "faulting slot Count zero",
    )?;
    require(
        CASE,
        fault.cpu().pc() == GENERAL_EXCEPTION_VECTOR_PC,
        "slot exception vector",
    )?;
    require(
        CASE,
        fault.cpu_delay_slot_context().is_none(),
        "slot exception context cleared",
    )
}

fn probe_bltz_rejections() -> Result<(), StepProbeError> {
    const CASE: &str = "bltz-in-delay-slot-rejection";
    let nested_words = [
        (0x40, branch_word(0x04, 0, 0, 1)),
        (0x44, branch_word(0x01, 31, 0, 1)),
        (0x48, special_word(0, 0, 0, 0)),
    ];
    let (mut nested, _) = generated_cold_x105_machine(CASE, &nested_words)?;
    require_committed_identity(CASE, step(&mut nested, CASE)?, CpuInstructionIdentity::Beq)?;
    let pc_before = nested.cpu().pc();
    let next_pc_before = nested.cpu().next_pc();
    let count_before = nested.cpu().cop0_count();
    let context_before = nested.cpu_delay_slot_context();
    require(
        CASE,
        matches!(
            step(&mut nested, CASE)?,
            MachineRepresentedStepOutcome::Unsupported {
                instruction,
                cadence_plan,
            } if instruction.identity() == CpuInstructionIdentity::RegimmBltz
                && instruction.category()
                    == MachineStepUnsupportedInstructionCategory::ControlFlowInDelaySlot
                && cadence_plan.count_action() == MachineStepCountAction::DoNotAdvance
        ),
        "BLTZ in delay slot rejected",
    )?;
    require(CASE, nested.cpu().pc() == pc_before, "nested rejection pc")?;
    require(
        CASE,
        nested.cpu().next_pc() == next_pc_before,
        "nested rejection next_pc",
    )?;
    require(
        CASE,
        nested.cpu().cop0_count() == count_before,
        "nested rejection Count",
    )?;
    require(
        CASE,
        nested.cpu_delay_slot_context() == context_before,
        "nested rejection context",
    )?;

    let unknown_words = [(0x40, branch_word(0x01, 7, 0, 1))];
    let (mut unknown, _) = generated_cold_x105_machine(CASE, &unknown_words)?;
    let pc_before = unknown.cpu().pc();
    let next_pc_before = unknown.cpu().next_pc();
    match unknown.step() {
        Err(MachineRepresentedStepError::OrdinaryControlFlowRejected(rejection)) => require(
            CASE,
            rejection.reason()
                == MachineOrdinaryControlFlowRejectionReason::BootstrapSourceUnavailable {
                    register_index: 7,
                    source: MachineBootstrapGprSource::UnknownPifProduced,
                },
            "unknown BLTZ source rejection",
        )?,
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "unknown BLTZ source rejected"),
    }
    require(
        CASE,
        unknown.cpu().pc() == pc_before,
        "unknown rejection pc",
    )?;
    require(
        CASE,
        unknown.cpu().next_pc() == next_pc_before,
        "unknown rejection next_pc",
    )?;
    require(
        CASE,
        unknown.cpu().cop0_count() == 0,
        "unknown rejection Count",
    )
}

fn probe_mtc0_cause_and_timer() -> Result<(), StepProbeError> {
    const CLEAR_CASE: &str = "mtc0-cause-clear-software-pending";
    let words = [
        (0x40, immediate_word(0x0d, 0, 8, 0x0100)),
        (0x44, cop0_move_word(4, 8, 13)),
        (0x48, immediate_word(0x0d, 0, 8, 0)),
        (0x4c, cop0_move_word(4, 8, 13)),
    ];
    let (mut clear, _) = generated_cold_x105_machine(CLEAR_CASE, &words)?;
    require(
        CLEAR_CASE,
        !clear.cpu().cop0_software_interrupt_pending_known(),
        "software pending initially unknown",
    )?;
    require_committed_identity(
        CLEAR_CASE,
        step(&mut clear, CLEAR_CASE)?,
        CpuInstructionIdentity::Ori,
    )?;
    require_mtc0_commit(
        CLEAR_CASE,
        step(&mut clear, CLEAR_CASE)?,
        MachineMtc0Destination::CauseSoftwareInterruptPending,
        0x0100,
    )?;
    require(
        CLEAR_CASE,
        clear.cpu().cop0_software_interrupt_pending() == 0x0100,
        "IP0 set",
    )?;
    require_committed_identity(
        CLEAR_CASE,
        step(&mut clear, CLEAR_CASE)?,
        CpuInstructionIdentity::Ori,
    )?;
    require_mtc0_commit(
        CLEAR_CASE,
        step(&mut clear, CLEAR_CASE)?,
        MachineMtc0Destination::CauseSoftwareInterruptPending,
        0,
    )?;
    require(
        CLEAR_CASE,
        clear.cpu().cop0_software_interrupt_pending() == 0
            && clear.cpu().cop0_software_interrupt_pending_known(),
        "known cleared software pending",
    )?;

    const IP1_CASE: &str = "mtc0-cause-set-ip1";
    let words = [
        (0x40, immediate_word(0x0d, 0, 8, 0x0200)),
        (0x44, cop0_move_word(4, 8, 13)),
    ];
    let (mut ip1, _) = generated_cold_x105_machine(IP1_CASE, &words)?;
    step(&mut ip1, IP1_CASE)?;
    require_mtc0_commit(
        IP1_CASE,
        step(&mut ip1, IP1_CASE)?,
        MachineMtc0Destination::CauseSoftwareInterruptPending,
        0x0200,
    )?;
    require(
        IP1_CASE,
        ip1.cpu().cop0_software_interrupt_pending() == 0x0200,
        "IP1 set without IP0",
    )?;

    const READONLY_CASE: &str = "mtc0-cause-preserve-readonly-state";
    let words = [(0x40, immediate_word(0x23, 11, 8, 0x0041))];
    let (mut readonly, _) = generated_cold_x105_machine(READONLY_CASE, &words)?;
    readonly
        .write_rdram_u32_be(0x180, cop0_move_word(4, 0, 13))
        .map_err(|source| StepProbeError::Rdram {
            case: READONLY_CASE,
            source,
        })?;
    require(
        READONLY_CASE,
        matches!(
            step(&mut readonly, READONLY_CASE)?,
            MachineRepresentedStepOutcome::DataAddressError {
                identity: CpuInstructionIdentity::Lw,
                ..
            }
        ),
        "generated AdEL entered",
    )?;
    let readonly_before = (
        readonly.cpu().cop0_status(),
        readonly.cpu().cop0_epc(),
        readonly.cpu().cop0_bad_vaddr(),
        readonly.cpu().cop0_exception_code(),
        readonly.cpu().cop0_exception_branch_delay(),
    );
    require_mtc0_commit(
        READONLY_CASE,
        step(&mut readonly, READONLY_CASE)?,
        MachineMtc0Destination::CauseSoftwareInterruptPending,
        0,
    )?;
    require(
        READONLY_CASE,
        readonly_before
            == (
                readonly.cpu().cop0_status(),
                readonly.cpu().cop0_epc(),
                readonly.cpu().cop0_bad_vaddr(),
                readonly.cpu().cop0_exception_code(),
                readonly.cpu().cop0_exception_branch_delay(),
            ),
        "Cause read-only fields preserved",
    )?;

    const TIMER_CASE: &str = "mtc0-cause-preserve-timer-pending";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xffff)),
        (0x44, immediate_word(0x0d, 8, 8, 0xffff)),
        (0x48, cop0_move_word(4, 8, 9)),
        (0x4c, cop0_move_word(4, 0, 13)),
    ];
    let (mut timer, _) = generated_cold_x105_machine(TIMER_CASE, &words)?;
    step(&mut timer, TIMER_CASE)?;
    step(&mut timer, TIMER_CASE)?;
    require_mtc0_commit(
        TIMER_CASE,
        step(&mut timer, TIMER_CASE)?,
        MachineMtc0Destination::Count,
        0xffff_ffff,
    )?;
    require(
        TIMER_CASE,
        timer.cpu().cop0_count() == 0 && timer.cpu().cop0_timer_interrupt_pending(),
        "post-write equality latched timer",
    )?;
    require_mtc0_commit(
        TIMER_CASE,
        step(&mut timer, TIMER_CASE)?,
        MachineMtc0Destination::CauseSoftwareInterruptPending,
        0,
    )?;
    require(
        TIMER_CASE,
        timer.cpu().cop0_timer_interrupt_pending(),
        "Cause preserved timer pending",
    )
}

fn probe_mtc0_count_and_compare_ordering() -> Result<(), StepProbeError> {
    const COUNT_CASE: &str = "mtc0-count-write-before-cadence";
    let (mut zero, _) =
        generated_cold_x105_machine(COUNT_CASE, &[(0x40, cop0_move_word(4, 0, 9))])?;
    require_mtc0_commit(
        COUNT_CASE,
        step(&mut zero, COUNT_CASE)?,
        MachineMtc0Destination::Count,
        0,
    )?;
    require(
        COUNT_CASE,
        zero.cpu().cop0_count() == 1,
        "Count zero write precedes one cadence increment",
    )?;

    const MATCH_CASE: &str = "mtc0-count-compare-match-after-cadence";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xffff)),
        (0x44, immediate_word(0x0d, 8, 8, 0xffff)),
        (0x48, cop0_move_word(4, 8, 9)),
    ];
    let (mut matching, _) = generated_cold_x105_machine(MATCH_CASE, &words)?;
    step(&mut matching, MATCH_CASE)?;
    step(&mut matching, MATCH_CASE)?;
    require_mtc0_commit(
        MATCH_CASE,
        step(&mut matching, MATCH_CASE)?,
        MachineMtc0Destination::Count,
        0xffff_ffff,
    )?;
    require(
        MATCH_CASE,
        matching.cpu().cop0_count() == 0 && matching.cpu().cop0_timer_interrupt_pending(),
        "Count cadence equality latches timer",
    )?;

    const CLEAR_CASE: &str = "mtc0-compare-clear-timer";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xffff)),
        (0x44, immediate_word(0x0d, 8, 8, 0xffff)),
        (0x48, cop0_move_word(4, 8, 9)),
        (0x4c, cop0_move_word(4, 22, 11)),
    ];
    let (mut clear, _) = generated_cold_x105_machine(CLEAR_CASE, &words)?;
    step(&mut clear, CLEAR_CASE)?;
    step(&mut clear, CLEAR_CASE)?;
    step(&mut clear, CLEAR_CASE)?;
    require(
        CLEAR_CASE,
        clear.cpu().cop0_timer_interrupt_pending(),
        "timer preset",
    )?;
    require_mtc0_commit(
        CLEAR_CASE,
        step(&mut clear, CLEAR_CASE)?,
        MachineMtc0Destination::Compare,
        0x91,
    )?;
    require(
        CLEAR_CASE,
        clear.cpu().cop0_compare() == 0x91
            && clear.cpu().cop0_count() == 1
            && !clear.cpu().cop0_timer_interrupt_pending(),
        "Compare clears before nonmatching cadence",
    )?;

    const RELATCH_CASE: &str = "mtc0-compare-relatch-after-cadence";
    let words = [
        (0x40, immediate_word(0x0d, 0, 8, 0x008f)),
        (0x44, immediate_word(0x0d, 0, 9, 0x0091)),
        (0x48, cop0_move_word(4, 8, 9)),
        (0x4c, cop0_move_word(4, 9, 11)),
    ];
    let (mut relatch, _) = generated_cold_x105_machine(RELATCH_CASE, &words)?;
    step(&mut relatch, RELATCH_CASE)?;
    step(&mut relatch, RELATCH_CASE)?;
    step(&mut relatch, RELATCH_CASE)?;
    require(
        RELATCH_CASE,
        relatch.cpu().cop0_count() == 0x90,
        "Count staged one below new Compare",
    )?;
    require_mtc0_commit(
        RELATCH_CASE,
        step(&mut relatch, RELATCH_CASE)?,
        MachineMtc0Destination::Compare,
        0x91,
    )?;
    require(
        RELATCH_CASE,
        relatch.cpu().cop0_count() == 0x91
            && relatch.cpu().cop0_compare() == 0x91
            && relatch.cpu().cop0_timer_interrupt_pending(),
        "Compare clear precedes post-cadence relatch",
    )
}

fn probe_mtc0_delay_slot_and_rejections() -> Result<(), StepProbeError> {
    const DELAY_CASE: &str = "mtc0-delay-slot-committed";
    let words = [
        (0x40, branch_word(0x04, 0, 0, 1)),
        (0x44, cop0_move_word(4, 0, 13)),
        (0x48, special_word(0, 0, 0, 0)),
    ];
    let (mut delay, _) = generated_cold_x105_machine(DELAY_CASE, &words)?;
    require_committed_identity(
        DELAY_CASE,
        step(&mut delay, DELAY_CASE)?,
        CpuInstructionIdentity::Beq,
    )?;
    require_mtc0_commit(
        DELAY_CASE,
        step(&mut delay, DELAY_CASE)?,
        MachineMtc0Destination::CauseSoftwareInterruptPending,
        0,
    )?;
    require(
        DELAY_CASE,
        delay.cpu().pc() == 0xa400_0048
            && delay.cpu().next_pc() == 0xa400_004c
            && delay.cpu_delay_slot_context().is_none()
            && delay.cpu().cop0_count() == 2,
        "ordinary slot destination and cadence committed",
    )?;

    const UNKNOWN_CASE: &str = "mtc0-unknown-source-rejection";
    let (mut unknown, _) =
        generated_cold_x105_machine(UNKNOWN_CASE, &[(0x40, cop0_move_word(4, 8, 13))])?;
    let before = (
        unknown.cpu().pc(),
        unknown.cpu().next_pc(),
        unknown.cpu().cop0_count(),
    );
    match unknown.step() {
        Err(MachineRepresentedStepError::Mtc0Rejected(rejection)) => require(
            UNKNOWN_CASE,
            rejection.reason()
                == MachineMtc0RejectionReason::SourceUnavailable {
                    register_index: 8,
                    source: MachineBootstrapGprSource::UnknownPifProduced,
                },
            "unknown source rejection",
        )?,
        Err(source) => {
            return Err(StepProbeError::Step {
                case: UNKNOWN_CASE,
                source,
            })
        }
        Ok(_) => return assertion(UNKNOWN_CASE, "unknown source rejected"),
    }
    require(
        UNKNOWN_CASE,
        before
            == (
                unknown.cpu().pc(),
                unknown.cpu().next_pc(),
                unknown.cpu().cop0_count(),
            ),
        "unknown source complete rollback",
    )?;

    const DEST_CASE: &str = "mtc0-unsupported-destination-rejection";
    let (mut destination, _) =
        generated_cold_x105_machine(DEST_CASE, &[(0x40, cop0_move_word(4, 0, 12))])?;
    let before = (
        destination.cpu().pc(),
        destination.cpu().next_pc(),
        destination.cpu().cop0_count(),
    );
    match destination.step() {
        Err(MachineRepresentedStepError::Mtc0Rejected(rejection)) => require(
            DEST_CASE,
            rejection.reason()
                == MachineMtc0RejectionReason::UnsupportedDestination { register_index: 12 },
            "unsupported destination rejection",
        )?,
        Err(source) => {
            return Err(StepProbeError::Step {
                case: DEST_CASE,
                source,
            })
        }
        Ok(_) => return assertion(DEST_CASE, "unsupported destination rejected"),
    }
    require(
        DEST_CASE,
        before
            == (
                destination.cpu().pc(),
                destination.cpu().next_pc(),
                destination.cpu().cop0_count(),
            ),
        "unsupported destination complete rollback",
    )
}

fn probe_ri_select_routes_and_lifecycle() -> Result<(), StepProbeError> {
    const READ_CASE: &str = "ri-select-cold-read-committed";
    for ri_base in [0x8470_u16, 0xa470_u16] {
        let words = [
            (0x40, immediate_word(0x0f, 0, 8, ri_base)),
            (0x44, immediate_word(0x23, 8, 9, 0x000c)),
        ];
        let (mut machine, _) = generated_cold_x105_machine(READ_CASE, &words)?;
        let state = machine.ri_select_state().ok_or(StepProbeError::Assertion {
            case: READ_CASE,
            check: "cold-entry RI_SELECT available",
        })?;
        require(
            READ_CASE,
            state.value() == 0 && state.source() == MachineRiSelectSource::ColdX105Entry,
            "cold-entry value and source",
        )?;
        require_committed_identity(
            READ_CASE,
            step(&mut machine, READ_CASE)?,
            CpuInstructionIdentity::Lui,
        )?;
        require(
            READ_CASE,
            matches!(
                step(&mut machine, READ_CASE)?,
                MachineRepresentedStepOutcome::LoadWordCommitted {
                    target: MachineLoadWordTarget::RiSelect {
                        source: MachineRiSelectSource::ColdX105Entry,
                    },
                    destination_gpr: 9,
                    loaded_word: 0,
                    result_value: 0,
                    cadence_plan,
                    ..
                } if cadence_plan.advances_count()
            ),
            "exact RI_SELECT load target",
        )?;
        require(
            READ_CASE,
            machine.cpu().gpr(9) == Some(0)
                && machine.cpu().cop0_count() == 2
                && machine.ri_select_state() == Some(state),
            "load result, cadence, and no RI side effect",
        )?;
    }

    const MISS_CASE: &str = "ri-select-neighbor-target-miss";
    let miss_words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
        (0x44, immediate_word(0x23, 8, 9, 0x0004)),
    ];
    let (mut miss, _) = generated_cold_x105_machine(MISS_CASE, &miss_words)?;
    step(&mut miss, MISS_CASE)?;
    let miss_before = (
        miss.cpu().pc(),
        miss.cpu().next_pc(),
        miss.cpu().cop0_count(),
        miss.cpu().gpr(9),
        miss.ri_select_state(),
    );
    match miss.step() {
        Err(MachineRepresentedStepError::LoadWordRejected(rejection)) => require(
            MISS_CASE,
            rejection.cpu_address() == CpuAddress::new(0xa470_0004)
                && rejection.reason() == MachineLoadWordRejectionReason::DirectTargetMiss,
            "RI_CONFIG remains a direct target miss",
        )?,
        Err(source) => {
            return Err(StepProbeError::Step {
                case: MISS_CASE,
                source,
            })
        }
        Ok(_) => return assertion(MISS_CASE, "neighbor target rejection"),
    }
    require(
        MISS_CASE,
        miss_before
            == (
                miss.cpu().pc(),
                miss.cpu().next_pc(),
                miss.cpu().cop0_count(),
                miss.cpu().gpr(9),
                miss.ri_select_state(),
            ),
        "neighbor rejection rollback",
    )?;

    const ADEL_CASE: &str = "ri-select-unaligned-adel";
    let adel_words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
        (0x44, immediate_word(0x23, 8, 9, 0x000d)),
    ];
    let (mut adel, _) = generated_cold_x105_machine(ADEL_CASE, &adel_words)?;
    step(&mut adel, ADEL_CASE)?;
    require(
        ADEL_CASE,
        matches!(
            step(&mut adel, ADEL_CASE)?,
            MachineRepresentedStepOutcome::DataAddressError {
                identity: CpuInstructionIdentity::Lw,
                effective_address: 0xffff_ffff_a470_000d,
                address_error,
                cadence_plan,
            } if address_error.exception_kind() == CpuAddressErrorKind::AddressErrorLoad
                && address_error.bad_vaddr() == CpuAddress::new(0xa470_000d)
                && !cadence_plan.advances_count()
        ),
        "unaligned RI_SELECT uses existing AdEL",
    )?;

    const UNAVAILABLE_CASE: &str = "ri-select-unavailable-rejection";
    let unavailable_cartridge = generated_cartridge(
        UNAVAILABLE_CASE,
        &[
            (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
            (0x44, immediate_word(0x23, 8, 9, 0x000c)),
        ],
    )?;
    let mut unavailable = Machine::from_cartridge(unavailable_cartridge);
    unavailable
        .stage_cartridge_bootstrap()
        .map_err(|source| StepProbeError::Bootstrap {
            case: UNAVAILABLE_CASE,
            source,
        })?;
    require(
        UNAVAILABLE_CASE,
        unavailable.ri_select_state().is_none(),
        "ordinary bootstrap leaves RI_SELECT unavailable",
    )?;
    step(&mut unavailable, UNAVAILABLE_CASE)?;
    let unavailable_before = (
        unavailable.cpu().pc(),
        unavailable.cpu().next_pc(),
        unavailable.cpu().cop0_count(),
        unavailable.cpu().gpr(9),
    );
    match unavailable.step() {
        Err(MachineRepresentedStepError::LoadWordRejected(rejection)) => require(
            UNAVAILABLE_CASE,
            rejection.reason() == MachineLoadWordRejectionReason::RiSelectUnavailable,
            "unavailable RI_SELECT rejection",
        )?,
        Err(source) => {
            return Err(StepProbeError::Step {
                case: UNAVAILABLE_CASE,
                source,
            })
        }
        Ok(_) => return assertion(UNAVAILABLE_CASE, "unavailable state rejection"),
    }
    require(
        UNAVAILABLE_CASE,
        unavailable_before
            == (
                unavailable.cpu().pc(),
                unavailable.cpu().next_pc(),
                unavailable.cpu().cop0_count(),
                unavailable.cpu().gpr(9),
            ),
        "unavailable rejection rollback",
    )?;

    const LIFECYCLE_CASE: &str = "ri-select-reset-or-entry-source";
    let (mut first, _) = generated_cold_x105_machine(LIFECYCLE_CASE, &[])?;
    let (second, _) = generated_cold_x105_machine(LIFECYCLE_CASE, &[])?;
    require(
        LIFECYCLE_CASE,
        first.ri_select_state().is_some() && second.ri_select_state().is_some(),
        "independent cold-entry states",
    )?;
    first.reset();
    require(
        LIFECYCLE_CASE,
        first.ri_select_state().is_none() && second.ri_select_state().is_some(),
        "reset clears one Machine only",
    )
}

fn probe_ri_config_routes_and_lifecycle() -> Result<(), StepProbeError> {
    const CASE: &str = "ri-config-store-committed";
    for (word, expected_input, expected_enable) in [
        (0x00_u16, 0x00_u8, false),
        (0x3f, 0x3f, false),
        (0x40, 0x00, true),
        (0x7f, 0x3f, true),
    ] {
        let words = [
            (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
            (0x44, immediate_word(0x0d, 0, 9, word)),
            (0x48, immediate_word(0x2b, 8, 9, 4)),
        ];
        let (mut machine, _) = generated_cold_x105_machine(CASE, &words)?;
        step(&mut machine, CASE)?;
        step(&mut machine, CASE)?;
        let source_lineage = machine
            .cartridge_bootstrap_state()
            .and_then(|state| state.gpr_source(9))
            .ok_or(StepProbeError::Assertion {
                case: CASE,
                check: "RI_CONFIG source lineage available",
            })?;
        require(
            CASE,
            matches!(
                step(&mut machine, CASE)?,
                MachineRepresentedStepOutcome::RiConfigStoreCommitted {
                    effective_address: 0xffff_ffff_a470_0004,
                    target: MachineStoreWordTarget::RiConfig,
                    source_gpr: 9,
                    stored_word,
                    state,
                    cadence_plan,
                } if stored_word == u32::from(word)
                    && state.current_control_input() == expected_input
                    && state.current_control_enable() == expected_enable
                    && state.source().instruction_pc() == CpuAddress::new(0xa400_0048)
                    && state.source().source_gpr() == 9
                    && state.source().source_lineage() == source_lineage
                    && cadence_plan.advances_count()
            ),
            "defined fields and CPU-store provenance",
        )?;
        require(
            CASE,
            machine.cpu().pc() == 0xa400_004c
                && machine.cpu().next_pc() == 0xa400_0050
                && machine.cpu().cop0_count() == 3,
            "RI_CONFIG success cadence",
        )?;
    }

    for base in [0x8470_u16, 0xa470_u16] {
        let words = [
            (0x40, immediate_word(0x0f, 0, 8, base)),
            (0x44, immediate_word(0x0d, 0, 9, 0x0040)),
            (0x48, immediate_word(0x2b, 8, 9, 4)),
        ];
        let (mut machine, _) = generated_cold_x105_machine(CASE, &words)?;
        step(&mut machine, CASE)?;
        step(&mut machine, CASE)?;
        require(
            CASE,
            matches!(
                step(&mut machine, CASE)?,
                MachineRepresentedStepOutcome::RiConfigStoreCommitted {
                    target: MachineStoreWordTarget::RiConfig,
                    stored_word: 0x40,
                    ..
                }
            ),
            "both direct aliases",
        )?;
    }

    const MISS_CASE: &str = "ri-config-neighbor-target-miss";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
        (0x44, immediate_word(0x2b, 8, 0, 8)),
    ];
    let (mut miss, _) = generated_cold_x105_machine(MISS_CASE, &words)?;
    step(&mut miss, MISS_CASE)?;
    let miss_before = (
        miss.cpu().pc(),
        miss.cpu().next_pc(),
        miss.cpu().cop0_count(),
        miss.ri_select_state(),
        miss.ri_config_state(),
    );
    match miss.step() {
        Err(MachineRepresentedStepError::StoreWordRejected(rejection)) => require(
            MISS_CASE,
            rejection.cpu_address() == Some(CpuAddress::new(0xa470_0008))
                && rejection.reason() == MachineStoreWordRejectionReason::DirectTargetMiss,
            "RI_CURRENT_LOAD remains a target miss",
        )?,
        Err(source) => {
            return Err(StepProbeError::Step {
                case: MISS_CASE,
                source,
            })
        }
        Ok(_) => return assertion(MISS_CASE, "neighbor write rejected"),
    }
    require(
        MISS_CASE,
        miss_before
            == (
                miss.cpu().pc(),
                miss.cpu().next_pc(),
                miss.cpu().cop0_count(),
                miss.ri_select_state(),
                miss.ri_config_state(),
            ),
        "neighbor rejection rollback",
    )?;

    const RESERVED_CASE: &str = "ri-config-reserved-bits-rejection";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
        (0x44, immediate_word(0x0d, 0, 9, 0x0080)),
        (0x48, immediate_word(0x2b, 8, 9, 4)),
    ];
    let (mut reserved, _) = generated_cold_x105_machine(RESERVED_CASE, &words)?;
    step(&mut reserved, RESERVED_CASE)?;
    step(&mut reserved, RESERVED_CASE)?;
    let reserved_before = (
        reserved.cpu().pc(),
        reserved.cpu().next_pc(),
        reserved.cpu().cop0_count(),
        reserved.ri_config_state(),
    );
    match reserved.step() {
        Err(MachineRepresentedStepError::StoreWordRejected(rejection)) => require(
            RESERVED_CASE,
            rejection.target() == Some(MachineStoreWordTarget::RiConfig)
                && rejection.reason()
                    == MachineStoreWordRejectionReason::RiConfigReservedBitsUnsupported {
                        unsupported_bits: 0x80,
                    },
            "reserved bits rejected before mutation",
        )?,
        Err(source) => {
            return Err(StepProbeError::Step {
                case: RESERVED_CASE,
                source,
            })
        }
        Ok(_) => return assertion(RESERVED_CASE, "reserved bits rejected"),
    }
    require(
        RESERVED_CASE,
        reserved_before
            == (
                reserved.cpu().pc(),
                reserved.cpu().next_pc(),
                reserved.cpu().cop0_count(),
                reserved.ri_config_state(),
            ),
        "reserved rejection rollback",
    )?;

    const ADES_CASE: &str = "ri-config-unaligned-ades";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
        (0x44, immediate_word(0x2b, 8, 7, 5)),
    ];
    let (mut ades, _) = generated_cold_x105_machine(ADES_CASE, &words)?;
    step(&mut ades, ADES_CASE)?;
    require(
        ADES_CASE,
        matches!(
            step(&mut ades, ADES_CASE)?,
            MachineRepresentedStepOutcome::DataAddressError {
                identity: CpuInstructionIdentity::Sw,
                effective_address: 0xffff_ffff_a470_0005,
                address_error,
                cadence_plan,
            } if address_error.exception_kind() == CpuAddressErrorKind::AddressErrorStore
                && address_error.bad_vaddr() == CpuAddress::new(0xa470_0005)
                && !cadence_plan.advances_count()
        ) && ades.ri_config_state().is_none(),
        "unaligned RI_CONFIG uses existing AdES",
    )?;

    const SLOT_CASE: &str = "ri-config-delay-slot-committed";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
        (0x44, immediate_word(0x0d, 0, 9, 0x0040)),
        (0x48, branch_word(0x04, 0, 0, 1)),
        (0x4c, immediate_word(0x2b, 8, 9, 4)),
        (0x50, special_shift_word(0, 0, 0, 0, 0)),
    ];
    let (mut slot, _) = generated_cold_x105_machine(SLOT_CASE, &words)?;
    step(&mut slot, SLOT_CASE)?;
    step(&mut slot, SLOT_CASE)?;
    require_committed_identity(
        SLOT_CASE,
        step(&mut slot, SLOT_CASE)?,
        CpuInstructionIdentity::Beq,
    )?;
    require(
        SLOT_CASE,
        matches!(
            step(&mut slot, SLOT_CASE)?,
            MachineRepresentedStepOutcome::RiConfigStoreCommitted {
                stored_word: 0x40,
                cadence_plan,
                ..
            } if cadence_plan.advances_count()
        ) && slot.cpu().pc() == 0xa400_0050
            && slot.cpu().next_pc() == 0xa400_0054
            && slot.cpu().cop0_count() == 4
            && slot.cpu_delay_slot_context().is_none(),
        "delay-slot register write and cadence",
    )?;

    const LIFECYCLE_CASE: &str = "ri-config-bootstrap-clears-stale-state";
    let words = [
        (0x40, immediate_word(0x0f, 0, 8, 0xa470)),
        (0x44, immediate_word(0x0d, 0, 9, 0x0040)),
        (0x48, immediate_word(0x2b, 8, 9, 4)),
    ];
    let (mut first, _) = generated_cold_x105_machine(LIFECYCLE_CASE, &words)?;
    let (second, _) = generated_cold_x105_machine(LIFECYCLE_CASE, &words)?;
    step(&mut first, LIFECYCLE_CASE)?;
    step(&mut first, LIFECYCLE_CASE)?;
    step(&mut first, LIFECYCLE_CASE)?;
    require(
        LIFECYCLE_CASE,
        first.ri_config_state().is_some() && second.ri_config_state().is_none(),
        "independent RI_CONFIG state",
    )?;
    first
        .stage_cartridge_bootstrap()
        .map_err(|source| StepProbeError::Bootstrap {
            case: LIFECYCLE_CASE,
            source,
        })?;
    require(
        LIFECYCLE_CASE,
        first.ri_config_state().is_none() && first.ri_select_state().is_some(),
        "rebootstrap clears stale config only",
    )?;
    step(&mut first, LIFECYCLE_CASE)?;
    step(&mut first, LIFECYCLE_CASE)?;
    step(&mut first, LIFECYCLE_CASE)?;
    first.reset();
    require(
        LIFECYCLE_CASE,
        first.ri_config_state().is_none() && first.ri_select_state().is_none(),
        "reset clears RI state",
    )
}

fn probe_generated_x105_post_mtc0_trio_frontier() -> Result<(), StepProbeError> {
    const CASE: &str = "generated-x105-post-mtc0-trio-frontier";
    let compare_word = cop0_move_word(4, 0, 11);
    let ri_base_word = immediate_word(0x0f, 0, 8, 0xa470);
    let ri_select_load_word = immediate_word(0x23, 8, 9, 0x000c);
    let words = [
        (0x40, special_word(29, 0, 9, 0x20)),
        (0x44, immediate_word(0x23, 9, 8, 0xf010)),
        (0x48, immediate_word(0x23, 11, 10, 0x0044)),
        (0x4c, special_word(10, 8, 10, 0x26)),
        (0x50, immediate_word(0x2b, 9, 10, 0xf010)),
        (0x54, immediate_word(0x08, 11, 11, 4)),
        (0x58, immediate_word(0x0c, 8, 8, 0x0fff)),
        (0x5c, branch_word(0x05, 8, 0, -7)),
        (0x60, immediate_word(0x08, 9, 9, 4)),
        (0x64, immediate_word(0x23, 11, 8, 0x0044)),
        (0x68, immediate_word(0x23, 11, 10, 0x0048)),
        (0x6c, immediate_word(0x2b, 9, 8, 0xf010)),
        (0x70, immediate_word(0x2b, 9, 10, 0xf014)),
        (0x74, immediate_word(0x01, 31, 0, 1)),
        (0x78, immediate_word(0x2b, 9, 0, 0xf018)),
        (0x7c, cop0_move_word(4, 0, 13)),
        (0x80, cop0_move_word(4, 0, 9)),
        (0x84, compare_word),
        (0x88, ri_base_word),
        (0x8c, ri_select_load_word),
        (0x90, branch_word(0x05, 9, 0, 15)),
        (0x94, special_shift_word(0, 0, 0, 0, 0)),
        (0x98, immediate_word(0x09, 29, 29, 0xffe8)),
        (0x9c, immediate_word(0x2b, 29, 19, 0x0000)),
        (0xa0, immediate_word(0x2b, 29, 20, 0x0004)),
        (0xa4, immediate_word(0x2b, 29, 21, 0x0008)),
        (0xa8, immediate_word(0x2b, 29, 22, 0x000c)),
        (0xac, immediate_word(0x2b, 29, 23, 0x0010)),
        (0xb0, immediate_word(0x0f, 0, 8, 0xa470)),
        (0xb4, immediate_word(0x0f, 0, 10, 0xa3f8)),
        (0xb8, immediate_word(0x0f, 0, 11, 0xa3f0)),
        (0xbc, immediate_word(0x0f, 0, 12, 0xa430)),
        (0xc0, immediate_word(0x0d, 0, 9, 0x0040)),
        (0xc4, immediate_word(0x2b, 8, 9, 0x0004)),
        (0xc8, immediate_word(0x09, 0, 17, 8000)),
        (0xcc, special_shift_word(0, 0, 0, 0, 0)),
        (0xd0, immediate_word(0x08, 17, 17, 0xffff)),
        (0xd4, branch_word(0x05, 17, 0, -3)),
        (0xd8, special_shift_word(0, 0, 0, 0, 0)),
        (0xdc, immediate_word(0x2b, 8, 0, 0x0008)),
    ];
    let (mut machine, generated_sp_imem_word) = generated_cold_x105_machine(CASE, &words)?;
    require(
        CASE,
        generated_sp_imem_word & 0x0fff == 0,
        "generated branch relation",
    )?;
    let expected = [
        CpuInstructionIdentity::SpecialAdd,
        CpuInstructionIdentity::Lw,
        CpuInstructionIdentity::Lw,
        CpuInstructionIdentity::SpecialXor,
        CpuInstructionIdentity::Sw,
        CpuInstructionIdentity::Addi,
        CpuInstructionIdentity::Andi,
        CpuInstructionIdentity::Bne,
        CpuInstructionIdentity::Addi,
        CpuInstructionIdentity::Lw,
        CpuInstructionIdentity::Lw,
        CpuInstructionIdentity::Sw,
        CpuInstructionIdentity::Sw,
    ];
    for (index, identity) in expected.into_iter().enumerate() {
        let outcome = step(&mut machine, CASE)?;
        require(
            CASE,
            outcome.identity() == Some(identity),
            "generated identity",
        )?;
        require(
            CASE,
            machine.cpu().cop0_count() == (index + 1) as u32,
            "generated Count cadence",
        )?;
        if index == 4 {
            require(
                CASE,
                matches!(
                    outcome,
                    MachineRepresentedStepOutcome::StoreWordCommitted {
                        target: MachineStoreWordTarget::SpImem { offset: 0 },
                        ..
                    }
                ),
                "frontier Sw committed to SP IMEM",
            )?;
        }
    }
    require(
        CASE,
        machine.cpu().pc() == 0xa400_0074,
        "pre-BLTZ frontier pc",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == 0xa400_0078,
        "pre-BLTZ frontier next_pc",
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 13,
        "pre-BLTZ frontier Count",
    )?;
    let retained_ra = machine.cpu().gpr(31);
    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::RegimmBltz,
    )?;
    require(CASE, machine.cpu().pc() == 0xa400_0078, "BLTZ slot pc")?;
    require(CASE, machine.cpu().next_pc() == 0xa400_007c, "BLTZ target")?;
    require(CASE, machine.cpu().cop0_count() == 14, "BLTZ Count")?;
    require(CASE, machine.cpu().gpr(31) == retained_ra, "BLTZ kept r31")?;

    require(
        CASE,
        matches!(
            step(&mut machine, CASE)?,
            MachineRepresentedStepOutcome::StoreWordCommitted {
                effective_address: 0xffff_ffff_a400_100c,
                target: MachineStoreWordTarget::SpImem { offset: 0x00c },
                source_gpr: 0,
                stored_word: 0,
                stored_bytes: [0, 0, 0, 0],
                provenance,
                ..
            } if provenance.instruction_pc() == CpuAddress::new(0xa400_0078)
                && provenance.source_lineage() == MachineBootstrapGprSource::ArchitecturalZero
        ),
        "x105 BLTZ delay-slot zero store",
    )?;
    require(
        CASE,
        machine.cpu().pc() == 0xa400_007c,
        "target frontier pc",
    )?;
    require(
        CASE,
        machine.cpu().next_pc() == 0xa400_0080,
        "target frontier next_pc",
    )?;
    require(CASE, machine.cpu().cop0_count() == 15, "post-slot Count")?;
    require(
        CASE,
        machine.cpu_delay_slot_context().is_none(),
        "post-slot context cleared",
    )?;
    require_mtc0_commit(
        CASE,
        step(&mut machine, CASE)?,
        MachineMtc0Destination::CauseSoftwareInterruptPending,
        0,
    )?;
    require(
        CASE,
        machine.cpu().cop0_software_interrupt_pending_known()
            && machine.cpu().cop0_software_interrupt_pending() == 0
            && machine.cpu().cop0_count() == 16,
        "generated MTC0 Cause",
    )?;
    require_mtc0_commit(
        CASE,
        step(&mut machine, CASE)?,
        MachineMtc0Destination::Count,
        0,
    )?;
    require(
        CASE,
        machine.cpu().cop0_count() == 1,
        "generated MTC0 Count",
    )?;
    require_mtc0_commit(
        CASE,
        step(&mut machine, CASE)?,
        MachineMtc0Destination::Compare,
        0,
    )?;
    require(
        CASE,
        machine.cpu().cop0_compare() == 0
            && machine.cpu().cop0_count() == 2
            && !machine.cpu().cop0_timer_interrupt_pending(),
        "generated MTC0 Compare",
    )?;
    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::Lui)?;
    require(
        CASE,
        machine.cpu().gpr(8) == Some(0xffff_ffff_a470_0000)
            && machine.cpu().pc() == 0xa400_008c
            && machine.cpu().next_pc() == 0xa400_0090
            && machine.cpu().cop0_count() == 3,
        "generated RI base construction",
    )?;

    require(
        CASE,
        matches!(
            step(&mut machine, CASE)?,
            MachineRepresentedStepOutcome::LoadWordCommitted {
                effective_address: 0xffff_ffff_a470_000c,
                target: MachineLoadWordTarget::RiSelect {
                    source: MachineRiSelectSource::ColdX105Entry,
                },
                destination_gpr: 9,
                loaded_word: 0,
                result_value: 0,
                cadence_plan,
            } if cadence_plan.advances_count()
        ),
        "RI_SELECT cold read",
    )?;
    require(
        CASE,
        machine.cpu().gpr(9) == Some(0)
            && machine.cpu().pc() == 0xa400_0090
            && machine.cpu().next_pc() == 0xa400_0094
            && machine.cpu().cop0_count() == 4,
        "RI_SELECT result and cadence",
    )?;

    require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::Bne)?;
    require(
        CASE,
        machine.cpu().pc() == 0xa400_0094
            && machine.cpu().next_pc() == 0xa400_0098
            && machine.cpu().cop0_count() == 5,
        "cold BNE fall-through slot",
    )?;
    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::SpecialSll,
    )?;
    require(
        CASE,
        machine.cpu().pc() == 0xa400_0098
            && machine.cpu().next_pc() == 0xa400_009c
            && machine.cpu().cop0_count() == 6
            && machine.cpu_delay_slot_context().is_none(),
        "cold NOP delay slot",
    )?;

    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::Addiu,
    )?;
    require(
        CASE,
        machine.cpu().gpr(29) == Some(0xffff_ffff_a400_1fd8),
        "cold stack adjustment",
    )?;

    for (index, (offset, source_gpr, stored_word)) in [
        (0x0fd8, 19, 0_u32),
        (0x0fdc, 20, 1_u32),
        (0x0fe0, 21, 0_u32),
        (0x0fe4, 22, 0x91_u32),
        (0x0fe8, 23, 0_u32),
    ]
    .into_iter()
    .enumerate()
    {
        require(
            CASE,
            matches!(
                step(&mut machine, CASE)?,
                MachineRepresentedStepOutcome::StoreWordCommitted {
                    target: MachineStoreWordTarget::SpImem { offset: observed_offset },
                    source_gpr: observed_source_gpr,
                    stored_word: observed_word,
                    stored_bytes,
                    provenance,
                    cadence_plan,
                    ..
                } if observed_offset == offset
                    && observed_source_gpr == source_gpr
                    && observed_word == stored_word
                    && stored_bytes == stored_word.to_be_bytes()
                    && provenance.instruction_pc()
                        == CpuAddress::new(0xa400_009c + index as u32 * 4)
                    && cadence_plan.advances_count()
            ),
            "cold stack store",
        )?;
    }

    for (identity, register, value) in [
        (CpuInstructionIdentity::Lui, 8, 0xffff_ffff_a470_0000),
        (CpuInstructionIdentity::Lui, 10, 0xffff_ffff_a3f8_0000),
        (CpuInstructionIdentity::Lui, 11, 0xffff_ffff_a3f0_0000),
        (CpuInstructionIdentity::Lui, 12, 0xffff_ffff_a430_0000),
        (CpuInstructionIdentity::Ori, 9, 0x40),
    ] {
        require_committed_identity(CASE, step(&mut machine, CASE)?, identity)?;
        require(
            CASE,
            machine.cpu().gpr(register) == Some(value),
            "RI prefix value",
        )?;
    }
    require(
        CASE,
        machine.cpu().pc() == 0xa400_00c4
            && machine.cpu().next_pc() == 0xa400_00c8
            && machine.cpu().cop0_count() == 17,
        "RI_CONFIG frontier state",
    )?;

    require(
        CASE,
        matches!(
            step(&mut machine, CASE)?,
            MachineRepresentedStepOutcome::RiConfigStoreCommitted {
                effective_address: 0xffff_ffff_a470_0004,
                target: MachineStoreWordTarget::RiConfig,
                source_gpr: 9,
                stored_word: 0x40,
                state,
                cadence_plan,
            } if state.current_control_input() == 0
                && state.current_control_enable()
                && state.source().instruction_pc() == CpuAddress::new(0xa400_00c4)
                && cadence_plan.advances_count()
        ),
        "RI_CONFIG generated store",
    )?;
    let config = machine.ri_config_state().ok_or(StepProbeError::Assertion {
        case: CASE,
        check: "generated RI_CONFIG state available",
    })?;
    require(
        CASE,
        machine.cpu().pc() == 0xa400_00c8
            && machine.cpu().next_pc() == 0xa400_00cc
            && machine.cpu().cop0_count() == 18,
        "post-config cadence",
    )?;
    require_committed_identity(
        CASE,
        step(&mut machine, CASE)?,
        CpuInstructionIdentity::Addiu,
    )?;
    require(
        CASE,
        machine.cpu().gpr(17) == Some(8000)
            && machine.cpu().pc() == 0xa400_00cc
            && machine.cpu().next_pc() == 0xa400_00d0
            && machine.cpu().cop0_count() == 19,
        "wait counter setup",
    )?;

    let mut loop_commits = 0_u32;
    let mut taken = 0_u32;
    let mut untaken = 0_u32;
    let mut slots = 0_u32;
    for iteration in 0..8000_u32 {
        require_committed_identity(
            CASE,
            step(&mut machine, CASE)?,
            CpuInstructionIdentity::SpecialSll,
        )?;
        loop_commits += 1;
        require_committed_identity(
            CASE,
            step(&mut machine, CASE)?,
            CpuInstructionIdentity::Addi,
        )?;
        loop_commits += 1;
        require_committed_identity(CASE, step(&mut machine, CASE)?, CpuInstructionIdentity::Bne)?;
        loop_commits += 1;
        let selected_next_pc = if iteration == 7999 {
            untaken += 1;
            0xa400_00dc
        } else {
            taken += 1;
            0xa400_00cc
        };
        require(
            CASE,
            machine.cpu().pc() == 0xa400_00d8 && machine.cpu().next_pc() == selected_next_pc,
            "wait branch selection",
        )?;
        require_committed_identity(
            CASE,
            step(&mut machine, CASE)?,
            CpuInstructionIdentity::SpecialSll,
        )?;
        loop_commits += 1;
        slots += 1;
    }
    require(
        CASE,
        loop_commits == 32_000
            && 33 + 1 + 1 + loop_commits == 32_035
            && taken == 7_999
            && untaken == 1
            && slots == 8_000
            && machine.cpu().gpr(17) == Some(0)
            && machine.cpu().pc() == 0xa400_00dc
            && machine.cpu().next_pc() == 0xa400_00e0
            && machine.cpu().cop0_count() == 32_019
            && machine.ri_config_state() == Some(config),
        "exact 8,000-iteration CPU wait composition",
    )?;

    let before = (
        machine.cpu().pc(),
        machine.cpu().next_pc(),
        machine.cpu().cop0_count(),
        machine.cpu().gpr(17),
        machine.ri_select_state(),
        machine.ri_config_state(),
    );
    match machine.step() {
        Err(MachineRepresentedStepError::StoreWordRejected(rejection)) => require(
            CASE,
            rejection.effective_address() == Some(0xffff_ffff_a470_0008)
                && rejection.cpu_address() == Some(CpuAddress::new(0xa470_0008))
                && rejection.reason() == MachineStoreWordRejectionReason::DirectTargetMiss,
            "RI_CURRENT_LOAD store frontier",
        )?,
        Err(source) => return Err(StepProbeError::Step { case: CASE, source }),
        Ok(_) => return assertion(CASE, "RI_CURRENT_LOAD remains unsupported"),
    }
    require(
        CASE,
        before
            == (
                machine.cpu().pc(),
                machine.cpu().next_pc(),
                machine.cpu().cop0_count(),
                machine.cpu().gpr(17),
                machine.ri_select_state(),
                machine.ri_config_state(),
            ),
        "RI_CURRENT_LOAD rejection rollback",
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

fn require_mtc0_commit(
    case: &'static str,
    outcome: MachineRepresentedStepOutcome,
    expected_destination: MachineMtc0Destination,
    expected_transfer_word: u32,
) -> Result<(), StepProbeError> {
    require(
        case,
        matches!(
            outcome,
            MachineRepresentedStepOutcome::Mtc0Committed {
                destination,
                transfer_word,
                cadence_plan,
                ..
            } if destination == expected_destination
                && transfer_word == expected_transfer_word
                && cadence_plan.source() == MachineStepCadenceSource::CommittedInstruction
                && cadence_plan.count_action() == MachineStepCountAction::Advance
        ),
        "MTC0 destination, transfer word, and cadence",
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

fn special_shift_word(rs: u8, rt: u8, rd: u8, sa: u8, funct: u8) -> u32 {
    (u32::from(rs) << 21)
        | (u32::from(rt) << 16)
        | (u32::from(rd) << 11)
        | (u32::from(sa) << 6)
        | u32::from(funct)
}

fn cop0_move_word(rs: u8, rt: u8, rd: u8) -> u32 {
    (0x10_u32 << 26) | (u32::from(rs) << 21) | (u32::from(rt) << 16) | (u32::from(rd) << 11)
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
    let mut firmware: Vec<u8> = (0..PIF_BOOT_ROM_SIZE_BYTES)
        .map(|index| 0xa5_u8.wrapping_add((index as u8).wrapping_mul(29)))
        .collect();
    let source_start = PifIpl2Profile::NtscPinned
        .copy_layout()
        .source_start_offset() as usize;
    write_be_u32(&mut firmware, source_start, 0x81ab_c000);
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
