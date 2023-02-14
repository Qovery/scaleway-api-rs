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
pub struct ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse {
    /// Remote instance logs details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponsePeriodInstanceLogDetail>>,
}

impl ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse {
    pub fn new() -> ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse {
        ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse { details: None }
    }
}
