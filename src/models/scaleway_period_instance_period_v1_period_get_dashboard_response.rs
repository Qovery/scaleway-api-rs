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
pub struct ScalewayPeriodInstancePeriodV1PeriodGetDashboardResponse {
    #[serde(rename = "dashboard", skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<Box<crate::models::ScalewayPeriodInstancePeriodV1PeriodDashboard>>,
}

impl ScalewayPeriodInstancePeriodV1PeriodGetDashboardResponse {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodGetDashboardResponse {
        ScalewayPeriodInstancePeriodV1PeriodGetDashboardResponse { dashboard: None }
    }
}
