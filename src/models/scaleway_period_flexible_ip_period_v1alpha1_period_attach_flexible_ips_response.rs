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
pub struct ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse {
    /// Total count of Flexible IPs being updated
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// Listing of Flexible IPs in updating state
    #[serde(rename = "flexible_ips", skip_serializing_if = "Option::is_none")]
    pub flexible_ips:
        Option<Vec<crate::models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp>>,
}

impl ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse {
    pub fn new() -> ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse {
        ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse {
            total_count: None,
            flexible_ips: None,
        }
    }
}