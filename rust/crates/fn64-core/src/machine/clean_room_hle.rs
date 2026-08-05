use core::fmt;

use crate::ai::Ai;
use crate::cartridge::{CartridgeReadError, RomSourceLayout};
use crate::cpu::address::CpuAddress;
use crate::cpu::{
    Cpu, CpuRegisterIndexError, MachinePrimaryCacheCleanRoomHleSource, CPU_GPR_COUNT,
};
use crate::dpc::Dpc;
use crate::mi::Mi;
use crate::pi::Pi;
use crate::rdram::{MachineRdramCartridgeStagingState, Rdram, RdramAccessError};
use crate::ri::Ri;
use crate::si::Si;
use crate::sp_dmem::SpDmem;
use crate::sp_imem::SpImem;
use crate::vi::Vi;

use super::rdram_reservation::CpuRdramReservation;
use super::{
    Machine, MachineBootstrapCop0StatusSource, MachineBootstrapCpuStateKind,
    MachineBootstrapGprSource, MachineStepProcessor,
};

pub const MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_START_OFFSET: u32 = 0x0000_1000;
pub const MACHINE_CLEAN_ROOM_CARTRIDGE_BYTE_COUNT: u32 = 0x0010_0000;
pub const MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_END_OFFSET_EXCLUSIVE: u32 =
    MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_START_OFFSET + MACHINE_CLEAN_ROOM_CARTRIDGE_BYTE_COUNT;
const MACHINE_CLEAN_ROOM_SIDE_DATA_START_OFFSET: u32 = 0x0000_0750;
const MACHINE_CLEAN_ROOM_SIDE_DATA_BYTE_COUNT: usize = 0x100;
const MACHINE_CLEAN_ROOM_NTSC_X105_SEED: u32 = 0x91;
const MACHINE_CLEAN_ROOM_X105_MULTIPLIER: u32 = 0x5d58_8b65;
const MACHINE_CLEAN_ROOM_COP0_STATUS: u32 = 0x3400_0000;
const MACHINE_CLEAN_ROOM_COP0_PAGE_MASK: u32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineCleanRoomBootProfile {
    NtscX105Pinned,
}

