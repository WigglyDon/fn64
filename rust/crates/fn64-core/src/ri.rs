use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const RI_CONFIG_PHYSICAL_ADDRESS: u32 = 0x0470_0004;
pub const RI_CONFIG_CURRENT_CONTROL_INPUT_MASK: u32 = 0x0000_003f;
pub const RI_CONFIG_CURRENT_CONTROL_ENABLE_MASK: u32 = 0x0000_0040;
pub const RI_CONFIG_DEFINED_FIELDS_MASK: u32 =
    RI_CONFIG_CURRENT_CONTROL_INPUT_MASK | RI_CONFIG_CURRENT_CONTROL_ENABLE_MASK;
pub const RI_SELECT_PHYSICAL_ADDRESS: u32 = 0x0470_000c;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineRiSelectSource {
    ColdX105Entry,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Ri {
    select: Option<MachineRiSelectState>,
    config: Option<MachineRiConfigState>,
}

impl Ri {
    pub(crate) const fn cold_x105_entry() -> Self {
        Self {
            select: Some(MachineRiSelectState::cold_x105_entry()),
            config: None,
        }
    }

    pub(crate) const fn select_state(self) -> Option<MachineRiSelectState> {
        self.select
    }

    pub(crate) const fn config_state(self) -> Option<MachineRiConfigState> {
        self.config
    }

    pub(crate) fn apply_config_store(&mut self, state: MachineRiConfigState) {
        self.config = Some(state);
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
        assert_eq!(
            Ri::cold_x105_entry().select_state(),
            Some(MachineRiSelectState {
                value: 0,
                source: MachineRiSelectSource::ColdX105Entry,
            })
        );
        assert_eq!(Ri::cold_x105_entry().config_state(), None);
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
}
