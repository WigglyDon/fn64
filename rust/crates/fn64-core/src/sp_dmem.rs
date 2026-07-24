use core::fmt;

use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;

pub const SP_DMEM_SIZE_BYTES: usize = 4 * 1024;
const SP_DMEM_WORD_COUNT: usize = SP_DMEM_SIZE_BYTES / 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDmemStoreWordProvenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
    effective_address: u64,
    cpu_address: CpuAddress,
    physical_address: u32,
}

impl MachineSpDmemStoreWordProvenance {
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
pub struct SpDmemOffset(u32);

impl SpDmemOffset {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpDmemReadError {
    offset: SpDmemOffset,
    width: usize,
}

impl SpDmemReadError {
    pub const fn offset(self) -> SpDmemOffset {
        self.offset
    }

    pub const fn width(self) -> usize {
        self.width
    }
}

impl fmt::Display for SpDmemReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SP DMEM access out of range: address={} width={}",
            self.offset.value(),
            self.width
        )
    }
}

impl std::error::Error for SpDmemReadError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpDmemWriteError {
    offset: SpDmemOffset,
    width: usize,
}

impl SpDmemWriteError {
    pub(crate) const fn offset(self) -> SpDmemOffset {
        self.offset
    }

    pub(crate) const fn width(self) -> usize {
        self.width
    }
}

pub struct SpDmem {
    bytes: [u8; SP_DMEM_SIZE_BYTES],
    word_store_provenance: Box<[Option<MachineSpDmemStoreWordProvenance>; SP_DMEM_WORD_COUNT]>,
    dma_provenance: Box<[Option<u8>; SP_DMEM_SIZE_BYTES]>,
}

impl SpDmem {
    pub const fn size_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub fn read_u8(&self, offset: SpDmemOffset) -> Result<u8, SpDmemReadError> {
        self.bytes
            .get(offset.as_usize())
            .copied()
            .ok_or(SpDmemReadError { offset, width: 1 })
    }

    pub fn read_u32_be(&self, offset: SpDmemOffset) -> Result<u32, SpDmemReadError> {
        let offset_usize = self.require_u32_be_offset(offset)?;

        Ok(((self.bytes[offset_usize] as u32) << 24)
            | ((self.bytes[offset_usize + 1] as u32) << 16)
            | ((self.bytes[offset_usize + 2] as u32) << 8)
            | self.bytes[offset_usize + 3] as u32)
    }

    pub fn store_word_provenance(
        &self,
        offset: SpDmemOffset,
    ) -> Option<MachineSpDmemStoreWordProvenance> {
        if offset.value() & 3 != 0 {
            return None;
        }
        self.word_store_provenance
            .get(offset.as_usize() / 4)
            .copied()
            .flatten()
    }

    pub fn dma_record_index(&self, offset: SpDmemOffset) -> Option<u8> {
        self.dma_provenance
            .get(offset.as_usize())
            .copied()
            .flatten()
    }

    fn require_u32_be_offset(&self, offset: SpDmemOffset) -> Result<usize, SpDmemReadError> {
        let offset_usize = offset.as_usize();
        if offset_usize > self.bytes.len() - 4 {
            return Err(SpDmemReadError { offset, width: 4 });
        }

        Ok(offset_usize)
    }

    pub(crate) fn write_bytes(
        &mut self,
        offset: SpDmemOffset,
        bytes: &[u8],
    ) -> Result<(), SpDmemWriteError> {
        let offset_usize = offset.as_usize();
        let Some(end) = offset_usize.checked_add(bytes.len()) else {
            return Err(SpDmemWriteError {
                offset,
                width: bytes.len(),
            });
        };
        let Some(destination) = self.bytes.get_mut(offset_usize..end) else {
            return Err(SpDmemWriteError {
                offset,
                width: bytes.len(),
            });
        };

        destination.copy_from_slice(bytes);
        if bytes.is_empty() {
            return Ok(());
        }
        let first_word = offset_usize / 4;
        let last_word = (end.saturating_sub(1)) / 4;
        for word in first_word..=last_word {
            self.word_store_provenance[word] = None;
        }
        self.dma_provenance[offset_usize..end].fill(None);
        Ok(())
    }

    pub(crate) fn write_cpu_u32_be(
        &mut self,
        offset: SpDmemOffset,
        value: u32,
        provenance: MachineSpDmemStoreWordProvenance,
    ) -> Result<(), SpDmemWriteError> {
        let offset_usize = self
            .require_u32_be_offset(offset)
            .map_err(|_| SpDmemWriteError { offset, width: 4 })?;
        if offset_usize & 3 != 0 {
            return Err(SpDmemWriteError { offset, width: 4 });
        }
        self.bytes[offset_usize..offset_usize + 4].copy_from_slice(&value.to_be_bytes());
        self.word_store_provenance[offset_usize / 4] = Some(provenance);
        self.dma_provenance[offset_usize..offset_usize + 4].fill(None);
        Ok(())
    }

    pub(crate) fn apply_sp_dma_byte(
        &mut self,
        offset: SpDmemOffset,
        value: u8,
        dma_record_index: u8,
    ) {
        let offset = offset.as_usize();
        self.bytes[offset] = value;
        self.word_store_provenance[offset / 4] = None;
        self.dma_provenance[offset] = Some(dma_record_index);
    }

