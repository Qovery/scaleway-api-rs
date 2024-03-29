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
pub struct ScalewayPeriodRdbPeriodV1PeriodInstanceSetting {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ScalewayPeriodRdbPeriodV1PeriodInstanceSetting {
    pub fn new() -> ScalewayPeriodRdbPeriodV1PeriodInstanceSetting {
        ScalewayPeriodRdbPeriodV1PeriodInstanceSetting {
            name: None,
            value: None,
        }
    }
}
