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
pub struct ProgramVideo {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ProgramVideoType>,
    #[serde(rename = "resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::ProgramVideoResolution>,
    #[serde(rename = "streamContent", skip_serializing_if = "Option::is_none")]
    pub stream_content: Option<i32>,
    #[serde(rename = "componentType", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<i32>,
}

impl ProgramVideo {
    pub fn new() -> ProgramVideo {
        ProgramVideo {
            _type: None,
            resolution: None,
            stream_content: None,
            component_type: None,
        }
    }
}


