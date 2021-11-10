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
pub enum ProgramVideoResolution {
    #[serde(rename = "240p")]
    _240p,
    #[serde(rename = "480i")]
    _480i,
    #[serde(rename = "480p")]
    _480p,
    #[serde(rename = "720p")]
    _720p,
    #[serde(rename = "1080i")]
    _1080i,
    #[serde(rename = "1080p")]
    _1080p,
    #[serde(rename = "2160p")]
    _2160p,
    #[serde(rename = "4320p")]
    _4320p,

}

impl ToString for ProgramVideoResolution {
    fn to_string(&self) -> String {
        match self {
            Self::_240p => String::from("240p"),
            Self::_480i => String::from("480i"),
            Self::_480p => String::from("480p"),
            Self::_720p => String::from("720p"),
            Self::_1080i => String::from("1080i"),
            Self::_1080p => String::from("1080p"),
            Self::_2160p => String::from("2160p"),
            Self::_4320p => String::from("4320p"),
        }
    }
}




