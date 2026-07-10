#![forbid(unsafe_code)]

pub mod boot_probe;

use core::fmt;

use fn64_core::cpu::address::CpuAddress;
use fn64_core::{
    load_cartridge, CartridgeLoadError, CartridgeReadError, DirectRdramAccessError, Machine,
    MachineDirectRdramCpuDataAccessError, RdramAccessError, RomSourceLayout,
    NON_BOOT_RESET_VECTOR_NEXT_PC, NON_BOOT_RESET_VECTOR_PC, RDRAM_SIZE_BYTES,
};

pub const MACHINE_PROBE_OUTPUT: &str =
    "fn64 machine probe\nconstruct: ok\nreset: ok\nno-window: ok\nresult: ok\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineProbeReport;

impl MachineProbeReport {
    pub const fn output(&self) -> &'static str {
        MACHINE_PROBE_OUTPUT
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineProbeError {
    CartridgeLoad(CartridgeLoadError),
    CartridgeRead(CartridgeReadError),
    RdramAccess(RdramAccessError),
    DirectRdramAccess(DirectRdramAccessError),
    DirectRdramCpuDataAccess(MachineDirectRdramCpuDataAccessError),
    AssertionFailed(&'static str),
}

impl fmt::Display for MachineProbeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CartridgeLoad(error) => write!(f, "probe cartridge load failed: {error}"),
            Self::CartridgeRead(error) => write!(f, "probe cartridge read failed: {error}"),
            Self::RdramAccess(error) => write!(f, "probe RDRAM access failed: {error}"),
            Self::DirectRdramAccess(error) => {
                write!(f, "probe direct RDRAM access failed: {error}")
            }
            Self::DirectRdramCpuDataAccess(error) => {
                write!(f, "probe direct RDRAM CPU data access failed: {error}")
            }
            Self::AssertionFailed(label) => write!(f, "probe assertion failed: {label}"),
        }
    }
}

impl std::error::Error for MachineProbeError {}

pub fn run_machine_probe() -> Result<MachineProbeReport, MachineProbeError> {
    let cartridge = load_cartridge(make_synthetic_probe_cartridge_bytes())
        .map_err(MachineProbeError::CartridgeLoad)?;
    let expected_cartridge_size = cartridge.size_bytes();
    let expected_cartridge_name = cartridge.metadata().image_name.clone();

    let mut machine = Machine::from_cartridge(cartridge);
    assert_represented_power_on_state(&machine, "construct")?;
    assert_probe_cartridge(
        &machine,
        expected_cartridge_size,
        &expected_cartridge_name,
        "construct cartridge",
    )?;

    dirty_represented_machine_state(&mut machine)?;
    machine.reset();

    assert_represented_power_on_state(&machine, "reset")?;
    assert_probe_cartridge(
        &machine,
        expected_cartridge_size,
        &expected_cartridge_name,
        "reset cartridge",
    )?;

    Ok(MachineProbeReport)
}

fn dirty_represented_machine_state(machine: &mut Machine) -> Result<(), MachineProbeError> {
    machine
        .write_rdram_u32_be(0, 0x1122_3344)
        .map_err(MachineProbeError::RdramAccess)?;
    machine
        .write_direct_rdram_u64_be(CpuAddress::new(0x8000_0010), 0x0102_0304_0506_0708)
        .map_err(MachineProbeError::DirectRdramAccess)?;
    let error = machine
        .read_direct_rdram_cpu_data_u32_be(CpuAddress::new(0x8000_0001))
        .expect_err("unaligned direct RDRAM CPU data read must enter address error");
    if error.exception_kind().is_none() {
        return Err(MachineProbeError::DirectRdramCpuDataAccess(error));
    }

    Ok(())
}

