#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuRdramReservation {
    valid: bool,
    rdram_offset: u32,
    width: usize,
}

impl CpuRdramReservation {
    pub(crate) fn new() -> Self {
        Self {
            valid: false,
            rdram_offset: 0,
            width: 0,
        }
    }

    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "LL/SC staging callers are intentionally not earned yet"
        )
    )]
    pub(super) fn stage(&mut self, rdram_offset: u32, width: usize) {
        self.valid = true;
        self.rdram_offset = rdram_offset;
        self.width = width;
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    pub(super) fn invalidate_for_rdram_write(&mut self, rdram_offset: u32, width: usize) {
        if !self.valid || width == 0 {
            return;
        }

        if self.overlaps_rdram_range(rdram_offset, width) {
            self.clear();
        }
    }

    fn overlaps_rdram_range(&self, rdram_offset: u32, width: usize) -> bool {
        let write_begin = u64::from(rdram_offset);
        let write_end = write_begin.wrapping_add(width as u64);
        let reservation_begin = u64::from(self.rdram_offset);
        let reservation_end = reservation_begin.wrapping_add(self.width as u64);

        write_begin < reservation_end && reservation_begin < write_end
    }

    #[cfg(test)]
    pub(crate) fn is_valid(&self) -> bool {
        self.valid
    }

    #[cfg(test)]
    pub(crate) fn rdram_offset(&self) -> u32 {
        self.rdram_offset
    }

    #[cfg(test)]
    pub(crate) fn width(&self) -> usize {
        self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_reservation(
        reservation: &CpuRdramReservation,
        valid: bool,
        rdram_offset: u32,
        width: usize,
    ) {
        assert_eq!(reservation.is_valid(), valid);
        assert_eq!(reservation.rdram_offset(), rdram_offset);
        assert_eq!(reservation.width(), width);
    }

    #[test]
    fn reservation_construction_matches_cpp_cleared_state() {
        let reservation = CpuRdramReservation::new();

        assert_reservation(&reservation, false, 0, 0);
    }

    #[test]
    fn staging_matches_cpp_set_cpu_rdram_reservation_assignments() {
        let mut reservation = CpuRdramReservation::new();

        reservation.stage(0x0012_3456, 4);
        assert!(reservation.is_valid());
        assert_eq!(reservation.rdram_offset(), 0x0012_3456);
        assert_eq!(reservation.width(), 4);

        reservation.stage(0x003f_fffc, 0);
        assert!(reservation.is_valid());
        assert_eq!(reservation.rdram_offset(), 0x003f_fffc);
        assert_eq!(reservation.width(), 0);
    }

    #[test]
    fn repeated_staging_overwrites_previous_reservation_fields() {
        let mut reservation = CpuRdramReservation::new();

        reservation.stage(0x0000_0100, 4);
        reservation.stage(0x0000_0200, 8);

        assert!(reservation.is_valid());
        assert_eq!(reservation.rdram_offset(), 0x0000_0200);
        assert_eq!(reservation.width(), 8);
    }

    #[test]
    fn invalidation_noops_for_invalid_reservation_and_zero_width_write() {
        let mut reservation = CpuRdramReservation::new();

        reservation.invalidate_for_rdram_write(0x0000_0100, 4);
        assert_reservation(&reservation, false, 0, 0);

        reservation.stage(0x0000_0100, 4);
        reservation.invalidate_for_rdram_write(0x0000_0100, 0);
        assert_reservation(&reservation, true, 0x0000_0100, 4);
    }

    #[test]
    fn invalidation_preserves_valid_non_overlapping_reservation() {
        let mut reservation = CpuRdramReservation::new();

        reservation.stage(0x0000_0100, 4);
        reservation.invalidate_for_rdram_write(0x0000_0200, 4);

        assert_reservation(&reservation, true, 0x0000_0100, 4);
    }

    #[test]
    fn invalidation_clears_overlapping_valid_reservation() {
        let mut reservation = CpuRdramReservation::new();

        reservation.stage(0x0000_0100, 4);
        reservation.invalidate_for_rdram_write(0x0000_0102, 1);

        assert_reservation(&reservation, false, 0, 0);
    }

    #[test]
    fn invalidation_uses_cpp_half_open_boundary_rules() {
        let mut reservation = CpuRdramReservation::new();

        reservation.stage(0x0000_0100, 4);
        reservation.invalidate_for_rdram_write(0x0000_00fc, 4);
        assert_reservation(&reservation, true, 0x0000_0100, 4);

        reservation.invalidate_for_rdram_write(0x0000_0104, 4);
        assert_reservation(&reservation, true, 0x0000_0100, 4);

        reservation.invalidate_for_rdram_write(0x0000_00ff, 2);
        assert_reservation(&reservation, false, 0, 0);

        reservation.stage(0x0000_0100, 4);
        reservation.invalidate_for_rdram_write(0x0000_0103, 1);
        assert_reservation(&reservation, false, 0, 0);
    }

    #[test]
    fn invalidation_clears_contained_and_exact_overlapping_ranges() {
        let mut reservation = CpuRdramReservation::new();

        reservation.stage(0x0000_0100, 8);
        reservation.invalidate_for_rdram_write(0x0000_0102, 2);
        assert_reservation(&reservation, false, 0, 0);

        reservation.stage(0x0000_0100, 4);
        reservation.invalidate_for_rdram_write(0x0000_00f0, 0x20);
        assert_reservation(&reservation, false, 0, 0);

        reservation.stage(0x0000_0100, 4);
        reservation.invalidate_for_rdram_write(0x0000_0100, 4);
        assert_reservation(&reservation, false, 0, 0);
    }

    #[test]
    fn invalidation_uses_latest_staged_reservation_and_cleared_state_stays_invalid() {
        let mut reservation = CpuRdramReservation::new();

        reservation.stage(0x0000_0100, 4);
        reservation.stage(0x0000_0200, 4);
        reservation.invalidate_for_rdram_write(0x0000_0100, 4);
        assert_reservation(&reservation, true, 0x0000_0200, 4);

        reservation.invalidate_for_rdram_write(0x0000_0202, 1);
        assert_reservation(&reservation, false, 0, 0);

        reservation.invalidate_for_rdram_write(0x0000_0200, 4);
        assert_reservation(&reservation, false, 0, 0);
    }
}
