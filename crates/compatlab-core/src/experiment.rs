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

/// Describes the checks an experiment will perform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    /// Maximum time allowed for the experiment, in seconds.
    pub timeout_seconds: u64,
}
