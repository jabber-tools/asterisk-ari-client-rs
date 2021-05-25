use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Playback {
    /// ID for this playback operation
    pub id: String,
    /// The URI for the media currently being played back.
    pub media_uri: String,
    /// If a list of URIs is being played, the next media URI to be played back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_media_uri: Option<String>,
    /// URI for the channel or bridge to play the media on. e.g.: channel:1607454635.2
    pub target_uri: String,
    /// For media types that support multiple languages, the language requested for playback.
    pub language: String,
    /// Current state of the playback operation.
    /// allowed values: queued, playing, continuing, done
    pub state: String,
}
