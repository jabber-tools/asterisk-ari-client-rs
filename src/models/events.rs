use super::playbacks::Playback;
#[cfg(feature = "parse-event-datetimes")]
use crate::models::channels::ari_date_format;
use crate::models::channels::Channel;
use crate::models::recordings::Recording;
#[cfg(feature = "parse-event-datetimes")]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TBD: Event extends Message and all event types extend event.
// Since rust does not support inheritance we need to figure out how
// how composition would work with serde. For now just duplicating the stuff

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Indicates the type of this message.
    #[serde(rename = "type")]
    pub r#type: String,

    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Indicates the type of this message.
    #[serde(rename = "type")]
    pub r#type: String,

    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StasisStart {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created. E.g. 2020-11-22T20:12:51.214+0000
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Arguments to the application.
    pub args: Vec<String>,

    /// Channel.
    pub channel: Channel,

    /// Replace_channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_channel: Option<Channel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelDtmfReceived {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// DTMF digit received (0-9, A-E, # or *).
    pub digit: String,

    /// Number of milliseconds DTMF was received.
    pub duration_ms: i64,

    /// The channel on which DTMF was received.
    pub channel: Channel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelHangupRequest {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Integer representation of the cause of the hangup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<i64>,

    /// Whether the hangup request was a soft hangup request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft: Option<bool>,

    /// The channel on which the hangup was requested.
    pub channel: Channel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StasisEnd {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Channel.
    pub channel: Channel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelTalkingFinished {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// The channel on which talking completed.
    pub channel: Channel,

    /// The length of time, in milliseconds, that talking was detected on the channel.
    pub duration: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelTalkingStarted {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// The channel on which talking started.
    pub channel: Channel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelDestroyed {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Integer representation of the cause of the hangup.
    pub cause: i64,

    /// Text representation of the cause of the hangup.
    pub cause_txt: String,

    /// Channel.
    pub channel: Channel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybackStarted {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// playback resource
    pub playback: Playback,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybackFinished {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created.
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// playback resource
    pub playback: Playback,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelStateChange {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created. E.g. 2020-11-22T20:12:51.214+0000
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Channel.
    pub channel: Channel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelVarset {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created. E.g. 2020-11-22T20:12:51.214+0000
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Channel.
    pub channel: Channel,

    pub variable: String,

    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordingStarted {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created. E.g. 2020-11-22T20:12:51.214+0000
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Recording.
    pub recording: Recording,

}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordingFinished {
    /// The unique ID for the Asterisk instance that raised this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asterisk_id: Option<String>,

    /// Name of the application receiving the event.
    pub application: String,

    /// Time at which this event was created. E.g. 2020-11-22T20:12:51.214+0000
    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub timestamp: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub timestamp: String,

    /// Recording.
    pub recording: Recording,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AriEvent {
    StasisStart(StasisStart),
    ChannelDtmfReceived(ChannelDtmfReceived),
    ChannelHangupRequest(ChannelHangupRequest),
    StasisEnd(StasisEnd),
    ChannelTalkingFinished(ChannelTalkingFinished),
    ChannelTalkingStarted(ChannelTalkingStarted),
    ChannelDestroyed(ChannelDestroyed),
    PlaybackStarted(PlaybackStarted),
    PlaybackFinished(PlaybackFinished),
    ChannelStateChange(ChannelStateChange),
    ChannelVarset(ChannelVarset),
    RecordingStarted(RecordingStarted),
    RecordingFinished(RecordingFinished),
}

#[cfg(test)]
mod tests {
    use super::*;

    const STR_JSON: &str = "{\n  \"type\": \"StasisStart\",\n  \"timestamp\": \"2020-11-22T20:17:06.150+0000\",\n  \"args\": [\n    \"its-va-demo-app\",\n    \"en-US\"\n  ],\n  \"channel\": {\n    \"id\": \"1606076223.3\",\n    \"name\": \"PJSIP/6001-00000003\",\n    \"state\": \"Up\",\n    \"caller\": {\n      \"name\": \"\",\n      \"number\": \"6001\"\n    },\n    \"connected\": {\n      \"name\": \"\",\n      \"number\": \"\"\n    },\n    \"accountcode\": \"\",\n    \"dialplan\": {\n      \"context\": \"from-internal\",\n      \"exten\": \"101\",\n      \"priority\": 6,\n      \"app_name\": \"Stasis\",\n      \"app_data\": \"va-voicegw,its-va-demo-app,en-US\"\n    },\n    \"creationtime\": \"2020-11-22T20:17:03.741+0000\",\n    \"language\": \"en\"\n  },\n  \"asterisk_id\": \"00:15:5d:01:65:04\",\n  \"application\": \"va-voicegw\"\n}";
    const STR_JSON2: &str = "{\n  \"type\": \"StasisStart\",\n  \"timestamp\": \"2021-01-07T21:12:57.268+0100\",\n  \"args\": [\n    \"freight-cs-voice\",\n    \"en-US\"\n  ],\n  \"channel\": {\n    \"id\": \"1610050377.0\",\n    \"name\": \"SIP/1004-00000000\",\n    \"state\": \"Ring\",\n    \"caller\": {\n      \"name\": \"Adam\",\n      \"number\": \"1004\"\n    },\n    \"connected\": {\n      \"name\": \"\",\n      \"number\": \"\"\n    },\n    \"accountcode\": \"\",\n    \"dialplan\": {\n      \"context\": \"internal\",\n      \"exten\": \"158\",\n      \"priority\": 10,\n      \"app_name\": \"Stasis\",\n      \"app_data\": \"va-voicegw-rs,freight-cs-voice,en-US\"\n    },\n    \"creationtime\": \"2021-01-07T21:12:57.267+0100\",\n    \"language\": \"en\"\n  },\n  \"asterisk_id\": \"00:50:56:98:74:21\",\n  \"application\": \"va-voicegw-rs\"\n}";

    const STR_JSON_CHNL_STATE_CHANGED: &str = "{\n  \"type\": \"ChannelStateChange\",\n  \"timestamp\": \"2021-01-07T22:12:29.571+0100\",\n  \"channel\": {\n    \"id\": \"1610053949.0\",\n    \"name\": \"SIP/1004-00000000\",\n    \"state\": \"Up\",\n    \"caller\": {\n      \"name\": \"Adam\",\n      \"number\": \"1004\"\n    },\n    \"connected\": {\n      \"name\": \"\",\n      \"number\": \"\"\n    },\n    \"accountcode\": \"\",\n    \"dialplan\": {\n      \"context\": \"internal\",\n      \"exten\": \"158\",\n      \"priority\": 10,\n      \"app_name\": \"Stasis\",\n      \"app_data\": \"va-voicegw-rs,freight-cs-voice,en-US\"\n    },\n    \"creationtime\": \"2021-01-07T22:12:29.369+0100\",\n    \"language\": \"en\"\n  },\n  \"asterisk_id\": \"00:50:56:98:74:21\",\n  \"application\": \"va-voicegw-rs\"\n}";

    // cargo test --package asterisk-ari-client -- --show-output test_parse_stasis_start
    #[test]
    fn test_parse_stasis_start() {
        let ari_event: StasisStart = serde_json::from_str(STR_JSON).unwrap();
        println!("{:#?}", ari_event);
    }

    // cargo test --package asterisk-ari-client -- --show-output test_2_parse_stasis_start
    // throws chrono lib error: no possible date and time matching input
    // if: "2021-01-07T21:12:57.268+0100 -> "2021-01-07T21:12:57.268+0000 (i.e. TZ 0100 -> 00) it will pass
    // to be investigated
    // for now this resulted in feature parse-event-datetimes
    // i.e. this will pass:
    //      cargo test --package asterisk-ari-client -- --show-output test_2_parse_stasis_start
    // but this will not:
    //      cargo test --features parse-event-datetimes --package asterisk-ari-client -- --show-output test_2_parse_stasis_start
    #[test]
    fn test_2_parse_stasis_start() {
        let ari_event: StasisStart = serde_json::from_str(STR_JSON2).unwrap();
        println!("{:#?}", ari_event);
    }

    // cargo test --package asterisk-ari-client -- --show-output test_parse_channel_state_change
    #[test]
    fn test_parse_channel_state_change() {
        let ari_event: ChannelStateChange =
            serde_json::from_str(STR_JSON_CHNL_STATE_CHANGED).unwrap();
        println!("{:#?}", ari_event);
    }

    // cargo test -- --show-output test_parse_ari_event_stasis_start
    // serialize into enum AriEvent. Test that 'Internally tagged enum representation' serde feature works fine
    #[test]
    fn test_parse_ari_event_stasis_start() {
        let ari_event: AriEvent = serde_json::from_str(STR_JSON).unwrap();
        println!("{:#?}", ari_event);
    }
}
