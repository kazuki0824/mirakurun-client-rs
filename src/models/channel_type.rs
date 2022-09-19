/*
 * Mirakurun
 *
 * DVR Tuner Server for Japanese TV.
 *
 * The version of the OpenAPI document: 3.9.0-rc.2
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "CS")]
    Cs,
    #[serde(rename = "SKY")]
    Sky,

}

impl ToString for ChannelType {
    fn to_string(&self) -> String {
        match self {
            Self::Gr => String::from("GR"),
            Self::Bs => String::from("BS"),
            Self::Cs => String::from("CS"),
            Self::Sky => String::from("SKY"),
        }
    }
}

impl Default for ChannelType {
    fn default() -> ChannelType {
        Self::Gr
    }
}




