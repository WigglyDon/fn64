use std::path::PathBuf;
use std::process::Command;

fn write_be_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset] = ((value >> 24) & 0xff) as u8;
    bytes[offset + 1] = ((value >> 16) & 0xff) as u8;
    bytes[offset + 2] = ((value >> 8) & 0xff) as u8;
    bytes[offset + 3] = value as u8;
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
    write_be_u32(&mut bytes, 0x40, 0x3c08_1234);
    write_be_u32(&mut bytes, 0x44, 0x8fa9_0000);
    bytes
}

fn generated_fixture_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fn64-boot-probe-generated-{}-{}.fixture",
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
    assert!(stdout.contains("identity=Lw"));
    assert!(stdout.contains("expected_frontier_exit_policy: success"));
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
