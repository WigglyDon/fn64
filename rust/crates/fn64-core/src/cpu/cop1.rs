use super::address::CpuAddress;
use super::Cpu;
use crate::machine::{MachineBootstrapGprSource, MachineLoadWordTarget};

pub const COP1_FCR31_DEFINED_FIELDS_MASK: u32 = 0x0183_ffff;
pub const COP1_FGR_COUNT: usize = 32;
const COP0_STATUS_FR: u32 = 0x0400_0000;
const COP1_FCR31_ROUNDING_MODE_MASK: u32 = 0x0000_0003;
const COP1_FCR31_INEXACT_FLAG: u32 = 0x0000_0004;
const COP1_FCR31_INEXACT_ENABLE: u32 = 0x0000_0080;
const COP1_FCR31_CAUSE_MASK: u32 = 0x0003_f000;
const COP1_FCR31_INEXACT_CAUSE: u32 = 0x0000_1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1FrMode {
    Fr0,
    Fr1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1DataWordAvailability {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1RoundingMode {
    NearestEven,
    TowardZero,
    TowardPositiveInfinity,
    TowardNegativeInfinity,
}

impl MachineCop1RoundingMode {
    const fn from_fcr31(raw_word: u32) -> Self {
        match raw_word & COP1_FCR31_ROUNDING_MODE_MASK {
            0 => Self::NearestEven,
            1 => Self::TowardZero,
            2 => Self::TowardPositiveInfinity,
            3 => Self::TowardNegativeInfinity,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1Lwc1Provenance {
    instruction_pc: CpuAddress,
    memory_target: MachineLoadWordTarget,
    unavailable_source: Option<MachineBootstrapGprSource>,
}

impl MachineCop1Lwc1Provenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        memory_target: MachineLoadWordTarget,
        unavailable_source: Option<MachineBootstrapGprSource>,
    ) -> Self {
        Self {
            instruction_pc,
            memory_target,
            unavailable_source,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn memory_target(self) -> MachineLoadWordTarget {
        self.memory_target
    }

    pub const fn unavailable_source(self) -> Option<MachineBootstrapGprSource> {
        self.unavailable_source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1Ldc1WordRole {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1Ldc1Provenance {
    instruction_pc: CpuAddress,
    memory_target: MachineLoadWordTarget,
    word_role: MachineCop1Ldc1WordRole,
}

impl MachineCop1Ldc1Provenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        memory_target: MachineLoadWordTarget,
        word_role: MachineCop1Ldc1WordRole,
    ) -> Self {
        Self {
            instruction_pc,
            memory_target,
            word_role,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn memory_target(self) -> MachineLoadWordTarget {
        self.memory_target
    }

    pub const fn word_role(self) -> MachineCop1Ldc1WordRole {
        self.word_role
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1Mtc1Provenance {
    instruction_pc: CpuAddress,
    source_gpr: u8,
    source_lineage: MachineBootstrapGprSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1CvtSingleWordProvenance {
    instruction_pc: CpuAddress,
    source_selector: u8,
    source_kind: MachineCop1DataWordSourceKind,
    source_instruction_pc: Option<CpuAddress>,
    inexact: bool,
}

impl MachineCop1CvtSingleWordProvenance {
    pub(crate) const fn new(
        instruction_pc: CpuAddress,
        source_selector: u8,
        source_kind: MachineCop1DataWordSourceKind,
        source_instruction_pc: Option<CpuAddress>,
        inexact: bool,
    ) -> Self {
        Self {
            instruction_pc,
            source_selector,
            source_kind,
            source_instruction_pc,
            inexact,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn source_selector(self) -> u8 {
        self.source_selector
    }

    pub const fn source_kind(self) -> MachineCop1DataWordSourceKind {
        self.source_kind
    }

    pub const fn source_instruction_pc(self) -> Option<CpuAddress> {
        self.source_instruction_pc
    }

    pub const fn inexact(self) -> bool {
        self.inexact
    }
}

impl MachineCop1Mtc1Provenance {
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
pub enum MachineCop1DataWordSource {
    ConstructionUnavailable,
    Lwc1(MachineCop1Lwc1Provenance),
    Ldc1(MachineCop1Ldc1Provenance),
    Mtc1(MachineCop1Mtc1Provenance),
    CvtSingleWord(MachineCop1CvtSingleWordProvenance),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCop1DataWordSourceKind {
    ConstructionUnavailable,
    Lwc1,
    Ldc1,
    Mtc1,
    CvtSingleWord,
}

impl MachineCop1DataWordSource {
    pub const fn kind(self) -> MachineCop1DataWordSourceKind {
        match self {
            Self::ConstructionUnavailable => MachineCop1DataWordSourceKind::ConstructionUnavailable,
            Self::Lwc1(_) => MachineCop1DataWordSourceKind::Lwc1,
            Self::Ldc1(_) => MachineCop1DataWordSourceKind::Ldc1,
            Self::Mtc1(_) => MachineCop1DataWordSourceKind::Mtc1,
            Self::CvtSingleWord(_) => MachineCop1DataWordSourceKind::CvtSingleWord,
        }
    }

    pub const fn instruction_pc(self) -> Option<CpuAddress> {
        match self {
            Self::ConstructionUnavailable => None,
            Self::Lwc1(provenance) => Some(provenance.instruction_pc()),
            Self::Ldc1(provenance) => Some(provenance.instruction_pc()),
            Self::Mtc1(provenance) => Some(provenance.instruction_pc()),
            Self::CvtSingleWord(provenance) => Some(provenance.instruction_pc()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1DataWordState {
    raw_word: u32,
    availability: MachineCop1DataWordAvailability,
    source: MachineCop1DataWordSource,
}

impl MachineCop1DataWordState {
    pub const fn construction_unavailable() -> Self {
        Self {
            raw_word: 0,
            availability: MachineCop1DataWordAvailability::Unavailable,
            source: MachineCop1DataWordSource::ConstructionUnavailable,
        }
    }

    pub(crate) const fn from_lwc1_available(
        raw_word: u32,
        provenance: MachineCop1Lwc1Provenance,
    ) -> Self {
        Self {
            raw_word,
            availability: MachineCop1DataWordAvailability::Available,
            source: MachineCop1DataWordSource::Lwc1(provenance),
        }
    }

    pub(crate) const fn from_lwc1_unavailable(provenance: MachineCop1Lwc1Provenance) -> Self {
        Self {
            raw_word: 0,
            availability: MachineCop1DataWordAvailability::Unavailable,
            source: MachineCop1DataWordSource::Lwc1(provenance),
        }
    }

    pub(crate) const fn from_ldc1_available(
        raw_word: u32,
        provenance: MachineCop1Ldc1Provenance,
    ) -> Self {
        Self {
            raw_word,
            availability: MachineCop1DataWordAvailability::Available,
            source: MachineCop1DataWordSource::Ldc1(provenance),
        }
    }

    pub(crate) const fn from_mtc1_available(
        raw_word: u32,
        provenance: MachineCop1Mtc1Provenance,
    ) -> Self {
        Self {
            raw_word,
            availability: MachineCop1DataWordAvailability::Available,
            source: MachineCop1DataWordSource::Mtc1(provenance),
        }
    }

    pub(crate) const fn from_mtc1_unavailable(provenance: MachineCop1Mtc1Provenance) -> Self {
        Self {
            raw_word: 0,
            availability: MachineCop1DataWordAvailability::Unavailable,
            source: MachineCop1DataWordSource::Mtc1(provenance),
        }
    }

    pub(crate) const fn from_cvt_single_word_available(
        raw_word: u32,
        provenance: MachineCop1CvtSingleWordProvenance,
    ) -> Self {
        Self {
            raw_word,
            availability: MachineCop1DataWordAvailability::Available,
            source: MachineCop1DataWordSource::CvtSingleWord(provenance),
        }
    }

    pub const fn availability(self) -> MachineCop1DataWordAvailability {
        self.availability
    }

    pub const fn raw_word(self) -> Option<u32> {
        match self.availability {
            MachineCop1DataWordAvailability::Available => Some(self.raw_word),
            MachineCop1DataWordAvailability::Unavailable => None,
        }
    }

    pub const fn source(self) -> MachineCop1DataWordSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1DataWordSummary {
    available_word_count: u8,
    unavailable_word_count: u8,
    construction_unavailable_word_count: u8,
    lwc1_word_count: u8,
    ldc1_word_count: u8,
    mtc1_word_count: u8,
    cvt_single_word_count: u8,
}

impl MachineCop1DataWordSummary {
    pub const fn available_word_count(self) -> u8 {
        self.available_word_count
    }

    pub const fn unavailable_word_count(self) -> u8 {
        self.unavailable_word_count
    }

    pub const fn construction_unavailable_word_count(self) -> u8 {
        self.construction_unavailable_word_count
    }

    pub const fn lwc1_word_count(self) -> u8 {
        self.lwc1_word_count
    }

    pub const fn ldc1_word_count(self) -> u8 {
        self.ldc1_word_count
    }

    pub const fn mtc1_word_count(self) -> u8 {
        self.mtc1_word_count
    }

    pub const fn cvt_single_word_count(self) -> u8 {
        self.cvt_single_word_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCop1CvtSingleWordFcr31Provenance {
    instruction_pc: CpuAddress,
    inexact: bool,
    trapped: bool,
}

impl MachineCop1CvtSingleWordFcr31Provenance {
    pub(crate) const fn new(instruction_pc: CpuAddress, inexact: bool, trapped: bool) -> Self {
        Self {
            instruction_pc,
            inexact,
            trapped,
        }
    }

    pub const fn instruction_pc(self) -> CpuAddress {
        self.instruction_pc
    }

    pub const fn inexact(self) -> bool {
        self.inexact
    }

    pub const fn trapped(self) -> bool {
        self.trapped
    }
}

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
    CleanRoomHleNtscX105Pinned,
    CpuControlTransfer(MachineCop1Fcr31WriteProvenance),
    CvtSingleWord(MachineCop1CvtSingleWordFcr31Provenance),
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

    pub(crate) const fn clean_room_hle_ntsc_x105_pinned() -> Self {
        Self {
            raw_word: 0,
            source: MachineCop1Fcr31Source::CleanRoomHleNtscX105Pinned,
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

    pub(crate) const fn from_cvt_single_word(
        previous: Self,
        inexact: bool,
        provenance: MachineCop1CvtSingleWordFcr31Provenance,
    ) -> Self {
        let mut raw_word = previous.raw_word & !COP1_FCR31_CAUSE_MASK;
        if inexact {
            raw_word |= COP1_FCR31_INEXACT_CAUSE;
            if !provenance.trapped() {
                raw_word |= COP1_FCR31_INEXACT_FLAG;
            }
        }
        Self {
            raw_word,
            source: MachineCop1Fcr31Source::CvtSingleWord(provenance),
        }
    }

    pub const fn raw_word(self) -> u32 {
        self.raw_word
    }

    pub const fn source(self) -> MachineCop1Fcr31Source {
        self.source
    }

    pub const fn rounding_mode(self) -> MachineCop1RoundingMode {
        MachineCop1RoundingMode::from_fcr31(self.raw_word)
    }

    pub const fn inexact_enable(self) -> bool {
        self.raw_word & COP1_FCR31_INEXACT_ENABLE != 0
    }

    pub const fn inexact_cause(self) -> bool {
        self.raw_word & COP1_FCR31_INEXACT_CAUSE != 0
    }

    pub const fn inexact_flag(self) -> bool {
        self.raw_word & COP1_FCR31_INEXACT_FLAG != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CvtSingleWordResult {
    raw_binary32_bits: u32,
    inexact: bool,
}

impl CvtSingleWordResult {
    pub(crate) const fn raw_binary32_bits(self) -> u32 {
        self.raw_binary32_bits
    }

    pub(crate) const fn inexact(self) -> bool {
        self.inexact
    }
}

pub(crate) const fn convert_signed_word_to_binary32(
    source: i32,
    rounding_mode: MachineCop1RoundingMode,
) -> CvtSingleWordResult {
    if source == 0 {
        return CvtSingleWordResult {
            raw_binary32_bits: 0,
            inexact: false,
        };
    }

    let negative = source < 0;
    let magnitude = source.unsigned_abs();
    let highest_set_bit = 31 - magnitude.leading_zeros();
    let mut exponent = highest_set_bit + 127;
    let (mut significand, remainder, discarded_bit_count) = if highest_set_bit <= 23 {
        (magnitude << (23 - highest_set_bit), 0, 0)
    } else {
        let discarded_bit_count = highest_set_bit - 23;
        let remainder_mask = (1_u32 << discarded_bit_count) - 1;
        (
            magnitude >> discarded_bit_count,
            magnitude & remainder_mask,
            discarded_bit_count,
        )
    };
    let inexact = remainder != 0;
    let increment = match rounding_mode {
        MachineCop1RoundingMode::NearestEven if inexact => {
            let half = 1_u32 << (discarded_bit_count - 1);
            remainder > half || (remainder == half && significand & 1 != 0)
        }
        MachineCop1RoundingMode::NearestEven | MachineCop1RoundingMode::TowardZero => false,
        MachineCop1RoundingMode::TowardPositiveInfinity => inexact && !negative,
        MachineCop1RoundingMode::TowardNegativeInfinity => inexact && negative,
    };
    if increment {
        significand += 1;
        if significand == 1 << 24 {
            significand >>= 1;
            exponent += 1;
        }
    }

    CvtSingleWordResult {
        raw_binary32_bits: ((negative as u32) << 31)
            | (exponent << 23)
            | (significand & 0x007f_ffff),
        inexact,
    }
}

pub(super) struct Cop1 {
    fcr31: Option<MachineCop1Fcr31State>,
    data_words: [MachineCop1DataWordState; COP1_FGR_COUNT],
}

impl Cop1 {
    pub(super) const fn new() -> Self {
        Self {
            fcr31: None,
            data_words: [MachineCop1DataWordState::construction_unavailable(); COP1_FGR_COUNT],
        }
    }
}

impl Cpu {
    pub fn cop1_fr_mode(&self) -> MachineCop1FrMode {
        if (self.cop0_status() & COP0_STATUS_FR) == 0 {
            MachineCop1FrMode::Fr0
        } else {
            MachineCop1FrMode::Fr1
        }
    }

    pub fn cop1_data_word_state(&self, selector: usize) -> Option<MachineCop1DataWordState> {
        self.cop1.data_words.get(selector).copied()
    }

    pub fn cop1_data_word_summary(&self) -> MachineCop1DataWordSummary {
        let mut available_word_count = 0_u8;
        let mut construction_unavailable_word_count = 0_u8;
        let mut lwc1_word_count = 0_u8;
        let mut ldc1_word_count = 0_u8;
        let mut mtc1_word_count = 0_u8;
        let mut cvt_single_word_count = 0_u8;
        for state in self.cop1.data_words {
            match state.availability() {
                MachineCop1DataWordAvailability::Available => {
                    available_word_count = available_word_count.saturating_add(1);
                }
                MachineCop1DataWordAvailability::Unavailable => {}
            }
            match state.source() {
                MachineCop1DataWordSource::ConstructionUnavailable => {
                    construction_unavailable_word_count =
                        construction_unavailable_word_count.saturating_add(1);
                }
                MachineCop1DataWordSource::Lwc1(_) => {
                    lwc1_word_count = lwc1_word_count.saturating_add(1);
                }
                MachineCop1DataWordSource::Ldc1(_) => {
                    ldc1_word_count = ldc1_word_count.saturating_add(1);
                }
                MachineCop1DataWordSource::Mtc1(_) => {
                    mtc1_word_count = mtc1_word_count.saturating_add(1);
                }
                MachineCop1DataWordSource::CvtSingleWord(_) => {
                    cvt_single_word_count = cvt_single_word_count.saturating_add(1);
                }
            }
        }
        MachineCop1DataWordSummary {
            available_word_count,
            unavailable_word_count: (COP1_FGR_COUNT as u8).saturating_sub(available_word_count),
            construction_unavailable_word_count,
            lwc1_word_count,
            ldc1_word_count,
            mtc1_word_count,
            cvt_single_word_count,
        }
    }

    pub fn cop1_fcr31_state(&self) -> Option<MachineCop1Fcr31State> {
        self.cop1.fcr31
    }

    pub(crate) fn stage_public_synthetic_cold_x105_fcr31(&mut self) {
        self.cop1.fcr31 = Some(MachineCop1Fcr31State::public_synthetic_cold_x105());
    }

    pub(crate) fn stage_clean_room_hle_fcr31(&mut self) {
        self.cop1.fcr31 = Some(MachineCop1Fcr31State::clean_room_hle_ntsc_x105_pinned());
    }

    pub(crate) fn write_cop1_fcr31(&mut self, state: MachineCop1Fcr31State) {
        self.cop1.fcr31 = Some(state);
    }

    pub(crate) fn write_cop1_data_word(&mut self, selector: u8, state: MachineCop1DataWordState) {
        self.cop1.data_words[usize::from(selector)] = state;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        convert_signed_word_to_binary32, MachineCop1CvtSingleWordFcr31Provenance,
        MachineCop1Fcr31Source, MachineCop1Fcr31State, MachineCop1RoundingMode,
        COP1_FCR31_CAUSE_MASK, COP1_FCR31_DEFINED_FIELDS_MASK, COP1_FCR31_INEXACT_CAUSE,
        COP1_FCR31_INEXACT_ENABLE, COP1_FCR31_INEXACT_FLAG,
    };
    use crate::cpu::address::CpuAddress;

    fn converted(source: i32, mode: MachineCop1RoundingMode) -> (u32, bool) {
        let result = convert_signed_word_to_binary32(source, mode);
        (result.raw_binary32_bits(), result.inexact())
    }

    #[test]
    fn cvt_s_w_integer_only_exact_public_cases_cover_zero_sign_and_range_edges() {
        let rn = MachineCop1RoundingMode::NearestEven;
        let exact_cases = [
            (0, 0x0000_0000),
            (1, 0x3f80_0000),
            (-1, 0xbf80_0000),
            (16_777_216, 0x4b80_0000),
            (-16_777_216, 0xcb80_0000),
            (1 << 30, 0x4e80_0000),
            (i32::MIN, 0xcf00_0000),
        ];
        for (source, expected) in exact_cases {
            assert_eq!(converted(source, rn), (expected, false));
        }
        assert_eq!(converted(i32::MAX, rn), (0x4f00_0000, true));
    }

    #[test]
    fn cvt_s_w_nearest_even_distinguishes_less_half_greater_half_and_ties() {
        let rn = MachineCop1RoundingMode::NearestEven;
        assert_eq!(converted(33_554_433, rn), (0x4c00_0000, true));
        assert_eq!(converted(33_554_435, rn), (0x4c00_0001, true));
        assert_eq!(converted(33_554_434, rn), (0x4c00_0000, true));
        assert_eq!(converted(33_554_438, rn), (0x4c00_0002, true));
        assert_eq!(converted(-33_554_434, rn), (0xcc00_0000, true));
        assert_eq!(converted(-33_554_438, rn), (0xcc00_0002, true));
    }

    #[test]
    fn cvt_s_w_directed_rounding_obeys_sign_for_positive_and_negative_sources() {
        let positive = 16_777_217;
        let negative = -16_777_217;
        assert_eq!(
            converted(positive, MachineCop1RoundingMode::TowardZero),
            (0x4b80_0000, true)
        );
        assert_eq!(
            converted(positive, MachineCop1RoundingMode::TowardPositiveInfinity),
            (0x4b80_0001, true)
        );
        assert_eq!(
            converted(positive, MachineCop1RoundingMode::TowardNegativeInfinity),
            (0x4b80_0000, true)
        );
        assert_eq!(
            converted(negative, MachineCop1RoundingMode::TowardZero),
            (0xcb80_0000, true)
        );
        assert_eq!(
            converted(negative, MachineCop1RoundingMode::TowardPositiveInfinity),
            (0xcb80_0000, true)
        );
        assert_eq!(
            converted(negative, MachineCop1RoundingMode::TowardNegativeInfinity),
            (0xcb80_0001, true)
        );
    }

    #[test]
    fn cvt_s_w_generated_powers_and_exact_trailing_zero_values_never_raise_inexact() {
        let modes = [
            MachineCop1RoundingMode::NearestEven,
            MachineCop1RoundingMode::TowardZero,
            MachineCop1RoundingMode::TowardPositiveInfinity,
            MachineCop1RoundingMode::TowardNegativeInfinity,
        ];
        for exponent in 0..=30 {
            let magnitude = 1_i32 << exponent;
            for mode in modes {
                assert!(!converted(magnitude, mode).1);
                assert!(!converted(-magnitude, mode).1);
            }
        }
        for source in [0x0123_4500_i32, -0x0123_4500_i32, 0x7fff_ff80_i32] {
            for mode in modes {
                assert!(!converted(source, mode).1);
            }
        }
    }

    #[test]
    fn cvt_s_w_fcr31_transition_clears_current_causes_and_applies_only_inexact_sticky_law() {
        let preserved = COP1_FCR31_DEFINED_FIELDS_MASK & !COP1_FCR31_CAUSE_MASK;
        let prior = MachineCop1Fcr31State {
            raw_word: preserved | COP1_FCR31_CAUSE_MASK,
            source: MachineCop1Fcr31Source::CleanRoomHleNtscX105Pinned,
        };
        let exact = MachineCop1Fcr31State::from_cvt_single_word(
            prior,
            false,
            MachineCop1CvtSingleWordFcr31Provenance::new(CpuAddress::new(0x1000), false, false),
        );
        assert_eq!(exact.raw_word(), preserved);

        let prior_untrapped = MachineCop1Fcr31State {
            raw_word: preserved & !COP1_FCR31_INEXACT_FLAG & !COP1_FCR31_INEXACT_ENABLE,
            source: prior.source(),
        };
        let untrapped = MachineCop1Fcr31State::from_cvt_single_word(
            prior_untrapped,
            true,
            MachineCop1CvtSingleWordFcr31Provenance::new(CpuAddress::new(0x1004), true, false),
        );
        assert!(untrapped.inexact_cause());
        assert!(untrapped.inexact_flag());
        assert_eq!(
            untrapped.raw_word() & !(COP1_FCR31_INEXACT_CAUSE | COP1_FCR31_INEXACT_FLAG),
            prior_untrapped.raw_word() & !COP1_FCR31_CAUSE_MASK
        );

        let prior_trapped = MachineCop1Fcr31State {
            raw_word: (preserved | COP1_FCR31_INEXACT_ENABLE) & !COP1_FCR31_INEXACT_FLAG,
            source: prior.source(),
        };
        let trapped = MachineCop1Fcr31State::from_cvt_single_word(
            prior_trapped,
            true,
            MachineCop1CvtSingleWordFcr31Provenance::new(CpuAddress::new(0x1008), true, true),
        );
        assert!(trapped.inexact_cause());
        assert!(!trapped.inexact_flag());
        assert_eq!(
            trapped.raw_word() & !COP1_FCR31_CAUSE_MASK,
            prior_trapped.raw_word() & !COP1_FCR31_CAUSE_MASK
        );

        let prior_trapped_sticky = MachineCop1Fcr31State {
            raw_word: prior_trapped.raw_word() | COP1_FCR31_INEXACT_FLAG,
            source: prior.source(),
        };
        let trapped_sticky = MachineCop1Fcr31State::from_cvt_single_word(
            prior_trapped_sticky,
            true,
            MachineCop1CvtSingleWordFcr31Provenance::new(CpuAddress::new(0x100c), true, true),
        );
        assert!(trapped_sticky.inexact_flag());
        assert_eq!(
            trapped_sticky.raw_word() & !COP1_FCR31_CAUSE_MASK,
            prior_trapped_sticky.raw_word() & !COP1_FCR31_CAUSE_MASK
        );
    }
}
