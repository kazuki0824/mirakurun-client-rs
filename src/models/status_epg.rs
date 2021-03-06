/*
 * Mirakurun
 *
 * DVR Tuner Server Service for Chinachu Air.
 *
 * The version of the OpenAPI document: 3.9.0-beta.20
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusEpg {
    #[serde(rename = "gatheringNetworks", skip_serializing_if = "Option::is_none")]
    pub gathering_networks: Option<Vec<i32>>,
    #[serde(rename = "storedEvents", skip_serializing_if = "Option::is_none")]
    pub stored_events: Option<i32>,
}

impl StatusEpg {
    pub fn new() -> StatusEpg {
        StatusEpg {
            gathering_networks: None,
            stored_events: None,
        }
    }
}


