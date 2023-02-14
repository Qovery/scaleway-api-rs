/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayPeriodInstancePeriodV1PeriodVolumeTypePeriodConstraints {
    /// (in bytes)
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    /// (in bytes)
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}

impl ScalewayPeriodInstancePeriodV1PeriodVolumeTypePeriodConstraints {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodVolumeTypePeriodConstraints {
        ScalewayPeriodInstancePeriodV1PeriodVolumeTypePeriodConstraints {
            min: None,
            max: None,
        }
    }
}
