use std::process::{Command, Output};
use std::time::Instant;

use crate::experiment::{Experiment, ExperimentConfig};
use crate::interpreter::inspect_interpreter;
use crate::report::{ExperimentReport, ExperimentStage, ExperimentStatus};

/// Run a compatibility experiment in an isolated Python virtual environment.
pub fn run_experiment(experiment: &Experiment, config: &ExperimentConfig) -> ExperimentReport {
    let started = Instant::now();

    let mut report = ExperimentReport {
        python_executable: experiment.python_executable.clone(),
        python_version: None,
        package: experiment.package.clone(),
        import_name: experiment.import_name.clone(),
        status: ExperimentStatus::Error,
        stage: ExperimentStage::EnvironmentSetup,
        duration_ms: 0,
        stdout: String::new(),
        stderr: String::new(),
        message: None,
    };

    let interpreter = match inspect_interpreter(&experiment.python_executable) {
        Ok(interpreter) => interpreter,
        Err(error) => {
            report.message = Some(error);
            report.duration_ms = started.elapsed().as_millis() as u64;
            return report;
        }
    };

    report.python_version = Some(interpreter.version);

    let temp_dir = match tempfile::tempdir() {
        Ok(directory) => directory,
        Err(error) => {
            report.message = Some(format!("Failed to create temporary directory: {error}"));
            report.duration_ms = started.elapsed().as_millis() as u64;
            return report;
        }
    };

    let venv_path = temp_dir.path().join("venv");

    let setup = Command::new(&experiment.python_executable)
        .args(["-m", "venv"])
        .arg(&venv_path)
        .output();

    let setup = match setup {
        Ok(output) => output,
        Err(error) => {
            report.message = Some(format!("Failed to create virtual environment: {error}"));
            report.duration_ms = started.elapsed().as_millis() as u64;
            return report;
        }
    };

    if !setup.status.success() {
        report.stderr = String::from_utf8_lossy(&setup.stderr).into_owned();
        report.stdout = String::from_utf8_lossy(&setup.stdout).into_owned();
        report.message = Some("Virtual environment creation failed".into());
        report.duration_ms = started.elapsed().as_millis() as u64;
        return report;
    }

    #[cfg(windows)]
    let python = venv_path.join("Scripts").join("python.exe");

    #[cfg(not(windows))]
    let python = venv_path.join("bin").join("python");

    report.stage = ExperimentStage::Installation;

    let mut pip = Command::new(&python);
    pip.args(["-m", "pip", "install", "--disable-pip-version-check"])
        .arg(&experiment.package);

    let install = match run_with_timeout(&mut pip, config.timeout_seconds) {
        Ok(output) => output,
        Err(error) => {
            report.status = if error == "timeout" {
                ExperimentStatus::TimedOut
            } else {
                ExperimentStatus::Error
            };
            report.message = Some(if error == "timeout" {
                "Package installation timed out".into()
            } else {
                format!("Failed to run package installation: {error}")
            });
            report.duration_ms = started.elapsed().as_millis() as u64;
            return report;
        }
    };

    report.stdout = String::from_utf8_lossy(&install.stdout).into_owned();
    report.stderr = String::from_utf8_lossy(&install.stderr).into_owned();

    if !install.status.success() {
        report.status = ExperimentStatus::Failed;
        report.message = Some(format!(
            "Package installation failed with status {}",
            install.status
        ));
        report.duration_ms = started.elapsed().as_millis() as u64;
        return report;
    }

    report.stage = ExperimentStage::ImportCheck;

    let mut import = Command::new(&python);
    import
        .args([
            "-c",
            "import importlib; importlib.import_module(__import__('sys').argv[1])",
        ])
        .arg(&experiment.import_name);

    let import = match run_with_timeout(&mut import, config.timeout_seconds) {
        Ok(output) => output,
        Err(error) => {
            report.status = if error == "timeout" {
                ExperimentStatus::TimedOut
            } else {
                ExperimentStatus::Error
            };
            report.message = Some(if error == "timeout" {
                "Import check timed out".into()
            } else {
                format!("Failed to run import check: {error}")
            });
            report.duration_ms = started.elapsed().as_millis() as u64;
            return report;
        }
    };

    report
        .stdout
        .push_str(&String::from_utf8_lossy(&import.stdout));
    report
        .stderr
        .push_str(&String::from_utf8_lossy(&import.stderr));

    report.status = if import.status.success() {
        ExperimentStatus::Passed
    } else {
        ExperimentStatus::Failed
    };

    report.stage = ExperimentStage::Completed;
    report.message = Some(if import.status.success() {
        "Package installed and module imported successfully".into()
    } else {
        format!("Module import failed with status {}", import.status)
    });
    report.duration_ms = started.elapsed().as_millis() as u64;

    report
}

/// Run a child process and kill it if the timeout expires.
fn run_with_timeout(command: &mut Command, timeout_seconds: u64) -> Result<Output, String> {
    use std::thread;
    use std::time::Duration;

    let mut child = command.spawn().map_err(|error| error.to_string())?;

    let deadline = Instant::now() + Duration::from_secs(timeout_seconds);

    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                return child.wait_with_output().map_err(|error| error.to_string());
            }
            Ok(None) if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(50));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("timeout".into());
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(error.to_string());
            }
        }
    }
}
