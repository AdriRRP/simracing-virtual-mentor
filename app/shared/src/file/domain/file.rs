use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Accepted,
    Success,
    Fail(String),
}

/// Represents a file.
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct File {
    /// The SHA256 hash of the source bytes.
    pub id: String,
    /// The name of the file.
    pub name: String,
    /// Indicates the status of the file.
    pub status: Status,
    /// Date on which the file was created
    pub created_on: DateTime<Utc>, // Serialization: 2024-06-19T19:15:25.258553Z
}

impl File {
    /// Creates a new `File` instance with the specified ID and name.
    ///
    /// # Arguments
    ///
    /// * `id` - The SHA256 hash of the source bytes.
    /// * `name` - The name of the file.
    ///
    /// # Returns
    ///
    /// A new `File` instance.
    #[must_use]
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            status: Status::Accepted,
            created_on: Utc::now(),
        }
    }

    /// Marks the file as complete.
    pub fn success(&mut self) {
        self.status = Status::Success;
    }
    pub fn fail(&mut self, msg: &str) {
        self.status = Status::Fail(msg.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepted_status_serialization() {
        let status = Status::Accepted;
        let result = serde_json::to_string(&status).unwrap();
        let expected = "\"accepted\"".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_success_status_serialization() {
        let status = Status::Success;
        let result = serde_json::to_string(&status).unwrap();
        let expected = "\"success\"".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_fail_status_serialization() {
        let status = Status::Fail("Error Message".to_owned());
        let result = serde_json::to_string(&status).unwrap();
        let expected = "{\"fail\":\"Error Message\"}".to_owned();
        assert_eq!(result, expected)
    }
}
