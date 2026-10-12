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
    inspect_command(executable.as_ref(), &["--version"])
}

fn inspect_command(executable: &Path, args: &[&str]) -> Result<PythonInterpreter, String> {
    let output = Command::new(executable)
        .args(args)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspect_interpreter_returns_version_for_valid_python() {
        let result = inspect_interpreter("python");

        assert!(
            result.is_ok(),
            "Expected Python inspection to succeed: {result:?}"
        );

        let interpreter = result.unwrap();

        assert_eq!(interpreter.executable, "python");
        assert!(
            interpreter.version.starts_with("Python "),
            "Unexpected version output: {}",
            interpreter.version
        );
    }

    #[test]
    fn inspect_interpreter_rejects_invalid_executable() {
        let result = inspect_interpreter("compatlab-python-executable-that-does-not-exist");

        assert!(
            result.is_err(),
            "Expected inspection of a nonexistent executable to fail"
        );
    }

    #[test]
    fn inspect_interpreter_rejects_nonzero_exit() {
        let result = if cfg!(windows) {
            let comspec = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_owned());

            inspect_command(Path::new(&comspec), &["/C", "exit", "1"])
        } else {
            inspect_command(Path::new("sh"), &["-c", "exit 1"])
        };

        let error = result.expect_err("Expected nonzero exit status to be rejected");

        assert!(
            error.contains("exited with status"),
            "Expected a nonzero-exit error, got: {error}"
        );
    }

    #[test]
    fn inspect_interpreter_rejects_empty_version_output() {
        let result = if cfg!(windows) {
            let comspec = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_owned());

            inspect_command(Path::new(&comspec), &["/C", "exit", "0"])
        } else {
            inspect_command(Path::new("sh"), &["-c", "exit 0"])
        };

        let error = result.expect_err("Expected empty version output to be rejected");

        assert!(
            error.contains("returned an empty version response"),
            "Unexpected error: {error}"
        );
    }
}
