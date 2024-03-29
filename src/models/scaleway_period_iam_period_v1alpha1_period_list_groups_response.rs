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
pub struct ScalewayPeriodIamPeriodV1alpha1PeriodListGroupsResponse {
    /// List of groups
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup>>,
    /// Total count of groups
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl ScalewayPeriodIamPeriodV1alpha1PeriodListGroupsResponse {
    pub fn new() -> ScalewayPeriodIamPeriodV1alpha1PeriodListGroupsResponse {
        ScalewayPeriodIamPeriodV1alpha1PeriodListGroupsResponse {
            groups: None,
            total_count: None,
        }
    }
}
