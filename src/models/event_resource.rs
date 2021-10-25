/*
 * Mirakurun
 *
 * DVR Tuner Server Service for Chinachu Air.
 *
 * The version of the OpenAPI document: 3.9.0-beta.7
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventResource {
    #[serde(rename = "program")]
    Program,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "tuner")]
    Tuner,

}

impl ToString for EventResource {
    fn to_string(&self) -> String {
        match self {
            Self::Program => String::from("program"),
            Self::Service => String::from("service"),
            Self::Tuner => String::from("tuner"),
        }
    }
}




