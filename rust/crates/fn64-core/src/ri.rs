use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const RI_MODE_PHYSICAL_ADDRESS: u32 = 0x0470_0000;
pub const RI_MODE_OPERATING_MODE_MASK: u32 = 0x0000_0003;
pub const RI_MODE_STOP_TRANSMIT_ACTIVE_MASK: u32 = 0x0000_0004;
pub const RI_MODE_STOP_RECEIVE_ACTIVE_MASK: u32 = 0x0000_0008;
pub const RI_MODE_DEFINED_FIELDS_MASK: u32 = RI_MODE_OPERATING_MODE_MASK
    | RI_MODE_STOP_TRANSMIT_ACTIVE_MASK
    | RI_MODE_STOP_RECEIVE_ACTIVE_MASK;
pub const RI_CONFIG_PHYSICAL_ADDRESS: u32 = 0x0470_0004;
pub const RI_CONFIG_CURRENT_CONTROL_INPUT_MASK: u32 = 0x0000_003f;
pub const RI_CONFIG_CURRENT_CONTROL_ENABLE_MASK: u32 = 0x0000_0040;
pub const RI_CONFIG_DEFINED_FIELDS_MASK: u32 =
    RI_CONFIG_CURRENT_CONTROL_INPUT_MASK | RI_CONFIG_CURRENT_CONTROL_ENABLE_MASK;
pub const RI_CURRENT_LOAD_PHYSICAL_ADDRESS: u32 = 0x0470_0008;
pub const RI_SELECT_PHYSICAL_ADDRESS: u32 = 0x0470_000c;
pub const RI_SELECT_X105_ENABLE_TX_RX_WORD: u32 = 0x0000_0014;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRiSelectSource {
    ColdX105Entry,
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    },
}

impl MachineRiSelectSource {
    pub const fn instruction_pc(self) -> Option<CpuAddress> {
        match self {
            Self::ColdX105Entry => None,
            Self::CpuStoreWord { instruction_pc, .. } => Some(instruction_pc),
        }
    }

    pub const fn source_gpr(self) -> Option<u8> {
        match self {
            Self::ColdX105Entry => None,
            Self::CpuStoreWord { source_gpr, .. } => Some(source_gpr),
        }
    }

