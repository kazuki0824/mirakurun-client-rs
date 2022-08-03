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
pub struct StatusProcess {
    #[serde(rename = "arch", skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<serde_json::Value>,
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<serde_json::Value>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(rename = "memoryUsage", skip_serializing_if = "Option::is_none")]
    pub memory_usage: Option<Box<crate::models::StatusProcessMemoryUsage>>,
}

impl StatusProcess {
    pub fn new() -> StatusProcess {
        StatusProcess {
            arch: None,
            platform: None,
            versions: None,
            env: None,
            pid: None,
            memory_usage: None,
        }
    }
}


