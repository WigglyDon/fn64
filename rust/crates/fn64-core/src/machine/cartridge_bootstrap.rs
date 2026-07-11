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
use crate::rdram::Rdram;
use crate::sp_dmem::{SpDmem, SpDmemOffset, SpDmemWriteError};
use crate::sp_imem::SpImem;

use super::rdram_reservation::CpuRdramReservation;

pub const MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC: u32 = 0xa400_0040;
pub const MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC: u32 = 0xa400_0044;
pub const MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET: u32 =
    CARTRIDGE_CANDIDATE_IPL3_START_OFFSET;
pub const MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE: u32 =
    CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE;
pub const MACHINE_GENERAL_PIF_RESET_GPR29_VALUE: u64 = 0xffff_ffff_a400_1ff0;
pub const MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX: u8 = 29;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineBootstrapCpuStateKind {
    /// The current represented reset subset plus the IPL3 `pc / next_pc` pair.
    ///
    /// Architectural GPR zero and the general PIF reset stack pointer are
    /// source-backed. Other PIF/CIC-produced GPR, COP0, or device state remains
    /// explicitly unknown rather than being inferred from zeroed storage.
    RepresentedResetSubset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineBootstrapGprSource {
    UnknownPifProduced,
    ArchitecturalZero,
    GeneralPifRomResetStackPointer,
    KnownInstructionResult {
        execution_address: CpuAddress,
        identity: CpuInstructionIdentity,
        source_gpr_a: Option<u8>,
        source_gpr_b: Option<u8>,
    },
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

    fn contains_sp_dmem_instruction(self, offset: SpDmemOffset) -> bool {
        let start = self.sp_dmem_start_offset;
        let Some(end) = offset.value().checked_add(4) else {
            return false;
        };

        offset.value() >= start && end <= self.sp_dmem_end_offset_exclusive
    }

    fn cartridge_offset_for_sp_dmem(self, offset: SpDmemOffset) -> u32 {
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
    /// Creates fn64's current machine-owned cartridge bootstrap state.
    ///
    /// This operation consumes only the already-normalized Cartridge owned by
    /// this Machine. It preflights and materializes the complete IPL3 source
    /// span before replacing represented CPU, RDRAM, SP DMEM, and reservation
    /// state. The execution PC is staged last in the replacement state. The
    /// represented reset subset stages only architectural zero and the general
    /// PIF reset stack pointer. Other PIF/CIC-produced register or device state
    /// remains explicitly unknown.
    pub fn stage_cartridge_bootstrap(
        &mut self,
    ) -> Result<MachineCartridgeBootstrapState, MachineCartridgeBootstrapError> {
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
        let replacement_sp_imem = SpImem::default();

        let mut replacement_cpu = Cpu::new();
        replacement_cpu
            .set_gpr(
                usize::from(MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX),
                MACHINE_GENERAL_PIF_RESET_GPR29_VALUE,
            )
            .map_err(MachineCartridgeBootstrapError::CpuRegister)?;
        replacement_cpu.stage_pc(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC);

        let mut gpr_sources = [MachineBootstrapGprSource::UnknownPifProduced; CPU_GPR_COUNT];
        gpr_sources[0] = MachineBootstrapGprSource::ArchitecturalZero;
        gpr_sources[usize::from(MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX)] =
            MachineBootstrapGprSource::GeneralPifRomResetStackPointer;

        let state = MachineCartridgeBootstrapState {
            source_layout: self.cartridge.source_layout(),
            cartridge_start_offset: CARTRIDGE_CANDIDATE_IPL3_START_OFFSET,
            cartridge_end_offset_exclusive: CARTRIDGE_CANDIDATE_IPL3_END_OFFSET_EXCLUSIVE,
            sp_dmem_start_offset: MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_START_OFFSET,
            sp_dmem_end_offset_exclusive: MACHINE_CARTRIDGE_BOOTSTRAP_SP_DMEM_END_OFFSET_EXCLUSIVE,
            execution_pc: CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_EXECUTION_PC),
            next_pc: CpuAddress::new(MACHINE_CARTRIDGE_BOOTSTRAP_NEXT_PC),
            cpu_state_kind: MachineBootstrapCpuStateKind::RepresentedResetSubset,
            gpr_sources,
        };

        self.cpu = replacement_cpu;
        self.rdram = Rdram::default();
        self.sp_dmem = replacement_sp_dmem;
        self.sp_imem = replacement_sp_imem;
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
                    Some(state) if state.contains_sp_dmem_instruction(offset) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::load_cartridge;
    use crate::cpu::{CpuInstructionIdentity, CPU_GPR_COUNT, NON_BOOT_RESET_VECTOR_PC};
    use crate::machine::{MachineRepresentedStepError, MachineRepresentedStepOutcome};
    use crate::sp_imem::{SpImemByteObservation, SpImemByteProvenance, SpImemOffset};

    #[derive(Debug, PartialEq, Eq)]
    struct MachineArchitecturalSnapshot {
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
        cop0_epc: u32,
        cop0_bad_vaddr: u32,
        cop0_exception_code: u8,
        cop0_exception_branch_delay: bool,
        rdram: Vec<u8>,
        sp_dmem: Vec<u8>,
        sp_imem: Vec<SpImemByteObservation>,
        bootstrap: Option<MachineCartridgeBootstrapState>,
    }

    fn architectural_snapshot(machine: &Machine) -> MachineArchitecturalSnapshot {
        MachineArchitecturalSnapshot {
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
                state.gpr_source(usize::from(
                    MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX
                )),
                Some(MachineBootstrapGprSource::GeneralPifRomResetStackPointer)
            );
            assert_eq!(
                state.gpr_is_known(usize::from(
                    MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX
                )),
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
                machine.cpu().gpr(usize::from(
                    MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX
                )),
                Some(MACHINE_GENERAL_PIF_RESET_GPR29_VALUE)
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
                usize::from(MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX),
                0x1357_9bdf_2468_ace0,
            )
            .unwrap();

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
            machine.cpu().gpr(usize::from(
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX
            )),
            Some(0x1357_9bdf_2468_ace0)
        );
        assert_eq!(machine.rdram().read_u32_be(0x30), Ok(0x1020_3040));
        assert_eq!(
            machine.sp_dmem().read_u32_be(SpDmemOffset::new(0x40)),
            Ok(0xaabb_ccdd)
        );
        assert_eq!(machine.cartridge_bootstrap_state(), None);
    }

    #[test]
    fn machine_bootstrap_reset_state_lineage_stages_only_zero_and_general_pif_stack_pointer() {
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
            machine.cpu().gpr(usize::from(
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX
            )),
            Some(MACHINE_GENERAL_PIF_RESET_GPR29_VALUE)
        );
        assert_eq!(
            state.gpr_source(usize::from(
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX
            )),
            Some(MachineBootstrapGprSource::GeneralPifRomResetStackPointer)
        );
        assert_eq!(
            state.gpr_is_known(usize::from(
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX
            )),
            Some(true)
        );

        for index in 1..CPU_GPR_COUNT {
            if index == usize::from(MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX) {
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
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX,
                0,
                9,
                Some(MACHINE_GENERAL_PIF_RESET_GPR29_VALUE),
            ),
            (
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX,
                0,
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX,
                Some(MACHINE_GENERAL_PIF_RESET_GPR29_VALUE),
            ),
            (
                MACHINE_GENERAL_PIF_RESET_STACK_POINTER_GPR_INDEX,
                0,
                0,
                Some(0),
            ),
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