    pub const fn source_lineage(self) -> Option<MachineBootstrapGprSource> {
        match self {
            Self::ColdX105Entry => None,
            Self::CpuStoreWord { source_lineage, .. } => Some(source_lineage),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineRiSelectState {
    value: u32,
    source: MachineRiSelectSource,
}

impl MachineRiSelectState {
    pub(crate) const fn cold_x105_entry() -> Self {
        Self {
            value: 0,
            source: MachineRiSelectSource::ColdX105Entry,
        }
    }

    pub(crate) const fn from_cpu_store_word(
        word: u32,
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        debug_assert!(word == RI_SELECT_X105_ENABLE_TX_RX_WORD);
        Self {
            value: word,
            source: MachineRiSelectSource::CpuStoreWord {
                instruction_pc,
                source_gpr,
                source_lineage,
            },
        }
    }

    pub const fn value(self) -> u32 {
        self.value
    }

    pub const fn source(self) -> MachineRiSelectSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRiConfigSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    },
}

impl MachineRiConfigSource {
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
pub struct MachineRiConfigState {
    current_control_input: u8,
    current_control_enable: bool,
    source: MachineRiConfigSource,
}

impl MachineRiConfigState {
    pub(crate) const fn from_cpu_store_word(
        word: u32,
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        debug_assert!(word & !RI_CONFIG_DEFINED_FIELDS_MASK == 0);
        Self {
            current_control_input: (word & RI_CONFIG_CURRENT_CONTROL_INPUT_MASK) as u8,
            current_control_enable: word & RI_CONFIG_CURRENT_CONTROL_ENABLE_MASK != 0,
            source: MachineRiConfigSource::CpuStoreWord {
                instruction_pc,
                source_gpr,
                source_lineage,
            },
        }
    }

    pub const fn current_control_input(self) -> u8 {
        self.current_control_input
    }

    pub const fn current_control_enable(self) -> bool {
        self.current_control_enable
    }

    pub const fn source(self) -> MachineRiConfigSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRiCurrentLoadSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    },
}

impl MachineRiCurrentLoadSource {
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
pub struct MachineRiCurrentLoadState {
    config_current_control_input: u8,
    config_current_control_enable: bool,
    transfer_word: u32,
    source: MachineRiCurrentLoadSource,
}

impl MachineRiCurrentLoadState {
    pub(crate) const fn from_cpu_store_word(
        config: MachineRiConfigState,
        transfer_word: u32,
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        Self {
            config_current_control_input: config.current_control_input(),
            config_current_control_enable: config.current_control_enable(),
            transfer_word,
            source: MachineRiCurrentLoadSource::CpuStoreWord {
                instruction_pc,
                source_gpr,
                source_lineage,
            },
        }
    }

    pub const fn config_current_control_input(self) -> u8 {
        self.config_current_control_input
    }

    pub const fn config_current_control_enable(self) -> bool {
        self.config_current_control_enable
    }

    pub const fn transfer_word(self) -> u32 {
        self.transfer_word
    }

    pub const fn source(self) -> MachineRiCurrentLoadSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRiModeSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    },
}

impl MachineRiModeSource {
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
pub struct MachineRiModeState {
    operating_mode_bits: u8,
    stop_transmit_active: bool,
    stop_receive_active: bool,
    source: MachineRiModeSource,
}

impl MachineRiModeState {
    pub(crate) const fn from_cpu_store_word(
        word: u32,
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        debug_assert!(word & !RI_MODE_DEFINED_FIELDS_MASK == 0);
        Self {
            operating_mode_bits: (word & RI_MODE_OPERATING_MODE_MASK) as u8,
            stop_transmit_active: word & RI_MODE_STOP_TRANSMIT_ACTIVE_MASK != 0,
            stop_receive_active: word & RI_MODE_STOP_RECEIVE_ACTIVE_MASK != 0,
            source: MachineRiModeSource::CpuStoreWord {
                instruction_pc,
                source_gpr,
                source_lineage,
            },
        }
    }

    pub const fn operating_mode_bits(self) -> u8 {
        self.operating_mode_bits
    }

    pub const fn stop_transmit_active(self) -> bool {
        self.stop_transmit_active
    }

    pub const fn stop_receive_active(self) -> bool {
        self.stop_receive_active
    }

    pub const fn source(self) -> MachineRiModeSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Ri {
    select: Option<MachineRiSelectState>,
    config: Option<MachineRiConfigState>,
    current_load: Option<MachineRiCurrentLoadState>,
    mode: Option<MachineRiModeState>,
}

impl Ri {
    pub(crate) const fn cold_x105_entry() -> Self {
        Self {
            select: Some(MachineRiSelectState::cold_x105_entry()),
            config: None,
            current_load: None,
            mode: None,
        }
    }

    pub(crate) const fn select_state(self) -> Option<MachineRiSelectState> {
        self.select
    }

    pub(crate) const fn config_state(self) -> Option<MachineRiConfigState> {
        self.config
    }

    pub(crate) const fn current_load_state(self) -> Option<MachineRiCurrentLoadState> {
        self.current_load
    }

    pub(crate) const fn mode_state(self) -> Option<MachineRiModeState> {
        self.mode
    }

    pub(crate) fn apply_config_store(&mut self, state: MachineRiConfigState) {
        self.config = Some(state);
    }

    pub(crate) fn apply_current_load_store(&mut self, state: MachineRiCurrentLoadState) {
        self.current_load = Some(state);
    }

    pub(crate) fn apply_select_store(&mut self, state: MachineRiSelectState) {
        self.select = Some(state);
    }

    pub(crate) fn apply_mode_store(&mut self, state: MachineRiModeState) {
        self.mode = Some(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ri_select_has_one_exact_cold_entry_creation_fact() {
        assert_eq!(RI_SELECT_PHYSICAL_ADDRESS, 0x0470_000c);
        assert_eq!(Ri::default().select_state(), None);
        assert_eq!(Ri::default().config_state(), None);
        assert_eq!(Ri::default().current_load_state(), None);
        assert_eq!(Ri::default().mode_state(), None);
        assert_eq!(
            Ri::cold_x105_entry().select_state(),
            Some(MachineRiSelectState {
                value: 0,
                source: MachineRiSelectSource::ColdX105Entry,
            })
        );
        assert_eq!(Ri::cold_x105_entry().config_state(), None);
        assert_eq!(Ri::cold_x105_entry().current_load_state(), None);
        assert_eq!(Ri::cold_x105_entry().mode_state(), None);
    }

    #[test]
    fn ri_select_exact_x105_cpu_store_replaces_value_and_source() {
        assert_eq!(RI_SELECT_X105_ENABLE_TX_RX_WORD, 0x14);
        let lineage = MachineBootstrapGprSource::KnownInstructionResult {
            execution_address: CpuAddress::new(0xa400_00e0),
            identity: crate::cpu::CpuInstructionIdentity::Ori,
            source_gpr_a: Some(0),
            source_gpr_b: None,
        };
        let state = MachineRiSelectState::from_cpu_store_word(
            RI_SELECT_X105_ENABLE_TX_RX_WORD,
            CpuAddress::new(0xa400_00e4),
            9,
            lineage,
        );

        assert_eq!(state.value(), 0x14);
        assert_eq!(
            state.source(),
            MachineRiSelectSource::CpuStoreWord {
                instruction_pc: CpuAddress::new(0xa400_00e4),
                source_gpr: 9,
                source_lineage: lineage,
            }
        );
        assert_eq!(
            state.source().instruction_pc(),
            Some(CpuAddress::new(0xa400_00e4))
        );
        assert_eq!(state.source().source_gpr(), Some(9));
        assert_eq!(state.source().source_lineage(), Some(lineage));

        let mut ri = Ri::cold_x105_entry();
        ri.apply_select_store(state);
        assert_eq!(ri.select_state(), Some(state));
        assert_eq!(ri.config_state(), None);
        assert_eq!(ri.current_load_state(), None);
        assert_eq!(ri.mode_state(), None);
    }

    #[test]
    fn ri_config_represents_only_defined_fields_and_cpu_store_lineage() {
        assert_eq!(RI_CONFIG_PHYSICAL_ADDRESS, 0x0470_0004);
        assert_eq!(RI_CONFIG_DEFINED_FIELDS_MASK, 0x7f);
        let source_lineage = MachineBootstrapGprSource::ArchitecturalZero;

        for (word, input, enable) in [
            (0x00, 0x00, false),
            (0x3f, 0x3f, false),
            (0x40, 0x00, true),
            (0x7f, 0x3f, true),
        ] {
            let state = MachineRiConfigState::from_cpu_store_word(
                word,
                CpuAddress::new(0xa400_0040),
                0,
                source_lineage,
            );
            assert_eq!(state.current_control_input(), input);
            assert_eq!(state.current_control_enable(), enable);
            assert_eq!(
                state.source().instruction_pc(),
                CpuAddress::new(0xa400_0040)
            );
            assert_eq!(state.source().source_gpr(), 0);
            assert_eq!(state.source().source_lineage(), source_lineage);
        }
    }

    #[test]
    fn ri_current_load_records_config_snapshot_and_cpu_store_evidence_only() {
        assert_eq!(RI_CURRENT_LOAD_PHYSICAL_ADDRESS, 0x0470_0008);
        let lineage = MachineBootstrapGprSource::X105Seed;
        let config = MachineRiConfigState::from_cpu_store_word(
            0x40,
            CpuAddress::new(0xa400_00c4),
            9,
            MachineBootstrapGprSource::KnownInstructionResult {
                execution_address: CpuAddress::new(0xa400_00c0),
                identity: crate::cpu::CpuInstructionIdentity::Ori,
                source_gpr_a: Some(0),
                source_gpr_b: None,
            },
        );
        let state = MachineRiCurrentLoadState::from_cpu_store_word(
            config,
            0x89ab_cdef,
            CpuAddress::new(0xa400_00dc),
            22,
            lineage,
        );

        assert_eq!(state.config_current_control_input(), 0);
        assert!(state.config_current_control_enable());
        assert_eq!(state.transfer_word(), 0x89ab_cdef);
        assert_eq!(
            state.source().instruction_pc(),
            CpuAddress::new(0xa400_00dc)
        );
        assert_eq!(state.source().source_gpr(), 22);
        assert_eq!(state.source().source_lineage(), lineage);
    }

    #[test]
    fn ri_mode_represents_all_defined_fields_and_cpu_store_lineage() {
        assert_eq!(RI_MODE_PHYSICAL_ADDRESS, 0x0470_0000);
        assert_eq!(RI_MODE_DEFINED_FIELDS_MASK, 0x0f);
        let source_lineage = MachineBootstrapGprSource::ArchitecturalZero;

        for (word, operating_mode_bits, stop_transmit, stop_receive) in [
            (0x00, 0, false, false),
            (0x02, 2, false, false),
            (0x04, 0, true, false),
            (0x08, 0, false, true),
            (0x0e, 2, true, true),
            (0x0f, 3, true, true),
        ] {
            let state = MachineRiModeState::from_cpu_store_word(
                word,
                CpuAddress::new(0xa400_00e8),
                0,
                source_lineage,
            );
            assert_eq!(state.operating_mode_bits(), operating_mode_bits);
            assert_eq!(state.stop_transmit_active(), stop_transmit);
            assert_eq!(state.stop_receive_active(), stop_receive);
            assert_eq!(
                state.source().instruction_pc(),
                CpuAddress::new(0xa400_00e8)
            );
            assert_eq!(state.source().source_gpr(), 0);
            assert_eq!(state.source().source_lineage(), source_lineage);
        }
    }
}
