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
pub struct ScalewayPeriodIamPeriodV1alpha1PeriodListPermissionSetsResponse {
    /// List of permission sets
    #[serde(rename = "permission_sets", skip_serializing_if = "Option::is_none")]
    pub permission_sets:
        Option<Vec<crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPermissionSet>>,
    /// Total count of permission sets
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl ScalewayPeriodIamPeriodV1alpha1PeriodListPermissionSetsResponse {
    pub fn new() -> ScalewayPeriodIamPeriodV1alpha1PeriodListPermissionSetsResponse {
        ScalewayPeriodIamPeriodV1alpha1PeriodListPermissionSetsResponse {
            permission_sets: None,
            total_count: None,
        }
    }
}