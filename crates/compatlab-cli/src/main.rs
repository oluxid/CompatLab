use std::env;
use std::process::ExitCode;

use compatlab_core::experiment::{Experiment, ExperimentConfig};
use compatlab_core::interpreter::inspect_interpreter;
use compatlab_core::runner::run_experiment;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("inspect") => {
            let Some(executable) = args.next() else {
                eprintln!("Usage: compatlab-cli inspect <python-executable>");
                return ExitCode::from(2);
            };

            match inspect_interpreter(&executable) {
                Ok(interpreter) => {
                    println!("Executable: {}", interpreter.executable);
                    println!("Version: {}", interpreter.version);
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("Error: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("run") => {
            let Some(python_executable) = args.next() else {
                eprintln!(
                    "Usage: compatlab-cli run <python-executable> <package> <import-name> [timeout-seconds]"
                );
                return ExitCode::from(2);
            };

            let Some(package) = args.next() else {
                eprintln!("Missing package requirement.");
                return ExitCode::from(2);
            };

            let Some(import_name) = args.next() else {
                eprintln!("Missing Python module name to import.");
                return ExitCode::from(2);
            };

            let timeout_seconds = match args.next() {
                Some(value) => match value.parse::<u64>() {
                    Ok(seconds) if seconds > 0 => seconds,
                    _ => {
                        eprintln!("Timeout must be a positive integer.");
                        return ExitCode::from(2);
                    }
                },
                None => 120,
            };

            let experiment = Experiment {
                python_executable,
                package,
                import_name,
            };

            let config = ExperimentConfig { timeout_seconds };
            let report = run_experiment(&experiment, &config);

            println!("{}", serde_json::to_string_pretty(&report).unwrap());

            match report.status {
                compatlab_core::report::ExperimentStatus::Passed => ExitCode::SUCCESS,
                _ => ExitCode::FAILURE,
            }
        }
        Some("--help") | Some("-h") | None => {
            println!("CompatLab - Python compatibility experiments");
            println!();
            println!("Usage:");
            println!("  compatlab-cli inspect <python-executable>");
            println!(
                "  compatlab-cli run <python-executable> <package> <import-name> [timeout-seconds]"
            );
            ExitCode::SUCCESS
        }
        Some(command) => {
            eprintln!("Unknown command: {command}");
            eprintln!("Run 'compatlab-cli --help' for usage.");
            ExitCode::from(2)
        }
    }
}
