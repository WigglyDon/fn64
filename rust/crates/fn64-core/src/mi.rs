use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const MI_INIT_MODE_PHYSICAL_ADDRESS: u32 = 0x0430_0000;
pub const MI_INIT_MODE_X105_WRITE_WORD: u32 = 0x0000_010f;
pub const MI_INIT_MODE_X105_INIT_LENGTH: u8 = 15;
pub const MI_INIT_MODE_X105_REPEATED_BYTE_COUNT: u8 = 16;
pub const MI_VERSION_PHYSICAL_ADDRESS: u32 = 0x0430_0004;
pub const MI_VERSION_STANDARD_RETAIL_NUS_WORD: u32 = 0x0202_0102;
pub const MI_SET_RDRAM_REGISTER_MODE_WORD: u32 = 0x0000_2000;
pub const MI_CLEAR_RDRAM_REGISTER_MODE_WORD: u32 = 0x0000_1000;
pub const MI_INTR_MASK_PHYSICAL_ADDRESS: u32 = 0x0430_000c;
pub const MI_CLEAR_DP_INTERRUPT_WORD: u32 = 0x0000_0800;
pub const MI_X105_CLEAR_ALL_INTERRUPT_MASKS_WORD: u32 = 0x0000_0555;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineMiCpuStoreProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineMiCpuStoreProvenance {
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
pub enum MachineMiInterruptSource {
    Sp,
    Si,
    Ai,
    Vi,
    Pi,
    Dp,
}

impl MachineMiInterruptSource {
    const fn index(self) -> usize {
        match self {
            Self::Sp => 0,
            Self::Si => 1,
            Self::Ai => 2,
            Self::Vi => 3,
            Self::Pi => 4,
            Self::Dp => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MachineMiInterruptState {
    pending: [bool; 6],
    masks: [bool; 6],
    pending_clear_provenance: [Option<MachineMiCpuStoreProvenance>; 6],
    mask_clear_provenance: Option<MachineMiCpuStoreProvenance>,
}

impl MachineMiInterruptState {
    pub const fn pending(self, source: MachineMiInterruptSource) -> bool {
        self.pending[source.index()]
    }
    pub const fn mask_enabled(self, source: MachineMiInterruptSource) -> bool {
        self.masks[source.index()]
    }
    pub const fn pending_clear_provenance(
        self,
        source: MachineMiInterruptSource,
    ) -> Option<MachineMiCpuStoreProvenance> {
        self.pending_clear_provenance[source.index()]
    }
    pub const fn mask_clear_provenance(self) -> Option<MachineMiCpuStoreProvenance> {
        self.mask_clear_provenance
    }
    fn set_pending(&mut self, source: MachineMiInterruptSource) {
        self.pending[source.index()] = true;
    }
    fn clear_pending(
        &mut self,
        source: MachineMiInterruptSource,
        provenance: MachineMiCpuStoreProvenance,
    ) {
        self.pending[source.index()] = false;
        self.pending_clear_provenance[source.index()] = Some(provenance);
    }
    fn clear_all_masks(&mut self, provenance: MachineMiCpuStoreProvenance) {
        self.masks = [false; 6];
        self.mask_clear_provenance = Some(provenance);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineMiVersionState {
    word: u32,
}

impl MachineMiVersionState {
    const fn standard_retail_nus() -> Self {
        Self {
            word: MI_VERSION_STANDARD_RETAIL_NUS_WORD,
        }
    }

    pub const fn word(self) -> u32 {
        self.word
    }

    pub const fn io_version(self) -> u8 {
        self.word as u8
    }

    pub const fn rac_version(self) -> u8 {
        (self.word >> 8) as u8
    }

    pub const fn rdp_version(self) -> u8 {
        (self.word >> 16) as u8
    }

    pub const fn rsp_version(self) -> u8 {
        (self.word >> 24) as u8
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineMiRdramRegisterModeSource {
    CpuStoreWord {
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    },
}

impl MachineMiRdramRegisterModeSource {
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
pub struct MachineMiRdramRegisterModeState {
    command_word: u32,
    enabled: bool,
    source: MachineMiRdramRegisterModeSource,
}

impl MachineMiRdramRegisterModeState {
    pub(crate) const fn from_cpu_store_word(
        command_word: u32,
        previously_enabled: bool,
        instruction_pc: CpuAddress,
        source_gpr: u8,
        source_lineage: MachineBootstrapGprSource,
    ) -> Self {
        let enabled = match command_word {
            MI_SET_RDRAM_REGISTER_MODE_WORD => true,
            MI_CLEAR_RDRAM_REGISTER_MODE_WORD => false,
            0 => previously_enabled,
            _ => previously_enabled,
        };
        Self {
            command_word,
            enabled,
            source: MachineMiRdramRegisterModeSource::CpuStoreWord {
                instruction_pc,
                source_gpr,
                source_lineage,
            },
        }
    }

    pub const fn command_word(self) -> u32 {
        self.command_word
    }

    pub const fn enabled(self) -> bool {
        self.enabled
    }

    pub const fn source(self) -> MachineMiRdramRegisterModeSource {
        self.source
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Mi {
    version: MachineMiVersionState,
    init_mode: Option<MachineMiInitModeState>,
    init_transfer: Option<MachineMiInitTransferState>,
    rdram_register_mode: Option<MachineMiRdramRegisterModeState>,
    interrupts: MachineMiInterruptState,
}

impl Default for Mi {
    fn default() -> Self {
        Self {
            version: MachineMiVersionState::standard_retail_nus(),
            init_mode: None,
            init_transfer: None,
            rdram_register_mode: None,
            interrupts: MachineMiInterruptState::default(),
        }
    }
}

impl Mi {
    pub(crate) const fn version_state(self) -> MachineMiVersionState {
        self.version
    }

    pub(crate) const fn init_mode_state(self) -> Option<MachineMiInitModeState> {
        self.init_mode
    }

    pub(crate) const fn init_transfer_state(self) -> Option<MachineMiInitTransferState> {
        self.init_transfer
    }

    pub(crate) const fn rdram_register_mode_state(self) -> Option<MachineMiRdramRegisterModeState> {
        self.rdram_register_mode
    }

    pub(crate) const fn rdram_register_mode_enabled(self) -> bool {
        match self.rdram_register_mode {
            Some(state) => state.enabled(),
            None => false,
        }
    }

    pub(crate) const fn interrupt_state(self) -> MachineMiInterruptState {
        self.interrupts
    }

    pub(crate) fn set_pi_pending_from_dma(&mut self) {
        self.interrupts.set_pending(MachineMiInterruptSource::Pi);
    }

    pub(crate) fn clear_pending_interrupt(
        &mut self,
        source: MachineMiInterruptSource,
        provenance: MachineMiCpuStoreProvenance,
    ) {
        self.interrupts.clear_pending(source, provenance);
    }

    pub(crate) fn clear_all_interrupt_masks(&mut self, provenance: MachineMiCpuStoreProvenance) {
        self.interrupts.clear_all_masks(provenance);
    }

    pub(crate) fn apply_init_mode_store(&mut self, state: MachineMiInitModeState) {
        self.init_mode = Some(state);
        self.init_transfer = Some(MachineMiInitTransferState::from_exact_x105_init_mode(state));
    }

    pub(crate) fn consume_init_transfer(&mut self) {
        self.init_mode = None;
        self.init_transfer = None;
    }

    pub(crate) fn apply_rdram_register_mode_store(
        &mut self,
        state: MachineMiRdramRegisterModeState,
    ) {
        self.rdram_register_mode = Some(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CpuInstructionIdentity;

    fn interrupt_source(pc: u32, physical_address: u32) -> MachineMiCpuStoreProvenance {
        MachineMiCpuStoreProvenance::new(
            CpuAddress::new(pc),
            8,
            MachineBootstrapGprSource::ArchitecturalZero,
            u64::from(0xa000_0000 | physical_address),
            CpuAddress::new(0xa000_0000 | physical_address),
            physical_address,
        )
    }

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

    #[test]
    fn mi_version_is_one_fixed_raw_word_with_derived_fields() {
        assert_eq!(MI_VERSION_PHYSICAL_ADDRESS, 0x0430_0004);
        assert_eq!(MI_VERSION_STANDARD_RETAIL_NUS_WORD, 0x0202_0102);

        let state = Mi::default().version_state();
        assert_eq!(state.word(), 0x0202_0102);
        assert_eq!(state.io_version(), 0x02);
        assert_eq!(state.rac_version(), 0x01);
        assert_eq!(state.rdp_version(), 0x02);
        assert_eq!(state.rsp_version(), 0x02);
    }

    #[test]
    fn rdram_register_mode_set_clear_and_zero_are_exact_and_provenanced() {
        let lineage = MachineBootstrapGprSource::KnownInstructionResult {
            execution_address: CpuAddress::new(0xa400_0b34),
            identity: CpuInstructionIdentity::Ori,
            source_gpr_a: Some(0),
            source_gpr_b: None,
        };
        let mut mi = Mi::default();
        assert!(!mi.rdram_register_mode_enabled());

        let set = MachineMiRdramRegisterModeState::from_cpu_store_word(
            MI_SET_RDRAM_REGISTER_MODE_WORD,
            false,
            CpuAddress::new(0xa400_0b38),
            9,
            lineage,
        );
        mi.apply_rdram_register_mode_store(set);
        assert!(mi.rdram_register_mode_enabled());
        assert_eq!(set.command_word(), 0x0000_2000);
        assert_eq!(set.source().instruction_pc(), CpuAddress::new(0xa400_0b38));
        assert_eq!(set.source().source_gpr(), 9);
        assert_eq!(set.source().source_lineage(), lineage);

        let zero = MachineMiRdramRegisterModeState::from_cpu_store_word(
            0,
            mi.rdram_register_mode_enabled(),
            CpuAddress::new(0xa400_0b60),
            0,
            MachineBootstrapGprSource::ArchitecturalZero,
        );
        mi.apply_rdram_register_mode_store(zero);
        assert!(mi.rdram_register_mode_enabled());
        assert_eq!(zero.command_word(), 0);

        let clear = MachineMiRdramRegisterModeState::from_cpu_store_word(
            MI_CLEAR_RDRAM_REGISTER_MODE_WORD,
            mi.rdram_register_mode_enabled(),
            CpuAddress::new(0xa400_0b3c),
            9,
            lineage,
        );
        mi.apply_rdram_register_mode_store(clear);
        assert!(!mi.rdram_register_mode_enabled());
        assert_eq!(clear.command_word(), 0x0000_1000);
    }

    #[test]
    fn interrupt_owner_records_pi_completion_and_exact_generated_clears_without_delivery() {
        let mut mi = Mi::default();
        let cold = mi.interrupt_state();
        for source in [
            MachineMiInterruptSource::Sp,
            MachineMiInterruptSource::Si,
            MachineMiInterruptSource::Ai,
            MachineMiInterruptSource::Vi,
            MachineMiInterruptSource::Pi,
            MachineMiInterruptSource::Dp,
        ] {
            assert!(!cold.pending(source));
            assert!(!cold.mask_enabled(source));
            assert_eq!(cold.pending_clear_provenance(source), None);
        }
        assert_eq!(cold.mask_clear_provenance(), None);

        mi.set_pi_pending_from_dma();
        assert!(mi.interrupt_state().pending(MachineMiInterruptSource::Pi));
        let pi_clear = interrupt_source(0x8000_01d4, 0x0460_0010);
        mi.clear_pending_interrupt(MachineMiInterruptSource::Pi, pi_clear);
        assert!(!mi.interrupt_state().pending(MachineMiInterruptSource::Pi));
        assert_eq!(
            mi.interrupt_state()
                .pending_clear_provenance(MachineMiInterruptSource::Pi),
            Some(pi_clear)
        );

        let mask_clear = interrupt_source(0x8000_01ac, MI_INTR_MASK_PHYSICAL_ADDRESS);
        mi.clear_all_interrupt_masks(mask_clear);
        assert_eq!(
            mi.interrupt_state().mask_clear_provenance(),
            Some(mask_clear)
        );
        for source in [
            MachineMiInterruptSource::Sp,
            MachineMiInterruptSource::Si,
            MachineMiInterruptSource::Ai,
            MachineMiInterruptSource::Vi,
            MachineMiInterruptSource::Pi,
            MachineMiInterruptSource::Dp,
        ] {
            assert!(!mi.interrupt_state().mask_enabled(source));
        }
    }
}
