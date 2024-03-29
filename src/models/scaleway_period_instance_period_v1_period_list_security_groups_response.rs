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
pub struct ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupsResponse {
    /// List of security groups
    #[serde(rename = "security_groups", skip_serializing_if = "Option::is_none")]
    pub security_groups:
        Option<Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroup>>,
}

impl ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupsResponse {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupsResponse {
        ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupsResponse {
            security_groups: None,
        }
    }
}
