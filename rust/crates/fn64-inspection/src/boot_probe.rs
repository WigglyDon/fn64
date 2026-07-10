use core::fmt;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::PathBuf;

use fn64_core::{
    load_cartridge, rom_source_layout_name, CartridgeLoadError, CpuInstructionIdentity, Machine,
    MachineCartridgeBootstrapError, MachineCpuInstructionInspection, MachineCpuInstructionSource,
    MachineRepresentedStepError, MachineRepresentedStepOutcome, MachineSpDmemInstructionProvenance,
    RomMetadata, RomSourceLayout, CPU_GPR_COUNT,
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
            Self::Boot2 => "ROM-derived instruction committed a represented machine effect",
            Self::Boot3 => "machine behavior reached the cartridge-declared program entry",
            Self::Boot4 => "program instruction after bootstrap handoff executed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootProbeArguments {
    input_path: PathBuf,
    max_steps: u64,
}

impl BootProbeArguments {
    pub fn input_path(&self) -> &PathBuf {
        &self.input_path
    }

    pub const fn max_steps(&self) -> u64 {
        self.max_steps
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootProbeArgumentError {
    Usage,
    InvalidMaxSteps,
}

impl fmt::Display for BootProbeArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => write!(
                f,
                "usage: fn64_boot_probe <rom-path> [--max-steps <positive-integer>]"
            ),
            Self::InvalidMaxSteps => write!(f, "--max-steps requires a positive integer"),
        }
    }
}

impl std::error::Error for BootProbeArgumentError {}

