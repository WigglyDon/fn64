use std::path::{Path, PathBuf};
use std::process::Command;

use fn64_core::{PIF_BOOT_ROM_SIZE_BYTES, PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES};

fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}

fn make_generated_cartridge() -> Vec<u8> {
    let mut bytes = vec![0; 0x1000];
    write_be_u32(&mut bytes, 0, 0x8037_1240);
    write_be_u32(&mut bytes, 8, 0xa400_0040);
    bytes[0x20..0x34].copy_from_slice(b"FN64 GENERATED INPUT");
    write_be_u32(&mut bytes, 0x40, 0x03a0_4820);
    bytes
}

fn make_generated_pif(size: usize) -> Vec<u8> {
    (0..size)
        .map(|index| 0x29_u8.wrapping_add((index as u8).wrapping_mul(41)))
        .collect()
}

fn fixture_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fn64-user-cartridge-probe-generated-{}-{label}.fixture",
        std::process::id()
    ))
}

fn remove_if_present(path: &Path) {
    let _ = std::fs::remove_file(path);
}

#[test]
fn user_cartridge_probe_without_explicit_pif_stops_at_material_owner() {
    let cartridge_path = fixture_path("missing-explicit-pif");
    std::fs::write(&cartridge_path, make_generated_cartridge()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_user_cartridge_probe"))
        .arg(&cartridge_path)
        .arg("--max-steps")
        .arg("1")
        .output()
        .unwrap();

    remove_if_present(&cartridge_path);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("PIF_FIRMWARE_REQUIRED_FOR_AUTHENTIC_BOOT"));
    assert!(stderr.contains("owner=PifFirmware"));
    assert!(stderr.contains("material=unavailable"));
    assert!(stderr.contains("synthetic_fallback=none"));
    assert!(!stderr.contains(&cartridge_path.display().to_string()));
    assert!(!stderr.contains("public-synthetic-cold-x105-bootstrap"));
}

#[test]
fn user_cartridge_probe_reads_only_an_explicit_redacted_pif_path() {
    let cartridge_path = fixture_path("explicit-cartridge");
    let pif_path = fixture_path("explicit-private-name-must-not-appear");
    std::fs::write(&cartridge_path, make_generated_cartridge()).unwrap();
    std::fs::write(&pif_path, make_generated_pif(PIF_BOOT_ROM_SIZE_BYTES)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_user_cartridge_probe"))
        .arg(&cartridge_path)
        .arg("--pif-rom")
        .arg(&pif_path)
        .arg("--max-steps")
        .arg("1")
        .output()
        .unwrap();

    remove_if_present(&cartridge_path);
    remove_if_present(&pif_path);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("step ceiling 1 reached"));
    assert!(!stderr.contains("PIF_FIRMWARE_REQUIRED_FOR_AUTHENTIC_BOOT"));
    assert!(!stderr.contains(&cartridge_path.display().to_string()));
    assert!(!stderr.contains(&pif_path.display().to_string()));
    assert!(!stderr.contains("explicit-private-name-must-not-appear"));
}

#[test]
fn user_cartridge_probe_redacts_unreadable_and_malformed_pif_inputs() {
    let cartridge_path = fixture_path("validation-cartridge");
    let missing_pif_path = fixture_path("unreadable-private-pif-name");
    std::fs::write(&cartridge_path, make_generated_cartridge()).unwrap();
    remove_if_present(&missing_pif_path);

    let unreadable = Command::new(env!("CARGO_BIN_EXE_fn64_user_cartridge_probe"))
        .arg(&cartridge_path)
        .arg("--pif-rom")
        .arg(&missing_pif_path)
        .output()
        .unwrap();
    assert_eq!(unreadable.status.code(), Some(1));
    let unreadable_stderr = String::from_utf8(unreadable.stderr).unwrap();
    assert!(unreadable_stderr.contains("<REDACTED_USER_PIF_FIRMWARE>"));
    assert!(!unreadable_stderr.contains(&missing_pif_path.display().to_string()));
    assert!(!unreadable_stderr.contains("unreadable-private-pif-name"));

    for (label, size, expected) in [
        (
            "malformed-private-pif-name",
            PIF_BOOT_ROM_SIZE_BYTES - 1,
            "malformed PIF firmware input",
        ),
        (
            "unsupported-private-pif-name",
            PIF_PHYSICAL_ADDRESS_SPACE_SIZE_BYTES,
            "unsupported PIF firmware layout",
        ),
    ] {
        let pif_path = fixture_path(label);
        std::fs::write(&pif_path, make_generated_pif(size)).unwrap();
        let rejected = Command::new(env!("CARGO_BIN_EXE_fn64_user_cartridge_probe"))
            .arg(&cartridge_path)
            .arg("--pif-rom")
            .arg(&pif_path)
            .output()
            .unwrap();
        remove_if_present(&pif_path);
        assert_eq!(rejected.status.code(), Some(1));
        let stderr = String::from_utf8(rejected.stderr).unwrap();
        assert!(stderr.contains(expected));
        assert!(!stderr.contains(&pif_path.display().to_string()));
        assert!(!stderr.contains(label));
    }

    remove_if_present(&cartridge_path);
}

#[test]
fn user_cartridge_probe_does_not_search_for_pif_material() {
    let directory = std::env::temp_dir().join(format!(
        "fn64-user-cartridge-probe-generated-dir-{}",
        std::process::id()
    ));
    std::fs::create_dir(&directory).unwrap();
    let cartridge_path = directory.join("generated-cartridge.fixture");
    let tempting_pif = directory.join("pifdata.bin");
    std::fs::write(&cartridge_path, make_generated_cartridge()).unwrap();
    std::fs::write(&tempting_pif, make_generated_pif(PIF_BOOT_ROM_SIZE_BYTES)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_fn64_user_cartridge_probe"))
        .current_dir(&directory)
        .env("FN64_PIF_ROM", &tempting_pif)
        .env("PIF_ROM", &tempting_pif)
        .arg("generated-cartridge.fixture")
        .arg("--max-steps")
        .arg("1")
        .output()
        .unwrap();

    remove_if_present(&cartridge_path);
    remove_if_present(&tempting_pif);
    std::fs::remove_dir(&directory).unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("PIF_FIRMWARE_REQUIRED_FOR_AUTHENTIC_BOOT"));
    assert!(stderr.contains("synthetic_fallback=none"));
}