impl MachineCleanRoomBootProfile {
    pub const fn name(self) -> &'static str {
        match self {
            Self::NtscX105Pinned => "ntsc-x105-pinned",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineBootSource {
    CleanRoomHle {
        profile: MachineCleanRoomBootProfile,
    },
    ExplicitPifFirmware,
    PublicSyntheticProof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineCleanRoomHleState {
    profile: MachineCleanRoomBootProfile,
    source_layout: RomSourceLayout,
    cartridge_start_offset: u32,
    cartridge_end_offset_exclusive: u32,
    rdram_start_offset: u32,
    rdram_end_offset_exclusive: u32,
    execution_pc: CpuAddress,
    next_pc: CpuAddress,
    cpu_state_kind: MachineBootstrapCpuStateKind,
    cop0_status_source: MachineBootstrapCop0StatusSource,
    gpr_sources: [MachineBootstrapGprSource; CPU_GPR_COUNT],
}

impl MachineCleanRoomHleState {
    pub const fn profile(self) -> MachineCleanRoomBootProfile {
        self.profile
    }

    pub const fn source_layout(self) -> RomSourceLayout {
        self.source_layout
    }

    pub const fn cartridge_start_offset(self) -> u32 {
        self.cartridge_start_offset
    }

    pub const fn cartridge_end_offset_exclusive(self) -> u32 {
        self.cartridge_end_offset_exclusive
    }

    pub const fn rdram_start_offset(self) -> u32 {
        self.rdram_start_offset
    }

    pub const fn rdram_end_offset_exclusive(self) -> u32 {
        self.rdram_end_offset_exclusive
    }

    pub const fn execution_pc(self) -> CpuAddress {
        self.execution_pc
    }

    pub const fn next_pc(self) -> CpuAddress {
        self.next_pc
    }

    pub const fn cpu_state_kind(self) -> MachineBootstrapCpuStateKind {
        self.cpu_state_kind
    }

    pub const fn cop0_status_source(self) -> MachineBootstrapCop0StatusSource {
        self.cop0_status_source
    }

    pub fn gpr_source(self, index: usize) -> Option<MachineBootstrapGprSource> {
        self.gpr_sources.get(index).copied()
    }

    pub fn gpr_is_known(self, index: usize) -> Option<bool> {
        self.gpr_source(index)
            .map(MachineBootstrapGprSource::is_known)
    }

    pub(super) fn record_gpr_source(
        &mut self,
        register_index: u8,
        source: MachineBootstrapGprSource,
    ) {
        if register_index == 0 {
            self.gpr_sources[0] = MachineBootstrapGprSource::ArchitecturalZero;
        } else {
            self.gpr_sources[usize::from(register_index)] = source;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineCleanRoomHleError {
    ConflictingPifBootInputs,
    CartridgeSourceRangeUnavailable {
        required_end_offset_exclusive: u32,
        actual_size_bytes: usize,
    },
    CartridgeRead(CartridgeReadError),
    UnsupportedEntryAddress {
        entry_address: u32,
    },
    RdramDestinationRangeUnavailable {
        start_offset: u32,
        byte_count: u32,
    },
    Rdram(RdramAccessError),
    CpuRegister(CpuRegisterIndexError),
}

impl fmt::Display for MachineCleanRoomHleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictingPifBootInputs => f.write_str(
                "clean-room cartridge-entry staging rejects installed PIF boot inputs",
            ),
            Self::CartridgeSourceRangeUnavailable {
                required_end_offset_exclusive,
                actual_size_bytes,
            } => write!(
                f,
                "clean-room cartridge source range unavailable: required_end={} actual_size={}",
                required_end_offset_exclusive, actual_size_bytes
            ),
            Self::CartridgeRead(error) => {
                write!(f, "clean-room cartridge source read rejected: {error}")
            }
            Self::UnsupportedEntryAddress { entry_address } => write!(
                f,
                "clean-room cartridge entry address unsupported by pinned profile: address=0x{entry_address:08X}"
            ),
            Self::RdramDestinationRangeUnavailable {
                start_offset,
                byte_count,
            } => write!(
                f,
                "clean-room RDRAM destination unavailable: start={} width={}",
                start_offset, byte_count
            ),
            Self::Rdram(error) => write!(f, "clean-room RDRAM staging rejected: {error}"),
            Self::CpuRegister(error) => {
                write!(f, "clean-room CPU register staging rejected: {error}")
            }
        }
    }
}

impl std::error::Error for MachineCleanRoomHleError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CleanRoomCpuPlan {
    gprs: [u64; CPU_GPR_COUNT],
    gpr_sources: [MachineBootstrapGprSource; CPU_GPR_COUNT],
    hi: u64,
    lo: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CleanRoomChecksumState {
    last_word: u32,
    last_rotate: u32,
    sum: u32,
    carry: u32,
    conditional: u32,
    side_sum: u32,
    xor: u32,
    rotate_sum: u32,
}

impl Machine {
    /// Stages one atomic, firmware-free cartridge-entry handoff.
    ///
    /// This transition does not execute PIF, IPL2, IPL3, CPU, or RSP code. It
    /// consumes the normalized Cartridge and one explicit public profile,
    /// materializes only the profile's cartridge-derived payload and public
    /// post-boot CPU state, and leaves boot-local SP memories unavailable.
    pub fn stage_clean_room_cartridge_entry(
        &mut self,
        profile: MachineCleanRoomBootProfile,
    ) -> Result<MachineCleanRoomHleState, MachineCleanRoomHleError> {
        if self.pif_firmware.is_some()
            || self.pif_ipl2_profile.is_some()
            || self.pif_ipl3_family.is_some()
            || self.pif_ipl2_handoff_reset_kind.is_some()
            || self.pif_ipl2_handoff_boot_medium.is_some()
            || self.pif_version_bit.is_some()
        {
            return Err(MachineCleanRoomHleError::ConflictingPifBootInputs);
        }

        let required_end = MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_END_OFFSET_EXCLUSIVE;
        if self.cartridge.size_bytes() < required_end as usize {
            return Err(MachineCleanRoomHleError::CartridgeSourceRangeUnavailable {
                required_end_offset_exclusive: required_end,
                actual_size_bytes: self.cartridge.size_bytes(),
            });
        }

        let entry_address = self.cartridge.metadata().entry_point;
        if !(0x8000_0000..=0x9fff_ffff).contains(&entry_address) {
            return Err(MachineCleanRoomHleError::UnsupportedEntryAddress { entry_address });
        }
        let rdram_start_offset = entry_address & 0x1fff_ffff;
        let Some(rdram_end_offset_exclusive) =
            rdram_start_offset.checked_add(MACHINE_CLEAN_ROOM_CARTRIDGE_BYTE_COUNT)
        else {
            return Err(MachineCleanRoomHleError::RdramDestinationRangeUnavailable {
                start_offset: rdram_start_offset,
                byte_count: MACHINE_CLEAN_ROOM_CARTRIDGE_BYTE_COUNT,
            });
        };
        if rdram_end_offset_exclusive as usize > crate::rdram::RDRAM_SIZE_BYTES {
            return Err(MachineCleanRoomHleError::RdramDestinationRangeUnavailable {
                start_offset: rdram_start_offset,
                byte_count: MACHINE_CLEAN_ROOM_CARTRIDGE_BYTE_COUNT,
            });
        }

        let payload = read_cartridge_span(
            &self.cartridge,
            MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_START_OFFSET,
            MACHINE_CLEAN_ROOM_CARTRIDGE_BYTE_COUNT as usize,
        )?;
        let side_data = read_cartridge_span(
            &self.cartridge,
            MACHINE_CLEAN_ROOM_SIDE_DATA_START_OFFSET,
            MACHINE_CLEAN_ROOM_SIDE_DATA_BYTE_COUNT,
        )?;
        let side_data: [u8; MACHINE_CLEAN_ROOM_SIDE_DATA_BYTE_COUNT] = side_data
            .try_into()
            .expect("preflighted side-data span has one exact public-profile width");
        let cpu_plan = clean_room_cpu_plan(profile, entry_address, &payload, &side_data);
        let replacement_pi =
            Pi::clean_room_hle_cartridge_entry(self.cartridge.pi_domain_one_timing());
        let replacement_sp = match profile {
            MachineCleanRoomBootProfile::NtscX105Pinned => {
                crate::sp::Sp::clean_room_ntsc_x105_post_boot()
            }
        };

        let replacement_rdram = Rdram::from_clean_room_hle_cartridge_payload(
            rdram_start_offset,
            &payload,
            MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_START_OFFSET,
        )
        .map_err(MachineCleanRoomHleError::Rdram)?;
        let mut replacement_cpu = Cpu::new();
        for (index, value) in cpu_plan.gprs.into_iter().enumerate() {
            replacement_cpu
                .set_gpr(index, value)
                .map_err(MachineCleanRoomHleError::CpuRegister)?;
        }
        replacement_cpu.stage_hi(cpu_plan.hi);
        replacement_cpu.stage_lo(cpu_plan.lo);
        replacement_cpu.stage_clean_room_cartridge_entry_cop0(
            MACHINE_CLEAN_ROOM_COP0_STATUS,
            MACHINE_CLEAN_ROOM_COP0_PAGE_MASK,
        );
        replacement_cpu.stage_clean_room_hle_fcr31();
        replacement_cpu.stage_clean_room_hle_primary_caches(
            MachinePrimaryCacheCleanRoomHleSource::NtscX105Pinned,
        );
        replacement_cpu.stage_pc(entry_address);

        let state = MachineCleanRoomHleState {
            profile,
            source_layout: self.cartridge.source_layout(),
            cartridge_start_offset: MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_START_OFFSET,
            cartridge_end_offset_exclusive:
                MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_END_OFFSET_EXCLUSIVE,
            rdram_start_offset,
            rdram_end_offset_exclusive,
            execution_pc: CpuAddress::new(entry_address),
            next_pc: CpuAddress::new(entry_address.wrapping_add(4)),
            cpu_state_kind: MachineBootstrapCpuStateKind::CleanRoomCartridgeEntryNtscX105Pinned,
            cop0_status_source: MachineBootstrapCop0StatusSource::CleanRoomHlePublicProfile,
            gpr_sources: cpu_plan.gpr_sources,
        };

        self.cpu = replacement_cpu;
        self.rdram = replacement_rdram;
        self.sp_dmem = SpDmem::default();
        *self.sp_imem = SpImem::default();
        self.sp = replacement_sp;
        self.dpc = Dpc::default();
        self.ri = Ri::default();
        self.mi = Mi::default();
        self.pi = replacement_pi;
        self.si = Si::default();
        self.ai = Ai::default();
        self.vi = Vi::default();
        self.processor_turn = MachineStepProcessor::Cpu;
        self.cpu_rdram_reservation = CpuRdramReservation::new();
        self.cartridge_bootstrap = None;
        self.clean_room_hle = Some(state);
        self.powered_on = true;

        Ok(state)
    }

    pub const fn clean_room_hle_state(&self) -> Option<MachineCleanRoomHleState> {
        self.clean_room_hle
    }

    pub fn boot_source(&self) -> Option<MachineBootSource> {
        if let Some(state) = self.clean_room_hle {
            return Some(MachineBootSource::CleanRoomHle {
                profile: state.profile(),
            });
        }
        match self.pif_firmware_state().classification() {
            Some(crate::pif_firmware::PifFirmwareClassification::RawBootRom) => {
                Some(MachineBootSource::ExplicitPifFirmware)
            }
            Some(
                crate::pif_firmware::PifFirmwareClassification::PublicSyntheticColdX105Bootstrap,
            ) => Some(MachineBootSource::PublicSyntheticProof),
            None => None,
        }
    }

    pub const fn rdram_cartridge_staging_state(&self) -> Option<MachineRdramCartridgeStagingState> {
        self.rdram.cartridge_staging_state()
    }
}

fn read_cartridge_span(
    cartridge: &crate::cartridge::Cartridge,
    start_offset: u32,
    byte_count: usize,
) -> Result<Vec<u8>, MachineCleanRoomHleError> {
    let mut bytes = Vec::with_capacity(byte_count);
    for index in 0..byte_count {
        bytes.push(
            cartridge
                .read_u8(start_offset + index as u32)
                .map_err(MachineCleanRoomHleError::CartridgeRead)?,
        );
    }
    Ok(bytes)
}

fn clean_room_cpu_plan(
    _profile: MachineCleanRoomBootProfile,
    entry_address: u32,
    payload: &[u8],
    side_data: &[u8; MACHINE_CLEAN_ROOM_SIDE_DATA_BYTE_COUNT],
) -> CleanRoomCpuPlan {
    let product = u64::from(MACHINE_CLEAN_ROOM_NTSC_X105_SEED)
        * u64::from(MACHINE_CLEAN_ROOM_X105_MULTIPLIER);
    let hi = product >> 32;
    let lo_word = product as u32;
    let initial = lo_word.wrapping_add(1);
    let checksum = clean_room_checksum_state(payload, initial, side_data);

    let mut gprs = [0_u64; CPU_GPR_COUNT];
    gprs[1] = sign_extend_word(0xa460_0000);
    gprs[2] = sign_extend_word(checksum.last_word);
    gprs[3] = u64::from(checksum.last_word & 0x1f);
    gprs[4] = u64::from(checksum.last_rotate);
    gprs[5] = u64::from(checksum.sum);
    gprs[6] = sign_extend_word(checksum.conditional);
    gprs[7] = sign_extend_word(checksum.sum ^ checksum.carry ^ checksum.xor);
    gprs[8] = sign_extend_word(0xa400_2000);
    gprs[9] = sign_extend_word(entry_address);
    gprs[10] = sign_extend_word(checksum.carry);
    gprs[11] = sign_extend_word(0xb000_0000);
    gprs[12] = sign_extend_word(checksum.side_sum);
    gprs[13] = 0x20;
    gprs[14] = sign_extend_word(checksum.sum ^ checksum.carry);
    gprs[15] = sign_extend_word(0xa000_02ff);
    gprs[16] = 0x400;
    gprs[17] = sign_extend_word(0xa3f0_8000);
    gprs[18] = 0x0018_0000;
    gprs[19] = 0;
    gprs[20] = 1;
    gprs[21] = 0;
    gprs[22] = sign_extend_word(0xa000_0200);
    gprs[23] = 0;
    gprs[24] = sign_extend_word(checksum.rotate_sum ^ checksum.conditional);
    gprs[25] = sign_extend_word(checksum.xor);
    gprs[26] = sign_extend_word(0xa430_0000);
    gprs[27] = 1;
    gprs[28] = 8;
    gprs[29] = sign_extend_word(0xa400_1ff0);
    gprs[30] = sign_extend_word(0xa400_1f90);
    gprs[31] = sign_extend_word(0xa400_0324);

    let mut gpr_sources = [MachineBootstrapGprSource::CleanRoomHlePublicProfile; CPU_GPR_COUNT];
    gpr_sources[0] = MachineBootstrapGprSource::ArchitecturalZero;
    gpr_sources[9] = MachineBootstrapGprSource::CleanRoomHleCartridgeEntry;
    for index in [2_usize, 3, 4, 5, 6, 7, 10, 12, 14, 24, 25] {
        gpr_sources[index] = MachineBootstrapGprSource::CleanRoomHleCartridgePayload;
    }

    CleanRoomCpuPlan {
        gprs,
        gpr_sources,
        hi,
        lo: sign_extend_word(lo_word),
    }
}

fn clean_room_checksum_state(
    payload: &[u8],
    initial: u32,
    side_data: &[u8; MACHINE_CLEAN_ROOM_SIDE_DATA_BYTE_COUNT],
) -> CleanRoomChecksumState {
    debug_assert_eq!(
        payload.len(),
        MACHINE_CLEAN_ROOM_CARTRIDGE_BYTE_COUNT as usize
    );
    let mut sum = initial;
    let mut carry = initial;
    let mut xor = initial;
    let mut rotate_sum = initial;
    let mut conditional = initial;
    let mut side_sum = initial;
    let mut last_word = 0;
    let mut last_rotate = 0;

    for (index, bytes) in payload.chunks_exact(4).enumerate() {
        let word = u32::from_be_bytes(bytes.try_into().expect("word chunk width is exact"));
        let next_sum = sum.wrapping_add(word);
        if next_sum < sum {
            carry = carry.wrapping_add(1);
        }
        let rotated = word.rotate_left(word & 0x1f);
        sum = next_sum;
        xor ^= word;
        rotate_sum = rotate_sum.wrapping_add(rotated);
        if conditional < word {
            conditional ^= sum ^ word;
        } else {
            conditional ^= rotated;
        }
        let side_offset = (index * 4) & 0xff;
        let side_word = u32::from_be_bytes(
            side_data[side_offset..side_offset + 4]
                .try_into()
                .expect("wrapped side-data word width is exact"),
        );
        side_sum = side_sum.wrapping_add(word ^ side_word);
        last_word = word;
        last_rotate = rotated;
    }

    CleanRoomChecksumState {
        last_word,
        last_rotate,
        sum,
        carry,
        conditional,
        side_sum,
        xor,
        rotate_sum,
    }
}

const fn sign_extend_word(value: u32) -> u64 {
    if value & 0x8000_0000 == 0 {
        value as u64
    } else {
        0xffff_ffff_0000_0000 | value as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::{load_cartridge, Cartridge, CartridgePiDomain1Timing};
    use crate::cpu::CpuInstructionIdentity;
    use crate::pif_firmware::{MachinePifFirmwareState, PIF_BOOT_ROM_SIZE_BYTES};
    use crate::rdram::{
        MachineRdramCartridgeStagingCause, MachineRdramInitializationSource, RDRAM_SIZE_BYTES,
    };
    use crate::{
        MachineCop1ControlTransferKind, MachineCop1Fcr31Source, MachineCop1Fcr31State,
        MachineLoadWordTarget, MachinePiDomain, MachinePiDomainTimingField,
        MachinePiDomainTimingRegister, MachinePiDomainTimingRegisterState,
        MachinePiDomainTimingSource, MachineRepresentedStepOutcome,
    };

    const GENERATED_ENTRY: u32 = 0x8000_1000;
    const GENERATED_ENTRY_INSTRUCTION: u32 = 0x2402_0042;

    fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }

    fn generated_cartridge(entry: u32, payload_variant: u32) -> Vec<u8> {
        let mut bytes = vec![0; MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_END_OFFSET_EXCLUSIVE as usize];
        write_be_u32(&mut bytes, 0x00, 0x8037_1240);
        write_be_u32(&mut bytes, 0x04, 0x0102_0304);
        write_be_u32(&mut bytes, 0x08, entry);
        write_be_u32(&mut bytes, 0x0c, 0x0506_0708);
        write_be_u32(&mut bytes, 0x10, 0x1112_1314);
        write_be_u32(&mut bytes, 0x14, 0x1516_1718);
        bytes[0x20..0x34].copy_from_slice(b"FN64 CLEAN ROOM HLE ");
        bytes[0x3c] = b'H';
        bytes[0x3d] = b'L';
        bytes[0x3e] = 0x45;
        bytes[0x3f] = 1;

        for (index, word_bytes) in bytes
            [MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_START_OFFSET as usize..]
            .chunks_exact_mut(4)
            .enumerate()
        {
            let index = index as u32;
            let word =
                index.wrapping_mul(0x045d_9f3b).rotate_left(7) ^ 0x9e37_79b9 ^ payload_variant;
            word_bytes.copy_from_slice(&word.to_be_bytes());
        }
        write_be_u32(
            &mut bytes,
            MACHINE_CLEAN_ROOM_CARTRIDGE_SOURCE_START_OFFSET as usize,
            GENERATED_ENTRY_INSTRUCTION,
        );
        bytes
    }

    fn generated_machine(entry: u32, payload_variant: u32) -> Machine {
        let cartridge = load_cartridge(generated_cartridge(entry, payload_variant))
            .expect("generated cartridge should normalize");
        Machine::from_cartridge(cartridge)
    }

    fn generated_machine_with_pi_timing(
        entry: u32,
        payload_variant: u32,
        timing: CartridgePiDomain1Timing,
    ) -> Machine {
        let cartridge = load_cartridge(generated_cartridge(entry, payload_variant))
            .expect("generated cartridge should normalize")
            .with_generated_public_pi_domain_one_timing(timing);
        Machine::from_cartridge(cartridge)
    }

    const fn domain_one_register(
        field: MachinePiDomainTimingField,
    ) -> MachinePiDomainTimingRegister {
        MachinePiDomainTimingRegister::new(MachinePiDomain::One, field)
    }

    fn domain_one_timing_states(
        machine: &Machine,
    ) -> [Option<MachinePiDomainTimingRegisterState>; 4] {
        [
            machine.pi_domain_timing_register_state(domain_one_register(
                MachinePiDomainTimingField::Latency,
            )),
            machine.pi_domain_timing_register_state(domain_one_register(
                MachinePiDomainTimingField::PulseWidth,
            )),
            machine.pi_domain_timing_register_state(domain_one_register(
                MachinePiDomainTimingField::PageSize,
            )),
            machine.pi_domain_timing_register_state(domain_one_register(
                MachinePiDomainTimingField::Release,
            )),
        ]
    }

    fn gprs(machine: &Machine) -> [u64; CPU_GPR_COUNT] {
        core::array::from_fn(|index| {
            machine
                .cpu()
                .gpr(index)
                .expect("architectural GPR index is in range")
        })
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RejectionSnapshot {
        gprs: [u64; CPU_GPR_COUNT],
        pc: u32,
        next_pc: u32,
        hi: u64,
        lo: u64,
        count: u32,
        status: u32,
        cause: Option<u32>,
        fcr31: Option<MachineCop1Fcr31State>,
        processor: MachineStepProcessor,
        boot_source: Option<MachineBootSource>,
        clean_room: Option<MachineCleanRoomHleState>,
        rdram_staging: Option<MachineRdramCartridgeStagingState>,
        first_rdram_word: u32,
        rdram_initialization_source: Option<MachineRdramInitializationSource>,
        sp_status_present: bool,
        sp_pc_present: bool,
        rsp_scalar_registers:
            [crate::rsp::MachineRspScalarRegisterState; crate::rsp::RSP_SCALAR_REGISTER_COUNT],
        rsp_committed: u64,
        first_sp_imem_word_present: bool,
        pi_domain_one_timing: [Option<MachinePiDomainTimingRegisterState>; 4],
    }

    fn rejection_snapshot(machine: &Machine) -> RejectionSnapshot {
        RejectionSnapshot {
            gprs: gprs(machine),
            pc: machine.cpu().pc(),
            next_pc: machine.cpu().next_pc(),
            hi: machine.cpu().hi(),
            lo: machine.cpu().lo(),
            count: machine.cpu().cop0_count(),
            status: machine.cpu().cop0_status(),
            cause: machine.cpu().cop0_cause_word(),
            fcr31: machine.cpu().cop1_fcr31_state(),
            processor: machine.processor_turn(),
            boot_source: machine.boot_source(),
            clean_room: machine.clean_room_hle_state(),
            rdram_staging: machine.rdram_cartridge_staging_state(),
            first_rdram_word: machine.rdram().read_u32_be(0).unwrap(),
            rdram_initialization_source: machine.rdram_initialization_source(),
            sp_status_present: machine.sp_status_state().is_some(),
            sp_pc_present: machine.sp_pc_state().is_some(),
            rsp_scalar_registers: core::array::from_fn(|index| {
                machine
                    .rsp_scalar_register(index)
                    .expect("all architectural RSP scalar-register slots exist")
            }),
            rsp_committed: machine.rsp_committed_instruction_count(),
            first_sp_imem_word_present: machine.sp_imem_opaque_word_state(0).is_some(),
            pi_domain_one_timing: domain_one_timing_states(machine),
        }
    }

    #[test]
    fn clean_room_hle_stages_exact_public_profile_and_cartridge_lineage() {
        let cartridge_bytes = generated_cartridge(GENERATED_ENTRY, 0);
        let cartridge = load_cartridge(cartridge_bytes.clone()).unwrap();
        let mut machine = Machine::from_cartridge(cartridge);
        assert_eq!(machine.cpu().cop0_page_mask(), None);

        let state = machine
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();

        assert_eq!(state.profile(), MachineCleanRoomBootProfile::NtscX105Pinned);
        assert_eq!(state.source_layout(), RomSourceLayout::BigEndian);
        assert_eq!(state.cartridge_start_offset(), 0x1000);
        assert_eq!(state.cartridge_end_offset_exclusive(), 0x0010_1000);
        assert_eq!(state.rdram_start_offset(), 0x1000);
        assert_eq!(state.rdram_end_offset_exclusive(), 0x0010_1000);
        assert_eq!(state.execution_pc(), CpuAddress::new(GENERATED_ENTRY));
        assert_eq!(state.next_pc(), CpuAddress::new(GENERATED_ENTRY + 4));
        assert_eq!(machine.clean_room_hle_state(), Some(state));
        assert_eq!(
            machine.boot_source(),
            Some(MachineBootSource::CleanRoomHle {
                profile: MachineCleanRoomBootProfile::NtscX105Pinned,
            })
        );
        assert_eq!(
            machine.pif_firmware_state(),
            MachinePifFirmwareState::Absent
        );
        assert_eq!(machine.cartridge_bootstrap_state(), None);
        assert_eq!(
            machine.rdram_initialization_source(),
            Some(MachineRdramInitializationSource::CleanRoomHleNtscX105Pinned)
        );
        assert!(machine.rdram_initialization_complete());

        let staging = machine.rdram_cartridge_staging_state().unwrap();
        assert_eq!(staging.cartridge_start_offset(), 0x1000);
        assert_eq!(staging.cartridge_end_offset_exclusive(), 0x0010_1000);
        assert_eq!(staging.rdram_start_offset(), 0x1000);
        assert_eq!(staging.rdram_end_offset_exclusive(), 0x0010_1000);
        assert_eq!(staging.byte_count(), 0x0010_0000);
        assert_eq!(
            staging.cause(),
            MachineRdramCartridgeStagingCause::CleanRoomHle
        );
        for offset in [0_u32, 4, 0x1234, 0x000f_fffc] {
            assert_eq!(
                machine
                    .rdram()
                    .read_u32_be((staging.rdram_start_offset() + offset) as usize)
                    .unwrap(),
                u32::from_be_bytes(
                    cartridge_bytes[(staging.cartridge_start_offset() + offset) as usize
                        ..(staging.cartridge_start_offset() + offset + 4) as usize]
                        .try_into()
                        .unwrap()
                )
            );
        }

        assert_eq!(machine.cpu().pc(), GENERATED_ENTRY);
        assert_eq!(machine.cpu().next_pc(), GENERATED_ENTRY + 4);
        assert_eq!(machine.cpu_delay_slot_context(), None);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().hi(), 0x34);
        assert_eq!(machine.cpu().lo(), 0xffff_ffff_df26_f435);
        assert_eq!(machine.cpu().cop0_count(), 0);
        assert_eq!(machine.cpu().cop0_compare(), 0);
        assert_eq!(machine.cpu().cop0_status(), MACHINE_CLEAN_ROOM_COP0_STATUS);
        assert_eq!(
            machine.cpu().cop0_page_mask(),
            Some(MACHINE_CLEAN_ROOM_COP0_PAGE_MASK)
        );
        assert_eq!(machine.cpu().cop0_cause_word(), Some(0));
        assert_eq!(
            machine
                .cpu()
                .cop1_fcr31_state()
                .map(|state| state.raw_word()),
            Some(0)
        );
        assert_eq!(
            machine.cpu().cop1_fcr31_state().map(|state| state.source()),
            Some(MachineCop1Fcr31Source::CleanRoomHleNtscX105Pinned)
        );
        assert_eq!(
            gprs(&machine),
            [
                0,
                0xffff_ffff_a460_0000,
                0xffff_ffff_d907_1b05,
                5,
                0x20e3_60bb,
                0x1eef_776c,
                0xffff_ffff_d26c_5fa6,
                0xffff_ffff_fad4_0ecc,
                0xffff_ffff_a400_2000,
                0xffff_ffff_8000_1000,
                0xffff_ffff_df28_f436,
                0xffff_ffff_b000_0000,
                0x1eef_776c,
                0x20,
                0xffff_ffff_c1c7_835a,
                0xffff_ffff_a000_02ff,
                0x400,
                0xffff_ffff_a3f0_8000,
                0x0018_0000,
                0,
                1,
                0,
                0xffff_ffff_a000_0200,
                0,
                0xffff_ffff_ca7a_1833,
                0x3b13_8d96,
                0xffff_ffff_a430_0000,
                1,
                8,
                0xffff_ffff_a400_1ff0,
                0xffff_ffff_a400_1f90,
                0xffff_ffff_a400_0324,
            ]
        );
        assert_eq!(machine.processor_turn(), MachineStepProcessor::Cpu);
        assert!(
            (0..machine.cpu().primary_caches().instruction_line_count()).all(|index| {
                machine
                    .cpu()
                    .primary_caches()
                    .instruction_line(index)
                    .unwrap()
                    .clean_room_hle_source()
                    == Some(MachinePrimaryCacheCleanRoomHleSource::NtscX105Pinned)
            })
        );
        assert!(
            (0..machine.cpu().primary_caches().data_line_count()).all(|index| {
                machine
                    .cpu()
                    .primary_caches()
                    .data_line(index)
                    .unwrap()
                    .clean_room_hle_source()
                    == Some(MachinePrimaryCacheCleanRoomHleSource::NtscX105Pinned)
            })
        );
        assert_eq!(machine.sp_status_state(), None);
        assert_eq!(machine.sp_pc_state(), None);
        assert_eq!(machine.sp_imem_opaque_word_state(0), None);
        assert_eq!(machine.rsp_committed_instruction_count(), 0);
        assert_eq!(machine.rsp_scalar_register(0).unwrap().value(), Some(0));
        assert_eq!(
            machine
                .rsp_scalar_register(crate::rsp::RSP_NTSC_X105_POST_BOOT_GPR_11_INDEX)
                .unwrap(),
            crate::rsp::MachineRspScalarRegisterState::Available {
                value: crate::rsp::RSP_NTSC_X105_POST_BOOT_GPR_11_VALUE,
                source:
                    crate::rsp::MachineRspScalarRegisterSource::CleanRoomHleNtscX105PinnedPostBoot,
            }
        );
        for index in 1..crate::rsp::RSP_SCALAR_REGISTER_COUNT {
            if index != crate::rsp::RSP_NTSC_X105_POST_BOOT_GPR_11_INDEX {
                assert_eq!(machine.rsp_scalar_register(index).unwrap().value(), None);
            }
        }
        for (field, expected) in [
            (MachinePiDomainTimingField::Latency, 0x80),
            (MachinePiDomainTimingField::PulseWidth, 0x12),
            (MachinePiDomainTimingField::PageSize, 0x07),
            (MachinePiDomainTimingField::Release, 0x00),
        ] {
            let state = machine
                .pi_domain_timing_register_state(domain_one_register(field))
                .unwrap();
            assert_eq!(state.raw_word(), expected);
            assert_eq!(
                state.source(),
                MachinePiDomainTimingSource::CleanRoomHleCartridgeHeaderConfiguration
            );
        }
    }

    #[test]
    fn first_machine_step_after_clean_room_hle_naturally_attempts_cartridge_cpu_entry() {
        let mut machine = generated_machine(GENERATED_ENTRY, 0);
        machine
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();

        let outcome = machine.step().unwrap();

        assert!(matches!(
            outcome,
            MachineRepresentedStepOutcome::CpuLocalCommitted {
                identity: CpuInstructionIdentity::Addiu,
                cadence_plan,
            } if cadence_plan.advances_count()
        ));
        assert_eq!(outcome.processor(), MachineStepProcessor::Cpu);
        assert_eq!(machine.cpu().pc(), GENERATED_ENTRY + 4);
        assert_eq!(machine.cpu().next_pc(), GENERATED_ENTRY + 8);
        assert_eq!(machine.cpu().gpr(0), Some(0));
        assert_eq!(machine.cpu().gpr(2), Some(0x42));
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert_eq!(machine.rsp_committed_instruction_count(), 0);
    }

    #[test]
    fn clean_room_hle_uses_public_initialized_4_mib_profile_for_absent_memory_loads() {
        let mut bytes = generated_cartridge(GENERATED_ENTRY, 0);
        write_be_u32(&mut bytes, 0x1000, 0x3c01_a040);
        write_be_u32(&mut bytes, 0x1004, 0x8c22_0000);
        let mut machine = Machine::from_cartridge(load_cartridge(bytes).unwrap());
        machine
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();

        assert!(matches!(
            machine.step(),
            Ok(MachineRepresentedStepOutcome::CpuLocalCommitted {
                identity: CpuInstructionIdentity::Lui,
                ..
            })
        ));
        assert!(matches!(
            machine.step(),
            Ok(MachineRepresentedStepOutcome::LoadWordCommitted {
                target: MachineLoadWordTarget::RdramAbsentModuleMemory { physical_address },
                loaded_word: 0,
                result_value: 0,
                data_cache_hit: None,
                cadence_plan,
                ..
            }) if physical_address == RDRAM_SIZE_BYTES as u32 && cadence_plan.advances_count()
        ));
        assert_eq!(machine.cpu().gpr(2), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 2);
        assert_eq!(machine.rsp_committed_instruction_count(), 0);
    }

    #[test]
    fn clean_room_hle_stages_public_fcr31_for_first_cfc1_read() {
        let mut bytes = generated_cartridge(GENERATED_ENTRY, 0);
        write_be_u32(&mut bytes, 0x1000, 0x4442_f800);
        let mut machine = Machine::from_cartridge(load_cartridge(bytes).unwrap());
        machine
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();

        assert!(matches!(
            machine.step(),
            Ok(MachineRepresentedStepOutcome::Cop1ControlTransferCommitted {
                kind: MachineCop1ControlTransferKind::Cfc1,
                transfer_gpr: 2,
                transfer_word: 0,
                result_value: Some(0),
                state,
                cadence_plan,
            }) if state.source() == MachineCop1Fcr31Source::CleanRoomHleNtscX105Pinned
                && cadence_plan.advances_count()
        ));
        assert_eq!(machine.cpu().gpr(2), Some(0));
        assert_eq!(machine.cpu().cop0_count(), 1);
        assert_eq!(machine.rsp_committed_instruction_count(), 0);
    }

    #[test]
    fn clean_room_hle_pi_domain_one_tuple_reads_through_existing_cpu_loads() {
        let mut bytes = generated_cartridge(GENERATED_ENTRY, 0);
        for (offset, word) in [
            (0x1000, 0x8c22_0014),
            (0x1004, 0x8c23_0018),
            (0x1008, 0x8c24_001c),
            (0x100c, 0x8c25_0020),
        ] {
            write_be_u32(&mut bytes, offset, word);
        }
        let mut machine = Machine::from_cartridge(load_cartridge(bytes).unwrap());
        machine
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();
        let pi_before = domain_one_timing_states(&machine);

        for (destination_gpr, field, expected) in [
            (2, MachinePiDomainTimingField::Latency, 0x80),
            (3, MachinePiDomainTimingField::PulseWidth, 0x12),
            (4, MachinePiDomainTimingField::PageSize, 0x07),
            (5, MachinePiDomainTimingField::Release, 0x00),
        ] {
            assert!(matches!(
                machine.step(),
                Ok(MachineRepresentedStepOutcome::LoadWordCommitted {
                    target: MachineLoadWordTarget::PiDomainTiming { register },
                    destination_gpr: actual_destination,
                    loaded_word,
                    result_value,
                    cadence_plan,
                    ..
                }) if register == domain_one_register(field)
                    && actual_destination == destination_gpr
                    && loaded_word == expected
                    && result_value == expected as u64
                    && cadence_plan.advances_count()
            ));
        }

        assert_eq!(machine.cpu().cop0_count(), 4);
        assert_eq!(machine.rsp_committed_instruction_count(), 0);
        assert_eq!(domain_one_timing_states(&machine), pi_before);
    }

    #[test]
    fn unavailable_or_invalid_clean_room_handoff_rejects_before_machine_mutation() {
        let mut short_bytes = generated_cartridge(GENERATED_ENTRY, 0);
        short_bytes.truncate(0x1000);
        let mut short = Machine::from_cartridge(load_cartridge(short_bytes).unwrap());
        let before = rejection_snapshot(&short);
        assert!(matches!(
            short.stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned),
            Err(MachineCleanRoomHleError::CartridgeSourceRangeUnavailable { .. })
        ));
        assert_eq!(rejection_snapshot(&short), before);

        let mut invalid = generated_machine(0xa000_1000, 0);
        let before = rejection_snapshot(&invalid);
        assert_eq!(
            invalid.stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned),
            Err(MachineCleanRoomHleError::UnsupportedEntryAddress {
                entry_address: 0xa000_1000,
            })
        );
        assert_eq!(rejection_snapshot(&invalid), before);
    }

    #[test]
    fn clean_room_explicit_pif_and_public_synthetic_boot_sources_are_separate() {
        let mut clean_room = generated_machine(GENERATED_ENTRY, 0);
        clean_room
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();
        assert!(matches!(
            clean_room.boot_source(),
            Some(MachineBootSource::CleanRoomHle { .. })
        ));

        let mut explicit = generated_machine(GENERATED_ENTRY, 0);
        explicit
            .install_pif_firmware(vec![0x5a; PIF_BOOT_ROM_SIZE_BYTES])
            .unwrap();
        assert_eq!(
            explicit.boot_source(),
            Some(MachineBootSource::ExplicitPifFirmware)
        );
        let before = rejection_snapshot(&explicit);
        assert_eq!(
            explicit.stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned),
            Err(MachineCleanRoomHleError::ConflictingPifBootInputs)
        );
        assert_eq!(rejection_snapshot(&explicit), before);

        let mut synthetic = generated_machine(GENERATED_ENTRY, 0);
        synthetic.install_public_synthetic_cold_x105_bootstrap();
        assert_eq!(
            synthetic.boot_source(),
            Some(MachineBootSource::PublicSyntheticProof)
        );
        let before = rejection_snapshot(&synthetic);
        assert_eq!(
            synthetic.stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned),
            Err(MachineCleanRoomHleError::ConflictingPifBootInputs)
        );
        assert_eq!(rejection_snapshot(&synthetic), before);
    }

