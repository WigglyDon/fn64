use core::fmt;

use crate::cpu::address::CpuAddress;
use crate::machine::MachineBootstrapGprSource;
use crate::rsp::MachineRspInstructionSource;

pub const SP_DMEM_SIZE_BYTES: usize = 4 * 1024;

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
pub struct MachineSpDmemRspStoreWordProvenance {
    instruction_pc: u16,
    instruction_source: MachineRspInstructionSource,
    base_gpr: u8,
    base_value: u32,
    source_gpr: u8,
    source_value: u32,
    signed_offset: i16,
    local_dmem_address: u16,
}

impl MachineSpDmemRspStoreWordProvenance {
    pub(crate) const fn new(
        instruction_pc: u16,
        instruction_source: MachineRspInstructionSource,
        base: (u8, u32),
        source: (u8, u32),
        signed_offset: i16,
        local_dmem_address: u16,
    ) -> Self {
        Self {
            instruction_pc,
            instruction_source,
            base_gpr: base.0,
            base_value: base.1,
            source_gpr: source.0,
            source_value: source.1,
            signed_offset,
            local_dmem_address,
        }
    }

    pub const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub const fn instruction_source(self) -> MachineRspInstructionSource {
        self.instruction_source
    }

    pub const fn base_gpr(self) -> u8 {
        self.base_gpr
    }

    pub const fn base_value(self) -> u32 {
        self.base_value
    }

    pub const fn source_gpr(self) -> u8 {
        self.source_gpr
    }

    pub const fn source_value(self) -> u32 {
        self.source_value
    }

    pub const fn signed_offset(self) -> i16 {
        self.signed_offset
    }

