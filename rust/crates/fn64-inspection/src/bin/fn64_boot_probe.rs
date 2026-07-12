use std::process::ExitCode;

use fn64_inspection::boot_probe::{parse_boot_probe_arguments, run_boot_probe_with_pif_firmware};

fn main() -> ExitCode {
    let arguments = match parse_boot_probe_arguments(std::env::args_os().skip(1)) {
        Ok(arguments) => arguments,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(2);
        }
    };

    let bytes = match std::fs::read(arguments.input_path()) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("fn64 boot probe");
            eprintln!("result: fail");
            eprintln!(
                "error: local input read failed: path={} detail={}",
                arguments.input_path().display(),
                error
            );
            return ExitCode::from(1);
        }
    };

    let profiled_pif_firmware = match (arguments.pif_rom_path(), arguments.pif_profile()) {
        (Some(path), Some(profile)) => match std::fs::read(path) {
            Ok(bytes) => Some((profile, bytes)),
            Err(error) => {
                eprintln!("fn64 boot probe");
                eprintln!("result: fail");
                eprintln!(
                    "error: local PIF firmware read failed: path={} detail={}",
                    path.display(),
                    error
                );
                return ExitCode::from(1);
            }
        },
        (None, None) => None,
        _ => unreachable!("argument parser requires a paired PIF path and profile"),
    };

    let input_path = arguments.input_path().display().to_string();
    match run_boot_probe_with_pif_firmware(
        bytes,
        &input_path,
        profiled_pif_firmware,
        arguments.max_steps(),
    ) {
        Ok(report) => {
            print!("{}", report.output());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("fn64 boot probe");
            eprintln!("result: fail");
            eprintln!("error: {error}");
            ExitCode::from(error.exit_status())
        }
    }
}
