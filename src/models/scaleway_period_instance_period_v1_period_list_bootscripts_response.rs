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
pub struct ScalewayPeriodInstancePeriodV1PeriodListBootscriptsResponse {
    /// List of bootscripts
    #[serde(rename = "bootscripts", skip_serializing_if = "Option::is_none")]
    pub bootscripts: Option<Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodBootscript>>,
}

impl ScalewayPeriodInstancePeriodV1PeriodListBootscriptsResponse {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodListBootscriptsResponse {
        ScalewayPeriodInstancePeriodV1PeriodListBootscriptsResponse { bootscripts: None }
    }
}