    pub const fn local_dmem_address(self) -> u16 {
        self.local_dmem_address
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDmemRspSqvProvenance {
    instruction_pc: u16,
    instruction_source: MachineRspInstructionSource,
    rsp_commit_index: u64,
    base_gpr: u8,
    base_value: u32,
    source_vector: u8,
    element: u8,
    signed_offset: i8,
    local_dmem_address: u16,
    byte_count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmemRspSqvUnavailableVectorSource {
    ConstructionOrReset,
    Lqv,
    Vsub,
    Vaddc,
    Vxor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MachineSpDmemRspSqvPayload {
    Available([u8; 16]),
    Unavailable {
        source: MachineSpDmemRspSqvUnavailableVectorSource,
    },
}

impl MachineSpDmemRspSqvProvenance {
    pub(crate) const fn new(
        instruction_pc: u16,
        instruction_source: MachineRspInstructionSource,
        rsp_commit_index: u64,
        base: (u8, u32),
        source: (u8, u8),
        transfer: (i8, u16, u8),
    ) -> Self {
        Self {
            instruction_pc,
            instruction_source,
            rsp_commit_index,
            base_gpr: base.0,
            base_value: base.1,
            source_vector: source.0,
            element: source.1,
            signed_offset: transfer.0,
            local_dmem_address: transfer.1,
            byte_count: transfer.2,
        }
    }

    pub const fn instruction_pc(self) -> u16 {
        self.instruction_pc
    }

    pub const fn instruction_source(self) -> MachineRspInstructionSource {
        self.instruction_source
    }

    pub const fn rsp_commit_index(self) -> u64 {
        self.rsp_commit_index
    }

    pub const fn base_gpr(self) -> u8 {
        self.base_gpr
    }

    pub const fn base_value(self) -> u32 {
        self.base_value
    }

    pub const fn source_vector(self) -> u8 {
        self.source_vector
    }

    pub const fn element(self) -> u8 {
        self.element
    }

    pub const fn signed_offset(self) -> i8 {
        self.signed_offset
    }

    pub const fn local_dmem_address(self) -> u16 {
        self.local_dmem_address
    }

    pub const fn byte_count(self) -> u8 {
        self.byte_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmemUnavailableSource {
    ConstructionOrReset,
    BootstrapUncovered,
    RspSqvOpaque {
        provenance: MachineSpDmemRspSqvProvenance,
        source_byte_index: u8,
        source_vector_source: MachineSpDmemRspSqvUnavailableVectorSource,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmemByteSource {
    CartridgeBootstrap {
        cartridge_offset: u32,
    },
    CpuStoreWord {
        provenance: MachineSpDmemStoreWordProvenance,
    },
    RspStoreWord {
        provenance: MachineSpDmemRspStoreWordProvenance,
    },
    RspSqv {
        provenance: MachineSpDmemRspSqvProvenance,
        source_byte_index: u8,
    },
    SpDma {
        record_index: u8,
    },
    #[cfg(test)]
    GeneratedMachineTestStaging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MachineSpDmemStoredByteKnowledge {
    Available {
        source: MachineSpDmemByteSource,
    },
    Unavailable {
        source: MachineSpDmemUnavailableSource,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmemByteKnowledge {
    Available {
        value: u8,
        source: MachineSpDmemByteSource,
    },
    Unavailable {
        source: MachineSpDmemUnavailableSource,
    },
}

impl MachineSpDmemByteKnowledge {
    pub const fn value(self) -> Option<u8> {
        match self {
            Self::Available { value, .. } => Some(value),
            Self::Unavailable { .. } => None,
        }
    }

    pub const fn source(self) -> MachineSpDmemByteKnowledgeSource {
        match self {
            Self::Available { source, .. } => {
                MachineSpDmemByteKnowledgeSource::Available { source }
            }
            Self::Unavailable { source } => {
                MachineSpDmemByteKnowledgeSource::Unavailable { source }
            }
        }
    }

    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineSpDmemByteKnowledgeSource {
    Available {
        source: MachineSpDmemByteSource,
    },
    Unavailable {
        source: MachineSpDmemUnavailableSource,
    },
}

impl MachineSpDmemByteKnowledgeSource {
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineSpDmemByteKnowledgeDescriptor {
    offset: SpDmemOffset,
    source: MachineSpDmemByteKnowledgeSource,
}

impl MachineSpDmemByteKnowledgeDescriptor {
    pub(crate) const fn new(
        offset: SpDmemOffset,
        source: MachineSpDmemByteKnowledgeSource,
    ) -> Self {
        Self { offset, source }
    }

    pub const fn offset(self) -> SpDmemOffset {
        self.offset
    }

    pub const fn source(self) -> MachineSpDmemByteKnowledgeSource {
        self.source
    }

    pub const fn is_available(self) -> bool {
        self.source.is_available()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineSpDmemRspStoreWordPlan {
    offset: SpDmemOffset,
    value: u32,
    provenance: MachineSpDmemRspStoreWordProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MachineSpDmemRspSqvPlan {
    offset: SpDmemOffset,
    payload: MachineSpDmemRspSqvPayload,
    byte_count: u8,
    provenance: MachineSpDmemRspSqvProvenance,
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
    byte_knowledge: Box<[MachineSpDmemStoredByteKnowledge; SP_DMEM_SIZE_BYTES]>,
}

impl SpDmem {
    pub const fn size_bytes(&self) -> usize {
        self.bytes.len()
    }

    /// Returns private backing storage, which is not value truth unless the
    /// corresponding byte knowledge is available.
    pub fn read_u8(&self, offset: SpDmemOffset) -> Result<u8, SpDmemReadError> {
        self.bytes
            .get(offset.as_usize())
            .copied()
            .ok_or(SpDmemReadError { offset, width: 1 })
    }

    /// Returns private backing storage, which is not value truth unless the
    /// corresponding byte knowledge is available.
    pub fn read_u32_be(&self, offset: SpDmemOffset) -> Result<u32, SpDmemReadError> {
        let offset_usize = self.require_u32_be_offset(offset)?;

        Ok(((self.bytes[offset_usize] as u32) << 24)
            | ((self.bytes[offset_usize + 1] as u32) << 16)
            | ((self.bytes[offset_usize + 2] as u32) << 8)
            | self.bytes[offset_usize + 3] as u32)
    }

    pub fn observe_byte(
        &self,
        offset: SpDmemOffset,
    ) -> Result<MachineSpDmemByteKnowledge, SpDmemReadError> {
        let index = offset.as_usize();
        let stored = self
            .byte_knowledge
            .get(index)
            .copied()
            .ok_or(SpDmemReadError { offset, width: 1 })?;
        Ok(match stored {
            MachineSpDmemStoredByteKnowledge::Available { source } => {
                MachineSpDmemByteKnowledge::Available {
                    value: self.bytes[index],
                    source,
                }
            }
            MachineSpDmemStoredByteKnowledge::Unavailable { source } => {
                MachineSpDmemByteKnowledge::Unavailable { source }
            }
        })
    }

    pub fn observe_range<const N: usize>(
        &self,
        offset: SpDmemOffset,
    ) -> Result<[MachineSpDmemByteKnowledge; N], SpDmemReadError> {
        let start = offset.as_usize();
        let Some(end) = start.checked_add(N) else {
            return Err(SpDmemReadError { offset, width: N });
        };
        if end > self.bytes.len() {
            return Err(SpDmemReadError { offset, width: N });
        }
        let mut result = [MachineSpDmemByteKnowledge::Unavailable {
            source: MachineSpDmemUnavailableSource::ConstructionOrReset,
        }; N];
        for (index, entry) in result.iter_mut().enumerate() {
            *entry = self
                .observe_byte(SpDmemOffset::new(offset.value() + index as u32))
                .expect("preflighted SP DMEM byte range remains in bounds");
        }
        Ok(result)
    }

    pub fn describe_range<const N: usize>(
        &self,
        offset: SpDmemOffset,
    ) -> Result<[MachineSpDmemByteKnowledgeDescriptor; N], SpDmemReadError> {
        let observations = self.observe_range::<N>(offset)?;
        let mut result = [MachineSpDmemByteKnowledgeDescriptor::new(
            offset,
            MachineSpDmemByteKnowledgeSource::Unavailable {
                source: MachineSpDmemUnavailableSource::ConstructionOrReset,
            },
        ); N];
        for (index, entry) in result.iter_mut().enumerate() {
            *entry = MachineSpDmemByteKnowledgeDescriptor::new(
                SpDmemOffset::new(offset.value() + index as u32),
                observations[index].source(),
            );
        }
        Ok(result)
    }

    pub fn store_word_provenance(
        &self,
        offset: SpDmemOffset,
    ) -> Option<MachineSpDmemStoreWordProvenance> {
        if offset.value() & 3 != 0 {
            return None;
        }
        let observations = self.observe_range::<4>(offset).ok()?;
        let first = match observations[0] {
            MachineSpDmemByteKnowledge::Available {
                source: MachineSpDmemByteSource::CpuStoreWord { provenance },
                ..
            } => provenance,
            _ => return None,
        };
        observations
            .iter()
            .all(|observation| {
                matches!(
                    observation,
                    MachineSpDmemByteKnowledge::Available {
                        source: MachineSpDmemByteSource::CpuStoreWord { provenance },
                        ..
                    } if *provenance == first
                )
            })
            .then_some(first)
    }

    pub fn rsp_store_word_provenance(
        &self,
        offset: SpDmemOffset,
    ) -> Option<MachineSpDmemRspStoreWordProvenance> {
        if offset.value() & 3 != 0 {
            return None;
        }
        let observations = self.observe_range::<4>(offset).ok()?;
        let first = match observations[0] {
            MachineSpDmemByteKnowledge::Available {
                source: MachineSpDmemByteSource::RspStoreWord { provenance },
                ..
            } => provenance,
            _ => return None,
        };
        observations
            .iter()
            .all(|observation| {
                matches!(
                    observation,
                    MachineSpDmemByteKnowledge::Available {
                        source: MachineSpDmemByteSource::RspStoreWord { provenance },
                        ..
                    } if *provenance == first
                )
            })
            .then_some(first)
    }

    pub fn dma_record_index(&self, offset: SpDmemOffset) -> Option<u8> {
        match self.observe_byte(offset).ok()? {
            MachineSpDmemByteKnowledge::Available {
                source: MachineSpDmemByteSource::SpDma { record_index },
                ..
            } => Some(record_index),
            _ => None,
        }
    }

    fn require_u32_be_offset(&self, offset: SpDmemOffset) -> Result<usize, SpDmemReadError> {
        let offset_usize = offset.as_usize();
        if offset_usize > self.bytes.len() - 4 {
            return Err(SpDmemReadError { offset, width: 4 });
        }

        Ok(offset_usize)
    }

    #[cfg(test)]
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
        self.byte_knowledge[offset_usize..end].fill(MachineSpDmemStoredByteKnowledge::Available {
            source: MachineSpDmemByteSource::GeneratedMachineTestStaging,
        });
        Ok(())
    }

    pub(crate) fn write_cartridge_bootstrap_bytes(
        &mut self,
        offset: SpDmemOffset,
        bytes: &[u8],
        cartridge_offset: u32,
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

        for knowledge in self.byte_knowledge.iter_mut() {
            if matches!(
                knowledge,
                MachineSpDmemStoredByteKnowledge::Unavailable {
                    source: MachineSpDmemUnavailableSource::ConstructionOrReset,
                }
            ) {
                *knowledge = MachineSpDmemStoredByteKnowledge::Unavailable {
                    source: MachineSpDmemUnavailableSource::BootstrapUncovered,
                };
            }
        }
        destination.copy_from_slice(bytes);
        for (index, knowledge) in self.byte_knowledge[offset_usize..end]
            .iter_mut()
            .enumerate()
        {
            *knowledge = MachineSpDmemStoredByteKnowledge::Available {
                source: MachineSpDmemByteSource::CartridgeBootstrap {
                    cartridge_offset: cartridge_offset + index as u32,
                },
            };
        }
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
        self.byte_knowledge[offset_usize..offset_usize + 4].fill(
            MachineSpDmemStoredByteKnowledge::Available {
                source: MachineSpDmemByteSource::CpuStoreWord { provenance },
            },
        );
        Ok(())
    }

    pub(crate) fn plan_rsp_store_word(
        &self,
        offset: SpDmemOffset,
        value: u32,
        provenance: MachineSpDmemRspStoreWordProvenance,
    ) -> Result<MachineSpDmemRspStoreWordPlan, SpDmemWriteError> {
        let offset_usize = self
            .require_u32_be_offset(offset)
            .map_err(|_| SpDmemWriteError { offset, width: 4 })?;
        if offset_usize & 3 != 0 {
            return Err(SpDmemWriteError { offset, width: 4 });
        }
        Ok(MachineSpDmemRspStoreWordPlan {
            offset,
            value,
            provenance,
        })
    }

    pub(crate) fn apply_rsp_store_word(&mut self, plan: MachineSpDmemRspStoreWordPlan) {
        let offset = plan.offset.as_usize();
        self.bytes[offset..offset + 4].copy_from_slice(&plan.value.to_be_bytes());
        self.byte_knowledge[offset..offset + 4].fill(MachineSpDmemStoredByteKnowledge::Available {
            source: MachineSpDmemByteSource::RspStoreWord {
                provenance: plan.provenance,
            },
        });
    }

    pub(crate) fn plan_rsp_sqv(
        &self,
        offset: SpDmemOffset,
        payload: MachineSpDmemRspSqvPayload,
        byte_count: u8,
        provenance: MachineSpDmemRspSqvProvenance,
    ) -> Result<MachineSpDmemRspSqvPlan, SpDmemWriteError> {
        let width = usize::from(byte_count);
        if width == 0 || width > 16 {
            return Err(SpDmemWriteError { offset, width });
        }
        let start = offset.as_usize();
        let Some(end) = start.checked_add(width) else {
            return Err(SpDmemWriteError { offset, width });
        };
        if end > self.bytes.len() {
            return Err(SpDmemWriteError { offset, width });
        }
        Ok(MachineSpDmemRspSqvPlan {
            offset,
            payload,
            byte_count,
            provenance,
        })
    }

    pub(crate) fn apply_rsp_sqv(&mut self, plan: MachineSpDmemRspSqvPlan) {
        let start = plan.offset.as_usize();
        let width = usize::from(plan.byte_count);
        match plan.payload {
            MachineSpDmemRspSqvPayload::Available(bytes) => {
                self.bytes[start..start + width].copy_from_slice(&bytes[..width]);
                for (source_byte_index, knowledge) in self.byte_knowledge[start..start + width]
                    .iter_mut()
                    .enumerate()
                {
                    *knowledge = MachineSpDmemStoredByteKnowledge::Available {
                        source: MachineSpDmemByteSource::RspSqv {
                            provenance: plan.provenance,
                            source_byte_index: source_byte_index as u8,
                        },
                    };
                }
            }
            MachineSpDmemRspSqvPayload::Unavailable { source } => {
                for (source_byte_index, knowledge) in self.byte_knowledge[start..start + width]
                    .iter_mut()
                    .enumerate()
                {
                    *knowledge = MachineSpDmemStoredByteKnowledge::Unavailable {
                        source: MachineSpDmemUnavailableSource::RspSqvOpaque {
                            provenance: plan.provenance,
                            source_byte_index: source_byte_index as u8,
                            source_vector_source: source,
                        },
                    };
                }
            }
        }
    }

    pub(crate) fn apply_sp_dma_byte(
        &mut self,
        offset: SpDmemOffset,
        value: u8,
        dma_record_index: u8,
    ) {
        let offset = offset.as_usize();
        self.bytes[offset] = value;
        self.byte_knowledge[offset] = MachineSpDmemStoredByteKnowledge::Available {
            source: MachineSpDmemByteSource::SpDma {
                record_index: dma_record_index,
            },
        };
    }

    #[cfg(test)]
    pub(crate) fn write_u32_be_for_test(&mut self, offset: SpDmemOffset, value: u32) {
        self.write_bytes(offset, &value.to_be_bytes())
            .expect("test word staging remains inside SP DMEM");
    }
}

impl Default for SpDmem {
    fn default() -> Self {
        Self {
            bytes: [0; SP_DMEM_SIZE_BYTES],
            byte_knowledge: Box::new(
                [MachineSpDmemStoredByteKnowledge::Unavailable {
                    source: MachineSpDmemUnavailableSource::ConstructionOrReset,
                }; SP_DMEM_SIZE_BYTES],
            ),
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
        assert_eq!(
            sp_dmem.observe_byte(SpDmemOffset::new(0)),
            Ok(MachineSpDmemByteKnowledge::Unavailable {
                source: MachineSpDmemUnavailableSource::ConstructionOrReset,
            })
        );
        assert_eq!(
            sp_dmem.observe_byte(SpDmemOffset::new(0)).unwrap().value(),
            None,
            "private backing zero is not Machine value truth"
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

    #[test]
    fn sp_dmem_knowledge_bootstrap_covered_and_uncovered_ranges_are_exact() {
        let mut sp_dmem = SpDmem::default();
        let bytes = [0x91, 0x72, 0x53, 0x34, 0x15];

        sp_dmem
            .write_cartridge_bootstrap_bytes(SpDmemOffset::new(0x40), &bytes, 0x1040)
            .unwrap();

        for offset in [0, 0x0f, 0x3f, 0x45, 0xfff] {
            assert_eq!(
                sp_dmem.observe_byte(SpDmemOffset::new(offset)),
                Ok(MachineSpDmemByteKnowledge::Unavailable {
                    source: MachineSpDmemUnavailableSource::BootstrapUncovered,
                })
            );
        }
        for (index, value) in bytes.into_iter().enumerate() {
            assert_eq!(
                sp_dmem.observe_byte(SpDmemOffset::new(0x40 + index as u32)),
                Ok(MachineSpDmemByteKnowledge::Available {
                    value,
                    source: MachineSpDmemByteSource::CartridgeBootstrap {
                        cartridge_offset: 0x1040 + index as u32,
                    },
                })
            );
        }
        let descriptions = sp_dmem
            .describe_range::<5>(SpDmemOffset::new(0x40))
            .unwrap();
        assert!(descriptions.iter().enumerate().all(|(index, descriptor)| {
            descriptor.offset() == SpDmemOffset::new(0x40 + index as u32)
                && matches!(
                    descriptor.source(),
                    MachineSpDmemByteKnowledgeSource::Available {
                        source: MachineSpDmemByteSource::CartridgeBootstrap {
                            cartridge_offset,
                        },
                    } if cartridge_offset == 0x1040 + index as u32
                )
        }));
    }

    #[test]
    fn sp_dmem_knowledge_cpu_store_and_dma_replace_only_exact_bytes() {
        let mut sp_dmem = SpDmem::default();
        let provenance = MachineSpDmemStoreWordProvenance::new(
            CpuAddress::new(0x8000_0040),
            7,
            MachineBootstrapGprSource::ArchitecturalZero,
            0xffff_ffff_a400_0020,
            CpuAddress::new(0xa400_0020),
            0x0400_0020,
        );

        sp_dmem
            .write_cpu_u32_be(SpDmemOffset::new(0x20), 0x1020_3040, provenance)
            .unwrap();
        for (index, value) in [0x10, 0x20, 0x30, 0x40].into_iter().enumerate() {
            assert_eq!(
                sp_dmem.observe_byte(SpDmemOffset::new(0x20 + index as u32)),
                Ok(MachineSpDmemByteKnowledge::Available {
                    value,
                    source: MachineSpDmemByteSource::CpuStoreWord { provenance },
                })
            );
        }
        assert_eq!(
            sp_dmem.observe_byte(SpDmemOffset::new(0x24)),
            Ok(MachineSpDmemByteKnowledge::Unavailable {
                source: MachineSpDmemUnavailableSource::ConstructionOrReset,
            })
        );

        sp_dmem.apply_sp_dma_byte(SpDmemOffset::new(0x22), 0xa5, 3);
        assert_eq!(
            sp_dmem.observe_byte(SpDmemOffset::new(0x22)),
            Ok(MachineSpDmemByteKnowledge::Available {
                value: 0xa5,
                source: MachineSpDmemByteSource::SpDma { record_index: 3 },
            })
        );
        assert_eq!(sp_dmem.dma_record_index(SpDmemOffset::new(0x22)), Some(3));
        assert_eq!(sp_dmem.store_word_provenance(SpDmemOffset::new(0x20)), None);
    }

    #[test]
    fn sp_dmem_rsp_sw_plan_commits_exact_word_provenance_and_rejects_atomically() {
        let mut first = SpDmem::default();
        let second = SpDmem::default();
        let provenance = MachineSpDmemRspStoreWordProvenance::new(
            0x020,
            MachineRspInstructionSource::GeneratedMachineTestStaging,
            (3, 0xabcd_f020),
            (4, 0x1122_3344),
            0,
            0x020,
        );
        let plan = first
            .plan_rsp_store_word(SpDmemOffset::new(0x020), 0x1122_3344, provenance)
            .unwrap();
        assert!(first
            .observe_range::<4>(SpDmemOffset::new(0x020))
            .unwrap()
            .iter()
            .all(|knowledge| !knowledge.is_available()));
        first.apply_rsp_store_word(plan);
        assert_eq!(first.read_u32_be(SpDmemOffset::new(0x020)), Ok(0x1122_3344));
        assert_eq!(
            first.rsp_store_word_provenance(SpDmemOffset::new(0x020)),
            Some(provenance)
        );
        assert_eq!(provenance.instruction_pc(), 0x020);
        assert_eq!(
            provenance.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );
        assert_eq!(provenance.base_gpr(), 3);
        assert_eq!(provenance.base_value(), 0xabcd_f020);
        assert_eq!(provenance.source_gpr(), 4);
        assert_eq!(provenance.source_value(), 0x1122_3344);
        assert_eq!(provenance.signed_offset(), 0);
        assert_eq!(provenance.local_dmem_address(), 0x020);
        assert!(matches!(
            first.observe_byte(SpDmemOffset::new(0x024)),
            Ok(MachineSpDmemByteKnowledge::Unavailable {
                source: MachineSpDmemUnavailableSource::ConstructionOrReset,
            })
        ));
        assert!(second
            .observe_range::<4>(SpDmemOffset::new(0x020))
            .unwrap()
            .iter()
            .all(|knowledge| !knowledge.is_available()));

        for offset in [0x021, 0x0ffd] {
            let before = first.observe_range::<8>(SpDmemOffset::new(0x018)).unwrap();
            let error = first
                .plan_rsp_store_word(SpDmemOffset::new(offset), 0xaabb_ccdd, provenance)
                .unwrap_err();
            assert_eq!(error.offset(), SpDmemOffset::new(offset));
            assert_eq!(error.width(), 4);
            assert_eq!(
                first.observe_range::<8>(SpDmemOffset::new(0x018)).unwrap(),
                before
            );
        }
    }

    #[test]
    fn sp_dmem_rsp_sqv_plan_commits_exact_prefix_provenance_and_rejects_atomically() {
        let mut first = SpDmem::default();
        let second = SpDmem::default();
        let bytes = core::array::from_fn(|index| 0x31_u8.wrapping_add(index as u8 * 5));
        let provenance = MachineSpDmemRspSqvProvenance::new(
            0x028,
            MachineRspInstructionSource::GeneratedMachineTestStaging,
            1,
            (3, 0xabcd_f027),
            (12, 0),
            (0, 0x027, 9),
        );
        let plan = first
            .plan_rsp_sqv(
                SpDmemOffset::new(0x027),
                MachineSpDmemRspSqvPayload::Available(bytes),
                9,
                provenance,
            )
            .unwrap();
        assert!(first
            .observe_range::<9>(SpDmemOffset::new(0x027))
            .unwrap()
            .iter()
            .all(|knowledge| !knowledge.is_available()));
        first.apply_rsp_sqv(plan);
        for index in 0..9 {
            assert_eq!(
                first.observe_byte(SpDmemOffset::new(0x027 + index)),
                Ok(MachineSpDmemByteKnowledge::Available {
                    value: bytes[index as usize],
                    source: MachineSpDmemByteSource::RspSqv {
                        provenance,
                        source_byte_index: index as u8,
                    },
                })
            );
        }
        assert_eq!(provenance.instruction_pc(), 0x028);
        assert_eq!(
            provenance.instruction_source(),
            MachineRspInstructionSource::GeneratedMachineTestStaging
        );
        assert_eq!(provenance.rsp_commit_index(), 1);
        assert_eq!(provenance.base_gpr(), 3);
        assert_eq!(provenance.base_value(), 0xabcd_f027);
        assert_eq!(provenance.source_vector(), 12);
        assert_eq!(provenance.element(), 0);
        assert_eq!(provenance.signed_offset(), 0);
        assert_eq!(provenance.local_dmem_address(), 0x027);
        assert_eq!(provenance.byte_count(), 9);
        assert!(matches!(
            first.observe_byte(SpDmemOffset::new(0x030)),
            Ok(MachineSpDmemByteKnowledge::Unavailable { .. })
        ));
        assert!(matches!(
            second.observe_byte(SpDmemOffset::new(0x027)),
            Ok(MachineSpDmemByteKnowledge::Unavailable { .. })
        ));

        let mut aligned = SpDmem::default();
        let aligned_provenance = MachineSpDmemRspSqvProvenance::new(
            0x040,
            MachineRspInstructionSource::GeneratedMachineTestStaging,
            2,
            (0, 0),
            (7, 0),
            (4, 0x040, 16),
        );
        let aligned_plan = aligned
            .plan_rsp_sqv(
                SpDmemOffset::new(0x040),
                MachineSpDmemRspSqvPayload::Available(bytes),
                16,
                aligned_provenance,
            )
            .unwrap();
        aligned.apply_rsp_sqv(aligned_plan);
        assert_eq!(aligned.read_u8(SpDmemOffset::new(0x040)), Ok(bytes[0]));
        assert_eq!(aligned.read_u8(SpDmemOffset::new(0x04f)), Ok(bytes[15]));
        assert!(aligned
            .observe_range::<16>(SpDmemOffset::new(0x040))
            .unwrap()
            .iter()
            .enumerate()
            .all(|(index, knowledge)| matches!(
                knowledge,
                MachineSpDmemByteKnowledge::Available {
                    source: MachineSpDmemByteSource::RspSqv {
                        provenance: candidate,
                        source_byte_index,
                    },
                    ..
                } if *candidate == aligned_provenance && usize::from(*source_byte_index) == index
            )));

        for (offset, byte_count) in [(0x020, 0), (0x020, 17), (0xfff, 2)] {
            let before = first.observe_range::<32>(SpDmemOffset::new(0x020)).unwrap();
            let error = first
                .plan_rsp_sqv(
                    SpDmemOffset::new(offset),
                    MachineSpDmemRspSqvPayload::Available(bytes),
                    byte_count,
                    provenance,
                )
                .unwrap_err();
            assert_eq!(error.offset(), SpDmemOffset::new(offset));
            assert_eq!(error.width(), usize::from(byte_count));
            assert_eq!(
                first.observe_range::<32>(SpDmemOffset::new(0x020)).unwrap(),
                before
            );
        }
    }

    #[test]
    fn sp_dmem_rsp_sqv_opaque_payload_supersedes_exact_knowledge_and_restores_exactly() {
        let mut known_destination = SpDmem::default();
        known_destination
            .write_bytes(SpDmemOffset::new(0x03f), &[0xa5; 18])
            .unwrap();
        let opaque_provenance = MachineSpDmemRspSqvProvenance::new(
            0x080,
            MachineRspInstructionSource::GeneratedMachineTestStaging,
            7,
            (0, 0x040),
            (12, 0),
            (0, 0x040, 16),
        );
        let opaque_plan = known_destination
            .plan_rsp_sqv(
                SpDmemOffset::new(0x040),
                MachineSpDmemRspSqvPayload::Unavailable {
                    source: MachineSpDmemRspSqvUnavailableVectorSource::ConstructionOrReset,
                },
                16,
                opaque_provenance,
            )
            .unwrap();
        known_destination.apply_rsp_sqv(opaque_plan);

        assert!(known_destination
            .observe_byte(SpDmemOffset::new(0x03f))
            .unwrap()
            .is_available());
        assert!(known_destination
            .observe_byte(SpDmemOffset::new(0x050))
            .unwrap()
            .is_available());
        for index in 0_u8..16 {
            assert_eq!(
                known_destination.observe_byte(SpDmemOffset::new(0x040 + u32::from(index))),
                Ok(MachineSpDmemByteKnowledge::Unavailable {
                    source: MachineSpDmemUnavailableSource::RspSqvOpaque {
                        provenance: opaque_provenance,
                        source_byte_index: index,
                        source_vector_source:
                            MachineSpDmemRspSqvUnavailableVectorSource::ConstructionOrReset,
                    },
                })
            );
        }

        let mut prior_unavailable = SpDmem::default();
        let prior_unavailable_plan = prior_unavailable
            .plan_rsp_sqv(
                SpDmemOffset::new(0x040),
                MachineSpDmemRspSqvPayload::Unavailable {
                    source: MachineSpDmemRspSqvUnavailableVectorSource::Vxor,
                },
                16,
                opaque_provenance,
            )
            .unwrap();
        prior_unavailable.apply_rsp_sqv(prior_unavailable_plan);
        assert!(prior_unavailable
            .observe_range::<16>(SpDmemOffset::new(0x040))
            .unwrap()
            .iter()
            .all(|knowledge| matches!(
                knowledge,
                MachineSpDmemByteKnowledge::Unavailable {
                    source: MachineSpDmemUnavailableSource::RspSqvOpaque {
                        source_vector_source: MachineSpDmemRspSqvUnavailableVectorSource::Vxor,
                        ..
                    }
                }
            )));

        let restored_bytes = core::array::from_fn(|index| 0x11_u8.wrapping_add(index as u8));
        let restored_provenance = MachineSpDmemRspSqvProvenance::new(
            0x084,
            MachineRspInstructionSource::GeneratedMachineTestStaging,
            8,
            (0, 0x040),
            (13, 0),
            (0, 0x040, 16),
        );
        let restored_plan = known_destination
            .plan_rsp_sqv(
                SpDmemOffset::new(0x040),
                MachineSpDmemRspSqvPayload::Available(restored_bytes),
                16,
                restored_provenance,
            )
            .unwrap();
        known_destination.apply_rsp_sqv(restored_plan);
        for index in 0_u8..16 {
            assert_eq!(
                known_destination.observe_byte(SpDmemOffset::new(0x040 + u32::from(index))),
                Ok(MachineSpDmemByteKnowledge::Available {
                    value: restored_bytes[usize::from(index)],
                    source: MachineSpDmemByteSource::RspSqv {
                        provenance: restored_provenance,
                        source_byte_index: index,
                    },
                })
            );
        }

        let before_failure = known_destination
            .observe_range::<16>(SpDmemOffset::new(0x040))
            .unwrap();
        let failure = known_destination
            .plan_rsp_sqv(
                SpDmemOffset::new(0xff1),
                MachineSpDmemRspSqvPayload::Unavailable {
                    source: MachineSpDmemRspSqvUnavailableVectorSource::ConstructionOrReset,
                },
                16,
                opaque_provenance,
            )
            .unwrap_err();
        assert_eq!(failure.offset(), SpDmemOffset::new(0xff1));
        assert_eq!(failure.width(), 16);
        assert_eq!(
            known_destination
                .observe_range::<16>(SpDmemOffset::new(0x040))
                .unwrap(),
            before_failure
        );
    }

    #[test]
    fn sp_dmem_knowledge_range_preflight_and_machine_instances_are_independent() {
        let mut first = SpDmem::default();
        let second = SpDmem::default();
        first
            .write_bytes(SpDmemOffset::new(0xff0), &[0x5a; 16])
            .unwrap();

        assert!(first
            .observe_range::<16>(SpDmemOffset::new(0xff0))
            .unwrap()
            .iter()
            .all(|knowledge| knowledge.is_available()));
        let error = first
            .observe_range::<16>(SpDmemOffset::new(0xff1))
            .unwrap_err();
        assert_eq!(error.offset(), SpDmemOffset::new(0xff1));
        assert_eq!(error.width(), 16);
        assert_eq!(
            second.observe_byte(SpDmemOffset::new(0xff0)),
            Ok(MachineSpDmemByteKnowledge::Unavailable {
                source: MachineSpDmemUnavailableSource::ConstructionOrReset,
            })
        );
    }
}
