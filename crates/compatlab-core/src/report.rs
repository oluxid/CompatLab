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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experiment_status_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&ExperimentStatus::Passed).unwrap(),
            "\"passed\""
        );
        assert_eq!(
            serde_json::to_string(&ExperimentStatus::TimedOut).unwrap(),
            "\"timed_out\""
        );
    }

    #[test]
    fn experiment_stage_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&ExperimentStage::EnvironmentSetup).unwrap(),
            "\"environment_setup\""
        );
        assert_eq!(
            serde_json::to_string(&ExperimentStage::ImportCheck).unwrap(),
            "\"import_check\""
        );
    }

    #[test]
    fn report_round_trips_through_json() {
        let report = ExperimentReport {
            python_executable: "python".to_owned(),
            python_version: Some("Python 3.14.7".to_owned()),
            package: "colorama".to_owned(),
            import_name: "colorama".to_owned(),
            status: ExperimentStatus::Passed,
            stage: ExperimentStage::Completed,
            duration_ms: 1500,
            stdout: String::new(),
            stderr: String::new(),
            message: Some("Package imported successfully".to_owned()),
        };

        let json = serde_json::to_string(&report).unwrap();
        let decoded: ExperimentReport = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.python_executable, report.python_executable);
        assert_eq!(decoded.python_version, report.python_version);
        assert_eq!(decoded.package, report.package);
        assert_eq!(decoded.import_name, report.import_name);
        assert_eq!(decoded.status, report.status);
        assert_eq!(decoded.stage, report.stage);
        assert_eq!(decoded.duration_ms, report.duration_ms);
        assert_eq!(decoded.stdout, report.stdout);
        assert_eq!(decoded.stderr, report.stderr);
        assert_eq!(decoded.message, report.message);
    }

    #[test]
    fn report_serializes_optional_fields_as_null() {
        let report = ExperimentReport {
            python_executable: "python".to_owned(),
            python_version: None,
            package: "example-package".to_owned(),
            import_name: "example_package".to_owned(),
            status: ExperimentStatus::Error,
            stage: ExperimentStage::EnvironmentSetup,
            duration_ms: 25,
            stdout: String::new(),
            stderr: "Python not found".to_owned(),
            message: None,
        };

        let json = serde_json::to_value(&report).unwrap();

        assert!(json["python_version"].is_null());
        assert!(json["message"].is_null());
        assert_eq!(json["status"], "error");
        assert_eq!(json["stage"], "environment_setup");
    }
}
