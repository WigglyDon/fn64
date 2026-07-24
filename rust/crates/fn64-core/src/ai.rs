use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const AI_CONTROL_PHYSICAL_ADDRESS: u32 = 0x0450_0008;
pub const AI_STATUS_PHYSICAL_ADDRESS: u32 = 0x0450_000c;
pub const AI_DAC_RATE_PHYSICAL_ADDRESS: u32 = 0x0450_0010;
pub const AI_BIT_RATE_PHYSICAL_ADDRESS: u32 = 0x0450_0014;
pub const AI_CONTROL_DMA_ENABLE_MASK: u32 = 0x0000_0001;
pub const AI_DAC_RATE_MASK: u32 = 0x0000_3fff;
pub const AI_BIT_RATE_MASK: u32 = 0x0000_000f;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineAiCpuStoreProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineAiCpuStoreProvenance {
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
pub struct MachineAiControlState {
    raw_word: u32,
    provenance: MachineAiCpuStoreProvenance,
}

impl MachineAiControlState {
    pub(crate) const fn new(raw_word: u32, provenance: MachineAiCpuStoreProvenance) -> Self {
        debug_assert!(raw_word & !AI_CONTROL_DMA_ENABLE_MASK == 0);
        Self {
            raw_word,
            provenance,
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn dma_enabled(self) -> bool {
        self.raw_word & AI_CONTROL_DMA_ENABLE_MASK != 0
    }

    pub const fn provenance(self) -> MachineAiCpuStoreProvenance {
        self.provenance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineAiDacRateState {
    raw_word: u32,
    provenance: MachineAiCpuStoreProvenance,
}

impl MachineAiDacRateState {
    pub(crate) const fn new(raw_word: u32, provenance: MachineAiCpuStoreProvenance) -> Self {
        debug_assert!(raw_word & !AI_DAC_RATE_MASK == 0);
        Self {
            raw_word,
            provenance,
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn dac_rate(self) -> u16 {
        (self.raw_word & AI_DAC_RATE_MASK) as u16
    }

    pub const fn provenance(self) -> MachineAiCpuStoreProvenance {
        self.provenance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineAiBitRateState {
    raw_word: u32,
    provenance: MachineAiCpuStoreProvenance,
}

impl MachineAiBitRateState {
    pub(crate) const fn new(raw_word: u32, provenance: MachineAiCpuStoreProvenance) -> Self {
        debug_assert!(raw_word & !AI_BIT_RATE_MASK == 0);
        Self {
            raw_word,
            provenance,
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn bit_rate(self) -> u8 {
        (self.raw_word & AI_BIT_RATE_MASK) as u8
    }

    pub const fn provenance(self) -> MachineAiCpuStoreProvenance {
        self.provenance
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Ai {
    control: Option<MachineAiControlState>,
    dac_rate: Option<MachineAiDacRateState>,
    bit_rate: Option<MachineAiBitRateState>,
}

impl Ai {
    pub(crate) const fn control_state(&self) -> Option<MachineAiControlState> {
        self.control
    }

    pub(crate) const fn dac_rate_state(&self) -> Option<MachineAiDacRateState> {
        self.dac_rate
    }

    pub(crate) const fn bit_rate_state(&self) -> Option<MachineAiBitRateState> {
        self.bit_rate
    }

    pub(crate) fn apply_control(&mut self, state: MachineAiControlState) {
        self.control = Some(state);
    }

    pub(crate) fn apply_dac_rate(&mut self, state: MachineAiDacRateState) {
        self.dac_rate = Some(state);
    }

    pub(crate) fn apply_bit_rate(&mut self, state: MachineAiBitRateState) {
        self.bit_rate = Some(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_keeps_one_raw_word_and_derives_enable() {
        let provenance = MachineAiCpuStoreProvenance::new(
            CpuAddress::new(0x8000_0100),
            9,
            MachineBootstrapGprSource::ArchitecturalZero,
            0xffff_ffff_a450_0008,
            CpuAddress::new(0xa450_0008),
            AI_CONTROL_PHYSICAL_ADDRESS,
        );
        let disabled = MachineAiControlState::new(0, provenance);
        let enabled = MachineAiControlState::new(1, provenance);

        assert_eq!(disabled.raw_word(), 0);
        assert!(!disabled.dma_enabled());
        assert_eq!(enabled.raw_word(), 1);
        assert!(enabled.dma_enabled());
        assert_eq!(enabled.provenance(), provenance);
    }

    #[test]
    fn dac_rate_keeps_one_masked_raw_word_without_timing_effects() {
        let provenance = MachineAiCpuStoreProvenance::new(
            CpuAddress::new(0x8000_0100),
            9,
            MachineBootstrapGprSource::ArchitecturalZero,
            0xffff_ffff_a450_0010,
            CpuAddress::new(0xa450_0010),
            AI_DAC_RATE_PHYSICAL_ADDRESS,
        );
        let state = MachineAiDacRateState::new(AI_DAC_RATE_MASK, provenance);

        assert_eq!(state.raw_word(), 0x3fff);
        assert_eq!(state.dac_rate(), 0x3fff);
        assert_eq!(state.provenance(), provenance);
    }
}
