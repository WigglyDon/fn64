use std::process::ExitCode;

fn main() -> ExitCode {
    if std::env::args_os().len() != 1 {
        eprintln!("usage: fn64_machine_probe");
        return ExitCode::from(2);
    }

    match fn64_inspection::run_machine_probe() {
        Ok(report) => {
            print!("{}", report.output());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("fn64 machine probe");
            eprintln!("result: fail");
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}
