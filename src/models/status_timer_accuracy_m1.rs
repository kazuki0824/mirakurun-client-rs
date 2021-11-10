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
pub struct StatusTimerAccuracyM1 {
    #[serde(rename = "avg", skip_serializing_if = "Option::is_none")]
    pub avg: Option<f32>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f32>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f32>,
}

impl StatusTimerAccuracyM1 {
    pub fn new() -> StatusTimerAccuracyM1 {
        StatusTimerAccuracyM1 {
            avg: None,
            min: None,
            max: None,
        }
    }
}


