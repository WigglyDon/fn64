use super::address::CpuAddress;
use super::Cpu;
use crate::machine::MachineBootstrapGprSource;

pub const COP1_FCR31_DEFINED_FIELDS_MASK: u32 = 0x0183_ffff;

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
}

impl Cop1 {
    pub(super) const fn new() -> Self {
        Self { fcr31: None }
    }
}

impl Cpu {
    pub fn cop1_fcr31_state(&self) -> Option<MachineCop1Fcr31State> {
        self.cop1.fcr31
    }

    pub(crate) fn stage_public_synthetic_cold_x105_fcr31(&mut self) {
        self.cop1.fcr31 = Some(MachineCop1Fcr31State::public_synthetic_cold_x105());
    }

    pub(crate) fn write_cop1_fcr31(&mut self, state: MachineCop1Fcr31State) {
        self.cop1.fcr31 = Some(state);
    }
}
