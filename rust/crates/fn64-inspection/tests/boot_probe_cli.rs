use std::path::PathBuf;
use std::process::Command;

use fn64_core::{PIF_BOOT_ROM_SIZE_BYTES, PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES};

fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset] = ((value >> 24) & 0xff) as u8;
    bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
    bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
    bytes[offset + 3] = value as u8;
}

const fn special_add_word(rs: u8, rt: u8, rd: u8) -> u32 {
    ((rs as u32) << 21) | ((rt as u32) << 16) | ((rd as u32) << 11) | 0x20
}

const fn lw_word(base: u8, rt: u8, immediate: u16) -> u32 {
    (0x23 << 26) | ((base as u32) << 21) | ((rt as u32) << 16) | immediate as u32
}

fn make_generated_boot_fixture() -> Vec<u8> {
    let mut bytes = vec![0; 0x1000];
    write_be_u32(&mut bytes, 0x00, 0x8037_1240);
    write_be_u32(&mut bytes, 0x04, 0x0102_0304);
    write_be_u32(&mut bytes, 0x08, 0x8000_1000);
    write_be_u32(&mut bytes, 0x0c, 0x0506_0708);
    write_be_u32(&mut bytes, 0x10, 0x1112_1314);
    write_be_u32(&mut bytes, 0x14, 0x1516_1718);
    bytes[0x20..0x33].copy_from_slice(b"FN64 GENERATED BOOT");
    bytes[0x3c] = b'C';
    bytes[0x3d] = b'L';
    bytes[0x3e] = 0x45;
    bytes[0x3f] = 3;
    write_be_u32(&mut bytes, 0x40, special_add_word(29, 0, 9));
    write_be_u32(&mut bytes, 0x44, lw_word(9, 8, 0xf010));
    bytes
}

fn make_generated_pif_firmware(size: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = (0..size)
        .map(|index| 0x31_u8.wrapping_add((index as u8).wrapping_mul(53)))
        .collect();
    let sentinel = b"FN64_GENERATED_PIF_BYTES_MUST_NOT_BE_LOGGED";
    if bytes.len() >= sentinel.len() {
        bytes[..sentinel.len()].copy_from_slice(sentinel);
    }
    bytes
}

fn generated_fixture_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fn64-boot-probe-generated-{}-{}.fixture",
        std::process::id(),
        label
    ))
}

fn generated_fixture_directory(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fn64-boot-probe-generated-dir-{}-{}",
        std::process::id(),
        label
    ))
}

#[test]
fn boot_probe_cli_generated_local_fixture_reaches_expected_frontier_with_success_exit() {
    let path = generated_fixture_path("success");
    std::fs::write(&path, make_generated_boot_fixture()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
        .arg(&path)
        .arg("--max-steps")
        .arg("100")
        .output()
        .unwrap();

    std::fs::remove_file(&path).unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("highest_checkpoint: BOOT-2"));
    assert!(stdout.contains("attempted_steps: 2"));
    assert!(stdout.contains("committed_steps: 1"));
    assert!(stdout.contains("last_committed_identity: SpecialAdd"));
    assert!(stdout.contains("last_committed_destination_gpr: 9"));
    assert!(stdout.contains("last_committed_destination_value_changed: yes"));
    assert!(stdout.contains("last_committed_destination_known: no->yes"));
    assert!(stdout.contains("identity=Lw"));
    assert!(stdout.contains("base_known=yes"));
    assert!(stdout.contains("target=sp-imem offset=0x00000000"));
    assert!(stdout.contains("reason=sp-imem-unknown"));
    assert!(stdout.contains("expected_frontier_exit_policy: success"));
    assert!(stdout.contains("pif_firmware_input: absent"));
    assert!(stdout.contains("pif_firmware_search: none"));
    assert!(stdout.contains("pif_firmware_default_path: none"));
    assert!(stdout.contains("no_window: yes"));
    assert!(!stdout.contains("timestamp"));
    assert!(!stdout.contains("SDL"));
    assert!(!stdout.contains("audio"));
}

#[test]
fn boot_probe_cli_generated_structural_failure_is_nonzero() {
    let path = generated_fixture_path("structural-failure");
    std::fs::write(&path, [0x80, 0x37, 0x12]).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
        .arg(&path)
        .arg("--max-steps")
        .arg("100")
        .output()
        .unwrap();

    std::fs::remove_file(&path).unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("result: fail"));
    assert!(stderr.contains("structural cartridge input rejected"));
}

#[test]
fn boot_probe_cli_argument_failure_uses_usage_exit_status() {
    let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("usage: fn64_boot_probe"));
}

