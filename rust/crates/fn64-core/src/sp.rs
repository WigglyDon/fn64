use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const SP_STATUS_PHYSICAL_ADDRESS: u32 = 0x0404_0010;
pub const SP_PC_PHYSICAL_ADDRESS: u32 = 0x0408_0000;
pub const SP_STATUS_X105_HALT_CONFIGURE_WORD: u32 = 0x0000_00ce;
pub const SP_STATUS_X105_START_WORD: u32 = 0x0000_00ad;
pub const SP_PC_X105_RESET_WORD: u32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpCpuStoreProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineSpCpuStoreProvenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
        effective_address: u64,
        cpu_address: CpuAddress,
        physical_address: u32,
    ) -> Self {
        Self {
            instruction_pc,
            source_gpr,
            source_lineage,
            effective_address,
            cpu_address,
            physical_address,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn source_gpr(self) -> u8 {
        self.source_gpr
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        self.source_lineage
    }

    pub const fn effective_address(self) -> u64 {
        self.effective_address
    }

    pub const fn cpu_address(self) -> CpuAddress {
        self.cpu_address
    }

    pub const fn physical_address(self) -> u32 {
        self.physical_address
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpStatusState {
    command_word: u32,
    halt: bool,
    broke: bool,
    interrupt_pending: bool,
    single_step: bool,
    interrupt_on_break: bool,
    source: MachineSpCpuStoreProvenance,
}

impl MachineSpStatusState {
    pub(crate) const fn from_x105_command(
        command_word: u32,
        source: MachineSpCpuStoreProvenance,
    ) -> Self {
        debug_assert!(
            command_word == SP_STATUS_X105_HALT_CONFIGURE_WORD
                || command_word == SP_STATUS_X105_START_WORD
        );
        let halt = command_word == SP_STATUS_X105_HALT_CONFIGURE_WORD;
        Self {
            command_word,
            halt,
            broke: false,
            interrupt_pending: false,
            single_step: halt,
            interrupt_on_break: false,
            source,
        }
    }

    pub const fn command_word(self) -> u32 {
        self.command_word
    }

    pub const fn halt(self) -> bool {
        self.halt
    }

    pub const fn broke(self) -> bool {
        self.broke
    }

    pub const fn interrupt_pending(self) -> bool {
        self.interrupt_pending
    }

    pub const fn single_step(self) -> bool {
        self.single_step
    }

    pub const fn interrupt_on_break(self) -> bool {
        self.interrupt_on_break
    }

    pub const fn source(self) -> MachineSpCpuStoreProvenance {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpPcState {
    raw_low_field: u32,
    source: MachineSpCpuStoreProvenance,
}

impl MachineSpPcState {
    pub(crate) const fn from_x105_zero(source: MachineSpCpuStoreProvenance) -> Self {
        Self {
            raw_low_field: SP_PC_X105_RESET_WORD,
            source,
        }
    }

    pub const fn raw_low_field(self) -> u32 {
        self.raw_low_field
    }

    pub const fn source(self) -> MachineSpCpuStoreProvenance {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Sp {
    status: Option<MachineSpStatusState>,
    pc: Option<MachineSpPcState>,
}

impl Sp {
    pub(crate) const fn status_state(self) -> Option<MachineSpStatusState> {
        self.status
    }

    pub(crate) const fn pc_state(self) -> Option<MachineSpPcState> {
        self.pc
    }

    pub(crate) fn apply_status_store(&mut self, state: MachineSpStatusState) {
        self.status = Some(state);
    }

    pub(crate) fn apply_pc_store(&mut self, state: MachineSpPcState) {
        self.pc = Some(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CpuInstructionIdentity;

    fn source(pc: u32, physical_address: u32) -> MachineSpCpuStoreProvenance {
        MachineSpCpuStoreProvenance::new(
            CpuAddress::new(pc),
            10,
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(pc - 4),
                identity: CpuInstructionIdentity::Addiu,
                source_gpr_a: Some(0),
                source_gpr_b: None,
            },
            u64::from(0xa000_0000 | physical_address),
            CpuAddress::new(0xa000_0000 | physical_address),
            physical_address,
        )
    }

    #[test]
    fn exact_x105_status_commands_derive_only_control_truth() {
        let halted = MachineSpStatusState::from_x105_command(
            SP_STATUS_X105_HALT_CONFIGURE_WORD,
            source(0xa400_0490, SP_STATUS_PHYSICAL_ADDRESS),
        );
        assert!(halted.halt());
        assert!(halted.single_step());
        assert!(!halted.broke());
        assert!(!halted.interrupt_pending());
        assert!(!halted.interrupt_on_break());

        let started = MachineSpStatusState::from_x105_command(
            SP_STATUS_X105_START_WORD,
            source(0xa400_0508, SP_STATUS_PHYSICAL_ADDRESS),
        );
        assert!(!started.halt());
        assert!(!started.single_step());
        assert!(!started.broke());
        assert!(!started.interrupt_pending());
        assert!(!started.interrupt_on_break());
    }

    #[test]
    fn sp_owner_starts_unavailable_and_replaces_exact_states() {
        let mut sp = Sp::default();
        assert_eq!(sp.status_state(), None);
        assert_eq!(sp.pc_state(), None);
        let status = MachineSpStatusState::from_x105_command(
            SP_STATUS_X105_HALT_CONFIGURE_WORD,
            source(0xa400_0490, SP_STATUS_PHYSICAL_ADDRESS),
        );
        let pc = MachineSpPcState::from_x105_zero(source(0xa400_04cc, SP_PC_PHYSICAL_ADDRESS));
        sp.apply_status_store(status);
        sp.apply_pc_store(pc);
        assert_eq!(sp.status_state(), Some(status));
        assert_eq!(sp.pc_state(), Some(pc));
    }
}
