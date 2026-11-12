use serde::{Deserialize, Serialize};

/// Describes a single Python package compatibility experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    /// Path to the Python executable to test.
    pub python_executable: String,

    /// Package requirement passed to pip, e.g. "requests==2.32.5".
    pub package: String,

    /// Python module to import, e.g. "requests".
    pub import_name: String,
}

impl Experiment {
    /// Validate the user-supplied values before starting an experiment.
    pub fn validate(&self) -> Result<(), String> {
        if self.python_executable.trim().is_empty() {
            return Err("Python executable must not be empty".to_owned());
        }
        if self.package.trim().is_empty() {
            return Err("Package requirement must not be empty".to_owned());
        }
        if self.import_name.trim().is_empty() {
            return Err("Import name must not be empty".to_owned());
        }

        for component in self.import_name.split('.') {
            let mut characters = component.chars();
            let valid_first = characters
                .next()
                .is_some_and(|character| character == '_' || character.is_ascii_alphabetic());
            if !valid_first
                || !characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
            {
                return Err(format!(
                    "Import name must be a dot-separated Python module name: {}",
                    self.import_name
                ));
            }
        }

        Ok(())
    }
}

/// Describes the checks an experiment will perform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    /// Maximum time allowed for the experiment, in seconds.
    pub timeout_seconds: u64,
}

impl ExperimentConfig {
    /// Validate configuration values before starting an experiment.
    pub fn validate(&self) -> Result<(), String> {
        if self.timeout_seconds == 0 {
            return Err("Timeout must be greater than zero seconds".to_owned());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_experiment() -> Experiment {
        Experiment {
            python_executable: "python".to_owned(),
            package: "example-package".to_owned(),
            import_name: "example_package".to_owned(),
        }
    }

    #[test]
    fn experiment_validation_rejects_empty_values_and_invalid_import_names() {
        let mut experiment = valid_experiment();
        experiment.python_executable = " ".to_owned();
        assert!(experiment.validate().is_err());

        let mut experiment = valid_experiment();
        experiment.package = "\t".to_owned();
        assert!(experiment.validate().is_err());

        let mut experiment = valid_experiment();
        experiment.import_name = " ".to_owned();
        assert!(experiment.validate().is_err());

        let mut experiment = valid_experiment();
        experiment.import_name = "package-name".to_owned();
        assert!(experiment.validate().is_err());
    }

    #[test]
    fn experiment_validation_accepts_dotted_python_module_names() {
        let mut experiment = valid_experiment();
        experiment.import_name = "_package.v2".to_owned();

        assert!(experiment.validate().is_ok());
    }

    #[test]
    fn config_validation_rejects_zero_timeout() {
        assert!(
            ExperimentConfig { timeout_seconds: 0 }
                .validate()
                .is_err()
        );
        assert!(
            ExperimentConfig { timeout_seconds: 1 }
                .validate()
                .is_ok()
        );
    }
}
