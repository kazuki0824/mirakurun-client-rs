/*
 * Mirakurun
 *
 * DVR Tuner Server for Japanese TV.
 *
 * The version of the OpenAPI document: 3.9.0-rc.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TunerUser {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "disableDecoder", skip_serializing_if = "Option::is_none")]
    pub disable_decoder: Option<bool>,
    #[serde(rename = "streamSetting", skip_serializing_if = "Option::is_none")]
    pub stream_setting: Option<Box<crate::models::TunerUserStreamSetting>>,
    #[serde(rename = "streamInfo", skip_serializing_if = "Option::is_none")]
    pub stream_info: Option<::std::collections::HashMap<String, crate::models::TunerUserStreamInfoValue>>,
}

impl TunerUser {
    pub fn new(id: String, priority: i32) -> TunerUser {
        TunerUser {
            id,
            priority,
            agent: None,
            url: None,
            disable_decoder: None,
            stream_setting: None,
            stream_info: None,
        }
    }
}


