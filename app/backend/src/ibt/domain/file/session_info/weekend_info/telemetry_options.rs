use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TelemetryOptions {
    pub telemetry_disk_file: Option<String>,
}