fn assert_represented_power_on_state(
    machine: &Machine,
    label: &'static str,
) -> Result<(), MachineProbeError> {
    assert_probe(machine.powered_on(), label)?;
    assert_probe(machine.cpu().pc() == NON_BOOT_RESET_VECTOR_PC, label)?;
    assert_probe(
        machine.cpu().next_pc() == NON_BOOT_RESET_VECTOR_NEXT_PC,
        label,
    )?;
    assert_probe(machine.cpu().hi() == 0, label)?;
    assert_probe(machine.cpu().lo() == 0, label)?;
    assert_probe(machine.cpu().gpr(0) == Some(0), label)?;
    assert_probe(machine.cpu().gpr(1) == Some(0), label)?;
    assert_probe(machine.cpu().gpr(8) == Some(0), label)?;
    assert_probe(machine.cpu().gpr(31) == Some(0), label)?;
    assert_probe(machine.cpu().cop0_count() == 0, label)?;
    assert_probe(machine.cpu().cop0_compare() == 0, label)?;
    assert_probe(!machine.cpu().cop0_timer_interrupt_pending(), label)?;
    assert_probe(machine.cpu().cop0_status() == 0, label)?;
    assert_probe(machine.cpu().cop0_software_interrupt_pending() == 0, label)?;
    assert_probe(machine.cpu().cop0_epc() == 0, label)?;
    assert_probe(machine.cpu().cop0_bad_vaddr() == 0, label)?;
    assert_probe(machine.cpu().cop0_exception_code() == 0, label)?;
    assert_probe(!machine.cpu().cop0_exception_branch_delay(), label)?;
    assert_probe(machine.rdram().size_bytes() == RDRAM_SIZE_BYTES, label)?;
    assert_probe(
        machine
            .rdram()
            .read_u8(0)
            .map_err(MachineProbeError::RdramAccess)?
            == 0,
        label,
    )?;
    assert_probe(
        machine
            .rdram()
            .read_u8(RDRAM_SIZE_BYTES - 1)
            .map_err(MachineProbeError::RdramAccess)?
            == 0,
        label,
    )?;
    assert_probe(
        machine
            .rdram()
            .read_u64_be(0x10)
            .map_err(MachineProbeError::RdramAccess)?
            == 0,
        label,
    )?;

    Ok(())
}

fn assert_probe_cartridge(
    machine: &Machine,
    expected_size: usize,
    expected_name: &str,
    label: &'static str,
) -> Result<(), MachineProbeError> {
    let cartridge = machine.cartridge();
    assert_probe(
        cartridge.source_layout() == RomSourceLayout::BigEndian,
        label,
    )?;
    assert_probe(cartridge.size_bytes() == expected_size, label)?;
    assert_probe(cartridge.metadata().image_name == expected_name, label)?;
    assert_probe(
        cartridge
            .read_u8(0)
            .map_err(MachineProbeError::CartridgeRead)?
            == 0x80,
        label,
    )?;

    Ok(())
}

fn assert_probe(condition: bool, label: &'static str) -> Result<(), MachineProbeError> {
    if condition {
        Ok(())
    } else {
        Err(MachineProbeError::AssertionFailed(label))
    }
}

fn make_synthetic_probe_cartridge_bytes() -> Vec<u8> {
    let mut bytes = vec![0; 0x60];
    write_be_u32(&mut bytes, 0x00, 0x8037_1240);
    write_be_u32(&mut bytes, 0x04, 0x1234_5678);
    write_be_u32(&mut bytes, 0x08, 0x8024_6000);
    write_be_u32(&mut bytes, 0x0c, 0x0040_0000);
    write_be_u32(&mut bytes, 0x10, 0x89ab_cdef);
    write_be_u32(&mut bytes, 0x14, 0x0123_4567);

    let image_name = b"FN64 MACHINE PROBE";
    bytes[0x20..0x20 + image_name.len()].copy_from_slice(image_name);
    bytes[0x3c] = b'F';
    bytes[0x3d] = b'P';
    bytes[0x3e] = 0x45;
    bytes[0x3f] = 0x01;

    bytes
}

fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset] = ((value >> 24) & 0xff) as u8;
    bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
    bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
    bytes[offset + 3] = (value & 0xff) as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_logic_succeeds_for_sealed_construction_and_reset_facts() {
        let report = run_machine_probe().unwrap();

        assert_eq!(report.output(), MACHINE_PROBE_OUTPUT);
    }

    #[test]
    fn probe_observes_reset_after_prior_rdram_and_exception_mutation() {
        let cartridge = load_cartridge(make_synthetic_probe_cartridge_bytes()).unwrap();
        let expected_size = cartridge.size_bytes();
        let expected_name = cartridge.metadata().image_name.clone();
        let mut machine = Machine::from_cartridge(cartridge);

        dirty_represented_machine_state(&mut machine).unwrap();
        assert_ne!(machine.rdram().read_u32_be(0).unwrap(), 0);
        assert_ne!(machine.cpu().pc(), NON_BOOT_RESET_VECTOR_PC);

        machine.reset();

        assert_represented_power_on_state(&machine, "reset after dirty state").unwrap();
        assert_probe_cartridge(
            &machine,
            expected_size,
            &expected_name,
            "reset cartridge preservation",
        )
        .unwrap();
    }

    #[test]
    fn probe_output_is_stable_plain_no_window_text() {
        let first = run_machine_probe().unwrap();
        let second = run_machine_probe().unwrap();

        assert_eq!(first.output(), second.output());
        assert_eq!(
            first.output(),
            "fn64 machine probe\nconstruct: ok\nreset: ok\nno-window: ok\nresult: ok\n"
        );
        assert!(!first.output().contains("step"));
        assert!(!first.output().contains("execute"));
        assert!(!first.output().contains("SDL"));
        assert!(!first.output().contains("window runtime"));
    }
}
