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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Ri {
    select: Option<MachineRiSelectState>,
}

impl Ri {
    pub(crate) const fn cold_x105_entry() -> Self {
        Self {
            select: Some(MachineRiSelectState::cold_x105_entry()),
        }
    }

    pub(crate) const fn select_state(self) -> Option<MachineRiSelectState> {
        self.select
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ri_select_has_one_exact_cold_entry_creation_fact() {
        assert_eq!(RI_SELECT_PHYSICAL_ADDRESS, 0x0470_000c);
        assert_eq!(Ri::default().select_state(), None);
        assert_eq!(
            Ri::cold_x105_entry().select_state(),
            Some(MachineRiSelectState {
                value: 0,
                source: MachineRiSelectSource::ColdX105Entry,
            })
        );
    }
}
