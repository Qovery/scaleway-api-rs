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
pub struct ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupResponse {
    #[serde(rename = "security_group", skip_serializing_if = "Option::is_none")]
    pub security_group:
        Option<Box<crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroup>>,
}

impl ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupResponse {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupResponse {
        ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupResponse {
            security_group: None,
        }
    }
}
