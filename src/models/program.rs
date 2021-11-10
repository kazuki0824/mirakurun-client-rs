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
pub struct Program {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "serviceId")]
    pub service_id: i32,
    #[serde(rename = "networkId")]
    pub network_id: i32,
    #[serde(rename = "startAt")]
    pub start_at: i32,
    #[serde(rename = "duration")]
    pub duration: i32,
    #[serde(rename = "isFree")]
    pub is_free: bool,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<crate::models::ProgramGenre>>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<crate::models::ProgramVideo>>,
    #[serde(rename = "audios", skip_serializing_if = "Option::is_none")]
    pub audios: Option<Vec<crate::models::ProgramAudios>>,
    #[serde(rename = "extended", skip_serializing_if = "Option::is_none")]
    pub extended: Option<serde_json::Value>,
    #[serde(rename = "relatedItems", skip_serializing_if = "Option::is_none")]
    pub related_items: Option<Vec<crate::models::RelatedItem>>,
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<Box<crate::models::ProgramSeries>>,
}

impl Program {
    pub fn new(id: i64, event_id: i32, service_id: i32, network_id: i32, start_at: i32, duration: i32, is_free: bool) -> Program {
        Program {
            id,
            event_id,
            service_id,
            network_id,
            start_at,
            duration,
            is_free,
            name: None,
            description: None,
            genres: None,
            video: None,
            audios: None,
            extended: None,
            related_items: None,
            series: None,
        }
    }
}


