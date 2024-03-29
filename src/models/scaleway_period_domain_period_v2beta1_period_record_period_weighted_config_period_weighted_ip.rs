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
pub struct ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodWeightedConfigPeriodWeightedIp {
    /// (IP address)
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodWeightedConfigPeriodWeightedIp {
    pub fn new() -> ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodWeightedConfigPeriodWeightedIp
    {
        ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodWeightedConfigPeriodWeightedIp {
            ip: None,
            weight: None,
        }
    }
}
