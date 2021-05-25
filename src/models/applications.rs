use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub name: String,

    pub channel_ids: Vec<String>,

    pub bridge_ids: Vec<String>,

    pub endpoint_ids: Vec<String>,

    pub device_names: Vec<String>,
    // TBD: this is vector of respective objects
    // currently not used/supported
    // pub events_allowed: Vec<String>,

    // TBD: this is vector of respective objects
    // currently not used/supported
    // pub events_disallowed: Vec<String>,
}
