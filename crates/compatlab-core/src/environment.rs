use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

/// A Python virtual environment that stays alive while it is in use.
pub struct PythonEnvironment {
    // Keeping this alive prevents the temporary directory from being deleted.
    _temp_dir: TempDir,
    pub python_executable: PathBuf,
}

/// Details about a failed virtual-environment setup.
pub struct EnvironmentSetupError {
    pub message: String,
    pub stdout: String,
    pub stderr: String,
}

/// Create a temporary directory and initialize a Python virtual environment.
pub fn create_python_environment(
    python_executable: &str,
) -> Result<PythonEnvironment, EnvironmentSetupError> {
    let temp_dir = tempfile::tempdir().map_err(|error| EnvironmentSetupError {
        message: format!("Failed to create temporary directory: {error}"),
        stdout: String::new(),
        stderr: String::new(),
    })?;

    let venv_path = temp_dir.path().join("venv");

    let output = Command::new(python_executable)
        .args(["-m", "venv"])
        .arg(&venv_path)
        .output()
        .map_err(|error| EnvironmentSetupError {
            message: format!("Failed to create virtual environment: {error}"),
            stdout: String::new(),
            stderr: String::new(),
        })?;

    if !output.status.success() {
        return Err(EnvironmentSetupError {
            message: "Virtual environment creation failed".into(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    #[cfg(windows)]
    let python_path = venv_path.join("Scripts").join("python.exe");

    #[cfg(not(windows))]
    let python_path = venv_path.join("bin").join("python");

    Ok(PythonEnvironment {
        _temp_dir: temp_dir,
        python_executable: python_path,
    })
}
