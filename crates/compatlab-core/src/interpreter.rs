use std::path::Path;
use std::process::Command;

/// Information about a discovered Python interpreter.
#[derive(Debug, Clone)]
pub struct PythonInterpreter {
    pub executable: String,
    pub version: String,
}

/// Query a specific Python executable for its version.
///
/// Returns an error if the executable cannot be started or does not
/// return a successful version response.
pub fn inspect_interpreter(executable: impl AsRef<Path>) -> Result<PythonInterpreter, String> {
    let executable = executable.as_ref();

    let output = Command::new(executable)
        .args(["--version"])
        .output()
        .map_err(|error| {
            format!(
                "Failed to execute Python interpreter '{}': {error}",
                executable.display()
            )
        })?;

    if !output.status.success() {
        return Err(format!(
            "Python interpreter '{}' exited with status {}: {}",
            executable.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let version_output = if output.stdout.is_empty() {
        &output.stderr
    } else {
        &output.stdout
    };

    let version = String::from_utf8_lossy(version_output).trim().to_owned();

    if version.is_empty() {
        return Err(format!(
            "Python interpreter '{}' returned an empty version response",
            executable.display()
        ));
    }

    Ok(PythonInterpreter {
        executable: executable.to_string_lossy().into_owned(),
        version,
    })
}
