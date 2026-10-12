use crate::environment::create_python_environment;
use crate::experiment::{Experiment, ExperimentConfig};
use crate::interpreter::inspect_interpreter;
use crate::report::{ExperimentReport, ExperimentStage, ExperimentStatus};
use std::process::{Command, Output};
use std::time::Instant;

fn apply_timeout_error(
    report: &mut ExperimentReport,
    error: &str,
    timeout_message: &str,
    operation: &str,
) {
    report.status = if error == "timeout" {
        ExperimentStatus::TimedOut
    } else {
        ExperimentStatus::Error
    };

    report.message = Some(if error == "timeout" {
        timeout_message.to_owned()
    } else {
        format!("Failed to run {operation}: {error}")
    });
}

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

    let environment = match create_python_environment(&experiment.python_executable) {
        Ok(environment) => environment,
        Err(error) => {
            report.message = Some(error.message);
            report.stdout = error.stdout;
            report.stderr = error.stderr;
            report.duration_ms = started.elapsed().as_millis() as u64;
            return report;
        }
    };

    let python = &environment.python_executable;

    report.stage = ExperimentStage::Installation;

    let mut pip = Command::new(python);
    pip.args(["-m", "pip", "install", "--disable-pip-version-check"])
        .arg(&experiment.package);

    let install = match run_with_timeout(&mut pip, config.timeout_seconds) {
        Ok(output) => output,
        Err(error) => {
            apply_timeout_error(
                &mut report,
                &error,
                "Package installation timed out",
                "package installation",
            );
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

    let mut import = Command::new(python);
    import
        .args([
            "-c",
            "import importlib; importlib.import_module(__import__('sys').argv[1])",
        ])
        .arg(&experiment.import_name);

    let import = match run_with_timeout(&mut import, config.timeout_seconds) {
        Ok(output) => output,
        Err(error) => {
            apply_timeout_error(
                &mut report,
                &error,
                "Import check timed out",
                "import check",
            );
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

    if timeout_seconds == 0 {
        return Err("timeout".into());
    }

    let deadline = Instant::now() + Duration::from_secs(timeout_seconds);
    let mut child = command.spawn().map_err(|error| error.to_string())?;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> ExperimentConfig {
        ExperimentConfig {
            timeout_seconds: 10,
        }
    }

    fn test_experiment(python_executable: &str) -> Experiment {
        Experiment {
            python_executable: python_executable.to_owned(),
            package: "example-package".to_owned(),
            import_name: "example_package".to_owned(),
        }
    }

    fn create_local_package(directory: &std::path::Path) -> std::path::PathBuf {
        let package_dir = directory.join("local_test_package");
        std::fs::create_dir_all(&package_dir).unwrap();

        std::fs::write(
            package_dir.join("setup.py"),
            "from setuptools import setup\nsetup(name='local-test-package', version='0.1.0', py_modules=['local_test_package'])\n",
        )
        .unwrap();

        std::fs::write(package_dir.join("local_test_package.py"), "VALUE = 42\n").unwrap();

        package_dir
    }

    #[test]
    fn invalid_interpreter_returns_error_report() {
        let experiment = test_experiment("compatlab-python-executable-that-does-not-exist");

        let report = run_experiment(&experiment, &test_config());

        assert_eq!(report.status, ExperimentStatus::Error);
        assert_eq!(report.stage, ExperimentStage::EnvironmentSetup);
        assert!(report.python_version.is_none());
        assert!(report.message.is_some());
        assert!(report.duration_ms < 10_000);
    }

    #[test]
    fn invalid_interpreter_preserves_experiment_details() {
        let experiment = test_experiment("compatlab-python-executable-that-does-not-exist");

        let report = run_experiment(&experiment, &test_config());

        assert_eq!(report.python_executable, experiment.python_executable);
        assert_eq!(report.package, "example-package");
        assert_eq!(report.import_name, "example_package");
    }

    #[test]
    fn invalid_interpreter_returns_empty_output_fields() {
        let experiment = test_experiment("compatlab-python-executable-that-does-not-exist");

        let report = run_experiment(&experiment, &test_config());

        assert!(report.stdout.is_empty());
        assert!(report.stderr.is_empty());
    }

    #[test]
    fn local_package_can_be_installed_and_imported() {
        let directory = tempfile::tempdir().unwrap();
        let package_dir = create_local_package(directory.path());

        let experiment = Experiment {
            python_executable: "python".to_owned(),
            package: package_dir.to_string_lossy().into_owned(),
            import_name: "local_test_package".to_owned(),
        };

        let report = run_experiment(
            &experiment,
            &ExperimentConfig {
                timeout_seconds: 60,
            },
        );

        assert_eq!(
            report.status,
            ExperimentStatus::Passed,
            "Experiment failed: {:?}\nstdout: {}\nstderr: {}",
            report.message,
            report.stdout,
            report.stderr
        );

        assert_eq!(report.stage, ExperimentStage::Completed);
        assert!(report.python_version.is_some());
    }

    #[test]
    fn installation_timeout_returns_timed_out_report() {
        let directory = tempfile::tempdir().unwrap();
        let package_dir = directory.path().join("slow_install_package");
        std::fs::create_dir_all(&package_dir).unwrap();
        std::fs::write(
            package_dir.join("setup.py"),
            "import time\ntime.sleep(30)\nfrom setuptools import setup\nsetup(name='slow-install-package', version='0.1.0', py_modules=['slow_install_package'])\n",
        )
        .unwrap();
        std::fs::write(
            package_dir.join("slow_install_package.py"),
            "VALUE = 42\n",
        )
        .unwrap();

        let report = run_experiment(
            &Experiment {
                python_executable: "python".to_owned(),
                package: package_dir.to_string_lossy().into_owned(),
                import_name: "slow_install_package".to_owned(),
            },
            &ExperimentConfig {
                timeout_seconds: 3,
            },
        );

        assert_eq!(report.status, ExperimentStatus::TimedOut);
        assert_eq!(report.stage, ExperimentStage::Installation);
        assert_eq!(
            report.message.as_deref(),
            Some("Package installation timed out")
        );
    }

    #[test]
    fn missing_module_returns_failed_import_report() {
        let directory = tempfile::tempdir().unwrap();
        let package_dir = create_local_package(directory.path());

        let experiment = Experiment {
            python_executable: "python".to_owned(),
            package: package_dir.to_string_lossy().into_owned(),
            import_name: "module_that_does_not_exist".to_owned(),
        };

        let report = run_experiment(
            &experiment,
            &ExperimentConfig {
                timeout_seconds: 60,
            },
        );

        assert_eq!(
            report.status,
            ExperimentStatus::Failed,
            "Expected import failure, got: {:?}\nstdout: {}\nstderr: {}",
            report.message,
            report.stdout,
            report.stderr
        );

        assert_eq!(report.stage, ExperimentStage::Completed);
        assert!(
            report
                .message
                .as_deref()
                .unwrap()
                .contains("Module import failed"),
            "Unexpected message: {:?}",
            report.message
        );
    }

    #[test]
    fn run_with_timeout_terminates_long_running_process() {
        let mut command = {
            #[cfg(windows)]
            {
                let mut command = Command::new("cmd");
                command.args(["/C", "ping -n 10 127.0.0.1 > NUL"]);
                command
            }

            #[cfg(not(windows))]
            {
                let mut command = Command::new("sh");
                command.args(["-c", "sleep 10"]);
                command
            }
        };

        let started = Instant::now();
        let result = run_with_timeout(&mut command, 1);
        let elapsed = started.elapsed();

        assert_eq!(
            result.unwrap_err(),
            "timeout",
            "Expected the process to time out"
        );

        assert!(elapsed.as_secs() < 5, "Timeout took too long: {elapsed:?}");
    }

    #[test]
    fn timeout_test_child_process() {
        if std::env::var("COMPATLAB_TIMEOUT_CHILD").is_ok() {
            std::thread::sleep(std::time::Duration::from_secs(30));
        }
    }

    #[test]
    fn run_with_timeout_terminates_test_child_process() {
        let executable = std::env::current_exe().unwrap();
        let mut command = Command::new(executable);

        command
            .args([
                "--exact",
                "runner::tests::timeout_test_child_process",
                "--nocapture",
            ])
            .env("COMPATLAB_TIMEOUT_CHILD", "1");

        let started = Instant::now();
        let result = run_with_timeout(&mut command, 1);
        let elapsed = started.elapsed();

        assert_eq!(
            result.unwrap_err(),
            "timeout",
            "Expected the child test process to time out"
        );

        assert!(
            elapsed.as_secs() < 5,
            "Child process took too long to terminate: {elapsed:?}"
        );
    }

    #[test]
    fn timeout_error_sets_timed_out_status() {
        let mut report = run_experiment(
            &test_experiment("compatlab-python-executable-that-does-not-exist"),
            &test_config(),
        );

        apply_timeout_error(
            &mut report,
            "timeout",
            "Package installation timed out",
            "package installation",
        );

        assert_eq!(report.status, ExperimentStatus::TimedOut);
        assert_eq!(
            report.message.as_deref(),
            Some("Package installation timed out")
        );
    }

    #[test]
    fn ordinary_process_error_sets_error_status() {
        let mut report = run_experiment(
            &test_experiment("compatlab-python-executable-that-does-not-exist"),
            &test_config(),
        );

        apply_timeout_error(
            &mut report,
            "permission denied",
            "Package installation timed out",
            "package installation",
        );

        assert_eq!(report.status, ExperimentStatus::Error);
        assert_eq!(
            report.message.as_deref(),
            Some("Failed to run package installation: permission denied")
        );
    }

    #[test]
    fn zero_second_timeout_does_not_spawn_process() {
        let mut command = Command::new("compatlab-command-that-must-not-be-spawned");

        let result = run_with_timeout(&mut command, 0);

        assert_eq!(result.unwrap_err(), "timeout");
    }

    #[test]
    fn run_with_timeout_returns_output_when_process_exits_before_deadline() {
        let mut command = Command::new("python");
        command
            .args(["-c", "print('finished')"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let output =
            run_with_timeout(&mut command, 5).expect("process should finish before the timeout");

        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "finished");
    }
}
