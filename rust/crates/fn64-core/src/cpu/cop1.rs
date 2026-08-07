use super::address::CpuAddress;
use super::Cpu;
use crate::machine::{MachineBootstrapGprSource, MachineLoadWordTarget};

pub const COP1_FCR31_DEFINED_FIELDS_MASK: u32 = 0x0183_ffff;
pub const COP1_FGR_COUNT: usize = 32;
const COP0_STATUS_FR: u32 = 0x0400_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1FrMode {
    Fr0,
    Fr1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1DataWordAvailability {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1Lwc1Provenance {
    instruction_pc: CpuAddress,
    memory_target: MachineLoadWordTarget,
    unavailable_source: Option<MachineBootstrapGprSource>,
}

impl MachineCop1Lwc1Provenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        memory_target: MachineLoadWordTarget,
        unavailable_source: Option<MachineBootstrapGprSource>,
    ) -> Self {
        Self {
            instruction_pc,
            memory_target,
            unavailable_source,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn memory_target(self) -> MachineLoadWordTarget {
        self.memory_target
    }

    pub const fn unavailable_source(self) -> Option<MachineBootstrapGprSource> {
        self.unavailable_source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1Ldc1WordRole {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1Ldc1Provenance {
    instruction_pc: CpuAddress,
    memory_target: MachineLoadWordTarget,
    word_role: MachineCop1Ldc1WordRole,
}

impl MachineCop1Ldc1Provenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        memory_target: MachineLoadWordTarget,
        word_role: MachineCop1Ldc1WordRole,
    ) -> Self {
        Self {
            instruction_pc,
            memory_target,
            word_role,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn memory_target(self) -> MachineLoadWordTarget {
        self.memory_target
    }

    pub const fn word_role(self) -> MachineCop1Ldc1WordRole {
        self.word_role
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1DataWordSource {
    ConstructionUnavailable,
    Lwc1(MachineCop1Lwc1Provenance),
    Ldc1(MachineCop1Ldc1Provenance),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1DataWordState {
    raw_word: u32,
    availability: MachineCop1DataWordAvailability,
    source: MachineCop1DataWordSource,
}

impl MachineCop1DataWordState {
    pub const fn construction_unavailable() -> Self {
        Self {
            raw_word: 0,
            availability: MachineCop1DataWordAvailability::Unavailable,
            source: MachineCop1DataWordSource::ConstructionUnavailable,
        }
    }

    pub(crate) const fn from_lwc1_available(
        raw_word: u32,
        provenance: MachineCop1Lwc1Provenance,
    ) -> Self {
        Self {
            raw_word,
            availability: MachineCop1DataWordAvailability::Available,
            source: MachineCop1DataWordSource::Lwc1(provenance),
        }
    }

    pub(crate) const fn from_lwc1_unavailable(provenance: MachineCop1Lwc1Provenance) -> Self {
        Self {
            raw_word: 0,
            availability: MachineCop1DataWordAvailability::Unavailable,
            source: MachineCop1DataWordSource::Lwc1(provenance),
        }
    }

    pub(crate) const fn from_ldc1_available(
        raw_word: u32,
        provenance: MachineCop1Ldc1Provenance,
    ) -> Self {
        Self {
            raw_word,
            availability: MachineCop1DataWordAvailability::Available,
            source: MachineCop1DataWordSource::Ldc1(provenance),
        }
    }

    pub const fn availability(self) -> MachineCop1DataWordAvailability {
        self.availability
    }

    pub const fn raw_word(self) -> Option<u32> {
        match self.availability {
            MachineCop1DataWordAvailability::Available => Some(self.raw_word),
            MachineCop1DataWordAvailability::Unavailable => None,
        }
    }

    pub const fn source(self) -> MachineCop1DataWordSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1DataWordSummary {
    available_word_count: u8,
    unavailable_word_count: u8,
    construction_unavailable_word_count: u8,
    lwc1_word_count: u8,
    ldc1_word_count: u8,
}

impl MachineCop1DataWordSummary {
    pub const fn available_word_count(self) -> u8 {
        self.available_word_count
    }

    pub const fn unavailable_word_count(self) -> u8 {
        self.unavailable_word_count
    }

    pub const fn construction_unavailable_word_count(self) -> u8 {
        self.construction_unavailable_word_count
    }

    pub const fn lwc1_word_count(self) -> u8 {
        self.lwc1_word_count
    }

    pub const fn ldc1_word_count(self) -> u8 {
        self.ldc1_word_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1Fcr31WriteProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
}

impl MachineCop1Fcr31WriteProvenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        Self {
            instruction_pc,
            source_gpr,
            source_lineage,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1Fcr31Source {
    PublicSyntheticColdX105Bootstrap,
    CleanRoomHleNtscX105Pinned,
    CpuControlTransfer(MachineCop1Fcr31WriteProvenance),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1Fcr31State {
    raw_word: u32,
    source: MachineCop1Fcr31Source,
}

impl MachineCop1Fcr31State {
    pub(crate) const fn public_synthetic_cold_x105() -> Self {
        Self {
            raw_word: 0,
            source: MachineCop1Fcr31Source::PublicSyntheticColdX105Bootstrap,
        }
    }

    pub(crate) const fn clean_room_hle_ntsc_x105_pinned() -> Self {
        Self {
            raw_word: 0,
            source: MachineCop1Fcr31Source::CleanRoomHleNtscX105Pinned,
        }
    }

    pub(crate) const fn from_cpu_control_transfer(
        transfer_word: u32,
        provenance: MachineCop1Fcr31WriteProvenance,
    ) -> Self {
        Self {
            raw_word: transfer_word & COP1_FCR31_DEFINED_FIELDS_MASK,
            source: MachineCop1Fcr31Source::CpuControlTransfer(provenance),
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn source(self) -> MachineCop1Fcr31Source {
        self.source
    }
}

pub(super) struct Cop1 {
    fcr31: Option<MachineCop1Fcr31State>,
    data_words: [MachineCop1DataWordState; COP1_FGR_COUNT],
}

impl Cop1 {
    pub(super) const fn new() -> Self {
        Self {
            fcr31: None,
            data_words: [MachineCop1DataWordState::construction_unavailable(); COP1_FGR_COUNT],
        }
    }
}

impl Cpu {
    pub fn cop1_fr_mode(&self) -> MachineCop1FrMode {
        if (self.cop0_status() & COP0_STATUS_FR) == 0 {
            MachineCop1FrMode::Fr0
        } else {
            MachineCop1FrMode::Fr1
        }
    }

    pub fn cop1_data_word_state(&self, selector: usize) -> Option<MachineCop1DataWordState> {
        self.cop1.data_words.get(selector).copied()
    }

    pub fn cop1_data_word_summary(&self) -> MachineCop1DataWordSummary {
        let mut available_word_count = 0_u8;
        let mut construction_unavailable_word_count = 0_u8;
        let mut lwc1_word_count = 0_u8;
        let mut ldc1_word_count = 0_u8;
        for state in self.cop1.data_words {
            match state.availability() {
                MachineCop1DataWordAvailability::Available => {
                    available_word_count = available_word_count.saturating_add(1);
                }
                MachineCop1DataWordAvailability::Unavailable => {}
            }
            match state.source() {
                MachineCop1DataWordSource::ConstructionUnavailable => {
                    construction_unavailable_word_count =
                        construction_unavailable_word_count.saturating_add(1);
                }
                MachineCop1DataWordSource::Lwc1(_) => {
                    lwc1_word_count = lwc1_word_count.saturating_add(1);
                }
                MachineCop1DataWordSource::Ldc1(_) => {
                    ldc1_word_count = ldc1_word_count.saturating_add(1);
                }
            }
        }
        MachineCop1DataWordSummary {
            available_word_count,
            unavailable_word_count: (COP1_FGR_COUNT as u8).saturating_sub(available_word_count),
            construction_unavailable_word_count,
            lwc1_word_count,
            ldc1_word_count,
        }
    }

    pub fn cop1_fcr31_state(&self) -> Option<MachineCop1Fcr31State> {
        self.cop1.fcr31
    }

    pub(crate) fn stage_public_synthetic_cold_x105_fcr31(&mut self) {
        self.cop1.fcr31 = Some(MachineCop1Fcr31State::public_synthetic_cold_x105());
    }

    pub(crate) fn stage_clean_room_hle_fcr31(&mut self) {
        self.cop1.fcr31 = Some(MachineCop1Fcr31State::clean_room_hle_ntsc_x105_pinned());
    }

    pub(crate) fn write_cop1_fcr31(&mut self, state: MachineCop1Fcr31State) {
        self.cop1.fcr31 = Some(state);
    }

    pub(crate) fn write_cop1_data_word(&mut self, selector: u8, state: MachineCop1DataWordState) {
        self.cop1.data_words[usize::from(selector)] = state;
    }
}
