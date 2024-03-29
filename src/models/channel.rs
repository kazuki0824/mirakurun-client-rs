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
pub struct Channel {
    #[serde(rename = "type")]
    pub r#type: crate::models::ChannelType,
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "satellite", skip_serializing_if = "Option::is_none")]
    pub satellite: Option<String>,
    #[serde(rename = "space", skip_serializing_if = "Option::is_none")]
    pub space: Option<i32>,
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<f32>,
    #[serde(rename = "polarity", skip_serializing_if = "Option::is_none")]
    pub polarity: Option<Polarity>,
    #[serde(rename = "tsmfRelTs", skip_serializing_if = "Option::is_none")]
    pub tsmf_rel_ts: Option<i32>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::Service>>,
}

impl Channel {
    pub fn new(r#type: crate::models::ChannelType, channel: String) -> Channel {
        Channel {
            r#type,
            channel,
            name: None,
            satellite: None,
            space: None,
            freq: None,
            polarity: None,
            tsmf_rel_ts: None,
            services: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Polarity {
    #[serde(rename = "H")]
    H,
    #[serde(rename = "V")]
    V,
}

impl Default for Polarity {
    fn default() -> Polarity {
        Self::H
    }
}

