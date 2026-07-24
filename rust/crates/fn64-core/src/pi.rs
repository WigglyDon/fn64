use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const PI_DRAM_ADDR_PHYSICAL_ADDRESS: u32 = 0x0460_0000;
pub const PI_CART_ADDR_PHYSICAL_ADDRESS: u32 = 0x0460_0004;
pub const PI_RD_LEN_PHYSICAL_ADDRESS: u32 = 0x0460_0008;
pub const PI_WR_LEN_PHYSICAL_ADDRESS: u32 = 0x0460_000c;
pub const PI_STATUS_PHYSICAL_ADDRESS: u32 = 0x0460_0010;
pub const PI_BSD_DOM1_LAT_PHYSICAL_ADDRESS: u32 = 0x0460_0014;
pub const PI_BSD_DOM1_PWD_PHYSICAL_ADDRESS: u32 = 0x0460_0018;
pub const PI_BSD_DOM1_PGS_PHYSICAL_ADDRESS: u32 = 0x0460_001c;
pub const PI_BSD_DOM1_RLS_PHYSICAL_ADDRESS: u32 = 0x0460_0020;
pub const PI_BSD_DOM2_LAT_PHYSICAL_ADDRESS: u32 = 0x0460_0024;
pub const PI_BSD_DOM2_PWD_PHYSICAL_ADDRESS: u32 = 0x0460_0028;
pub const PI_BSD_DOM2_PGS_PHYSICAL_ADDRESS: u32 = 0x0460_002c;
pub const PI_BSD_DOM2_RLS_PHYSICAL_ADDRESS: u32 = 0x0460_0030;
pub const PI_X105_WR_LEN_WORD: u32 = 0x000f_ffff;
pub const PI_X105_DMA_BYTE_COUNT: u32 = PI_X105_WR_LEN_WORD + 1;
pub const PI_DMA_LENGTH_DEFINED_MASK: u32 = 0x00ff_ffff;
pub const PI_STATUS_CLEAR_INTERRUPT_WORD: u32 = 0x0000_0002;
pub const PI_DOMAIN_ONE_ADDRESS_ONE_BASE: u32 = 0x0600_0000;
pub const PI_DOMAIN_ONE_ADDRESS_ONE_END_EXCLUSIVE: u32 = 0x0800_0000;
pub const PI_ABSENT_BULK_DEVICE_READ_WORD: u32 = 0xffff_ffff;
pub const PI_DOMAIN_ONE_ADDRESS_TWO_BASE: u32 = 0x1000_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MachinePiDomainOneAddressOneProfile {
    #[default]
    NoBulkDevice,
}

