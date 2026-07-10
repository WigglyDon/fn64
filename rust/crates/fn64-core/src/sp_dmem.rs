use core::fmt;

pub const SP_DMEM_SIZE_BYTES: usize = 4 * 1024;

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
        Ok(())
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
}
