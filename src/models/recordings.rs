use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recording {
    /// The name of the recording
    pub name: String,
    /// Format the recording was recorded in
    pub format: String,
    /// State of the recording
    pub state: String,
    pub target_uri: String,
}
