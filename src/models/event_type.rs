/*
 * Mirakurun
 *
 * DVR Tuner Server Service for Chinachu Air.
 *
 * The version of the OpenAPI document: 3.9.0-beta.20
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "remove")]
    Remove,

}

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            Self::Create => String::from("create"),
            Self::Update => String::from("update"),
            Self::Remove => String::from("remove"),
        }
    }
}