impl MachinePiDomainOneAddressOneProfile {
    pub const fn direct_read_word(self) -> u32 {
        match self {
            Self::NoBulkDevice => PI_ABSENT_BULK_DEVICE_READ_WORD,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePiDomain {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePiDomainTimingField {
    Latency,
    PulseWidth,
    PageSize,
    Release,
}

impl MachinePiDomainTimingField {
    pub const fn defined_mask(self) -> u32 {
        match self {
            Self::Latency | Self::PulseWidth => 0x0000_00ff,
            Self::PageSize => 0x0000_000f,
            Self::Release => 0x0000_0003,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePiDomainTimingRegister {
    domain: MachinePiDomain,
    field: MachinePiDomainTimingField,
}

impl MachinePiDomainTimingRegister {
    pub const fn new(domain: MachinePiDomain, field: MachinePiDomainTimingField) -> Self {
        Self { domain, field }
    }

    pub const fn domain(self) -> MachinePiDomain {
        self.domain
    }

    pub const fn field(self) -> MachinePiDomainTimingField {
        self.field
    }

    pub const fn physical_address(self) -> u32 {
        match (self.domain, self.field) {
            (MachinePiDomain::One, MachinePiDomainTimingField::Latency) => {
                PI_BSD_DOM1_LAT_PHYSICAL_ADDRESS
            }
            (MachinePiDomain::One, MachinePiDomainTimingField::PulseWidth) => {
                PI_BSD_DOM1_PWD_PHYSICAL_ADDRESS
            }
            (MachinePiDomain::One, MachinePiDomainTimingField::PageSize) => {
                PI_BSD_DOM1_PGS_PHYSICAL_ADDRESS
            }
            (MachinePiDomain::One, MachinePiDomainTimingField::Release) => {
                PI_BSD_DOM1_RLS_PHYSICAL_ADDRESS
            }
            (MachinePiDomain::Two, MachinePiDomainTimingField::Latency) => {
                PI_BSD_DOM2_LAT_PHYSICAL_ADDRESS
            }
            (MachinePiDomain::Two, MachinePiDomainTimingField::PulseWidth) => {
                PI_BSD_DOM2_PWD_PHYSICAL_ADDRESS
            }
            (MachinePiDomain::Two, MachinePiDomainTimingField::PageSize) => {
                PI_BSD_DOM2_PGS_PHYSICAL_ADDRESS
            }
            (MachinePiDomain::Two, MachinePiDomainTimingField::Release) => {
                PI_BSD_DOM2_RLS_PHYSICAL_ADDRESS
            }
        }
    }
}

pub const fn pi_domain_timing_register(
    physical_address: u32,
) -> Option<MachinePiDomainTimingRegister> {
    match physical_address {
        PI_BSD_DOM1_LAT_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::One,
            MachinePiDomainTimingField::Latency,
        )),
        PI_BSD_DOM1_PWD_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::One,
            MachinePiDomainTimingField::PulseWidth,
        )),
        PI_BSD_DOM1_PGS_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::One,
            MachinePiDomainTimingField::PageSize,
        )),
        PI_BSD_DOM1_RLS_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::One,
            MachinePiDomainTimingField::Release,
        )),
        PI_BSD_DOM2_LAT_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::Two,
            MachinePiDomainTimingField::Latency,
        )),
        PI_BSD_DOM2_PWD_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::Two,
            MachinePiDomainTimingField::PulseWidth,
        )),
        PI_BSD_DOM2_PGS_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::Two,
            MachinePiDomainTimingField::PageSize,
        )),
        PI_BSD_DOM2_RLS_PHYSICAL_ADDRESS => Some(MachinePiDomainTimingRegister::new(
            MachinePiDomain::Two,
            MachinePiDomainTimingField::Release,
        )),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePiCpuStoreProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachinePiCpuStoreProvenance {
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
pub enum MachinePiDomainTimingSource {
    ColdCartridgeHeaderConfiguration,
    ColdMachineResetZero,
    CpuStore(MachinePiCpuStoreProvenance),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePiDomainTimingRegisterState {
    register: MachinePiDomainTimingRegister,
    raw_word: u32,
    source: MachinePiDomainTimingSource,
}

impl MachinePiDomainTimingRegisterState {
    pub(crate) const fn new(
        register: MachinePiDomainTimingRegister,
        transfer_word: u32,
        source: MachinePiDomainTimingSource,
    ) -> Self {
        Self {
            register,
            raw_word: transfer_word & register.field().defined_mask(),
            source,
        }
    }

    pub const fn register(self) -> MachinePiDomainTimingRegister {
        self.register
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn source(self) -> MachinePiDomainTimingSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MachinePiDomainTimingState {
    latency: Option<MachinePiDomainTimingRegisterState>,
    pulse_width: Option<MachinePiDomainTimingRegisterState>,
    page_size: Option<MachinePiDomainTimingRegisterState>,
    release: Option<MachinePiDomainTimingRegisterState>,
}

impl MachinePiDomainTimingState {
    const fn unavailable() -> Self {
        Self {
            latency: None,
            pulse_width: None,
            page_size: None,
            release: None,
        }
    }

    const fn cold_domain_one(header_configuration_word: u32) -> Self {
        let source = MachinePiDomainTimingSource::ColdCartridgeHeaderConfiguration;
        Self {
            latency: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::One,
                    MachinePiDomainTimingField::Latency,
                ),
                header_configuration_word >> 24,
                source,
            )),
            pulse_width: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::One,
                    MachinePiDomainTimingField::PulseWidth,
                ),
                header_configuration_word >> 8,
                source,
            )),
            page_size: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::One,
                    MachinePiDomainTimingField::PageSize,
                ),
                header_configuration_word >> 16,
                source,
            )),
            release: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::One,
                    MachinePiDomainTimingField::Release,
                ),
                header_configuration_word,
                source,
            )),
        }
    }

    const fn cold_domain_two_zero() -> Self {
        let source = MachinePiDomainTimingSource::ColdMachineResetZero;
        Self {
            latency: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::Two,
                    MachinePiDomainTimingField::Latency,
                ),
                0,
                source,
            )),
            pulse_width: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::Two,
                    MachinePiDomainTimingField::PulseWidth,
                ),
                0,
                source,
            )),
            page_size: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::Two,
                    MachinePiDomainTimingField::PageSize,
                ),
                0,
                source,
            )),
            release: Some(MachinePiDomainTimingRegisterState::new(
                MachinePiDomainTimingRegister::new(
                    MachinePiDomain::Two,
                    MachinePiDomainTimingField::Release,
                ),
                0,
                source,
            )),
        }
    }

    const fn register_state(
        self,
        field: MachinePiDomainTimingField,
    ) -> Option<MachinePiDomainTimingRegisterState> {
        match field {
            MachinePiDomainTimingField::Latency => self.latency,
            MachinePiDomainTimingField::PulseWidth => self.pulse_width,
            MachinePiDomainTimingField::PageSize => self.page_size,
            MachinePiDomainTimingField::Release => self.release,
        }
    }

    fn apply(&mut self, state: MachinePiDomainTimingRegisterState) {
        match state.register().field() {
            MachinePiDomainTimingField::Latency => self.latency = Some(state),
            MachinePiDomainTimingField::PulseWidth => self.pulse_width = Some(state),
            MachinePiDomainTimingField::PageSize => self.page_size = Some(state),
            MachinePiDomainTimingField::Release => self.release = Some(state),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePiProgrammedRegisterState {
    raw_word: u32,
    source: MachinePiCpuStoreProvenance,
}

impl MachinePiProgrammedRegisterState {
    pub(crate) const fn new(raw_word: u32, source: MachinePiCpuStoreProvenance) -> Self {
        Self { raw_word, source }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }
    pub const fn source(self) -> MachinePiCpuStoreProvenance {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePiDmaDirection {
    CartridgeToRdram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePiDmaCompletion {
    AtomicFunctional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePiCompletedDmaState {
    trigger_instruction_pc: CpuAddress,
    programmed_dram_address: MachinePiProgrammedRegisterState,
    programmed_cartridge_address: MachinePiProgrammedRegisterState,
    programmed_write_length: MachinePiProgrammedRegisterState,
    cartridge_bus_address: u32,
    cartridge_byte_offset: u32,
    rdram_physical_address: u32,
    byte_count: u32,
    direction: MachinePiDmaDirection,
    completion: MachinePiDmaCompletion,
}

impl MachinePiCompletedDmaState {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        trigger_instruction_pc: CpuAddress,
        programmed_dram_address: MachinePiProgrammedRegisterState,
        programmed_cartridge_address: MachinePiProgrammedRegisterState,
        programmed_write_length: MachinePiProgrammedRegisterState,
        cartridge_bus_address: u32,
        cartridge_byte_offset: u32,
        rdram_physical_address: u32,
        byte_count: u32,
    ) -> Self {
        Self {
            trigger_instruction_pc,
            programmed_dram_address,
            programmed_cartridge_address,
            programmed_write_length,
            cartridge_bus_address,
            cartridge_byte_offset,
            rdram_physical_address,
            byte_count,
            direction: MachinePiDmaDirection::CartridgeToRdram,
            completion: MachinePiDmaCompletion::AtomicFunctional,
        }
    }

    pub const fn trigger_instruction_pc(self) -> CpuAddress {
        self.trigger_instruction_pc
    }
    pub const fn programmed_dram_address(self) -> MachinePiProgrammedRegisterState {
        self.programmed_dram_address
    }
    pub const fn programmed_cartridge_address(self) -> MachinePiProgrammedRegisterState {
        self.programmed_cartridge_address
    }
    pub const fn programmed_write_length(self) -> MachinePiProgrammedRegisterState {
        self.programmed_write_length
    }
    pub const fn cartridge_bus_address(self) -> u32 {
        self.cartridge_bus_address
    }
    pub const fn cartridge_byte_offset(self) -> u32 {
        self.cartridge_byte_offset
    }
    pub const fn rdram_physical_address(self) -> u32 {
        self.rdram_physical_address
    }
    pub const fn byte_count(self) -> u32 {
        self.byte_count
    }
    pub const fn direction(self) -> MachinePiDmaDirection {
        self.direction
    }
    pub const fn completion(self) -> MachinePiDmaCompletion {
        self.completion
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachinePiStatusClearState {
    command_word: u32,
    source: MachinePiCpuStoreProvenance,
}

impl MachinePiStatusClearState {
    pub(crate) const fn new(source: MachinePiCpuStoreProvenance) -> Self {
        Self {
            command_word: PI_STATUS_CLEAR_INTERRUPT_WORD,
            source,
        }
    }
    pub const fn command_word(self) -> u32 {
        self.command_word
    }
    pub const fn source(self) -> MachinePiCpuStoreProvenance {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Pi {
    dram_address: Option<MachinePiProgrammedRegisterState>,
    cartridge_address: Option<MachinePiProgrammedRegisterState>,
    write_length: Option<MachinePiProgrammedRegisterState>,
    dma_busy: bool,
    io_busy: bool,
    error: bool,
    completed_dma: Option<MachinePiCompletedDmaState>,
    last_status_clear: Option<MachinePiStatusClearState>,
    domain_one_address_one_profile: MachinePiDomainOneAddressOneProfile,
    domain_one_timing: MachinePiDomainTimingState,
    domain_two_timing: MachinePiDomainTimingState,
}

impl Pi {
    pub(crate) const fn cold_cartridge_entry(header_configuration_word: u32) -> Self {
        Self {
            dram_address: None,
            cartridge_address: None,
            write_length: None,
            dma_busy: false,
            io_busy: false,
            error: false,
            completed_dma: None,
            last_status_clear: None,
            domain_one_address_one_profile: MachinePiDomainOneAddressOneProfile::NoBulkDevice,
            domain_one_timing: MachinePiDomainTimingState::cold_domain_one(
                header_configuration_word,
            ),
            domain_two_timing: MachinePiDomainTimingState::cold_domain_two_zero(),
        }
    }

    pub(crate) const fn dram_address_state(self) -> Option<MachinePiProgrammedRegisterState> {
        self.dram_address
    }
    pub(crate) const fn cartridge_address_state(self) -> Option<MachinePiProgrammedRegisterState> {
        self.cartridge_address
    }
    pub(crate) const fn write_length_state(self) -> Option<MachinePiProgrammedRegisterState> {
        self.write_length
    }
    pub(crate) const fn completed_dma_state(self) -> Option<MachinePiCompletedDmaState> {
        self.completed_dma
    }
    pub(crate) const fn last_status_clear_state(self) -> Option<MachinePiStatusClearState> {
        self.last_status_clear
    }
    pub(crate) const fn domain_one_address_one_profile(
        self,
    ) -> MachinePiDomainOneAddressOneProfile {
        self.domain_one_address_one_profile
    }
    pub(crate) const fn domain_timing_register_state(
        self,
        register: MachinePiDomainTimingRegister,
    ) -> Option<MachinePiDomainTimingRegisterState> {
        match register.domain() {
            MachinePiDomain::One => self.domain_one_timing.register_state(register.field()),
            MachinePiDomain::Two => self.domain_two_timing.register_state(register.field()),
        }
    }
    pub(crate) const fn status_word(self) -> u32 {
        (self.dma_busy as u32) | ((self.io_busy as u32) << 1) | ((self.error as u32) << 2)
    }
    pub(crate) fn apply_dram_address(&mut self, state: MachinePiProgrammedRegisterState) {
        self.dram_address = Some(state);
    }
    pub(crate) fn apply_cartridge_address(&mut self, state: MachinePiProgrammedRegisterState) {
        self.cartridge_address = Some(state);
    }
    pub(crate) fn apply_completed_dma(&mut self, state: MachinePiCompletedDmaState) {
        self.write_length = Some(state.programmed_write_length());
        self.completed_dma = Some(state);
        self.dma_busy = false;
        self.io_busy = false;
        self.error = false;
    }
    pub(crate) fn apply_status_clear(&mut self, state: MachinePiStatusClearState) {
        self.last_status_clear = Some(state);
    }
    pub(crate) fn apply_domain_timing_store(&mut self, state: MachinePiDomainTimingRegisterState) {
        match state.register().domain() {
            MachinePiDomain::One => self.domain_one_timing.apply(state),
            MachinePiDomain::Two => self.domain_two_timing.apply(state),
        }
    }
}

impl Default for MachinePiDomainTimingState {
    fn default() -> Self {
        Self::unavailable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(instruction_pc: u32, physical_address: u32) -> MachinePiCpuStoreProvenance {
        MachinePiCpuStoreProvenance::new(
            CpuAddress::new(instruction_pc),
            9,
            MachineBootstrapGprSource::ArchitecturalZero,
            u64::from(0xa000_0000 | physical_address),
            CpuAddress::new(0xa000_0000 | physical_address),
            physical_address,
        )
    }

    #[test]
    fn cold_pi_owner_is_idle_and_has_no_programmed_or_completed_truth() {
        let pi = Pi::default();
        assert_eq!(pi.status_word(), 0);
        assert_eq!(pi.dram_address_state(), None);
        assert_eq!(pi.cartridge_address_state(), None);
        assert_eq!(pi.write_length_state(), None);
        assert_eq!(pi.completed_dma_state(), None);
        assert_eq!(pi.last_status_clear_state(), None);
    }

    #[test]
    fn cold_cartridge_domain_one_fields_and_cpu_defined_bit_masks_are_exact() {
        let mut pi = Pi::cold_cartridge_entry(0x8037_1240);
        let register = |domain, field| MachinePiDomainTimingRegister::new(domain, field);

        for (field, expected) in [
            (MachinePiDomainTimingField::Latency, 0x80),
            (MachinePiDomainTimingField::PulseWidth, 0x12),
            (MachinePiDomainTimingField::PageSize, 0x07),
            (MachinePiDomainTimingField::Release, 0x00),
        ] {
            let state = pi
                .domain_timing_register_state(register(MachinePiDomain::One, field))
                .unwrap();
            assert_eq!(state.raw_word(), expected);
            assert_eq!(
                state.source(),
                MachinePiDomainTimingSource::ColdCartridgeHeaderConfiguration
            );
        }
        for field in [
            MachinePiDomainTimingField::Latency,
            MachinePiDomainTimingField::PulseWidth,
            MachinePiDomainTimingField::PageSize,
            MachinePiDomainTimingField::Release,
        ] {
            let state = pi
                .domain_timing_register_state(register(MachinePiDomain::Two, field))
                .unwrap();
            assert_eq!(state.raw_word(), 0);
            assert_eq!(
                state.source(),
                MachinePiDomainTimingSource::ColdMachineResetZero
            );
        }

        let page_register = register(MachinePiDomain::Two, MachinePiDomainTimingField::PageSize);
        let page_state = MachinePiDomainTimingRegisterState::new(
            page_register,
            0xffff_fff9,
            MachinePiDomainTimingSource::CpuStore(source(
                0x8000_1000,
                page_register.physical_address(),
            )),
        );
        pi.apply_domain_timing_store(page_state);
        assert_eq!(
            pi.domain_timing_register_state(page_register)
                .unwrap()
                .raw_word(),
            9
        );
        assert_eq!(
            pi.domain_timing_register_state(register(
                MachinePiDomain::Two,
                MachinePiDomainTimingField::Latency,
            ))
            .unwrap()
            .raw_word(),
            0
        );
    }

    #[test]
    fn exact_programming_and_atomic_completion_record_one_transfer_without_busy_time() {
        let dram = MachinePiProgrammedRegisterState::new(
            0x0000_1000,
            source(0x8000_001c, PI_DRAM_ADDR_PHYSICAL_ADDRESS),
        );
        let cartridge = MachinePiProgrammedRegisterState::new(
            0x1000_1000,
            source(0x8000_0044, PI_CART_ADDR_PHYSICAL_ADDRESS),
        );
        let length = MachinePiProgrammedRegisterState::new(
            PI_X105_WR_LEN_WORD,
            source(0x8000_0054, PI_WR_LEN_PHYSICAL_ADDRESS),
        );
        let completed = MachinePiCompletedDmaState::new(
            CpuAddress::new(0x8000_0054),
            dram,
            cartridge,
            length,
            0x1000_1000,
            0x0000_1000,
            0x0000_1000,
            PI_X105_DMA_BYTE_COUNT,
        );
        let mut pi = Pi::default();
        pi.apply_dram_address(dram);
        pi.apply_cartridge_address(cartridge);
        pi.apply_completed_dma(completed);

        assert_eq!(pi.dram_address_state(), Some(dram));
        assert_eq!(pi.cartridge_address_state(), Some(cartridge));
        assert_eq!(pi.write_length_state(), Some(length));
        assert_eq!(pi.completed_dma_state(), Some(completed));
        assert_eq!(
            completed.trigger_instruction_pc(),
            CpuAddress::new(0x8000_0054)
        );
        assert_eq!(completed.cartridge_bus_address(), 0x1000_1000);
        assert_eq!(completed.cartridge_byte_offset(), 0x0000_1000);
        assert_eq!(completed.rdram_physical_address(), 0x0000_1000);
        assert_eq!(completed.byte_count(), 0x0010_0000);
        assert_eq!(
            completed.direction(),
            MachinePiDmaDirection::CartridgeToRdram
        );
        assert_eq!(
            completed.completion(),
            MachinePiDmaCompletion::AtomicFunctional
        );
        assert_eq!(pi.status_word(), 0);
    }

    #[test]
    fn status_clear_records_only_the_exact_cpu_request_and_preserves_programming() {
        let dram = MachinePiProgrammedRegisterState::new(
            0x0000_1000,
            source(0x8000_001c, PI_DRAM_ADDR_PHYSICAL_ADDRESS),
        );
        let mut pi = Pi::default();
        pi.apply_dram_address(dram);
        let clear = MachinePiStatusClearState::new(source(0x8000_01d4, PI_STATUS_PHYSICAL_ADDRESS));
        pi.apply_status_clear(clear);

        assert_eq!(clear.command_word(), PI_STATUS_CLEAR_INTERRUPT_WORD);
        assert_eq!(pi.last_status_clear_state(), Some(clear));
        assert_eq!(pi.dram_address_state(), Some(dram));
        assert_eq!(pi.status_word(), 0);
    }
}
