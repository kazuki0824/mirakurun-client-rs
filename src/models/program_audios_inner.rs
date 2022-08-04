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
pub struct ProgramAudiosInner {
    #[serde(rename = "componentType", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<i32>,
    #[serde(rename = "componentTag", skip_serializing_if = "Option::is_none")]
    pub component_tag: Option<i32>,
    #[serde(rename = "isMain", skip_serializing_if = "Option::is_none")]
    pub is_main: Option<bool>,
    #[serde(rename = "samplingRate", skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<crate::models::ProgramAudioSamplingRate>,
    #[serde(rename = "langs", skip_serializing_if = "Option::is_none")]
    pub langs: Option<Vec<Langs>>,
}

impl ProgramAudiosInner {
    pub fn new() -> ProgramAudiosInner {
        ProgramAudiosInner {
            component_type: None,
            component_tag: None,
            is_main: None,
            sampling_rate: None,
            langs: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Langs {
    #[serde(rename = "jpn")]
    Jpn,
    #[serde(rename = "eng")]
    Eng,
    #[serde(rename = "deu")]
    Deu,
    #[serde(rename = "fra")]
    Fra,
    #[serde(rename = "ita")]
    Ita,
    #[serde(rename = "rus")]
    Rus,
    #[serde(rename = "zho")]
    Zho,
    #[serde(rename = "kor")]
    Kor,
    #[serde(rename = "spa")]
    Spa,
    #[serde(rename = "etc")]
    Etc,
}

impl Default for Langs {
    fn default() -> Langs {
        Self::Jpn
    }
}
