use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const PIF_RAM_PHYSICAL_START: u32 = 0x1fc0_07c0;
pub const PIF_RAM_SIZE_BYTES: usize = 64;
pub const SI_STATUS_PHYSICAL_ADDRESS: u32 = 0x0480_0018;
pub const SI_STATUS_DMA_BUSY: u32 = 0x0000_0001;
pub const SI_STATUS_IO_READ_BUSY: u32 = 0x0000_0002;
pub const SI_STATUS_DMA_ERROR: u32 = 0x0000_0008;
pub const SI_STATUS_INTERRUPT: u32 = 0x0000_1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSiInputProfile {
    NoControllerConnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSiCpuStoreProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineSiCpuStoreProvenance {
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
pub struct MachinePifRamState {
    bytes: [u8; PIF_RAM_SIZE_BYTES],
    store_word_provenance: [Option<MachineSiCpuStoreProvenance>; PIF_RAM_SIZE_BYTES / 4],
}

impl MachinePifRamState {
    pub const fn bytes(&self) -> &[u8; PIF_RAM_SIZE_BYTES] {
        &self.bytes
    }

    pub(crate) fn read_u32_be(self, offset: u32) -> Option<u32> {
        let start = usize::try_from(offset).ok()?;
        let bytes = self.bytes.get(start..start.checked_add(4)?)?;
        Some(u32::from_be_bytes(bytes.try_into().ok()?))
    }

    pub const fn store_word_provenance(self, offset: u32) -> Option<MachineSiCpuStoreProvenance> {
        if offset & 3 != 0 || offset >= PIF_RAM_SIZE_BYTES as u32 {
            return None;
        }
        self.store_word_provenance[(offset / 4) as usize]
    }

    fn write_u32_be(&mut self, offset: u32, word: u32, provenance: MachineSiCpuStoreProvenance) {
        let start = offset as usize;
        self.bytes[start..start + 4].copy_from_slice(&word.to_be_bytes());
        self.store_word_provenance[start / 4] = Some(provenance);
    }
}

impl Default for MachinePifRamState {
    fn default() -> Self {
        Self {
            bytes: [0; PIF_RAM_SIZE_BYTES],
            store_word_provenance: [None; PIF_RAM_SIZE_BYTES / 4],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MachineSiStatusState {
    dma_busy: bool,
    io_read_busy: bool,
    dma_error: bool,
}

impl MachineSiStatusState {
    pub const fn dma_busy(self) -> bool {
        self.dma_busy
    }

    pub const fn io_read_busy(self) -> bool {
        self.io_read_busy
    }

    pub const fn dma_error(self) -> bool {
        self.dma_error
    }

    pub const fn word(self, interrupt_pending: bool) -> u32 {
        (if self.dma_busy { SI_STATUS_DMA_BUSY } else { 0 })
            | (if self.io_read_busy {
                SI_STATUS_IO_READ_BUSY
            } else {
                0
            })
            | (if self.dma_error {
                SI_STATUS_DMA_ERROR
            } else {
                0
            })
            | (if interrupt_pending {
                SI_STATUS_INTERRUPT
            } else {
                0
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Si {
    input_profile: MachineSiInputProfile,
    pif_ram: MachinePifRamState,
    status: MachineSiStatusState,
}

impl Si {
    pub(crate) const fn input_profile(self) -> MachineSiInputProfile {
        self.input_profile
    }

    pub(crate) const fn pif_ram_state(self) -> MachinePifRamState {
        self.pif_ram
    }

    pub(crate) const fn status_state(self) -> MachineSiStatusState {
        self.status
    }

    pub(crate) fn apply_pif_ram_store_word(
        &mut self,
        offset: u32,
        word: u32,
        provenance: MachineSiCpuStoreProvenance,
    ) {
        self.pif_ram.write_u32_be(offset, word, provenance);
    }
}

impl Default for Si {
    fn default() -> Self {
        Self {
            input_profile: MachineSiInputProfile::NoControllerConnected,
            pif_ram: MachinePifRamState::default(),
            status: MachineSiStatusState::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cold_status_is_idle_and_interrupt_bit_is_externally_owned() {
        let si = Si::default();
        assert_eq!(
            si.input_profile(),
            MachineSiInputProfile::NoControllerConnected
        );
        assert_eq!(si.pif_ram_state().bytes(), &[0; PIF_RAM_SIZE_BYTES]);
        let status = si.status_state();
        assert!(!status.dma_busy());
        assert!(!status.io_read_busy());
        assert!(!status.dma_error());
        assert_eq!(status.word(false), 0);
        assert_eq!(status.word(true), SI_STATUS_INTERRUPT);
    }
}
