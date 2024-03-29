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
pub struct ScalewayPeriodInstancePeriodV1PeriodListPrivateNicsResponse {
    #[serde(rename = "private_nics", skip_serializing_if = "Option::is_none")]
    pub private_nics: Option<Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodPrivateNic>>,
}

impl ScalewayPeriodInstancePeriodV1PeriodListPrivateNicsResponse {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodListPrivateNicsResponse {
        ScalewayPeriodInstancePeriodV1PeriodListPrivateNicsResponse { private_nics: None }
    }
}