#[test]
fn boot_probe_cli_missing_pif_option_value_is_explicit() {
    let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
        .arg("generated-rom.fixture")
        .arg("--pif-rom")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("--pif-rom requires an explicit path"));
}

#[test]
fn boot_probe_cli_unreadable_explicit_pif_path_fails_without_search() {
    let rom_path = generated_fixture_path("unreadable-pif-rom");
    let missing_pif_path = generated_fixture_path("intentionally-missing-pif");
    let _ = std::fs::remove_file(&missing_pif_path);
    std::fs::write(&rom_path, make_generated_boot_fixture()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
        .arg(&rom_path)
        .arg("--pif-rom")
        .arg(&missing_pif_path)
        .output()
        .unwrap();

    std::fs::remove_file(&rom_path).unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("result: fail"));
    assert!(stderr.contains("local PIF firmware read failed"));
    assert!(stderr.contains(&missing_pif_path.display().to_string()));
}

#[test]
fn boot_probe_cli_accepts_generated_raw_boot_rom_without_dumping_bytes_or_path() {
    let rom_path = generated_fixture_path("accepted-pif-rom");
    let pif_path = generated_fixture_path("accepted-pif-source");
    std::fs::write(&rom_path, make_generated_boot_fixture()).unwrap();
    std::fs::write(
        &pif_path,
        make_generated_pif_firmware(PIF_BOOT_ROM_SIZE_BYTES),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
        .arg(&rom_path)
        .arg("--pif-rom")
        .arg(&pif_path)
        .arg("--max-steps")
        .arg("100")
        .output()
        .unwrap();

    std::fs::remove_file(&rom_path).unwrap();
    std::fs::remove_file(&pif_path).unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("pif_firmware_input: accepted"));
    assert!(stdout.contains("pif_firmware_classification: raw-boot-rom"));
    assert!(stdout.contains("pif_firmware_size_bytes: 1984"));
    assert!(stdout.contains("pif_firmware_sp_imem_production: unavailable"));
    assert!(stdout.contains("highest_checkpoint: BOOT-2"));
    assert!(stdout.contains("reason=sp-imem-unknown"));
    assert!(!stdout.contains("FN64_GENERATED_PIF_BYTES_MUST_NOT_BE_LOGGED"));
    assert!(!stdout.contains(&pif_path.display().to_string()));
}

#[test]
fn boot_probe_cli_distinguishes_generated_malformed_and_unsupported_pif_files() {
    let rom_path = generated_fixture_path("pif-validation-rom");
    std::fs::write(&rom_path, make_generated_boot_fixture()).unwrap();

    for (label, size, expected) in [
        (
            "malformed-pif",
            PIF_BOOT_ROM_SIZE_BYTES - 1,
            "malformed PIF firmware input",
        ),
        (
            "unsupported-pif",
            PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
            "unsupported PIF firmware layout",
        ),
    ] {
        let pif_path = generated_fixture_path(label);
        std::fs::write(&pif_path, make_generated_pif_firmware(size)).unwrap();

        let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
            .arg(&rom_path)
            .arg("--pif-rom")
            .arg(&pif_path)
            .output()
            .unwrap();

        std::fs::remove_file(&pif_path).unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("PIF firmware input rejected"));
        assert!(stderr.contains(expected));
        assert!(!stderr.contains("FN64_GENERATED_PIF_BYTES_MUST_NOT_BE_LOGGED"));
    }

    std::fs::remove_file(&rom_path).unwrap();
}

#[test]
fn boot_probe_cli_does_not_consult_a_default_pif_filename() {
    let directory = generated_fixture_directory("no-default-search");
    std::fs::create_dir(&directory).unwrap();
    let rom_path = directory.join("generated-cartridge.fixture");
    let tempting_default = directory.join("pifdata.bin");
    std::fs::write(&rom_path, make_generated_boot_fixture()).unwrap();
    std::fs::write(
        &tempting_default,
        make_generated_pif_firmware(PIF_BOOT_ROM_SIZE_BYTES),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_boot_probe"))
        .current_dir(&directory)
        .arg("generated-cartridge.fixture")
        .output()
        .unwrap();

    std::fs::remove_file(&rom_path).unwrap();
    std::fs::remove_file(&tempting_default).unwrap();
    std::fs::remove_dir(&directory).unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("pif_firmware_input: absent"));
    assert!(stdout.contains("pif_firmware_search: none"));
    assert!(stdout.contains("pif_firmware_default_path: none"));
}
