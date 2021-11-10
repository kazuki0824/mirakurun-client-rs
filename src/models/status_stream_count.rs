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
pub struct StatusStreamCount {
    #[serde(rename = "tunerDevice", skip_serializing_if = "Option::is_none")]
    pub tuner_device: Option<i32>,
    #[serde(rename = "tsFilter", skip_serializing_if = "Option::is_none")]
    pub ts_filter: Option<i32>,
    #[serde(rename = "decoder", skip_serializing_if = "Option::is_none")]
    pub decoder: Option<i32>,
}

impl StatusStreamCount {
    pub fn new() -> StatusStreamCount {
        StatusStreamCount {
            tuner_device: None,
            ts_filter: None,
            decoder: None,
        }
    }
}


