use core::fmt;

use serde::{Deserialize, Serialize};
// https://serde.rs/custom-date-format.html
#[cfg(feature = "parse-event-datetimes")]
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// Unique identifier of the channel.  This is the same as the Uniqueid field in AMI.
    pub id: String,

    /// Name of the channel (i.e. SIP/foo-0000a7e3)
    pub name: String,

    pub state: String,

    pub caller: CallerId,

    pub connected: CallerId,

    pub accountcode: String,

    pub dialplan: DialplanCep,

    #[cfg(feature = "parse-event-datetimes")]
    #[serde(with = "ari_date_format")]
    pub creationtime: DateTime<Utc>,

    #[cfg(not(feature = "parse-event-datetimes"))]
    pub creationtime: String,

    /// The default spoken language
    pub language: Option<String>,

    /// Channel variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channelvars: Option<serde_json::Value>,
}

#[cfg(feature = "parse-event-datetimes")]
pub mod ari_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    // see https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
    // same as %+ using longer format for better readability
    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.f%:z";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallerId {
    pub name: String,

    pub number: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DialplanCep {
    /// Context in the dialplan
    pub context: String,

    /// Extension in the dialplan
    pub exten: String,

    /// Priority in the dialplan
    pub priority: i64,

    /// Name of current dialplan application
    pub app_name: String,

    /// Parameter of current dialplan application
    pub app_data: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RtPstat {
    pub txcount: i64,

    pub rxcount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txjitter: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rxjitter: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_maxjitter: Option<f64>,

    #[serde(rename = "remote_minjitter", skip_serializing_if = "Option::is_none")]
    pub remote_minjitter: Option<f64>,

    #[serde(
        rename = "remote_normdevjitter",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_normdevjitter: Option<f64>,

    #[serde(rename = "remote_stdevjitter", skip_serializing_if = "Option::is_none")]
    pub remote_stdevjitter: Option<f64>,

    #[serde(rename = "local_maxjitter", skip_serializing_if = "Option::is_none")]
    pub local_maxjitter: Option<f64>,

    #[serde(rename = "local_minjitter", skip_serializing_if = "Option::is_none")]
    pub local_minjitter: Option<f64>,

    #[serde(
        rename = "local_normdevjitter",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_normdevjitter: Option<f64>,

    #[serde(rename = "local_stdevjitter", skip_serializing_if = "Option::is_none")]
    pub local_stdevjitter: Option<f64>,

    pub txploss: i64,

    pub rxploss: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_maxrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_minrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_normdevrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_stdevrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_maxrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_minrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_normdevrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_stdevrxploss: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtt: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxrtt: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minrtt: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub normdevrtt: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdevrtt: Option<f64>,

    pub local_ssrc: i64,

    pub remote_ssrc: i64,

    pub txoctetcount: i64,

    pub rxoctetcount: i64,

    /// The Asterisk channel's unique ID that owns this instance.
    pub channel_uniqueid: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum Direction {
    #[default]
    None,
    Both,
    Out,
    In
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Direction::None => "none",
            Direction::Both => "both",
            Direction::Out => "out",
            Direction::In => "in"
        };
        write!(f, "{}", str)
    }
}
