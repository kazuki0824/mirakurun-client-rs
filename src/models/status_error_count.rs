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
pub struct StatusErrorCount {
    #[serde(rename = "uncaughtException", skip_serializing_if = "Option::is_none")]
    pub uncaught_exception: Option<i32>,
    #[serde(rename = "unhandledRejection", skip_serializing_if = "Option::is_none")]
    pub unhandled_rejection: Option<i32>,
    #[serde(rename = "bufferOverflow", skip_serializing_if = "Option::is_none")]
    pub buffer_overflow: Option<i32>,
    #[serde(rename = "tunerDeviceRespawn", skip_serializing_if = "Option::is_none")]
    pub tuner_device_respawn: Option<i32>,
    #[serde(rename = "decoderRespawn", skip_serializing_if = "Option::is_none")]
    pub decoder_respawn: Option<i32>,
}

impl StatusErrorCount {
    pub fn new() -> StatusErrorCount {
        StatusErrorCount {
            uncaught_exception: None,
            unhandled_rejection: None,
            buffer_overflow: None,
            tuner_device_respawn: None,
            decoder_respawn: None,
        }
    }
}