    #[test]
    fn clean_room_hle_staging_and_execution_are_machine_local() {
        let first_timing = CartridgePiDomain1Timing::from_header_configuration_word(0x9135_27c2);
        let second_timing = CartridgePiDomain1Timing::from_header_configuration_word(0xa64a_18fd);
        let mut first = generated_machine_with_pi_timing(GENERATED_ENTRY, 0, first_timing);
        let mut second = generated_machine_with_pi_timing(GENERATED_ENTRY, 1, second_timing);

        first
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();
        assert!(first.clean_room_hle_state().is_some());
        assert!(second.clean_room_hle_state().is_none());
        assert_ne!(first.cpu().pc(), second.cpu().pc());

        second
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();
        assert_ne!(
            domain_one_timing_states(&first),
            domain_one_timing_states(&second)
        );
        let second_before = rejection_snapshot(&second);
        first.step().unwrap();
        assert_eq!(rejection_snapshot(&second), second_before);
        assert_ne!(first.cpu().gpr(2), second.cpu().gpr(2));
        assert_ne!(
            first.rdram().read_u32_be(0x2000).unwrap(),
            second.rdram().read_u32_be(0x2000).unwrap()
        );
    }

    #[test]
    fn reset_clears_clean_room_staging_without_materializing_boot_memory() {
        let mut machine = generated_machine(GENERATED_ENTRY, 0);
        machine
            .stage_clean_room_cartridge_entry(MachineCleanRoomBootProfile::NtscX105Pinned)
            .unwrap();
        machine.reset();

        assert_eq!(machine.boot_source(), None);
        assert_eq!(machine.clean_room_hle_state(), None);
        assert_eq!(machine.rdram_cartridge_staging_state(), None);
        assert_eq!(machine.rdram_initialization_source(), None);
        assert!(!machine.rdram_initialization_complete());
        assert_eq!(machine.cpu().pc(), crate::cpu::NON_BOOT_RESET_VECTOR_PC);
        assert_eq!(machine.cpu().cop1_fcr31_state(), None);
        assert_eq!(machine.sp_imem_opaque_word_state(0), None);
        assert_eq!(machine.sp_status_state(), None);
        assert_eq!(machine.rsp_committed_instruction_count(), 0);
        assert_eq!(machine.rdram().read_u32_be(0x1000), Ok(0));
        assert_eq!(machine.cartridge().size_bytes(), 0x0010_1000);
        assert_eq!(domain_one_timing_states(&machine), [None; 4]);
    }

    #[test]
    fn empty_machine_has_no_implicit_clean_room_or_synthetic_boot_source() {
        let machine = Machine::from_cartridge(Cartridge::default());
        assert_eq!(machine.boot_source(), None);
        assert_eq!(machine.clean_room_hle_state(), None);
        assert_eq!(
            machine.pif_firmware_state(),
            MachinePifFirmwareState::Absent
        );
    }
}
