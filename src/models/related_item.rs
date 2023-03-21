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
pub struct RelatedItem {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "networkId", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<i64>,
    #[serde(rename = "serviceId", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<i64>,
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
}

impl RelatedItem {
    pub fn new() -> RelatedItem {
        RelatedItem {
            r#type: None,
            network_id: None,
            service_id: None,
            event_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "relay")]
    Relay,
    #[serde(rename = "movement")]
    Movement,
}

impl Default for Type {
    fn default() -> Type {
        Self::Shared
    }
}

