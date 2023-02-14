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
pub struct ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse {
    /// Paginated list of tags matching filters
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ScalewayPeriodRegistryPeriodV1PeriodTag>>,
    /// Total number of tags matching filters
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse {
    pub fn new() -> ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse {
        ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse {
            tags: None,
            total_count: None,
        }
    }
}