pub fn parse_boot_probe_arguments<I>(
    arguments: I,
) -> Result<BootProbeArguments, BootProbeArgumentError>
where
    I: IntoIterator<Item = OsString>,
{
    let arguments: Vec<OsString> = arguments.into_iter().collect();
    match arguments.as_slice() {
        [input_path] => Ok(BootProbeArguments {
            input_path: PathBuf::from(input_path),
            max_steps: DEFAULT_BOOT_PROBE_MAX_STEPS,
        }),
        [input_path, flag, value] if flag == "--max-steps" => {
            let max_steps = value
                .to_str()
                .and_then(|value| value.parse::<u64>().ok())
                .filter(|value| *value > 0)
                .ok_or(BootProbeArgumentError::InvalidMaxSteps)?;
            Ok(BootProbeArguments {
                input_path: PathBuf::from(input_path),
                max_steps,
            })
        }
        _ => Err(BootProbeArgumentError::Usage),
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct CpuObservation {
    pc: u32,
    next_pc: u32,
    count: u32,
    hi: u64,
    lo: u64,
    gprs: [u64; CPU_GPR_COUNT],
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
        Self {
            pc: machine.cpu().pc(),
            next_pc: machine.cpu().next_pc(),
            count: machine.cpu().cop0_count(),
            hi: machine.cpu().hi(),
            lo: machine.cpu().lo(),
            gprs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LastCommittedStep {
    inspection: MachineCpuInstructionInspection,
    outcome: &'static str,
    effect: String,
}

pub fn run_boot_probe(
    owned_input_bytes: Vec<u8>,
    input_path: &str,
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
    let staging = machine
        .stage_cartridge_bootstrap()
        .map_err(BootProbeError::Bootstrap)?;
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
                    | MachineRepresentedStepOutcome::NoEffectCommitted { .. } => {}
                    MachineRepresentedStepOutcome::Stopped { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-stop",
                            inspection,
                            outcome.identity(),
                        ));
                        break;
                    }
                    MachineRepresentedStepOutcome::Unsupported { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-unsupported-instruction",
                            inspection,
                            outcome.identity(),
                        ));
                        break;
                    }
                    MachineRepresentedStepOutcome::ArithmeticOverflowException { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-arithmetic-overflow-exception",
                            inspection,
                            outcome.identity(),
                        ));
                        break;
                    }
                    MachineRepresentedStepOutcome::InstructionFetchAddressError { .. } => {
                        first_frontier = Some(format_frontier(
                            "represented-instruction-fetch-address-error",
                            inspection,
                            None,
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
        if before.gprs[index] != after.gprs[index] {
            gpr_changes.push(format!(
                "r{}=0x{:016X}->0x{:016X}",
                index, before.gprs[index], after.gprs[index]
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

fn format_frontier(
    classification: &str,
    inspection: Option<MachineCpuInstructionInspection>,
    identity: Option<CpuInstructionIdentity>,
) -> String {
    match inspection {
        Some(inspection) => format!(
            "{} address=0x{:08X} identity={:?} rs={} rt={} rd={} immediate=0x{:04X} source={}",
            classification,
            inspection.cpu_address().value(),
            identity.unwrap_or_else(|| inspection.identity()),
            inspection.fields().rs(),
            inspection.fields().rt(),
            inspection.fields().rd(),
            inspection.fields().immediate_u16(),
            format_instruction_source(inspection.source())
        ),
        None => format!(
            "{} identity={:?} source=unavailable",
            classification, identity
        ),
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
    writeln!(
        output,
        "unrepresented_pif_cpu_state: {}",
        yes_no(facts.staging.has_unrepresented_pif_cpu_state())
    )
    .unwrap();
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
        }
        None => {
            writeln!(output, "last_committed_address: unavailable").unwrap();
            writeln!(output, "last_committed_identity: unavailable").unwrap();
            writeln!(output, "last_committed_source: unavailable").unwrap();
            writeln!(output, "last_committed_outcome: none").unwrap();
            writeln!(output, "last_committed_effect: none").unwrap();
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

    #[test]
    fn boot_probe_reports_rom_derived_commit_and_expected_frontier() {
        let report = run_boot_probe(
            make_generated_boot_fixture(0x3c08_1234, 0x8fa9_0000),
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
            .contains("unrepresented-instruction address=0xA4000044 identity=Lw rs=29 rt=9"));
        assert!(report.output().contains("last_committed_identity: Lui"));
        assert!(report
            .output()
            .contains("last_committed_source: cartridge-bootstrap cartridge_offset=0x00000040"));
        assert!(report.output().contains(
            "last_committed_effect: pc=0xA4000040->0xA4000044 next_pc=0xA4000044->0xA4000048 count=0->1"
        ));
        assert!(report.output().contains("highest_checkpoint: BOOT-2"));
        assert!(report.output().contains("probe_exit_status: 0"));
        assert!(report.output().contains("compatibility_claim: none"));
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
    }

    #[test]
    fn boot_probe_unrepresented_first_instruction_stops_at_boot1_without_mutation() {
        let report = run_boot_probe(
            make_generated_boot_fixture(0x8fa8_0000, 0),
            "generated-fixture.z64",
            100,
        )
        .unwrap();

        assert_eq!(report.highest_checkpoint(), BootCheckpoint::Boot1);
        assert_eq!(report.attempted_steps(), 1);
        assert_eq!(report.committed_steps(), 0);
        assert!(report.output().contains("pc: 0xA4000040"));
        assert!(report.output().contains("next_pc: 0xA4000044"));
        assert!(report.output().contains("last_committed_effect: none"));
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
    fn boot_probe_argument_parser_owns_fixed_budget_policy() {
        let default = parse_boot_probe_arguments([OsString::from("fixture.z64")]).unwrap();
        assert_eq!(default.input_path(), &PathBuf::from("fixture.z64"));
        assert_eq!(default.max_steps(), DEFAULT_BOOT_PROBE_MAX_STEPS);

        let explicit = parse_boot_probe_arguments([
            OsString::from("fixture.any"),
            OsString::from("--max-steps"),
            OsString::from("17"),
        ])
        .unwrap();
        assert_eq!(explicit.max_steps(), 17);
        assert_eq!(
            parse_boot_probe_arguments([
                OsString::from("fixture"),
                OsString::from("--max-steps"),
                OsString::from("0"),
            ]),
            Err(BootProbeArgumentError::InvalidMaxSteps)
        );
        assert_eq!(
            parse_boot_probe_arguments(Vec::<OsString>::new()),
            Err(BootProbeArgumentError::Usage)
        );
    }
}
