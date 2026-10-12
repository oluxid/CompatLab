use serde::{Deserialize, Serialize};

/// Overall outcome of an experiment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentStatus {
    Passed,
    Failed,
    Error,
    TimedOut,
}

/// The stage at which an experiment stopped or failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentStage {
    EnvironmentSetup,
    Installation,
    ImportCheck,
    Completed,
}

/// Results from a single compatibility experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentReport {
    pub python_executable: String,
    pub python_version: Option<String>,
    pub package: String,
    pub import_name: String,
    pub status: ExperimentStatus,
    pub stage: ExperimentStage,
    pub duration_ms: u64,
    pub stdout: String,
    pub stderr: String,
    pub message: Option<String>,
}
