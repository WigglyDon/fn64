use core::fmt;

pub const RDRAM_SIZE_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RdramAccessError {
    offset: usize,
    width: usize,
}

impl RdramAccessError {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl fmt::Display for RdramAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RDRAM access out of range: address={} width={}",
            self.offset, self.width
        )
    }
}

impl std::error::Error for RdramAccessError {}

pub struct Rdram {
    bytes: Vec<u8>,
}

impl Rdram {
    pub fn size_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub fn read_u8(&self, offset: usize) -> Result<u8, RdramAccessError> {
        self.bytes
            .get(offset)
            .copied()
            .ok_or(RdramAccessError { offset, width: 1 })
    }

    pub fn read_u16_be(&self, offset: usize) -> Result<u16, RdramAccessError> {
        self.require_u16_be_offset(offset)?;

        Ok(((self.bytes[offset] as u16) << 8) | self.bytes[offset + 1] as u16)
    }

    pub fn read_u32_be(&self, offset: usize) -> Result<u32, RdramAccessError> {
        self.require_u32_be_offset(offset)?;

        Ok(((self.bytes[offset] as u32) << 24)
            | ((self.bytes[offset + 1] as u32) << 16)
            | ((self.bytes[offset + 2] as u32) << 8)
            | self.bytes[offset + 3] as u32)
    }

    pub fn read_u64_be(&self, offset: usize) -> Result<u64, RdramAccessError> {
        self.require_u64_be_offset(offset)?;

        Ok(((self.bytes[offset] as u64) << 56)
            | ((self.bytes[offset + 1] as u64) << 48)
            | ((self.bytes[offset + 2] as u64) << 40)
            | ((self.bytes[offset + 3] as u64) << 32)
            | ((self.bytes[offset + 4] as u64) << 24)
            | ((self.bytes[offset + 5] as u64) << 16)
            | ((self.bytes[offset + 6] as u64) << 8)
            | self.bytes[offset + 7] as u64)
    }

    pub(crate) fn require_u8_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        self.bytes
            .get(offset)
            .map(|_| ())
            .ok_or(RdramAccessError { offset, width: 1 })
    }

    pub(crate) fn require_u16_be_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        if offset > self.bytes.len() - 2 {
            return Err(RdramAccessError { offset, width: 2 });
        }

        Ok(())
    }

    pub(crate) fn require_u32_be_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        if offset > self.bytes.len() - 4 {
            return Err(RdramAccessError { offset, width: 4 });
        }

        Ok(())
    }

    pub(crate) fn require_u64_be_offset(&self, offset: usize) -> Result<(), RdramAccessError> {
        if offset > self.bytes.len() - 8 {
            return Err(RdramAccessError { offset, width: 8 });
        }

        Ok(())
    }

    pub(crate) fn write_u8_at_checked_offset(&mut self, offset: usize, value: u8) {
        self.bytes[offset] = value;
    }

    pub(crate) fn write_u16_be_at_checked_offset(&mut self, offset: usize, value: u16) {
        self.bytes[offset] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset + 1] = (value & 0xff) as u8;
    }

    pub(crate) fn write_u32_be_at_checked_offset(&mut self, offset: usize, value: u32) {
        self.bytes[offset] = ((value >> 24) & 0xff) as u8;
        self.bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
        self.bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset + 3] = (value & 0xff) as u8;
    }

    pub(crate) fn write_u64_be_at_checked_offset(&mut self, offset: usize, value: u64) {
        self.bytes[offset] = ((value >> 56) & 0xff) as u8;
        self.bytes[offset + 1] = ((value >> 48) & 0xff) as u8;
        self.bytes[offset + 2] = ((value >> 40) & 0xff) as u8;
        self.bytes[offset + 3] = ((value >> 32) & 0xff) as u8;
        self.bytes[offset + 4] = ((value >> 24) & 0xff) as u8;
        self.bytes[offset + 5] = ((value >> 16) & 0xff) as u8;
        self.bytes[offset + 6] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset + 7] = (value & 0xff) as u8;
    }
}

impl Default for Rdram {
    fn default() -> Self {
        Self {
            bytes: vec![0; RDRAM_SIZE_BYTES],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rdram_has_cpp_construction_size() {
        let rdram = Rdram::default();

        assert_eq!(rdram.size_bytes(), RDRAM_SIZE_BYTES);
        assert_eq!(rdram.size_bytes(), 4 * 1024 * 1024);
    }

    #[test]
    fn default_rdram_storage_is_zero_filled() {
        let rdram = Rdram::default();

        assert!(rdram.bytes.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn byte_read_returns_default_storage_bytes_by_offset_without_mutation() {
        let rdram = Rdram::default();
        let last_offset = RDRAM_SIZE_BYTES - 1;

        assert_eq!(rdram.read_u8(0), Ok(0));
        assert_eq!(rdram.read_u8(last_offset), Ok(0));
        assert_eq!(rdram.size_bytes(), RDRAM_SIZE_BYTES);
        assert!(rdram.bytes.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn byte_read_out_of_range_is_explicit_rust_api_safety() {
        let rdram = Rdram::default();
        let error = rdram.read_u8(RDRAM_SIZE_BYTES).unwrap_err();
        let past_error = rdram.read_u8(RDRAM_SIZE_BYTES + 1).unwrap_err();

        assert_eq!(error.offset(), RDRAM_SIZE_BYTES);
        assert_eq!(error.width(), 1);
        assert_eq!(
            error.to_string(),
            "RDRAM access out of range: address=4194304 width=1"
        );
        assert_eq!(past_error.offset(), RDRAM_SIZE_BYTES + 1);
        assert_eq!(past_error.width(), 1);
        assert_eq!(
            past_error.to_string(),
            "RDRAM access out of range: address=4194305 width=1"
        );
    }

    #[test]
    fn read_width_out_of_range_errors_carry_raw_storage_offset_and_width() {
        let rdram = Rdram::default();

        let u16_error = rdram.read_u16_be(RDRAM_SIZE_BYTES - 1).unwrap_err();
        let u32_error = rdram.read_u32_be(RDRAM_SIZE_BYTES - 3).unwrap_err();
        let u64_error = rdram.read_u64_be(RDRAM_SIZE_BYTES - 7).unwrap_err();

        assert_eq!(u16_error.offset(), RDRAM_SIZE_BYTES - 1);
        assert_eq!(u16_error.width(), 2);
        assert_eq!(
            u16_error.to_string(),
            "RDRAM access out of range: address=4194303 width=2"
        );
        assert_eq!(u32_error.offset(), RDRAM_SIZE_BYTES - 3);
        assert_eq!(u32_error.width(), 4);
        assert_eq!(
            u32_error.to_string(),
            "RDRAM access out of range: address=4194301 width=4"
        );
        assert_eq!(u64_error.offset(), RDRAM_SIZE_BYTES - 7);
        assert_eq!(u64_error.width(), 8);
        assert_eq!(
            u64_error.to_string(),
            "RDRAM access out of range: address=4194297 width=8"
        );
    }
}
