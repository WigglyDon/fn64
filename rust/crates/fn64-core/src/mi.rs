use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const MI_INIT_MODE_PHYSICAL_ADDRESS: u32 = 0x0430_0000;
pub const MI_INIT_MODE_X105_WRITE_WORD: u32 = 0x0000_010f;
pub const MI_INIT_MODE_X105_INIT_LENGTH: u8 = 15;
pub const MI_INIT_MODE_X105_REPEATED_BYTE_COUNT: u8 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineMiInitModeSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    },
}

impl MachineMiInitModeSource {
    pub const fn instruction_pc(self) -> CpuAddress {
        match self {
            Self::CpuStoreWord { instruction_pc, .. } => instruction_pc,
        }
    }

    pub const fn source_gpr(self) -> u8 {
        match self {
            Self::CpuStoreWord { source_gpr, .. } => source_gpr,
        }
    }

    pub const fn source_lineage(self) -> MachineBootstrapGprSource {
        match self {
            Self::CpuStoreWord { source_lineage, .. } => source_lineage,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineMiInitModeState {
    init_length: u8,
    init_mode: bool,
    source: MachineMiInitModeSource,
}

impl MachineMiInitModeState {
    pub(crate) const fn from_exact_x105_cpu_store(
        word: u32,
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        debug_assert!(word == MI_INIT_MODE_X105_WRITE_WORD);
        Self {
            init_length: MI_INIT_MODE_X105_INIT_LENGTH,
            init_mode: true,
            source: MachineMiInitModeSource::CpuStoreWord {
                instruction_pc,
                source_gpr,
                source_lineage,
            },
        }
    }

    pub const fn init_length(self) -> u8 {
        self.init_length
    }

    pub const fn init_mode(self) -> bool {
        self.init_mode
    }

    pub const fn source(self) -> MachineMiInitModeSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineMiInitTransferState {
    source_init_length: u8,
    repeated_byte_count: u8,
    command_word: u32,
    source: MachineMiInitModeSource,
}

impl MachineMiInitTransferState {
    const fn from_exact_x105_init_mode(state: MachineMiInitModeState) -> Self {
        Self {
            source_init_length: state.init_length(),
            repeated_byte_count: MI_INIT_MODE_X105_REPEATED_BYTE_COUNT,
            command_word: MI_INIT_MODE_X105_WRITE_WORD,
            source: state.source(),
        }
    }

    pub const fn source_init_length(self) -> u8 {
        self.source_init_length
    }

    pub const fn repeated_byte_count(self) -> u8 {
        self.repeated_byte_count
    }

    pub const fn command_word(self) -> u32 {
        self.command_word
    }

    pub const fn source(self) -> MachineMiInitModeSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Mi {
    init_mode: Option<MachineMiInitModeState>,
    init_transfer: Option<MachineMiInitTransferState>,
}

impl Mi {
    pub(crate) const fn init_mode_state(self) -> Option<MachineMiInitModeState> {
        self.init_mode
    }

    pub(crate) const fn init_transfer_state(self) -> Option<MachineMiInitTransferState> {
        self.init_transfer
    }

    pub(crate) fn apply_init_mode_store(&mut self, state: MachineMiInitModeState) {
        self.init_mode = Some(state);
        self.init_transfer = Some(MachineMiInitTransferState::from_exact_x105_init_mode(state));
    }

    pub(crate) fn consume_init_transfer(&mut self) {
        self.init_mode = None;
        self.init_transfer = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CpuInstructionIdentity;

    #[test]
    fn mi_init_mode_exact_x105_write_owns_result_state_and_lineage() {
        assert_eq!(MI_INIT_MODE_PHYSICAL_ADDRESS, 0x0430_0000);
        assert_eq!(MI_INIT_MODE_X105_WRITE_WORD, 0x0000_010f);
        assert_eq!(MI_INIT_MODE_X105_INIT_LENGTH, 15);
        assert_eq!(Mi::default().init_mode_state(), None);

        let lineage = MachineBootstrapGprSource::KnownInstructionResult {
            execution_address: CpuAddress::new(0xa400_0114),
            identity: CpuInstructionIdentity::Ori,
            source_gpr_a: Some(0),
            source_gpr_b: None,
        };
        let state = MachineMiInitModeState::from_exact_x105_cpu_store(
            MI_INIT_MODE_X105_WRITE_WORD,
            CpuAddress::new(0xa400_0118),
            9,
            lineage,
        );

        assert_eq!(state.init_length(), 15);
        assert!(state.init_mode());
        assert_eq!(
            state.source().instruction_pc(),
            CpuAddress::new(0xa400_0118)
        );
        assert_eq!(state.source().source_gpr(), 9);
        assert_eq!(state.source().source_lineage(), lineage);

        let mut mi = Mi::default();
        mi.apply_init_mode_store(state);
        assert_eq!(mi.init_mode_state(), Some(state));
        let transfer = mi.init_transfer_state().unwrap();
        assert_eq!(transfer.source_init_length(), 15);
        assert_eq!(transfer.repeated_byte_count(), 16);
        assert_eq!(transfer.command_word(), MI_INIT_MODE_X105_WRITE_WORD);
        assert_eq!(transfer.source(), state.source());

        mi.consume_init_transfer();
        assert_eq!(mi.init_mode_state(), None);
        assert_eq!(mi.init_transfer_state(), None);
    }
}