    #[cfg(test)]
    pub(crate) fn write_u32_be_for_test(&mut self, offset: SpDmemOffset, value: u32) {
        let offset_usize = self.require_u32_be_offset(offset).unwrap();
        self.bytes[offset_usize] = ((value >> 24) & 0xff) as u8;
        self.bytes[offset_usize + 1] = ((value >> 16) & 0xff) as u8;
        self.bytes[offset_usize + 2] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset_usize + 3] = (value & 0xff) as u8;
    }
}

impl Default for SpDmem {
    fn default() -> Self {
        Self {
            bytes: [0; SP_DMEM_SIZE_BYTES],
            word_store_provenance: Box::new([None; SP_DMEM_WORD_COUNT]),
            dma_provenance: Box::new([None; SP_DMEM_SIZE_BYTES]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_sp_dmem_has_cpp_storage_size() {
        let sp_dmem = SpDmem::default();

        assert_eq!(sp_dmem.size_bytes(), SP_DMEM_SIZE_BYTES);
        assert_eq!(sp_dmem.size_bytes(), 4 * 1024);
    }

    #[test]
    fn default_sp_dmem_storage_is_zero_filled() {
        let sp_dmem = SpDmem::default();

        assert!(sp_dmem.bytes.iter().all(|byte| *byte == 0));
        assert_eq!(sp_dmem.read_u8(SpDmemOffset::new(0)), Ok(0));
        assert_eq!(
            sp_dmem.read_u8(SpDmemOffset::new((SP_DMEM_SIZE_BYTES - 1) as u32)),
            Ok(0)
        );
    }

    #[test]
    fn sp_dmem_u32_be_read_observes_big_endian_storage_order() {
        let mut sp_dmem = SpDmem::default();

        sp_dmem.write_u32_be_for_test(SpDmemOffset::new(0x20), 0x3c01_1234);

        assert_eq!(
            sp_dmem.read_u32_be(SpDmemOffset::new(0x20)),
            Ok(0x3c01_1234)
        );
        assert_eq!(sp_dmem.read_u8(SpDmemOffset::new(0x20)), Ok(0x3c));
        assert_eq!(sp_dmem.read_u8(SpDmemOffset::new(0x21)), Ok(0x01));
        assert_eq!(sp_dmem.read_u8(SpDmemOffset::new(0x22)), Ok(0x12));
        assert_eq!(sp_dmem.read_u8(SpDmemOffset::new(0x23)), Ok(0x34));
    }

    #[test]
    fn sp_dmem_u32_be_read_uses_width_four_span_boundary() {
        let mut sp_dmem = SpDmem::default();
        let last_valid_offset = SpDmemOffset::new((SP_DMEM_SIZE_BYTES - 4) as u32);

        sp_dmem.write_u32_be_for_test(last_valid_offset, 0x0123_4567);

        assert_eq!(sp_dmem.read_u32_be(last_valid_offset), Ok(0x0123_4567));

        for offset in [
            SP_DMEM_SIZE_BYTES - 3,
            SP_DMEM_SIZE_BYTES - 2,
            SP_DMEM_SIZE_BYTES - 1,
            SP_DMEM_SIZE_BYTES,
        ] {
            let error = sp_dmem
                .read_u32_be(SpDmemOffset::new(offset as u32))
                .unwrap_err();
            assert_eq!(error.offset(), SpDmemOffset::new(offset as u32));
            assert_eq!(error.width(), 4);
        }
    }

    #[test]
    fn sp_dmem_range_write_preflights_before_mutation() {
        let mut sp_dmem = SpDmem::default();
        let error = sp_dmem
            .write_bytes(
                SpDmemOffset::new((SP_DMEM_SIZE_BYTES - 1) as u32),
                &[0x11, 0x22],
            )
            .unwrap_err();

        assert_eq!(
            error.offset(),
            SpDmemOffset::new((SP_DMEM_SIZE_BYTES - 1) as u32)
        );
        assert_eq!(error.width(), 2);
        assert_eq!(
            sp_dmem
                .read_u8(SpDmemOffset::new((SP_DMEM_SIZE_BYTES - 1) as u32))
                .unwrap(),
            0
        );
    }

    #[test]
    fn cpu_word_store_records_one_provenance_and_bulk_replacement_clears_it() {
        let mut sp_dmem = SpDmem::default();
        let provenance = MachineSpDmemStoreWordProvenance::new(
            CpuAddress::new(0x8000_0270),
            14,
            MachineBootstrapGprSource::ArchitecturalZero,
            0xffff_ffff_a400_0020,
            CpuAddress::new(0xa400_0020),
            0x0400_0020,
        );
        sp_dmem
            .write_cpu_u32_be(SpDmemOffset::new(0x20), 0xa400_2000, provenance)
            .unwrap();
        assert_eq!(
            sp_dmem.read_u32_be(SpDmemOffset::new(0x20)),
            Ok(0xa400_2000)
        );
        assert_eq!(
            sp_dmem.store_word_provenance(SpDmemOffset::new(0x20)),
            Some(provenance)
        );
        assert_eq!(sp_dmem.store_word_provenance(SpDmemOffset::new(0x21)), None);

        sp_dmem
            .write_bytes(SpDmemOffset::new(0x21), &[0x55])
            .unwrap();
        assert_eq!(sp_dmem.store_word_provenance(SpDmemOffset::new(0x20)), None);
    }
}
